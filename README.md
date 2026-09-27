# Simulação de fogo usando automatos celulares
Esse é um trabalho para a matéria de computação científica, utilizando automatos celulares.
 
## Para executar:

### Instalar cargo e o rust
Para realizar a instalação é preciso instlar o rust e o cargo ver no link [Página oficial do rust](https://rust-lang.org/)

### Instalar dependencias do sistema operacional
Será preciso instalar as bibliotecas do sistema

#### Linux
```sh
# ubuntu system dependencies
apt install pkg-config libx11-dev libxi-dev libgl1-mesa-dev libasound2-dev

# fedora system dependencies
dnf install libX11-devel libXi-devel mesa-libGL-devel alsa-lib-devel

# arch linux system dependencies
pacman -S pkg-config libx11 libxi mesa-libgl alsa-lib
```

### Compilar
#### Para compilar e executar
```sh
cargo run --release
```
#### Para compilar somente
```sh
cargo build --release
```

#### Para compilar em webAssambly
```sh

```

#### Para debug
Você pode somente remover o ```--relase```
