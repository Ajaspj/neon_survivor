use bevy::prelude::*;

#[derive(Resource)]
pub struct Score {
    pub current: u32,
    pub high_score: u32,
    pub combo: u32,
    pub combo_timer: f32,
}

impl Default for Score {
    fn default() -> Self {
        Self {
            current: 0,
            high_score: 0,
            combo: 0,
            combo_timer: 0.0,
        }
    }
}

impl Score {
    pub fn add(&mut self, amount: u32) {
        self.combo += 1;
        self.combo_timer = 2.5; // Combo window
        let multiplier = 1.0 + (self.combo as f32 * 0.1).min(2.0);
        let earned = (amount as f32 * multiplier) as u32;
        self.current += earned;
        if self.current > self.high_score {
            self.high_score = self.current;
        }
    }

    pub fn tick(&mut self, dt: f32) {
        if self.combo_timer > 0.0 {
            self.combo_timer -= dt;
            if self.combo_timer <= 0.0 {
                self.combo = 0;
            }
        }
    }
}

#[derive(Resource, Default)]
pub struct SurvivalTimer {
    pub seconds: f32,
}

impl SurvivalTimer {
    pub fn formatted(&self) -> String {
        let total_secs = self.seconds as u32;
        let mins = total_secs / 60;
        let secs = total_secs % 60;
        format!("{:02}:{:02}", mins, secs)
    }
}

#[derive(Resource)]
pub struct WaveManager {
    pub current_wave: u32,
    pub wave_timer: f32,
    pub wave_duration: f32,
    pub spawn_timer: f32,
    pub spawn_interval: f32,
    pub enemies_spawned_this_wave: u32,
    pub max_enemies_this_wave: u32,
    pub announcement_timer: f32,
    pub total_kills: u32,
}

impl Default for WaveManager {
    fn default() -> Self {
        Self {
            current_wave: 1,
            wave_timer: 0.0,
            wave_duration: 35.0,
            spawn_timer: 0.0,
            spawn_interval: 1.2,
            enemies_spawned_this_wave: 0,
            max_enemies_this_wave: 18,
            announcement_timer: 3.0,
            total_kills: 0,
        }
    }
}

impl WaveManager {
    pub fn advance_wave(&mut self) {
        self.current_wave += 1;
        self.wave_timer = 0.0;
        self.enemies_spawned_this_wave = 0;
        self.announcement_timer = 3.5;
        // Scale difficulty with each wave
        self.spawn_interval = (1.2 * 0.88f32.powi((self.current_wave - 1) as i32)).max(0.35);
        self.max_enemies_this_wave = 15 + self.current_wave * 10;
    }
}

#[derive(Resource)]
pub struct PlayerStats {
    pub health: f32,
    pub max_health: f32,
    pub level: u32,
    pub current_xp: u32,
    pub xp_to_next_level: u32,
    pub move_speed: f32,
    pub invincibility_timer: f32,
    pub magnet_radius: f32,
}

impl Default for PlayerStats {
    fn default() -> Self {
        Self {
            health: 100.0,
            max_health: 100.0,
            level: 1,
            current_xp: 0,
            xp_to_next_level: 30,
            move_speed: 340.0,
            invincibility_timer: 0.0,
            magnet_radius: 90.0,
        }
    }
}

impl PlayerStats {
    pub fn take_damage(&mut self, amount: f32) -> bool {
        if self.invincibility_timer > 0.0 {
            return false;
        }
        self.health = (self.health - amount).max(0.0);
        self.invincibility_timer = 0.75;
        true
    }

    #[allow(dead_code)]
    pub fn heal(&mut self, amount: f32) {
        self.health = (self.health + amount).min(self.max_health);
    }

    pub fn add_xp(&mut self, amount: u32) -> bool {
        self.current_xp += amount;
        if self.current_xp >= self.xp_to_next_level {
            self.current_xp -= self.xp_to_next_level;
            self.level += 1;
            // Exponential scaling for XP requirements
            self.xp_to_next_level = (self.xp_to_next_level as f32 * 1.35) as u32 + 15;
            true
        } else {
            false
        }
    }
}

#[derive(Resource)]
pub struct WeaponConfig {
    pub fire_rate: f32, // shots per second
    pub fire_timer: f32,
    pub projectile_speed: f32,
    pub projectile_damage: f32,
    pub projectile_count: u32,
    pub projectile_spread: f32,
    pub projectile_pierce: u32,
}

impl Default for WeaponConfig {
    fn default() -> Self {
        Self {
            fire_rate: 4.0,
            fire_timer: 0.0,
            projectile_speed: 680.0,
            projectile_damage: 28.0,
            projectile_count: 1,
            projectile_spread: 0.18,
            projectile_pierce: 1,
        }
    }
}

#[derive(Resource)]
pub struct ArenaBounds {
    pub half_width: f32,
    pub half_height: f32,
}

impl Default for ArenaBounds {
    fn default() -> Self {
        Self {
            half_width: 720.0,
            half_height: 420.0,
        }
    }
}

impl ArenaBounds {
    pub fn clamp(&self, pos: Vec2, margin: f32) -> Vec2 {
        Vec2::new(
            pos.x.clamp(-self.half_width + margin, self.half_width - margin),
            pos.y.clamp(-self.half_height + margin, self.half_height - margin),
        )
    }

