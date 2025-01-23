# 🦀 Rust Snippet Manager  

The **Rust Snippet Manager** is a command-line application designed to help developers store, search, manage, and organize code snippets efficiently. This tool supports features like advanced search, editing, tagging, history tracking, and importing/exporting snippets—all wrapped in a user-friendly interface built with Rust.  

## 🌟 Features  

- **Add Snippets**: Store your favorite Rust snippets with tags for easy retrieval.  
- **Search Snippets**: Search by title, code content, or tags using advanced fuzzy matching.  
- **Edit & Delete**: Modify existing snippets or remove outdated ones with ease.  
- **Tag Management**: Organize snippets using tags for quick categorization.  
- **History Tracking**: View recent actions performed on your snippet library.  
- **Export & Import**: Easily back up your snippets to a file or restore them.  
- **TUI Support**: Enjoy a clean, terminal-based interface for better usability.  

## 🛠️ Installation  

### Prerequisites  
- Rust (v1.65 or higher)  
- Cargo (Rust package manager)  

### Clone and Build  
```bash  
# Clone the repository  
git clone https://github.com/piter231/snippet_manager.git

# Navigate to the project directory  
cd snippet_manager

# Build the application  
cargo build --release  

# Run the application  
cargo run  
```  

## 🚀 Usage  

### Menu Options  
1. **Add a Snippet**: Save a new code snippet with tags.  
2. **List All Snippets**: View all saved snippets and their tags.  
3. **Search Snippets**: Search for snippets by keywords in the title, code, or tags.  
4. **Edit a Snippet**: Modify a snippet's code or tags.  
5. **Delete a Snippet**: Remove a snippet permanently.  
6. **List by Tag**: Filter snippets by specific tags.  
7. **Export Snippets**: Save your snippets to a JSON file for backup.  
8. **Import Snippets**: Restore snippets from a JSON file.  
9. **View Action History**: See a log of your recent actions.  
10. **Exit**: Save your data and quit the program.  

### Example: Adding a Snippet  
```plaintext  
Enter a title for the snippet: My First Snippet  
Enter the Rust code for the snippet: println!("Hello, world!");  
Enter tags (comma-separated): beginner, hello_world  
✅ Snippet added successfully!  
```  

### Example: Exporting Snippets  
```plaintext  
Enter the file name to export snippets: snippets_backup.json  
✅ Snippets exported successfully.  
```  

## 📂 File Structure  

- `snippets.json`: Stores all your saved snippets persistently.  
- `history.log`: Tracks recent actions for easy reference.  

## 📚 Dependencies  

The project uses the following crates:  
- **serde**: For serialization and deserialization of JSON data.  
- **crossterm**: For enhanced terminal user interface and styling.  

Add them to your `Cargo.toml`:  
```toml  
[dependencies]  
serde = { version = "1.0", features = ["derive"] }  
crossterm = "0.24"  
```  

## 🛡️ License  

This project is licensed under the [MIT License](LICENSE).  

## 💡 Future Enhancements  

- Syntax highlighting for code snippets using `syntect`.  
- Cloud sync to save snippets remotely.  
- Advanced fuzzy search with custom scoring algorithms.  

## 🤝 Contributing  

Contributions are welcome! Feel free to submit a pull request or file an issue on GitHub.  

## ❤️ Acknowledgments  

Thanks to the Rust community for their incredible resources and guidance!  
