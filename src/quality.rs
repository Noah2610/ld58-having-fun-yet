use crate::camera::MainCamera;
use bevy::{
    post_process::bloom::{Bloom, BloomCompositeMode},
    prelude::*,
    render::view::Hdr,
};

pub(super) fn plugin(app: &mut App) {
    app.init_state::<Quality>();
    app.add_systems(Update, set_quality.run_if(state_changed::<Quality>));
}

#[derive(States, Reflect, Clone, Copy, PartialEq, Eq, Hash, Default, Debug)]
#[reflect(State)]
pub enum Quality {
    None,
    Low,
    High,
    Ultra,
    Vert,
    Horz,
    VertBig,
    HorzBig,
    #[default]
    Natural,
    Anamorphic,
    OldSchool,
    ScreenBlur,
}

fn set_quality(
    mut cmd: Commands,
    quality: Res<State<Quality>>,
    query: Single<Entity, With<MainCamera>>,
) {
    use Quality::*;
    match quality.get() {
        None => set_quality_none(cmd, query),
        Low => set_quality_low(cmd, query),
        High => set_quality_high(cmd, query),
        Ultra => set_quality_ultra(cmd, query),
        Vert => {
            cmd.entity(query.entity()).insert(Bloom {
                intensity: 0.3,
                scale: Vec2::new(10.0, 20.0),
                max_mip_dimension: 2048,
                low_frequency_boost: 0.2,
                composite_mode: BloomCompositeMode::Additive,
                high_pass_frequency: 4.0,
                ..Bloom::ANAMORPHIC
            });
        },
        HorzBig => {
            cmd.entity(query.entity()).insert(Bloom {
                intensity: 0.3,
                scale: Vec2::new(20.0, 10.0),
                max_mip_dimension: 2048,
                low_frequency_boost: 0.2,
                composite_mode: BloomCompositeMode::Additive,
                high_pass_frequency: 4.0,
                ..Bloom::ANAMORPHIC
            });
        },
        VertBig => {
            cmd.entity(query.entity()).insert(Bloom {
                intensity: 0.15,
                scale: Vec2::new(20.0, 100.0),
                max_mip_dimension: 2048,
                low_frequency_boost: 0.2,
                composite_mode: BloomCompositeMode::Additive,
                high_pass_frequency: 4.0,
                ..Bloom::ANAMORPHIC
            });
        },
        Horz => {
            cmd.entity(query.entity()).insert(Bloom {
                intensity: 0.15,
                scale: Vec2::new(100.0, 24.0),
                max_mip_dimension: 2048,
                low_frequency_boost: 0.2,
                composite_mode: BloomCompositeMode::Additive,
                high_pass_frequency: 4.0,
                ..Bloom::ANAMORPHIC
            });
        },
        Natural => set_bloom(Bloom::NATURAL, cmd, query),
        Anamorphic => set_bloom(Bloom::ANAMORPHIC, cmd, query),
        OldSchool => set_bloom(Bloom::OLD_SCHOOL, cmd, query),
        ScreenBlur => set_bloom(Bloom::SCREEN_BLUR, cmd, query),
    }
}

fn set_bloom(bloom: Bloom, mut commands: Commands, query: Single<Entity, With<MainCamera>>) {
    commands.entity(query.entity()).insert(bloom);
}

fn set_quality_none(mut commands: Commands, query: Single<Entity, With<MainCamera>>) {
    commands.entity(query.entity()).remove::<(Hdr, Bloom)>();
}

fn set_quality_low(mut commands: Commands, camera: Single<Entity, With<MainCamera>>) {
    commands.entity(camera.entity()).insert(Bloom {
        intensity: 0.5,
        low_frequency_boost: 0.8,
        // composite_mode: BloomCompositeMode::Additive,
        ..Bloom::NATURAL
    });
}

fn set_quality_high(mut commands: Commands, camera: Single<Entity, With<MainCamera>>) {
    commands.entity(camera.entity()).insert((Hdr, Bloom {
        intensity: 0.4,
        low_frequency_boost: 0.2,
        composite_mode: BloomCompositeMode::Additive,
        max_mip_dimension: 1024,
        ..Bloom::SCREEN_BLUR
    }));
}

fn set_quality_ultra(mut commands: Commands, camera: Single<Entity, With<MainCamera>>) {
    commands.entity(camera.entity()).insert(Bloom {
        intensity: 0.6,
        low_frequency_boost: 0.8,
        composite_mode: BloomCompositeMode::Additive,
        max_mip_dimension: 2048,
        ..Bloom::ANAMORPHIC
    });
}
