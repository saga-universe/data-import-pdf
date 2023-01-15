extern crate lopdf;
extern crate pdf_extract;

use core::panic;
use data::saga::NewSaga;
use data::status;
use pdf_extract::*;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::PathBuf;

mod data;
mod db;
mod schema;
mod test;

use crate::data::category::FullCategory;
use crate::data::category::LIST_OF_CATEGORY;
use crate::data::country;
use crate::data::country::Country;
use crate::data::country::FullCountry;
use crate::data::sagas_subcategories::SagasSubcategories;
use crate::data::sub_categories::FullSubcategory;
use crate::data::sub_categories::LIST_OF_SUBCATEGORY;
use crate::data::sub_categories::Subcategory;
use crate::data::status::FullStatus;
use crate::data::{status::Status, status::LIST_OF_STATUS};
use crate::schema::sagas::categoryID;
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
    let mut data_array: Vec<HashMap<String, String>> = vec![];
    let mut list_of_status: Vec<FullStatus> = vec![];
    let mut list_of_category: Vec<FullCategory> = vec![];
    let mut list_of_subcategory: Vec<FullSubcategory> = vec![];
    let mut list_of_country: Vec<FullCountry> = vec![];

    let mut saga: Vec<NewSaga> = vec![];
    let mut conn = db::connect();
    Status::create(&mut conn, LIST_OF_STATUS);
    list_of_status = Status::find(&mut conn);

    Country::create(&mut conn, LIST_OF_COUNTRY);
    list_of_country = Country::find(&mut conn);

    Category::create(&mut conn,LIST_OF_CATEGORY);
    list_of_category = Category::find(&mut conn);

    Subcategory::create(&mut conn,LIST_OF_SUBCATEGORY);
    list_of_subcategory = Subcategory::find(&mut conn);

    for path in paths {
        let file = path.as_ref().unwrap().file_name().into_string();
        let mut name = "".to_string();
        let output_kind = env::args().nth(2).unwrap_or_else(|| "txt".to_owned());
        let path_name = path.unwrap().path();
        let filename = path_name.file_name().expect("expected a filename");
        let mut output_file = PathBuf::new();
        output_file.push(filename);
        output_file.set_extension(&output_kind);

        let info = extract_text(path_name);

        if let Ok(mut val) = info {
            // Isolate wanted info by adding a $ at its beginning
            val = find_wanted_key_in_text(val);
            val = cleanup_values(&val, "\u{a0}".to_string(), "");
            val = cleanup_values(&val, "\n\n".to_string(), "$");
            val = cleanup_values(&val, "\n".to_string(), " ");
            let mut pdf_data = find_data_in_text_string(val, name);
            if pdf_data.contains_key("Statut") {
                match pdf_data.get("Statut") {
                    None => {}
                    Some(status) => {
                        let cleaned_status = Status::clean_status_list(&status);
                        let status_key =
                            Status::find_status_in_array(&list_of_status, &cleaned_status.name);
                        pdf_data.insert("Statut".to_string(), status_key);
                    }
                }
            }
            
            if pdf_data.contains_key("Origine") {
                match pdf_data.get("Origine") {
                    None => {}
                    Some(country) => {
                        let country_key = Country::find_country_in_array(
                            &list_of_country,
                            country,
                        );
                        pdf_data.insert("Origine".to_string(), country_key);
                    }
                }
            }

            if pdf_data.contains_key("Genre") {
                match pdf_data.get("Genre") {
                    None => {}
                    Some(category) => {
                        let cleaned_category = Category::clean_category_list(&category);
                        let category_key = Category::find_category_in_array(
                            &list_of_category,
                            &cleaned_category.name,
                        );
                        pdf_data.insert("Genre".to_string(), category_key);
                    }
                }
            }

            if pdf_data.contains_key("Style") {
                match pdf_data.get("Style") {
                    None => {}
                    Some(category) => {
                        let cleaned_subcategory = Subcategory::clean_subcategory_list(&category);
                        let subcategory_key = Subcategory::find_subcategory_in_array(
                            &list_of_subcategory,
                            &cleaned_subcategory.name,
                        );
                        pdf_data.insert("Style".to_string(), subcategory_key);
                    }
                }
            }
            match file {
                Ok(filename) => {
                    name = filename.to_string();
                    name = cleanup_values(&name, ".pdf".to_string(), "");
                    let new_saga = create_saga(&pdf_data, name);
                    saga.push(new_saga.clone());
                    let saga_id = Saga::create(&mut conn, new_saga);
                    

                    match pdf_data.get("Style") {
                        Some(subcategory) => {
                            match subcategory.parse::<u32>() {
                                Ok(id) => {
                                    let new_saga_subcategory = SagasSubcategories{sagaID: saga_id, subcategoryID: id};
                                    SagasSubcategories::create(&mut conn, new_saga_subcategory)
                                },
                                Err(e) => panic!("No id found for country: {:#?}", e),
                            }                            
                        }
                        None => {}
                    }
                    
                }
                Err(_) => {}
            }
        }
    }
    println!("Data array: {:#?}", saga.len());
}

