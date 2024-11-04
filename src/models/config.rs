use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct BookDetails {
    book_title: String,
    links: Links,
    identifiers: Identifiers,
}

#[derive(Deserialize, Debug)]
struct Links {
    source_url: String,
}

#[derive(Deserialize, Debug)]
struct Identifiers {
    next_chapter: ElementSelector,
    content: ElementSelector,
    title: ElementSelector,
}

#[derive(Deserialize, Debug)]
struct ElementSelector {
    tag: String,
    #[serde(rename = "type")]
    identifier_type: HtmlIdentifier,
    name: String,
    attribute: Option<String>,
}

#[derive(Deserialize, Debug)]
enum HtmlIdentifier {
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
