use crate::{AppSystems, GameplaySet, game::player::Player};
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (sync_alive_dead, handle_dead_entities, despawn_entities)
            .chain()
            .in_set(AppSystems::Update)
            .in_set(GameplaySet),
    )
    .add_systems(
        Update,
        render_health.in_set(AppSystems::Update).in_set(GameplaySet),
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

#[derive(Component, Reflect, Clone, Copy, Default)]
#[reflect(Component)]
pub struct HealthValueUi;

impl Health {
    pub fn new(health: u32) -> Self {
        Self(health)
    }

    pub fn is_alive(&self) -> bool {
        self.0 > 0
    }

    pub fn damage(&mut self, amount: u32) {
        self.0 = self.0.saturating_sub(amount);
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

fn handle_dead_entities(time: Res<Time>, dead_query: Query<&mut Transform, With<Dead>>) {
    let dt = time.delta_secs();
    let delta = 1.0 * dt;
    for mut transform in dead_query {
        transform.scale = Vec3::new(
            transform.scale.x - delta,
            transform.scale.y - delta,
            transform.scale.z,
        )
        .max(Vec3::ZERO);
    }
}

fn despawn_entities(mut commands: Commands, dead_query: Query<(Entity, &Transform), With<Dead>>) {
    for (entity, transform) in dead_query {
        if transform.scale.x <= 0.0 || transform.scale.y <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

fn render_health(
    health: Single<&Health, (Changed<Health>, With<Player>)>,
    query: Query<&mut TextSpan, With<HealthValueUi>>,
) {
    for mut ui_text in query {
        ui_text.0 = health.0.to_string();
    }
}
