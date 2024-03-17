use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub pub_name: String,
    pub parent_pub_name: String,
    pub booknum: Value,
    #[serde(rename = "pub")]
    pub pub_field: String,
    pub issue: String,
    pub formatted_date: String,
    pub fileformat: Vec<String>,
    pub track: Value,
    pub specialty: String,
    pub pub_image: PubImage,
    #[serde(skip)]
    pub languages: serde_json::Value,
    
    pub files: serde_json::Map<String, serde_json::Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PubImage {
    pub url: String,
    pub modified_datetime: String,
    pub checksum: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Files {
    #[serde(rename = "MP3")]
    pub mp3: Vec<Mp3>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mp3 {
    pub title: String,
    pub file: File,
    pub filesize: i64,
    pub track_image: TrackImage,
    pub markers: Option<Markers>,
    pub label: String,
    pub track: i64,
    pub has_track: bool,
    #[serde(rename = "pub")]
    pub pub_field: String,
    pub docid: i64,
    pub booknum: i64,
    pub mimetype: String,
    pub edition: String,
    pub edition_descr: String,
    pub format: String,
    pub format_descr: String,
    pub specialty: String,
    pub specialty_descr: String,
    pub subtitled: bool,
    pub frame_width: i64,
    pub frame_height: i64,
    pub frame_rate: i64,
    pub duration: f64,
    pub bit_rate: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct File {
    pub url: String,
    pub stream: String,
    pub modified_datetime: String,
    pub checksum: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackImage {
    pub url: String,
    pub modified_datetime: String,
    pub checksum: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Markers {
    pub meps_language_spoken: String,
    pub meps_language_written: String,
    pub document_id: i64,
    pub markers: Vec<Marker>,
    #[serde(rename = "type")]
    pub type_field: String,
    pub hash: String,
    pub introduction: Option<Introduction>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Marker {
    pub duration: String,
    pub start_time: String,
    pub meps_paragraph_id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Introduction {
    pub duration: String,
    pub start_time: String,
}
