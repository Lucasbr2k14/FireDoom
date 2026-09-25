use macroquad::{
    miniquad,
    prelude::{
        Conf,
        next_frame
    }
};

// Todas as entidades
mod entities;
use entities::cell::Grid;

fn init_config() -> Conf {
    Conf { 
        window_title: "Fire Doom".to_string(), 
        window_width: 1280, 
        window_height: 720, 
        fullscreen: false,
        window_resizable: false,
        platform: miniquad::conf::Platform {
            swap_interval: Some(0),
            ..Default::default()
        },
        
        ..Default::default()
    }
}

struct Engine {
    window_size: [u16; 2],
    window_name: String,
    frame_count: u32,
}

impl Engine {
    fn start_from_config() -> Self {
        let config = init_config();
        Self {
            window_name: config.window_title,
            frame_count: 0,
            window_size: [
                config.window_width as u16, 
                config.window_height as u16
            ]
        }
    }

    async fn run(&mut self) {
        loop {
            
            self.update();
            self.draw();

            next_frame().await
        }
    }
    
    fn draw(&self) {}
    fn update(&self) {}
    
}


/*
Para mudar o tamanho temos
request_new_screen_size(w, h)
*/

#[macroquad::main(init_config)]
async fn main() {
    let mut engine = Engine::start_from_config();

    engine.run().await;
    
}