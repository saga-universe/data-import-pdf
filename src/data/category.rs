use diesel::{Insertable, MysqlConnection, insert_into, QueryDsl, RunQueryDsl, Queryable};
use serde::Deserialize;

use crate::schema::categories;

#[derive(Debug)]
pub struct Category {
    pub name: String,
}

#[derive(Debug, Queryable, Deserialize)]
#[diesel(table_name = categories)]
pub struct FullCategory {
    pub id: u32,
    pub name: String,
}

#[derive(Debug, Deserialize, Clone, Insertable)]
#[diesel(table_name = categories)]
pub struct NewCategory<'a> {
    pub name: &'a str,
}

pub const LIST_OF_CATEGORY: [NewCategory; 90] = [
    NewCategory { name: "Mono mp3" },
    NewCategory {
        name: "Super héros",
    },
    NewCategory { name: "Parodie" },
    NewCategory { name: "Fantasy" },
    NewCategory {
        name: "Fantastique",
    },
    NewCategory {
        name: "Science-fiction",
    },
    NewCategory { name: "Espionnage" },
    NewCategory { name: "Thriller" },
    NewCategory { name: "Absurde" },
    NewCategory {
        name: "Contemporain",
    },
    NewCategory {
        name: "Série de Mono",
    },
    NewCategory {
        name: "Jeux vidéo"
    },
    NewCategory { name: "Quotidien" },
    NewCategory { name: "Cyberpunk" },
    NewCategory {
        name: "Expérimental",
    },
    NewCategory { name: "Émission" },
    NewCategory { name: "Policier" },
    NewCategory { name: "Historique" },
    NewCategory { name: "Socatoa" },
    NewCategory {
        name: "Épopée navale",
    },
    NewCategory { name: "Reportage" },
    NewCategory { name: "Délire" },
    NewCategory {
        name: "Art martial",
    },
    NewCategory {
        name: "Saga de Noël",
    },
    NewCategory { name: "Polar" },
    NewCategory { name: "ARG" },
    NewCategory { name: "Interview" },
    NewCategory { name: "Top" },
    NewCategory {
        name: "Post-apocalyptique",
    },
    NewCategory { name: "Adaptation" },
    NewCategory { name: "Western" },
    NewCategory { name: "Politique" },
    NewCategory { name: "Pirates" },
    NewCategory {
        name: "Émission Culturelle",
    },
    NewCategory { name: "Monologue" },
    NewCategory {
        name: "Found footage",
    },
    NewCategory {
        name: "Multi-genres",
    },
    NewCategory { name: "Mythologie" },
    NewCategory { name: "Zombie" },
    NewCategory { name: "Sitcom" },
    NewCategory {
        name: "Publicitaire",
    },
    NewCategory { name: "Enquête" },
    NewCategory {
        name: "Pop culture",
    },
    NewCategory { name: "Horreur" },
    NewCategory { name: "Film" },
    NewCategory { name: "Scolaire" },
    NewCategory {
        name: "Anticipation",
    },
    NewCategory {
        name: "Saga dont tu es le héros",
    },
    NewCategory { name: "Dystopie" },
    NewCategory {
        name: "Space opéra",
    },
    NewCategory { name: "Théâtre" },
    NewCategory {
        name: "Vie d'entreprise",
    },
    NewCategory { name: "Uchronie" },
    NewCategory { name: "Aventure" },
    NewCategory { name: "Steampunk" },
    NewCategory { name: "Conte" },
    NewCategory { name: "Journal" },
    NewCategory { name: "Oriental" },
    NewCategory { name: "Chronique" },
    NewCategory { name: "Anti-saga" },
    NewCategory { name: "Manga" },
    NewCategory { name: "Survival" },
    NewCategory {
        name: "Jeux de rôle",
    },
    NewCategory {
        name: "Super-héroïque",
    },
    NewCategory { name: "Humour" },
    NewCategory {
        name: "Communautaire",
    },
    NewCategory { name: "Suspense" },
    NewCategory {
        name: "Émission Radio",
    },
    NewCategory { name: "Critique" },
    NewCategory {
        name: "Tranche de vie",
    },
    NewCategory { name: "Drama" },
    NewCategory {
        name: "Naturaliste",
    },
    NewCategory { name: "Médiéval" },
    NewCategory { name: "Moderne" },
    NewCategory {
        name: "Improvisation",
    },
    NewCategory {
        name: "Pré-historique",
    },
    NewCategory {
        name: "Télévision"
    },
    NewCategory {
        name: "Dictionnaire",
    },
    NewCategory { name: "Bootleg" },
    NewCategory { name: "Guerre" },
    NewCategory {
        name: "Ecclésiastique",
    },
    NewCategory { name: "Nanar" },
    NewCategory {
        name: "Voyage spatio-temporel",
    },
    NewCategory { name: "Capsule" },
    NewCategory {
        name: "Bande annonce",
    },
    NewCategory { name: "Onirique" },
    NewCategory {
        name: "Exploration",
    },
    NewCategory { name: "Médical" },
    NewCategory { name: "Inconnus" },
    NewCategory { name: "Cinéma" },
];

