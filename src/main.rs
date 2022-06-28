// A very simple ray tracer by Bourbon
// Assume BSD-style license

use std::thread;

use image::{ImageBuffer, RgbaImage};

// Global constants
// u32 for compatibility.
const RESX: u32 = 1024;
const RESY: u32 = 768;
const MAXPRIMCOUNT: u32 = 64;
const MAXLIGHTCOUNT: u32 = 10;
const MAXTHREADS: u32 = 4;

// Three dimensional vector
#[derive(Debug, Clone)]
struct Vector3D {
    x: f64,
    y: f64,
    z: f64,
}

// Methods on Vectors
impl Vector3D {
    // Instantiate vector
    fn v3d_new(coordinates: (f64, f64, f64)) -> Self {
        Self {
            x: coordinates.0,
            y: coordinates.1,
            z: coordinates.2,
        }
    }
    
    // Set coordinates on vector
    fn v3d_update(&mut self, coordinates: (f64, f64, f64)) {
        self.x = coordinates.0;
        self.y = coordinates.1;
        self.z = coordinates.2;
    }

    // Clone `original_vector` into new vector `self`
    // original_vector <---- self
    // This is only because I was too lazy to implement `Clone` or `Copy`.
    fn v3d_clone_from(&mut self, original_vector: Vector3D) {
        self.x = original_vector.x;
        self.y = original_vector.y;
        self.z = original_vector.z;
    }

    // Print vector
    // We could use `impl Display` but hey! :)
    fn v3d_print(&self) {
        println!("x: {} y: {} z: {}", self.x, self.y, self.z);
    }

    // Add to the vector 
    fn v3d_add(&mut self, other_vector: Vector3D) {
        self.x += other_vector.x;
        self.y += other_vector.y;
        self.z += other_vector.z;
    }

    // Subtract from the vector
    fn v3d_sub(&mut self, other_vector: Vector3D) {
        self.x -= other_vector.x;
        self.y -= other_vector.y;
        self.z -= other_vector.z;
    }

    // Multiply by a scalar
    fn v3d_mul_scalar(&mut self, scalar: f64) {
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar;
    }

    // Multiply by another vector
    fn v3d_mul_v3d(&mut self, other_vector: Vector3D) {
        self.x *= other_vector.x;
        self.y *= other_vector.y;
        self.z *= other_vector.z;
    }

    // Dot multiplication 
    fn v3d_dot_mul(&self, other_vector: Vector3D) -> f64{
        self.x * other_vector.x 
            + self.y * other_vector.y 
            + self.z * other_vector.z
    }

    // Cross multiplication
    fn v3d_cross_mul(&mut self, b: Vector3D, c: Vector3D) {
        self.x = b.y * c.z - b.z * c.y;
        self.y = b.z * c.x - b.x * c.z;
        self.z = b.x * c.y - b.y * c.x;
    }

    // Length of vector
    fn v3d_length(&self) -> f64 {
        (self.x * self.x  + self.y * self.y + self.z * self.z).sqrt()
    }

    // Length of but without sqrt
    fn v3d_length_sqr(&self) -> f64 {
        self.x * self.x  + self.y * self.y + self.z * self.z
    }

    // Normalise vector
    fn v3d_norm(&mut self) {
        let l: f64 = 1.0 / self.v3d_length();
        self.x *= l;
        self.y *= l;
        self.z *= l;
    }
}

// Material properties and color
#[derive(Clone)]
struct Material {
    specular: f64,
    diffusive: f64,
    reflective: f64,
    color: Vector3D,
}

// Sphere
struct PrimSphere {
    position: Vector3D,
    radius: f64,
    m: Material,
}

// Light ray
struct Ray {
    direction: Vector3D,
    origin: Vector3D,
}

// Light source
struct Light {
    position: Vector3D,
    color: Vector3D,
}


// Global struct
struct GlobalSettings {
    img: RgbaImage,

    primitive_count: u32,
    primitive_list: Vec<PrimSphere>,

