extern crate sdl2;
// Engine para executar o sdl2
mod engine;
use engine::Engine;

// Todas as entidades
mod entities;
use entities::cell::Grid;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut engine = Engine::init(
        "Game".to_string(),
        [1280, 720]
    )?;

    engine.engine_loop();
    
    let mut grid = Grid::create(200, 200);

    grid.print();

    Ok(())
}