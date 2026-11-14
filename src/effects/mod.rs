pub mod particles;

use bevy::prelude::*;
use particles::*;

pub struct EffectsPlugin;

impl Plugin for EffectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<SpawnParticlesMessage>()
            .add_message::<SpawnFloatingTextMessage>()
            .add_systems(
                Update,
                (
                    handle_particle_messages,
                    update_particles,
                    handle_floating_text_messages,
                    update_floating_text,
                ),
            );
    }
}
