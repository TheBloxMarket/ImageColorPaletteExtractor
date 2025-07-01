# 🎨 Image Color Palette Extractor

A high-performance WebAssembly (WASM) library for extracting color palettes from images using K-means clustering. Built with Rust and based on the excellent [kmeans-colors](https://github.com/okaneco/kmeans-colors) crate.

## ✨ Features

- **Fast K-means clustering** - Efficient color palette extraction using the kmeans-colors algorithm
- **WebAssembly powered** - High performance in the browser with automatic initialization
- **Remote URL support** - Load images from URLs with automatic CORS handling
- **File upload support** - Drag & drop or file picker for local images
- **Flexible API** - Extract palettes, dominant colors, and perform color analysis
- **Color utilities** - Sort by luminance, remove similar colors, calculate distances
- **TypeScript support** - Full type definitions included
- **Comprehensive tests** - Well-tested with WASM browser tests

## 🚀 Quick Start

### Installation

```bash
# Build the WASM library
wasm-pack build --target web

# The generated files will be in the pkg/ directory
```

### Basic Usage

```html
<!DOCTYPE html>
<html lang="en-US">
<head>
    <title>Color Palette Extractor</title>
</head>
<body>
    <script type="module">
        import { PaletteExtractor } from './pkg/image_color_palette_extractor.js';

        async function extractPalette() {
            // Create extractor instance (no init needed)
            const extractor = new PaletteExtractor();
            
            // Configure (optional)
            extractor.set_max_iterations(20);
            extractor.set_convergence(5.0);
            extractor.set_verbose(true);
            
            // Get image data from canvas
            const canvas = document.getElementById('myCanvas');
            const ctx = canvas.getContext('2d');
            const imageData = ctx.getImageData(0, 0, canvas.width, canvas.height);
            
            // Extract palette (5 colors)
            const palette = extractor.extract_palette_from_pixels(imageData.data, 5);
            
            // Get results
            const colors = palette.colors();
            const percentages = palette.percentages();
            
            // Display results
            colors.forEach((color, index) => {
                console.log(`Color ${index + 1}: ${color.to_hex()} (${percentages[index].toFixed(1)}%)`);
            });
            
            // Extract dominant color
            const dominant = extractor.extract_dominant_color(imageData.data);
            console.log(`Dominant color: ${dominant.to_hex()}`);
        }
        
        extractPalette();
    </script>
</body>
</html>
```

## 📖 API Reference

### PaletteExtractor

The main class for extracting color palettes.

#### Constructor

```javascript
const extractor = new PaletteExtractor();
```

#### Configuration Methods

```javascript
extractor.set_max_iterations(20);     // Set maximum iterations (default: 20)
extractor.set_convergence(5.0);       // Set convergence threshold (default: 5.0)
extractor.set_verbose(true);          // Enable/disable verbose logging (default: false)
```

#### Extraction Methods

```javascript
// Extract palette from RGBA pixel data
const palette = extractor.extract_palette_from_pixels(pixelData, k);

// Extract palette from image data with dimensions
const palette = extractor.extract_palette_from_image_data(imageData, width, height, k);

// Extract dominant color
const dominantColor = extractor.extract_dominant_color(pixelData);
```

### Color

Represents a color with RGB values.

#### Constructor

```javascript
const color = new Color(255, 128, 64);
```

#### Properties

```javascript
color.r()  // Red component (0-255)
color.g()  // Green component (0-255)
color.b()  // Blue component (0-255)
```

#### Methods

```javascript
color.to_hex()        // Returns "#ff8040"
color.to_rgb_string() // Returns "rgb(255, 128, 64)"
```

### PaletteResult

Contains the extracted palette information.

#### Methods

```javascript
palette.colors()       // Array of Color objects
palette.percentages()  // Array of percentages for each color
palette.get_color(i)   // Get color at index i
palette.get_percentage(i) // Get percentage at index i
palette.length()       // Number of colors in palette
```

### Utility Functions

```javascript
// Sort colors by luminance (dark to light)
const sortedColors = sort_colors_by_luminance(colors);

// Calculate RGB distance between two colors
const distance = color_distance_rgb(color1, color2);

// Remove similar colors based on threshold
const filteredColors = remove_similar_colors(colors, threshold);
```

## 🎯 Examples

### Complete Image Upload Example

See `example.html` for a complete working example with:
- Image upload (drag & drop or file picker)
- Remote image URL loading with CORS proxy fallback
- Canvas-based image processing
- Palette extraction and display
- Color copying functionality
- Error handling and loading states

### Remote Image URL Support

The library supports loading images from remote URLs with automatic CORS handling:

```javascript
// The example.html includes robust URL loading with multiple CORS proxy fallbacks
const corsProxies = [
    `https://cors-anywhere.herokuapp.com/`,
    `https://api.allorigins.win/raw?url=`,
    `https://corsproxy.io/?`,
    `https://cors.bridged.cc/`
];