    pub fn is_inside(&self, pos: Vec2, margin: f32) -> bool {
        pos.x >= -self.half_width - margin
            && pos.x <= self.half_width + margin
            && pos.y >= -self.half_height - margin
            && pos.y <= self.half_height + margin
    }
}

#[derive(Resource, Default)]
pub struct ScreenShake {
    pub trauma: f32,
}

impl ScreenShake {
    pub fn add(&mut self, amount: f32) {
        self.trauma = (self.trauma + amount).min(1.0);
    }

    pub fn tick(&mut self, dt: f32) {
        self.trauma = (self.trauma - dt * 2.5).max(0.0);
    }

    pub fn offset(&self) -> Vec2 {
        if self.trauma <= 0.0 {
            return Vec2::ZERO;
        }
        let shake = self.trauma * self.trauma * 16.0;
        // Pseudo-random offset based on trauma
        let angle = self.trauma * 87.3;
        Vec2::new(angle.cos() * shake, angle.sin() * shake)
    }
}

#[derive(Resource)]
pub struct GameSettings {
    pub sound_enabled: bool,
    pub music_enabled: bool,
    pub screen_shake_enabled: bool,
    pub master_volume: f32,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            sound_enabled: true,
            music_enabled: true,
            screen_shake_enabled: true,
            master_volume: 0.8,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UpgradeType {
    PlasmaBooster,
    TwinBlasters,
    PhotonAmplifier,
    HyperionThrusters,
    NaniteHull,
    RailgunPiercer,
    QuantumMagnet,
    HyperAccelerator,
}

#[derive(Clone, Debug)]
pub struct UpgradeCard {
    pub upgrade_type: UpgradeType,
    pub title: &'static str,
    pub category: &'static str,
    pub description: &'static str,
}

impl UpgradeType {
    pub fn card(self) -> UpgradeCard {
        match self {
            UpgradeType::PlasmaBooster => UpgradeCard {
                upgrade_type: self,
                title: "PLASMA REPEATER",
                category: "WEAPON MATRIX",
                description: "+25% Fire Rate and Faster Weapon Cycling",
            },
            UpgradeType::TwinBlasters => UpgradeCard {
                upgrade_type: self,
                title: "MULTI-BARREL ARRAY",
                category: "AMMUNITION",
                description: "+1 Energy Projectile per Volley with Spread",
            },
            UpgradeType::PhotonAmplifier => UpgradeCard {
                upgrade_type: self,
                title: "PHOTON OVERCHARGER",
                category: "ENERGY SYSTEM",
                description: "+30% Projectile Impact Damage",
            },
            UpgradeType::HyperionThrusters => UpgradeCard {
                upgrade_type: self,
                title: "IONIC THRUSTERS",
                category: "PROPULSION",
                description: "+18% Player Movement & Sprint Speed",
            },
            UpgradeType::NaniteHull => UpgradeCard {
                upgrade_type: self,
                title: "NANITE REPAIR MATRIX",
                category: "SURVIVABILITY",
                description: "+35 Max Health and Instant Full Restoration",
            },
            UpgradeType::RailgunPiercer => UpgradeCard {
                upgrade_type: self,
                title: "RAILGUN PENETRATOR",
                category: "BALLISTICS",
                description: "+1 Projectile Penetration Through Enemies",
            },
            UpgradeType::QuantumMagnet => UpgradeCard {
                upgrade_type: self,
                title: "QUANTUM HARVESTER",
                category: "UTILITY",
                description: "+55% XP Shard Gravitational Attraction Radius",
            },
            UpgradeType::HyperAccelerator => UpgradeCard {
                upgrade_type: self,
                title: "ACCELERATOR COIL",
                category: "WEAPON MATRIX",
                description: "+35% Projectile Velocity & Range",
            },
        }
    }

    pub fn apply(self, player_stats: &mut PlayerStats, weapon_config: &mut WeaponConfig) {
        match self {
            UpgradeType::PlasmaBooster => {
                weapon_config.fire_rate *= 1.25;
            }
            UpgradeType::TwinBlasters => {
                weapon_config.projectile_count += 1;
                weapon_config.projectile_spread += 0.08;
            }
            UpgradeType::PhotonAmplifier => {
                weapon_config.projectile_damage *= 1.30;
            }
            UpgradeType::HyperionThrusters => {
                player_stats.move_speed *= 1.18;
            }
            UpgradeType::NaniteHull => {
                player_stats.max_health += 35.0;
                player_stats.health = player_stats.max_health;
            }
            UpgradeType::RailgunPiercer => {
                weapon_config.projectile_pierce += 1;
            }
            UpgradeType::QuantumMagnet => {
                player_stats.magnet_radius *= 1.55;
            }
            UpgradeType::HyperAccelerator => {
                weapon_config.projectile_speed *= 1.35;
            }
        }
    }

    pub fn all() -> &'static [UpgradeType] {
        &[
            UpgradeType::PlasmaBooster,
            UpgradeType::TwinBlasters,
            UpgradeType::PhotonAmplifier,
            UpgradeType::HyperionThrusters,
            UpgradeType::NaniteHull,
            UpgradeType::RailgunPiercer,
            UpgradeType::QuantumMagnet,
            UpgradeType::HyperAccelerator,
        ]
    }
}

#[derive(Resource, Default)]
pub struct OfferedUpgrades {
    pub cards: Vec<UpgradeCard>,
}
