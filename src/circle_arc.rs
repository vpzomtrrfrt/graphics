//! Draw an arc

use { triangulation, DrawState, Graphics };
use math::Matrix2d;

use types::{ Color, Radius, Rectangle, Resolution, Scalar };

/// A curved line
#[derive(Copy, Clone)]
pub struct CircleArc {

    /// The arcs color
    pub color: Color,

    /// The radius of the arc (Thickness of the drawing, not the radius of the circle)
    pub radius: Radius,

    /// The start of the arc in radians
    pub start: Scalar,

    /// The end of the arc in radians
    pub end: Scalar,

    /// The resolution for the arc.
    pub resolution: Resolution,
}

impl CircleArc {
    /// Creates a new arc
    pub fn new(color: Color, radius: Radius, start: Scalar, end: Scalar) -> CircleArc {
        CircleArc {
            color: color,
            radius: radius,
            start: start,
            end: end,
            resolution: 128,
        }
    }

    /// Sets the arcs color.
    pub fn color(mut self, value: Color) -> Self {
        self.color = value;
        self
    }

    /// Sets the radius of the arc (Thickness of the arc, not the radius of the circle it wraps)
    pub fn radius(mut self, value: Radius) -> Self {
        self.radius = value;
        self
    }

    /// Sets the start of the arc (in radians).
    pub fn start(mut self, value: Scalar) -> Self {
        self.start = value;
        self
    }

    /// Sets the end of the arc (in radians).
    pub fn end(mut self, value: Scalar) -> Self {
        self.end = value;
        self
    }

    /// Sets the resolution of the arcs smoothness.
    pub fn resolution(mut self, value: Resolution) -> Self {
        self.resolution = value;
        self
    }

    /// Draws the arc.
    pub fn draw<R: Into<Rectangle>, G>(
        &self,
        rectangle: R,
        draw_state: &DrawState,
        transform: Matrix2d,
        g: &mut G
    )
        where G: Graphics
    {
        let rectangle = rectangle.into();
        g.tri_list(
            &draw_state,
            &self.color,
            |f|
        triangulation::with_arc_tri_list(
            self.start,
            self.end,
            self.resolution,
            transform,
            rectangle,
            self.radius,
            |vertices| f(vertices)
        ));
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_circle_arc() {
        let _arc = CircleArc::new([1.0; 4], 4.0, 0.0, ::std::f64::consts::PI)
            .color([0.0; 4])
            .radius(4.0)
            .start(::std::f64::consts::PI * 0.25)
            .end(::std::f64::consts::PI * 1.25);
    }
}