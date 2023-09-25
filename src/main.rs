extern crate ggez;
use ggez::conf::WindowMode;
use ggez::event::{self, EventHandler};
use ggez::graphics::Rect;
use ggez::graphics::{self, Color, Text};
use ggez::graphics::{DrawParam, Mesh};
use ggez::input::keyboard::KeyCode;
use ggez::{glam::*, Context, ContextBuilder, GameResult};

use std::env;

const WINDOW_BOTTOM: f32 = 390.0;
const WINDOW_Y: f32 = 800.0;
const WINDOW_H: f32 = 800.0;
const WINDOW_MIDDLE: f32 = 650.0;
const MOVIMENTATION_FORCE: f32 = 0.0009;
#[derive(Clone, Copy, PartialEq, Debug)]
struct Vector2 {
    x: f32,
    y: f32,
}

struct Rocket {
    shape: Rect,
    acceleration: Vector2,
    velocity: Vector2,
    up_force: Vector2,
    left_force: Vector2,
    right_force: Vector2,
    weight: f32,
    is_moving: bool,
    fuel: f32,
    is_flying: bool,
}

#[derive(Clone, Copy, PartialEq, Debug)]
struct Platform {
    shape: Rect,
}
impl Platform {
    fn new(x: f32, y: f32, w: f32, h: f32) -> Platform {
        return Platform {
            shape: Rect::new(x, y, w, h),
        };
    }

    fn create_mesh(&self, context: &mut Context) -> GameResult<Mesh> {
        let rect_bounds: Rect = self.shape.clone();
        let mesh = graphics::Mesh::new_rectangle(
            context,
            graphics::DrawMode::fill(),
            rect_bounds,
            Color::YELLOW,
        )?;
        Ok(mesh)
    }
}

impl Vector2 {
    fn new(x: f32, y: f32) -> Vector2 {
        Vector2 { x, y }
    }
}

impl Rocket {
    fn new(x: f32, y: f32, weight: f32, fuel: f32) -> Rocket {
        let acceleration = Vector2::new(0.0, 0.0);
        let velocity = Vector2::new(0.0, 0.0);
        let up_force = Vector2::new(0.0, -MOVIMENTATION_FORCE);
        let left_force = Vector2::new(-MOVIMENTATION_FORCE, 0.0);
        let right_force = Vector2::new(MOVIMENTATION_FORCE, 0.0);

        Rocket {
            shape: Rect::new(x, y, 60.0, 150.0),
            acceleration,
            velocity,
            up_force,
            left_force,
            right_force,
            weight,
            is_moving: false,
            fuel,
            is_flying: true,
        }
    }

    fn create_mesh(&self, context: &mut Context) -> GameResult<Mesh> {
        let rect_bounds = Rect::new(0.0, 0.0, self.shape.w, self.shape.h);

        let mesh = graphics::Mesh::new_rectangle(
            context,
            graphics::DrawMode::fill(),
            rect_bounds,
            Color::WHITE,
        )?;

        Ok(mesh)
    }

    fn apply_force(&mut self, force: &Vector2) {
        self.acceleration.x += force.x * self.weight;
        self.acceleration.y += force.y * self.weight;
    }

    fn fly(&mut self) {
        self.velocity.x += self.acceleration.x;
        self.velocity.y += self.acceleration.y;

        self.shape.x = self.velocity.x;
        self.shape.y = self.velocity.y;

        self.acceleration.x *= 0.0;
        // self.acceleration.x *= 0.0;
        // self.acceleration.y *= 0.0;
    }

    fn hit_ground(&mut self, window_height: f32) {
        if self.shape.y + self.shape.h >= window_height {
            println!("Chegou no chao");
            self.velocity.y = 0.0;
            self.shape.y = window_height - self.shape.h;
            self.is_flying = false;
        }
    }

    fn up(&mut self) {
        let up_force = self.up_force.clone();
        self.apply_force(&up_force);
        self.fuel += &up_force.y * self.weight * 0.01;
    }

    fn down_movimentation(&mut self) {
        let down_force = Vector2::new(0.0, MOVIMENTATION_FORCE);
        self.apply_force(&down_force);
        self.acceleration.y -= MOVIMENTATION_FORCE;
    }
    fn left_movimentation(&mut self) {
        let left_force = self.left_force.clone();
        self.apply_force(&left_force);
        self.fuel += &left_force.x * self.weight * 0.01;
    }

    fn rigth_movimentation(&mut self) {
        let right_force = self.right_force.clone();
        self.apply_force(&right_force);
        self.fuel -= &right_force.x * self.weight * 0.01;
    }

    fn is_running_into_platform(&self, platform: &Platform) -> bool {
        return self.shape.x >= platform.shape.x + platform.shape.w
            && self.shape.x - self.shape.w
                <= platform.shape.x + platform.shape.w + platform.shape.w
            && self.shape.y >= WINDOW_MIDDLE;
    }

    fn has_fuel(&self) -> bool {
        return !(self.fuel <= 0.0);
    }

