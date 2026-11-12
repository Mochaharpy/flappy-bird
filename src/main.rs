use macroquad::{
  input::{
    KeyCode::{Space, Up, W},
    MouseButton::Left,
  },
  prelude::*,
  rand::gen_range,
};

const GAME_HEIGHT: f32 = 817.0;
const GAME_WIDTH: f32 = 1920.0;

const GRAVITY: f32 = 1200.0;
const BIRD_FLAP_FORCE: f32 = 400.0;
const BIRD_SIZE: f32 = 20.0;

const PIPE_SPACING: f32 = 200.0;
const PIPE_WIDTH: f32 = 40.0;
const PIPE_GAP_SIZE: f32 = 200.0;
const PIPE_SPEED: f32 = 1000.0;

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
  passed: bool,
}

impl Pipe {
  fn new(gap_center: f32) -> Self {
    Self {
      x: GAME_WIDTH + PIPE_WIDTH,
      gap_center,
      gap_size: PIPE_GAP_SIZE,
      width: PIPE_WIDTH,
      speed: PIPE_SPEED,
      passed: false,
    }
  }

  fn recycle(&mut self, new_x: f32) {
    self.x = new_x;
    self.passed = false;
    self.gap_center = gen_range(PIPE_GAP_SIZE, GAME_HEIGHT - PIPE_GAP_SIZE)
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
      GAME_HEIGHT - top_of_bottom_pipe,
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
      y: GAME_HEIGHT / 2.0,
      vel_y: 0.0,
      size: BIRD_SIZE,
    }
  }

  fn reset(&mut self) {
    self.x = 150.0;
    self.y = GAME_HEIGHT / 2.0;
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
    if is_key_pressed(Space)
      || is_mouse_button_pressed(Left)
      || is_key_pressed(W)
      || is_key_pressed(Up)
    {
      self.vel_y = -BIRD_FLAP_FORCE;
    }

    self.vel_y += GRAVITY * dt;
    self.y += self.vel_y * dt;

    // so the bird doesnt fly away
    self.y = self
      .y
      .clamp(self.size / 2.0, GAME_HEIGHT - self.size / 2.0);

    // if bird hits top, set vel to 0 so it doesnt 'stick' to the ceil
    if self.y == self.size / 2.0 {
      self.vel_y = 0.0;
    } else if self.y == GAME_HEIGHT - self.size / 2.0 {
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

fn calculate_letterbox_scale() -> f32 {
  (screen_width() / GAME_WIDTH).min(screen_height() / GAME_HEIGHT)
}

fn calculate_letterbox_offset() -> Vec2 {
  let scale = calculate_letterbox_scale();
  vec2(
    (screen_width() - GAME_WIDTH * scale) / 2.0,
    (screen_height() - GAME_HEIGHT * scale) / 2.0,
  )
}

fn game_mouse_position() -> Vec2 {
  let (mouse_x, mouse_y) = mouse_position();
  (vec2(mouse_x, mouse_y) - calculate_letterbox_offset()) / calculate_letterbox_scale()
}

fn draw_button(label: &str, button_rect: Rect, font_size: u16) -> bool {
  let is_hovered = button_rect.contains(game_mouse_position());

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
    pipe.recycle(GAME_WIDTH / 2.0 + index as f32 * PIPE_SPACING);
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
  let mut score: u32 = 0;
  let pipe_count: usize = (GAME_WIDTH / PIPE_SPACING) as usize + 1;
  let mut bird = Bird::new();
  let mut pipes: Vec<Pipe> = (0..pipe_count).map(|_| Pipe::new(0.0)).collect();
  reset_pipes(&mut pipes);

  let mut state = GameState::StartMenu;
let render_target = render_target(GAME_WIDTH as u32, GAME_HEIGHT as u32);
render_target.texture.set_filter(FilterMode::Linear);

let mut game_camera = Camera2D::from_display_rect(Rect::new(0.0, 0.0, GAME_WIDTH, GAME_HEIGHT));
game_camera.render_target = Some(render_target.clone());
  loop {
    set_camera(&game_camera);
    clear_background(SKYBLUE);

    for pipe in &pipes {
      pipe.draw();
    }
    bird.draw();
    draw_text(&format!("Score: {}", score), 20.0, 40.0, 36.0, WHITE);
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
            center(0.0, GAME_WIDTH, button_size.width + button_padding),
            center(0.0, GAME_HEIGHT, button_size.height + button_padding),
            button_size.width + button_padding,
            button_size.height + button_padding,
          ),
          font_size as u16,
        ) {
          state = GameState::Playing;
        }
      }

      GameState::Playing => {
        // bird update returns a bool; when bool is true then it hit the floor
        let hit_floor = bird.update(delta_time);
        for pipe in pipes.iter_mut() {
          pipe.update(delta_time);

          if !pipe.passed && pipe.x + pipe.width < bird.x {
            pipe.passed = true;
            score = score + 1;
          }
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
        let button_label: &str = "Restart";
        let button_padding: f32 = 20.0;
        let button_size = measure_text(button_label, None, font_size as u16, 1.0);
        if draw_button(
          button_label,
          Rect::new(
            center(0.0, GAME_WIDTH, button_size.width + button_padding),
            center(0.0, GAME_HEIGHT, button_size.width + button_padding),
            button_size.width + button_padding,
            button_size.height + button_padding,
          ),
          font_size as u16,
        ) {
          score = 0;
          bird.reset();
          reset_pipes(&mut pipes);
          state = GameState::Playing;
        }
      }
    }
    set_default_camera();
clear_background(BLACK);

let scale = calculate_letterbox_scale();
let offset = calculate_letterbox_offset();
draw_texture_ex(
  &render_target.texture,
  offset.x,
  offset.y,
  WHITE,
  DrawTextureParams {
    dest_size: Some(vec2(GAME_WIDTH * scale, GAME_HEIGHT * scale)),
    flip_y: true,
    ..Default::default()
  },
);

next_frame().await;
  }
}
