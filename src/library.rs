mod item;
mod parser;

#[derive(Debug)]
pub struct Library {
    data_path: String,
    items: Vec<item::Item>,
    liquids: Vec<item::Item>,
}

impl Library {
    pub fn new(data_path: &str) -> std::io::Result<Self> {
        let mut item_parser = parser::Parser::from_file(&format!("{data_path}/items.txt"))?;
        let mut items = Vec::new();
        let mut liquids = Vec::new();

        // parse items and liquids
        item_parser.expect_section("Items")?;
        for seq in item_parser.iter_seq([
            Some(parser::ParserTokenKind::Litteral),
            Some(parser::ParserTokenKind::Comma),
        ]) {
            let [item, _] = seq?;
            let name = match item {
                Some(parser::ParserToken::Litteral(name)) => name,
                _ => unreachable!(),
            };
            items.push(item::Item::new(name));
        }

        item_parser.expect_section("Liquids")?;
        for seq in item_parser.iter_seq([
            Some(parser::ParserTokenKind::Litteral),
            Some(parser::ParserTokenKind::Comma),
        ]) {
            let [liquid, _] = seq?;
            let name = match liquid {
                Some(parser::ParserToken::Litteral(name)) => name,
                _ => unreachable!(),
            };
            liquids.push(item::Item::new(name));
        }

        Ok(Self {
            data_path: data_path.to_string(),
            items,
            liquids,
        })
    }

    pub fn info(&self) -> String {
        let mut info = format!("Library loaded from {}", self.data_path);
        info.push_str(&format!("\nItems: {}", self.items.len()));
        info.push_str(&format!("\nLiquids: {}", self.liquids.len()));

        info
    }
}
