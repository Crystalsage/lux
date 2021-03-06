// A very simple ray tracer by Bourbon
// Assume BSD-style license
//
// The images in the 'renders' folder are results of experiments with the
// camera's position (`camerapos` variable in `Render` function).
//
// ----------------------------------------------------------------------------
//
// PRIM is for a primitive (Sphere here)

use image::{ImageBuffer, RgbaImage};

// Global constants
// `u32` for compatibility with 
const RESX: u32 = 1920;
const RESY: u32 = 1080;
const MAXPRIMCOUNT: u32 = 64;
const MAXLIGHTCOUNT: u32 = 10;
const MAXTHREADS: u32 = 4;

// Three dimensional vector
#[derive(Clone, Copy)]
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

    // Clone `original_vector` into new vector `self`
    // original_vector <---- self
    // This is only because I was too lazy to implement `Clone` or `Copy`.
    fn _v3d_clone_from(&mut self, original_vector: Vector3D) {
        self.x = original_vector.x;
        self.y = original_vector.y;
        self.z = original_vector.z;
    }

    // Print vector
    // We could use `impl Display` but hey! :)
    fn _v3d_print(&self) {
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
    fn v3d_dot_mul(&self, other_vector: Vector3D) -> f64 {
        self.x * other_vector.x + self.y * other_vector.y + self.z * other_vector.z
    }

    // Cross multiplication
    fn _v3d_cross_mul(&mut self, b: Vector3D, c: Vector3D) {
        self.x = b.y * c.z - b.z * c.y;
        self.y = b.z * c.x - b.x * c.z;
        self.z = b.x * c.y - b.y * c.x;
    }

    // Length of vector
    fn v3d_length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    // Length of but without sqrt
    fn _v3d_length_sqr(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
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
#[derive(Clone, Copy)]
struct Material {
    specular: f64,
    diffusive: f64,
    reflective: f64,
    color: Vector3D,
}

// Sphere
#[derive(Copy, Clone)]
struct PrimSphere {
    position: Vector3D,
    radius: f64,
    m: Material,
}

// Light ray
#[derive(Copy, Clone)]
struct Ray {
    direction: Vector3D,
    origin: Vector3D,
}

#[derive(Copy, Clone)]
// Light source
struct Light {
    position: Vector3D,
    color: Vector3D,
}

// Global struct
#[derive(Clone)]
struct GlobalSettings {
    img: RgbaImage,

    primitive_count: u32,
    primitive_list: Vec<PrimSphere>,

    light_count: u32,
    light_list: Vec<Light>,
}

// Methods for `Prim` 
impl PrimSphere{
	fn normal(&self, pos: Vector3D) -> Vector3D {
		let mut ret = pos;
		ret.v3d_sub(self.position);
		let f = (1.0 / self.radius) as f64;
		ret.v3d_mul_scalar(f);
		ret.v3d_norm();

		return ret;
	}

	fn intersect(&self, ray: Ray, dist: &mut f64) -> i32 {
		let mut v_precalc = ray.origin;
		v_precalc.v3d_sub(self.position);

		let det_precalc: f64 = self.radius * self.radius - v_precalc.v3d_dot_mul(v_precalc);

		let b = - v_precalc.v3d_dot_mul(ray.direction);
		let mut det = b*b + det_precalc;

		let mut retval: i32 = 0;

		if det > 0.0 {
			det = det.sqrt();
			let i1 = b - det;
			let i2 = b + det;

			if i2 > 0.0 && i1 < 0.0 {
				retval = -1;
				*dist = i2;
			} else if i2 > 0.0 && i1 >= 0.0 {
				retval = 1;
				*dist = i1;
			}
		}

		return retval;
	}
}

// Spawn a sphere at the specified position `pos`.
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

// Spawn a light source at the specified position `pos`. Call this less than 
// the `MAXLIGHTCOUNT` times in `main` to add more light sources.
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

// Trace the ray
fn trace(ray: &Ray, refl_depth: u32, globals: &mut GlobalSettings) -> Vector3D{
    let mut color: Vector3D = Vector3D::v3d_new((0.02, 0.1, 0.17));

    let mut dist: f64 = 1000000000.0;
    let mut prim: Option<PrimSphere> = None;

    // Make the ray bounce off every sphere 
    for i in 0..globals.primitive_count {
        let mut temp_dist: f64 = 0.0;
        let p = globals.primitive_list[i as usize];

        let res = p.intersect(*ray, &mut temp_dist);

        if res == 0 {
            continue;
        }

        if temp_dist < dist {
            prim = Some(p);
            dist = temp_dist;
            // result = ret;
        }
    }

    match prim {
        Some(_) => {},
        None => {
            let ret_vector = Vector3D{
                x: color.x,
                y: color.y,
                z: color.z,
            };

            return ret_vector;
        }
    }

    let prim = prim.unwrap();

    let mut pi = ray.direction;
    pi.v3d_mul_scalar(dist);
    pi.v3d_add(ray.origin);

    let prim_color: Vector3D = prim.m.color;

    for i in 0..globals.light_count {
        let light_iter = globals.light_list[i as usize];

        let mut l: Vector3D = light_iter.position;
        l.v3d_sub(pi);
        l.v3d_norm();

        let n: Vector3D = prim.normal(pi);
        
        if prim.m.diffusive > 0.0 {
            let dot = l.v3d_dot_mul(n);
            if dot > 0.0 {
                let diff = dot * prim.m.diffusive;

                //color += ((lightiter)->Color * prim_color) * diff;
                let mut color_add = light_iter.color;
                color_add.v3d_mul_v3d(prim_color);
                color_add.v3d_mul_scalar(diff);
                color.v3d_add(color_add);

            }
        }

        if prim.m.specular > 0.0 {
            //FIXME: Maybe this is messed up.

            // R = L -  N * L.Dot(N) * 2.0l;
            let mut r1: Vector3D = n;
            let r2: f64 = l.v3d_dot_mul(n) * 2.0;
            r1.v3d_mul_scalar(r2);
            let mut r: Vector3D = l;
            r.v3d_sub(r1);

            let mut dot: f64 = ray.direction.v3d_dot_mul(r);
            if dot > 0.0 {
                dot *= dot;
                dot *= dot;
                dot *= dot;
                let spec = dot * prim.m.specular;

                let mut color_add = light_iter.color;
                color_add.v3d_mul_scalar(spec);
                color.v3d_add(color_add);
            }
        }

        let refl = prim.m.reflective;
        if refl > 0.0 && refl_depth < 4 {
            prim.normal(pi);


            let mut r: Vector3D = ray.direction;
            let mut r1: Vector3D = n;

            let r2 = ray.direction.v3d_dot_mul(n) * 2.0;
            r1.v3d_mul_scalar(r2);
            r.v3d_sub(r1);

            // newpi = pi + r * 0.0001
            let mut newpi: Vector3D = r;
            newpi.v3d_mul_scalar(0.0001);
            newpi.v3d_add(pi);

            let tempr: Ray = Ray {
                origin: newpi,
                direction: r,
            };

            let mut rcol:Vector3D = trace(&tempr, refl_depth +1, globals);

            rcol.v3d_mul_scalar(refl);
            rcol.v3d_mul_v3d(prim_color);
            color.v3d_add(rcol);
        }
    }

    let ret_vector: Vector3D = Vector3D {
        x: color.x,
        y: color.y,
        z: color.z,
    };

    return ret_vector;
}

// Boss function
fn render(thread_id: u32, globals: &mut GlobalSettings) {
    let camerapos = Vector3D::v3d_new((0.0, 0.0, -5.0));

    let wx1: f64 = -2.0; let wx2: f64 = 2.0; 
    let wy1: f64 = 1.5; let wy2: f64 = -1.5;

    let dx: f64 = (wx2 - wx1) as f64 / (globals.img.width()) as f64;
    let dy: f64 = (wy2 - wy1) as f64 / (globals.img.height()) as f64;

    let mut sy: f64 = wy1 + dy * (thread_id as f64);

    // Spawn rays
    for y in (thread_id..globals.img.height()).step_by(MAXTHREADS as usize) {
        let mut sx = wx1;

        for x in 0..globals.img.width() {
            let camera_target = Vector3D::v3d_new((sx, sy, 0.0));

            let mut ray = Ray {
                origin: camerapos,
                direction: camera_target,
            };

            ray.direction.v3d_sub(ray.origin);
            ray.direction.v3d_norm();

            // Trace the ray
            let color: Vector3D = trace(&ray, 0, globals);


            // Normalize colors
            let r: i32 = (color.x * 255.0) as i32;
            let g: i32 = (color.y * 255.0) as i32;
            let b: i32 = (color.z * 255.0) as i32;

            let r: u8 = match r {
                r if r > 255 => 255,
                r if r < 0 => 0,
                _ => r as u8,
            };
            let g: u8 = match g {
                g if g > 255 => 255,
                g if g < 0 => 0,
                _ => g as u8,
            };
            let b: u8 = match b {
                b if b > 255 => 255,
                b if b < 0 => 0,
                _ => b as u8,
            };

            let cl: image::Rgba<u8> = image::Rgba([r, g, b, 255]);
            globals.img.put_pixel(x, y, cl);

            sx += dx;
        }

        sy += dy * (MAXTHREADS as f64);
    }
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

    // Set three materials here.
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

    // FIXME: Use a single `Vec<char>` here, maybe
    // All this because Rust can't index into strings :)))))))))))
    // ----
    // Position of spheres in the image. Edit this to alter the sphere pattern.
    // But make sure that you enter 9 x 6 pattern. Otherwise, change the 
    // `j` and `i` in the iterator below for a custom map.
    let mut sphere_pos_map: Vec<Vec<char>> = Vec::new();
    sphere_pos_map.push("g..g..r..".chars().collect());
    sphere_pos_map.push("g..g.....".chars().collect());
    sphere_pos_map.push("gggg..r..".chars().collect());
    sphere_pos_map.push("g..g..r..".chars().collect());
    sphere_pos_map.push("g..g..r..".chars().collect());
    sphere_pos_map.push("......r..".chars().collect());

    // Place colored spheres here and there.
    for j in 0..6 {
        for i in 0..9 {
            let mut m = &mirror;
            let mut z = 2.0_f64;
            let sn = ((i + j) as f64).sin() * 0.8;

            match sphere_pos_map[j][i] {
                'g' => {
                    z += -0.5 /*- sn * 0.4 */;
                    m = &green;
                },
                'r' => {
                    z += -0.5 /* - sn * 0.4 */;
                    m = &red;
                },

                _ => {
                    z += sn;
                },
            }

            let sphere_pos = Vector3D::v3d_new((-2.0 + (i as f64) * 0.5, 1.25 - (j as f64) * 0.5, z));
            add_sphere(&sphere_pos, 0.25, m, &mut globals);
        }
    }

    // Add a single light source  
    let lightpos: Vector3D = Vector3D::v3d_new((0.0, 0.0, 0.0));
    let lightcolor: Vector3D = Vector3D::v3d_new((2.0, 2.0, 2.0));
    add_light(lightpos, lightcolor, &mut globals);

    println!("Rendering...\n");

    // Simulating 4 threads. Each 'thread' (call) completes a part of the image. 
    // FIXME: Actually implement threads! :')
    render(0, &mut globals);
    render(1, &mut globals);
    render(2, &mut globals);
    render(3, &mut globals);

    println!("Writing test.png image...");
    globals.img.save("test.png").unwrap();
}
