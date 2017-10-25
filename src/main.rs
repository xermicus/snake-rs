extern crate rand;
extern crate pancurses;

use rand::Rng;
use std::{thread, time};
use std::io::Write;
use pancurses::Input::*;
use std::net::{TcpListener, TcpStream};

const DIM_X: usize = 42;
const DIM_Y: usize = 22;
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
    snake: Vec<(i32, i32)>,
    dir: (i32, i32),
    vel: i32,
    apple: Vec<(i32, i32)>,
    wall: Vec<(i32, i32)>
}


fn init() -> State {
    let mut state =  State {
        snake: vec![get_rand_tuple()],
        dir: (1,0),
        vel: 4,
        apple: vec![get_rand_tuple()],
        wall: Vec::new()
    };

    for x in 0..(DIM_Y) {
        for y in 0..(DIM_X) {
            if x==0 || x==DIM_Y-1 || y==0 || y==DIM_X-1 {
                state.wall.push((x as i32, y as i32));
            }
        }
    }

    state
}


fn get_rand_tuple() -> (i32, i32) {
        (rand::thread_rng().gen_range(1, DIM_Y as i32 -1), rand::thread_rng().gen_range(1, DIM_X as i32 -1))
}


fn render(s: &State, w: &pancurses::Window) {
    w.clear();

    w.attrset(pancurses::COLOR_PAIR(EMPTY_COLOR_ID as u32));
    for e in &s.wall {
        w.mvaddch(e.0, e.1, O_WALL);
    }

    w.attrset(pancurses::COLOR_PAIR(SNAKE_COLOR_ID as u32));
    for e in &s.snake {
        w.mvaddch(e.0, e.1, O_SNAKE);
    }

    w.attrset(pancurses::COLOR_PAIR(APPLE_COLOR_ID as u32));
    for e in &s.apple {
        w.mvaddch(e.0, e.1, O_APPLE);
    }

    w.attrset(pancurses::COLOR_PAIR(EMPTY_COLOR_ID as u32));
    w.mvaddch(DIM_Y as i32, DIM_X as i32, '\n');
    w.addstr("Score: ");
    w.addstr(&s.snake.len().to_string());

    w.refresh();
}


fn highscore(h: usize, w: &pancurses::Window) {
    w.attrset(pancurses::COLOR_PAIR(EMPTY_COLOR_ID as u32));
    let mut counter: usize = h;
    while counter > 1 {
        counter -= 1;
        w.clear();
        w.mvaddstr(10, 20 ,"Score: ");
        w.addstr(&(h-counter).to_string());
        w.refresh();
        thread::sleep(time::Duration::from_millis(1000 / counter as u64));
    }
    w.attrset(pancurses::COLOR_PAIR(APPLE_COLOR_ID as u32));
    w.clear();
    w.mvaddstr(10, 20 ,"Score: ");
    w.addstr(&h.to_string());
    w.refresh();
    thread::sleep(time::Duration::from_millis(3000));
}


fn menu(w: &pancurses::Window) -> std::option::Option<pancurses::Input> {
    w.clear();
    w.attrset(pancurses::COLOR_PAIR(EMPTY_COLOR_ID as u32));
    w.addstr("Welcome to sanke-rs!\n");
    w.addstr("What to do?\np\tplay\nq\tquit\n");
    w.getch()
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


fn update(mut s: &mut State, w: &pancurses::Window) -> bool {
    // tick
    let wait_fps = time::Duration::from_millis(1000 / s.vel as u64);
    thread::sleep(wait_fps);

    // input
    if let Some(keypress) = w.getch() {
        input(&mut s, keypress);
    }

    // move
    let mut next: (i32, i32) = (s.snake[0].0 + s.dir.0, s.snake[0].1 + s.dir.1);
    if s.wall.contains(&next) {
        if next.1 >= DIM_X as i32 -1 {
            next.1 = 1;
        } else if next.1 <= 1 {
            next.1 = DIM_X as i32 -2;
        } else if next.0 >= DIM_Y as i32 -1 {
            next.0 = 1;
        } else if next.0 <= 1 {
            next.0 = DIM_Y as i32 -2;
        }
    }

    // check collision
    if s.snake.contains(&next) {
        return false
    }

    // check apple
    if let Some(index) = s.apple.iter().position(|&r| r == next) {
        s.apple.remove(index);   
        s.apple.push(get_rand_tuple());
        s.vel += 1;
    } else {
        s.snake.pop();
    }

    s.snake.insert(0, next);

    true
}


fn main() {
    let window = pancurses::initscr();
    let result = std::panic::catch_unwind(|| {
        pancurses::start_color();
        pancurses::init_pair(SNAKE_COLOR_ID as i16, SNAKE_COLOR, BG_COLOR);
        pancurses::init_pair(APPLE_COLOR_ID as i16, APPLE_COLOR, BG_COLOR);
        pancurses::init_pair(EMPTY_COLOR_ID as i16, FG_COLOR, BG_COLOR);
        pancurses::nl();
        pancurses::noecho();
        pancurses::curs_set(0);
        window.nodelay(false);
        window.keypad(true);
    
        loop {
            match menu(&window) {
            Some(Character('p')) => {
                window.nodelay(true);
                let mut game_state = init();
                while update(&mut game_state, &window) {
                    render(&game_state, &window);
                }
                highscore(game_state.snake.len(), &window);
                window.nodelay(false); },
            Some(Character('q')) => break,
            _ => continue
            }
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
