use raylib::prelude::*;
use std::ops::Add;

mod drawing;

const MAX_AXIS: f32 = 1.0;
const MOVE_KEYS: [raylib::consts::KeyboardKey; 4] = [
    KeyboardKey::KEY_DOWN,
    KeyboardKey::KEY_UP,
    KeyboardKey::KEY_LEFT,
    KeyboardKey::KEY_RIGHT,
];

struct Player {
    position: Vector3,
    acceleration: f32,
    velocity: Vector3,
}

impl Player {
    fn new(initial_position: Vector3) -> Player {
        Player{
            position: initial_position,
            acceleration: 0.0,
            velocity: Vector3::new(0.0, 0.0, 0.0),
        }

    }
}

trait DirectionKeys {
    fn direction_keys_pressed(&mut self) -> bool;
}

impl DirectionKeys for RaylibHandle {
    fn direction_keys_pressed(&mut self) -> bool{
        for key in MOVE_KEYS {
            if self.is_key_down(key){
                return true;
            }
        }
        return false;
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("Hello, World")
        .resizable()
        .build();

    let mut player = Player::new(Vector3::new(0.0, 2.0, 0.0));
    let mut camera = Camera3D::perspective(
        Vector3::new(0.0, 20.0, -20.0),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        40.0,
    );

    let model = rl.load_model(&thread, "resources/house_1.obj").unwrap();

    rl.set_target_fps(60);
    rl.set_exit_key(None);

    let mut custom_red = Color::RED;
    custom_red.a = 122;
    
    // Prepare timing functions 
    // --------------------------
    let mut deltatime = 0.0;
    let mut time_start = 0.0;
    let mut time_end = 0.0;
    let mut rotation = 0.0;
    let mut friction_power = 20.0;
    let mut friction_direction: Vector3;
    
    while !rl.window_should_close() {
        time_start = rl.get_time();
        // Start update routines
        // --------------------------------------------------------------------
        let mut position_axis = Vector3::new(0.0, 0.0, 0.0);
        // Get key events
        if rl.is_key_down(KeyboardKey::KEY_DOWN){
            position_axis.z = -MAX_AXIS;
            player.acceleration = position_axis.z * 6000.0 * deltatime;
            if player.velocity.z >= -8.0 {
                player.velocity.z += player.acceleration * deltatime;
            }
        }

        else if rl.is_key_down(KeyboardKey::KEY_UP){
            position_axis.z = MAX_AXIS;
            player.acceleration = position_axis.z * 6000.0 * deltatime;
            if player.velocity.z <= 8.0 {
                player.velocity.z += player.acceleration * deltatime;
            }
        }
        
        if rl.is_key_down(KeyboardKey::KEY_RIGHT){
            position_axis.x = -MAX_AXIS;
            player.acceleration = position_axis.x * 6000.0 * deltatime;
            if player.velocity.x >= -8.0 {
                player.velocity.x += player.acceleration * deltatime;
            }
        }

        else if rl.is_key_down(KeyboardKey::KEY_LEFT){
            position_axis.x = MAX_AXIS;
            player.acceleration = position_axis.x * 6000.0 * deltatime;
            if player.velocity.x <= 8.0 {
                player.velocity.x += player.acceleration * deltatime;
            }
        } 
        
        if !rl.direction_keys_pressed() {
            if player.velocity.length() > deltatime * friction_power {
                let v_direction = player.velocity / player.velocity.length();
                friction_direction = -v_direction;
                player.velocity += friction_direction * 12.0 * deltatime;
            } else {
                player.velocity = Vector3::zero();
            }
        }

        player.position += player.velocity * deltatime;
        camera.target = camera.target.lerp(player.position, deltatime * 5.0);
        println!("{}: {}", player.position.x, player.position.z);
        // Draw routine
        {
            let mut d = rl.begin_drawing(&thread);
            d.clear_background(Color::RAYWHITE);
            unsafe {
                let mut d_3D = d.begin_mode3D(camera);
                    drawing::draw_cube(player.position, 2.0, 2.0, 2.0, rotation, custom_red);
                    drawing::draw_cube_wires(player.position, 2.0, 2.0, 2.0, rotation, Color::BLACK);
                    
                    d_3D.draw_model_ex(&model, 
                        Vector3::new(7.0, 3.0, 7.0), 
                        Vector3::new(0.0, 1.0, 0.0),
                        225.0,
                        Vector3::new(0.035, 0.035, 0.035), 
                        Color::DARKGRAY);
                    d_3D.draw_model_wires_ex(&model, 
                        Vector3::new(7.0, 3.0, 7.0), 
                        Vector3::new(0.0, 1.0, 0.0),
                        225.0,
                        Vector3::new(0.035, 0.035, 0.035),
                        Color::BLACK);
                    d_3D.draw_grid(10, 1.0);
            
            }
        }

        // End update routines
        // --------------------------------------------------------------------
        time_end = rl.get_time();
        deltatime = (time_end - time_start) as f32;        
    }
}
