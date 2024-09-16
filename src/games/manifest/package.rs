use serde::{Serialize, Deserialize};
use serde_json::{json, Value as Json};

use crate::core::prelude::*;
use crate::packages::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Package {
    pub url: String
}

impl AsJson for Package {
    fn to_json(&self) -> Result<Json, AsJsonError> {
        Ok(json!({
            "url": self.url
        }))
    }

    fn from_json(json: &Json) -> Result<Self, AsJsonError> where Self: Sized {
        Ok(Self {
            url: json.get("url")
                .ok_or_else(|| AsJsonError::FieldNotFound("package.url"))?
                .as_str()
                .ok_or_else(|| AsJsonError::InvalidFieldValue("package.url"))?
                .to_string()
        })
    }
}

impl AsHash for Package {
    #[inline]
    fn hash(&self) -> Hash {
        self.url.hash()
    }
}
