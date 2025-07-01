//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;
use image_color_palette_extractor::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_palette_extractor_creation() {
    let _extractor = PaletteExtractor::new();
    // Test that we can create the extractor without errors
    assert!(true, "PaletteExtractor created successfully");
}

#[wasm_bindgen_test]
fn test_color_creation() {
    let color = Color::new(255, 128, 64);
    assert_eq!(color.r(), 255);
    assert_eq!(color.g(), 128);
    assert_eq!(color.b(), 64);
    assert_eq!(color.to_hex(), "#ff8040");
    assert_eq!(color.to_rgb_string(), "rgb(255, 128, 64)");
}

#[wasm_bindgen_test]
fn test_palette_extraction_simple() {
    let extractor = PaletteExtractor::new();
    
    // Create a simple 2x2 RGBA image with red and blue pixels
    let pixels = vec![
        255, 0, 0, 255,   // Red pixel
        0, 0, 255, 255,   // Blue pixel
        255, 0, 0, 255,   // Red pixel
        0, 0, 255, 255,   // Blue pixel
    ];
    
    let result = extractor.extract_palette_from_pixels(&pixels, 2);
    assert!(result.is_ok(), "Palette extraction should succeed");
    
    let palette = result.unwrap();
    assert_eq!(palette.length(), 2, "Should extract 2 colors");
    
    // Check that percentages sum to approximately 100%
    let total_percentage: f32 = palette.percentages().iter().sum();
    assert!((total_percentage - 100.0).abs() < 1.0, "Percentages should sum to ~100%");
}

#[wasm_bindgen_test]
fn test_dominant_color_extraction() {
    let extractor = PaletteExtractor::new();
    
    // Create a simple image with mostly red pixels (6 red, 2 green)
    let pixels = vec![
        255, 0, 0, 255,   // Red
        255, 0, 0, 255,   // Red
        255, 0, 0, 255,   // Red
        255, 0, 0, 255,   // Red
        255, 0, 0, 255,   // Red
        255, 0, 0, 255,   // Red
        0, 255, 0, 255,   // Green (minority)
        0, 255, 0, 255,   // Green (minority)
    ];
    
    let result = extractor.extract_dominant_color(&pixels);
    assert!(result.is_ok(), "Dominant color extraction should succeed");
    
    let dominant = result.unwrap();
    // Should be close to red - the algorithm may average colors, so we check for reddish
    assert!(dominant.r() > dominant.g(), "Dominant color should be more red than green");
    assert!(dominant.r() > dominant.b(), "Dominant color should be more red than blue");
    assert!(dominant.r() > 150, "Dominant color should be significantly red");
}

#[wasm_bindgen_test]
fn test_color_distance() {
    let red = Color::new(255, 0, 0);
    let blue = Color::new(0, 0, 255);
    let dark_red = Color::new(200, 0, 0);
    
    let distance_red_blue = color_distance_rgb(&red, &blue);
    let distance_red_dark_red = color_distance_rgb(&red, &dark_red);
    
    assert!(distance_red_blue > distance_red_dark_red, 
           "Distance between red and blue should be greater than red and dark red");
}

#[wasm_bindgen_test]
fn test_sort_colors_by_luminance() {
    let colors = vec![
        Color::new(255, 255, 255), // White (brightest)
        Color::new(0, 0, 0),       // Black (darkest)
        Color::new(128, 128, 128), // Gray (middle)
    ];
    
    let sorted = sort_colors_by_luminance(colors);
    
    // Should be sorted from darkest to brightest
    assert!(sorted[0].r() < sorted[1].r(), "First color should be darker");
    assert!(sorted[1].r() < sorted[2].r(), "Second color should be darker than third");
}

#[wasm_bindgen_test]
fn test_remove_similar_colors() {
    let colors = vec![
        Color::new(255, 0, 0),     // Red
        Color::new(250, 5, 5),     // Very similar red
        Color::new(0, 0, 255),     // Blue (very different)
    ];
    
    let filtered = remove_similar_colors(colors, 20.0);
    
    // Should remove one of the similar reds
    assert_eq!(filtered.len(), 2, "Should filter out similar colors");
}

#[wasm_bindgen_test]
fn test_invalid_input_handling() {
    let extractor = PaletteExtractor::new();
    
    // Test with invalid pixel data (not divisible by 4)
    let invalid_pixels = vec![255, 0, 0]; // Only 3 bytes should be RGBA
    let result = extractor.extract_palette_from_pixels(&invalid_pixels, 2);
    assert!(result.is_err(), "Should fail with invalid pixel data");
    
    // Test with k=0
    let valid_pixels = vec![255, 0, 0, 255];
    let result = extractor.extract_palette_from_pixels(&valid_pixels, 0);
    assert!(result.is_err(), "Should fail with k=0");
}