#[derive(Debug)]
pub struct Saga_Style {
    pub id: i16,
    pub name: String,
}

impl Saga_Style {
    pub fn new(id: i16, name: String) -> Self {
        Self {
            id: id,
            name: name,
        }
    }

    pub fn insert_style_in_array(
        mut list_of_style: Vec<Saga_Style>,
        value: &str,
    ) -> Vec<Saga_Style> {
        let style = list_of_style.iter().find(|cat| cat.name == value.to_string());

        match style {
            None => {
                list_of_style.push(Saga_Style::new(
                    (list_of_style.len()+1) as i16,
                    String::from(value),
                ));
                list_of_style
            }
            Some(_) => list_of_style,
        }
    }
}
