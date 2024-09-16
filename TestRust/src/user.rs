#[derive(Debug)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}

impl User {
    pub fn new(id: i32, name: String, email: String) -> User {
        User { id, name, email }
    }

    pub fn get_id(&self) -> i32 {
        self.id
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_email(&self) -> &String {
        &self.email
    }

    pub fn to_string(&self) -> String {
        format!(
            "id: {}, name: {}, email: {}",
            self.id, self.name, self.email
        )
    }
}
