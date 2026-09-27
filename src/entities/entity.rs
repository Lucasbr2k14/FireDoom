/// Representa uma entidade, ela tem que ter essas funções para ser uma entidade dentro do "Jogo".
pub trait Entity {
    fn draw(&self, delta:f32);
    fn update(&mut self, delta:f32);
}