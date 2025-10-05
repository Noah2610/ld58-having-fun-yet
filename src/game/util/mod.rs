mod collision_tag;
mod fix_object_colliders;

use bevy::prelude::*;
pub use collision_tag::*;
pub use fix_object_colliders::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((collision_tag::plugin, fix_object_colliders::plugin));
}
