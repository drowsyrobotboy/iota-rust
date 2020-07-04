use std::io::{BufRead, BufReader};
use subprocess::Exec;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Stat {
    metric: String,
    value: String,
}

#[get("/status")]
pub fn info() -> String {
    format!("{}", get_info())
}

fn get_info()->String{
    let br = BufReader::new(Exec::shell("free | grep Mem | awk '{print $4/$2 * 100.0}'").stream_stdout().unwrap());
    let mut output = String::new();
    for (_i, line) in br.lines().enumerate() {
        output =line.unwrap();
    }
    let mem = Stat { metric: "freeMemory".to_string(), value: output };
    // Convert the Point to a JSON string.
    let serialized = serde_json::to_string(&mem).unwrap();
    // Prints serialized = {"x":1,"y":2}
    format!("serialized = {}", serialized)

    //// Convert the JSON string back to a Point.
    //let deserialized: Point = serde_json::from_str(&serialized).unwrap();
    //// Prints deserialized = Point { x: 1, y: 2 }
    //format!("deserialized = {:?}", deserialized)
}