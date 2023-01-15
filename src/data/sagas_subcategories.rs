use crate::schema::sagas_subcategories;
use diesel::RunQueryDsl;
use diesel::{insert_into, sql_query, Insertable, MysqlConnection};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, Insertable)]
#[diesel(table_name = sagas_subcategories)]
#[diesel(belongs_to(Saga))]
#[diesel(belongs_to(FullSubcategory))]
pub struct SagasSubcategories {
    pub sagaID: u32,
    pub subcategoryID: u32,
}

impl SagasSubcategories {
    pub fn create(conn: &mut MysqlConnection, records: SagasSubcategories) {
        use crate::schema::sagas_subcategories::dsl::*;

        let record_inserted = insert_into(sagas_subcategories)
            .values::<SagasSubcategories>(records)
            .execute(conn);

        match record_inserted {
            Ok(_) => {}
            Err(err) => {
                panic!("Error during insertion: {:#?}", err)
            }
        }
    }
}
