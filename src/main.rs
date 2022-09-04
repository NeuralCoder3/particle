use std::{collections::HashMap, iter::Map, ops::Mul, rc::Rc};
// use std::hash::*;
use core::hash::*;
use uuid::{uuid, Uuid};

use macroquad::prelude::*;


fn scale(s: f32, v: Vec2) -> Vec2 {
    Vec2::new(v.x * s, v.y * s)
}


#[derive(Debug, Clone, Copy)]
struct Particle {
    pos: Vec2,
    vel: Vec2,
    // kind: &ParticleGroup,
}

#[derive(Clone)]
struct ParticleGroup {
    particles: Vec<Particle>,
    color: Color,
    mass: f32,
    radius: f32,
    id: Uuid, 
    interactions: HashMap<Rc<Uuid>, f32>,
}

impl Eq for ParticleGroup {}
impl PartialEq for ParticleGroup {
    fn eq(&self, other: &Self) -> bool {
        self as *const _ == other as *const _
    }
}
impl Hash for ParticleGroup {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // self.color.hash(state);
        // self.mass.hash(state);
    }
}

impl Particle {
    fn new(pos: Vec2) -> Self {
        Self {
            pos,
            vel: Vec2::ZERO,
        }
    }

    fn random() -> Self {
        let pos = Vec2::new(rand::gen_range(-1.0, 1.0), rand::gen_range(-1.0, 1.0));
        Self {
            // pos: Vec2::new(rand::gen_range(0.0, screen_width()), rand::gen_range(0.0, screen_height())),
            pos: pos,
            // vel: -pos.normalize() ,
            vel: Vec2::ZERO,
        }
    }

    fn update(&mut self, dt: f32) {
        self.pos += self.vel * dt;
        // checks bounds
        if self.pos.x < -1.0 {
            self.pos.x = -1.0;
            self.vel.x = -self.vel.x;
        }
        if self.pos.x > 1.0 {
            self.pos.x = 1.0;
            self.vel.x = -self.vel.x;
        }
        if self.pos.y < -1.0 {
            self.pos.y = -1.0;
            self.vel.y = -self.vel.y;
        }
        if self.pos.y > 1.0 {
            self.pos.y = 1.0;
            self.vel.y = -self.vel.y;
        }
    }

    fn draw(&self, color: Color, radius: f32) {
        draw_circle(
            screen_width() * (1.0 + self.pos.x) / 2.0,
            screen_height() * (1.0 + self.pos.y) / 2.0,
            radius,
            color,
        );
    }
}

impl ParticleGroup {
    fn new(color: Color, mass: f32, radius: f32) -> Self {
        Self {
            particles: Vec::new(),
            color,
            mass,
            radius,
            id: Uuid::new_v4(), 
            interactions: HashMap::new()
        }
    }

    fn clone_empty(&self) -> Self {
        Self {
            particles: Vec::new(),
            color: self.color,
            mass: self.mass,
            id: self.id, 
            radius: self.radius,
            interactions: self.interactions.clone(),
        }
    }

    fn fill(&mut self, n: usize) {
        for _ in 0..n {
            self.particles.push(Particle::random());
        }
    }

    fn interact_all(&self, groups: &Vec<ParticleGroup>, dt: f32) -> ParticleGroup {
        let mut new_group = self.clone_empty();
        new_group.particles = self.particles.clone();
        for other in groups {
            match self.interactions.get(&other.id) {
                Some(&G) => {
                    new_group.particles = new_group.interact(other, G, dt);
                }
                None => {}
            }
            // new_group.particles = new_group.interact(other, G, dt);
            // if group.id != self.id {
            //     self.interact(group, G, dt);
            // }
        }
        new_group
    }

