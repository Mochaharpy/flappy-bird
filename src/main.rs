use macroquad::{
  input::{KeyCode::Space, MouseButton::Left},
  prelude::*,
  rand::gen_range,
};

const GRAVITY: f32 = 1200.0;
const BIRD_FLAP_FORCE: f32 = 400.0;
const BIRD_SIZE: f32 = 20.0;

const PIPE_SPACING: f32 = 200.0;
const PIPE_WIDTH: f32 = 40.0;
const PIPE_GAP_SIZE: f32 = 200.0;
const PIPE_SPEED: f32 = 100.0;

// the player
struct Bird {
  x: f32,
  y: f32,
  vel_y: f32,
  size: f32,
}

struct Pipe {
  x: f32,
  gap_center: f32,
  gap_size: f32,
  width: f32,
  speed: f32,
}

impl Pipe {
  fn new(gap_center: f32) -> Self {
    Self {
      x: screen_width() + PIPE_WIDTH,
      gap_center,
      gap_size: PIPE_GAP_SIZE,
      width: PIPE_WIDTH,
      speed: PIPE_SPEED,
    }
  }

  fn recycle(&mut self, new_x: f32) {
    self.x = new_x;
    self.gap_center = gen_range(PIPE_GAP_SIZE, screen_height() - PIPE_GAP_SIZE)
  }

  fn top_pipe(&self) -> Rect {
    Rect::new(
      self.x,
      0.0,
      self.width,
      self.gap_center - self.gap_size / 2.0,
    )
  }

  fn bottom_pipe(&self) -> Rect {
    let top_of_bottom_pipe = self.gap_center + self.gap_size / 2.0;
    Rect::new(
      self.x,
      top_of_bottom_pipe,
      self.width,
      screen_height() - top_of_bottom_pipe,
    )
  }

  fn update(&mut self, dt: f32) {
    self.x -= self.speed * dt;
  }

  fn draw(&self) {
    let top_pipe = self.top_pipe();
    let bottom_pipe = self.bottom_pipe();
    draw_rectangle(top_pipe.x, top_pipe.y, top_pipe.w, top_pipe.h, RED);
    draw_rectangle(
      bottom_pipe.x,
      bottom_pipe.y,
      bottom_pipe.w,
      bottom_pipe.h,
      RED,
    );
  }
}

impl Bird {
  // creates the bird :o
  fn new() -> Self {
    Self {
      x: 150.0,
      y: screen_height() / 2.0,
      vel_y: 0.0,
      size: BIRD_SIZE,
    }
  }

  fn reset(&mut self) {
    self.x = 150.0;
    self.y = screen_height() / 2.0;
    self.vel_y = 0.0;
  }

  fn collision_box(&self) -> Rect {
    Rect::new(
      self.x - self.size / 2.0,
      self.y - self.size / 2.0,
      self.size,
      self.size,
    )
  }

  // updates the velocity and position based on player input
  fn update(&mut self, dt: f32) -> bool {
    if is_key_pressed(Space) || is_mouse_button_pressed(Left) {
      self.vel_y = -BIRD_FLAP_FORCE;
    }

    self.vel_y += GRAVITY * dt;
    self.y += self.vel_y * dt;

    // so the bird doesnt fly away
    self.y = self
      .y
      .clamp(self.size / 2.0, screen_height() - self.size / 2.0);

    // if bird hits top, set vel to 0 so it doesnt 'stick' to the ceil
    if self.y == self.size / 2.0 {
      self.vel_y = 0.0;
    } else if self.y == screen_height() - self.size / 2.0 {
      return true;
    }
    false
  }
  // draws a rect where the bird is
  fn draw(&self) {
    draw_rectangle(
      self.x - self.size / 2.0,
      self.y - self.size / 2.0,
      self.size,
      self.size,
      GREEN,
    );
    draw_rectangle_lines(
      self.x - self.size / 2.0,
      self.y - self.size / 2.0,
      self.size,
      self.size,
      4.0,
      BLACK,
    );
  }
}

