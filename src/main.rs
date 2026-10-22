mod framebuffer;
mod obj;
mod triangle;

use framebuffer::Framebuffer;
use minifb::{Key, Window, WindowOptions};
use obj::Obj;
use triangle::{triangle, Vertex};

const SCREEN_WIDTH: usize = 800;
const SCREEN_HEIGHT: usize = 600;

// Pasa los vertices 3D a coordenadas de pantalla: inclina un poco el modelo,
// lo escala para que ocupe ~70% de la pantalla y lo centra.
fn to_screen(obj: &Obj) -> Vec<Vertex> {
    let tilt: f32 = 0.25; // rotacion en X para verlo un poco desde arriba/atras
    let (s, c) = tilt.sin_cos();
    let projected: Vec<Vertex> = obj
        .vertices
        .iter()
        .map(|v| [v[0], v[1] * c - v[2] * s])
        .collect();

    let (mut min_x, mut max_x, mut min_y, mut max_y) = (f32::MAX, f32::MIN, f32::MAX, f32::MIN);
    for p in &projected {
        min_x = min_x.min(p[0]);
        max_x = max_x.max(p[0]);
        min_y = min_y.min(p[1]);
        max_y = max_y.max(p[1]);
    }

    let scale = 0.7 * (SCREEN_WIDTH as f32 / (max_x - min_x)).min(SCREEN_HEIGHT as f32 / (max_y - min_y));
    let (cx, cy) = ((min_x + max_x) / 2.0, (min_y + max_y) / 2.0);

    projected
        .iter()
        .map(|p| {
            [
                SCREEN_WIDTH as f32 / 2.0 + (p[0] - cx) * scale,
                SCREEN_HEIGHT as f32 / 2.0 + (p[1] - cy) * scale,
            ]
        })
        .collect()
}

fn render(fb: &mut Framebuffer, obj: &Obj, verts: &[Vertex]) {
    for face in &obj.faces {
        // caras de mas de 3 vertices se parten en triangulos (abanico)
        for i in 1..face.len() - 1 {
            let a = verts[face[0]];
            let b = verts[face[i]];
            let c = verts[face[i + 1]];
            triangle(fb, a, b, c);
        }
    }
}

fn main() {
    let obj = Obj::load("assets/spaceship.obj");
    let verts = to_screen(&obj);

    let mut fb = Framebuffer::new(SCREEN_WIDTH, SCREEN_HEIGHT);
    fb.set_background_color(0x101018);

    let mut window = Window::new("Software Renderer", SCREEN_WIDTH, SCREEN_HEIGHT, WindowOptions::default())
        .expect("no se pudo abrir la ventana");
    window.set_target_fps(60);

    let mut saved = false;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        fb.clear();
        fb.set_current_color(0xFFFF00); // pueden cambiar los colores
        render(&mut fb, &obj, &verts);

        if !saved {
            fb.save_png("render.png");
            saved = true;
        }

        window.update_with_buffer(&fb.buffer, SCREEN_WIDTH, SCREEN_HEIGHT).unwrap();
    }
}
