use std::io;
use std::fs::File;
use std::io::BufRead;

use crate::calculator::calculator::calculate_eq;
use image::{Rgb, RgbImage};

mod calculator;

#[derive(Debug, Copy, Clone)]
struct Pixel {
    r: u8,
    g: u8,
    b: u8
}

impl Default for Pixel {
    fn default() -> Self {
        Pixel {
            r: 255,
            g: 255,
            b: 255,
        }
    }
}

fn get_cmds_from_file(file_path: &str) -> Vec<Vec<String>> {
    let file = File::open(file_path).expect("Failed to open file");
    let reader = io::BufReader::new(file);

    let mut all_cmds: Vec<Vec<String>> = vec![];

    for line in reader.lines() {
        if let Ok(line) = line {
            let unedited_cmds: Vec<&str> = line.split(',').collect();
            let mut trimmed_cmds: Vec<String> = vec![];

            for cmd in unedited_cmds {
                let trimmed_cmd = sanitize_input(&cmd.to_string());
                trimmed_cmds.push(trimmed_cmd);
            }

            all_cmds.push(trimmed_cmds);
        }
    }

    all_cmds
}

fn sanitize_input(input: &String) -> String {
    // Define a list of characters to allow
    let allowed_chars: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789/+-.,*^()% ".chars().collect();

    // Filter out disallowed characters
    let sanitized_input: String = input.chars().filter(|&c| allowed_chars.contains(&c)).collect();

    sanitized_input
}

fn add_color_change(image_width: usize, screen_range_of_y: Vec<i32>, color_change: Pixel, pixels: &Vec<Vec<Pixel>>) -> Vec<Vec<Pixel>> {
    let mut new_pixels = pixels.clone();
    for col in 0..image_width {
        if col <= screen_range_of_y.len() && screen_range_of_y[col] >= 0 {
            let idx = screen_range_of_y[col] as usize;
            let pixel = pixels[idx][col];
            let new_r: u16 = pixel.r as u16 + color_change.r as u16;
            let new_g : u16= pixel.g as u16 + color_change.g as u16;
            let new_b: u16 = pixel.b as u16 + color_change.b as u16;
            new_pixels[idx][col] = Pixel{r: new_r.clamp(0, 255) as u8, g: new_g.clamp(0, 255) as u8, b: new_b.clamp(0, 255) as u8};
        }
    }

    new_pixels
}

fn subtract_color_change(image_width: usize, screen_range_of_y: Vec<i32>, color_change: Pixel, pixels: &Vec<Vec<Pixel>>) -> Vec<Vec<Pixel>> {
    let mut new_pixels = pixels.clone();
    for col in 0..image_width {
        if col <= screen_range_of_y.len() && screen_range_of_y[col] >= 0 {
            let idx = screen_range_of_y[col] as usize;
            let pixel = pixels[idx][col];
            let new_r: i16 = pixel.r as i16 - color_change.r as i16;
            let new_g : i16= pixel.g as i16 - color_change.g as i16;
            let new_b: i16 = pixel.b as i16 - color_change.b as i16;
            new_pixels[idx][col] = Pixel{r: new_r.clamp(0, 255) as u8, g: new_g.clamp(0, 255) as u8, b: new_b.clamp(0, 255) as u8};
        }
    }

    new_pixels
}

fn set_color_change(image_width: usize, screen_range_of_y: Vec<i32>, color_change: Pixel, pixels: &Vec<Vec<Pixel>>) -> Vec<Vec<Pixel>> {
    let mut new_pixels = pixels.clone();
    for col in 0..image_width {
        if col <= screen_range_of_y.len() && screen_range_of_y[col] >= 0 {
            let idx = screen_range_of_y[col] as usize;
            let _pixel = pixels[idx][col];
            let new_r = color_change.r;
            let new_g = color_change.g;
            let new_b = color_change.b;
            new_pixels[idx][col] = Pixel{r: new_r.clamp(0, 255), g: new_g.clamp(0, 255), b: new_b.clamp(0, 255)};
        }
    }

    new_pixels
}

fn handle_cmds(cmds: Vec<String>, image_width: usize, image_height: usize, pixels: &Vec<Vec<Pixel>>) -> Vec<Vec<Pixel>> {
    const _WHITE_PIXEL: Pixel = Pixel{r:255, g:255, b:255};
    const _BLACK_PIXEL: Pixel = Pixel{r:0, g:0, b:0};

    let rgb_values: Vec<u8> = cmds[2]
    .split_whitespace()
    .filter_map(|s| s.parse().ok())
    .collect();

    let action_cmd = cmds[0].to_lowercase();

    let mut color_change: Pixel = _WHITE_PIXEL;
    if rgb_values.len() == 3 {
        color_change = Pixel {
            r: rgb_values[0],
            g: rgb_values[1],
            b: rgb_values[2],
        };
    } else {
        println!("Error: RGB string does not contain 3 values");
    }

    let range_of_y: Vec<i32> = calculate_eq(&cmds[1], image_width);
    let mut screen_range_of_y: Vec<i32> = vec![];

    for y in range_of_y {
        let new_y = (image_height - 1) as i32 - y;
        let clamped_y = new_y.clamp(-1, (image_height - 1) as i32);
        screen_range_of_y.push(clamped_y);
    }
    
    let mut new_pixels = pixels.clone();
    
    if action_cmd == "add" {
        new_pixels = add_color_change(image_width, screen_range_of_y, color_change, &pixels)
    } else if action_cmd == "subtract" {
        new_pixels = subtract_color_change(image_width, screen_range_of_y, color_change, &pixels)
    } else if action_cmd == "set" {
        new_pixels = set_color_change(image_width, screen_range_of_y, color_change, &pixels)
    } else {
        println!("INVALID CMD!");
    }

    new_pixels
}

fn main() {
    const IMAGE_WIDTH: usize = 100;
    const IMAGE_HEIGHT: usize = 100;

    const _WHITE_PIXEL: Pixel = Pixel{r:255, g:255, b:255};
    const _BLACK_PIXEL: Pixel = Pixel{r:0, g:0, b:0};
    let mut pixels: Vec<Vec<Pixel>> = vec![vec![_BLACK_PIXEL; IMAGE_WIDTH]; IMAGE_HEIGHT];

    let file_path = "./cmds.txt";
    let all_cmds = get_cmds_from_file(file_path);

    for cmds in all_cmds {
        let new_pixels = handle_cmds(cmds, IMAGE_WIDTH, IMAGE_HEIGHT, &pixels);
        pixels = new_pixels;
    }

    let mut image = RgbImage::new(IMAGE_WIDTH as u32, IMAGE_HEIGHT as u32);

    for row in 0..IMAGE_HEIGHT {
        for col in 0..IMAGE_WIDTH {
            let pixel = pixels[row][col];
            let rgb_pixel = Rgb([pixel.r, pixel.g, pixel.b]);
            image.put_pixel(col.try_into().unwrap(), row as u32, rgb_pixel);
        }
    }

    image.save("output.png").expect("Failed to save image");
}