    fn fall(&mut self, platform_h: f32) {
        self.shape.y = WINDOW_Y - self.shape.h - platform_h;
        self.is_flying = false;
    }
}
struct MyGame {
    rocket: Rocket,
    rocket_mesh: Mesh,
    gravity: Vector2,
    platform: Platform,
    platform_mesh: Mesh,
    game_state: GameState,
}

enum GameState {
    Playing,
    GameOver,
}

impl MyGame {
    pub fn new(
        _context: &mut Context,
        rocket: Rocket,
        gravity: Vector2,
        platform: Platform,
    ) -> GameResult<MyGame> {
        // Load/create resources such as images here.

        let rocket_mesh = rocket.create_mesh(_context)?;
        let platform_mesh = platform.create_mesh(_context)?;
        let game_state = GameState::Playing;

        Ok(MyGame {
            rocket,
            rocket_mesh,
            gravity,
            platform,
            platform_mesh,
            game_state,
        })
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, context: &mut Context) -> GameResult {
        match self.game_state {
            GameState::Playing => {
                if self.rocket.is_flying {
                    self.rocket.apply_force(&self.gravity);
                    self.rocket.fuel -= &self.gravity.y * self.rocket.weight * 0.01;
                    self.rocket.fly();
                    self.rocket.hit_ground(WINDOW_Y);
                } else if self.rocket.is_running_into_platform(&self.platform) {
                    println!("Parabéns, você aterrizou a nave na plataforma!");
                    self.gravity = Vector2::new(0.0, 0.0);
                    self.rocket.fall(20.0);
                    self.game_state = GameState::GameOver;
                } else {
                    println!("Perdeu o jogo, a nave caiu no chão!");
                    self.gravity = Vector2::new(0.0, 0.0);
                    self.rocket.fall(0.0);
                    self.game_state = GameState::GameOver;
                }

                if !self.rocket.has_fuel() && !self.rocket.is_running_into_platform(&self.platform)
                {
                    println!("Perdeu o jogo, acabou combustivel");
                    self.gravity = Vector2::new(0.0, 0.0);
                    self.rocket.fall(0.0);
                    self.game_state = GameState::GameOver;
                }

                if context.keyboard.is_key_just_released(KeyCode::Up)
                    || context.keyboard.is_key_just_released(KeyCode::Right)
                    || context.keyboard.is_key_just_released(KeyCode::Left)
                {
                    self.rocket.is_moving = false;
                }

                if context.keyboard.is_key_pressed(KeyCode::Up) {
                    self.rocket.is_moving = true;
                    self.rocket.up();
                }
                if context.keyboard.is_key_pressed(KeyCode::Left) {
                    self.rocket.is_moving = true;
                    self.rocket.left_movimentation();
                }
                if context.keyboard.is_key_pressed(KeyCode::Right) {
                    self.rocket.is_moving = true;
                    self.rocket.rigth_movimentation();
                }

                if context.keyboard.is_key_pressed(KeyCode::Down) {
                    self.rocket.is_moving = true;
                    self.rocket.down_movimentation();
                }
            }
            GameState::GameOver => {
                // println!("Game over");
            }
        }

        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(context, Color::from([0.02, 0.03, 0.15, 1.0]));

        let fuel_text = Text::new(format!("Combustivel: {:.2}", self.rocket.fuel));

        canvas.draw(
            &fuel_text,
            graphics::DrawParam::from([0.0, 0.0]).color(Color::WHITE),
        );

        canvas.draw(
            &self.rocket_mesh,
            DrawParam::default().dest([self.rocket.shape.x, self.rocket.shape.y]),
        );

        canvas.draw(
            &self.platform_mesh,
            DrawParam::default().dest([self.platform.shape.x, self.platform.shape.y]),
        );

        canvas.finish(context)
    }
}

fn main() {
    let window_mode = WindowMode::default().dimensions(750.0, WINDOW_H);
    let (mut context, event_loop) = ContextBuilder::new("Foguete", "Bianca Beppler")
        .window_mode(window_mode)
        .build()
        .expect("Could not create ggez context!");

    let args: Vec<String> = env::args().collect();

    let weight: f32 = match args[1].parse() {
        Ok(value) => value,
        Err(_) => {
            eprintln!("Error: Invalid weight value");
            std::process::exit(1);
        }
    };

    let fuel: f32 = match args[2].parse() {
        Ok(value) => value,
        Err(_) => {
            eprintln!("Error: Invalid fuel value");
            std::process::exit(1);
        }
    };

    let gravity: f32 = match args[3].parse() {
        Ok(value) => value,
        Err(_) => {
            eprintln!("Error: Invalid gravity value");
            std::process::exit(1);
        }
    };

    let rocket = Rocket::new(0.0, 0.0, weight, fuel);
    let platform = Platform::new(160.0, WINDOW_BOTTOM, 100.0, 30.0);
    let game_gravity = Vector2::new(0.0, gravity);

    let my_game = MyGame::new(&mut context, rocket, game_gravity, platform).unwrap();

    event::run(context, event_loop, my_game);
}
