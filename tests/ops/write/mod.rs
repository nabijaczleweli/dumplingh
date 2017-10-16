use serde::ser::{Serializer, Serialize, Error as SerError};

mod save_data;
mod save_to_file;


struct Unserialisable;

impl Serialize for Unserialisable {
    fn serialize<S: Serializer>(&self, _: S) -> Result<S::Ok, S::Error> {
        Err(S::Error::custom("unserialisable"))
    }
}
