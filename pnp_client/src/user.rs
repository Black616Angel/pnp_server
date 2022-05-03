
pub struct User {
    name: String,
    uid: String,
    session_id: String,
}

impl User{
    pub fn new(name: String, uid: String, session_id: String) -> Self {
        Self{name, uid, session_id}
    }
}