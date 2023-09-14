use draw::render::bitmap::PNGRenderer;
use draw::*;
use rand::distributions::WeightedIndex;
use rand::prelude::*;
use std::dbg;

fn main() {
    // Empty vector to hold all of the particles
    let mut particle_vector: Vec<Particle> = Vec::new();

    // Place down particles.
    random_particle_placement_3d(&mut particle_vector);

    particle_vector.push(Particle::new(1250.0, 1250.0, 0.0, 0.0, 0.0, 0.0, ParticleType::Alpha));
    particle_vector.push(Particle::new(
        1250.0,
        1100.0,
        0.0,
        -12.3,
        0.0,
        0.0,
        ParticleType::Electron,
    ));

    // Change colors of Electrons to be more visible...
    let mut counter1 = 0;
    for i in &mut particle_vector {
        if i.kind == ParticleType::Electron {
            if counter1 % 2 == 1 {
                i.color.r += counter1*9;
                i.color.g += counter1*4;
            }
            i.color.r += counter1*4;
            i.color.g += counter1*9;
        }
        counter1 += 1;
    }

    // Draw the Current State of the Canvas
    let mut reference = particle_vector.clone();
    draw_system(&particle_vector, 0);

    // Draw over time
    let mut static_canvas = Canvas::new(2500, 2500);

    for _ in 0..=8000_u32 {
        for i in &mut particle_vector {
            i.update_particle(&reference);
        }
        // draw_system(&particle_vector, frame);

        for i in &particle_vector {
            let color = i.color;
            static_canvas.display_list.add(
                Drawing::new()
                    .with_shape(Shape::Circle { radius: 1 })
                    .with_xy(i.posx as f32, static_canvas.height as f32 - i.posy as f32)
                    .with_style(Style::stroked(5, color)),
            );
        }

        reference = particle_vector.clone();
        // if frame == 1_u32 {
        //     dbg!(&particle_vector);
        // }
        // if frame == 1000_u32 {
        //     dbg!(&particle_vector);
        // }
    }

    render::save(&static_canvas, "overall.svg", SvgRenderer::new()).expect("Failed to render svg");
}

fn random_particle_placement_2d(particle_vector: &mut Vec<Particle>) {
    // Generate Particles
    let choices: [u32; 5] = [0, 1, 2, 3, 4];
    let weights = [5000, 4, 3, 6, 1];
    let dist = WeightedIndex::new(&weights).unwrap();
    let mut rng = thread_rng();
    let output = [(); 3600].map(|_| choices[dist.sample(&mut rng)]);
    for i in output {
        match i {
            1 => particle_vector.push(Particle::new(
                1250.0 + thread_rng().gen_range(-200..200) as f64,
                1250.0 + thread_rng().gen_range(-200..200) as f64,
                0.0,
                thread_rng().gen_range(-63..63) as f64 / 700.0,
                thread_rng().gen_range(-63..63) as f64 / 700.0,
                0.0,
                ParticleType::Proton,
            )),
            2 => particle_vector.push(Particle::new(
                1250.0 + thread_rng().gen_range(-200..200) as f64,
                1250.0 + thread_rng().gen_range(-200..200) as f64,
                0.0,
                thread_rng().gen_range(-63..63) as f64 / 700.0,
                thread_rng().gen_range(-63..63) as f64 / 700.0,
                0.0,
                ParticleType::Antiproton,
            )),
            3 => particle_vector.push(Particle::new(
                1250.0 + thread_rng().gen_range(-800..800) as f64,
                1250.0 + thread_rng().gen_range(-800..800) as f64,
                0.0,
                thread_rng().gen_range(-63..63) as f64 / 25.0,
                thread_rng().gen_range(-63..63) as f64 / 25.0,
                0.0,
                ParticleType::Electron,
            )),
            4 => particle_vector.push(Particle::new(
                1250.0 + thread_rng().gen_range(-200..200) as f64,
                1250.0 + thread_rng().gen_range(-200..200) as f64,
                0.0,
                thread_rng().gen_range(-63..63) as f64 / 1000.0,
                thread_rng().gen_range(-63..63) as f64 / 1000.0,
                0.0,
                ParticleType::Alpha,
            )),
            _ => (),
        }
    }
}

