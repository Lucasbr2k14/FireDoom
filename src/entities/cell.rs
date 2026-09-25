// Aqui temos uma struct para criar o grid
// Com contagens de linhas e colunas
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
    rigth: Option<u8>,
}

impl Vizinhos {
    fn default() -> Self {
        Self { up:None, down:None, left:None, rigth:None }
    }
}


impl Grid {
    pub fn create(rows:usize, cols:usize, gap:f32, size:f32) -> Self {
        let mut grid = vec![vec![0u8; cols]; rows];
        
        // for i in 0..( cols ) {
        //    grid[rows - 1][i] = 255;
        // }

        Self { grid, rows, cols, gap, size} // Construindo a struct para usar durante a execução
    }

    pub fn print(&mut self) {
        for i in self.grid.iter() {
            println!("{:?}", i);
        }
    }

    pub fn step() {
        
    }


    pub fn draw(&mut self) {
        for i in &self.grid {
            println!("{:?}", i);
        }
    }

    // fn get_vizinhos(&mut self, x:u32, y:u32) -> Vizinhos {
    //     // Essa função é responsável para criar a struct
    //     // para utilizar dentro de update_by_position

    //     let cols = self.cols;
    //     let rows = self.rows;

    //     let mut vi = Vizinhos::default();

    //     // Saber se x e y estão dentro dos limites do grid
    //     // Retornar tudo nulo se não estiver no grid
    //     if ( x < 0 ) || ( x >= cols ) || ( y < 0 ) || ( y >= rows ) {
    //         return vi;
    //     }
        
    //     // Verificando se existe vizinho acima
    //     if ( (x + 1) <= cols ) && ( (x + 1) >= 0 ) {
    //         vi.up = Some(self.grid[x + 1][y])
    //     }


    //     if ( (x - 1) <= cols ) && ( (x - 1) >= 0 ) {
    //         vi.down = Some(self.grid[x - 1][y])
    //     }


    //     vi
    // }


    fn save(){}
    fn load(){}

}
