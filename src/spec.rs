use std::fs::File;
use std::io::{self, BufRead};

pub(crate) fn read_requirements_file(filename: &str) -> io::Result<Vec<String>> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    let mut lines = Vec::new();
    let mut current_line = String::new();

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();

        if line.starts_with("-e") || line.starts_with("#") {
            continue;
        }

        if line.ends_with('\\') {
            current_line.push_str(&line[..line.len() - 1]);
        } else {
            current_line.push_str(line);
            let cleaned_line = current_line.split(';').next().unwrap().trim().to_string();
            let cleaned_line = cleaned_line.split_whitespace().next().unwrap().to_string();
            lines.push(cleaned_line);
            current_line.clear();
        }
    }

    if !current_line.is_empty() {
        let cleaned_line = current_line.split(';').next().unwrap().trim().to_string();
        let cleaned_line = cleaned_line.split_whitespace().next().unwrap().to_string();
        lines.push(cleaned_line);
    }

    Ok(lines)
}

// pub(crate) fn read_requirements_file(filename: &str) -> io::Result<Vec<String>> {
//     let file = File::open(filename)?;
//     let reader = io::BufReader::new(file);
//     let mut lines = Vec::new();
//     for line in reader.lines() {
//         let line = line?;
//         let line = line.trim();
//         if !line.starts_with("-e") && !line.starts_with("#") && !line.is_empty() {
//             lines.push(line.to_string());
//         }
//     }
//     Ok(lines)
// }
