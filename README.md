# Para executar:

## Instalar cargo e o rust
Para realizar a instalação é preciso instlar o rust e o cargo ver no link [Página oficial do rust](https://rust-lang.org/)

## Instalar dependencias do sistema operacional
Será preciso instalar as bibliotecas do sistema
### Debian
```sh
sudo apt install libsdl2-dev libsdl2-ttf-dev 
```
## Compilar
### Para compilar e executar
```sh
cargo run --release
```
### Para compilar somente
```sh
cargo build --release
```

### Para compilar em webAssambly
```sh
rustup target add asmjs-unknown-emscripten
export EMCC_CFLAGS="-s USE_SDL=2"
cargo build --target asmjs-unknown-emscripten
```

### Para debug
Você pode somente remover o ```--relase```
