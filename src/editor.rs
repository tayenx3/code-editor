use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, TextureCreator, TextureQuery};
use sdl2::video::{Window, WindowContext};
use sdl2::Sdl;

use crate::lexer::{to_color, tokenize};

pub const COLORS: (Color, Color, Color, Color, Color, Color, Color, Color, Color, Color, Color, Color, Color, Color) = (
    Color::RGB(27, 27, 30),     // background
    Color::RGB(125, 125, 133),  // line numbers
    Color::RGB(212, 212, 212),  // identifiers
    Color::RGB(86, 156, 214),   // keywords
    Color::RGB(206, 145, 120),  // strings
    Color::RGB(181, 206, 168),  // numbers
    Color::RGB(78, 201, 176),   // types
    Color::RGB(106, 153, 85),   // comments
    Color::RGB(220, 220, 170),  // functions
    Color::RGB(212, 212, 212),  // punctuation
    Color::RGB(197, 134, 192),  // operators
    Color::RGB(156, 220, 254),  // separators
    Color::RGB(19, 19, 21),     // help bar
    Color::RGB(245, 245, 255)   // white
);


pub struct CodeEditor {
    context: Sdl,
    canvas: Canvas<Window>,
    texture_creator: TextureCreator<WindowContext>,
    pub font_path: String,
    help_bar_size: u32
}

impl CodeEditor {
    pub fn new(font_path: &str) -> Result<Self, String> {
        // Create SDL2 context
        let context = sdl2::init()?;
        sdl2::hint::set("SDL_START_TEXT_INPUT_ON_FOCUS", "1");

        // Create video subsystem
        let vsub = context.video()?;

        // Create window
        let window = vsub.window(
            "Keystone Editor",
            900, 700
        )
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;
   
        // Create canvas
        #[allow(unused_mut)]
        let mut canvas = window
        .into_canvas()
        .build()
        .map_err(
            |e| e.to_string()
        )?;

        // Create Texture Creator
        let texture_creator = canvas.texture_creator();

        // Return the built Code Editor
        Ok(Self {
            context,
            canvas,
            texture_creator,
            font_path: font_path.to_string(),
            help_bar_size: 35
        })
    }

    pub fn edit(&mut self, script: &mut String, file_name: &str) -> Result<(), String> {
        // Create TTF context
        let ttf_context = sdl2
        ::ttf
        ::init()
        .map_err(
            |e| 
            e
        )?;

        // Setup font
        let font = ttf_context.load_font(&self.font_path, 20)?;

        // Keep window open
        let video = self.context.video()?;
        video.text_input().start();


        let mut event_pump = self.context.event_pump()?;
        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    sdl2::event::Event::Quit {..} => break 'running,
                    sdl2::event::Event::TextInput { text, .. } => {
                        script.push_str(&text);
                    }
                    sdl2::event::Event::KeyDown { keycode: Some(key), .. } => {
                        match key {
                            Keycode::Escape => break 'running,
                            Keycode::Return => script.push('\n'),
                            Keycode::Backspace => { script.pop(); },
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }

            let mut y = self.help_bar_size as i32 + 5;

            self.canvas.set_draw_color(COLORS.0);
            self.canvas.clear();

            self.canvas.set_draw_color(COLORS.12);
            self.canvas.fill_rect(Rect::new(0, 0, 900, self.help_bar_size))?;
            let help_bar_surface = font.render(file_name)
            .blended(COLORS.13)
            .map_err(|e| e.to_string())?;
            let help_bar_texture = help_bar_surface.as_texture(&self.texture_creator)
            .map_err(|e| e.to_string())?;
            let TextureQuery { width, height, .. } = help_bar_texture.query();
            let target = Rect::new(5, 5, width, height);

            self.canvas.copy(&help_bar_texture, None, Some(target))?;

            for (i, line) in script
            .split('\n')
            .collect::<Vec<_>>()
            .iter()
            .enumerate() 
            {
                let i = i + 1;
                let mut x = 5;
                let w = i.checked_ilog10().unwrap_or(0) + 1;
                let gutter_width = w as i32 * font.size_of_char('0').unwrap().0 as i32 + 10;
                // Render this line
                let surface = font.render(&format!("{:>width$}", i, width = w as usize))
                    .blended(COLORS.1)
                    .map_err(|e| e.to_string())?;

                let texture = surface.as_texture(&self.texture_creator)
                    .map_err(|e| e.to_string())?;

                let TextureQuery { width, height, .. } = texture.query();
                let target = Rect::new(x, y, width, height);

                self.canvas.copy(&texture, None, Some(target))?;
                x += gutter_width;

                if line.is_empty() {
                    y += height as i32;
                    continue
                }

                let tokens = tokenize(line);

                for token in tokens {
                    if token.1.trim().is_empty() {
                        x += font.size_of_char(' ').unwrap().0 as i32; // still advance slightly for spacing
                        continue;
                    }
                    let code_surface = font.render(&token.1)
                    .blended(
                        to_color(token.0)
                    )
                    .map_err(|e| e.to_string())?;
                    let code_texture = code_surface.as_texture(&self.texture_creator)
                        .map_err(|e| e.to_string())?;
                    let TextureQuery { width: code_w, .. } = code_texture.query();
                    let code_target = Rect::new(x, y, code_w, height);
                    self.canvas.copy(&code_texture, None, Some(code_target))?;
                    x += code_w as i32;
                }

                y += height as i32;
            }
            self.canvas.present();
        }

        Ok(())
    }

    #[allow(dead_code)]
    pub fn change_font(&mut self, new_path: &str) {
        self.font_path = new_path.to_string()
    }
}