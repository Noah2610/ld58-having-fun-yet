use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, handle_set_scale);
}

#[derive(Component, Reflect, Clone)]
#[reflect(Component)]
pub struct SetScale(pub Vec2);

impl From<Vec2> for SetScale {
    fn from(value: Vec2) -> Self {
        Self(value)
    }
}

fn handle_set_scale(
    mut commands: Commands,
    query: Query<(Entity, &SetScale, &mut Transform), Changed<SetScale>>,
) {
    for (entity, set_scale, mut transform) in query {
        transform.scale = Vec3::new(set_scale.0.x, set_scale.0.y, transform.scale.z);
        commands.entity(entity).remove::<SetScale>();
    }
}
