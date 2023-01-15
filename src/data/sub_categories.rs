use diesel::{Insertable, MysqlConnection, QueryDsl, Queryable, insert_into, RunQueryDsl};
use serde::Deserialize;

use crate::schema::subcategories;

#[derive(Debug)]
pub struct Subcategory {
    pub name: String,
}
#[derive(Debug, Deserialize, Clone, Insertable)]
#[diesel(table_name = subcategories)]
pub struct NewSubcategories<'a> {
    pub name: &'a str,
}

#[derive(Debug, Queryable, Deserialize)]
#[diesel(table_name = subcategories)]
pub struct FullSubcategory {
    pub id: u32,
    pub name: String,
}

pub const LIST_OF_SUBCATEGORY: [NewSubcategories; 56] = [
    NewSubcategories { name: "Sérieux" },
    NewSubcategories { name: "Aventure" },
    NewSubcategories { name: "Suspense" },
    NewSubcategories { name: "Action" },
    NewSubcategories { name: "Sombre" },
    NewSubcategories { name: "Horreur" },
    NewSubcategories { name: "Absurde" },
    NewSubcategories { name: "Drame" },
    NewSubcategories { name: "Policier" },
    NewSubcategories { name: "Enquête" },
    NewSubcategories { name: "Sarcasme" },
    NewSubcategories { name: "Lyrisme" },
    NewSubcategories { name: "Polar" },
    NewSubcategories { name: "Déprimant" },
    NewSubcategories { name: "Survival" },
    NewSubcategories {
        name: "Misanthrope",
    },
    NewSubcategories { name: "Onirique" },
    NewSubcategories {
        name: "Multi-styles",
    },
    NewSubcategories { name: "Thriller" },
    NewSubcategories { name: "Sérieux" },
    NewSubcategories { name: "Tragique" },
    NewSubcategories { name: "Humour" },
    NewSubcategories {
        name: "Philosophique",
    },
    NewSubcategories { name: "Comédie" },
    NewSubcategories {
        name: "Improvisation",
    },
    NewSubcategories { name: "Exercice" },
    NewSubcategories {
        name: "Psychologique",
    },
    NewSubcategories { name: "Épique" },
    NewSubcategories {
        name: "Surréaliste",
    },
    NewSubcategories { name: "Oppressant" },
    NewSubcategories {
        name: "Jeu de rôle",
    },
    NewSubcategories { name: "Narratif" },
    NewSubcategories { name: "Léger" },
    NewSubcategories { name: "Épouvante" },
    NewSubcategories { name: "Héroïque" },
    NewSubcategories { name: "Fanfiction" },
    NewSubcategories { name: "Délire" },
    NewSubcategories { name: "Cyberpunk" },
    NewSubcategories {
        name: "Infiltration",
    },
    NewSubcategories {
        name: "Mélancolique",
    },
    NewSubcategories {
        name: "Contemplatif",
    },
    NewSubcategories { name: "Paranormal" },
    NewSubcategories { name: "Réaliste" },
    NewSubcategories { name: "Voyage" },
    NewSubcategories { name: "Musical" },
    NewSubcategories { name: "Politique" },
    NewSubcategories { name: "Immersif" },
    NewSubcategories { name: "Émouvant" },
    NewSubcategories { name: "Parodique" },
    NewSubcategories { name: "Espionnage" },
    NewSubcategories { name: "Satire" },
    NewSubcategories {
        name: "Expérimental",
    },
    NewSubcategories {
        name: "Tragicomédie",
    },
    NewSubcategories { name: "Guerre" },
    NewSubcategories { name: "Noir" },
    NewSubcategories { name: "-" },
];

impl Subcategory {

