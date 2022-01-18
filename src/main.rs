use anyhow::Result;
use serde::{Serialize, Deserialize};
use std::fs::File;
use csv::Reader;
use serde_json;
use std::io::{BufWriter};

#[derive(Deserialize, Debug)]
struct Record {
    id: String,
    trait_animal: String,
    trait_name: String,
    trait_type: String,
    description: String,
    image: String,
    name: String,
    trait_name2: String
}

#[derive(Serialize, Debug)]
struct Trait {
    trait_type: String,
    value: String
}

#[derive(Serialize, Debug)]
struct Metadata {
    attributes: Vec<Trait>,
    description: String,
    image: String,
    name: String
}

fn save(file_name: String, metadata: Metadata) -> Result<()> {
    /* let metadata = serde_json::to_string_pretty(&metadata)?;
    println!("{}", metadata); */

    let mut writer = BufWriter::new(File::create(format!("metadata/items/{}", file_name))?);
    serde_json::to_writer_pretty(&mut writer, &metadata)?;

    Ok(())
}

fn run() -> Result<()> {
    let file_path = format!("{}/items.csv", env!("CARGO_MANIFEST_DIR"));
    let file = File::open(file_path)?;

    let mut reader = Reader::from_reader(file);

    for record in reader.deserialize() {
        let record: Record = record?;

        let traits: Vec<Trait> = vec![
            Trait {
                trait_type: "ID".to_string(),
                value: record.id.clone()
            },
            Trait {
                trait_type: "Item".to_string(),
                value: record.trait_animal
            },
            Trait {
                trait_type: "Name".to_string(),
                value: record.trait_name
            },
            Trait {
                trait_type: "Type".to_string(),
                value: record.trait_type
            }
        ];
    
        let metadata: Metadata = Metadata {
            attributes: traits,
            description: record.description,
            image: record.image,
            name: record.name
        };

        let id: String = record.id.parse::<i32>().unwrap().to_string();
        save(format!("{}.json", id), metadata)?;
    }

    Ok(())
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
    }
}
