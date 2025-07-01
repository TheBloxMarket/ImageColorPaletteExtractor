mod utils;

use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use kmeans_colors::get_kmeans;
use utils::set_panic_hook;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// Macro for logging to console
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

// Color structure for JavaScript interop
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

#[wasm_bindgen]
impl Color {
    #[wasm_bindgen(constructor)]
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b }
    }

    #[wasm_bindgen(getter)]
    pub fn r(&self) -> u8 {
        self.r
    }

    #[wasm_bindgen(getter)]
    pub fn g(&self) -> u8 {
        self.g
    }

    #[wasm_bindgen(getter)]
    pub fn b(&self) -> u8 {
        self.b
    }

    #[wasm_bindgen]
    pub fn to_hex(&self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }

    #[wasm_bindgen]
    pub fn to_rgb_string(&self) -> String {
        format!("rgb({}, {}, {})", self.r, self.g, self.b)
    }
}

// Palette result structure
#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaletteResult {
    colors: Vec<Color>,
    percentages: Vec<f32>,
}

#[wasm_bindgen]
impl PaletteResult {
    #[wasm_bindgen(getter)]
    pub fn colors(&self) -> Vec<Color> {
        self.colors.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn percentages(&self) -> Vec<f32> {
        self.percentages.clone()
    }

    #[wasm_bindgen]
    pub fn get_color(&self, index: usize) -> Option<Color> {
        self.colors.get(index).copied()
    }

    #[wasm_bindgen]
    pub fn get_percentage(&self, index: usize) -> Option<f32> {
        self.percentages.get(index).copied()
    }

    #[wasm_bindgen]
    pub fn length(&self) -> usize {
        self.colors.len()
    }
}

// Simple RGB struct that implements the necessary traits for kmeans
#[derive(Debug, Clone, Copy, Default)]
struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

impl Rgb {
    fn new(r: u8, g: u8, b: u8) -> Self {
        Rgb { r, g, b }
    }
}

// Implement the Calculate trait for our RGB struct
impl kmeans_colors::Calculate for Rgb {
    fn get_closest_centroid(data: &[Self], centroids: &[Self], indices: &mut Vec<u8>) {
        for color in data.iter() {
            let mut closest_index = 0;
            let mut min_distance = f32::MAX;
            
            for (idx, centroid) in centroids.iter().enumerate() {
                let distance = Self::difference(color, centroid);
                if distance < min_distance {
                    min_distance = distance;
                    closest_index = idx;
                }
            }
            indices.push(closest_index as u8);
        }
    }

    fn recalculate_centroids(
        _rng: &mut impl rand::Rng,
        data: &[Self],
        centroids: &mut [Self],
        indices: &[u8],
    ) {
        for (idx, centroid) in centroids.iter_mut().enumerate() {
            let mut sum_r = 0u32;
            let mut sum_g = 0u32;
            let mut sum_b = 0u32;
            let mut count = 0u32;

            for (&cluster_idx, &color) in indices.iter().zip(data) {
                if cluster_idx as usize == idx {
                    sum_r += color.r as u32;
                    sum_g += color.g as u32;
                    sum_b += color.b as u32;
                    count += 1;
                }
            }

            if count > 0 {
                centroid.r = (sum_r / count) as u8;
                centroid.g = (sum_g / count) as u8;
                centroid.b = (sum_b / count) as u8;
            }
        }
    }

    fn check_loop(centroids: &[Self], old_centroids: &[Self]) -> f32 {
        let mut total_diff = 0.0;
        for (new, old) in centroids.iter().zip(old_centroids) {
            total_diff += Self::difference(new, old);
        }
        total_diff
    }

    fn create_random(rng: &mut impl rand::Rng) -> Self {
        Rgb::new(
            rng.gen_range(0..=255),
            rng.gen_range(0..=255),
            rng.gen_range(0..=255),
        )
    }

    fn difference(c1: &Self, c2: &Self) -> f32 {
        let dr = (c1.r as f32 - c2.r as f32).powi(2);
        let dg = (c1.g as f32 - c2.g as f32).powi(2);
        let db = (c1.b as f32 - c2.b as f32).powi(2);
        (dr + dg + db).sqrt()
    }
}

// Main palette extractor class
#[wasm_bindgen]
pub struct PaletteExtractor {
    max_iter: usize,
    converge: f32,
    verbose: bool,
}

#[wasm_bindgen]
impl PaletteExtractor {
    #[wasm_bindgen(constructor)]
    pub fn new() -> PaletteExtractor {
        set_panic_hook();
        
        PaletteExtractor {
            max_iter: 20,
            converge: 5.0,
            verbose: false,
        }
    }