    fn interact(&self, other: &ParticleGroup, G: f32, dt: f32) -> Vec<Particle> {
        let mut forces = Vec::new();
        for particle in self.particles.iter() {
            let mut cummulated_force = Vec2::ZERO;
            for other_particle in other.particles.iter() {
                let dist = (particle.pos - other_particle.pos).length();
                if dist == 0.0 {
                    continue;
                }
                let direction = (other_particle.pos - particle.pos).normalize();
                let force =
                    direction * G * self.mass * other.mass
                        / dist.powi(2);
                // let force = Vec2::new(0.0,0.01);
                // let force = scale(0.01, force);
                // println!("pos1: {:?}", particle.pos);
                // println!("pos2: {:?}", other_particle.pos);
                // println!("force: {:?}", force);
                // println!("direction: {:?}", direction);
                cummulated_force += force;
            }
            forces.push(cummulated_force);
        }

        let mut new_particles = Vec::new();

        for (i, particle) in self.particles.iter().enumerate() {
            let mut new_particle = particle.clone();
            new_particle.vel += dt * forces[i];
            if new_particle.vel.length() > 1.0 {
                new_particle.vel = new_particle.vel.normalize() * 1.0;
            }
            // new_particle.vel += Vec2::new(0.0, 0.0);
            // new_particle.vel += forces[i];
            new_particles.push(new_particle);
        }

        new_particles
        // self.particles.clone()
    }

    fn update(&mut self, dt: f32) {
        for particle in self.particles.iter_mut() {
            particle.update(dt);
        }
    }

    fn draw(&self, radius: f32) {
        for particle in self.particles.iter() {
            // println!("pos: {:?}", particle.pos.x);
            particle.draw(self.color, radius);
        }
    }
}

#[macroquad::main("BasicShapes")]
async fn main() {
    let dt = 0.01;

    

    let mut groups = Vec::new();
    // let mut interactions = HashMap::new();
    // let mut interactions = Vec::new();
    {
        let mut group = ParticleGroup::new(RED, 0.1, 1.0);
        group.fill(3);
        // group.interactions.insert(Box::new(group), 1.0);
        // let id = group.id;

        // interactions.entry(id).or_insert(Vec::new()).push((1.0,id));
        // interactions
        //     .entry(&group)
        //     .or_insert(Vec::new())
        //     .push((1.0, &group));
        // interactions.entry(Box::new(group)).or_insert(Vec::new()).push((1.0,Box::new(group)));
        // interactions.push((group, group, 1.0));


        let mut green_group = ParticleGroup::new(GREEN, 0.1, 1.0);
        green_group.fill(3);

        group.interactions.insert(Rc::new(group.id), 1.0);


        group.interactions.insert(Rc::new(green_group.id), -1.0);


        groups.push(group);
        groups.push(green_group);
    }

    loop {
        clear_background(WHITE);

        let mut new_groups = Vec::new();

        for group in groups.iter() {
            // let mut new_group = group.clone_empty();
            // new_group.particles = group.particles.clone();
            // new_group.particles = group.interact(group, 1.0, dt);
            let mut new_group = group.interact_all(&groups, dt);
            new_group.draw(10.0);
            new_group.update(dt);
            new_groups.push(new_group);
        }
        groups = new_groups;

        // for (group, interactions) in interactions.iter() {
        //     // group.draw(10.0);
        //     let mut new_group = group.clone_empty();
        //     new_group.particles = group.particles.clone();
        //     for (G, other) in interactions.iter() {
        //         let new_particles = new_group.interact(other, *G, 0.01);
        //     //     let mut new_group = group.clone_empty();
        //         new_group.particles = new_particles;
        //     //     new_groups.push(new_group);
        //     }
        //     new_group.draw(5.0);
        // }

        // for group in groups.iter() {
        //     let mut new_group = group.clone_empty();
        //     new_group.particles = group.particles.clone();

        //     // for (g, other, G) in interactions.iter() {
        //     //     if g == group {
        //     //         new_group.particles = group.interact(other, *G, 0.01);
        //     //     }
        //     // }

        //     // for (G, other) in interactions.get(*group).unwrap().iter() {
        //     //     new_group.particles = new_group.interact(other, *G, 0.01);
        //     // }

        //     new_group.update(get_frame_time());
        //     new_group.draw(10.0);

        //     new_groups.push(&new_group);
        //     // new_groups.push(new_group.clone());
        //     // new_groups.push(new_group.clone());
        // }

        // groups = new_groups;

        // draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        // draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        // draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);

        // draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
}
