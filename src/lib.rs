extern crate rand;
/// A `Particle` is a model of a gas molecule. For simplicity all of its
/// collision is considered elastic
extern crate sdl2;

use rand::Rng;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;
use std::ops::Add;
use std::ops::Sub;

#[derive(Clone, Copy, Debug)]
pub struct Vector {
    x: f32,
    y: f32,
}

impl Vector {
    fn new(x: f32, y: f32) -> Self {
        Vector { x: x, y: y }
    }

    fn hypot(&self) -> f32 {
        ((self.x * self.x) + (self.y * self.y)).sqrt()
    }

    fn to_rect(&self) -> Rect {
        Rect::new(self.x.round() as i32, self.y.round() as i32, 10, 10)
    }
}

impl Add for Vector {
    type Output = Vector;
    fn add(self, rhs: Vector) -> Vector {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Vector {
    type Output = Vector;
    fn sub(self, rhs: Vector) -> Vector {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Particle {
    position: Vector,
    velocity: Vector,
}

impl Particle {
    pub fn new(pos: Vector, vel: Vector) -> Self {
        Particle {
            position: pos,
            velocity: vel,
        }
    }

    pub fn update(&mut self) {
        self.position = self.position + self.velocity;
    }
}

pub struct Environment {
    particles: Vec<Particle>,
    height: f32,
    width: f32,
}

impl Environment {
    pub fn new(height: f32, width: f32, number: i32) -> Self {
        let mut rng = rand::thread_rng();
        let mut new_list: Vec<Particle> = Vec::new();
        for _x in 0..number {
            new_list.push(Particle::new(
                Vector::new(rng.gen_range(1f32, width), rng.gen_range(1f32, height)),
                Vector::new(rng.gen_range(-10f32, 10f32), rng.gen_range(-10f32, 10f32)),
            ));
        }

        Environment {
            particles: new_list,
            height: height,
            width: width,
        }
    }

    pub fn update(&mut self) {
        for i in 0..self.particles.len() {
            // edge detection
            if self.particles[i].position.x >= self.width {
                self.particles[i].position.x = self.width;
                self.particles[i].velocity.x *= -1f32;
            }

            if self.particles[i].position.x <= 0f32 {
                self.particles[i].position.x = 0f32;
                self.particles[i].velocity.x *= -1f32;
            }

            if self.particles[i].position.y >= self.height {
                self.particles[i].position.y = self.height;
                self.particles[i].velocity.y *= -1f32;
            }

            if self.particles[i].position.y <= 0f32 {
                self.particles[i].position.y = 0f32;
                self.particles[i].velocity.y *= -1f32;
            }

            for x in i + 1..self.particles.len() {
                let dv = self.particles[x].position - self.particles[i].position;
                let dist = dv.hypot();
                if dist <= 20f32 {
                    let tmp = self.particles[i].velocity;
                    self.particles[i].velocity = self.particles[x].velocity;
                    self.particles[x].velocity = tmp;
                }
            }

            self.particles[i].update();
        }
    }

    pub fn render(&self, canvas: &mut Canvas<Window>, tex: &Texture) {
        for particle in &self.particles {
            let rect = particle.position.to_rect();
            canvas.copy(tex, None, Some(rect)).unwrap();
        }
    }
}