fn random_particle_placement_3d(particle_vector: &mut Vec<Particle>) {
    // Generate Particles
    let choices: [u32; 5] = [0, 1, 2, 3, 4];
    let weights = [5000, 4, 3, 6, 1];
    let dist = WeightedIndex::new(&weights).unwrap();
    let mut rng = thread_rng();
    let output = [(); 3600].map(|_| choices[dist.sample(&mut rng)]);
    for i in output {
        match i {
            1 => particle_vector.push(Particle::new(
                1250.0 + thread_rng().gen_range(-200..200) as f64,
                1250.0 + thread_rng().gen_range(-200..200) as f64,
                1250.0 + thread_rng().gen_range(-200..200) as f64,
                thread_rng().gen_range(-63..63) as f64 / 700.0,
                thread_rng().gen_range(-63..63) as f64 / 700.0,
                thread_rng().gen_range(-63..63) as f64 / 700.0,
                ParticleType::Proton,
            )),
            2 => particle_vector.push(Particle::new(
                1250.0 + thread_rng().gen_range(-200..200) as f64,
                1250.0 + thread_rng().gen_range(-200..200) as f64,
                1250.0 + thread_rng().gen_range(-200..200) as f64,
                thread_rng().gen_range(-63..63) as f64 / 700.0,
                thread_rng().gen_range(-63..63) as f64 / 700.0,
                thread_rng().gen_range(-63..63) as f64 / 700.0,
                ParticleType::Antiproton,
            )),
            3 => particle_vector.push(Particle::new(
                1250.0 + thread_rng().gen_range(-800..800) as f64,
                1250.0 + thread_rng().gen_range(-800..800) as f64,
                1250.0 + thread_rng().gen_range(-800..800) as f64,
                thread_rng().gen_range(-63..63) as f64 / 25.0,
                thread_rng().gen_range(-63..63) as f64 / 25.0,
                thread_rng().gen_range(-63..63) as f64 / 25.0,
                ParticleType::Electron,
            )),
            4 => particle_vector.push(Particle::new(
                1250.0 + thread_rng().gen_range(-200..200) as f64,
                1250.0 + thread_rng().gen_range(-200..200) as f64,
                1250.0 + thread_rng().gen_range(-200..200) as f64,
                thread_rng().gen_range(-63..63) as f64 / 1000.0,
                thread_rng().gen_range(-63..63) as f64 / 1000.0,
                thread_rng().gen_range(-63..63) as f64 / 1000.0,
                ParticleType::Alpha,
            )),
            _ => (),
        }
    }
}

fn draw_system(particle_vector: &Vec<Particle>, frame_number: u32) {
    let mut canvas = Canvas::new(2500, 2500);
    for i in particle_vector {
        let color = i.color;
        canvas.display_list.add(
            Drawing::new()
                .with_shape(Shape::Circle { radius: 1 })
                .with_xy(i.posx as f32, canvas.height as f32 - i.posy as f32)
                .with_style(Style::stroked(80, color)),
        );
    }
    let mut temp = frame_number.to_string();
    temp.push_str(".svg");
    render::save(&canvas, &temp, SvgRenderer::new()).expect("Failed to render svg");
}

#[derive(Debug, Clone)]
struct Particle {
    posx: f64,
    posy: f64,
    posz: f64,
    vx: f64,
    vy: f64,
    vz: f64,
    charge: i32,
    mass: i32,
    color: RGB,
    kind: ParticleType,
}

impl Particle {
    fn new(posx: f64, posy: f64, posz: f64, vx: f64, vy: f64, vz: f64, kind: ParticleType) -> Particle {
        Particle {
            posx: posx,
            posy: posy,
            posz: posz,
            vx: vx,
            vy: vy,
            vz: vz,
            charge: {
                match kind {
                    ParticleType::Electron => -1,
                    ParticleType::Neutron => 0,
                    ParticleType::Proton => 1,
                    ParticleType::Antiproton => -1,
                    ParticleType::Alpha => 2,
                }
            },
            mass: {
                match kind {
                    ParticleType::Electron => 1,
                    ParticleType::Proton => 1833,
                    ParticleType::Neutron => 1834,
                    ParticleType::Antiproton => 1833,
                    ParticleType::Alpha => 7295,
                }
            },
            color: {
                match kind {
                    ParticleType::Electron => RGB {
                        r: 60,
                        g: 60,
                        b: 255,
                    },
                    ParticleType::Neutron => Color::black(),
                    ParticleType::Proton => RGB {
                        r: 255,
                        g: 100,
                        b: 100,
                    },
                    ParticleType::Antiproton => RGB {
                        r: 255,
                        g: 100,
                        b: 255,
                    },
                    ParticleType::Alpha => RGB {
                        r: 255,
                        g: 255,
                        b: 100,
                    },
                }
            },
            kind: kind,
        }
    }

