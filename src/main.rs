use nannou::prelude::*;

#[derive(Debug)]
struct Model {
    _window_id: WindowId,
    clear_color: Hsl,

    planets: Vec<Planet>,
}

const GRAVITY: f32 = 6.674;

#[derive(Debug)]
struct Planet {
    mass: f32, // kg
    pos: Vec2, // x, y
    vel: Vec2, // m/s
    acc: Vec2, // m/s²
}

impl Planet {
    fn new(pos: Vec2) -> Planet {
        Planet {
            mass: 100.,
            pos: pos,
            vel: Vec2::ZERO,
            acc: Vec2::ZERO,
        }
    }

    fn update_acc(pos: Vec2, other_pos: Vec2, other_mass: f32) -> Vec2{
        let direction = (other_pos - pos).normalize();
        let squared_dis = pos.distance_squared(other_pos);

        return -1. * GRAVITY * other_mass * direction / squared_dis;
    }
}

fn main() {
    nannou::app(model)
        .view(view) // The function that will be called for presenting graphics to a frame.
        .update(update) // Called every frame
        .run();
}

fn model(app: &App) -> Model {
    // Create a new window! Store the ID so we can refer to it later.
    let _window_id = app
        .new_window()
        .title("nannou-app | Gravity simulation")
        .size(512u32, 512u32)
        .resizable(false)
        .mouse_pressed(mouse_pressed)
        .build().unwrap();

    let clear_color = hsl(0., 0., 0.);

    let planets = Vec::new();

    Model { 
        _window_id,
        clear_color,
        
        planets,
    }
}

fn view(_app: &App, _model: &Model, _frame: Frame) {
    let draw = _app.draw();
    
    // Background color
    draw.background().color(_model.clear_color);

    // Draw Planets
    for planet in &_model.planets {
        draw.ellipse()
            .color(BLUEVIOLET)
            .xy(planet.pos)
            .wh(Vec2::ONE * planet.mass / 10.);
    }

    // Draw to frame
    draw.to_frame(_app, &_frame).unwrap();
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    for planet in 0.._model.planets.len() {
        for other_planet in 0.._model.planets.len() {
            if planet != other_planet {
                let pos = _model.planets[planet].pos;
                let other_mass = _model.planets[other_planet].mass;
                let other_pos = _model.planets[other_planet].pos;
                let new_acc = Planet::update_acc(pos, other_pos, other_mass); // Calculate the new acc

                _model.planets[other_planet].acc = new_acc;
            }
        }

        // Updates the position
        // Idk if there's a better method other than creating a new variable
        // Rust screams at me when i do otherwise
        let new_acc = _model.planets[planet].acc.clamp_length_max(1.);
        _model.planets[planet].vel += new_acc;
        _model.planets[planet].vel.clamp_length_max(5.);
        let new_vel = _model.planets[planet].vel;
        _model.planets[planet].pos += new_vel;
    }
}

fn mouse_pressed(_app: &App, _model: &mut Model, _button: MouseButton) {
    match _button {
        MouseButton::Left => { _model.planets.push( Planet::new(_app.mouse.position()) ) },

        MouseButton::Right => { _model.planets.pop(); println!("Planets left: {}", _model.planets.len()); },

        _ => {}
    }
}