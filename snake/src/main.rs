extern crate glutin_window;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;
use rand::Rng;

use std::collections::LinkedList;
use std::iter::FromIterator;

const PIXEL_SIZE: i32 = 20;

const WORLD_LIMIT: i32 = 29;
const SQUARE_SIZE : f64 = 20_f64;

#[derive(Clone, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, PartialEq)]
enum SnakeState {
    Alive,
    Dead,
}

struct Game {
    gl: GlGraphics,
    snake: Snake,
    food: Food,
}

#[allow(non_snake_case)]
impl Game {
    fn render(&mut self, arg: &RenderArgs) {
        let BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
        self.gl.draw(arg.viewport(), |_c, gl| {
            graphics::clear(BLUE, gl);
        });

        self.snake.render(&mut self.gl, arg);
        self.food.render(&mut self.gl, arg);
    }

    fn update(&mut self) {
        let eaten: bool = self.snake.update(&self.food);
        if eaten {
            self.food.update();
        }
    }

    fn pressed(&mut self, btn: &Button) {
        self.snake.pressed(btn);
        self.update();
    }
}

struct Snake {
    body: LinkedList<(i32, i32)>,
    dir: Direction,
    snakestate : SnakeState,
}

#[allow(non_snake_case)]
impl Snake {
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        let RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        let GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

        let squares: Vec<graphics::types::Rectangle> = self
            .body
            .iter()
            .map(|&(x, y)| {
                graphics::rectangle::square(
                    (x * PIXEL_SIZE) as f64,
                    (y * PIXEL_SIZE) as f64,
                    SQUARE_SIZE,
                )
            })
            .collect();

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;
            for (i, square) in squares.into_iter().enumerate() {
                if i == 0 {
                    graphics::rectangle(GREEN, square, transform, gl)
                } else {
                    graphics::rectangle(RED, square, transform, gl)
                }
            }
        });
    }

    fn update(&mut self, food: &Food) -> bool {
        let mut new_head = (*self.body.front().expect("Snake has no body")).clone();
        match self.dir {
            Direction::Up => {
                new_head.1 -= 1;
                if new_head.1 < 0 {
                    new_head.1 = WORLD_LIMIT;
                }
                if new_head.1 > WORLD_LIMIT {
                    new_head.1 = 0;
                }
            }
            Direction::Down => {
                new_head.1 += 1;
                if new_head.1 < 0 {
                    new_head.1 = WORLD_LIMIT;
                }
                if new_head.1 > WORLD_LIMIT {
                    new_head.1 = 0;
                }
            }
            Direction::Left => {
                new_head.0 -= 1;
                if new_head.0 < 0 {
                    new_head.0 = WORLD_LIMIT;
                }
                if new_head.0 > WORLD_LIMIT {
                    new_head.0 = 0;
                }
            }
            Direction::Right => {
                new_head.0 += 1;
                if new_head.0 < 0 {
                    new_head.0 = WORLD_LIMIT;
                }
                if new_head.0 > WORLD_LIMIT {
                    new_head.0 = 0;
                }
            }
        }
        if self.body.contains(&new_head) {
            self.snakestate = SnakeState::Dead;
        }
        self.body.push_front(new_head);
        if new_head.0 == food.x && new_head.1 == food.y {
            return true;
        } else {
            self.body.pop_back().unwrap();
            return false;
        }
    }

    fn pressed(&mut self, btn: &Button) {
        let last_direction = self.dir.clone();

        self.dir = match btn {
            &Button::Keyboard(Key::Up) if last_direction != Direction::Down => Direction::Up,
            &Button::Keyboard(Key::Down) if last_direction != Direction::Up => Direction::Down,
            &Button::Keyboard(Key::Left) if last_direction != Direction::Right => Direction::Left,
            &Button::Keyboard(Key::Right) if last_direction != Direction::Left => Direction::Right,
            _ => last_direction,
        }
    }
}

struct Food {
    x: i32,
    y: i32,
}

#[allow(non_snake_case)]
impl Food {
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        let WHITE = [1.0, 1.0, 1.0, 1.0];
        let square = graphics::rectangle::square(
            (self.x * PIXEL_SIZE) as f64,
            (self.y * PIXEL_SIZE) as f64,
            SQUARE_SIZE,
        );

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;
            graphics::rectangle(WHITE, square, transform, gl);
        });
    }

    fn update(&mut self) {
        self.x = rand::thread_rng().gen_range(0, WORLD_LIMIT+1);
        self.y = rand::thread_rng().gen_range(0, WORLD_LIMIT+1);
    }
}

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: GlutinWindow = WindowSettings::new("snake", [600, 600])
        .graphics_api(opengl)
        .exit_on_esc(true)
        //.fullscreen(true)
        .build()
        .unwrap();

    let mut game = Game {
        gl: GlGraphics::new(opengl),
        snake: Snake {
            body: LinkedList::from_iter((vec![(0, 0)]).into_iter()),
            dir: Direction::Right,
            snakestate: SnakeState::Alive,
        },
        food: Food {
            x: rand::thread_rng().gen_range(0, WORLD_LIMIT+1),
            y: rand::thread_rng().gen_range(0, WORLD_LIMIT+1),
        },
    };

    let mut events = Events::new(EventSettings::new()).ups(10);
    while game.snake.snakestate == SnakeState::Alive {
        if let Some(e) = events.next(&mut window) {
            if let Some(r) = e.render_args() {
                game.render(&r);
            }

            if let Some(_u) = e.update_args() {
                game.update();
            }

            if let Some(k) = e.button_args() {
                if k.state == ButtonState::Press {
                    game.pressed(&k.button);
                }
            }
        }
    }
    println!("Game Over");
    println!("You ate {} blocks!", game.snake.body.len()-1);
}
