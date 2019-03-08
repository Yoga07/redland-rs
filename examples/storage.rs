extern crate bincode;
extern crate libc;
extern crate redland_rs;
#[macro_use]
extern crate unwrap;

use libc::c_char;
use redland_rs::librdf_new_serializer;
use redland_rs::{EntryAction, KvStorage, Model, Node, Serializer, Uri, World};
use std::fs::File;
use std::io::prelude::*;
use std::ptr;

use bincode::{deserialize, serialize};

fn create_mock_model(world: &World, storage: &KvStorage) -> Result<Model, i32> {
    let ms_schema = Uri::new(world, "http://maidsafe.net/")?;
    let subject = Node::new_from_uri_local_name(world, &ms_schema, "MaidSafe")?;
    let predicate = Node::new_from_uri_local_name(world, &ms_schema, "location")?;
    let model = Model::new(world, storage)?;
    model.add_string_literal_statement(&subject, &predicate, "Ayr", None, false)?;
    Ok(model)
}

fn main() {
    let world = World::new();
    let mut storage = unwrap!(KvStorage::new(&world));

    // Convert entries into a hash
    {
        // Create mock entries and write to a file
        let _model = unwrap!(create_mock_model(&world, &storage));
        let entry_actions = storage.entry_actions();
        println!("{:?}", entry_actions);

        let ser = unwrap!(serialize(entry_actions));

        {
            let mut file = unwrap!(File::create("md-storage"));
            file.write_all(&ser).unwrap();
        }
    }

    // Load entries from a file
    let mut entry_actions: Vec<EntryAction> = {
        let mut file = unwrap!(File::open("md-storage"));
        let mut contents = Vec::new();
        unwrap!(file.read_to_end(&mut contents));

        unwrap!(deserialize(&contents))
    };
    println!("{:?}", entry_actions);

    unwrap!(storage.copy_entries(&mut entry_actions));

    let model = unwrap!(Model::new(&world, &storage));

    // Serialise to string - Turtle
    let serializer = Serializer(unsafe {
        librdf_new_serializer(
            world.as_mut_ptr(),
            b"turtle\0" as *const _ as *const c_char,
            ptr::null(),
            ptr::null_mut(),
        )
    });
    let ms_schema = unwrap!(Uri::new(&world, "http://maidsafe.net/"));
    unwrap!(serializer.set_namespace(&ms_schema, "ms"));

    println!("{}", unwrap!(serializer.serialize_model_to_string(&model)));
}
