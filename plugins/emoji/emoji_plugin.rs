#[path = "../plugins.rs"]
mod plugins;
use plugins::{Plugin, PluginAction, PluginHtmlResult, PluginResult, PluginSearchResult};

use arboard::Clipboard;
use once_cell::sync::Lazy;
use serde::Deserialize;
use std::collections::{HashMap, HashSet};

const NGRAM_SIZE: usize = 2;

#[derive(Deserialize, Debug, Clone)]
struct Emoji {
    name: String,
    slug: String,
}

/// Holds the cached emoji database and the n-gram search index.
struct EmojiIndex {
    /// Stores the full Emoji object, keyed by the emoji character itself (e.g., "😀").
    /// This is used to retrieve full details after a search.
    emojis: HashMap<String, Emoji>,
    /// The search index. Maps an n-gram (e.g., "fac") to a list of
    /// emoji characters that contain that n-gram in their text.
    index: HashMap<String, Vec<String>>,
}

/// A global, lazily-initialized static variable for our emoji database.
/// The code inside `Lazy::new` will run only once, the first time EMOJI_DB is accessed.
static EMOJI_DB: Lazy<EmojiIndex> = Lazy::new(|| {
    println!("[Emoji Plugin] Initializing index...");

    let mut index: HashMap<String, Vec<String>> = HashMap::new();
    let mut emojis = HashMap::new();
    let url = "https://unpkg.com/unicode-emoji-json/data-by-emoji.json";

    match reqwest::blocking::get(url) {
        Ok(resp) => match resp.json::<HashMap<String, Emoji>>() {
            Ok(data) => {
                for (emoji_char, emoji_data) in data {
                    emojis.insert(emoji_char.clone(), emoji_data.clone());
                    let full_text =
                        format!("{} {}", emoji_data.name, emoji_data.slug).to_lowercase();
                    if full_text.chars().count() >= NGRAM_SIZE {
                        let chars: Vec<char> = full_text.chars().collect();
                        for i in 0..=(chars.len() - NGRAM_SIZE) {
                            let ngram: String = chars[i..i + NGRAM_SIZE].iter().collect();
                            index.entry(ngram).or_default().push(emoji_char.clone());
                        }
                    }
                }
            }
            Err(e) => println!("[Emoji Plugin] JSON parse error: {}", e),
        },
        Err(e) => println!("[Emoji Plugin] Network error: {}", e),
    }

    println!("[Emoji Plugin] Index loaded with {} emojis.", emojis.len());
    EmojiIndex { emojis, index }
});

#[no_mangle]
pub extern "Rust" fn get_plugin_info() -> Plugin {
    Plugin {
        id: "emojis".to_string(),
        name: "Emojis".to_string(),
        description: "Search for emojis by name or shortcode".to_string(),
        prefix: "e".to_string(),
        icon: "😀".to_string(),
        config: None,
    }
}

