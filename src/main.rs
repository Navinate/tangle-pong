use macroquad::prelude::*;
use macroquad::rand::*;

const PADDLE_SIZE: Vec2 = Vec2::from_array([20.0, 300.0]);
const PADDLE_SPEED: f32 = 600f32;

#[derive(Clone, Copy)]
pub enum Side {
    Left,
    Right,
}

#[derive(Clone, Copy)]
pub struct Paddle {
    rect: Rect,
    side: Side,
}

impl Paddle {
    pub fn new(side: Side) -> Self {
        Self {
            rect: Rect::new(
                match side {
                    Side::Left => screen_width() * 0.05f32,
                    Side::Right => screen_width() * 0.95f32,
                },
                screen_height() * 0.5f32 - PADDLE_SIZE.y / 2f32,
                PADDLE_SIZE.x,
                PADDLE_SIZE.y,
            ),
            side,
        }
    }

    pub fn update(&mut self, dt: f32) {
        let up_key: KeyCode = match self.side {
            Side::Left => KeyCode::W,
            Side::Right => KeyCode::Up,
        };
        let down_key: KeyCode = match self.side {
            Side::Left => KeyCode::S,
            Side::Right => KeyCode::Down,
        };
        let y_move = match (is_key_down(up_key), is_key_down(down_key)) {
            (true, false) => -1f32,
            (false, true) => 1f32,
            _ => 0f32,
        };
        self.rect.y += y_move * dt * PADDLE_SPEED;

        if self.rect.y < 0f32 {
            self.rect.y = screen_height();
        }
        if self.rect.y > screen_height() {
            self.rect.y = 0f32;
        }
    }

    pub fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, WHITE);
    }
}

pub struct Ball {
    pos: Vec2,
    vel: Vec2,
    radius: f32,
}

impl Ball {
    pub fn new(radius: f32, vel: Vec2) -> Self {
        Self {
            pos: [screen_width() / 2f32, screen_height() / 2f32].into(),
            vel: [vel.x, vel.y].into(),
            radius,
        }
    }

    pub fn update(&mut self, left: Paddle, right: Paddle, dt: f32) {
        if self.check_intersect(left) || self.check_intersect(right) {
            self.vel.x *= -1.01;
        }

        if self.pos.y + self.radius > screen_height() || self.pos.y < self.radius {
            self.vel.y *= -1.0;
        }
        if self.pos.x < 0.0 || self.pos.x > screen_width() {
            self.pos.x = screen_width() / 2.0;
            self.pos.y = screen_height() / 2.0;
            self.vel.x = f32::gen_range(-1.0, 1.0).signum() * 400.0;
            self.vel.y = f32::gen_range(50.0, 150.0);
        }

        self.pos.x += self.vel.x * dt;
        self.pos.y += self.vel.y * dt;
    }

    pub fn check_intersect(&self, paddle: Paddle) -> bool {
        paddle.rect.overlaps(&Rect::new(
            self.pos.x - self.radius,
            self.pos.y - self.radius,
            self.radius * 2.0,
            self.radius * 2.0,
        ))
    }

    pub fn draw(&self) {
        draw_circle(self.pos.x + 0.2, self.pos.y, self.radius, BLUE);
        draw_circle(self.pos.x - 0.2, self.pos.y, self.radius, RED);
        draw_circle(self.pos.x, self.pos.y, self.radius, WHITE);
    }
}

#[macroquad::main("Pong")]
async fn main() {
    let mut left_paddle = Paddle::new(Side::Left);
    let mut right_paddle = Paddle::new(Side::Right);
    let mut ball = Ball::new(20.0, [500.0, 150.0].into());

    loop {
        clear_background(BLACK);
        left_paddle.update(get_frame_time());
        right_paddle.update(get_frame_time());
        ball.update(left_paddle, right_paddle, get_frame_time());
        left_paddle.draw();
        right_paddle.draw();
        ball.draw();
        next_frame().await
    }
}
