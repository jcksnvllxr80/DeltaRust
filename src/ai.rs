//! Enemy AI: steering behaviors, perception, and archetype tuning.
//!
//! Architecture: COMPOSITION, not inheritance. Every enemy carries an
//! `Archetype` tag; all behavior differences flow from the `AiTuning` table
//! plus a small per-type action layer in game.rs (hops, burrows, dives,
//! thrusts). This keeps every tunable in one place and makes sub-variants a
//! one-line table edit.
//!
//! Layers:
//!   1. Steering math (pure functions, this module) — seek / flee / arrive /
//!      pursue / wander / separation / wall probe.
//!   2. Perception (this module) — line of sight, field of view, hearing.
//!   3. State machine + per-type attack actions (game.rs driver).
//!
//! Performance: O(n^2) separation and O(n) raycasts are comfortably fine for
//! the ~100-enemy ceiling here (16x11 tile rooms). Past a few hundred actors,
//! add a spatial hash for separation and cache LOS per decision interval.

use crate::constants::TILE;
use crate::model::{Archetype, Enemy};
use crate::world::World;

// ---------------------------------------------------------------------------
// Tuning
// ---------------------------------------------------------------------------

/// Every knob that shapes how an archetype moves and perceives.
/// Distances are in TILES (converted at use), times in frames (60/s).
pub struct AiTuning {
    /// Multiplier on the enemy's base speed for its top speed.
    pub max_speed_mult: f32,
    /// 0..1 — how quickly velocity converges on the desired velocity.
    /// Low = heavy/drifty (tank momentum), high = snappy.
    pub accel: f32,
    /// Max radians/frame the movement heading may rotate. Low values make an
    /// enemy commit to a line and turn in wide arcs.
    pub turn_rate: f32,
    /// How far the enemy can see (tiles).
    pub sight_tiles: f32,
    /// Vision cone full angle in degrees. 360 = eyes everywhere.
    pub fov_deg: f32,
    /// Radius (tiles) in which player noise (walking/attacking) is heard.
    pub hearing_tiles: f32,
    /// Inside this radius the enemy aggros regardless of facing (skin sense).
    pub aggro_tiles: f32,
    /// Combat-positioning distance the enemy tries to hold (tiles).
    pub preferred_tiles: f32,
    /// Closer than this and the enemy backs off (tiles; snipers).
    pub min_range_tiles: f32,
    /// Frames between spotting the player and actually reacting.
    pub reaction_frames: i32,
    /// Frames between steering retargets — enemies do NOT track every frame.
    pub decision_interval: i32,
    /// Directional imprecision (radians) added to every retarget.
    pub wobble: f32,
    /// Flee when hp/max_hp drops below this (0.0 = never flees).
    pub flee_hp_frac: f32,
    /// How long (frames) the last known player position is remembered.
    pub memory_frames: i32,
    /// First sighting alerts allies within this radius (tiles). 0 = silent.
    pub shout_tiles: f32,
    /// Personal-space radius (tiles) for separation forces.
    pub separation_tiles: f32,
    /// Weight of the separation force relative to the primary steering.
    pub separation_weight: f32,
    /// Arrive: begin decelerating inside this radius (tiles).
    pub arrive_tiles: f32,
    /// Seconds of player movement the enemy leads its pursuit by.
    pub lookahead: f32,
}

