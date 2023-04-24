use crate::codegen::misc::VariableScope;

use super::item_data::ItemData;

#[derive(Debug, Clone)]
pub struct Item {
    pub id: String,
    pub slot: i32,
    pub item: ItemData,
}

/*
"item": {
              "id": "txt",
              "data": {
                "name": "How is everyone?"
              }
            },
            "slot": 1
 */
impl Item {
    pub fn to_json(self) -> String {
        format!(
            r#"{{"item":{{"id":"{}",{}}},"slot":{}}}"#,
            self.id,
            self.item.to_json(),
            self.slot
        )
    }
    pub fn from_strs(from: String, full_line: String, line_number: i32, raw_file: String) -> Vec<Self> {
        let mut out = Vec::new();

        let mut in_string = false;
        let mut builder = String::new();
        let mut split: Vec<String> = Vec::new();
        for ch in from.chars() {
            if ch == '"' {
                in_string = !in_string;
                builder.push(ch);
            } else if ch == ',' && !in_string {
                split.push(builder.trim().to_string());
                builder = String::new();
            } else {
                builder.push(ch);
            }
        }
        split.push(builder.trim().to_string());

        let mut slot = 0;
        for arg in split {
            out.push(Item::from_str(arg.to_string(), slot, full_line.clone(), line_number, raw_file.clone()));
            slot += 1;
        }
        out
    }
    pub fn from_str(from: String, slot: i32, full_line: String, line_number: i32, raw_file: String) -> Self {
        if from.starts_with("\"") && from.ends_with("\"") {
            let from = from.replace("\"", "");
            return Item {
                id: "txt".to_string(),
                slot,
                item: ItemData::Text { data: from },
            };
        }
        if let Ok(v) = from.parse::<f32>() {
            return Item {
                id: "num".to_string(),
                slot,
                item: ItemData::Number { data: v },
            };
        }
        if from.starts_with("vanilla_item![") && from.ends_with("]") {
            let from = from
                .replace("vanilla_item![", "")
                .replace("]", "");
            let split = from.split(':').collect::<Vec<_>>();
            if split.len() < 2 {
                // errors::throw_error("not enough arguments in `vanilla_item!` statement", &full_line, &raw_file, line_number)
            }
            let id = split.get(0).expect("failed to get id");
            let count = split.get(1).expect("failed to get count");
            let datas = format!(r#"{{Count:{count}b,DF_NBT:3120,id:\"minecraft:{id}\"}}"#);
            return Item {
                id: "item".to_string(),
                slot,
                item: ItemData::VanillaItem { data: datas }
            }
        }
        println!("bad item from string {from} to item");
        return Item {
            id: "var".to_string(),
            slot,
            item: ItemData::Variable {
                scope: VariableScope::Unsaved,
                name: from,
            },
        };
    }
}