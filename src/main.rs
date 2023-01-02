extern crate lopdf;
extern crate pdf_extract;

use pdf_extract::*;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path;
use std::path::PathBuf;

mod data;
mod test;

use crate::data::saga_style::Saga_Style;
use crate::data::status::Status;
use data::{category::Category, country::LIST_OF_COUNTRY, saga::Saga};

const WANTED_KEYS: [&str; 9] = [
    "Auteur",
    "Synopsis",
    "Musique",
    "Origine",
    "Genre",
    "Style",
    "Statut",
    "Création",
    "Épisodes",
];

fn main() {
    let paths = fs::read_dir("/saga-files/").unwrap();
    for path in paths {
        
        let file = path.as_ref().unwrap().file_name().into_string();
        let output_kind = env::args().nth(2).unwrap_or_else(|| "txt".to_owned());
        let path_name = path.unwrap().path();
        let filename = path_name.file_name().expect("expected a filename");
        let mut output_file = PathBuf::new();
        output_file.push(filename);
        output_file.set_extension(&output_kind);

        let info = extract_text(path_name);

        if let Ok(val) = info {
            let mut name = "".to_string();
            match file {
                Ok(filename) => {
                    name = filename.to_string();
                }
                Err(_) => {

                }
            }
            cleanup_text(val,  name);
        }
    }
}

fn cleanup_text(mut info: String, file: String) {
    let mut data_array: HashMap<String, String> = HashMap::new();
    let mut link_array: Vec<String> = Vec::new();
    let mut total_of_season = 0;
    let mut list_of_category: Vec<Category> = vec![];
    let mut list_of_status: Vec<Status> = vec![];
    let mut list_of_style: Vec<Saga_Style> = vec![];
    let name = cleanup_values(&file, ".pdf".to_string(), "")
        .trim()
        .to_string();
    // Isolate wanted info by adding a $ at its beginning
    info = find_wanted_key_in_text(info);
    info = cleanup_values(&info, "\n\n".to_string(), "$");
    info = cleanup_values(&info, "\n".to_string(), " ");

    //get vec of data isolated using a dollar sign
    let mut data = split_on_sign(&info, String::from("$"));
    let synopsis = find_synopsis(&data);

    for element in data {
        // If I found an URL
        match element.find("http") {
            None => {
                // If there's : to split
                let index = element.find(':');
                //element.split(":");
                match index {
                    None => {},
                    Some(split_index) => {
                        let (split1, split2) = element.split_at(split_index);
                        let key = cleanup_values(split1, " ".to_string(), "");
                        let mut value = cleanup_values(split2, ": ".to_string(), "");
                        value = value.trim().to_owned();
                        if split1.contains("Genre") {
                            list_of_category =
                                Category::insert_category_in_array(list_of_category, &value);
                        }

                        if split1.contains("Statut") {
                            list_of_status = Status::insert_status_in_array(list_of_status, &value);
                        }

                        if split1.contains("Style") {
                            list_of_style =
                                Saga_Style::insert_style_in_array(list_of_style, &value);
                        }

                        if split1.contains("Saison") {
                            total_of_season = total_of_season + 1;
                        } else {
                            data_array.insert(key, value);
                        }
                    }
                }
            }
            Some(_) => {}
        }
    }

    data_array.insert("Season".to_string(), total_of_season.to_string());
    data_array.insert("Synopsis".to_string(), synopsis.trim().to_string());
    let saga = create_saga(
        data_array,
        list_of_category,
        list_of_status,
        list_of_style,
        name,
    );
}

fn create_saga(
    data_array: HashMap<String, String>,
    category_array: Vec<Category>,
    status_array: Vec<Status>,
    list_of_style: Vec<Saga_Style>,
    name: String,
) -> Saga {
    let mut saga: Saga = Saga::default();
    saga.name = Some(name);
    for (key, value) in data_array {
        match key.as_str() {
            "Auteur" => {
                saga.author = Some(value);
            }
            "Musique" => {
                saga.music = Some(value);
            }
            "Origine" => {
                let country = LIST_OF_COUNTRY.iter().find(|country| country.name == value);
                match country {
                    None => saga.country_of_origin = None,
                    Some(country) => {
                        saga.country_of_origin = Some(country.id);
                    }
                }
            }
            "Genre" => {
                match category_array
                    .iter()
                    .find(|category| category.name == Some(value.to_string()))
                {
                    None => saga.category = None,
                    Some(category) => {
                        saga.category = Some(category.id);
                    }
                }
            }
            "Style" => {
                match list_of_style
                    .iter()
                    .find(|style| style.name == value.to_string())
                {
                    None => saga.saga_type = None,
                    Some(status) => {
                        saga.saga_type = Some(status.id as i16);
                    }
                }
            }
            "Statut" => {
                match status_array
                    .iter()
                    .find(|status| status.name == value.to_string())
                {
                    None => saga.status = None,
                    Some(status) => {
                        saga.status = Some(status.id as i16);
                    }
                }
            }
            "Création" => {
                saga.creation_date = Some(value);
            }
            "Season" => {
                saga.season = Some(value.parse::<i16>().unwrap_or(0));
            }
            "Synopsis" => {
                saga.description = Some(value);
            }
            _ => {},
        }
    }

    saga
}

fn cleanup_values(mut text: &str, char: String, to: &str) -> String {
    text.replace(&char, to)
}

fn find_synopsis(data: &Vec<&str>) -> String {
    match data.iter().position(|&x| x == "Synopsis") {
        None => "".to_string(),
        Some(index) => {
            if index + 1 >= data.len() {
                return "".to_string();
            }
            data[index + 1].replace("\n", " ")
        }
    }
}

/// Find wanted key in text and add $ at the beginning of the key for later
///
/// # Examples
///
/// ```
/// WANTED_KEY = ["Saga"];
/// text = "La Saga : nom";
/// find_wanted_key_in_text(text) => "La &Saga : nom"
/// ```
fn find_wanted_key_in_text(mut text: String) -> String {
    WANTED_KEYS.into_iter().for_each(|key| {
        if !text.contains(key) {
            return;
        }
        text = detect_wanted_keys(&text, key);
    });

    text
}

fn split_on_sign(text: &str, sign: String) -> Vec<&str> {
    let mut split_sections = text.split(&sign);
    split_sections.collect::<Vec<&str>>()
}

// If the key exist in the text concatenate it with th $ sign
fn detect_wanted_keys(mut text: &str, key: &str) -> String {
    text.replace(&key, &String::from("$".to_owned() + &key))
}
