use bevy::{prelude::*, sprite::Anchor};
use edges::binary_image::BinaryImage;

use super::components::Fields;

pub fn as_tuple(sprite: &Sprite) -> Fields {
    (
        &sprite.image,
        sprite.texture_atlas.as_ref(),
        sprite.custom_size,
        sprite.rect,
        sprite.anchor,
        BVec2::new(sprite.flip_x, sprite.flip_y),
    )
}

pub fn process_image(
    image: BinaryImage,
    atlas_rect: Option<URect>,
    size: Option<Vec2>,
    rect: Option<Rect>,
    anchor: Anchor,
    flip: BVec2,
) -> Option<BinaryImage> {
    todo!()
}
