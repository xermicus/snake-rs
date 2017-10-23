extern crate rand;
extern crate pancurses;

use rand::Rng;
use std::{thread, time};
use std::io::Write;
use pancurses::Input::*;

const DIM_X: usize = 42;
const DIM_Y: usize = 22;
const FPS: u64 = 4;
const SNAKE_COLOR_ID: u16 = 1;
const APPLE_COLOR_ID: u16 = 2;
const EMPTY_COLOR_ID: u16 = 3;
const SNAKE_COLOR: i16 = pancurses::COLOR_GREEN;
const APPLE_COLOR: i16 = pancurses::COLOR_RED;
const BG_COLOR: i16 = pancurses::COLOR_BLACK;
const FG_COLOR: i16 = pancurses::COLOR_WHITE;
const O_APPLE: char = 'o';
const O_SNAKE: char = 'O';
const O_WALL: char = '#';


struct State {
    field: [[char; DIM_X]; DIM_Y],
    snake: Vec<(i8, i8)>,
    dir: (i8, i8),
    vel: (i8, i8),
    apple: Vec<(i8, i8)>,
}


fn init() -> State {
    let mut state =  State {
        field: [[' '; DIM_X]; DIM_Y],
        snake: vec![(rand::thread_rng().gen_range(1, DIM_Y as i8 -1), rand::thread_rng().gen_range(1, DIM_X as i8 -1))],
        dir: (1,0),
        vel: (1,1),
        apple: vec![(rand::thread_rng().gen_range(1, DIM_Y as i8 -1), rand::thread_rng().gen_range(1, DIM_X as i8 -1))],
    };

    for x in 0..(DIM_Y) {
        for y in 0..(DIM_X) {
            if x==0 || x==DIM_Y-1 || y==0 || y==DIM_X-1 {
                state.field[x][y] = O_WALL;
            }
        }
    }
    state.field[state.snake[0].0 as usize][state.snake[0].1 as usize] = O_SNAKE;
    state.field[state.apple[0].0 as usize][state.apple[0].1 as usize] = O_APPLE;

    state
}


fn render(s: &State, w: &pancurses::Window) {
    w.clear();
    w.addstr("Score: ");
    w.addstr(&s.snake.len().to_string());
    w.addstr("\n\n");
    for line in s.field.iter() {
        for tile in line.iter() {
            match tile {
                &O_SNAKE => w.attrset(pancurses::COLOR_PAIR(SNAKE_COLOR_ID as u32)),
                &O_APPLE => w.attrset(pancurses::COLOR_PAIR(APPLE_COLOR_ID as u32)),
                _ => w.attrset(pancurses::COLOR_PAIR(EMPTY_COLOR_ID as u32))
            };
            w.addch(tile.to_owned());
        }
        w.addch('\n');
    }
    w.refresh();
}



fn input(s: &mut State, input: pancurses::Input) {
    s.dir = match input {
        KeyUp    => (-1, 0),
        KeyDown  => (1, 0),
        KeyLeft  => (0, -1),
        KeyRight => (0, 1),
        _ => return
    }
}


fn update(mut s: &mut State, w: &pancurses::Window) {
    // tick
    let wait_fps = time::Duration::from_millis(1000 / FPS);
    thread::sleep(wait_fps);

    // input
    if let Some(keypress) = w.getch() {
        input(&mut s, keypress);
    }

    s.snake[0].0 += s.dir.0;
    s.snake[0].1 += s.dir.1;
    s.field[s.snake[0].0 as usize][s.snake[0].1 as usize] = O_SNAKE;
}

fn main() {
    let window = pancurses::initscr();
    let result = std::panic::catch_unwind(|| {
        let mut game_state = init();

        pancurses::start_color();
        pancurses::init_pair(SNAKE_COLOR_ID as i16, SNAKE_COLOR, BG_COLOR);
        pancurses::init_pair(APPLE_COLOR_ID as i16, APPLE_COLOR, BG_COLOR);
        pancurses::init_pair(EMPTY_COLOR_ID as i16, FG_COLOR, BG_COLOR);
        pancurses::nl();
        pancurses::noecho();
        pancurses::curs_set(0);
        window.nodelay(true);
        window.keypad(true);
    
        loop {
            update(&mut game_state, &window);
            render(&game_state, &window);
        }
    });

    pancurses::endwin();
        if let Err(e) = result {
        if let Some(e) = e.downcast_ref::<&'static str>() {
            writeln!(&mut std::io::stderr(), "Error: {}", e).unwrap();
        } else {
            writeln!(&mut std::io::stderr(), "Unknown error: {:?}", e).unwrap();
        }
        std::process::exit(1);
    }
}
