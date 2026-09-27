use macroquad::{
    shapes::draw_rectangle,
    color::WHITE,
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
}

pub struct Vizinhos {
    up:    Option<u8>,
    down:  Option<u8>,
    left:  Option<u8>,
    right: Option<u8>,
}

impl Vizinhos {
    fn default() -> Self {
        Self { up:None, down:None, left:None, right:None }
    }
}


impl Grid {
    /// Função construtora para criar a matriz e depois rendenizar 
    pub fn create(rows:usize, cols:usize, gap:f32, size:f32) -> Self {

        Self { 
            grid: vec![vec![0u8; cols]; rows], 
            rows: rows,
            cols: cols,
            gap:  gap,
            size: size
        }
    }

    /// Essa função retorna todos os vizinhos de um ponto da grid
    /// Mas os vizinhos sempre são {cima, baixo, esquerda, direita}
    /// sendo associado a Vizinhança de Von neumann
    fn get_vizinhos(&mut self, x: usize, y: usize) -> Vizinhos {
        let cols = self.cols;
        let rows = self.rows;

        let mut vi = Vizinhos::default();

        // Verifica se x e y estão dentro dos limites do grid
        if x >= cols || y >= rows {
            return vi;
        }

        // Vizinho de cima
        if x + 1 < cols {
            vi.up = Some(self.grid[x + 1][y]);
        }

        // Vizinho de baixo
        if x > 0 {
            vi.down = Some(self.grid[x - 1][y]);
        }

        // Vizinho da direita
        if y + 1 < rows {
            vi.right = Some(self.grid[x][y + 1]);
        }

        // Vizinho da esquerda
        if y > 0 {
            vi.left = Some(self.grid[x][y - 1]);
        }

        vi
    }

    fn save() {}
    fn load() {}
}

impl Entity for Grid {

    /// Essa função serve para desenhar o grid na tela
    /// Para cada celula desenhar na tela
    fn draw(&self, delta:f32) {
        for v in 0..self.grid.len() {
            for c in 0..self.grid[v].len() {
                draw_rectangle(
                    (self.size + self.gap) * (c + 1) as f32, // x
                    (self.size + self.gap) * (v + 1) as f32, // y
                    self.size, 
                    self.size, 
                    WHITE
                );
            }
        }
    }
    
    /// Cada quadro chama o update
    /// Mas é bom fazer uma lógica que acada x quadros chama o step
    /// Porque a pessoa pode diminuir a quantidade de steps
    /// Para fazer a simulação rodar mais rápido ou mais lento
    /// Mas o mais rápido que ele pode ir é a quantidade de fps que está setado nas configurações
    fn update(&mut self, delta:f32) {}
}