/// The single source of truth for how each archetype behaves.
pub fn tuning(archetype: Archetype) -> &'static AiTuning {
    match archetype {
        // Direct pursuer. Mid sight, sloppy aim, gives up on nothing,
        // panics when nearly dead.
        Archetype::Grunt => &AiTuning {
            max_speed_mult: 1.0,
            accel: 0.18,
            turn_rate: 0.35,
            sight_tiles: 6.0,
            fov_deg: 200.0,
            hearing_tiles: 4.0,
            aggro_tiles: 2.0,
            preferred_tiles: 0.0,
            min_range_tiles: 0.0,
            reaction_frames: 14,
            decision_interval: 10,
            wobble: 0.35,
            flee_hp_frac: 0.34,
            memory_frames: 180,
            shout_tiles: 4.0,
            separation_tiles: 1.0,
            separation_weight: 0.8,
            arrive_tiles: 0.6,
            lookahead: 0.0,
        },
        // Boids unit: weak alone, moves as a school. Wide senses (the swarm
        // shares eyes), no fear, low shout (they're already together).
        Archetype::Swarm => &AiTuning {
            max_speed_mult: 1.25,
            accel: 0.25,
            turn_rate: 0.6,
            sight_tiles: 7.0,
            fov_deg: 360.0,
            hearing_tiles: 6.0,
            aggro_tiles: 3.0,
            preferred_tiles: 0.0,
            min_range_tiles: 0.0,
            reaction_frames: 6,
            decision_interval: 6,
            wobble: 0.25,
            flee_hp_frac: 0.0,
            memory_frames: 120,
            shout_tiles: 0.0,
            separation_tiles: 0.8,
            separation_weight: 1.2,
            arrive_tiles: 0.5,
            lookahead: 0.1,
        },
        // Side-and-back attacker. Predicts player movement, approaches off-axis,
        // strikes from the flank. Sharp eyes, narrow attention.
        Archetype::Flanker => &AiTuning {
            max_speed_mult: 1.15,
            accel: 0.12,
            turn_rate: 0.25,
            sight_tiles: 8.0,
            fov_deg: 260.0,
            hearing_tiles: 5.0,
            aggro_tiles: 2.5,
            preferred_tiles: 3.0,
            min_range_tiles: 1.2,
            reaction_frames: 10,
            decision_interval: 8,
            wobble: 0.2,
            flee_hp_frac: 0.0,
            memory_frames: 240,
            shout_tiles: 3.0,
            separation_tiles: 1.2,
            separation_weight: 0.6,
            arrive_tiles: 1.0,
            lookahead: 0.35,
        },
        // Distance fighter. Holds a firing range, retreats when crowded,
        // repositions rather than brawls.
        Archetype::Sniper => &AiTuning {
            max_speed_mult: 0.9,
            accel: 0.2,
            turn_rate: 0.5,
            sight_tiles: 9.0,
            fov_deg: 360.0,
            hearing_tiles: 7.0,
            aggro_tiles: 2.0,
            preferred_tiles: 5.0,
            min_range_tiles: 3.0,
            reaction_frames: 8,
            decision_interval: 12,
            wobble: 0.1,
            flee_hp_frac: 0.5,
            memory_frames: 300,
            shout_tiles: 5.0,
            separation_tiles: 2.0,
            separation_weight: 0.7,
            arrive_tiles: 1.5,
            lookahead: 0.2,
        },
        // Slow to start, slow to turn, terrifying once rolling. Never flees.
        Archetype::Tank => &AiTuning {
            max_speed_mult: 1.3,
            accel: 0.04,
            turn_rate: 0.07,
            sight_tiles: 5.5,
            fov_deg: 160.0,
            hearing_tiles: 3.5,
            aggro_tiles: 1.8,
            preferred_tiles: 0.0,
            min_range_tiles: 0.0,
            reaction_frames: 24,
            decision_interval: 16,
            wobble: 0.08,
            flee_hp_frac: 0.0,
            memory_frames: 360,
            shout_tiles: 2.5,
            separation_tiles: 1.4,
            separation_weight: 0.4,
            arrive_tiles: 0.8,
            lookahead: 0.25,
        },
        // Plays dead until the player is close, then erupts. Tiny senses,
        // explosive engagement, long memory once woken.
        Archetype::Ambusher => &AiTuning {
            max_speed_mult: 1.6,
            accel: 0.3,
            turn_rate: 0.5,
            sight_tiles: 2.5,
            fov_deg: 360.0,
            hearing_tiles: 1.5,
            aggro_tiles: 2.5,
            preferred_tiles: 0.0,
            min_range_tiles: 0.0,
            reaction_frames: 2,
            decision_interval: 6,
            wobble: 0.15,
            flee_hp_frac: 0.0,
            memory_frames: 420,
            shout_tiles: 3.0,
            separation_tiles: 1.0,
            separation_weight: 0.7,
            arrive_tiles: 0.5,
            lookahead: 0.15,
        },
        // Arena boss: deliberate drifting menace that holds mid-range.
        Archetype::Boss => &AiTuning {
            max_speed_mult: 1.0,
            accel: 0.06,
            turn_rate: 0.15,
            sight_tiles: 20.0,
            fov_deg: 360.0,
            hearing_tiles: 20.0,
            aggro_tiles: 20.0,
            preferred_tiles: 4.0,
            min_range_tiles: 2.0,
            reaction_frames: 0,
            decision_interval: 10,
            wobble: 0.12,
            flee_hp_frac: 0.0,
            memory_frames: 600,
            shout_tiles: 0.0,
            separation_tiles: 1.5,
            separation_weight: 0.3,
            arrive_tiles: 1.5,
            lookahead: 0.3,
        },
    }
}

