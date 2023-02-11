use std::{time::Duration, rc::Rc, cell::RefCell};

use rusty_audio::Audio;
use rusty_time::Timer;

use crate::{frame::{Frame, Drawable, get_safe_coords}, player::Player, NUM_ROWS};

const NUM_EXPLOSION_STAGES: i32 = 5;

pub enum ExplosionStatus {
    None,
    Progress,
    End
}

pub fn get_bullet_coords(center: (i32, i32), radius: i32) -> Vec<(i32, i32)> {
    let mut coords: Vec<(i32, i32)> = Vec::new();
    let start_vec = (radius, 0);
    let sector = 5;
    for i in 0..=360/sector {
        let theta = i * sector;
        let x = start_vec.0 as f32 * (theta as f32).cos() - start_vec.1 as f32 * (theta as f32).sin();
        let y = start_vec.0 as f32 * (theta as f32).sin() + start_vec.1 as f32 * (theta as f32).cos();
        coords.push((x.round() as i32 + center.0, y.round() as i32 + center.1));
    }
    coords
}

pub struct Shot {
    pub x: usize,
    pub y: usize,
    pub x_despawn: usize,
    pub y_despawn: usize,
    pub particle_effects_coords: Vec<(usize, usize)>,
    pub is_explode: bool,
    pub is_despawned: bool,
    audio: Rc<RefCell<Audio>>,
    explosion_stage: i32,
    pub explosion_status: ExplosionStatus,
    timer: Timer
}

impl Shot {
    pub fn new(x: usize, y:usize, audio: Rc<RefCell<Audio>>) -> Self {
        Self {
            x,
            y,
            x_despawn: 0,
            y_despawn: 0,
            particle_effects_coords: Vec::new(),
            is_explode: false,
            is_despawned: false,
            audio,
            explosion_stage: 1,
            explosion_status: ExplosionStatus::None,
            timer: Timer::from_millis(100),
        }
    }

    pub fn update(&mut self, delta: Duration) {
        self.timer.update(delta);
        if self.timer.ready {
            self.update_particle_effects();
            if matches!(self.explosion_status, ExplosionStatus::None) {
                if self.y > 0 {
                    self.y -= 1;
                } else {
                    self.explode();
                }
            }
            self.timer.reset();
        }
    }

    pub fn explode(&mut self) {
        self.audio.borrow_mut().play("explode");
        self.is_explode = true;
        self.explosion_status = ExplosionStatus::Progress;
        self.x_despawn = self.x;
        self.y_despawn = self.y;
    }

    pub fn dead(&self) -> bool {
        matches!(self.explosion_status, ExplosionStatus::End)
    }

    fn update_particle_effects(self: &mut Self) {
        if !matches!(self.explosion_status, ExplosionStatus::Progress) {
            return;
        }

        let new_coords: Vec<(usize, usize)> = get_bullet_coords(
                (self.x_despawn as i32, self.y_despawn as i32), self.explosion_stage
            ).iter().map(|x| get_safe_coords(x.0, x.1)).collect(); 

        self.particle_effects_coords = new_coords;
        self.explosion_stage = if self.explosion_stage < NUM_EXPLOSION_STAGES {
            self.timer=Timer::from_millis(250);
            self.explosion_stage + 1
        } else {
            self.explosion_status = ExplosionStatus::End;
            self.particle_effects_coords.clear();
            self.explosion_stage 
        };
    }


}


impl Drawable for Shot {
    fn draw(self: &Self, frame: &mut Frame) {
        if matches!(self.explosion_status, ExplosionStatus::Progress) {
            frame[self.x][self.y] = "*";
            for coord in self.particle_effects_coords.iter() {
                frame[coord.0][coord.1] = ".";
            }
        } else if matches!(self.explosion_status, ExplosionStatus::End) {
            frame[self.x_despawn][self.y_despawn] = " ";
            for coord in self.particle_effects_coords.iter() {
                frame[coord.0][coord.1] = " ";
            }
        } else {
            frame[self.x][self.y] = "|";
        }
    }
}

