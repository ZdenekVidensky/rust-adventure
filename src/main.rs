use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

struct Scene {
    id: u8,
    description: String,
    options: Vec<Option>,
    is_final: bool,
}

impl Scene {
    fn from_line(line: String) -> Self {
        let values: Vec<&str> = line.split(';').collect();
        let id = values[0].parse::<u8>().unwrap();
        let description = values[1].to_string();
        let is_final = values[3].parse::<bool>().unwrap();

        let options_values: Vec<&str> = values[2].split(',').collect();
        let mut options: Vec<Option> = Vec::new();

        if options_values[0].contains(':') {
            for option in options_values {
                let vals: Vec<&str> = option.split(':').collect();
                let option_description = vals[0].to_string();
                let option_scene_id = vals[1].parse::<u8>().unwrap();

                options.push(Option::from(option_description, option_scene_id));
            }
        }

        Scene {
            id,
            options,
            description,
            is_final,
        }
    }
}

struct Option {
    description: String,
    scene_id: u8,
}

impl Option {
    fn from(description: String, scene_id: u8) -> Self {
        Option {
            description,
            scene_id,
        }
    }
}

fn main() -> std::io::Result<()> {
    let scenes = load_scenes()?;
    game_loop(scenes);

    Ok(())
}

fn game_loop(scenes: HashMap<u8, Scene>) {
    let mut current_scene = scenes.get(&1).unwrap();

    loop {
        print_scene(current_scene);

        if current_scene.is_final {
            break;
        }

        let selected_option = get_user_option(current_scene);
        let option = current_scene.options.get(selected_option as usize).unwrap();

        current_scene = scenes.get(&option.scene_id).unwrap();
    }
}

fn print_scene(scene: &Scene) {
    println!("{}", scene.description);
    println!("------------------------");

    for (i, option) in scene.options.iter().enumerate() {
        println!("{}: {}", i + 1, option.description);
    }
}

fn get_user_option(scene: &Scene) -> u8 {
    loop {
        let mut selected_option = String::new();
        io::stdin().read_line(&mut selected_option).unwrap();

        let option_result = selected_option.trim().parse::<u8>();
        let options_len: u8 = scene.options.len() as u8;

        let final_option = match option_result {
            Ok(act) => act,
            Err(_) => continue,
        };

        if final_option < 1 || final_option > options_len {
            println!("Můžeš vybrat jen možnosti od 1 do {}", options_len);
            continue;
        }

        return final_option - 1;
    }
}

fn load_scenes() -> std::io::Result<HashMap<u8, Scene>> {
    let scene_file = File::open("resources/story.csv")?;
    let scene_reader = BufReader::new(scene_file);

    let mut scenes: HashMap<u8, Scene> = HashMap::new();

    for line in scene_reader.lines().skip(1) {
        let current_line = line?.trim().to_string();

        let scene = Scene::from_line(current_line);
        scenes.insert(scene.id, scene);
    }

    Ok(scenes)
}
