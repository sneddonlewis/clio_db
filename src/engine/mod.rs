#[derive(serde::Serialize, serde::Deserialize)]
pub struct Database {
    pub name: String,
    pub collections: Vec<Collection>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Collection {
    pub name: String,
}

pub fn get_databases() -> Vec<Database> {
    vec![Database {
        name: "db".to_string(),
        collections: vec![Collection {
            name: "clx".to_string(),
        }],
    }]
}