// ---------------------------------------------------------------------------
// Steering behaviors (pure vector math)
// ---------------------------------------------------------------------------

pub fn normalize(x: f32, y: f32) -> (f32, f32) {
    let len = (x * x + y * y).sqrt();
    if len < 0.0001 { (0.0, 0.0) } else { (x / len, y / len) }
}

/// Full speed straight at the target.
pub fn seek(x: f32, y: f32, tx: f32, ty: f32, max_speed: f32) -> (f32, f32) {
    let (nx, ny) = normalize(tx - x, ty - y);
    (nx * max_speed, ny * max_speed)
}

/// Straight away from the threat.
pub fn flee(x: f32, y: f32, tx: f32, ty: f32, max_speed: f32) -> (f32, f32) {
    let (nx, ny) = normalize(x - tx, y - ty);
    (nx * max_speed, ny * max_speed)
}

/// Seek that decelerates inside `slow_radius` so the enemy settles instead
/// of orbiting/overshooting its target point.
pub fn arrive(x: f32, y: f32, tx: f32, ty: f32, max_speed: f32, slow_radius: f32) -> (f32, f32) {
    let dx = tx - x;
    let dy = ty - y;
    let dist = (dx * dx + dy * dy).sqrt();
    if dist < 0.0001 {
        return (0.0, 0.0);
    }
    let speed = if dist < slow_radius {
        max_speed * (dist / slow_radius)
    } else {
        max_speed
    };
    (dx / dist * speed, dy / dist * speed)
}

/// Seek toward where the target WILL be, leading by `lookahead` seconds of
/// its current velocity.
pub fn pursue(
    x: f32,
    y: f32,
    tx: f32,
    ty: f32,
    tvx: f32,
    tvy: f32,
    max_speed: f32,
    lookahead: f32,
) -> (f32, f32) {
    let frames = lookahead * 60.0;
    seek(x, y, tx + tvx * frames, ty + tvy * frames, max_speed)
}

/// Gentle aimless drift: the wander angle random-walks, the enemy follows it.
pub fn wander(wander_angle: &mut f32, max_speed: f32, jitter: f32) -> (f32, f32) {
    *wander_angle += macroquad::rand::gen_range(-jitter, jitter);
    (
        wander_angle.cos() * max_speed * 0.4,
        wander_angle.sin() * max_speed * 0.4,
    )
}

/// Push-apart force from every nearby ally. Prevents clumping/stacking.
/// `positions` is a pre-pass snapshot of (x, y, active) for all enemies.
pub fn separation(
    index: usize,
    x: f32,
    y: f32,
    positions: &[(f32, f32, bool)],
    radius: f32,
) -> (f32, f32) {
    let mut fx = 0.0;
    let mut fy = 0.0;
    for (i, &(ox, oy, active)) in positions.iter().enumerate() {
        if i == index || !active {
            continue;
        }
        let dx = x - ox;
        let dy = y - oy;
        let dist = (dx * dx + dy * dy).sqrt();
        if dist > 0.001 && dist < radius {
            // Inverse falloff: the closer the ally, the harder the push.
            let strength = (radius - dist) / radius;
            fx += dx / dist * strength;
            fy += dy / dist * strength;
        }
    }
    (fx, fy)
}

