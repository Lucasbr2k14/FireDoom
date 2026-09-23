#[derive(Debug)]
// Aqui temos uma struct para criar o grid
// Com contagens de linhas e colunas
pub struct Grid {
    pub grid: Vec<Vec<u8>>,
    pub rows: usize,
    pub cols: usize 
}

pub struct Vizinhos {
    up:    Option<u8>,
    down:  Option<u8>,
    left:  Option<u8>,
    rigth: Option<u8>
}

impl Vizinhos {
    fn default() -> Self {
        Self { up:None, down:None, left:None, rigth:None }
    }
}


impl Grid {
    pub fn create(rows:usize, cols:usize) -> Self {
        let mut grid = vec![vec![0u8; cols]; rows];
        
        for i in 0..( cols ) {
           grid[rows - 1][i] = 255;
        }

        Self { grid, rows, cols } // Construindo a struct para usar durante a execução
    }

    pub fn print(&mut self) {
        for i in self.grid.iter() {
            println!("{:?}", i);
        }
    }

    pub fn step() {
        
    }


    pub fn draw() {

    }

    fn update_by_position(i: u32) {
        
    }

    fn get_vizinhos(&mut self, x:u32, y:u32) -> Vizinhos {
        // Essa função é responsável para criar a struct
        // para utilizar dentro de update_by_position

        let cols = self.cols as u32;
        let rows = self.rows as u32;

        let vi = Vizinhos::default();

        // Saber se x e y estão dentro dos limites do grid
        // Retornar tudo nulo se não estiver no grid

        if ( x < 0 ) || ( x >= cols ) || ( y < 0 ) || ( y >= rows ) {
            return vi;
        }

        // Verificando se existe vizinho acima
        vi.up = Some(self.grid[][])


        vi
    }

}