    // Electrostatic Force
    fn calc_paired_electrostatic_force(&self, other: &Particle) -> [f64; 3] {
        let (difx, dify, difz): (f64, f64, f64) = (other.posx - self.posx, other.posy - self.posy, other.posz - self.posz);
        if difx == 0.0 && dify == 0.0 && difz == 0.0 {
            return [0.0; 3];
        } // Particle is at the same position as another particle! We'd prefer this not to happen because it is ugly...
        if self.charge * other.charge == 0 {
            return [0.0; 3];
        } // At least one of the particles is neutral! No electrostatic force here.
        let hypotenuse: f64 = (difx.powi(2) + dify.powi(2) + difz.powi(2)).sqrt();
        let magnitude: f64 = 0.44 * self.charge as f64 * 160.2 * other.charge as f64 * 160.2
            / ((difx.powi(2) + dify.powi(2) + difz.powi(2)) as f64);
        let (mut xout, mut yout, mut zout): (f64, f64, f64) = (magnitude * difx / hypotenuse, magnitude * dify / hypotenuse, magnitude * difz / hypotenuse);
        match self.charge * other.charge > 0 {
            true => {
                // If the charges are alike...
                if self.posx > other.posx { // Self to the right of other. Move the self rightwards (+x).
                    xout = -1.0 * xout.abs()
                } else { // Self to the left of other. Move the self leftwards (-x).
                    xout = xout.abs()
                } 
                if self.posy > other.posy { // Self above other. Move the self up (+y).
                    yout = - 1.0 * yout.abs()
                } else { // Self below other Move the self down (-y).
                    yout = yout.abs()
                }
                if self.posz > other.posz { // Self in front of other. Move the self farther front (+z).
                    zout = - 1.0 * zout.abs()
                } else { // Self below other Move the self down (-z).
                    zout = zout.abs()
                }
            }
            false => {
                // If the charges are different...
                if self.posx > other.posx { // Self to the right of other. Move the self rightwards (-x).
                    xout = xout.abs()
                } else { // Self to the left of other. Move the self leftwards (+x).
                    xout = -1.0 * xout.abs()
                } 
                if self.posy > other.posy { // Self above other. Move the self down (-y).
                    yout = yout.abs()
                } else { // Self below other Move the self up (+y).
                    yout = - 1.0 * yout.abs()
                }
                if self.posz > other.posz { // Self in front of other. Move the self farther front (-z).
                    zout = zout.abs()
                } else { // Self behind the other, move the self forward (+z).
                    zout = - 1.0 * zout.abs()
                }
            }
        }
        [xout, yout, zout]
    }

    fn calc_electrostatic_force_superposition(
        &self,
        particle_vector: &Vec<Particle>,
    ) -> [f64; 3] {
        let (mut outx, mut outy, mut outz): (f64, f64, f64) = (0.0, 0.0, 0.0);
        let _: Vec<[f64; 3]> = particle_vector
            .into_iter()
            .map(|x| {
                let output = x.calc_paired_electrostatic_force(&self);
                outx += output[0];
                outy += output[1];
                outz += output[2];
                output
            })
            .collect();
        // dbg!(_);
        [outx, outy, outz]
    }