fn find_data_in_text_string(data: String, file: String) -> HashMap<String, String> {
    let mut saga_info: HashMap<String, String> = HashMap::new();
    let mut total_of_season = 0;

    let name = cleanup_values(&file, ".pdf".to_string(), "")
        .trim()
        .to_string();

    let index = split_on_sign(&data, "$".to_string());

    let synopsis = find_synopsis(&index);
    saga_info.insert("Synopsis".to_string(), synopsis.trim().to_string());

    for element in index {
        // If I found an URL
        match element.find("http") {
            None => {
                let index = element.find(':');
                match index {
                    None => {}
                    Some(split_index) => {
                        let (split1, split2) = element.split_at(split_index);
                        let key = cleanup_values(split1, " ".to_string(), "");
                        let mut value = cleanup_values(split2, ": ".to_string(), "");
                        value = value.trim().to_owned();

                        if split1.contains("Saison") {
                            total_of_season = total_of_season + 1;
                        } else {
                            saga_info.insert(key, value);
                        }
                    }
                }
            }
            Some(_) => {}
        }
    }

    saga_info.insert("Season".to_string(), total_of_season.to_string());
    saga_info.insert("Name".to_string(), name);

    saga_info
}


fn create_saga(data_array: &HashMap<String, String>, name: String) -> NewSaga {
    let mut saga: NewSaga = NewSaga::default();
    saga.name = name;
    for (key, value) in data_array {
        match key.as_str() {
            "Auteur" => {
                saga.author = value.to_string();
            }
            "Musique" => {
                saga.music = value.to_string();
            }
            "Origine" => match value.parse::<u32>() {
                Ok(country_id) => saga.countryID = country_id,
                Err(e) => panic!("No id found for country: {:#?}", e),
            },
            "Genre" => match value.parse::<u32>() {
                Ok(category_id) => saga.categoryID = category_id,
                Err(e) => panic!("No id found for category: {:#?}", e),
            },
            "Statut" => match value.parse::<u32>() {
                Ok(status_id) => saga.statusID = status_id,
                Err(e) => panic!("No id found for status: {:#?}", e),
            },
            "Création" => {
                saga.creation_date = value.to_string();
            }
            "Season" => {
                saga.season = value.parse::<u16>().unwrap_or(0);
            }
            "Synopsis" => {
                saga.description = value.to_string();
            }
            _ => {}
        }
    }
    saga
}

fn cleanup_values(text: &str, char: String, to: &str) -> String {
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
    let split_sections = text.split(&sign);
    split_sections.collect::<Vec<&str>>()
}

// If the key exist in the text concatenate it with th $ sign
fn detect_wanted_keys(text: &str, key: &str) -> String {
    text.replace(&key, &String::from("$".to_owned() + &key))
}
