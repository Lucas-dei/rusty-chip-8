use chip8::*;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::{env, fs::File, io::Read};

// Scale the Display to fit modern monitors
const SCALE: u32 = 15;
const WINDOW_WIDTH: u32 = (SCREEN_WIDTH as u32) * SCALE;
const WINDOW_HEIGHT: u32 = (SCREEN_HEIGHT as u32) * SCALE;

const BLACK: Color = Color::RGB(0, 0, 0);
const WHITE: Color = Color::RGB(255, 255, 255);

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("Use \"cargo run path/to/rom\"");
        return;
    }

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Rusty Chip-8", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut chip8 = Cpu::setup_cpu();

    let mut rom =
        File::open(&args[1]).unwrap_or_else(|_| panic!("Unable to opn file {:?}", args[1]));
    let mut buffer = Vec::new();
    rom.read_to_end(&mut buffer).unwrap();
    //TODO: Better Error handling
    let _ = chip8.load_rom(&buffer);

    'gameloop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    break 'gameloop;
                }
                _ => (),
            }

            chip8.tick();
            draw_screen(&chip8, &mut canvas);
        }
    }
}

fn draw_screen(cpu: &Cpu, canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(BLACK);
    canvas.clear();

    let screen_buf = cpu.get_display();
    canvas.set_draw_color(WHITE);
    for (i, pixel) in screen_buf.iter().enumerate() {
        if *pixel {
            // Convert 1D array into 2D (x,y) position
            let x = (i % SCREEN_WIDTH) as u32;
            let y = (i / SCREEN_WIDTH) as u32;

            let rect = Rect::new((x * SCALE) as i32, (y * SCALE) as i32, SCALE, SCALE);
            canvas.fill_rect(rect).unwrap();
        }
    }
    canvas.present();
}
