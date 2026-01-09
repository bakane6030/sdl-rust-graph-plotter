use sdl3::Error;
use sdl3::pixels::{Color};
use sdl3::render::{Canvas, FPoint, RenderTarget, Vertex};
use crate::vec::{vec2, Vec2};

pub trait CanvasExt {
    fn draw_thick_lines<'a, P: Into<&'a [Vec2]>>(&mut self, points: P, thickness: f32, color: Color) -> Result<(), sdl3::Error>;
}

impl <T: RenderTarget> CanvasExt for Canvas<T> {
    fn draw_thick_lines<'a, P: Into<&'a [Vec2]>>(&mut self, points: P, thickness: f32, color: Color) -> Result<(), Error> {
        let p = points.into();
        let half_thickness = thickness / 2.0;
        let fcolor = color.into();

        self.set_draw_color(color);

        let mut iter = p.windows(2);
        while let Some([p1, p2]) = iter.next() {
            let angle = f32::atan2(p2.y - p1.y, p2.x - p1.x);
            let dx = half_thickness * angle.sin();
            let dy = -half_thickness * angle.cos();

            let vertices = [
                vec2(p1.x - dx, p1.y - dy),
                vec2(p1.x + dx, p1.y + dy),
                vec2(p2.x - dx, p2.y - dy),
                vec2(p2.x + dx, p2.y + dy),
            ].map(|v| Vertex {
                position: v.into(),
                color: fcolor,
                tex_coord: FPoint::new(0.0, 0.0)
            });

            self.render_geometry(&vertices, None, &[0, 1, 2, 1, 2, 3])?
        }

        Ok(())
    }
}
