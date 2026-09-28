use macroquad::{
    shapes::draw_rectangle,
    color::Color,
    color::WHITE,
    rand::gen_range
};

use crate::entities::entity::Entity;

/// Struct para o grid
/// # Exemplo de uso
///'''
///Grid::new(50, 50, 1.0, 1.0);
///'''
///
#[derive(Debug)]
pub struct Grid {
    pub grid: Vec<Vec<u8>>,
    pub rows: usize,
    pub cols: usize,
    pub size: f32,
    pub gap:  f32,

    position: [f32; 2],
    updates_ps: f32,
    sum_time: f32,
    color_pallet: Vec<Color>,
    auto_update: bool,
}

impl Grid {
    /// Função construtora para criar a matriz e depois rendenizar 
    pub fn create(
        position:[f32;2],
        rows:usize,
        cols:usize,
        gap:f32,
        size:f32,
        updates_ps:f32,
        colors:Vec<Color>
    ) -> Self {
        Self { 
            grid: vec![vec![0u8; cols]; rows], // <- aqui eu vou ter que concertar porque provavelmente é ao contrario seria rows e cols
            rows: rows,
            cols: cols,
            gap:  gap,
            size: size,
            updates_ps: 1.0 / updates_ps,
            color_pallet: colors,
            sum_time: 0.0,
            auto_update: true,
            position:position
        }
    }

    /// Função para receber o x,y e retorna a temperatura
    fn get_temp(&self, x:usize, y:usize) -> u8 {
        self.grid[x][y]
    }

    fn set_rand(&mut self, x:usize ,y:usize) {
        self.grid[x][y] = gen_range(0, self.color_pallet.len() ) as u8;
    }

    fn set_pos(&mut self, x:usize, y:usize, v:u8) {
        self.grid[x][y] = v;
    }


    /// Essa função é aonde está toda a lógica do automato
    /// Cria novos morre e se propaga
    pub fn step(&mut self) {
        let cols = self.cols;
        let rows = self.rows;

        for x in 0..cols {
            for y in 0..rows {
                // A última linha recebe novos valores aleatórios
                if y == rows - 1 {
                    self.set_rand(x, y);
                    continue;
                }

                let down = self.get_temp(x, y + 1);

                let value = down.saturating_sub(1);

                self.set_pos(x, y, value);
            }
        }
    }
    

    pub fn save(&mut self, ) {

    }


    pub fn load() {}
}

impl Entity for Grid {

    /// Essa função serve para desenhar o grid na tela
    /// Para cada celula desenhar na tela
    fn draw(&self, delta: f32) {
        for y in 0..self.grid.len() {
            for x in 0..self.grid[y].len() {
                println!("A");
                draw_rectangle(
                    ((self.size + self.gap) * x as f32) + self.position[0],
                    ((self.size + self.gap) * y as f32) + self.position[1],
                    self.size,
                    self.size,
                    self.color_pallet[
                        self.get_temp(x, y) as usize
                    ],
                );
            }
        }
    }
    
    /// Cada quadro chama o update
    /// Mas é bom fazer uma lógica que acada x quadros chama o step
    /// Porque a pessoa pode diminuir a quantidade de steps
    /// Para fazer a simulação rodar mais rápido ou mais lento
    /// Mas o mais rápido que ele pode ir é a quantidade de fps que está setado nas configurações
    fn update(&mut self, delta:f32) {
        self.sum_time += delta;

        if self.auto_update && self.sum_time >= self.updates_ps {
            self.step();
            self.sum_time = 0.0;
        }

    }
}