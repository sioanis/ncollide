use na::Real;
use bounding_volume::{BoundingSphere, HasBoundingVolume};
use bounding_volume;
use shape::ConvexHull;
use math::Isometry;

impl<N: Real> HasBoundingVolume<N, BoundingSphere<N>> for ConvexHull<N> {
    #[inline]
    fn bounding_volume(&self, m: &Isometry<N>) -> BoundingSphere<N> {
        let (center, radius) = bounding_volume::point_cloud_bounding_sphere(self.points());

        BoundingSphere::new(m * center, radius)
    }
}
