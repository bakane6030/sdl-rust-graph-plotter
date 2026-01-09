use glam::{vec2, Vec2};
use sdl3::pixels::Color;
use sdl3::render::{Canvas, FPoint, RenderTarget, Vertex};
use sdl3::Error;

pub trait CanvasExt {
    fn draw_thick_lines(
        &mut self,
        points: &[Vec2],
        thickness: f32,
        color: Color,
    ) -> Result<(), Error>;
}

impl<T: RenderTarget> CanvasExt for Canvas<T> {
    fn draw_thick_lines(
        &mut self,
        points: &[Vec2],
        thickness: f32,
        color: Color,
    ) -> Result<(), Error> {
        let half_thickness = thickness / 2.0;
        let fcolor = color.into();

        self.set_draw_color(color);

        let mut deltas = Vec::with_capacity(points.len() - 1);

        let mut iter = points.windows(2);
        while let Some([p1, p2]) = iter.next() {
            let angle = f32::atan2(p2.y - p1.y, p2.x - p1.x);
            let d = vec2(half_thickness * angle.sin(), -half_thickness * angle.cos());

            deltas.push(d);
            let vertices = [p1 - d, p1 + d, p2 - d, p2 + d].map(|v| Vertex {
                position: FPoint::new(v.x, v.y),
                color: fcolor,
                tex_coord: FPoint::new(0.0, 0.0),
            });

            self.render_geometry(&vertices, None, &[0u8, 1, 2, 1, 2, 3])?;
        }

        let mut iter = points.windows(3).enumerate();
        while let Some((i, [p1, p2, p3])) = iter.next() {
            let v1 = p1 - p2;
            let v2 = p3 - p2;

            let sign = v1.perp_dot(v2).signum();
            if sign == 0.0 {
                continue;
            }

            let vertices =
                [*p2, p2 + deltas[i] * -sign, p2 + deltas[i + 1] * -sign].map(|v| Vertex {
                    position: FPoint::new(v.x, v.y),
                    color: fcolor,
                    tex_coord: FPoint::new(0.0, 0.0),
                });

            self.render_geometry(&vertices, None, &[0u8, 1, 2])?;
        }

        Ok(())
    }
}
