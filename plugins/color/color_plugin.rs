#[path = "../plugins.rs"]
mod plugins;
use plugins::{Plugin, PluginAction, PluginResult, PluginSearchResult, PluginHtmlResult};

use arboard::Clipboard;
use std::fmt;

#[derive(Debug, Clone)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: f32,
}

impl Color {
    fn new(r: u8, g: u8, b: u8, a: f32) -> Self {
        Self { r, g, b, a }
    }

    fn to_hex(&self) -> String {
        if self.a < 1.0 {
            format!("#{:02X}{:02X}{:02X}{:02X}", self.r, self.g, self.b, (self.a * 255.0) as u8)
        } else {
            format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
        }
    }

    fn to_rgb(&self) -> String {
        if self.a < 1.0 {
            format!("rgba({}, {}, {}, {:.2})", self.r, self.g, self.b, self.a)
        } else {
            format!("rgb({}, {}, {})", self.r, self.g, self.b)
        }
    }

    fn to_hsl(&self) -> String {
        let (h, s, l) = rgb_to_hsl(self.r, self.g, self.b);
        if self.a < 1.0 {
            format!("hsla({:.0}, {:.0}%, {:.0}%, {:.2})", h, s * 100.0, l * 100.0, self.a)
        } else {
            format!("hsl({:.0}, {:.0}%, {:.0}%)", h, s * 100.0, l * 100.0)
        }
    }

    fn to_css_rgba(&self) -> String {
        format!("rgba({}, {}, {}, {})", self.r, self.g, self.b, self.a)
    }
}

fn rgb_to_hsl(r: u8, g: u8, b: u8) -> (f32, f32, f32) {
    let r = r as f32 / 255.0;
    let g = g as f32 / 255.0;
    let b = b as f32 / 255.0;

    let max = r.max(g.max(b));
    let min = r.min(g.min(b));
    let delta = max - min;

    let l = (max + min) / 2.0;

    if delta == 0.0 {
        return (0.0, 0.0, l);
    }

    let s = if l < 0.5 {
        delta / (max + min)
    } else {
        delta / (2.0 - max - min)
    };

    let h = if max == r {
        ((g - b) / delta + if g < b { 6.0 } else { 0.0 }) * 60.0
    } else if max == g {
        ((b - r) / delta + 2.0) * 60.0
    } else {
        ((r - g) / delta + 4.0) * 60.0
    };

    (h, s, l)
}

fn parse_color(input: &str) -> Option<Color> {
    let input = input.trim();

    // Try hex format
    if let Some(color) = parse_hex(input) {
        return Some(color);
    }

    // Try rgb/rgba format
    if let Some(color) = parse_rgb(input) {
        return Some(color);
    }

    // Try hsl/hsla format
    if let Some(color) = parse_hsl(input) {
        return Some(color);
    }

    None
}

fn parse_hex(input: &str) -> Option<Color> {
    let hex = input.strip_prefix('#').unwrap_or(input);
    
    match hex.len() {
        3 => {
            // #RGB
            let r = u8::from_str_radix(&hex[0..1].repeat(2), 16).ok()?;
            let g = u8::from_str_radix(&hex[1..2].repeat(2), 16).ok()?;
            let b = u8::from_str_radix(&hex[2..3].repeat(2), 16).ok()?;
            Some(Color::new(r, g, b, 1.0))
        }
        6 => {
            // #RRGGBB
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            Some(Color::new(r, g, b, 1.0))
        }
        8 => {
            // #RRGGBBAA
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            let a = u8::from_str_radix(&hex[6..8], 16).ok()? as f32 / 255.0;
            Some(Color::new(r, g, b, a))
        }
        _ => None,
    }
}

