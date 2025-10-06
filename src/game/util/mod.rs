mod collision_tag;
mod fix_object_colliders;
mod set_scale;

use bevy::prelude::*;
pub use collision_tag::*;
pub use fix_object_colliders::*;
pub use set_scale::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((fix_object_colliders::plugin, set_scale::plugin));
}
