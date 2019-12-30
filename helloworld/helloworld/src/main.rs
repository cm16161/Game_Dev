extern crate piston_window;

use piston_window::*;

const WINDOW_WIDTH: f64 = 1920_f64;
const WINDOW_HEIGHT: f64 = 200_f64;

fn get_center(string: &str, glyphs: &mut Glyphs) -> f64 {
    use piston_window::character::CharacterCache;
    let w = glyphs.width(32, string).unwrap();
    return (WINDOW_WIDTH - w) / 2.0;
}

fn print_text(glyphs: &mut Glyphs, string: &str, size: u32, c: &Context, g: &mut G2d) {
    let transform = c
        .transform
        .trans(get_center(string, glyphs), WINDOW_HEIGHT / 2.0);

    

    text::Text::new(size)
        .draw(string, glyphs, &c.draw_state, transform, g)
        .unwrap();
}

fn main() {
    let mut window: PistonWindow =
        WindowSettings::new("Hello World", [WINDOW_WIDTH, WINDOW_HEIGHT])
            .exit_on_esc(true)
            .build()
            .unwrap();

    let mut glyphs = window.load_font("assets/FiraSans-Regular.ttf").unwrap();
    window.set_lazy(true);

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, device| {
            clear([0.0, 1.0, 1.0, 1.0], g);
            print_text(&mut glyphs, "World!", 32, &c, g);
            glyphs.factory.encoder.flush(device);
        });
    }
}