    light_count: u32,
    light_list: Vec<Light>,
}

fn add_sphere(pos: &Vector3D, rad: f64, m: &Material, globals: &mut GlobalSettings) {
    if globals.primitive_count < MAXPRIMCOUNT {
        let p = PrimSphere {
            position: Vector3D::v3d_new((pos.x, pos.y, pos.z)),
            radius: rad,
            m: m.clone(),
        };

        globals.primitive_list.push(p);
        globals.primitive_count += 1;
    }
}

fn add_light(pos: Vector3D, color: Vector3D, globals: &mut GlobalSettings) {
    if globals.light_count < MAXLIGHTCOUNT {
        let l = Light {
            position: Vector3D::v3d_new((pos.x, pos.y, pos.z)),
            color: Vector3D::v3d_new((color.x, color.y, color.z)),
        };

        globals.light_list.push(l);
        globals.light_count += 1;
    }
}

fn render(img: &RgbaImage, thread_id: u32, c: u32) {
    todo!()
} 


fn main() {
    println!("Simple ray tracer by Bourbon! :)");
    println!("Creating scene...\n");

    let img: RgbaImage = ImageBuffer::new(RESX, RESY);
    let primitive_list: Vec<PrimSphere> = Vec::new();
    let light_list: Vec<Light> = Vec::new();

    let mut globals: GlobalSettings = GlobalSettings { 
        img,
        primitive_count: 0, 
        primitive_list,
        light_count: 0, 
        light_list,
    };

    let mirror = Material {
        color: Vector3D::v3d_new((0.6, 0.6, 0.6)),
        specular: 0.3,
        diffusive: 0.2,
        reflective: 0.8,
    };

    let green = Material {
        color: Vector3D::v3d_new((0.1, 1.0, 0.1)),
        specular: 0.1,
        diffusive: 0.3,
        reflective: 0.4,
    };

    let red = Material {
        color: Vector3D::v3d_new((1.0, 0.1, 0.1)),
        specular: 0.1,
        diffusive: 0.3,
        reflective: 0.4,
    };


    // Use a single `Vec<char>` here, maybe
    // All this because Rust can't index into strings :)))))))))))
    let mut sphere_pos_map: Vec<Vec<char>> = Vec::new();
    sphere_pos_map.push(".........".chars().collect());
    sphere_pos_map.push(".ggg.....".chars().collect());
    sphere_pos_map.push(".g...rrr.".chars().collect());
    sphere_pos_map.push(".g.g.r.r.".chars().collect());
    sphere_pos_map.push(".ggg.rrr.".chars().collect());
    sphere_pos_map.push(".........".chars().collect());
    
    let mut sphere_pos: Vector3D = Vector3D { x: 0.0, y: 0.0, z: 0.0 };

    for j in 0..6 {
        for i in 0..9 {
            let mut m = &mirror;
            let mut z = 2.0_f64;
            let sn = ((i + j) as f64).sin() * 0.8;

            match sphere_pos_map[j][i] {
                'g' => {
                    z += -0.5;
                    m = &green; 
                },
                'r' => {
                    z += -0.5;
                    m = &red;
                }

                _ => {
                    z += sn;
                }
            }

            sphere_pos.v3d_update( (-2.0 + (i as f64) * 0.5, 1.25 - (j as f64) * 0.5, z));
            add_sphere(&sphere_pos, 0.25, m, &mut globals);
        }
    }

    let lightpos: Vector3D = Vector3D::v3d_new((0.0, 0.0, 0.0));
    let lightcolor: Vector3D = Vector3D::v3d_new((2.0, 2.0, 2.0));
    add_light(lightpos, lightcolor, &mut globals);

    println!("Rendering...\n");


    // TODO: What is this
    let c = 0;

    for i in 0..MAXTHREADS {
        println!("Thread {} started", i);
        thread::spawn(move || {
            render(&globals.img, i, c);
        });
    }
}