// Try direct loading first, then fallback to proxies if CORS blocks the request
```

Supported URL examples:
- `https://picsum.photos/400/300` - Random images
- `https://images.unsplash.com/photo-1506905925346-21bda4d32df4` - Unsplash images  
- `https://httpbin.org/image/jpeg` - Test images

### Processing Different Image Sources

```javascript
// From file input
const fileInput = document.getElementById('fileInput');
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
        // Process palette...
    };
    img.src = URL.createObjectURL(file);
});

// From existing canvas
const canvas = document.getElementById('myCanvas');
const ctx = canvas.getContext('2d');
const imageData = ctx.getImageData(0, 0, canvas.width, canvas.height);
const palette = extractor.extract_palette_from_pixels(imageData.data, 5);

// From webcam
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
            // Process palette...
        });
    });
```

## 🌐 CORS Handling

The library includes robust Cross-Origin Resource Sharing (CORS) handling for remote images:

### Automatic Proxy Fallback

When loading images from remote URLs, the library automatically handles CORS restrictions:

1. **Direct Loading**: First attempts to load the image directly with `crossOrigin = 'anonymous'`
2. **Proxy Fallback**: If CORS blocks the request, automatically tries multiple proxy services:
   - `cors-anywhere.herokuapp.com` (primary)
   - `api.allorigins.win` (secondary) 
   - `corsproxy.io` (tertiary)
   - `cors.bridged.cc` (fallback)
3. **Error Handling**: Provides clear feedback if all methods fail

### Usage

```javascript
// Just provide any image URL - CORS is handled automatically
const imageUrl = "https://example.com/image.jpg";
loadImageFromUrl(imageUrl); // Handles CORS internally
```

This makes it easy to work with images from any source without worrying about CORS configuration.

## 🔧 Development

### Building

```bash
# Install dependencies
cargo build

# Build for WASM (creates pkg/ directory)
wasm-pack build --target web

# The generated files will be in pkg/:
# - image_color_palette_extractor.js (main module)
# - image_color_palette_extractor_bg.wasm (WASM binary)
# - image_color_palette_extractor.d.ts (TypeScript definitions)
```

### Testing the Demo

```bash
# After building, open either demo file in your browser
# example.html - Full-featured demo with file upload and URL support
# test.html - Quick test interface

# Or serve via a local server (required for WASM modules)
python -m http.server 8000
# Then open http://localhost:8000/example.html
```

### Testing

```bash
# Run Rust tests
cargo test

# Run WASM tests in browser
wasm-pack test --headless --firefox
```

### Project Structure

```
├── src/
│   ├── lib.rs      # Main WASM library code
│   └── utils.rs    # Utility functions (panic hooks, etc.)
├── tests/
│   └── web.rs      # WASM browser tests
├── pkg/            # Generated WASM files (after build)
├── example.html    # Complete demo with file upload and URL support
├── test.html       # Quick test interface
└── README.md       # This file
```

## 🎨 How K-means Clustering Works

K-means clustering groups pixels with similar colors together:

1. **Initialization**: Start with K random color centroids
2. **Assignment**: Assign each pixel to the nearest centroid
3. **Update**: Recalculate centroids as the average of assigned pixels
4. **Repeat**: Continue until convergence or max iterations reached

The algorithm finds the K most representative colors in the image based on color similarity in RGB space.

## 🌟 Features & Benefits

- **High Performance**: Written in Rust, compiled to WebAssembly
- **Easy Integration**: No initialization required - just import and use
- **CORS Friendly**: Automatic proxy fallback for remote images
- **Accurate Results**: Uses proven K-means++ initialization
- **Flexible**: Configurable iterations, convergence, and color count
- **Browser Native**: No external dependencies, runs entirely in the browser
- **Type Safe**: Full TypeScript definitions included
- **Well Tested**: Comprehensive test suite
- **User Friendly**: Complete demo with drag & drop and URL support

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 🙏 Acknowledgments

- Built on top of the excellent [kmeans-colors](https://github.com/okaneco/kmeans-colors) crate
- Inspired by various color palette extraction tools and libraries