#[no_mangle]
pub extern "Rust" fn search_plugin(query: String) -> PluginSearchResult {
    let query = query.to_lowercase();

    if query.is_empty() {
        // Show popular emojis when no query
        let popular_emojis = [
            "😀", "😂", "😍", "🥰", "😊", "😎", "🤔", "😢", "😡", "🤯", "👍", "👎", "❤️", "🔥",
            "💯", "🎉", "🚀", "⭐", "🌟", "💡",
        ];
        let emoji_vec: Vec<(String, String)> = popular_emojis
            .iter()
            .map(|&e| (e.to_string(), format!("Popular emoji: {}", e)))
            .collect();
        let html = generate_emoji_grid(&emoji_vec);
        return PluginSearchResult::Html(PluginHtmlResult { html });
    }

    // Don't bother searching if the query is too short for our n-gram index.
    if query.chars().count() < NGRAM_SIZE {
        return PluginSearchResult::Results(vec![]);
    }

    let mut query_ngrams = Vec::new();
    let query_chars: Vec<char> = query.chars().collect();
    for i in 0..=(query_chars.len() - NGRAM_SIZE) {
        let ngram: String = query_chars[i..i + NGRAM_SIZE].iter().collect();
        query_ngrams.push(ngram);
    }

    // Step 2: Find candidate emojis by intersecting results from all n-grams.
    // This is more accurate than just using the first n-gram.
    let mut candidate_sets = query_ngrams.into_iter().map(|ngram| {
        EMOJI_DB
            .index
            .get(&ngram)
            .map(|v| v.iter().collect::<HashSet<_>>())
            .unwrap_or_default()
    });

    // Take the first set as the base for our intersection.
    let Some(mut final_candidates_set) = candidate_sets.next() else {
        return PluginSearchResult::Results(vec![]);
    };

    // Intersect with the rest of the sets.
    for next_set in candidate_sets {
        final_candidates_set.retain(|item| next_set.contains(item));
    }

    let mut final_emojis = Vec::new();
    for emoji_char in final_candidates_set {
        if let Some(emoji_data) = EMOJI_DB.emojis.get(emoji_char) {
            let full_text = format!("{} {}", emoji_data.name, emoji_data.slug).to_lowercase();
            if full_text.contains(&query) {
                final_emojis.push((emoji_char.clone(), emoji_data.name.clone()));
            }
        }
    }

    if final_emojis.is_empty() {
        return PluginSearchResult::Results(vec![]);
    }

    let html = generate_emoji_grid(&final_emojis);
    PluginSearchResult::Html(PluginHtmlResult { html })
}

fn generate_emoji_grid(emojis: &[(String, String)]) -> String {
    let mut html = String::new();

    // Add container div
    html.push_str("<div style=\"");
    html.push_str("display: grid; ");
    html.push_str("grid-template-columns: repeat(auto-fill, minmax(60px, 1fr)); ");
    html.push_str("gap: 8px; ");
    html.push_str("padding: 16px; ");
    html.push_str("max-height: 400px; ");
    html.push_str("overflow-y: auto;");
    html.push_str("\">\n");

    // Add emoji items
    for (emoji, name) in emojis.iter().take(100) {
        let safe_emoji = emoji.replace('"', "&quot;").replace('\'', "&#39;");
        let safe_name = name.replace('"', "&quot;").replace('\'', "&#39;");

        html.push_str("<div ");
        html.push_str(&format!("onclick=\"copyEmoji('{}')\" ", safe_emoji));
        html.push_str("style=\"");
        html.push_str("display: flex; ");
        html.push_str("flex-direction: column; ");
        html.push_str("align-items: center; ");
        html.push_str("padding: 8px; ");
        html.push_str("border-radius: 8px; ");
        html.push_str("cursor: pointer; ");
        html.push_str("transition: background-color 0.2s; ");
        html.push_str("background: rgba(255,255,255,0.05);");
        html.push_str("\" ");
        html.push_str("onmouseover=\"this.style.background='rgba(255,255,255,0.1)'\" ");
        html.push_str("onmouseout=\"this.style.background='rgba(255,255,255,0.05)'\" ");
        html.push_str(&format!("title=\"{}\"", safe_name));
        html.push_str(">");
        html.push_str(&format!(
            "<span style=\"font-size: 24px; margin-bottom: 4px;\">{}</span>",
            emoji
        ));
        html.push_str("</div>\n");
    }

    // Close container
    html.push_str("</div>\n");

    // Add script
    html.push_str("<script>");
    html.push_str("function copyEmoji(emoji) { ");
    html.push_str("navigator.clipboard.writeText(emoji).then(() => { ");
    html.push_str("console.log('Copied:', emoji); ");
    html.push_str("}); ");
    html.push_str("}");
    html.push_str("</script>");

    html
}

#[no_mangle]
pub extern "Rust" fn execute_plugin_action(
    result_id: String,
    action_id: String,
) -> Result<String, String> {
    match action_id.as_str() {
        "copy" => match Clipboard::new() {
            Ok(mut clipboard) => match clipboard.set_text(&result_id) {
                Ok(_) => Ok("Copied to clipboard".to_string()),
                Err(e) => Err(format!("Failed to copy: {}", e)),
            },
            Err(e) => Err(format!("Failed to access clipboard: {}", e)),
        },
        _ => Err("Unknown action".to_string()),
    }
}
