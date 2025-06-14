use edges::{anchor::Anchor, binary_image::Bit, EdgesIter};
use image::GenericImageView;

use crate::{
    abstract_collider::AbstractCollider, collider_type::ColliderType, utils::heights_and_scale,
};

mod from;

/// A builder for creating colliders from an image.
#[derive(Clone, Debug)]
pub struct Builder<I: GenericImageView<Pixel = Bit>> {
    image: I,
    anchor: Anchor,
    collider_type: ColliderType,
}

impl<I: GenericImageView<Pixel = Bit>> Builder<I> {
    /// Creates a new `Builder` with the given image and a default anchor at the center.
    ///
    /// # Arguments
    ///
    /// * `image` - The binary image used for collider creation.
    ///
    /// # Returns
    ///
    /// A `Builder` instance initialized with the provided image.
    pub fn new(image: I) -> Self {
        Self {
            anchor: Anchor::Center(image.height(), image.width()),
            image,
            collider_type: ColliderType::default(),
        }
    }

    /// Returns a reference to the image used in the builder.
    ///
    /// # Returns
    ///
    /// A reference to the binary image.
    pub fn image(&self) -> &I {
        &self.image
    }

    /// Returns the current anchor of the builder.
    ///
    /// # Returns
    ///
    /// The anchor used for translating polygons.
    pub fn anchor(&self) -> Anchor {
        self.anchor
    }
    /// Sets a new anchor for the builder.
    ///
    /// # Arguments
    ///
    /// * `anchor` - The new anchor to be used.
    ///
    /// # Returns
    ///
    /// A new `Builder` instance with the updated anchor.
    #[must_use]
    pub fn with_anchor(self, anchor: Anchor) -> Self {
        Self { anchor, ..self }
    }
    #[must_use]
    pub fn center(self, width: u32, height: u32) -> Self {
        self.with_anchor(Anchor::Center(height, width))
    }
    #[must_use]
    pub fn horisontal(self, width: u32) -> Self {
        self.with_anchor(Anchor::HorisontalCenter(width))
    }
    #[must_use]
    pub fn vertical(self, height: u32) -> Self {
        self.with_anchor(Anchor::VerticalCenter(height))
    }
    #[must_use]
    pub fn absolute(self) -> Self {
        self.with_anchor(Anchor::AbsoluteCenter)
    }

    /// Returns the current collider type of the builder.
    ///
    /// # Returns
    ///
    /// The collider type.
    pub fn collider_type(&self) -> ColliderType {
        self.collider_type
    }

    /// Sets a new collider type for the builder.
    ///
    /// # Arguments
    ///
    /// * `collider_type` - The new collider type to be used.
    ///
    /// # Returns
    ///
    /// A new `Builder` instance with the updated collider type.
    #[must_use]
    pub fn with_type(self, collider_type: ColliderType) -> Self {
        Self {
            collider_type,
            ..self
        }
    }
    #[must_use]
    pub fn polyline(self) -> Self {
        self.with_type(ColliderType::Polyline)
    }
    #[must_use]
    pub fn convex_polyline(self) -> Self {
        self.with_type(ColliderType::ConvexPolyline)
    }
    #[must_use]
    pub fn convex_hull(self) -> Self {
        self.with_type(ColliderType::ConvexHull)
    }
    #[must_use]
    pub fn heightfield(self) -> Self {
        self.with_type(ColliderType::Heightfield)
    }

    /// Generates multiple colliders based on the current builder's settings.
    #[must_use]
    pub fn multiple(&self) -> Vec<AbstractCollider> {
        let iter = EdgesIter::new(&self.image);
        if matches!(self.collider_type, ColliderType::Heightfield) {
            iter.map(|polygon| {
                let (heights, scale) = heights_and_scale(polygon, self.anchor);
                AbstractCollider::Heightfield(heights, scale)
            })
            .collect()
        } else {
            self.anchor
                .translate_polygons(iter)
                .into_iter()
                .map(match self.collider_type {
                    ColliderType::Polyline => AbstractCollider::Polyline,
                    ColliderType::ConvexPolyline => AbstractCollider::ConvexPolyline,
                    ColliderType::ConvexHull => AbstractCollider::ConvexHull,
                    ColliderType::Heightfield => unreachable!(),
                })
                .collect()
        }
    }
    /// Generates a single collider based on the current builder's settings.
    #[must_use]
    pub fn single(&self) -> Option<AbstractCollider> {
        let polygon = EdgesIter::new(&self.image).next();
        if matches!(self.collider_type, ColliderType::Heightfield) {
            polygon.map(|polygon| {
                let (heights, scale) = heights_and_scale(polygon, self.anchor);
                AbstractCollider::Heightfield(heights, scale)
            })
        } else {
            polygon
                .map(|polygon| self.anchor.translate(polygon))
                .map(match self.collider_type {
                    ColliderType::Polyline => AbstractCollider::Polyline,
                    ColliderType::ConvexPolyline => AbstractCollider::ConvexPolyline,
                    ColliderType::ConvexHull => AbstractCollider::ConvexHull,
                    ColliderType::Heightfield => unreachable!(),
                })
        }
    }
}
