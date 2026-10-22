use crate::framebuffer::Framebuffer;

pub type Vertex = [f32; 2];

// Bresenham
pub fn line(fb: &mut Framebuffer, a: Vertex, b: Vertex) {
    let (mut x0, mut y0) = (a[0].round() as i32, a[1].round() as i32);
    let (x1, y1) = (b[0].round() as i32, b[1].round() as i32);
    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    loop {
        fb.point(x0, y0);
        if x0 == x1 && y0 == y1 {
            break;
        }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x0 += sx;
        }
        if e2 <= dx {
            err += dx;
            y0 += sy;
        }
    }
}

pub fn triangle(fb: &mut Framebuffer, a: Vertex, b: Vertex, c: Vertex) {
    line(fb, a, b);
    line(fb, b, c);
    line(fb, c, a);
}
