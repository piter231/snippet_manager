use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use std::process;

use crossterm::{execute, terminal, ExecutableCommand};
use crossterm::style::{Color, Print, ResetColor, SetForegroundColor};
use crossterm::terminal::{Clear, ClearType};

const DATA_FILE: &str = "snippets.json";
const HISTORY_FILE: &str = "history.log";

#[derive(Serialize, Deserialize, Debug,Clone)]
struct Snippet {
    title: String,
    code: String,
    tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct ActionHistory {
    actions: Vec<String>,
}

fn main() {
    let mut snippets = load_snippets();
    let mut history = load_history();

    println!("🦀 Welcome to the Advanced Rust Snippet Manager! 🦀");

    loop {
        display_main_menu();
        let choice = get_user_input("Choose an option: ");

        match choice.trim() {
            "1" => add_snippet(&mut snippets, &mut history),
            "2" => list_snippets(&snippets),
            "3" => search_snippets(&snippets),
            "4" => edit_snippet(&mut snippets, &mut history),
            "5" => delete_snippet(&mut snippets, &mut history),
            "6" => list_snippets_by_tag(&snippets),
            "7" => export_snippets(&snippets),
            "8" => import_snippets(&mut snippets, &mut history),
            "9" => view_history(&history),
            "10" => {
                save_snippets(&snippets);
                save_history(&history);
                println!("Goodbye! 👋");
                process::exit(0);
            }
            _ => println!("⚠️ Invalid choice. Please try again."),
        }
    }
}

// === Core Functions ===

fn display_main_menu() {
    clear_screen();
    println!("\n=== Main Menu ===");
    println!("1) Add a snippet");
    println!("2) List all snippets");
    println!("3) Search snippets");
    println!("4) Edit a snippet");
    println!("5) Delete a snippet");
    println!("6) List snippets by tag");
    println!("7) Export snippets to file");
    println!("8) Import snippets from file");
    println!("9) View action history");
    println!("10) Exit");
    println!("=================");
}

fn load_snippets() -> HashMap<String, Snippet> {
    if Path::new(DATA_FILE).exists() {
        let data = fs::read_to_string(DATA_FILE).unwrap_or_else(|_| "{}".to_string());
        serde_json::from_str(&data).unwrap_or_default()
    } else {
        HashMap::new()
    }
}

fn save_snippets(snippets: &HashMap<String, Snippet>) {
    let data = serde_json::to_string_pretty(&snippets).unwrap();
    let mut file = File::create(DATA_FILE).unwrap();
    file.write_all(data.as_bytes()).unwrap();
    println!("✅ Snippets saved successfully.");
}

fn add_snippet(snippets: &mut HashMap<String, Snippet>, history: &mut ActionHistory) {
    let title = get_user_input("Enter a title for the snippet: ");
    if snippets.contains_key(&title) {
        println!("⚠️ A snippet with this title already exists.");
        return;
    }

    let code = get_user_input("Enter the Rust code for the snippet: ");
    let tags_input = get_user_input("Enter tags (comma-separated): ");
    let tags: Vec<String> = tags_input.split(',').map(|s| s.trim().to_string()).collect();

    let snippet = Snippet {
        title: title.clone(),
        code: code.trim().to_string(),
        tags,
    };

    snippets.insert(title.clone(), snippet);
    history.actions.push(format!("Added snippet: {}", title));
    println!("✅ Snippet added successfully!");
}

fn list_snippets(snippets: &HashMap<String, Snippet>) {
    if snippets.is_empty() {
        println!("⚠️ No snippets available.");
        return;
    }

    println!("\n=== Available Snippets ===");
    for (title, snippet) in snippets {
        println!("- {} (Tags: {})", title, snippet.tags.join(", "));
    }
    println!("==========================");
}

fn search_snippets(snippets: &HashMap<String, Snippet>) {
    let query = get_user_input("Enter a keyword to search for: ").to_lowercase();

    let results: Vec<_> = snippets
        .iter()
        .filter(|(_, snippet)| {
            snippet.title.to_lowercase().contains(&query)
                || snippet.code.to_lowercase().contains(&query)
                || snippet
                    .tags
                    .iter()
                    .any(|tag| tag.to_lowercase().contains(&query))
        })
        .collect();

    if results.is_empty() {
        println!("⚠️ No matching snippets found.");
    } else {
        println!("\n=== Search Results ===");
        for (title, snippet) in results {
            display_snippet(title, snippet);
        }
        println!("========================");
    }
}

fn edit_snippet(snippets: &mut HashMap<String, Snippet>, history: &mut ActionHistory) {
    let title = get_user_input("Enter the title of the snippet to edit: ");
    if let Some(snippet) = snippets.get_mut(&title) {
        println!("Editing snippet: {}", snippet.title);

        let new_code = get_user_input("Enter the new code (leave empty to keep current): ");
        if !new_code.trim().is_empty() {
            snippet.code = new_code.trim().to_string();
        }

        let new_tags = get_user_input("Enter new tags (comma-separated, leave empty to keep current): ");
        if !new_tags.trim().is_empty() {
            snippet.tags = new_tags
                .split(',')
                .map(|s| s.trim().to_string())
                .collect();
        }

        history.actions.push(format!("Edited snippet: {}", title));
        println!("✅ Snippet updated successfully!");
    } else {
        println!("⚠️ Snippet not found.");
    }
}

fn delete_snippet(snippets: &mut HashMap<String, Snippet>, history: &mut ActionHistory) {
    let title = get_user_input("Enter the title of the snippet to delete: ");
    if snippets.remove(&title).is_some() {
        history.actions.push(format!("Deleted snippet: {}", title));
        println!("✅ Snippet deleted successfully.");
    } else {
        println!("⚠️ Snippet not found.");
    }
}

fn list_snippets_by_tag(snippets: &HashMap<String, Snippet>) {
    let tag = get_user_input("Enter a tag to filter by: ").to_lowercase();

    let results: Vec<_> = snippets
        .iter()
        .filter(|(_, snippet)| snippet.tags.iter().any(|t| t.to_lowercase() == tag))
        .collect();

    if results.is_empty() {
        println!("⚠️ No snippets found with this tag.");
    } else {
        println!("\n=== Snippets with tag: {} ===", tag);
        for (title, snippet) in results {
            display_snippet(title, snippet);
        }
        println!("================================");
    }
}

fn export_snippets(snippets: &HashMap<String, Snippet>) {
    let file_name = get_user_input("Enter the file name to export snippets: ");
    let data = serde_json::to_string_pretty(&snippets).unwrap();
    let mut file = File::create(file_name).unwrap();
    file.write_all(data.as_bytes()).unwrap();
    println!("✅ Snippets exported successfully.");
}

fn import_snippets(snippets: &mut HashMap<String, Snippet>, history: &mut ActionHistory) {
    let file_name = get_user_input("Enter the file name to import snippets: ");
    let data = fs::read_to_string(&file_name).unwrap_or_else(|_| "{}".to_string());
    let imported: HashMap<String, Snippet> = serde_json::from_str(&data).unwrap_or_default();
    snippets.extend(imported.clone());
    history.actions.push(format!("Imported snippets from file: {}", file_name));
    println!("✅ Snippets imported successfully.");
}

fn view_history(history: &ActionHistory) {
    if history.actions.is_empty() {
        println!("⚠️ No actions recorded.");
        return;
    }

    println!("\n=== Action History ===");
    for action in &history.actions {
        println!("- {}", action);
    }
    println!("=======================");
}

fn display_snippet(title: &str, snippet: &Snippet) {
    println!("Title: {}\nCode:\n{}\nTags: {}\n", title, snippet.code, snippet.tags.join(", "));
}

fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn clear_screen() {
    let _ = terminal::disable_raw_mode();
    let mut stdout = io::stdout();
    execute!(stdout, Clear(ClearType::All)).unwrap();
}

// === History Functions ===

fn load_history() -> ActionHistory {
    if Path::new(HISTORY_FILE).exists() {
        let data = fs::read_to_string(HISTORY_FILE).unwrap_or_else(|_| "{}".to_string());
        serde_json::from_str(&data).unwrap_or_default()
    } else {
        ActionHistory { actions: Vec::new() }
    }
}

fn save_history(history: &ActionHistory) {
    let data = serde_json::to_string_pretty(&history).unwrap();
    let mut file = File::create(HISTORY_FILE).unwrap();
    file.write_all(data.as_bytes()).unwrap();
}
