use crate::{args::StringArg, handlers::types::WordEntry};

pub fn handle_dict(word: StringArg) {
    if let Err(e) = lookup(&word.value) {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}

fn lookup(word: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!(
        "https://en.wiktionary.org/api/rest_v1/page/definition/{}",
        urlencoding::encode(word)
    );

    let client = reqwest::blocking::Client::new();

    let response = client
        .get(&url)
        .header(reqwest::header::USER_AGENT, "mavencore/0.1.0")
        .send()?;

    if response.status() == reqwest::StatusCode::NOT_FOUND {
        println!("No Wiktionary entry found for '{}'.", word);
        return Ok(());
    }

    response.error_for_status_ref()?;

    let data: serde_json::Value = response.json()?;

    let entries: Vec<WordEntry> = match data.get("en") {
        Some(value) => serde_json::from_value(value.clone())?,
        None => {
            println!("No English definition found for '{}'.", word);
            return Ok(());
        }
    };

    println!("<h3>{}</h3>", escape_html(word));

    for entry in entries {
        if !entry.part_or_speech.is_empty() {
            println!(
                "<hr><h5><b>{}</b></h5><ol type='a'>",
                escape_html(&entry.part_or_speech.to_uppercase())
            );
        }

        for definition in &entry.definitions {
            let definition_text = clean(&definition.definition);
            if definition_text.is_empty() {
                continue;
            }

            print!("<li>{}", escape_html(&definition_text));
            print!("<blockquote><ul>");
            for example in &definition.examples {
                let example_text = clean(&example.example);
                if !example_text.is_empty() {
                    print!("<li><i>{}</i></li>", escape_html(&example_text));
                }

                if let Some(translation) = &example.translation {
                    let translation_text = clean(translation);
                    if !translation_text.is_empty() {
                        print!("<br>{}", escape_html(&translation_text));
                    }
                }
            }
            println!("</blockquote></ul></li>");
        }
        println!("</ol>");

        if !entry.synonyms.is_empty() {
            let synonyms = entry
                .synonyms
                .iter()
                .map(|synonym| escape_html(synonym))
                .collect::<Vec<_>>()
                .join(", ");
            println!("<p><b>Synonyms:</b> {synonyms}</p>");
        }
    }
    println!("<hr>");

    Ok(())
}

fn clean(s: &str) -> String {
    let mut text = String::with_capacity(s.len());
    let mut chars = s.char_indices().peekable();
    let mut skip_tag: Option<String> = None;

    while let Some((_, c)) = chars.next() {
        if c == '<' {
            let mut tag = String::new();
            while let Some((_, next)) = chars.next() {
                if next == '>' {
                    break;
                }
                tag.push(next);
            }

            let tag_name = tag
                .trim_start_matches('/')
                .split(|c: char| c.is_whitespace() || c == '/')
                .next()
                .unwrap_or("")
                .to_ascii_lowercase();
            let closing = tag.starts_with('/');

            if closing {
                if skip_tag.as_deref() == Some(tag_name.as_str()) {
                    skip_tag = None;
                } else if tag_name == "li" {
                    text.push_str("; ");
                }
            } else if matches!(tag_name.as_str(), "style" | "script") {
                skip_tag = Some(tag_name);
            } else if tag_name == "li" {
                text.push_str("• ");
            }
            continue;
        }

        if skip_tag.is_some() {
            continue;
        }

        if c == '&' {
            let mut entity = String::new();
            let mut found_end = false;
            while let Some((_, next)) = chars.peek() {
                if *next == ';' {
                    chars.next();
                    found_end = true;
                    break;
                }
                if !next.is_ascii_alphanumeric() && *next != '#' {
                    break;
                }
                entity.push(*next);
                chars.next();
            }
            if found_end {
                if let Some(decoded) = decode_entity(&entity) {
                    text.push(decoded);
                }
            } else {
                text.push('&');
                text.push_str(&entity);
            }
        } else {
            text.push(c);
        }
    }

    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn decode_entity(entity: &str) -> Option<char> {
    match entity {
        "nbsp" => Some(' '),
        "amp" => Some('&'),
        "quot" => Some('"'),
        "apos" | "#39" => Some('\''),
        "lt" => Some('<'),
        "gt" => Some('>'),
        _ if entity.starts_with("#x") || entity.starts_with("#X") => {
            u32::from_str_radix(&entity[2..], 16)
                .ok()
                .and_then(char::from_u32)
        }
        _ if entity.starts_with('#') => entity[1..].parse::<u32>().ok().and_then(char::from_u32),
        _ => None,
    }
}

fn escape_html(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

#[cfg(test)]
mod tests {
    use super::clean;

    #[test]
    fn removes_markup_entities_and_style_content() {
        assert_eq!(
            clean("A&nbsp;<b>firm</b> apple<style>.hidden{display:none}</style> &amp; pear"),
            "A firm apple & pear"
        );
    }
}
