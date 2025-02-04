#![allow(clippy::needless_pass_by_value)]
use bevy::prelude::*;
use edges::binary_image::BinaryImage;

use super::{
    utils::{as_tuple, process_image},
    ColliderSource, DynamicCollider,
};
use crate::prelude::{AbstractCollider, AbstractCollidersBuilder};

pub fn add_colliders<Target>(
    mut commands: Commands,
    targets: Query<(
        Entity,
        &DynamicCollider,
        Option<&ColliderSource>,
        Option<&Sprite>,
    )>,
    images: Res<Assets<Image>>,
    layouts: Res<Assets<TextureAtlasLayout>>,
) where
    AbstractCollider: Into<Option<Target>>,
    Target: Component,
{
    for (entity, collider_type, source, sprite) in &targets {
        let Some(mut target) = commands.get_entity(entity) else {
            continue;
        };
        let Some(image) = source
            .map(|source| source.as_tuple())
            .or_else(|| sprite.map(as_tuple))
            .and_then(|(id, atlas, size, rect, anchor, flip)| {
                images
                    .get(id)
                    .and_then(|image| BinaryImage::try_from(image).ok())
                    .and_then(|image| {
                        process_image(
                            image,
                            atlas.and_then(|atlas| atlas.texture_rect(&layouts)),
                            size,
                            rect,
                            anchor,
                            flip,
                        )
                    })
            })
        else {
            continue;
        };

        if let Some(collider) = AbstractCollidersBuilder::new(image)
            .with_type(**collider_type)
            .single()
        {
            if let Some(collider) = Into::<Option<Target>>::into(collider) {
                target.insert_if_new(collider);
            }
        }
    }
}
