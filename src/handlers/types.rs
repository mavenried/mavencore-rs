use serde::{Deserialize, Serialize};

#[derive(Serialize, Default)]
pub struct State {
    pub workspaces: Vec<usize>,
    pub workspace_id: usize,
    pub window_name: String,
}

#[derive(Serialize)]
pub struct DesktopEntry {
    pub name: String,
    pub icon: Option<String>,
    pub path: String,
}

#[derive(Debug, Deserialize)]
pub struct Definition {
    pub definition: String,

    // `examples` in the API is a list of HTML strings; parsedExamples has
    // the structured objects this renderer expects.
    #[serde(default, rename = "parsedExamples")]
    pub examples: Vec<Example>,
}

#[derive(Debug, Deserialize)]
pub struct Example {
    pub example: String,

    #[serde(default)]
    pub translation: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct WordEntry {
    #[serde(default, rename = "partOfSpeech", alias = "part_of_speech")]
    pub part_or_speech: String,

    #[serde(default)]
    pub definitions: Vec<Definition>,

    #[serde(default)]
    pub synonyms: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::WordEntry;

    #[test]
    fn parses_wiktionary_definition_response() {
        let response = serde_json::json!({
            "en": [{
                "language": "English",
                "partOfSpeech": "Noun",
                "definitions": [{
                    "definition": "A sample definition.",
                    "parsedExamples": [{
                        "example": "A sample sentence.",
                        "translation": null
                    }]
                }]
            }]
        });

        let entries: Vec<WordEntry> =
            serde_json::from_value(response["en"].clone()).unwrap();

        assert_eq!(entries[0].part_or_speech, "Noun");
        assert_eq!(entries[0].definitions[0].definition, "A sample definition.");
        assert_eq!(
            entries[0].definitions[0].examples[0].example,
            "A sample sentence."
        );
    }
}
