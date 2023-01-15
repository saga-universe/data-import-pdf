use chrono::NaiveDateTime;
use diesel::{Insertable, MysqlConnection, insert_into, RunQueryDsl, Queryable, sql_query, QueryableByName, QueryDsl, sql_types::Integer};
use serde::Deserialize;
use crate::schema::sagas;
use diesel::ExpressionMethods;

#[derive(Debug, QueryableByName, Deserialize)]
#[diesel(table_name = sagas)]
#[diesel(belongs_to(Country))]
#[diesel(belongs_to(Status))]
#[diesel(belongs_to(Category))]
pub struct Saga {
    pub id: u32,
    pub name: String,
    pub author: String,
    pub music: String,
    pub season: u16, //total of season
    pub creation_date: String,//Date,
    pub countryID: u32,//Id of Country,
    pub statusID: u32,//Id of Status
    pub categoryID: u32,//Id of category
    pub description: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}

#[derive(Debug, Deserialize, Clone, Insertable)]
#[diesel(table_name = sagas)]
pub struct NewSaga {
    pub name: String,
    pub author: String,
    pub music: String,
    pub countryID: u32,//Id of Country,
    pub categoryID: u32,//Id of category
    pub statusID: u32,//Id of Status
    pub creation_date: String,//Date,
    pub season: u16, //total of season
    pub description: String,
}

impl Default for NewSaga {
    fn default() -> Self {
        Self { 
            name: "".to_string(),
            author: "".to_string(),
            music: "".to_string(),
            countryID: 0,
            categoryID: 0,
            statusID: 0,
            creation_date: "".to_string(),
            season: 0,
            description: "".to_string(),
        }
    }
}

impl Saga {

    pub fn create<'a>(conn: &mut MysqlConnection, newSaga: NewSaga) -> u32 {
        use crate::schema::sagas::dsl::*;
        let saga_name = newSaga.name.clone();
        let record_inserted = insert_into(sagas).values::<NewSaga>(newSaga).execute(conn);

        match record_inserted {
            Ok(_)=>{
                match sagas.filter(name.eq(saga_name)).select(id).first::<u32>(conn){
                    Ok(info)=>{
                        info
                    }
                    Err(err)=>{
                        panic!("Error during insertion: {:#?}", err);
                    }
                }
            }
            Err(err)=>{
                panic!("Error during insertion: {:#?}", err);
            }
        }
    }
}