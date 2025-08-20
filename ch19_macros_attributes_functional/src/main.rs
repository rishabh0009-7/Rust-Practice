use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]  // custom derive macros 
struct User {
    name: String,
    age: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    #[serde(rename = "full_name")]  // attribute maros 
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]  // attribute maros 
    email: Option<String>,
    #[serde(skip)] // attribute maros 
    password: String,
}

#[derive(Serialize, Deserialize, Debug)]
enum Status {
    Active,
    Inactive,
    Pending,
}

fn main() -> Result<(), serde_json::Error> {
    // ----- User Example -----
    let user = User {
        name: "Alice".to_string(),
        age: 30,
    };
    let json = serde_json::to_string(&user)?;
    println!("JSON: {}", json);
    let back: User = serde_json::from_str(&json)?;
    println!("Back to struct: {:?}", back);

    // ----- Person Example (email = None) -----
    let person = Person {
        name: "Bob".to_string(),
        email: None,
        password: "secret".to_string(),
    };
    let json = serde_json::to_string(&person)?;
    println!("Person JSON (email = None): {}", json);

    // ----- Person Example (email = Some) -----
    let person_with_email = Person {
        name: "Charlie".to_string(),
        email: Some("charlie@example.com".to_string()),
        password: "hidden".to_string(),
    };
    let json = serde_json::to_string(&person_with_email)?;
    println!("Person JSON (with email): {}", json);

    // ----- Status Enum Example -----
    let status = Status::Active;
    let status_json = serde_json::to_string(&status)?;
    println!("Status: {}", status_json);

    Ok(())
}
