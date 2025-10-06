use crate::{AppSystems, GameplaySet};
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        sync_alive_dead
            .in_set(AppSystems::Update)
            .in_set(GameplaySet),
    );
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
#[require(Alive)]
pub struct Health(u32);

#[derive(Component, Reflect, Clone, Copy, Default)]
#[reflect(Component)]
pub struct Alive;

#[derive(Component, Reflect, Clone, Copy, Default)]
#[reflect(Component)]
pub struct Dead;

impl Health {
    pub fn new(health: u32) -> Self {
        Self(health)
    }

    pub fn is_alive(&self) -> bool {
        self.0 > 0
    }

    pub fn damage(&mut self, amount: u32) {
        self.0 = self.0.checked_sub(amount).unwrap_or(0);
    }
}

fn sync_alive_dead(
    mut commands: Commands,
    alive_query: Query<(Entity, &Health), (Changed<Health>, With<Alive>, Without<Dead>)>,
    dead_query: Query<(Entity, &Health), (Changed<Health>, With<Dead>, Without<Alive>)>,
) {
    for (entity, health) in alive_query {
        if !health.is_alive() {
            commands.entity(entity).remove::<Alive>().insert(Dead);
        }
    }

    for (entity, health) in dead_query {
        if health.is_alive() {
            commands.entity(entity).remove::<Dead>().insert(Alive);
        }
    }
}
