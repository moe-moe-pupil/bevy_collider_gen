use bevy::prelude::*;

use crate::prelude::AbstractCollider;
pub use components::{ColliderSource, DynamicCollider};
use systems::add_colliders;

pub mod components;
mod systems;
mod tilemap;
pub(crate) mod utils;

#[derive(Debug, Default)]
pub struct DynamicColliderPlugin<Target>(std::marker::PhantomData<Target>)
where
    Target: Component;

impl<Target> DynamicColliderPlugin<Target>
where
    Target: Component,
{
    #[must_use]
    pub fn new() -> DynamicColliderPlugin<Target> {
        DynamicColliderPlugin(std::marker::PhantomData)
    }
}

impl<Target> bevy::prelude::Plugin for DynamicColliderPlugin<Target>
where
    AbstractCollider: Into<Option<Target>>,
    Target: Component,
{
    fn build(&self, app: &mut App) {
        app.register_type::<ColliderSource>()
            .add_systems(Update, add_colliders::<Target>);
    }
}
