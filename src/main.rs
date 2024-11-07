mod pypi;
mod spec;

fn generate_resource_blocks(requirements_file: &str) -> Result<String, Box<dyn std::error::Error>> {
    let lines = spec::read_requirements_file(requirements_file)?;
    // for line in &lines {
    //     println!("{}", line);
    // }

    let mut resource_blocks = String::new();

    for line in lines {
        let parts: Vec<&str> = line.split("==").collect();
        if parts.len() != 2 {
            println!("Skipping invalid line: {}", line);
            continue;
        }
        let pkg_name = parts[0];
        let version = parts[1];

        match pypi::fetch_package_info(pkg_name, version) {
            Ok(pkg_info) => {
                if let Some(selected_url) = pkg_info
                    .urls
                    .iter()
                    .find(|url_info| url_info.url.ends_with(".tar.gz"))
                {
                    let resource_block = format!(
                        "resource \"{}\" do\n  url \"{}\"\n  sha256 \"{}\"\nend\n\n",
                        pkg_name, selected_url.url, selected_url.digests.sha256
                    );
                    resource_blocks.push_str(&resource_block);
                } else {
                    println!(
                        "No .tar.gz distribution found for {}=={}",
                        pkg_name, version
                    );
                }
            }
            Err(_) => {
                println!("Failed to fetch package info for {}=={}", pkg_name, version);
            }
        }
    }

    Ok(resource_blocks)
}

fn main() {
    match generate_resource_blocks("requirements.txt") {
        Ok(resources_text) => println!("{}", resources_text),
        Err(e) => eprintln!("Error: {}", e),
    }
}
