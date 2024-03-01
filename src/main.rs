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
    fn new(pos: Vec2, mass: f32) -> Planet {
        Planet {
            mass: mass,
            pos: pos,
            vel: Vec2::ZERO,
            acc: Vec2::ZERO,
        }
    }

    fn gravity_force(pos: Vec2, mass: f32, other_pos: Vec2, other_mass: f32) -> Vec2{
        let direction = (other_pos - pos).clamp_length(1., 1.);
        let squared_dis = pos.distance_squared(other_pos);

        return GRAVITY * mass * other_mass * direction / squared_dis;
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
            .color(PLUM)
            .xy(planet.pos)
            .wh(Vec2::ONE * (planet.mass / 3.).sqrt());
    }

    // Draw to frame
    draw.to_frame(_app, &_frame).unwrap();
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    for planet in 0.._model.planets.len() {
        for other_planet in 0.._model.planets.len() {
            if planet != other_planet {
                let pos =_model.planets[planet].pos;
                let mass =_model.planets[planet].mass;
                let other_pos =_model.planets[other_planet].pos;
                let other_mass =_model.planets[other_planet].mass;

                _model.planets[planet].acc += Planet::gravity_force(pos, mass, other_pos, other_mass);
            }
        }

        _model.planets[planet].acc = _model.planets[planet].acc.normalize_or_zero();
        let acc = _model.planets[planet].acc / _model.planets[planet].mass ;
        _model.planets[planet].vel += acc;
        let vel = _model.planets[planet].vel.clamp_length_max(5.);
        _model.planets[planet].pos += vel;

    }
}

fn mouse_pressed(_app: &App, _model: &mut Model, _button: MouseButton) {
    match _button {
        MouseButton::Left => { _model.planets.push( Planet::new(_app.mouse.position(), 27.) ) },

        MouseButton::Middle => { _model.planets.push( Planet::new(_app.mouse.position(), 10_000.) ) },

        MouseButton::Right => { _model.planets.pop(); println!("Planets left: {}", _model.planets.len()); },

        _ => {}
    }
}