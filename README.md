# Simulação de fogo usando automatos celulares
Esse é um trabalho para a matéria de computação científica, utilizando automatos celulares.
 
## Libs usadas
### Macroquad
Foi usada para gerar os gráficos e para os inputs do usuários.

### Serde e serde json
Para serializar o json para entrada e saída do programa com json.

## Como instlar e executar

### Instalar cargo e o rust
Para realizar a instalação é preciso instlar o rust e o cargo ver no link [Página oficial do rust](https://rust-lang.org/).

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
# É preciso adicionar o target primeiro para depois compilar
rustup target add wasm32-unknown-unknown
# E agora é só compilar co código
cargo build --target wasm32-unknown-unknown --release
```

Para executar precisamos abrir um servidor web para usar você vai usar o arquivo que está em target no seu html ```target/wasm32-unknown-unknown/release/fogo-doom.wasm```, mas isso já está dentro do arquivo `web/index.html` não será preciso modificar o arquivo.

Você pode rodar com o simple server de python3
```
python3 -m http.server
```
E no seu navegador você precisa acessar `localhost:8000/web` já será possível testar.

#### Para debug
Você pode somente remover o ```--relase```