    // Magnetic Force (Moving charges produce magnetic fields and are acted upon by a magnetic force when moving through such a field)
    fn calc_others_magnetic_field(&self, other: &Particle) -> [f64; 3] {
        // Components from other to self (Test magnetic field produced by other at the location of self).
        let (difx, dify, difz): (f64, f64, f64) = (self.posx - other.posx, self.posy - other.posy, self.posz - other.posz);
        if difx == 0.0 && dify == 0.0 && difz == 0.0 {
            return [0.0; 3];
        } // Particle is at the same position as another particle! We'd prefer this not to happen because it is ugly...
        if self.charge * other.charge == 0 {
            return [0.0; 3];
        } // At least one of the particles is neutral! No electrostatic force here.
        // Magnitude of the vector.
        let hypotenuse: f64 = (difx.powi(2) + dify.powi(2) + difz.powi(2)).sqrt();
        // Permeability of free space divided by 4pi times scaling of Ampere*seconds and so on
        let coefficient: f64 = 0.00003895172 * 160.2 * other.charge as f64 / hypotenuse / hypotenuse;
        // Multiply coefficient by velocity components
        let (x, y, z): (f64, f64, f64) = (other.vx * coefficient, other.vy * coefficient, other.vz * coefficient);
        // Calculate components of the unit vector in the direction from self to other.
        let (ux, uy, uz): (f64, f64, f64) = (difx / hypotenuse, dify / hypotenuse, difz / hypotenuse);
        // Perform the cross product. Use the "cross product matrix" here: https://people.eecs.ku.edu/~jrmiller/Courses/VectorGeometry/VectorOperations.html
        [
            -1.0 * z * uy + y * uz,
            z * ux + -1.0 * x * uz,
            -1.0 * y * ux + x * uy
        ]
    }

    fn calc_paired_magnetic_force(&self, other: &Particle) -> [f64; 3] {
        let (qvx, qvy, qvz): (f64, f64, f64) = (self.charge as f64 * self.vx, self.charge as f64 * self.vy, self.charge as f64 * self.vz);
        let b = self.calc_others_magnetic_field(other);
        // Perform the cross product (F = qv x B)
        [
            -1.0 * qvz * b[1] + qvy * b[2],
            qvz * b[0] + -1.0 * qvx * b[2],
            -1.0 * qvy * b[0] + qvx * b[1]
        ]
    }

    fn calc_magnetic_force_superposition(
        &self,
        particle_vector: &Vec<Particle>,
    ) -> [f64; 3] {
        let (mut outx, mut outy, mut outz): (f64, f64, f64) = (0.0, 0.0, 0.0);
        let _: Vec<[f64; 3]> = particle_vector
            .into_iter()
            .map(|x| {
                let output = x.calc_paired_magnetic_force(&self);
                outx += output[0];
                outy += output[1];
                outz += output[2];
                output
            })
            .collect();
        // dbg!(_);
        [outx, outy, outz]
    }

    // Final Update Function
    fn update_particle(&mut self, particle_vector: &Vec<Particle>) {
        // Still learning how to do this properly in rust: https://stackoverflow.com/questions/41207666/how-do-i-add-two-rust-arrays-element-wise
        let mut net_force:[f64; 3] = [0.0; 3];
        for i in 0..3 {
            net_force[i] = self.calc_magnetic_force_superposition(particle_vector)[i] + self.calc_electrostatic_force_superposition(particle_vector)[i];
        }
        self.posx += self.vx;
        self.posy += self.vy;
        self.posz += self.vz;
        self.vx += net_force[0] / (self.mass as f64);
        self.vy += net_force[1] / (self.mass as f64);
        self.vz += net_force[2] / (self.mass as f64);
    }
}

#[derive(Debug, Clone, PartialEq)]
enum ParticleType {
    Electron,
    Proton,
    Neutron,
    Antiproton,
    Alpha,
}

// let mut rng = rand_pcg::Mcg128Xsl64::new(4);

// let mut canvas = Canvas::new(100, 100);
// let mut circ = Drawing::new()
//     .with_shape(Shape::Circle {
//         radius: 5
//     })
//     .with_xy(100.0, 100.0)
//     .with_style(Style::stroked(5, Color::black()));
// canvas.display_list.add(circ);
// render::save(
//     &canvas,
//     "test.svg",
//     SvgRenderer::new()
// ).expect("failed to save");
// let mut output = Vec::new();
// println!("Hello, world!");
// for i in 1..=100 {
//     rng.advance(1);
//     output.push(rng.next_u32());
// }
// for i in output {
//     println!("{}", i)
// }
