use enigo::{
    Button, Coordinate,
    Direction::{Click, Press, Release},
    Enigo, Key, Keyboard, Mouse, Settings
};
use autopilot;
use autopilot::geometry::Point;
use device_query::{DeviceQuery, DeviceState, Keycode};
use std::thread;
use std::time::Duration;
use std::process;

fn main() {
    let device_state = DeviceState::new();
    let mut x_coord = 35;
    let mut y_coord = 200;
    loop {
        let keys: Vec<Keycode> = device_state.get_keys();
        if keys.contains(&Keycode::K) {
            process::exit(0);
        }
        get_flask(&mut x_coord, &mut y_coord);
        roll_flask();
        thread::sleep(Duration::from_millis(100));
    }

}

fn get_flask(x_coord: &mut i32, y_coord: &mut i32) {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    // move cursor to quad tab
    enigo.move_mouse(275, 152, Coordinate::Abs);
    thread::sleep(Duration::from_millis(500));
    enigo.button(Button::Left, Click);
    // move cursor to flask
    enigo.move_mouse(*x_coord, *y_coord, Coordinate::Abs);
    thread::sleep(Duration::from_millis(500));
    enigo.button(Button::Left, Click);
    *x_coord += 35;
    if *x_coord == 840 {
        *y_coord += 70;
        *x_coord = 35;
    }
    // move cursor to currency tab
    enigo.move_mouse(90, 150, Coordinate::Abs);
    thread::sleep(Duration::from_millis(500));
    enigo.button(Button::Left, Click);
    // place flask
    enigo.move_mouse(400, 600, Coordinate::Abs);
    thread::sleep(Duration::from_millis(500));
    enigo.button(Button::Left, Click);
}

fn roll_flask() {
    let device_state = DeviceState::new();
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    let point = Point::new(500.0, 700.0);
    // two if statements for the pixel scan because the scan is too slow and will roll again
    loop {
        let keys: Vec<Keycode> = device_state.get_keys();
        let pixel = autopilot::screen::get_color(point).unwrap();
        if (pixel.0[0] == 231) & (pixel.0[1] == 180) & (pixel.0[2] == 119){
            enigo.key(Key::Control, Press);
            enigo.button(Button::Left, Click);
            enigo.move_mouse(2600, 850, Coordinate::Abs);
            thread::sleep(Duration::from_millis(500));
            enigo.button(Button::Left, Click);
            enigo.key(Key::Control, Release);
            break;
        }
        // grab alteration orb
        enigo.move_mouse(150, 350, Coordinate::Abs);
        thread::sleep(Duration::from_millis(500));
        enigo.button(Button::Right, Click);
        // use orb on flask
        enigo.move_mouse(400, 600, Coordinate::Abs);
        thread::sleep(Duration::from_millis(500));
        enigo.button(Button::Left, Click);
        let pixel = autopilot::screen::get_color(point).unwrap();
        println!(
            "Pixel color at 500, 700: {},{},{},{}",
            pixel.0[0], pixel.0[1], pixel.0[2], pixel.0[3]
        );
        // check for pixel match on boarder
        if (pixel.0[0] == 231) & (pixel.0[1] == 180) & (pixel.0[2] == 119){
            enigo.key(Key::Control, Press);
            enigo.button(Button::Left, Click);
            enigo.move_mouse(2600, 850, Coordinate::Abs);
            thread::sleep(Duration::from_millis(500));
            enigo.button(Button::Left, Click);
            enigo.key(Key::Control, Release);
            break;
        }
        if keys.contains(&Keycode::K) {
            process::exit(0);
        }
        thread::sleep(Duration::from_millis(1500));
    }

}