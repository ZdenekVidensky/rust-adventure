use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::collections::HashMap;

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

        Scene{id, options, description, is_final}
    }
}

struct Option {
    description: String,
    scene_id: u8
}

impl Option {
    fn from(description: String, scene_id: u8) -> Self{
        Option {
            description,
            scene_id
        }
    }
}

fn main() -> std::io::Result<()> {

    let scenes = load_scenes()?;

    // todo: start scene
    // todo: loop

    Ok(())
}

fn load_scenes() -> std::io::Result<HashMap<u8, Scene>> {
    let scene_file = File::open("resources/story.csv")?;
    let scene_reader = BufReader::new(scene_file);

    let mut scenes: HashMap<u8, Scene> = HashMap::new();

    for line in scene_reader.lines().skip(1){
        let current_line = line?.trim().to_string();

        let scene = Scene::from_line(current_line);
        scenes.insert(scene.id, scene);
    }

    Ok(scenes)
}