fn draw_button(label: &str, button_rect: Rect, font_size: u16) -> bool {
  let (mousex, mousey) = mouse_position();
  let is_hovered = button_rect.contains(vec2(mousex, mousey));

  let fill_color = if is_hovered { DARKGRAY } else { GRAY };

  draw_rectangle(
    button_rect.x,
    button_rect.y,
    button_rect.w,
    button_rect.h,
    fill_color,
  );

  let text_size = measure_text(label, None, font_size, 1.0);
  draw_text(
    label,
    button_rect.x + (button_rect.w - text_size.width) / 2.0,
    button_rect.y + (button_rect.h + text_size.height) / 2.0,
    font_size as f32,
    WHITE,
  );

  is_hovered && is_mouse_button_pressed(Left)
}

fn reset_pipes(pipes: &mut [Pipe]) {
  for (index, pipe) in pipes.iter_mut().enumerate() {
    pipe.recycle(screen_width() + index as f32 * PIPE_SPACING);
  }
}

fn center(start: f32, end: f32, size: f32) -> f32 {
  (end + start) / 2.0 - (size / 2.0)
}

#[derive(Clone, Copy, PartialEq)]
enum GameState {
  StartMenu,
  Playing,
  GameOver,
}

#[macroquad::main("FlappyBird")]
async fn main() {
  let pipe_count: usize = (screen_width() / PIPE_SPACING) as usize + 1;
  let mut bird = Bird::new();
  let mut pipes: Vec<Pipe> = (0..pipe_count).map(|_| Pipe::new(0.0)).collect();
  reset_pipes(&mut pipes);
  // makes it so they start so much earlier!
  for pipe in pipes.iter_mut() {
    pipe.x -= screen_width() / 2.0
  }
  let mut state = GameState::StartMenu;

  loop {
    clear_background(SKYBLUE);
    for pipe in &pipes {
      pipe.draw();
    }
    bird.draw();

    let delta_time = get_frame_time();
    match state {
      GameState::StartMenu => {
        let font_size: f32 = 50.0;
        let button_label: &str = "Start";
        let button_padding: f32 = 20.0;
        let button_size = measure_text(button_label, None, font_size as u16, 1.0);

        if draw_button(
          "Start",
          Rect::new(
            center(0.0, screen_width(), button_size.width + button_padding),
            center(0.0, screen_height(), button_size.height + button_padding),
            button_size.width + button_padding,
            button_size.height + button_padding,
          ),
          font_size as u16
        ) {
          state = GameState::Playing;
        }
      }

      GameState::Playing => {
        // bird update returns a bool; when bool is true then it hit the floor
        let hit_floor = bird.update(delta_time);
        for pipe in pipes.iter_mut() {
          pipe.update(delta_time);
        }

        // if they are further then they should go, then they get 'recycled'
        let furthest_x = pipes.iter().map(|pipe| pipe.x).fold(f32::MIN, f32::max);
        for pipe in pipes.iter_mut() {
          if pipe.x < -PIPE_WIDTH {
            pipe.recycle(furthest_x + PIPE_SPACING);
          }
        }

        // checks collisions between pipes and bird
        let bird_rect = bird.collision_box();
        let hit_pipe = pipes.iter().any(|pipe| {
          bird_rect.overlaps(&pipe.top_pipe()) || bird_rect.overlaps(&pipe.bottom_pipe())
        });

        if hit_floor || hit_pipe {
          state = GameState::GameOver;
        }
      }

      GameState::GameOver => {
        let font_size: f32 = 30.0;
        let button_label: &str = "restart";
        let button_padding: f32 = 20.0;
        let button_size = measure_text(button_label, None, font_size as u16, 1.0);
        if draw_button(
          "Restart",
          Rect::new(
            center(0.0, screen_width(), button_size.width + button_padding),
            center(0.0, screen_height(), button_size.width + button_padding),
            button_size.width + button_padding,
            button_size.height + button_padding,
          ),
          font_size as u16
        ) {
          bird.reset();
          reset_pipes(&mut pipes);
          state = GameState::Playing;
        }
      }
    }
    next_frame().await;
  }
}
