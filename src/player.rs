use rusty_audio::Audio;
use std::{rc::Rc, cell::RefCell, time::Duration};

use crate::{NUM_COLS, NUM_ROWS, frame::{Drawable, Frame}, shot::{Shot, ExplosionStatus}};

pub struct Player {
    pub x: usize,
    pub y: usize,
    audio: Rc<RefCell<Audio>>,
    pub shots: Vec<Shot>
}

impl Player {
    pub fn new(audio: Rc<RefCell<Audio>>) -> Self {
        Self {
            x: NUM_COLS / 2,
            y: NUM_ROWS - 1,
            audio,
            shots: Vec::new(),
        }
    }

    pub fn move_left(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        }
    }

    pub fn move_right(&mut self) {
        if self.x < NUM_COLS - 1 {
            self.x += 1;
        }
    }

    pub fn move_up(&mut self) {
        if self.y > 0 {
            self.y -= 1;
        }
    }

    pub fn move_down(&mut self) {
        if self.y < NUM_ROWS - 1 {
            self.y += 1;
        }
    }

    pub fn shoot(&mut self) {
        let shot = Shot::new(self.x, self.y, Rc::clone(&(self.audio)));
        self.shots.push(shot);
    }

    pub fn update(&mut self, delta: Duration) {
        for shot in self.shots.iter_mut() {
            shot.update(delta);
        }

        self.shots.retain(|shot| !shot.dead());
    }

}

impl Drawable for Player {
    fn draw(&self, frame: &mut Frame) {
        for shot in self.shots.iter() {
            shot.draw(frame);
        }        
        if frame[self.x][self.y] == "X" {
            frame[self.x][self.y] = "O";
        } else {
            frame[self.x][self.y] = "A";
        }
    }
}
