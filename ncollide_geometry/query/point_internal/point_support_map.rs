use na::{self, Translation, Rotate, Transform};
use query::algorithms::gjk;
use query::algorithms::minkowski_sampling;
use query::algorithms::simplex::Simplex;
use query::algorithms::johnson_simplex::JohnsonSimplex;
use query::{PointQuery, PointProjection};
use shape::{SupportMap, Cylinder, Cone, Capsule, ConvexHull};
use math::{Point, Vector};

/// Projects a point on a shape using the GJK algorithm.
pub fn support_map_point_projection<P, M, S, G>(m:       &M,
                                                shape:   &G,
                                                simplex: &mut S,
                                                point:   &P,
                                                solid:   bool)
                                                -> PointProjection<P>
    where P: Point,
          M: Translation<P::Vect>,
          S: Simplex<P>,
          G: SupportMap<P, M> {
    let m = na::append_translation(m, &-*point.as_vector());

    let support_point = shape.support_point(&m, &-*point.as_vector());

    simplex.reset(support_point);

    match gjk::project_origin(&m, shape, simplex) {
        Some(p) => {
            PointProjection::new(false, p + *point.as_vector())
        },
        None => {
            let proj;

            // Fallback algorithm.
            // FIXME: we use the Minkowski Sampling for now, but this should be changed for the EPA
            // in the future.
            if !solid {
                match minkowski_sampling::project_origin(&m, shape, simplex) {
                    Some(p) => proj = p + *point.as_vector(),
                    None    => proj = point.clone()
                }
            }
            else {
                proj = point.clone()
            }

            PointProjection::new(true, proj)
        }
    }
}

impl<P, M> PointQuery<P, M> for Cylinder<<P::Vect as Vector>::Scalar>
    where P: Point,
          M: Transform<P> + Rotate<P::Vect> + Translation<P::Vect> {
    #[inline]
    fn project_point(&self, m: &M, point: &P, solid: bool) -> PointProjection<P> {
        support_map_point_projection(m, self, &mut JohnsonSimplex::<P>::new_w_tls(), point, solid)
    }
}

impl<P, M> PointQuery<P, M> for Cone<<P::Vect as Vector>::Scalar>
    where P: Point,
          M: Transform<P> + Rotate<P::Vect> + Translation<P::Vect> {
    #[inline]
    fn project_point(&self, m: &M, point: &P, solid: bool) -> PointProjection<P> {
        support_map_point_projection(m, self, &mut JohnsonSimplex::<P>::new_w_tls(), point, solid)
    }
}

impl<P, M> PointQuery<P, M> for Capsule<<P::Vect as Vector>::Scalar>
    where P: Point,
          M: Transform<P> + Rotate<P::Vect> + Translation<P::Vect> {
    #[inline]
    fn project_point(&self, m: &M, point: &P, solid: bool) -> PointProjection<P> {
        support_map_point_projection(m, self, &mut JohnsonSimplex::<P>::new_w_tls(), point, solid)
    }
}

impl<P, M> PointQuery<P, M> for ConvexHull<P>
    where P: Point,
          M: Transform<P> + Rotate<P::Vect> + Translation<P::Vect> {
    #[inline]
    fn project_point(&self, m: &M, point: &P, solid: bool) -> PointProjection<P> {
        support_map_point_projection(m, self, &mut JohnsonSimplex::<P>::new_w_tls(), point, solid)
    }
}