    #[wasm_bindgen]
    pub fn set_max_iterations(&mut self, max_iter: usize) {
        self.max_iter = max_iter;
    }

    #[wasm_bindgen]
    pub fn set_convergence(&mut self, converge: f32) {
        self.converge = converge;
    }

    #[wasm_bindgen]
    pub fn set_verbose(&mut self, verbose: bool) {
        self.verbose = verbose;
    }

    #[wasm_bindgen]
    pub fn extract_palette_from_pixels(
        &self, 
        pixels: &[u8], 
        k: usize
    ) -> Result<PaletteResult, JsValue> {
        if pixels.len() % 4 != 0 {
            return Err(JsValue::from_str("Pixel data must be RGBA format (length divisible by 4)"));
        }

        if k == 0 {
            return Err(JsValue::from_str("Number of colors (k) must be greater than 0"));
        }

        // Convert RGBA pixels to RGB pixels for kmeans
        let rgb_pixels: Vec<Rgb> = pixels
            .chunks_exact(4)
            .map(|rgba| Rgb::new(rgba[0], rgba[1], rgba[2])) // Skip alpha channel
            .collect();

        if rgb_pixels.is_empty() {
            return Err(JsValue::from_str("No valid pixels found"));
        }

        if self.verbose {
            console_log!("Processing {} pixels for {} colors", rgb_pixels.len(), k);
        }

        // Perform k-means clustering
        let kmeans_result = get_kmeans(
            k,
            self.max_iter,
            self.converge,
            self.verbose,
            &rgb_pixels,
            0, // seed
        );

        // Convert results to our format
        let colors: Vec<Color> = kmeans_result
            .centroids
            .iter()
            .map(|centroid| Color::new(centroid.r, centroid.g, centroid.b))
            .collect();

        // Calculate percentages
        let total_pixels = rgb_pixels.len() as f32;
        let percentages: Vec<f32> = kmeans_result
            .indices
            .iter()
            .fold(vec![0; k], |mut acc, &cluster_index| {
                if (cluster_index as usize) < k {
                    acc[cluster_index as usize] += 1;
                }
                acc
            })
            .iter()
            .map(|&count| (count as f32 / total_pixels) * 100.0)
            .collect();

        Ok(PaletteResult { colors, percentages })
    }

    #[wasm_bindgen]
    pub fn extract_palette_from_image_data(
        &self,
        image_data: &[u8],
        width: u32,
        height: u32,
        k: usize,
    ) -> Result<PaletteResult, JsValue> {
        let expected_len = (width * height * 4) as usize;
        if image_data.len() != expected_len {
            return Err(JsValue::from_str(&format!(
                "Image data length {} doesn't match expected length {} for {}x{} RGBA image",
                image_data.len(), expected_len, width, height
            )));
        }

        self.extract_palette_from_pixels(image_data, k)
    }

    #[wasm_bindgen]
    pub fn extract_dominant_color(&self, pixels: &[u8]) -> Result<Color, JsValue> {
        let result = self.extract_palette_from_pixels(pixels, 1)?;
        result.get_color(0).ok_or_else(|| JsValue::from_str("Failed to extract dominant color"))
    }
}

// Utility functions
#[wasm_bindgen]
pub fn sort_colors_by_luminance(colors: Vec<Color>) -> Vec<Color> {
    let mut color_luminance: Vec<(Color, f32)> = colors
        .into_iter()
        .map(|color| {
            // Calculate relative luminance
            let r = color.r as f32 / 255.0;
            let g = color.g as f32 / 255.0;
            let b = color.b as f32 / 255.0;
            let luminance = 0.2126 * r + 0.7152 * g + 0.0722 * b;
            (color, luminance)
        })
        .collect();

    color_luminance.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    color_luminance.into_iter().map(|(color, _)| color).collect()
}

#[wasm_bindgen]
pub fn color_distance_rgb(color1: &Color, color2: &Color) -> f32 {
    let dr = (color1.r as f32 - color2.r as f32).powi(2);
    let dg = (color1.g as f32 - color2.g as f32).powi(2);
    let db = (color1.b as f32 - color2.b as f32).powi(2);
    (dr + dg + db).sqrt()
}

#[wasm_bindgen]
pub fn remove_similar_colors(colors: Vec<Color>, threshold: f32) -> Vec<Color> {
    let mut result = Vec::new();
    
    for color in colors {
        let is_similar = result.iter().any(|existing_color| {
            color_distance_rgb(&color, existing_color) < threshold
        });
        
        if !is_similar {
            result.push(color);
        }
    }
    
    result
}