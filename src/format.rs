use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct L {
    pub languages: Vec<LANGUANGES>,

    pub tables: Vec<TABLES>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LANGUANGES {
    pub code: String,
    pub requireSeparateFont: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TABLES {
    pub tableName: String,
    pub entries: Vec<ENTRIES>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ENTRIES {
    pub key: String,
    pub values: Vec<VALUE>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VALUE {
    pub langCode: String,
    pub value: String,
}
