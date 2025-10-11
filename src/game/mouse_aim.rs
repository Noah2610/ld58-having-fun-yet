use crate::{
    CursorMoved,
    camera::MainCamera,
    direction::Direction,
    game::{aim::AimDirection, bullet::Bullet, player::Player},
    game_state::{AppSystems, GameplaySet},
    input::{ActionState, MouseAction},
};
use avian2d::prelude::LinearVelocity;
use bevy::{ecs::query::QuerySingleError, prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.init_state::<MouseAimEnabled>();

    app.add_systems(OnEnter(MouseAimEnabled(true)), enable_mouse_aim);
    app.add_systems(OnEnter(MouseAimEnabled(false)), disable_mouse_aim);

    app.add_systems(
        Update,
        handle_mouse_aim.run_if(mouse_aim_enabled), /* .in_set(GameplaySet)
                                                     * .in_set(AppSystems::Update), */
    );
}

fn mouse_aim_enabled(mouse_enabled: Res<State<MouseAimEnabled>>) -> bool {
    mouse_enabled.0
}

#[derive(States, Reflect, Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[reflect(State)]
pub struct MouseAimEnabled(pub bool);

impl FromWorld for MouseAimEnabled {
    fn from_world(_world: &mut World) -> Self {
        use std::env;
        Self(
            env::var("MOUSE_AIM")
                .ok()
                .map(|v| v.len() > 0 && v != "0" && v != "false")
                .unwrap_or_default(),
        )
    }
}

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
#[require(
    Name::new("Mouse Aim Crosshair"),
    Transform::from_translation(Vec3::new(0.0, 0.0, 3.0)),
    Sprite {
        color: Color::hsla(0.0, 0.0, 1.0, 0.5),
        custom_size: Some(Vec2::splat(16.0)),
        ..default()
    },
)]
struct MouseAimCrosshair;

fn enable_mouse_aim(
    mut commands: Commands,
    mut state: ResMut<NextState<MouseAimEnabled>>,
    crosshairs: Query<Entity, With<MouseAimCrosshair>>,
) {
    state.set(MouseAimEnabled(true));

    match crosshairs.single() {
        Ok(e) => e,
        Err(err) => {
            if let QuerySingleError::MultipleEntities(_) = err {
                for e in crosshairs.iter() {
                    commands.entity(e).despawn();
                }
            }

            commands.spawn(MouseAimCrosshair).id()
        },
    };
}

fn disable_mouse_aim(
    mut commands: Commands,
    mut state: ResMut<NextState<MouseAimEnabled>>,
    crosshairs: Query<Entity, With<MouseAimCrosshair>>,
) {
    state.set(MouseAimEnabled(false));
    for e in crosshairs.iter() {
        commands.entity(e).despawn();
    }
}

fn handle_mouse_aim(
    action_state: Single<&ActionState<MouseAction>, With<MainCamera>>,
    mut player_query: Single<
        (&Transform, &mut AimDirection),
        (With<Player>, Without<MouseAimCrosshair>),
    >,
    mut crosshair_query: Single<&mut Transform, (With<MouseAimCrosshair>, Without<Player>)>,
) {
    const CROSSHAIR_OFFSET: f32 = 32.0;
    const DEADZONE: f32 = 40.0;

    let pan_vec = action_state.axis_pair(&MouseAction::Move);
    if pan_vec.length_squared() > DEADZONE {
        if let Ok(direction) = Direction::try_from(pan_vec).map(Direction::opposite_y) {
            player_query.1.0 = Some(direction);
        }
    }

    let aim_direction = player_query.1.0.unwrap_or(Direction::Right);
    let player_pos = player_query.0.translation.truncate().extend(3.0);

    let z = crosshair_query.translation.z;
    crosshair_query.translation =
        (player_pos.truncate() + aim_direction.vec() * CROSSHAIR_OFFSET).extend(z);
}
