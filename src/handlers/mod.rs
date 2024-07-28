pub struct User {
  pub id: u16,
  pub name: String,
}

impl User {
  pub fn to_json(&self) -> String {
    return format!("{{\"id\": {}, \"name\": \"{}\" }}", &self.id, &self.name);
  }
}

pub fn get_users() -> String {
  let list = vec![
    User {
      id: 1,
      name: "Caio".to_string(),
    }
    .to_json(),
    User {
      id: 2,
      name: "Camila".to_string(),
    }
    .to_json(),
  ];

  return format!("[{}]", list.join(","));
}
