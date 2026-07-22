use serde::{Deserialize, Deserializer, Serialize};

// Search API Response Structures
#[derive(Debug, Deserialize, Serialize)]
pub struct SearchChannel {
    pub title: String,
    pub link: String,
    pub description: String,
    #[serde(rename = "lastBuildDate")]
    pub last_build_date: String,
    pub total: u32,
    pub start: u32,
    pub num: u32,
    #[serde(rename = "item", default)]
    pub items: Vec<SearchItem>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchItem {
    pub target_code: u32,
    pub word: String,
    pub sup_no: u32,
    #[serde(default)]
    pub origin: Option<String>,
    #[serde(default)]
    pub pronunciation: Option<String>,
    #[serde(default)]
    pub word_grade: Option<String>,
    #[serde(default)]
    pub pos: Option<String>,
    pub link: String,
    #[serde(rename = "sense", default)]
    pub senses: Vec<Sense>,
    #[serde(default)]
    pub example: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Sense {
    pub sense_order: u32,
    pub definition: String,
    #[serde(rename = "translation", default)]
    pub translations: Vec<Translation>,
}

// Custom deserializer to trim whitespace from trans_lang
fn deserialize_trimmed_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(s.trim().to_string())
}



#[derive(Debug, Deserialize, Serialize)]
pub struct Translation {
    #[serde(deserialize_with = "deserialize_trimmed_string")]
    pub trans_lang: String,
    #[serde(default)]
    pub trans_word: Option<String>,
    #[serde(default)]
    pub trans_dfn: Option<String>,
}

// View API Response Structures
#[derive(Debug, Deserialize, Serialize)]
pub struct ViewChannel {
    pub title: String,
    pub link: String,
    pub description: String,
    #[serde(rename = "lastBuildDate")]
    pub last_build_date: String,
    pub total: u32,
    #[serde(rename = "item", default)]
    pub items: Vec<ViewItem>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ViewItem {
    pub target_code: u32,
    pub word_info: WordInfo,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WordInfo {
    pub word: String,
    pub sup_no: u32,
    pub word_unit: String,
    #[serde(default)]
    pub pos: Vec<String>,
    #[serde(default)]
    pub word_type: Option<String>,
    #[serde(default)]
    pub original_language_info: Option<OriginalLanguageInfo>,
    #[serde(default)]
    pub pronunciation_info: Option<PronunciationInfo>,
    #[serde(default)]
    pub conju_info: Option<ConjuInfo>,
    #[serde(default)]
    pub der_info: Vec<DerInfo>,
    #[serde(default)]
    pub ref_info: Vec<RefInfo>,
    #[serde(default)]
    pub word_grade: Option<String>,
    #[serde(default)]
    pub category_info: Vec<CategoryInfo>,
    #[serde(default)]
    pub sense_info: Vec<SenseInfo>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OriginalLanguageInfo {
    pub original_language: String,
    pub language_type: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PronunciationInfo {
    pub pronunciation: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConjuInfo {
    #[serde(default)]
    pub conjugation_info: Vec<ConjugationInfo>,
    #[serde(default)]
    pub abbreviation_info: Vec<AbbreviationInfo>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConjugationInfo {
    pub conjugation: String,
    #[serde(default)]
    pub pronunciation_info: Option<PronunciationInfo>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AbbreviationInfo {
    pub abbreviation: String,
    #[serde(default)]
    pub pronunciation_info: Option<PronunciationInfo>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DerInfo {
    pub word: String,
    pub link_type: String,
    #[serde(default)]
    pub link_target_code: Option<u32>,
    pub link: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RefInfo {
    pub word: String,
    pub link_type: String,
    #[serde(default)]
    pub link_target_code: Option<u32>,
    pub link: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CategoryInfo {
    #[serde(rename = "type")]
    pub category_type: String,
    pub written_form: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SenseInfo {
    pub definition: String,
    #[serde(default)]
    pub reference: Option<String>,
    #[serde(default)]
    pub pattern_info: Vec<PatternInfo>,
    #[serde(default)]
    pub example_info: Vec<ExampleInfo>,
    #[serde(default)]
    pub rel_info: Vec<RelInfo>,
    #[serde(default)]
    pub multimedia_info: Vec<MultimediaInfo>,
    #[serde(default)]
    pub subword_info: Vec<SubwordInfo>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PatternInfo {
    pub pattern: String,
    #[serde(default)]
    pub pattern_reference: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ExampleInfo {
    #[serde(rename = "type")]
    pub example_type: String,
    pub example: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RelInfo {
    pub word: String,
    #[serde(rename = "type")]
    pub rel_type: String,
    pub link_type: String,
    #[serde(default)]
    pub link_target_code: Option<u32>,
    pub link: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MultimediaInfo {
    pub label: String,
    #[serde(rename = "type")]
    pub media_type: String,
    pub link: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SubwordInfo {
    pub subword: String,
    pub subword_unit: String,
    #[serde(default)]
    pub reference: Option<String>,
    #[serde(default)]
    pub subsense_info: Vec<SubsenseInfo>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SubsenseInfo {
    pub definition: String,
    #[serde(default)]
    pub reference: Option<String>,
    #[serde(default)]
    pub pattern_info: Vec<PatternInfo>,
    #[serde(default)]
    pub example_info: Vec<ExampleInfo>,
    #[serde(default)]
    pub rel_info: Vec<RelInfo>,
}

// Error Response Structure
#[derive(Debug, Deserialize, Serialize)]
pub struct ApiError {
    pub error_code: String,
    pub message: String,
}