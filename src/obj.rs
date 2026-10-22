use std::fs;

pub struct Obj {
    pub vertices: Vec<[f32; 3]>,
    pub faces: Vec<Vec<usize>>, // indices (base 0) de los vertices de cada cara
}

impl Obj {
    pub fn load(path: &str) -> Obj {
        let text = fs::read_to_string(path).expect("no se pudo leer el obj");
        let mut vertices = Vec::new();
        let mut faces = Vec::new();

        for line in text.lines() {
            let mut parts = line.split_whitespace();
            match parts.next() {
                Some("v") => {
                    let v: Vec<f32> = parts.take(3).map(|n| n.parse().unwrap()).collect();
                    vertices.push([v[0], v[1], v[2]]);
                }
                Some("f") => {
                    // "f 1 2 3" o "f 1/1/1 2/2/2 3/3/3" -> nos quedamos con el indice del vertice
                    let face = parts
                        .map(|p| p.split('/').next().unwrap().parse::<usize>().unwrap() - 1)
                        .collect();
                    faces.push(face);
                }
                _ => {}
            }
        }

        Obj { vertices, faces }
    }
}
