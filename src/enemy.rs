use std::{rc::Rc, cell::RefCell, time::Duration};
use rusty_audio::Audio;
use rusty_time::Timer;

use crate::{NUM_COLS, NUM_ROWS, frame::{Drawable, Frame}};

fn set_n_bit(bit_arr: &mut u8, n: u8, bit: u8) {
    *bit_arr &= !(1 << n);
    *bit_arr |= bit << n;
}

fn get_n_bit(bit: u8, n: u8) -> u8 {
    (bit & (1 << n)) >> n 
}

const PAD: usize = 5;

const IS_MOVE: u8 = 0;
const UP_DOWN: u8 = 1;
const LEFT_RIGHT: u8 = 2;

const LEFT: u8 = 0;
const RIGHT: u8 = 1;

const UP: u8 = 0;
const DOWN: u8 = 1;

const STOP: u8 = 0;
const MOVE: u8 = 1;


pub struct Enemy {
    pub x: usize,
    pub y: usize,
    pub i: usize
}

pub struct Enemies {
    pub enemy_list: Vec<Enemy>,
    timer: Timer,
    audio: Rc<RefCell<Audio>>,
    direction: u8
}

impl Enemies {
    pub fn new(audio: Rc<RefCell<Audio>>) -> Self {
        let mut temp_enemy_list: Vec<Enemy> = Vec::new();
        let mut enemy_count: usize = 0;
        for x in 0..NUM_COLS {
            for y in 0..(NUM_ROWS/4) {
                if (x % 2 == 0 && y % 2 == 1) ||
                    (x % 2 == 1 && y % 2 == 0) ||
                    (x < PAD || x > NUM_COLS - 1 - PAD){
                    continue;
                }
                temp_enemy_list.push(Enemy{x, y, i: enemy_count});
                enemy_count += 1;
            }
        }
        Self {
            enemy_list: temp_enemy_list,
            timer: Timer::from_millis(3000),
            audio,
            direction: 0
        }
    }

    pub fn get_shot(self: &mut Self, index_to_get_shot: usize) {
        self.enemy_list.retain(|e| e.i != index_to_get_shot);
    }

    pub fn update(self: &mut Self, delta: Duration) -> bool {
        self.timer.update(delta);
        if self.timer.ready {
            let min_x = self.enemy_list.iter().map(|e| e.x).min().unwrap_or(0);
            let max_x = self.enemy_list.iter().map(|e| e.x).max().unwrap_or(0);
            let max_y = self.enemy_list.iter().map(|e| e.y).max().unwrap_or(0);

            if max_y >= NUM_ROWS - 1 {
                set_n_bit(&mut self.direction, IS_MOVE, STOP);
            } else {
                set_n_bit(&mut self.direction, IS_MOVE,
                          MOVE);
                set_n_bit(&mut self.direction, UP_DOWN, DOWN);
            }

            if min_x <= 0 {
                set_n_bit(&mut self.direction, LEFT_RIGHT, RIGHT);
            } else if max_x > NUM_COLS - 2 - 1 {
                log::info!("switch!");
                set_n_bit(&mut self.direction, LEFT_RIGHT, LEFT);
            } 


            log::info!("min: {}, max: {}, direction: {:#b}", min_x, max_x, self.direction);
            for e in self.enemy_list.iter_mut() {
                if get_n_bit(self.direction, IS_MOVE) == MOVE {
                    if get_n_bit(self.direction, LEFT_RIGHT) == LEFT {
                        e.move_left();
                    } else {
                        e.move_right();
                    }

                    if get_n_bit(self.direction, UP_DOWN) == UP {
                        e.move_up();
                    } else {
                        e.move_down();
                    }
                }
                e.update();
            } 

            if get_n_bit(self.direction, IS_MOVE) == MOVE {
                self.audio.borrow_mut().play("move");
            }
            self.timer.reset();
            return true;
        }
        return false;
    }
}

impl Enemy {
    pub fn new(x: usize, y: usize, i: usize) -> Self {
        Self {
            x,
            y,
            i
        }
    }

    pub fn move_left(&mut self) -> bool {
        if self.x > 0 {
            self.x -= 1;
            return true;
        }
        return false;
    }

    pub fn move_right(&mut self) -> bool {
        if self.x < NUM_COLS - 1 {
            self.x += 1;
            return true;
        }
        return false;
    }

    pub fn move_up(&mut self) -> bool {
        if self.y > 0 {
            self.y -= 1;
            return true;
        }
        return false;
    }

    pub fn move_down(&mut self) -> bool {
        if self.y < NUM_ROWS - 1 {
            self.y += 1;
            return true;
        }
        return false;
    }

    pub fn update(&mut self) {
    }
}

impl Drawable for Enemy {
    fn draw(&self, frame: &mut Frame) {
        frame[self.x][self.y] = "X";
    }
}


impl Drawable for Enemies {
    fn draw(&self, frame: &mut Frame) {
        for e in self.enemy_list.iter() {
            e.draw(frame);
        }
    }
}
