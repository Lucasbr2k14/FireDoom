use macroquad::color;
// Esse arquivo serve somente para pegar as cores que estão configuradas no arquivo json
use macroquad::prelude::Color;

use serde::{
    Serialize, 
    Deserialize
};

use std::fs::File;
use std::io::prelude::*;
use std::ops::ControlFlow;

/// Minha struct somente para criar o json e serializar ele
#[derive(Debug, Serialize, Deserialize)]
struct MyColor {
    r: f32,
    g: f32,
    b: f32,
}

pub fn get(file:String) -> Vec<Color> {
    
    let mut file = File::open("FireDoom/colors.json")
    .expect("Erro ao abrir o arquivo FireDoom/colors.json");
    
    let mut content:String = String::new();

    let _ = file.read_to_string(&mut content);

    let values:Vec<MyColor> = serde_json::from_str(&content)
    .expect("Erro ao desserializar o json colors.json");

    values.iter()
    .map(
        |v| Color::new(v.r as f32, v.g as f32, v.b as f32, 1.0)
    )
    .collect()

}