use clap::Parser;
use regex::Regex;
use std::fs;
use std::path::Path;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file to process
    #[arg(required = true)]
    input_file: String,

    /// Translation files containing name mappings
    #[arg(required = true, num_args = 1..)]
    translation_files: Vec<String>,

    /// Output file path (defaults to 'translated_' + input filename)
    #[arg(short, long)]
    output: Option<String>,
}

fn read_translations(translation_file: &Path) -> Vec<(String, String)> {
    let content = fs::read_to_string(translation_file).expect("Failed to read translation file");

    content
        .lines()
        .filter(|line| !line.trim_start().starts_with('#')) // Skip comment-only lines
        .filter_map(|line| {
            // Remove inline comments
            let line = line.split('#').next().unwrap_or(line).trim();
            let parts: Vec<&str> = line.split("->").collect();
            if parts.len() == 2 {
                Some((parts[0].trim().to_string(), parts[1].trim().to_string()))
            } else {
                None
            }
        })
        .collect()
}

fn apply_translations(content: &str, translations: &[(String, String)]) -> String {
    let mut result = content.to_string();

    for (old_name, new_name) in translations {
        let pattern = format!(r"(?-u:\b){}(?-u:\b)", regex::escape(old_name));
        let re = Regex::new(&pattern).unwrap();
        result = re.replace_all(&result, new_name).to_string();
    }

    result
}

fn get_output_path(input_path: &str, output_option: Option<&str>) -> String {
    if let Some(output) = output_option {
        output.to_string()
    } else {
        let input_path = Path::new(input_path);
        let file_name = input_path
            .file_name()
            .expect("Invalid input file path")
            .to_string_lossy();
        let parent = input_path
            .parent()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_default();

        if parent.is_empty() {
            format!("translated_{}", file_name)
        } else {
            format!("{}/translated_{}", parent, file_name)
        }
    }
}

fn main() {
    let args = Args::parse();

    // Read input file
    let content = fs::read_to_string(&args.input_file).expect("Failed to read input file");

    // Collect all translations
    let mut all_translations = Vec::new();
    for translation_file in &args.translation_files {
        let translations = read_translations(Path::new(translation_file));
        all_translations.extend(translations);
    }

    // Apply translations
    let result = apply_translations(&content, &all_translations);

    // Determine output path
    let output_path = get_output_path(&args.input_file, args.output.as_deref());

    // Write results to output file
    fs::write(&output_path, result).expect("Failed to write output file");

    println!("Translation complete. Output written to: {}", output_path);
}
