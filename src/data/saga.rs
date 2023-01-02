
#[derive(Debug)]
pub struct Saga {
    pub name: Option<String>,
    pub author: Option<String>,
    pub music: Option<String>,
    pub saga_type: Option<i16>,
    pub country_of_origin: Option<i16>,//Id of Country,
    pub category: Option<i16>,//Id of categor>y
    pub status: Option<i16>,//Id of Status
    pub creation_date: Option<String>,//Date>,
    pub season: Option<i16>, //total of season
    pub description: Option<String>,
}

impl Default for Saga {
    fn default() -> Self {
        Self { 
            name: None,
            author: None,
            music: None,
            saga_type: None,
            country_of_origin: None,
            category: None,
            status: None,
            creation_date: None,
            season: None,
            description: None,
        }
    }
}