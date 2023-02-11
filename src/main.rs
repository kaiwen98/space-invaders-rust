use std::{error::Error, time::{Duration, Instant}, sync::mpsc, thread::{Thread, self}, cell::RefCell, rc::Rc};

use crossterm::{
    terminal::{
        self, 
        EnterAlternateScreen, 
        LeaveAlternateScreen
    }, 
    cursor::{
        Hide, 
        Show
    }, 
    event::{
        self, 
        Event, 
        KeyCode
    }
};
use space_invaders::{frame::Drawable, shot::Shot, enemy::Enemies, game_logic::{GameLogic, GameState}};

use crossterm::ExecutableCommand;
use rusty_audio::Audio;
use space_invaders::{render, frame::{new_frame, self}, player::Player};
use std::fs;
use std::io;
mod utils;
use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Root};

fn main() -> Result<(), Box<dyn Error>>{
    // Set up log
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
        .build("log/output.log")?;

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder()
                   .appender("logfile")
                   .build(LevelFilter::Info))?;

    log4rs::init_config(config)?;


    // Set up Audio
    let mut _audio = Audio::new();
    let mut audio_ref_cell = Rc::new(RefCell::new(_audio));
    let paths = fs::read_dir(format!("{}/src/assets/", utils::get_project_root_str())).unwrap();

    for path in paths {
        let audio_filename = path.unwrap().path().display().to_string();
        let audio_name: String = audio_filename.split('/').last().unwrap().to_string()
            .split('.').next().unwrap().to_string();

        println!("{} - {}", audio_name, audio_filename);
        audio_ref_cell.borrow_mut().add(
            audio_name,
            audio_filename
        ); 
    }

    audio_ref_cell.borrow_mut().play("startup");
    
    // Terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;
    // Render Loop (Seperate thread)
    let (render_tx, render_rx) = mpsc::channel();
    let render_handle = thread::spawn(move || {
        let mut last_frame = frame::new_frame();
        let mut stdout = io::stdout();
        render::render(&mut stdout, &last_frame, &last_frame, true);
        loop {
            // Blocking wait
            let current_frame = match render_rx.recv() {
                Ok(x) => x,
                Err(e) => break,
            };
            render::render(&mut stdout, &last_frame, &current_frame, false);
            last_frame = current_frame;
        }
    });

    let mut player = Player::new(Rc::clone(&audio_ref_cell));
    let mut enemies = Enemies::new(Rc::clone(&audio_ref_cell));
    let mut instant = Instant::now();
    let game_logic: GameLogic = GameLogic::new();
    // Game loop
    'gameloop: loop {
        // Per-frame init
        let mut curr_frame = new_frame();
        let delta = instant.elapsed();
        instant = Instant::now();
        

        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        audio_ref_cell.borrow_mut().play("lose");
                        break 'gameloop;
                    },
                    KeyCode::Char('w') | KeyCode::Up => {
                        player.move_up();
                        audio_ref_cell.borrow_mut().play("move");
                    },
                    KeyCode::Char('s') | KeyCode::Down => {
                        player.move_down();
                        audio_ref_cell.borrow_mut().play("move");
                    }
                    KeyCode::Char('a') | KeyCode::Left => {
                        player.move_left();
                        audio_ref_cell.borrow_mut().play("move");
                    },
                    KeyCode::Char('d') | KeyCode::Right => {
                        player.move_right();
                        audio_ref_cell.borrow_mut().play("move");
                    },
                    KeyCode::Char('f') | KeyCode::Enter => {
                        let shot = player.shoot();
                        audio_ref_cell.borrow_mut().play("pew");
                    }
                    _ => {}
                }
            }
        }

        //player.update(delta);
        enemies.update(Duration::from_micros(5000));
        player.update(Duration::from_micros(5000));
        enemies.draw(&mut curr_frame);
        player.draw(&mut curr_frame);

        // Draw & render
        let _ = render_tx.send(curr_frame);

        // Determine game state
        match game_logic.check_game_state(&mut player, &mut enemies) {
            GameState::Progress => {},
            GameState::Win => {
                audio_ref_cell.borrow_mut().play("win");
                break 'gameloop;
            },
            GameState::Lose => {
                audio_ref_cell.borrow_mut().play("lose");
                break 'gameloop;
            }
        }

        thread::sleep(Duration::from_millis(1));
    }

    // Cleanup
    drop(render_tx);
    render_handle.join().unwrap();
    
    // Cleanup
    audio_ref_cell.borrow_mut().wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    println!("Hello, world!");
    Ok(())
}