    pub fn clean_subcategory_list(value: &str) -> Subcategory {
        match value {
            "Sérieuse" | "Sérieux" => Subcategory {
                name: String::from("Sérieux"),
            },
            "Aventure," | "Aventure  humour" | "Humour Aventure" | "aventure"
            | "Aventures   humour" | "Aventure,  humour" | "Aventure" => Subcategory {
                name: String::from("Aventure"),
            },
            "Suspense" => Subcategory {
                name: String::from("Suspense"),
            },
            "Humour noir" | "Humour, action" | "Humour, action  aventure" | "Action" => {
                Subcategory {
                    name: String::from("Action"),
                }
            }
            "Humour-  sombre" | "Sombre, Action" | "Sombre" => Subcategory {
                name: String::from("Sombre"),
            },
            "Expérimental,  horreur"
            | "Fantastique,  horreur"
            | "Horreur humour"
            | "Horreur  suspense"
            | "Horreur" => Subcategory {
                name: String::from("Horreur"),
            },
            "Nanar, absurde" | "Sérieux absurde" | "Humour,  absurde" | "Absurde,"
            | "Humour,  Absurde" | "Absurde" => Subcategory {
                name: String::from("Absurde"),
            },
            "Drame aventure" | "Humour, Drame" | "Humour Drame" | "Dramatique"
            | "comico- dramatique" | "Drame" => Subcategory {
                name: String::from("Drame"),
            },
            "Policier" => Subcategory {
                name: String::from("Policier"),
            },
            "Enquête" | "Enquêtes" => Subcategory {
                name: String::from("Enquête"),
            },
            "Sarcasme" => Subcategory {
                name: String::from("Sarcasme"),
            },
            "Lyrisme" => Subcategory {
                name: String::from("Lyrisme"),
            },
            "Polar" => Subcategory {
                name: String::from("Polar"),
            },
            "Déprimante" => Subcategory {
                name: String::from("Déprimant"),
            },
            "Survival" => Subcategory {
                name: String::from("Survival"),
            },
            "Misanthropie" | "Misanthrope" => Subcategory {
                name: String::from("Misanthrope"),
            },
            "Onirique" => Subcategory {
                name: String::from("Onirique"),
            },
            "Multi-styles" => Subcategory {
                name: String::from("Multi-styles"),
            },
            "Thriller, roman noir" | "Thriller" => Subcategory {
                name: String::from("Thriller"),
            },
            "Sérieux Humour" => Subcategory {
                name: String::from("Sérieux"),
            },
            "Tragique" | "tragi-comique" => Subcategory {
                name: String::from("Tragique"),
            },
            "Humours" | "Comédie de  mœurs" | "Comique" | "Humoristique" | "Humour" | "humour" => {
                Subcategory {
                    name: String::from("Humour"),
                }
            }
            "Philosophique" => Subcategory {
                name: String::from("Philosophique"),
            },
            "Comédie" => Subcategory {
                name: String::from("Comédie"),
            },
            "Conte,  improvisation" => Subcategory {
                name: String::from("Improvisation"),
            },
            "Exercice" => Subcategory {
                name: String::from("Exercice"),
            },
            "Psychologique" | "psychologique" => Subcategory {
                name: String::from("Psychologique"),
            },
            "Humour épique" | "Drame, Épique" | "Épique" => Subcategory {
                name: String::from("Épique"),
            },
            "Surréaliste" => Subcategory {
                name: String::from("Surréaliste"),
            },
            "Absurde,  oppressant" | "Absurde,  Oppressant" | "Oppressant" => Subcategory {
                name: String::from("Oppressant"),
            },
            "Jeu de rôle" => Subcategory {
                name: String::from("Jeu de rôle"),
            },
            "Narratif" => Subcategory {
                name: String::from("Narratif"),
            },
            "Léger" => Subcategory {
                name: String::from("Léger"),
            },
            "Épouvante" => Subcategory {
                name: String::from("Épouvante"),
            },
            "Héroïque" => Subcategory {
                name: String::from("Héroïque"),
            },
            "Fanfiction" => Subcategory {
                name: String::from("Fanfiction"),
            },
            "Délire" => Subcategory {
                name: String::from("Délire"),
            },
            "Cyberpunk" => Subcategory {
                name: String::from("Cyberpunk"),
            },
            "Action- Infiltration" => Subcategory {
                name: String::from("Infiltration"),
            },
            "Drame,  Mélancolique" => Subcategory {
                name: String::from("Mélancolique"),
            },
            "Contemplatif" => Subcategory {
                name: String::from("Contemplatif"),
            },
            "Comédie,  paranormal,  horreur" => Subcategory {
                name: String::from("Paranormal"),
            },
            "Réaliste" => Subcategory {
                name: String::from("Réaliste"),
            },
            "Suspense  Voyage" => Subcategory {
                name: String::from("Voyage"),
            },
            "Humour,  musical" => Subcategory {
                name: String::from("Musical"),
            },
            "Sérieux,  politique" | "Thriller  Politique" => Subcategory {
                name: String::from("Politique"),
            },
            "Immersif" => Subcategory {
                name: String::from("Immersif"),
            },
            "Drame Émouvant" => Subcategory {
                name: String::from("Émouvant"),
            },
            "Parodique" => Subcategory {
                name: String::from("Parodique"),
            },
            "Espionnage,  Humour" | "Espionnage" => Subcategory {
                name: String::from("Espionnage"),
            },
            "Humour, Satire" => Subcategory {
                name: String::from("Satire"),
            },
            "Expérimental" => Subcategory {
                name: String::from("Expérimental"),
            },
            "Tragicomédie" => Subcategory {
                name: String::from("Tragicomédie"),
            },
            "Guerre" => Subcategory {
                name: String::from("Guerre"),
            },
            "Polar noir" => Subcategory {
                name: String::from("Noir"),
            },
            _ => Subcategory {
                name: String::from("-"),
            },
        }
    }

    pub fn find_subcategory_in_array(list_of_subcategory: &Vec<FullSubcategory>, value: &str) -> String {
        let subcategory = list_of_subcategory.iter().find(|x| x.name == value);
        match subcategory {
            Some(subcategory_info) => String::from(subcategory_info.id.to_string()),
            None => {
                panic!("This category {:#?}  doesn't exist in db : {:#?}", value,value);
            }
        }
    }

    pub fn create<'a>(conn: &mut MysqlConnection, records: [NewSubcategories<'a>; 56]) {
        use crate::schema::subcategories::dsl::*;
        let record_inserted = insert_into(subcategories).values::<Vec<NewSubcategories<'a>>>(records.to_vec()).execute(conn);
        match record_inserted {
            Ok(_)=>{}
            Err(err)=>{
                panic!("Error during insertion: {:#?}", err)
            }
        }
    }

    pub fn find(conn: &mut MysqlConnection) -> Vec<FullSubcategory> {
        use crate::schema::subcategories::dsl::*;
        match subcategories.select((id,name)).load::<FullSubcategory>(conn) {
            Ok(categories_list) => {
                categories_list
            },
            Err(_) => {
                panic!("No subcategory found in database");
            }
        }
    }

}
