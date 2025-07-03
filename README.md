# Image Color Palette Extractor

A Rust-based WebAssembly library for extracting color palettes from images using K-means clustering. Wraps the [kmeans-colors](https://github.com/okaneco/kmeans-colors) crate with a browser-friendly API.

## Features

- Efficient K-means clustering algorithm for color extraction
- WASM compilation with web target for optimal browser performance
- Native support for browser File API and canvas imageData processing
- Comprehensive color analysis API (palette extraction, dominant colors)
- Utility functions for RGB manipulation and comparison
- Complete TypeScript definitions
- Cross-browser compatibility with Wasm tests

## Quick Start

### Installation

```bash
# Build the WASM library in release mode
wasm-pack build --target web --release
```

### Basic Usage

```html
<!DOCTYPE html>
<html>
<head>
    <title>Color Palette Example</title>
</head>
<body>
    <script type="module">
        import { PaletteExtractor } from './pkg/image_color_palette_extractor.js';

        async function extractPalette() {
            const extractor = new PaletteExtractor();
            
            // Configure K-means parameters (optional)
            extractor.set_max_iterations(20); 
            extractor.set_convergence(5.0);
            
            // Get image data from canvas
            const canvas = document.getElementById('myCanvas');
            const ctx = canvas.getContext('2d');
            const imageData = ctx.getImageData(0, 0, canvas.width, canvas.height);
            
            // Extract palette with 5 colors
            const palette = extractor.extract_palette_from_pixels(imageData.data, 5);
            
            // Process results
            const colors = palette.colors();
            const percentages = palette.percentages();
            
            colors.forEach((color, index) => {
                console.log(`Color ${index + 1}: ${color.to_hex()} (${percentages[index].toFixed(1)}%)`);
            });
            
            // Get dominant color (alternative API)
            const dominant = extractor.extract_dominant_color(imageData.data);
            console.log(`Dominant color: ${dominant.to_hex()}`);
        }
        
        extractPalette();
    </script>
</body>
</html>
```

## API Reference

### PaletteExtractor

Main extraction class that handles the K-means algorithm implementation.

```javascript
// Constructor (no initialization needed, zero overhead)
const extractor = new PaletteExtractor();

// K-means algorithm configuration
extractor.set_max_iterations(20);     // Maximum iterations (default: 20)
extractor.set_convergence(5.0);       // Convergence distance threshold (default: 5.0)
extractor.set_verbose(true);          // Enable debug logging (default: false)

// Extraction methods
const palette = extractor.extract_palette_from_pixels(pixelData, k);  // From raw RGBA array
const palette = extractor.extract_palette_from_image_data(imageData, width, height, k);  // From ImageData
const dominantColor = extractor.extract_dominant_color(pixelData);  // Most prominent color
```

### Color

RGB color value representation with utility methods.

```javascript
// Constructor
const color = new Color(255, 128, 64);  // R, G, B values (0-255)

// Component access
const r = color.r();  // Red channel
const g = color.g();  // Green channel
const b = color.b();  // Blue channel

// Format conversion
const hex = color.to_hex();        // "#ff8040"
const rgb = color.to_rgb_string(); // "rgb(255, 128, 64)"
```

### PaletteResult

Result container with colors and their statistical distribution.

```javascript
// Data access
const allColors = palette.colors();        // Array<Color>
const allPercentages = palette.percentages();   // Array<number> (percentage of pixels)
const color = palette.get_color(i);        // Get Color at index i
const percentage = palette.get_percentage(i);  // Get percentage at index i
const count = palette.length();            // Number of colors in palette
```

### Utility Functions

```javascript
// Luminance-based color sorting (dark to light)
const sortedColors = sort_colors_by_luminance(colors);

// Euclidean RGB distance calculation
const distance = color_distance_rgb(color1, color2);

// Similar color filtering (removes colors below threshold distance)
const filteredColors = remove_similar_colors(colors, threshold);
```

## Examples

### Demo Implementation

See `example.html` for a reference implementation demonstrating:
- File input handling (drag & drop + file picker)
- Remote URL loading with CORS fallbacks
- Canvas-based image processing
- Configurable palette extraction
- DOM result visualization

### CORS Handling

The library implements a cascading proxy approach for CORS:

```javascript
// Available CORS proxies (ordered by preference)
const corsProxies = [
    `https://cors-anywhere.herokuapp.com/`,
    `https://api.allorigins.win/raw?url=`,
    `https://corsproxy.io/?`,
    `https://cors.bridged.cc/`
];

// Implementation attempts direct loading then tries each proxy sequentially
```

### Next.js

To use this library in Next.js, you must use the `import` import.

```typescript
"use client";
import { PaletteExtractor } from "@tbmwebui/image_color_extractor";

/**
 * Initializes the palette extractor WASM module for client-side color extraction
 *
 * @returns A Promise that resolves to a new instance of PaletteExtractor
 * @throws Error if called on the server side where window is undefined
 */
export async function initializePaletteExtractor(): Promise<PaletteExtractor> {
    // Ensure this runs only on the client side
    if (typeof window === "undefined") {
        throw new Error("WASM module can only be loaded on the client side.");
    }

    // Dynamically import the WASM module
    const { default: initWasm } = await import("@tbmwebui/image_color_extractor/");

    // Initialize the WASM module (no need to specify a path if the package handles it internally)
    await initWasm();

    // Return a new instance of PaletteExtractor
    return new PaletteExtractor();
}

/**
 * Extracts dominant colors from a canvas element using the WASM-based color extractor
 *
 * @param canvas - HTML Canvas element containing the image data
 * @param numberOfColors - Number of dominant colors to extract (default: 2)
 * @param verbose - Whether to log detailed extraction information to console (default: false)
 * @returns A Promise that resolves to an array of hex color strings
 * @throws Error if unable to get canvas context or image data
 */
