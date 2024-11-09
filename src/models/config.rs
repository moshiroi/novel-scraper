use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct BookDetails {
    pub book_title: String,
    pub links: Links,
    pub identifiers: Identifiers,
}

#[derive(Deserialize, Debug)]
pub struct Links {
    pub source_url: String,
}

#[derive(Deserialize, Debug)]
pub struct Identifiers {
    pub next_chapter: ElementSelector,
    pub content: ElementSelector,
    pub title: ElementSelector,
}

#[derive(Deserialize, Debug)]
pub struct ElementSelector {
    pub tag: String,
    #[serde(rename = "type")]
    pub identifier_type: HtmlIdentifier,
    pub name: String,
    pub attribute: Option<String>,
}

#[derive(Deserialize, Debug)]
pub enum HtmlIdentifier {
    #[serde(rename = "id")]
    Id,
    #[serde(rename = "class_")]
    Class,
}

impl AsRef<str> for HtmlIdentifier {
    fn as_ref(&self) -> &str {
        match self {
            HtmlIdentifier::Id => "id",
            HtmlIdentifier::Class => "class",
        }
    }
}
