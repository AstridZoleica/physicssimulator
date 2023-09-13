use draw::render::bitmap::PNGRenderer;
use rand::prelude::*;
use rand::distributions::WeightedIndex;
use draw::*;
use std::dbg;

fn main() {
    let mut particle_vector: Vec<Particle> = Vec::new();

    // Generate Particles
    let choices: [u32; 5] = [0, 1, 2, 3, 4];
    let weights = [5000, 4, 3, 6, 1];
    let dist = WeightedIndex::new(&weights).unwrap();
    let mut rng = thread_rng();
    let output = [(); 3600].map(|_| choices[dist.sample(&mut rng)]);
    let mut counter: f64 = 0.0;
    for i in output {
        match i {
            1 => particle_vector.push(
                    Particle::new(1250.0 + thread_rng().gen_range(-100..100) as f64,
                        1250.0 + thread_rng().gen_range(-100..100) as f64,
                        thread_rng().gen_range(-63..63) as f64 / 700.0,
                        thread_rng().gen_range(-63..63) as f64 / 700.0, ParticleType::Proton)
            ),
            2 => particle_vector.push(
                    Particle::new(1250.0 + thread_rng().gen_range(-100..100) as f64,
                    1250.0 + thread_rng().gen_range(-100..100) as f64,
                    thread_rng().gen_range(-63..63) as f64 / 700.0,
                    thread_rng().gen_range(-63..63) as f64 / 700.0, ParticleType::Antiproton)
            ),
            3 => particle_vector.push(
                    Particle::new(1250.0 + thread_rng().gen_range(-100..100) as f64,
                    1250.0 + thread_rng().gen_range(-100..100) as f64,
                    thread_rng().gen_range(-63..63) as f64 / 700.0,
                    thread_rng().gen_range(-63..63) as f64 / 700.0, ParticleType::Electron)
            ),
            4 => particle_vector.push(
                Particle::new(1250.0 + thread_rng().gen_range(-100..100) as f64,
                1250.0 + thread_rng().gen_range(-100..100) as f64,
                thread_rng().gen_range(-63..63) as f64 / 700.0,
                thread_rng().gen_range(-63..63) as f64 / 700.0, ParticleType::Alpha)
        ),
            _ => ()
        }
        counter += 1.0;
    }

    // particle_vector.push(
    //     Particle::new(2500.0, 2500.0, -0.075, 0.0, ParticleType::Proton)
    // );
    // particle_vector.push(
    //     Particle::new(2560.0, 2560.0, 0.05, 0.0, ParticleType::Antiproton)
    // );


    // Draw the Current State of the Canvas
    dbg!(&particle_vector);
    let mut reference = particle_vector.clone();
    draw_system(&particle_vector, 0);

    let mut staticCanvas = Canvas::new(2500, 2500);

    for frame in 1..=2000_u32 {
        for i in &mut particle_vector {
            i.update_particle(&reference);
        }
        draw_system(&particle_vector, frame);

        for i in &particle_vector {
            let color = match i.kind {
                ParticleType::Electron => RGB{r: 100, g: 100, b: 255},
                ParticleType::Neutron => Color::black(),
                ParticleType::Proton => RGB{r: 255, g: 100, b: 100},
                ParticleType::Antiproton => RGB{r: 255, g: 100, b: 255},
                ParticleType::Alpha => RGB{r: 255, g: 255, b: 100}
            };
            staticCanvas.display_list.add(
                Drawing::new()
                    .with_shape(Shape::Circle { radius: 1 })
                    .with_xy(i.posx as f32, i.posy as f32)
                    .with_style(Style::stroked(5, color))
            );
        }


        reference = particle_vector.clone();
        if frame == 1_u32 {
            dbg!(&particle_vector);
        }
        if frame == 1000_u32 {
            dbg!(&particle_vector);
        }
    }
    dbg!(&particle_vector);

    render::save(
        &staticCanvas,
        "overall.svg",
        SvgRenderer::new()
    ).expect("Failed to render svg");

}

