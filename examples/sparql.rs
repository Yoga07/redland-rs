#[macro_use]
extern crate unwrap;

use redland_rs::{KvStorage, Model, Parser, Query, Uri};
use std::fs;
use std::fs::File;

fn main() {
    let file_path = "sample_code/turtle_code.ttl";
    let turtle_file = unwrap!(File::open(file_path));
    //Initialize Key-Value storage
    let storage = unwrap!(KvStorage::new());
    //Initialize Model
    let model = unwrap!(Model::new(&storage));
    //Variables for setting up parser
    let mime_type = "text/turtle";
    let base_uri = unwrap!(Uri::new("http://www.w3.org/2006/vcard/ns#"));
    //Initialize Parser
    let parser = unwrap!(Parser::new(None, Some(mime_type), None));
    //Parsing from file
    let _ = Parser::parse_from_file(parser, &turtle_file, Some(&base_uri), &model);

    let query_string = unwrap!(fs::read_to_string("sample_code/sparql_query.txt"));
    let query = unwrap!(Query::new("sparql", &query_string, Some(&base_uri.clone())));

    let query_result = unwrap!(query.execute(&model));
    println!(
        "{}",
        unwrap!(query_result.to_string(
            "turtle",
            Some("text/turtle"),
            None,
            Some(&base_uri.clone())
        ))
    );
}
