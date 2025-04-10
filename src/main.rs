use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;

fn main() {
    let dir = "./";
    let old_dir = format!("{}/OLD", dir);
    fs::create_dir_all(&old_dir).unwrap();

    let paths = fs::read_dir(dir).unwrap();
    let mut groups: HashMap<String, Vec<String>> = HashMap::new();

    // Updated regex: captures _A_, _rev.A_, _sub.A_, etc., but only extracts the 'A'
    let version_regex = Regex::new(r"_(?:[a-zA-Z]+\.)?([0-9A-Z])_").unwrap();

    // Step 1: Group files by their base identifier
    for path in paths {
        let path = path.unwrap();
        if path.file_type().unwrap().is_file() {
            let file_name = path.file_name().into_string().unwrap();
            if let Some((group, _)) = extract_group_version(&file_name, &version_regex) {
                groups.entry(group).or_default().push(file_name);
            }
        }
    }

    // Step 2: Keep only the newest file for each group
    for (_, mut files) in groups {
        files.sort_by(|a, b| compare_versions(a, b, &version_regex));

        let newest_file = files.pop().unwrap(); // The last file is the newest

        // Move old files to OLD folder
        for file in files {
            let old_path = format!("{}/{}", dir, file);
            let new_path = format!("{}/{}", old_dir, file);
            fs::rename(&old_path, &new_path).unwrap();
        }

        println!("Keeping: {}", newest_file);
    }
}

// Extract group identifier and version
fn extract_group_version(file_name: &str, regex: &Regex) -> Option<(String, String)> {
    if let Some(captures) = regex.captures(file_name) {
        let version = captures.get(1).unwrap().as_str().to_string();
        let _match_start = captures.get(1).unwrap().start();

        // Go back to find the underscore before the whole match (to slice the group cleanly)
        if let Some(m) = regex.find(file_name) {
            let group = file_name[..m.start()].to_string();
            Some((group, version))
        } else {
            None
        }
    } else {
        None
    }
}

// Compare versions correctly (Numbers > Letters)
fn compare_versions(a: &str, b: &str, regex: &Regex) -> Ordering {
    let (group_a, version_a) = extract_group_version(a, regex).unwrap();
    let (group_b, version_b) = extract_group_version(b, regex).unwrap();

    if group_a != group_b {
        return group_a.cmp(&group_b);
    }

    let a_is_number = version_a.chars().all(char::is_numeric);
    let b_is_number = version_b.chars().all(char::is_numeric);

    match (a_is_number, b_is_number) {
        (true, true) => version_a
            .parse::<i32>()
            .unwrap()
            .cmp(&version_b.parse::<i32>().unwrap()), // Compare numbers
        (true, false) => Ordering::Greater, // Numbers are newer than letters
        (false, true) => Ordering::Less,
        (false, false) => version_a.cmp(&version_b), // Compare letters alphabetically
    }
}
