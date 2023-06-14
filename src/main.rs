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
    dead: bool
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
            speed: 300.0
        }
    }
    fn draw(&self, d: &mut raylib::core::drawing::RaylibDrawHandle) {
        d.draw_rectangle(
            (self.x - self.width as f32 / 2.0) as i32,
            (self.y - self.height as f32 / 2.0) as i32, 
            self.width, 
            self.height, 
            Color::RED
        );
        d.draw_circle(
            self.x as i32,
            self.y as i32,
            4.0, 
            Color::GREEN
        );
    }
    fn start_moving_up(&mut self) {
        self.moving_up = true;
        self.time = 0.0;
    }
    fn stop_moving_up(&mut self) {
        self.moving_up = false;
        
        if (self.y < 360.0){self.moving_down = true;}
    }
    fn update(&mut self, dt: f32) {
        if self.moving_up && !self.moving_down {
            self.y -= self.speed * dt;
            self.time += dt;
        }
        if self.time >= 0.7 && !self.moving_down {
            self.moving_up = false;
            self.moving_down = true;
            self.time = 0.0;
            
        }
        if self.moving_down {
            self.y += self.speed * dt;
            
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
    
    rl.set_target_fps(60);
    
    let mut rng = rand::thread_rng();
    
    let mut player = Player::new(60.0, 360.0, 40, 40);
    
    let mut enemies: Vec<Enemy> = vec![];
    
    let mut timer = 0.0;
    
    enemies.push(Enemy::new(800.0, 360.0, 40,40, 300.0));
    
    let background = rl.load_texture(&thread, "./pics/back.png").unwrap(); 

    
    
    while !rl.window_should_close() {
        let dt = rl.get_frame_time();
        timer += dt;
        if timer >= 1.0 {
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
        }

            
        
        player.update(dt);
        for i in 0..enemies.len() {
            enemies[i].update(dt);

        }

        enemies.retain(|enemy| enemy.x >= 0.0);

        //check collision
        for enemy in &enemies {
            if (player.x - enemy.x).abs() < 10.0 as f32
                && (player.y - enemy.y).abs() < 10.0 as f32
            {
                player.dead = true;
            }
        }

        
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        d.draw_texture(&background, 0, 0, Color::WHITE);
        d.draw_rectangle(0, 380, 800, 260, Color::BLACK);
        if !player.dead { player.draw(&mut d); }
        for i in 0..enemies.len() {
            enemies[i].update(dt);
            d.draw_circle(
                enemies[i].x as i32,
                enemies[i].y as i32,
                10.0,
                // enemies[i].width,
                // enemies[i].height,
                Color::GREEN);
        }
        
    }
}
