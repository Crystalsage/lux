// A very simple ray tracer by Bourbon
// Assume BSD-style license

use image::{ImageBuffer, RgbaImage};

// Global constants
// u32 for compatibility.
const RESX: u32 = 1024;
const RESY: u32 = 768;
const MAXPRIMCOUNT: u32 = 64;
const MAXLIGHTCOUNT: u32 = 10;
const MAXTHREADS: u32 = 4;

// Three dimensional vector
#[derive(Debug, Clone, Copy)]
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

#[derive(Clone, Debug)]
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

impl PrimSphere{
	fn normal(&self, pos: Vector3D) -> Vector3D {
		let mut ret = pos;
		ret.v3d_sub(self.position);
		let f = (1.0 / self.radius) as f64;
		ret.v3d_mul_scalar(f);
		ret.v3d_norm();

		return ret;
	}

	fn intersect(&self, ray: Ray, dist: f64) -> u32 {
		let mut v_precalc = ray.origin;
		v_precalc.v3d_sub(self.position);

		let mut dist = dist;

		let det_precalc: f64 = self.radius * self.radius - v_precalc.v3d_dot_mul(v_precalc);

		let b = - v_precalc.v3d_dot_mul(ray.direction);
		let mut det = b*b + det_precalc;

		let mut retval: u32 = 0;

		if det > 0.0 {
			det = det.sqrt();
			let i1 = b - det;
			let i2 = b + det;

			if i2 > 0.0 && i1 < 0.0 {
				retval = 1;
				dist = i2;
			} else if i2 > 0.0 && i1 >= 0.0 {
				retval = 1;
				dist = i1;
			}
		}

		return retval;
	}
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
    dbg!(globals.light_count);
    println!("{:?}", globals.light_list);
}

fn trace(ray: Ray, refl_depth: u32, globals: &mut GlobalSettings) -> Vector3D{
    let mut color: Vector3D = Vector3D::v3d_new((0.02, 0.1, 0.17));

		let mut dist: f64 = 1000000000.0;
		let mut prim: Option<PrimSphere> = None;

		for i in 0..globals.primitive_count {
			let temp_dist: f64 = 0.0;
			let p = globals.primitive_list[i as usize];

			let res = p.intersect(ray, temp_dist);

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

		let mut pi = Vector3D::v3d_new((ray.direction.x, ray.direction.y, ray.direction.z));
		pi.v3d_mul_scalar(dist);
		pi.v3d_add(ray.origin);

		let prim_color: Vector3D = prim.m.color;

		for i in 0..globals.light_count {
			let light_iter = &globals.light_list[i as usize];

			let mut l: Vector3D = light_iter.position;
			l.v3d_sub(pi);
			l.v3d_norm();

			let n: Vector3D = prim.normal(pi);
			
			if prim.m.diffusive > 0.0 {
				let dot = l.v3d_dot_mul(n);
				if dot > 0.0 {
					let diff = dot * prim.m.diffusive;

					let mut color_add = light_iter.color;
					color_add.v3d_mul_v3d(prim_color);
					color_add.v3d_mul_scalar(diff);
					color.v3d_add(color_add);

					//color += ((lightiter)->Color * prim_color) * diff;
				}

				if prim.m.specular > 0.0 {
					//FIXME: Maybe this is messed up.

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
						color_add.v3d_add(color_add);
					}
					// R = L -  N * L.Dot(N) * 2.0l;
				}

				let refl = prim.m.reflective;
				if refl > 0.0 && refl_depth < 4 {
					prim.normal(pi);


					let mut r: Vector3D = ray.direction;
					let mut r1: Vector3D = n;

					let r2 = ray.direction.v3d_dot_mul(n) * 2.0;
					r1.v3d_mul_scalar(r2);
					r.v3d_sub(r1);

					let mut newpi: Vector3D = r;
					newpi.v3d_mul_scalar(0.0001);
					newpi.v3d_add(pi);

					let tempr: Ray = Ray {
						origin: newpi,
						direction: r,
					};

					let mut rcol:Vector3D = trace(tempr, refl_depth +1, globals);

					rcol.v3d_mul_scalar(refl);
					rcol.v3d_mul_v3d(prim_color);
					color.v3d_add(rcol);
				}
			}
		}

		let ret_vector: Vector3D = Vector3D {
			x: color.x,
			y: color.y,
			z: color.z,
		};

		return ret_vector;
}

fn render(thread_id: u32, globals: &mut GlobalSettings) {
    let camerapos = Vector3D::v3d_new((0.0, 0.0, -5.0));

    let wx1: f64 = -2.0;
    let wx2: f64 = 2.0;
    let wy1: f64 = 1.5;
    let wy2: f64 = -1.5;

    let dx: f64 = (wx2 - wx1) as f64 / (globals.img.width()) as f64;
    let dy: f64 = (wy2 - wy1) as f64 / (globals.img.height()) as f64;

    let mut sx: f64 = wx1;
    let mut sy: f64 = wy1 + dy * (thread_id as f64);

    for y in (thread_id..globals.img.height()).step_by(MAXTHREADS as usize) {
        sx = wx1;

        for x in 0..globals.img.width() {
            let camera_target = Vector3D::v3d_new((sx, sy, 0.0));

            let mut ray = Ray {
                origin: camerapos,
                direction: camera_target,
            };

            ray.direction.v3d_sub(ray.origin);
            ray.direction.v3d_norm();

            let color: Vector3D = trace(ray, 0, globals);
            let r: u8 = (color.x * 255.0) as u8;
            let g: u8 = (color.y * 255.0) as u8;
            let b: u8 = (color.z * 255.0) as u8;

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

    let mut sphere_pos: Vector3D = Vector3D {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    for j in 0..6 {
        for i in 0..9 {
            let mut m = &mirror;
            let mut z = 2.0_f64;
            let sn = ((i + j) as f64).sin() * 0.8;

            match sphere_pos_map[j][i] {
                'g' => {
                    z += -0.5;
                    m = &green;
                }
                'r' => {
                    z += -0.5;
                    m = &red;
                }

                _ => {
                    z += sn;
                }
            }

            sphere_pos.v3d_update((-2.0 + (i as f64) * 0.5, 1.25 - (j as f64) * 0.5, z));
            add_sphere(&sphere_pos, 0.25, m, &mut globals);
        }
    }

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

