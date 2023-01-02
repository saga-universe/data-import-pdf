#[derive(Debug)]
pub struct Status {
    pub id: i16,
    pub name: String,
}

impl Status {
    pub fn new(id: i16, name: String) -> Self {
        Self {
            id: id,
            name: name,
        }
    }

    pub fn insert_status_in_array(
        mut list_of_status: Vec<Status>,
        value: &str,
    ) -> Vec<Status> {
        let status = list_of_status.iter().find(|status| status.name == value.to_string());

        match status {
            None => {
                list_of_status.push(Status::new(
                    (list_of_status.len()+1) as i16,
                    String::from(value),
                ));
                list_of_status
            }
            Some(_) => list_of_status,
        }
    }
}
