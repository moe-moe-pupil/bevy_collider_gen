use bevy::{prelude::*, sprite::Anchor};

use crate::prelude::ColliderType;

#[derive(Component, Clone, Copy, Debug, Default, Deref, DerefMut)]
pub struct DynamicCollider(pub ColliderType);

#[derive(Component, Debug, Default, Clone, Reflect)]
#[reflect(Component, Default, Debug)]
pub struct ColliderSource {
    pub image: Handle<Image>,
    pub texture_atlas: Option<TextureAtlas>,
    pub custom_size: Option<Vec2>,
    pub rect: Option<Rect>,
    pub anchor: Anchor,
    pub flip: BVec2,
}

pub(crate) type Fields<'a> = (
    &'a Handle<Image>,
    Option<&'a TextureAtlas>,
    Option<Vec2>,
    Option<Rect>,
    Anchor,
    BVec2,
);

impl ColliderSource {
    #[must_use]
    pub fn sized(custom_size: Vec2) -> Self {
        ColliderSource {
            custom_size: Some(custom_size),
            ..Default::default()
        }
    }

    #[must_use]
    pub fn from_image(image: Handle<Image>) -> Self {
        Self {
            image,
            ..Default::default()
        }
    }

    #[must_use]
    pub fn from_atlas_image(image: Handle<Image>, atlas: TextureAtlas) -> Self {
        Self {
            image,
            texture_atlas: Some(atlas),
            ..Default::default()
        }
    }

    #[must_use]
    pub fn as_tuple(&self) -> Fields {
        (
            &self.image,
            self.texture_atlas.as_ref(),
            self.custom_size,
            self.rect,
            self.anchor,
            self.flip,
        )
    }
}

impl From<Handle<Image>> for ColliderSource {
    fn from(image: Handle<Image>) -> Self {
        Self::from_image(image)
    }
}
