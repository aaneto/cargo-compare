use std::collections::HashSet;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path of the first file to compare
    #[clap(short, long)]
    first_path: String,

    #[clap(short, long)]
    /// Path of the second file to compare
    second_path: String,
}

use serde_json::Value;

fn main() {
    let args: Args = Args::parse();
    let af = std::fs::read_to_string(args.first_path).unwrap();
    let bf = std::fs::read_to_string(args.second_path).unwrap();

    let ajson: Value = serde_json::from_str(&af).unwrap();
    let bjson: Value = serde_json::from_str(&bf).unwrap();

    let alines: HashSet<String> = to_serialized_str(ajson).into_iter().collect();
    let blines: HashSet<String> = to_serialized_str(bjson).into_iter().collect();

    println!("\nKeys only in A:");
    for ak in alines.difference(&blines) {
        println!("{}", ak);
    }

    println!("\nKeys only in B:");
    for bk in blines.difference(&alines) {
        println!("{}", bk);
    }
}

fn to_serialized_str(val: Value) -> Vec<String> {
    match val {
        Value::Array(arr) => {
            let mut out = vec![];
            for (i, val) in arr.iter().enumerate() {
                for v in to_serialized_str(val.clone()) {
                    out.push(format!("{}/{}", i, v));
                }
            }
            out
        },
        Value::Bool(bool) => vec![bool.to_string()],
        Value::Null => vec!["None".to_string()],
        Value::String(val) => vec![val],
        Value::Number(val) => vec![val.to_string()],
        Value::Object(mapper) => {
            let mut out = vec![];
            for (key, val) in mapper {
                for v2 in to_serialized_str(val.clone()) {
                    out.push(format!("{}={}", key, v2));
                }
            }

            out
        }
    }
}