impl Category {
    
    pub fn clean_category_list(value: &str) -> Category {
        match value {
            "Mono" | "Mono mp3" | "Monos mp3" | "MONO mp3" => Category {
                name: String::from("Mono mp3"),
            },
            "Super héros" | "Super-héros" => Category {
                name: String::from("Super héros"),
            },
            "Prodie"
            | "Parodie de jeux  vidéo"
            | "Parodie de Jeu  Vidéo"
            | "Parodie de film"
            | "Parodique"
            | "Parodie de Jeu  vidéo"
            | "Parodie de jeu  vidéo"
            | "Parodie de jeu"
            | "Film parodie"
            | "Parodie" => Category {
                name: String::from("Parodie"),
            },
            "fantastique" | "Médiéval- fantastique" | "Fantastique" => Category {
                name: String::from("Fantastique"),
            },
            "Science-fiction,  musical, parodie"
            | "Sciences fiction"
            | "Science fiction"
            | "Science-fiction Heroic fantasy"
            | "Science Fiction"
            | "Science-fiction" => Category {
                name: String::from("Science-fiction"),
            },
            "Espionnage" => Category {
                name: String::from("Espionnage"),
            },
            "Triller" | "Thrille" | "Thriller" => Category {
                name: String::from("Thriller"),
            },
            "Absurde" => Category {
                name: String::from("Absurde"),
            },
            "Contemporaine"
            | "Contemporain,  Policier"
            | "temporain  Heroic fantasy"
            | "Contemporain" => Category {
                name: String::from("Contemporain"),
            },
            "Série de Mono" | "Série Mono" => Category {
                name: String::from("Série de Mono"),
            },
            "Jeux video" | "Jeux Vidéo" | "Jeux vidéo" => Category {
                name: String::from("Jeux vidéo"),
            },
            "Quotidien" => Category {
                name: String::from("Quotidien"),
            },
            "Cyberpunk" => Category {
                name: String::from("Cyberpunk"),
            },
            "Expérimental" => Category {
                name: String::from("Expérimental"),
            },
            "Émission" => Category {
                name: String::from("Émission"),
            },
            "Policier" => Category {
                name: String::from("Policier"),
            },
            "Histoire" | "Historique" => Category {
                name: String::from("Historique"),
            },
            "SOCQATOA" | "Socatoa" | "Socqatoa" | "SOCATOA" => Category {
                name: String::from("Socatoa"),
            },
            "Épopée navale" | "Naufrage" => Category {
                name: String::from("Épopée navale"),
            },
            "Reportage" => Category {
                name: String::from("Reportage"),
            },
            "Délire" => Category {
                name: String::from("Délire"),
            },
            "Heroic fantasy  Fiction politique"
            | "Heroic-Fantasy"
            | "Héroic fantasy"
            | "Heroic-fantasy"
            | "Heroic Fantasy"
            | "Heroic fantasy"
            | "Science-fantasy"
            | "Dark fantasy"
            | "Tarantino- fantasy"
            | "Fanrasy"
            | "Fantasy" => Category {
                name: String::from("Fantasy"),
            },
            "Art martial" => Category {
                name: String::from("Art martial"),
            },
            "Saga de Noël" => Category {
                name: String::from("Saga de Noël"),
            },
            "Polar noir" | "Polar Noir" | "Polar" => Category {
                name: String::from("Polar"),
            },
            "ARG" => Category {
                name: String::from("ARG"),
            },
            "Interview" => Category {
                name: String::from("Interview"),
            },
            "Top" | "Top 50" => Category {
                name: String::from("Top"),
            },
            "Apocalypse"
            | "Post- apocalyptique"
            | "Post- apo"
            | "Apocalyptique"
            | "Post apocalypse"
            | "Post-apo"
            | "Post apo" => Category {
                name: String::from("Post-apocalyptique"),
            },
            "Fan-adaptation" | "Adaptation" => Category {
                name: String::from("Adaptation"),
            },
            "Western" | "Western  futuriste" => Category {
                name: String::from("Western"),
            },
            "Politique" => Category {
                name: String::from("Politique"),
            },
            "Pirates" | "Pirate" => Category {
                name: String::from("Pirates"),
            },
            "Émission  Culturelle" => Category {
                name: String::from("Émission Culturelle"),
            },
            "Monologue" => Category {
                name: String::from("Monologue"),
            },
            "found footage" => Category {
                name: String::from("Found footage"),
            },
            "Multi-genres" => Category {
                name: String::from("Multi-genres"),
            },
            "Mythologique" | "Mythologie" => Category {
                name: String::from("Mythologie"),
            },
            "Zombie" => Category {
                name: String::from("Zombie"),
            },
            "Sitcom" => Category {
                name: String::from("Sitcom"),
            },
            "Publicitaire" => Category {
                name: String::from("Publicitaire"),
            },
            "Enquêtes" | "Enquête" => Category {
                name: String::from("Enquête"),
            },
            "Pop culture" => Category {
                name: String::from("Pop culture"),
            },
            "Horrifique" | "Horreur" => Category {
                name: String::from("Horreur"),
            },
            "Péplum" | "Blockbuster" | "Film noir" | "Cinéma" => Category {
                name: String::from("Cinéma"),
            },
            "Scolaire" => Category {
                name: String::from("Scolaire"),
            },
            "Anticipation" => Category {
                name: String::from("Anticipation"),
            },
            "Saga dont tu es  le héros" => Category {
                name: String::from("Saga dont tu es le héros"),
            },
            "Dystopie" => Category {
                name: String::from("Dystopie"),
            },
            "Space opéra" | "Space Opera" | "Space opera" => Category {
                name: String::from("Space opéra"),
            },
            "Théâtre" | "Saynète" => Category {
                name: String::from("Théâtre"),
            },
            "Entreprise" | "Vie d'entreprise" => Category {
                name: String::from("Vie d'entreprise"),
            },
            "Uchronie" => Category {
                name: String::from("Uchronie"),
            },
            "Aventure" => Category {
                name: String::from("Aventure"),
            },
            "Steampunk" => Category {
                name: String::from("Steampunk"),
            },
            "Conte philosophique" | "Conte" | "Conte de fée" | "Contes" => Category {
                name: String::from("Conte"),
            },
            "Journal" | "Journalistique" => Category {
                name: String::from("Journal"),
            },
            "Oriental" => Category {
                name: String::from("Oriental"),
            },
            "Chronique" => Category {
                name: String::from("Chronique"),
            },
            "Anti-saga" => Category {
                name: String::from("Anti-saga"),
            },
            "Magical girl" | "Super sentai" | "Manga" => Category {
                name: String::from("Manga"),
            },
            "Survival-Horror" => Category {
                name: String::from("Survival"),
            },
            "Jeu de rôle" | "JDR" | "Jeux de rôle" => Category {
                name: String::from("Jeux de rôle"),
            },
            "Super-héroïque" => Category {
                name: String::from("Super-héroïque"),
            },
            "Suite de blague" | "Sketch" | "Humour" | "Sketchs" => Category {
                name: String::from("Humour"),
            },
            "Communautaire" => Category {
                name: String::from("Communautaire"),
            },
            "Suspens" => Category {
                name: String::from("Suspense"),
            },
            "Émission  radiophonique" | "Radio" | "Émission Radio" => Category {
                name: String::from("Émission Radio"),
            },
            "Critique" => Category {
                name: String::from("Critique"),
            },
            "Tranche de vie" => Category {
                name: String::from("Tranche de vie"),
            },
            "Drama" => Category {
                name: String::from("Drama"),
            },
            "Naturaliste" => Category {
                name: String::from("Naturaliste"),
            },
            "Medévial" | "Médiéval" => Category {
                name: String::from("Médiéval"),
            },
            "Moderne" => Category {
                name: String::from("Moderne"),
            },
            "Improvisation" => Category {
                name: String::from("Improvisation"),
            },
            "Pré-historique" => Category {
                name: String::from("Pré-historique"),
            },
            "Télévision" => Category {
                name: String::from("Télévision"),
            },
            "Dictionnaire" => Category {
                name: String::from("Dictionnaire"),
            },
            "Bootleg" => Category {
                name: String::from("Bootleg"),
            },
            "Guerre" => Category {
                name: String::from("Guerre"),
            },
            "Ecclésiastique" => Category {
                name: String::from("Ecclésiastique"),
            },
            "Nanard" | "Nanar" => Category {
                name: String::from("Nanar"),
            },
            "Voyage spatio- temporel" => Category {
                name: String::from("Voyage spatio-temporel"),
            },
            "Capsule" => Category {
                name: String::from("Capsule"),
            },
            "Bande annonce" => Category {
                name: String::from("Bande annonce"),
            },
            "Onirique" => Category {
                name: String::from("Onirique"),
            },
            "Exploration" => Category {
                name: String::from("Exploration"),
            },
            "Psychophobie" | "Psychologie" | "Médical" => Category {
                name: String::from("Médical"),
            },
            _ => Category {
                name: String::from("Inconnus"),
            },
        }
    }

    pub fn find_category_in_array(list_of_status: &Vec<FullCategory>, value: &str) -> String {
        let category = list_of_status.iter().find(|x| x.name == value);

        match category {
            Some(category_info) => String::from(category_info.id.to_string()),
            None => {
                panic!("This category {:#?}  doesn't exist in db : {:#?}", value,value);
            }
        }
    }

    pub fn create<'a>(conn: &mut MysqlConnection, records: [NewCategory<'a>; 90]) {
        use crate::schema::categories::dsl::*;
        let record_inserted = insert_into(categories).values::<Vec<NewCategory<'a>>>(records.to_vec()).execute(conn);
        match record_inserted {
            Ok(_)=>{}
            Err(err)=>{
                panic!("Error during insertion: {:#?}", err)
            }
        }
    }

    pub fn find(conn: &mut MysqlConnection) -> Vec<FullCategory> {
        use crate::schema::categories::dsl::*;
        match categories.select((id,name)).load::<FullCategory>(conn) {
            Ok(categories_list) => {
                categories_list
            },
            Err(_) => {
                panic!("No status found in database");
            }
        }
    }
}
