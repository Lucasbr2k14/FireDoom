use macroquad::{
    miniquad, prelude::{
        Color,
        Conf,
        clear_background,
        next_frame,
        get_frame_time
    }
};

// Todas as entidades
mod entities;
use entities::{
    cell::Grid,
    entity::Entity,
};

// Configurações para iniciar o macroquad
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
    entities: Vec<Box<dyn Entity>>,
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
        }
    }

    async fn run(&mut self) {
        loop {
            clear_background(Color { r: 0.0, g:0.0, b:0.0, a:1.0 });            
            
            let delta = get_frame_time();

            self.update(delta);
            self.draw(delta);
            
            next_frame().await
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
}


/*
Para mudar o tamanho temos
request_new_screen_size(w, h)
*/

#[macroquad::main(init_config)]
async fn main() {
    let mut engine = Engine::start_from_config();

    engine.add_entity(
        Box::new(Grid::create(50, 50, 1., 5.))
    );

    engine.run().await;
}