fn parse_rgb(input: &str) -> Option<Color> {
    let input = input.to_lowercase();
    
    if input.starts_with("rgb(") && input.ends_with(')') {
        let content = &input[4..input.len()-1];
        let parts: Vec<&str> = content.split(',').collect();
        if parts.len() == 3 {
            let r = parts[0].trim().parse::<u8>().ok()?;
            let g = parts[1].trim().parse::<u8>().ok()?;
            let b = parts[2].trim().parse::<u8>().ok()?;
            return Some(Color::new(r, g, b, 1.0));
        }
    }
    
    if input.starts_with("rgba(") && input.ends_with(')') {
        let content = &input[5..input.len()-1];
        let parts: Vec<&str> = content.split(',').collect();
        if parts.len() == 4 {
            let r = parts[0].trim().parse::<u8>().ok()?;
            let g = parts[1].trim().parse::<u8>().ok()?;
            let b = parts[2].trim().parse::<u8>().ok()?;
            let a = parts[3].trim().parse::<f32>().ok()?;
            return Some(Color::new(r, g, b, a));
        }
    }
    
    None
}

fn parse_hsl(input: &str) -> Option<Color> {
    let input = input.to_lowercase();
    
    if input.starts_with("hsl(") && input.ends_with(')') {
        let content = &input[4..input.len()-1];
        let parts: Vec<&str> = content.split(',').collect();
        if parts.len() == 3 {
            let h = parts[0].trim().parse::<f32>().ok()?;
            let s = parts[1].trim().trim_end_matches('%').parse::<f32>().ok()? / 100.0;
            let l = parts[2].trim().trim_end_matches('%').parse::<f32>().ok()? / 100.0;
            let (r, g, b) = hsl_to_rgb(h, s, l);
            return Some(Color::new(r, g, b, 1.0));
        }
    }
    
    if input.starts_with("hsla(") && input.ends_with(')') {
        let content = &input[5..input.len()-1];
        let parts: Vec<&str> = content.split(',').collect();
        if parts.len() == 4 {
            let h = parts[0].trim().parse::<f32>().ok()?;
            let s = parts[1].trim().trim_end_matches('%').parse::<f32>().ok()? / 100.0;
            let l = parts[2].trim().trim_end_matches('%').parse::<f32>().ok()? / 100.0;
            let a = parts[3].trim().parse::<f32>().ok()?;
            let (r, g, b) = hsl_to_rgb(h, s, l);
            return Some(Color::new(r, g, b, a));
        }
    }
    
    None
}

fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (u8, u8, u8) {
    let h = h / 360.0;
    
    let hue_to_rgb = |p: f32, q: f32, t: f32| -> f32 {
        let mut t = t;
        if t < 0.0 { t += 1.0; }
        if t > 1.0 { t -= 1.0; }
        if t < 1.0/6.0 { return p + (q - p) * 6.0 * t; }
        if t < 1.0/2.0 { return q; }
        if t < 2.0/3.0 { return p + (q - p) * (2.0/3.0 - t) * 6.0; }
        p
    };

    let (r, g, b) = if s == 0.0 {
        (l, l, l)
    } else {
        let q = if l < 0.5 { l * (1.0 + s) } else { l + s - l * s };
        let p = 2.0 * l - q;
        (
            hue_to_rgb(p, q, h + 1.0/3.0),
            hue_to_rgb(p, q, h),
            hue_to_rgb(p, q, h - 1.0/3.0),
        )
    };

    ((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8)
}

fn generate_color_html(color: &Color, original_input: &str) -> String {
    let mut html = String::new();
    
    // Container with fixed height
    html.push_str("<div style=\"padding: 16px; font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif; height: 200px; overflow: hidden;\">");
    
    // Main layout: color preview on left, formats on right
    html.push_str("<div style=\"display: flex; align-items: stretch; height: 100%; gap: 20px;\">");
    
    // Left side: Color preview
    html.push_str("<div style=\"flex: 0 0 160px; display: flex; flex-direction: column; align-items: center; justify-content: center;\">");
    html.push_str(&format!(
        "<div style=\"width: 120px; height: 120px; border-radius: 12px; background-color: {}; border: 2px solid rgba(255,255,255,0.2); box-shadow: 0 4px 12px rgba(0,0,0,0.3); margin-bottom: 12px;\"></div>",
        color.to_css_rgba()
    ));
    html.push_str(&format!("<div style=\"text-align: center; color: white; font-size: 14px; font-weight: 600;\">{}</div>", original_input));
    html.push_str(&format!("<div style=\"text-align: center; color: rgba(255,255,255,0.6); font-size: 12px; margin-top: 4px;\">RGB: {}, {}, {}</div>", color.r, color.g, color.b));
    if color.a < 1.0 {
        html.push_str(&format!("<div style=\"text-align: center; color: rgba(255,255,255,0.6); font-size: 12px;\">Alpha: {:.2}</div>", color.a));
    }
    html.push_str("</div>");
    
    // Right side: Format list
    html.push_str("<div style=\"flex: 1; display: flex; flex-direction: column; justify-content: center; gap: 8px;\">");
    
    let formats = vec![
        ("HEX", color.to_hex()),
        ("RGB", color.to_rgb()),
        ("HSL", color.to_hsl()),
    ];
    
    for (format_name, format_value) in formats {
        let safe_value = format_value.replace('\"', "&quot;").replace('\'', "&#39;");
        html.push_str(&format!(
            "<div onclick=\"copyColor('{}')\" style=\"background: rgba(255,255,255,0.05); border: 1px solid rgba(255,255,255,0.1); border-radius: 8px; padding: 12px; cursor: pointer; transition: all 0.2s; display: flex; justify-content: space-between; align-items: center;\" onmouseover=\"this.style.background='rgba(255,255,255,0.1)'\" onmouseout=\"this.style.background='rgba(255,255,255,0.05)'\">",
            safe_value
        ));
        html.push_str("<div>");
        html.push_str(&format!("<div style=\"color: rgba(255,255,255,0.6); font-size: 11px; font-weight: 600; margin-bottom: 2px;\">{}</div>", format_name));
        html.push_str(&format!("<div style=\"color: white; font-family: 'Consolas', 'Monaco', monospace; font-size: 13px;\">{}</div>", format_value));
        html.push_str("</div>");
        html.push_str("<div style=\"color: rgba(255,255,255,0.4); font-size: 11px;\">📋</div>");
        html.push_str("</div>");
    }
    
    html.push_str("</div>"); // End right side
    html.push_str("</div>"); // End main layout
    html.push_str("</div>"); // End container
    
    // Script
    html.push_str("<script>");
    html.push_str("function copyColor(colorValue) {");
    html.push_str("  navigator.clipboard.writeText(colorValue).then(() => {");
    html.push_str("    console.log('Copied color:', colorValue);");
    html.push_str("  }).catch(err => {");
    html.push_str("    console.error('Failed to copy:', err);");
    html.push_str("  });");
    html.push_str("}");
    html.push_str("</script>");
    
    html
}

#[no_mangle]
pub extern "Rust" fn get_plugin_info() -> Plugin {
    Plugin {
        id: "color".to_string(),
        name: "Color Converter".to_string(),
        description: "Convert and preview colors in different formats".to_string(),
        prefix: "color".to_string(),
        icon: "🎨".to_string(),
        config: None,
    }
}

#[no_mangle]
pub extern "Rust" fn search_plugin(query: String) -> PluginSearchResult {
    if query.is_empty() {
        let html = generate_example_colors();
        return PluginSearchResult::Html(PluginHtmlResult { html });
    }
    
    if let Some(color) = parse_color(&query) {
        let html = generate_color_html(&color, &query);
        PluginSearchResult::Html(PluginHtmlResult { html })
    } else {
        // Show format examples if no valid color detected
        let html = generate_format_help(&query);
        PluginSearchResult::Html(PluginHtmlResult { html })
    }
}

fn generate_example_colors() -> String {
    let mut html = String::new();
    
    html.push_str("<div style=\"padding: 16px; font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif; height: 200px; overflow: hidden;\">");
    html.push_str("<h3 style=\"margin: 0 0 12px 0; color: white; font-size: 16px;\">🎨 Color Converter</h3>");
    html.push_str("<p style=\"margin: 0 0 16px 0; color: rgba(255,255,255,0.7); font-size: 13px;\">Enter a color in any format to convert and preview it</p>");
    
    html.push_str("<div style=\"display: grid; grid-template-columns: repeat(6, 1fr); gap: 8px; margin-bottom: 16px;\">");
    
    let examples = vec![
        "#FF5733", "#3498DB", "#2ECC71", "#F39C12", "#9B59B6", "#E74C3C"
    ];
    
    for color_code in examples {
        html.push_str(&format!(
            "<div style=\"background: {}; height: 40px; border-radius: 6px; cursor: pointer; display: flex; align-items: center; justify-content: center; color: white; font-weight: 600; font-size: 11px; text-shadow: 1px 1px 2px rgba(0,0,0,0.5); transition: transform 0.2s; border: 1px solid rgba(255,255,255,0.2);\" onmouseover=\"this.style.transform='scale(1.05)'\" onmouseout=\"this.style.transform='scale(1)'\" title=\"Click to try {}\">{}</div>",
            color_code, color_code, color_code
        ));
    }
    
    html.push_str("</div>");
    
    html.push_str("<div style=\"background: rgba(255,255,255,0.05); border-radius: 6px; padding: 12px;\">");
    html.push_str("<div style=\"color: white; font-size: 12px; font-weight: 600; margin-bottom: 8px;\">Supported Formats:</div>");
    html.push_str("<div style=\"display: grid; grid-template-columns: 1fr 1fr; gap: 4px; font-size: 11px; color: rgba(255,255,255,0.8);\">");
    html.push_str("<div>• HEX: #FF5733, #RGB</div>");
    html.push_str("<div>• RGB: rgb(255, 87, 51)</div>");
    html.push_str("<div>• RGBA: rgba(255, 87, 51, 0.8)</div>");
    html.push_str("<div>• HSL: hsl(9, 100%, 60%)</div>");
    html.push_str("</div>");
    html.push_str("</div>");
    
    html.push_str("</div>");
    
    html
}

fn generate_format_help(query: &str) -> String {
    let mut html = String::new();
    
    html.push_str("<div style=\"padding: 16px; font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif; height: 200px; overflow: hidden;\">");
    html.push_str(&format!("<h3 style=\"margin: 0 0 12px 0; color: #ff6b6b; font-size: 16px;\">❌ Invalid Color: \"{}\"</h3>", query));
    
    html.push_str("<div style=\"background: rgba(255,255,255,0.05); border-radius: 8px; padding: 12px; margin-bottom: 12px;\">");
    html.push_str("<div style=\"color: white; font-size: 13px; font-weight: 600; margin-bottom: 8px;\">Valid Format Examples:</div>");
    html.push_str("<div style=\"display: grid; grid-template-columns: 1fr 1fr; gap: 4px; font-family: 'Consolas', 'Monaco', monospace; font-size: 12px;\">");
    html.push_str("<div style=\"color: #4CAF50;\">✓ #FF5733</div>");
    html.push_str("<div style=\"color: #4CAF50;\">✓ #F53</div>");
    html.push_str("<div style=\"color: #4CAF50;\">✓ rgb(255, 87, 51)</div>");
    html.push_str("<div style=\"color: #4CAF50;\">✓ rgba(255, 87, 51, 0.8)</div>");
    html.push_str("<div style=\"color: #4CAF50;\">✓ hsl(9, 100%, 60%)</div>");
    html.push_str("<div style=\"color: #4CAF50;\">✓ hsla(9, 100%, 60%, 0.8)</div>");
    html.push_str("</div>");
    html.push_str("</div>");
    
    html.push_str("<div style=\"background: rgba(255,193,7,0.1); border: 1px solid rgba(255,193,7,0.3); border-radius: 6px; padding: 10px;\">");
    html.push_str("<div style=\"color: rgba(255,255,255,0.8); font-size: 12px;\">💡 Try entering a valid color format to see the conversion and preview</div>");
    html.push_str("</div>");
    
    html.push_str("</div>");
    
    html
}

#[no_mangle]
pub extern "Rust" fn execute_plugin_action(
    result_id: String,
    action_id: String,
) -> Result<String, String> {
    match action_id.as_str() {
        "copy" => {
            match Clipboard::new() {
                Ok(mut clipboard) => {
                    match clipboard.set_text(&result_id) {
                        Ok(_) => Ok("Copied to clipboard".to_string()),
                        Err(e) => Err(format!("Failed to copy: {}", e)),
                    }
                }
                Err(e) => Err(format!("Failed to access clipboard: {}", e)),
            }
        }
        _ => Err("Unknown action".to_string()),
    }
}