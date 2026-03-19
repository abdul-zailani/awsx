use skim::prelude::*;
use std::io::Cursor;

pub fn pick(items: &[String], prompt: &str) -> Option<String> {
    if items.is_empty() {
        return None;
    }
    let input = items.join("\n");
    let options = SkimOptionsBuilder::default()
        .prompt(Some(prompt))
        .height(Some("40%"))
        .reverse(true)
        .build()
        .unwrap();
    let item_reader = SkimItemReader::default();
    let items = item_reader.of_bufread(Cursor::new(input));
    let output = Skim::run_with(&options, Some(items))?;
    if output.is_abort {
        return None;
    }
    output
        .selected_items
        .first()
        .map(|i| i.output().to_string())
}
