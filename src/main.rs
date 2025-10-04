use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: u32,
    name: String,
}

fn main() {
    let user = User { id: 1, name: "Rere".to_string() };
    let json = serde_json::to_string(&user).unwrap();
    println!("Serialized user: {}", json);

    let deserialized: User = serde_json::from_str(&json).unwrap();
    println!("Deserialized user: {:?}", deserialized);
}