export async function GetColorsFromCanvas(
    canvas: HTMLCanvasElement,
    numberOfColors = 2,
    verbose = false
) {
    const extractor = await initializePaletteExtractor();
    extractor.set_verbose(verbose);

    // get image data from canvas
    const ctx = canvas.getContext("2d");
    if (!ctx) {
        throw new Error("Failed to get canvas context");
    }
    const imgData = ctx.getImageData(0, 0, canvas.width, canvas.height);
    if (!imgData || !imgData.data) {
        throw new Error("Failed to get image data from canvas");
    }

    // extract colors
    const palette = extractor.extract_palette_from_pixels(new Uint8Array(imgData.data), numberOfColors);

    // Get results
    const colors = palette.colors;
    const percentages = palette.percentages;

    // display results
    if (verbose) {
        console.log("Extracted colors:", colors);
        console.log("Color percentages:", percentages);
    }

    // Convert colors to hex format
    const hexColors = colors.map((color) => color.to_hex());
    if (verbose) {
        console.log("Hex colors:", hexColors);
    }
    return hexColors;
}


```

### Input Sources

```javascript
// From file input
fileInput.addEventListener('change', async (e) => {
    const file = e.target.files[0];
    const img = new Image();
    img.onload = () => {
        const canvas = document.createElement('canvas');
        const ctx = canvas.getContext('2d');
        canvas.width = img.width;
        canvas.height = img.height;
        ctx.drawImage(img, 0, 0);
        
        const imageData = ctx.getImageData(0, 0, img.width, img.height);
        const palette = extractor.extract_palette_from_pixels(imageData.data, 8);
        // Process palette
    };
    img.src = URL.createObjectURL(file);
});

// From canvas element
const canvas = document.getElementById('myCanvas');
const ctx = canvas.getContext('2d');
const imageData = ctx.getImageData(0, 0, canvas.width, canvas.height);
const palette = extractor.extract_palette_from_pixels(imageData.data, 5);

// From webcam stream
navigator.mediaDevices.getUserMedia({ video: true })
    .then(stream => {
        const video = document.createElement('video');
        video.srcObject = stream;
        video.play();
        
        video.addEventListener('loadedmetadata', () => {
            const canvas = document.createElement('canvas');
            const ctx = canvas.getContext('2d');
            canvas.width = video.videoWidth;
            canvas.height = video.videoHeight;
            ctx.drawImage(video, 0, 0);
            
            const imageData = ctx.getImageData(0, 0, canvas.width, canvas.height);
            const palette = extractor.extract_palette_from_pixels(imageData.data, 6);
            // Process palette
        });
    });
```

## CORS Implementation Details

The library handles CORS using the following strategy:

1. Initial attempt: Direct image fetch with `crossOrigin = 'anonymous'`
2. Fallback chain: Sequential proxy attempts on failure
3. Error propagation if all attempts fail

```javascript
// The implementation abstracts away CORS complexity
function loadImage(url) {
    // CORS handling is internal - no special user code needed
    return fetch(url)
        .then(/* processing */);
}
```

## Development

### Build Process

```bash
# Development build
wasm-pack build --target web

# Release build with optimizations
wasm-pack build --target web --release

# Output artifacts in pkg/:
# - image_color_palette_extractor.js (JS interface)
# - image_color_palette_extractor_bg.wasm (WASM binary)
# - image_color_palette_extractor.d.ts (TS definitions)
# - Additional metadata files
```

### Local Testing

```bash
# WASM modules require proper MIME types, use a local server:
python -m http.server 8000
# Then navigate to http://localhost:8000/example.html

# Available demo files:
# - example.html: Full implementation with all features
# - test.html: Minimal test implementation
```

### Running Tests

```bash
# Browser-based WASM tests (headless)
wasm-pack test --headless --firefox

# Multiple browser testing
wasm-pack test --headless --chrome --firefox
```

> [!NOTE]
> Standard `cargo test` is not compatible with this WASM project. Always use `wasm-pack test`.

### Project Structure

```
├── src/
│   ├── lib.rs      # Core library implementation
│   └── utils.rs    # WASM utility functions
├── tests/
│   └── web.rs      # Browser-based test suite
├── pkg/            # Build artifacts (generated)
├── example.html    # Full-featured demo
├── test.html       # Minimal test interface
└── Cargo.toml      # Rust dependencies and config
```

## Implementation Details: K-means Algorithm

The core color extraction utilizes K-means clustering:

1. **Initialization**: K random centroids in RGB space (uses k-means++ seeding)
2. **Assignment**: Each pixel assigned to nearest centroid by Euclidean distance
3. **Update**: Recalculate centroids as mean of all assigned pixels
4. **Iteration**: Repeat steps 2-3 until convergence threshold or max iterations

This implementation operates directly in RGB space and is optimized for performance.

## Technical Advantages

- **Performance**: Native Rust speed with WASM compilation
- **Zero Overhead**: No runtime initialization requirements
- **CORS Handling**: Multi-stage proxy fallback implementation
- **Algorithm Quality**: K-means++ initialization for better clustering
- **Configurability**: Tunable algorithm parameters
- **Browser Compatibility**: Pure WASM/JS with no external dependencies
- **Type Safety**: Comprehensive TypeScript definitions
- **Test Coverage**: Browser-based test suite with headless capabilities

## License

MIT licensed. See [LICENSE](./LICENSE) for details.

## Contributing

Contributions welcome via standard pull request workflow. Please ensure tests pass before submitting.

## Credits

- Based on [kmeans-colors](https://github.com/okaneco/kmeans-colors) Rust crate
- Adapted for WebAssembly browser usage
