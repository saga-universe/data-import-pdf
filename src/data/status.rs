use std::error::Error;

use crate::schema::status;
use diesel::{prelude::*, sql_query, FromSqlRow, insert_into};
use serde::Deserialize;
use chrono::NaiveDateTime;

#[derive(Debug, Queryable,)]
pub struct Status {
    pub name: String,
}

#[diesel(table_name = status)]
#[derive(Queryable, Deserialize, Debug)]
pub struct FullStatus {
    pub id: u32,
    pub name: String,
}


#[derive(Debug, Deserialize, Clone, Insertable)]
#[diesel(table_name = status)]
pub struct NewStatus<'a> {
    pub name: &'a str,
}

const ABANDONNEE: &str = "Abandonnée";
const EN_COURS: &str = "En cours";
const TERMINEE: &str = "Terminée";
const PAUSE: &str = "En pause";
const INCONNUS: &str = "Inconnus";

pub const LIST_OF_STATUS: [NewStatus; 5] = [
    NewStatus{name:ABANDONNEE}, 
    NewStatus{name:EN_COURS}, 
    NewStatus{name:TERMINEE}, 
    NewStatus{name:PAUSE}, 
    NewStatus{name:INCONNUS}
];


impl Status {

    pub fn clean_status_list(value: &str) -> Status {
        match value {
            "Abandonnée" | "Abandonné" | "Abandonner" | "abandonné" => Status {
                name: String::from(ABANDONNEE),
            },
            "En cours" |"En cour" => Status {
                name: String::from(EN_COURS),
            },
            "Terminée" | "Terminé" | "Terminer" | "terminer" | "terminée"  => Status {
                name: String::from(TERMINEE),
            },
            "En pause" => Status {
                name: String::from(PAUSE),
            },
            _ => Status {
                name: String::from(INCONNUS),
            },
        }
    }

    pub fn find_status_in_array(list_of_status: &Vec<FullStatus>, value: &str) -> String {
        let status = list_of_status.iter().find(|x| x.name == value);

        match status {
            Some(status) => {
                String::from(status.id.to_string())
            }
            None => {
                panic!("This status doesn't exist in db : {:#?}", value);
            }
        }
    }

    pub fn create<'a>(conn: &mut MysqlConnection, records: [NewStatus<'a>; 5]) {
        use crate::schema::status::dsl::*;
        let record_inserted = insert_into(status).values::<Vec<NewStatus<'a>>>(records.to_vec()).execute(conn);
        match record_inserted {
            Ok(_)=>{}
            Err(err)=>{
                panic!("Error during insertion: {:#?}", err)
            }
        }
    }

    pub fn find(conn: &mut MysqlConnection) -> Vec<FullStatus> {
        use crate::schema::status::dsl::*;
        match status.select((id,name)).load::<FullStatus>(conn) {
            Ok(status_list) => {
                status_list
            },
            Err(_) => {
                panic!("No status found in database");
            }
        }
    }
}