fn draw_system(particle_vector:&Vec<Particle>, frame_number: u32) {
    let mut canvas = Canvas::new(2500, 2500);
    for i in particle_vector {
        let color = match i.kind {
            ParticleType::Electron => RGB{r: 100, g: 100, b: 255},
            ParticleType::Neutron => Color::black(),
            ParticleType::Proton => RGB{r: 255, g: 100, b: 100},
            ParticleType::Antiproton => RGB{r: 255, g: 100, b: 255},
            ParticleType::Alpha => RGB{r: 255, g: 255, b: 100}
        };
        canvas.display_list.add(
            Drawing::new()
                .with_shape(Shape::Circle { radius: 1 })
                .with_xy(i.posx as f32, i.posy as f32)
                .with_style(Style::stroked(80, color))
        );
    }
    let mut temp = frame_number.to_string();
    temp.push_str(".svg");
    render::save(
        &canvas,
        &temp,
        SvgRenderer::new()
    ).expect("Failed to render svg");
}

#[derive(Debug, Clone)]
struct Particle {
    posx: f64,
    posy: f64,
    vx: f64,
    vy: f64,
    charge: i32,
    mass: i32,
    kind: ParticleType
}

impl Particle {
    fn new(posx: f64, posy: f64, vx: f64, vy: f64, kind: ParticleType) -> Particle {
        Particle {
            posx: posx,
            posy: posy,
            vx: vx,
            vy: vy,
            charge: {
                match kind {
                    ParticleType::Electron => -1,
                    ParticleType::Neutron => 0,
                    ParticleType::Proton => 1,
                    ParticleType::Antiproton => -1,
                    ParticleType::Alpha => 2
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
            kind: kind
        }
    }

    fn calc_paired_electrostatic_force(&self, other: &Particle) -> (f64, f64) {
        let (difx, dify): (f64, f64) = (self.posx - other.posx, self.posy - other.posy);
        if (difx == 0.0 && dify == 0.0) {return (0.0, 0.0)} // Particle is at the same position as another particle! We'd prefer this not to happen...
        if self.charge * other.charge == 0 {return (0.0, 0.0)} // At least one of the particles is neutral! No electrostatic force here.
        let hypotenuse: f64 = (difx.powi(2) + dify.powi(2)).abs().sqrt();
        let magnitude:f64 = 0.44 * self.charge as f64 * 160.2 * other.charge as f64 * 160.2 / ((difx.powi(2) + dify.powi(2)) as f64);
        let mut xout = magnitude * difx / hypotenuse;
        let mut yout = magnitude * dify / hypotenuse;
        match (self.charge * other.charge > 0) {
            true => { // If the charges are alike...
                if self.posx > other.posx {xout = -1.0 * xout.abs()} // Self to the right of other. Move the self rightwards (+x).
                else {xout = xout.abs()} // Self to the left of other. Move the self leftwards (-x).
                if self.posy > other.posy {yout = -1.0 * yout.abs()} // Self above other. Move the self up (-y).
                else {yout = yout.abs()} // Self below other Move the self down (+y).
            },
            false => { // If the charges are opposites...
                if self.posx > other.posx {xout = xout.abs()} // Self to the right of other. Move the self leftwards (-x).
                else {xout = -1.0 * xout.abs()} // Self to the left of other. Move the self rightwards (+x).
                if self.posy > other.posy {yout = yout.abs()} // Self above other. Move the self down (+y).
                else {yout = -1.0 * yout.abs()} // Self below otherl Move the self up (-y).
            }
        }
        (xout,yout)
    }

    fn calc_electrostatic_force_superposition(&self, particle_vector:&Vec<Particle>) -> (f64, f64) {
        let (mut outx, mut outy): (f64, f64) = (0.0, 0.0);
        let test:Vec<(f64, f64)> = particle_vector.into_iter()
            .map(
                |x| {
                    let output = x.calc_paired_electrostatic_force(&self);
                    outx += output.0;
                    outy += output.1;
                    output
                }
            ).collect();
        // dbg!(test);
        (outx, outy)
    }

    fn update_particle(&mut self, particle_vector:&Vec<Particle>) {
        let net_force = self.calc_electrostatic_force_superposition(particle_vector);
        self.posx += self.vx;
        self.posy += self.vy;
        self.vx += net_force.0 / (self.mass as f64);
        self.vy += net_force.1 / (self.mass as f64);
    }
}

#[derive(Debug, Clone)]
enum ParticleType {
    Electron,
    Proton,
    Neutron,
    Antiproton,
    Alpha
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