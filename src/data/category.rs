#[derive(Debug)]
pub struct Category {
    pub id: i16,
    pub name: Option<String>,
    pub description: Option<String>,
}

impl Default for Category {
    fn default() -> Self {
        Self {
            id: 0,
            name: None,
            description: None,
        }
    }
}

impl Category {
    pub fn new(id: i16, name: String) -> Self {
        Self {
            id: id,
            name: Some(name),
            description: None,
        }
    }

    pub fn insert_category_in_array(
        mut list_of_category: Vec<Category>,
        value: &str,
    ) -> Vec<Category> {
        let category = list_of_category.iter().find(|cat| cat.name == Some(value.to_string()));

        match category {
            None => {
                list_of_category.push(Category::new(
                    (list_of_category.len()+1) as i16,
                    String::from(value),
                ));
                list_of_category
            }
            Some(_) => list_of_category,
        }
    }
}
