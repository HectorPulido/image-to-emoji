extern crate knn;

use std::env;
use image::open;
use serde::Deserialize;
use knn::PointCloud;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;


const CONFI_FILE: &str = "config.json";

#[derive(Debug, Deserialize)]
struct Emoji {
    character: Option<String>,
    color: Vec<u8>
}

impl Emoji {
    fn distance_func(p: &Emoji, q: &Emoji) -> f64 {
        let f = ((q.color[0] as f64) - (p.color[0] as f64)).powf(2.0) 
            + ((q.color[1] as f64) - (p.color[1] as f64)).powf(2.0) 
            + ((q.color[2] as f64) - (p.color[2] as f64)).powf(2.0);
        return f.powf(0.5);
    }

    fn read_emoji_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<Emoji>, Box<dyn Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let u = serde_json::from_reader(reader)?;
        Ok(u)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let image_url = args.get(1);
    if image_url == None {
        println!("Please write an image path");
        return;
    }
    let image_url = image_url.unwrap();

    let mut pc = PointCloud::new(Emoji::distance_func);
    let emojis: Vec<Emoji> = Emoji::read_emoji_from_file(CONFI_FILE).expect("JSON was not well-formatted");

    for i in 0..emojis.len() {
        pc.add_point(&emojis[i]);
    }

    let mut img =  open(image_url).unwrap();

    let width = img.width();
    let height = img.height();

    if let Some(size_percentage) = args.get(2) {
        let size_percentage = size_percentage.trim().parse::<f32>().unwrap();
        img = img.resize(
            (width as f32 * size_percentage) as u32,
            (height as f32 * size_percentage) as u32, 
            image::imageops::Nearest);
    }

    let width = img.width();
    let height = img.height();

    let img_rgb = img.into_rgba8();
    for x in 0..height {
        for y in 0..width {
            let p = img_rgb.get_pixel(y, x);
            let emoji = Emoji { color: vec![p[0], p[1], p[2]], character:None};
            let nearest_emoji = pc.get_nearest_k(&emoji, 1);
            let nearest_emoji = nearest_emoji[0].1.character.as_ref().unwrap();
            print!("{nearest_emoji}");
        }
        println!("");
    }
}
