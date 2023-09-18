extern crate ggez;
use ggez::conf::WindowMode;
use ggez::event::{self, EventHandler};
use ggez::graphics::Rect;
use ggez::graphics::{self, Color};
use ggez::graphics::{DrawParam, Mesh};
use ggez::input::keyboard::KeyCode;
use ggez::{Context, ContextBuilder, GameResult};

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
}

impl Vector2 {
    fn new(x: f32, y: f32) -> Vector2 {
        Vector2 { x, y }
    }
}

// impl GridPosition {
//     pub fn new(x: i16, y: i16) -> Self {
//         GridPosition { x, y }
//     }

//     pub fn new_from_move(pos: GridPosition, dir: Direction) -> Self {
//         match dir {
//             Direction::Up => GridPosition::new(pos.x, (pos.y - 1).modulo(GRID_SIZE.1)),
//             Direction::Down => GridPosition::new(pos.x, (pos.y + 1).modulo(GRID_SIZE.1)),
//             Direction::Left => GridPosition::new((pos.x - 1).modulo(GRID_SIZE.0), pos.y),
//             Direction::Right => GridPosition::new((pos.x + 1).modulo(GRID_SIZE.0), pos.y),
//         }
//     }
// }

impl Rocket {
    fn new(x: f32, y: f32) -> Rocket {
        let acceleration = Vector2::new(0.0, 0.0);
        let velocity = Vector2::new(0.0, 0.0);
        let up_force = Vector2::new(0.0, -0.009);
        let left_force = Vector2::new(-0.009, 0.0);
        let right_force = Vector2::new(0.009, 0.0);
        let weight = 3000.0;

        Rocket {
            shape: Rect::new(x, y, 60.0, 150.0),
            acceleration,
            velocity,
            up_force,
            left_force,
            right_force,
            weight,
            is_moving: false,
            fuel: 100.0,
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
        self.acceleration.y *= 0.0;
    }

    fn hit_ground(&mut self, window_height: f32) {
        if self.shape.y + self.shape.h >= window_height {
            self.velocity.y = 0.0;
            self.shape.y = window_height - self.shape.h;
        }
    }

    fn up(&mut self) {
        let up_force = self.up_force.clone();
        self.apply_force(&up_force);
    }

    fn left_movimentation(&mut self) {
        let left_force = self.left_force.clone();
        self.apply_force(&left_force);
    }

    fn rigth_movimentation(&mut self) {
        let right_force = self.right_force.clone();
        self.apply_force(&right_force);
    }
}
pub struct MyGame {
    rocket: Rocket,
    rocket_mesh: Mesh,
    gravity: Vector2,
}

impl MyGame {
    pub fn new(_context: &mut Context) -> GameResult<MyGame> {
        // Load/create resources such as images here.
        let rocket = Rocket::new(0.0, 0.0);
        let rocket_mesh = rocket.create_mesh(_context)?;
        let gravity = Vector2::new(0.0, 0.001);

        Ok(MyGame {
            rocket,
            rocket_mesh,
            gravity,
        })
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, context: &mut Context) -> GameResult {
        println!("Is moving: {}", self.rocket.is_moving);
        self.rocket.apply_force(&self.gravity);
        self.rocket.fly();
        self.rocket.hit_ground(800.0);

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

        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(context, Color::BLACK);

        canvas.draw(
            &self.rocket_mesh,
            DrawParam::default().dest([self.rocket.shape.x, self.rocket.shape.y]),
        );

        canvas.finish(context)
    }
}

fn main() {
    // Make a Context.
    let window_mode = WindowMode::default().dimensions(750.0, 800.0);
    let (mut context, event_loop) = ContextBuilder::new("Foguete", "Bianca Beppler")
        .window_mode(window_mode)
        .build()
        .expect("Could not create ggez context!");
    // graphics::set_window_title(&context, "Jogo do Foguete");

    let my_game = MyGame::new(&mut context).unwrap();

    // Run!
    event::run(context, event_loop, my_game);
}
