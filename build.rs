use image::{ImageBuffer, Rgba, ImageFormat};

fn main() {
    // Only run on Windows for icon generation
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    if target_os == "windows" {
        // Create a simple icon: circular button with mouse cursor
        let size = 256u32;
        let mut img = ImageBuffer::<Rgba<u8>, Vec<u8>>::new(size, size);
        
        let center_x = size as f32 / 2.0;
        let center_y = size as f32 / 2.0;
        let radius = size as f32 / 2.0 - 10.0;
        
        // Draw circular background with gradient
        for y in 0..size {
            for x in 0..size {
                let dx = x as f32 - center_x;
                let dy = y as f32 - center_y;
                let dist = (dx * dx + dy * dy).sqrt();
                
                if dist <= radius {
                    // Blue gradient
                    let intensity = (1.0 - (dist / radius).min(1.0)) as u8;
                    let r = (30 + intensity / 3) as u8;
                    let g = (80 + intensity) as u8;
                    let b = (180 + intensity / 2) as u8;
                    img.put_pixel(x, y, Rgba([r, g, b, 255]));
                } else {
                    img.put_pixel(x, y, Rgba([0, 0, 0, 0])); // Transparent
                }
            }
        }
        
        // Draw a simple mouse cursor (arrow) in white
        let cursor_x = center_x - 30.0;
        let cursor_y = center_y - 30.0;
        
        // Draw cursor arrow shape
        for i in 0..120 {
            for j in 0..120 {
                let px = (cursor_x + i as f32) as u32;
                let py = (cursor_y + j as f32) as u32;
                
                if px < size && py < size {
                    let di = i as f32;
                    let dj = j as f32;
                    
                    // Simple arrow shape: triangle pointing down-right
                    if (di + dj < 80.0 && di < 60.0 && dj < 60.0) || 
                       (di < 40.0 && dj < 40.0) {
                        img.put_pixel(px, py, Rgba([255, 255, 255, 255]));
                    }
                }
            }
        }
        
        // Draw click indicator (small circle)
        let click_radius = 12.0;
        let click_x = cursor_x + 50.0;
        let click_y = cursor_y + 50.0;
        for dy in -15..=15 {
            for dx in -15..=15 {
                let dist = ((dx * dx + dy * dy) as f32).sqrt();
                if dist <= click_radius {
                    let x = (click_x + dx as f32) as u32;
                    let y = (click_y + dy as f32) as u32;
                    if x < size && y < size {
                        img.put_pixel(x, y, Rgba([255, 200, 0, 255])); // Yellow/orange
                    }
                }
            }
        }
        
        // Save as ICO file
        let ico_path = "icon.ico";
        let mut ico_file = std::fs::File::create(ico_path).unwrap();
        img.write_to(&mut ico_file, ImageFormat::Ico).unwrap();
        
        // Use winres to embed the icon
        let mut res = winres::WindowsResource::new();
        res.set_icon(ico_path);
        res.compile().unwrap();
    }
}
