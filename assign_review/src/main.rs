//use hubcaps::{Credentials, Github};
//use jsonpath_lib as jsonpath;
use std::error::Error;
use std::{fs, io};

fn main() -> Result<(), Box<dyn Error>> {
    let event_name = std::env::var("GITHUB_EVENT_NAME")?;
    let event_path = std::env::var("GITHUB_EVENT_PATH")?;

    println!("handling {} event", event_name);
    println!("reading event data from {}", event_path);

    let file = fs::File::open(event_path)?;
    let reader = io::BufReader::new(file);

    println!("opened file");

    let data = serde_json::from_reader(reader);

    println!("{:#?}", data);

    Ok(())
}
