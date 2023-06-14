extern crate rand;
extern crate raylib;

use rand::Rng;
use raylib::prelude::*;

struct Player {
    x: f32,
    y: f32,
    width: i32,
    height: i32,
    moving_up: bool,
    moving_down: bool,
    time: f32,
    speed: f32,
    dead: bool,
    rotation_angle: f32
}


struct Enemy{
    x: f32,
    y: f32,
    width: i32,
    height: i32,
    speed: f32
}

impl Enemy{
    fn new(x: f32, y: f32, width: i32, height: i32, speed: f32) -> Self{
        Self{
            x,
            y,
            width,
            height,
            speed
        }
    }
    fn update(&mut self, dt: f32){
        self.x -= self.speed * dt;
    }
}
enum Control{
    Up,
    Down,
    Left,
    Right,
    StopUp
}

impl Player {
    fn new(x: f32, y: f32, width: i32, height: i32) -> Self {
        Self {
            x,
            y,
            width,
            height,
            moving_up: false,
            moving_down: false,
            time: 0.0,
            dead: false,
            speed: 300.0,
            rotation_angle: 0.0
        }
    }
    fn draw(&self, d: &mut raylib::core::drawing::RaylibDrawHandle, texture: &Texture2D) {
         // Specify your rotation angle here

        let source_rec = Rectangle::new(0.0, 0.0, texture.width() as f32, texture.height() as f32);
        let dest_rec = Vector2::new(self.x, self.y);
        let origin = Vector2::new(texture.width() as f32 / 2.0, texture.height() as f32 / 2.0); // rotate around the center of the texture

    d.draw_texture_pro(
        &texture,
        source_rec,
        Rectangle::new(dest_rec.x - self.width as f32 / 2.0, (dest_rec.y - self.height as f32 / 2.0) + 20.0, self.width as f32, self.height as f32),
        origin,
        self.rotation_angle,
        Color::WHITE,
    );
        // d.draw_rectangle(
        //     (self.x - self.width as f32 / 2.0) as i32,
        //     (self.y - self.height as f32 / 2.0) as i32, 
        //     self.width, 
        //     self.height, 
        //     Color::RED
        // );
        // d.draw_circle(
        //     self.x as i32,
        //     self.y as i32,
        //     25.0, 
        //     Color::GREEN
        // );
    }
    fn start_moving_up(&mut self) {
        self.moving_up = true;
        self.time = 0.0;
        self.rotation_angle = 0.0;
    }
    fn stop_moving_up(&mut self) {
        self.moving_up = false;
        
        if (self.y < 360.0){self.moving_down = true;}
    }
    fn update(&mut self, dt: f32) {
        if self.moving_up && !self.moving_down {
            self.y -= self.speed * dt;
            self.time += dt;
            
            self.rotation_angle += 1000.0 * dt;
        }
        if self.time >= 0.7 && !self.moving_down {
            self.moving_up = false;
            self.moving_down = true;
            self.time = 0.0;
            
        }
        if self.moving_down {
            self.y += self.speed * dt;
            self.rotation_angle -= 1000.0 * dt;
            
        }
        if self.y >= 360.0 {
            self.moving_down = false;
        }
    }

    fn control(&mut self, command: Control){
        match command {
            Control::Up => 
            {
                if(!self.moving_down && !self.moving_up){self.start_moving_up()}
            },
            Control::StopUp => 
            {
                self.stop_moving_up();
            }
            Control::Down => println!("Moving Down"),
            Control::Left => println!("Moving Left"),
            Control::Right => println!("Moving Right"),
        }
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .title("Shifting Shadows")
        .build();
    
    
    
    let mut rng = rand::thread_rng();
    
    let mut player = Player::new(60.0, 360.0, 39, 39);
    
    let mut enemies: Vec<Enemy> = vec![];
    
    let mut timer = 0.0;
    let mut score = 0;
    enemies.push(Enemy::new(800.0, 360.0, 40,40, 300.0));
    
    
    let background = rl.load_texture(&thread, "./pics/back.png").unwrap(); 
    let playerTexture = rl.load_texture(&thread, "./pics/player.png").unwrap(); 
    let fireBallTexture = rl.load_texture(&thread, "./pics/fireball.png").unwrap(); 
    
    


    let width = rl.get_screen_width();
    let height = rl.get_screen_height();
    
    while !rl.window_should_close() {
        let dt = rl.get_frame_time();
        timer += dt;
        if timer >= 0.5 && !player.dead {
            let value: f32 = rng.gen_range(240.0..360.0);
            enemies.push(Enemy::new(800.0, value, 40,40, 300.0));
            timer = 0.0;
        }
        if rl.is_key_down(KeyboardKey::KEY_W) {
            player.control(Control::Up);
        }else if !rl.is_key_down(KeyboardKey::KEY_W){
            player.control(Control::StopUp);
        }
        
        if rl.is_key_down(KeyboardKey::KEY_S) {
            player.control(Control::Down);
        }
        if rl.is_key_down(KeyboardKey::KEY_A) {
            player.control(Control::Left);
        }
        if rl.is_key_down(KeyboardKey::KEY_D) {
            player.control(Control::Right);
        }
        if rl.is_key_pressed(KeyboardKey::KEY_R){
            player.dead = false;
            enemies.clear();
            score = 0;
        }

            
        
        player.update(dt);
        for i in 0..enemies.len() {
            enemies[i].update(dt);

        }

        for i in (0..enemies.len()).rev() {
            if enemies[i].x < 0.0 {
                enemies.remove(i);
                score += 1;
            }
        }

        //check collision
        let mut to_remove = vec![];
        for (i, enemy) in enemies.iter().enumerate() {
            if (player.x - enemy.x).abs() < 25.0 as f32
                && (player.y - enemy.y).abs() < 25.0 as f32
            {
                to_remove.push(i);
                player.dead = true;
            }
        }
        if player.dead {
            
            if enemies.len() > 0 {
                enemies.remove(0);
            }
        }

        
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        d.draw_texture(&background, 0, 0, Color::WHITE);
      
        d.draw_rectangle(0, 380, 800, 260, Color::BLACK);
        if !player.dead { player.draw(&mut d, &playerTexture); }
        if !player.dead{for i in 0..enemies.len() {
            enemies[i].update(dt);
            d.draw_circle(
                enemies[i].x as i32,
                enemies[i].y as i32,
                10.0,
                // enemies[i].width,
                // enemies[i].height,
                Color::GREEN);
            
        }}
        let mut text = format!("Score: {}", score);
        d.draw_text(&text, 10, 10, 48, Color::WHITE);
        if (player.dead){
            
            d.draw_text("Press R to restart", width/8, height/2,48, Color::WHITE);
        }
    }
}
