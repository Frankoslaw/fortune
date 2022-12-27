use std::{process::Command, fs};
use rand::Rng;

fn main() {
    let result = Command::new("fortune")
        .arg("-f")
        .output()
        .expect("fortune command failed to start");

    let binding = String::from_utf8_lossy(&result.stderr[8..]);
    let fortune_path = binding
        .split('\n')
        .next()
        .unwrap();

    let mut file_list: Vec<String> = vec![];
    for path in fs::read_dir(fortune_path).unwrap() {
        let path = path.unwrap().path();
        if let Some(extension) = path.extension(){
            if extension == "u8" || extension == "dat"{
                continue;
            };
        }
        
        file_list.push(path.display().to_string());
    }

    let mut rng = rand::thread_rng();

    let fortune_file_path = &file_list[rng.gen_range(0..file_list.len())];

    let fortune_file_content = fs::read_to_string(fortune_file_path).expect("Should have been able to read the file");

    let fortunes: Vec<&str> = fortune_file_content.split('%').collect();
    let fortune = &fortunes[rng.gen_range(0..fortunes.len())];

    println!("{}", fortune)
}