use sdl2::{
    Sdl,
    VideoSubsystem,
    EventPump,
    render::Canvas,
    video::Window,
    event::Event,
    keyboard::Keycode,
};

pub struct Engine {
    pub resolution: [u32; 2], // Para criar um array com duas posições de resolução.
    pub running: bool,
    pub frame_count: u128,
    
    // Dentro da estrutura para criar o sdl
    pub sdl: Sdl,
    pub canvas: Canvas<Window>,
    pub video_system: VideoSubsystem,
    pub events: EventPump,
}

impl Engine {
    pub fn init(
        title_name: String,
        resolution: [u32; 2],
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let init = sdl2::init()?;
        
        let video = init.video()?;

        let window = video.window(
            &title_name,
            resolution[0],
            resolution[1]
        )
        .position_centered()
        .build()?;

        let canvas = window
        .into_canvas()
        .build()?;
        
        let event_pump = init.event_pump()?;
        
        Ok(
            Self {
                resolution: resolution,
                running: true,
                frame_count: 0,
                sdl: init,
                canvas: canvas,
                events: event_pump,
                video_system: video,
            }
        )
    }

    pub fn engine_loop(&mut self){
    
        while self.running {
            self.events();

            self.frame_count += 1;
        }

    }

    fn events(&mut self) {
        let mut quit = false;

        for event in self.events.poll_iter() {
        
            match event {
        
                Event::Quit { .. } | Event::KeyDown { 
                    keycode: Some(Keycode::ESCAPE), .. 
                } => quit = true,
                
                _ => {}

            }
        }
    
        if quit { self.quit(); }
    }

    fn quit(&mut self) {
        self.running = false;
    }

}