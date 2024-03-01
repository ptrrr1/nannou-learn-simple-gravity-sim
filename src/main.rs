use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};

struct Model {
    _window_id: WindowId,
    clear_color: Hsl,
    egui: Egui,

    planets: Vec<Planet>,
    planets_settings: PlanetSettings,
}

const GRAVITY: f32 = 6.674;

struct PlanetSettings {
    color: Rgb,
    mass: f32, // kg
    initial_vel: Vec2, // m/s
}

impl Default for PlanetSettings {
    fn default() -> PlanetSettings {
        PlanetSettings {
            color: rgb(1., 1., 1.),
            mass: 27.,
            initial_vel: Vec2::ZERO,
        }
    }
}

#[derive(Debug)]
struct Planet {
    color: Rgb,
    mass: f32, // kg
    pos: Vec2, // x, y
    vel: Vec2, // m/s
    acc: Vec2, // m/s²
}

impl Planet {
    fn new(color: Rgb, mass: f32, pos: Vec2, vel: Vec2) -> Planet {
        Planet {
            color: color,
            mass: mass,
            pos: pos,
            vel: vel,
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

fn model(_app: &App) -> Model {
    // Create a new window! Store the ID so we can refer to it later.
    let _window_id = _app
        .new_window()
        .title("nannou-app | Gravity simulation")
        .size(512u32, 512u32)
        .resizable(false)
        .mouse_pressed(mouse_pressed)
        .raw_event(raw_window_event)
        .build().unwrap();

    let clear_color = hsl(0., 0., 0.);

    let window = _app.window(_window_id).unwrap();
    let egui = Egui::from_window(&window);

    let planets = Vec::new();
    let planets_settings = Default::default();

    Model { 
        _window_id,
        clear_color,
        egui,
        
        planets,
        planets_settings,
    }
}

fn view(_app: &App, _model: &Model, _frame: Frame) {
    let draw = _app.draw();
    
    // Background color
    draw.background().color(_model.clear_color);

    // Draw Planets
    for planet in &_model.planets {
        draw.ellipse()
            .color(planet.color)
            .xy(planet.pos)
            .wh(Vec2::ONE * (planet.mass / 3.).sqrt());
    }

    // Draw to frame
    draw.to_frame(_app, &_frame).unwrap();

    // Draw GUI
    _model.egui.draw_to_frame(&_frame).unwrap();
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    // Values i wish to change or show through UI
    let planet_amout = _model.planets.len().to_string();
    let planet_settings = &mut _model.planets_settings;

    // Maybe this is stupid, but couldn't think of a better method to preserve in UI the color chosen
    let mut rgb_color: [f32; 3] = [planet_settings.color.red, planet_settings.color.green, planet_settings.color.blue];

    // GUI thingies
    let _egui = &mut _model.egui;
    _egui.set_elapsed_time(_update.since_start);

    let _ctx = _egui.begin_frame();
    
    // Creates new UI of type Window
    egui::Window::new("New Planet Configuration").show(&_ctx, |_ui| {
        //Current amount
        _ui.label("Number of Planets: ".to_owned() + &planet_amout);

        // Separator
        _ui.separator();

        // Mass Slider
        _ui.label("Mass Slider");
        _ui.add(egui::Slider::new(&mut planet_settings.mass, 27.0..=100_000.0).suffix("Kg"));

        // Initial Velocity
        _ui.label("Initial Velocity - Unit Vector");
        if _ui.add(egui::Button::new("ZERO or ONE")).clicked() {
            if planet_settings.initial_vel == Vec2::ZERO {
                planet_settings.initial_vel = Vec2::ONE;
            } else {
                planet_settings.initial_vel = Vec2::ZERO;
            }
        }

        // Color
        _ui.label("Planet Color");
        if _ui.color_edit_button_rgb(&mut rgb_color).changed() {
            planet_settings.color = Rgb::new(rgb_color[0], rgb_color[1], rgb_color[2])
        };
    });

    // Planets physics
    for planet in 0.._model.planets.len() {
        for other_planet in 0.._model.planets.len() {
            if planet != other_planet {
                let pos =_model.planets[planet].pos;
                let mass =_model.planets[planet].mass;
                let other_pos =_model.planets[other_planet].pos;
                let other_mass =_model.planets[other_planet].mass;

                // Sum of the forces
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
        MouseButton::Middle => { 
            _model.planets.push( 
                Planet::new(
                    _model.planets_settings.color, 
                    _model.planets_settings.mass, 
                    _app.mouse.position(), 
                    _model.planets_settings.initial_vel
                    ) 
                ); 
        },

        MouseButton::Right => { _model.planets.pop(); },

        _ => {}
    }
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    // Let egui handle things like keyboard and mouse input.
    model.egui.handle_raw_event(event);
}