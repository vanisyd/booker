use serde::{Deserialize, Serialize, Serializer};
use serde::ser::SerializeStruct;

pub mod open_library;

#[derive(Deserialize)]
pub struct OpenLibrarySearchResponse {
    pub num_found: usize,
    pub doc: Vec<OpenLibraryDoc>
}

#[derive(Deserialize)]
pub struct OpenLibraryDoc {
    pub author_key: Vec<String>,
    pub author_name: Vec<String>,
    pub title: String,
    pub key: String
}

impl OpenLibraryDoc {
    /**
    * Key in response has format /works/{key}. Use this method to get the actual key
    */
    pub fn get_key(&self) -> Option<String> {
        let parts = self.key.rsplit_once('/');
        match parts {
            Some((_, key)) => Some(key.to_string()),
            _ => None
        }
    }
}

#[derive(Default)]
pub struct OpenLibrarySearchRequest {
    pub query: Option<String>,
    pub author: Option<String>,
    pub title: Option<String>
}

impl Serialize for OpenLibrarySearchRequest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        let mut state = serializer.serialize_struct("OpenLibrarySearchRequest", 2)?;
        state.serialize_field("q", &self.query)?;
        state.serialize_field("author", &self.author)?;
        state.end()
    }
}

