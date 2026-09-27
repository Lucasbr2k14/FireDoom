use std::fmt::format;

use macroquad::{
    color::WHITE, miniquad, prelude::{
        Color, Conf, clear_background, draw_text, get_fps, get_frame_time, next_frame
    }, telemetry::frame
};

// Todas as entidades
mod entities;
use entities::{
    cell::Grid,
    entity::Entity,
};

// Cores
mod colors;
use colors::get_colors;

// Configurações para iniciar o macroquad
fn init_config() -> Conf {
    Conf { 
        window_title: "Fire Doom".to_string(), 
        window_width: 1280, 
        window_height: 720, 
        fullscreen: false,
        window_resizable: false,
        // platform: miniquad::conf::Platform {
        //     swap_interval: Some(0),
        //     ..Default::default()
        // },
        
        ..Default::default()
    }
}

struct Engine {
    window_size: [u16; 2],
    window_name: String,
    frame_count: u32,
    entities: Vec<Box<dyn Entity>>,

    debug: bool
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
            ],
            entities: Vec::new(),
            debug: false
        }
    }

    /// Função a onde fica o loop da execução
    async fn run(&mut self) {
        loop {
            clear_background(Color { r: 0.0, g:0.0, b:0.0, a:1.0 });            
            
            let delta = get_frame_time();

            self.update(delta);
            
            if self.debug { self.debug(); };
            
            self.draw(delta);
            
            next_frame().await; // Solicita ao sistema operacional o próximo frame.
            self.frame_count += 1;
        }
    }
    

    /// Função a onde da todos os updates nas entidades
    fn update(&mut self, delta:f32) {
        for i in &mut self.entities {
            i.update(delta);
        }
    }
    

    /// Função para desenhar todas entidades
    fn draw(&self, delta:f32) {
        for i in &self.entities {
            i.draw(delta);
        }
    }

    
    /// Essa função é para adicionar uma entidade nova na engine
    pub fn add_entity(&mut self, entity:Box<dyn Entity>) {
        self.entities.push(entity);
    }

    
    pub fn debug(&mut self) {
        let fps = get_fps(); 
        let frame_time = get_frame_time();

        let fps_str = format!("fps: {}", fps);
        let frame_time_str = format!("frame time: {}", frame_time);
        draw_text(
            fps_str, 
            10.0, 
            40.0, 
            40.0, 
            WHITE
        );

        draw_text(
            frame_time_str, 
            10.0, 
            80.0, 
            40.0, 
            WHITE
        );
    }

    



}


/*
Para mudar o tamanho temos
request_new_screen_size(w, h)
*/

#[macroquad::main(init_config)]
async fn main() {
    let mut engine = Engine::start_from_config();

    let grid = Box::new(
        Grid::create(
            [250.0, 100.0],
            50,  // Quantidade de linhas
            50,  // Quantidade de colunas
            0.0,  // Tamanho do gap
            5.0,  // Tamanho da celular
            20.0, // 20 updates por segundo
            get_colors("FireDoom/colors.json".to_string()) // Carregar as as cores
        )
    );

    engine.add_entity(grid);

    engine.run().await;
}