/// Boids: average velocity (alignment) and center pull (cohesion) of nearby
/// swarm-mates. Combined with `separation` this is the full flocking triple.
pub fn flock(
    index: usize,
    x: f32,
    y: f32,
    swarm: &[(usize, f32, f32, f32, f32)],
    radius: f32,
) -> (f32, f32) {
    let mut count = 0.0;
    let (mut avx, mut avy, mut cx, mut cy) = (0.0, 0.0, 0.0, 0.0);
    for &(i, ox, oy, ovx, ovy) in swarm {
        if i == index {
            continue;
        }
        let dx = ox - x;
        let dy = oy - y;
        if (dx * dx + dy * dy).sqrt() < radius {
            avx += ovx;
            avy += ovy;
            cx += ox;
            cy += oy;
            count += 1.0;
        }
    }
    if count == 0.0 {
        return (0.0, 0.0);
    }
    let (alx, aly) = normalize(avx / count, avy / count);
    let (cox, coy) = normalize(cx / count - x, cy / count - y);
    (alx * 0.5 + cox * 0.5, aly * 0.5 + coy * 0.5)
}

/// Lightweight obstacle avoidance: probe half a tile ahead of the velocity;
/// if it's solid, add a perpendicular swerve. This is also the hook point for
/// a future grid pathfinder — replace the swerve with a path-following force.
pub fn wall_avoid(world: &World, x: f32, y: f32, vx: f32, vy: f32, w: f32, h: f32) -> (f32, f32) {
    let speed = (vx * vx + vy * vy).sqrt();
    if speed < 0.001 {
        return (0.0, 0.0);
    }
    let probe = TILE * 0.6;
    let ahead_x = x + vx / speed * probe;
    let ahead_y = y + vy / speed * probe;
    if world.collides(ahead_x, ahead_y, w, h) {
        // Swerve perpendicular; pick the side that is open.
        let (px_, py_) = (-vy / speed, vx / speed);
        if !world.collides(x + px_ * probe, y + py_ * probe, w, h) {
            return (px_, py_);
        }
        return (-px_, -py_);
    }
    (0.0, 0.0)
}

// ---------------------------------------------------------------------------
// Perception
// ---------------------------------------------------------------------------

/// Tile raycast: samples the segment every half tile. Enemies do not see
/// through walls.
pub fn line_of_sight(world: &World, x0: f32, y0: f32, x1: f32, y1: f32) -> bool {
    let dx = x1 - x0;
    let dy = y1 - y0;
    let dist = (dx * dx + dy * dy).sqrt();
    let steps = (dist / (TILE * 0.5)).ceil() as i32;
    for i in 1..steps {
        let t = i as f32 / steps as f32;
        let sx = x0 + dx * t;
        let sy = y0 + dy * t;
        if world.is_solid((sx / TILE).floor() as i32, (sy / TILE).floor() as i32) {
            return false;
        }
    }
    true
}

/// Is the target inside the vision cone centered on `heading`?
pub fn in_fov(heading: f32, dx: f32, dy: f32, fov_deg: f32) -> bool {
    if fov_deg >= 360.0 {
        return true;
    }
    let to_target = dy.atan2(dx);
    let mut diff = (to_target - heading).abs();
    if diff > std::f32::consts::PI {
        diff = std::f32::consts::TAU - diff;
    }
    diff <= fov_deg.to_radians() / 2.0
}

/// Rotate `heading` toward `target_angle` by at most `turn_rate` radians.
/// This is what gives tanks their wide, committed turns.
pub fn turn_toward(heading: f32, target_angle: f32, turn_rate: f32) -> f32 {
    let mut diff = target_angle - heading;
    while diff > std::f32::consts::PI {
        diff -= std::f32::consts::TAU;
    }
    while diff < -std::f32::consts::PI {
        diff += std::f32::consts::TAU;
    }
    heading + diff.clamp(-turn_rate, turn_rate)
}

// ---------------------------------------------------------------------------
// State machine plumbing
// ---------------------------------------------------------------------------

/// Entry hook for state changes — the single place to attach animation
/// triggers, SFX, and VFX per transition.
pub fn on_state_enter(enemy: &mut Enemy) {
    use crate::model::AiState;
    match enemy.ai {
        AiState::Alert => {
            // Hook: play a "?" sting / perk-up animation here.
            enemy.alert_timer = 0;
        }
        AiState::Chase => {
            // Hook: play an aggro bark / "!" flash here.
        }
        AiState::Flee => {
            // Hook: panic animation.
        }
        _ => {}
    }
    crate::log_verbose!(
        "ai_state_enter type={:?} archetype={:?} state={:?}",
        enemy.enemy_type,
        enemy.archetype,
        enemy.ai
    );
}
