use std::collections::HashMap;
use std::fs;

fn main() {
    let dir = "./";
    let old_dir = format!("{}/OLD", dir);
    fs::create_dir_all(&old_dir).unwrap();

    let paths = fs::read_dir(dir).unwrap();
    let mut groups: HashMap<String, Vec<String>> = HashMap::new();

    for path in paths {
        let path = path.unwrap();
        if path.file_type().unwrap().is_file() {
            let file_name = path.file_name().into_string().unwrap();
            if let Some((group, version)) = extract_group_version(&file_name) {
                groups.entry(group).or_insert_with(Vec::new).push(version);
            }
        }
    }

    for (group, mut versions) in groups {
        versions.sort_by(|a, b| compare_versions(a, b));

        for version in versions.iter().take(versions.len() - 1) {
            let old_file = format!(
                "{}/{}_{}_Datasheet for Onshore Pipeline Valve.xls",
                dir, group, version
            );
            let new_path = format!(
                "{}/{}_{}_Datasheet for Onshore Pipeline Valve.xls",
                old_dir, group, version
            );
            fs::rename(&old_file, &new_path).unwrap();
        }
    }
}

fn extract_group_version(file_name: &str) -> Option<(String, String)> {
    let parts: Vec<&str> = file_name.split('_').collect();
    if parts.len() < 3 {
        return None;
    }
    let group = parts[0].to_string();
    let version = parts[1].to_string();
    Some((group, version))
}

fn compare_versions(a: &str, b: &str) -> std::cmp::Ordering {
    let a_is_number = a.chars().all(char::is_numeric);
    let b_is_number = b.chars().all(char::is_numeric);

    match (a_is_number, b_is_number) {
        (true, true) => a.parse::<i32>().unwrap().cmp(&b.parse::<i32>().unwrap()),
        (true, false) => std::cmp::Ordering::Greater,
        (false, true) => std::cmp::Ordering::Less,
        (false, false) => a.cmp(b),
    }
}
