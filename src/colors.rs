// Esse arquivo serve somente para pegar as cores que estão configuradas no arquivo json
use macroquad::{
    prelude::Color
    
};

use serde::{
    Serialize, 
    Deserialize
};

use std::fs::File;
use std::io::prelude::*;

/// Minha struct somente para criar o json e serializar ele
#[derive(Debug, Serialize, Deserialize)]
struct MyColor {
    r: f32,
    g: f32,
    b: f32,
}

pub fn get_colors(file:String) -> Vec<Color> {
    
    let mut content = include_str!("../FireDoom/colors.json");

    let values:Vec<MyColor> = serde_json::from_str(&content)
    .expect("Erro ao desserializar o json colors.json");

    values.iter()
    .map(
        |v| Color::new(
            v.r / 255.0 as f32,
            v.g / 255.0 as f32,
            v.b / 255.0 as f32, 
            1.0
        )
    )
    .collect()
}
