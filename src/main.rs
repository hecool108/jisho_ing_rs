use serde_json::Value;
use simple_xml_builder::XMLElement;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let args: Vec<String> = env::args().collect();
  let url = String::from(format!(
    "https://jisho.org/api/v1/search/words?keyword={}",
    args[1]
  ));
  let response = reqwest::get(&url).await?.text().await?;
  parse_result(&response);
  Ok(())
}

fn parse_result(data: &str) {
  let v: Value = serde_json::from_str(data).expect("json error");
  let mut xitems = XMLElement::new("items");
  for d in v["data"].as_array().unwrap() {
    let mut title = String::from("");
    for (rk, rv) in d["japanese"][0].as_object().unwrap() {
      if rk == "word" {
        title.push_str(&rv.as_str().unwrap());
      }
    }
    title.push_str("/");
    title.push_str(&d["japanese"][0]["reading"].as_str().unwrap());
    let mut subtitle = String::from("");
    for elem in d["senses"][0]["english_definitions"].as_array().unwrap() {
      subtitle.push_str(&elem.as_str().unwrap());
      subtitle.push_str("|");
    }
    let mut sitem = XMLElement::new("item");
    let mut xtitle = XMLElement::new("title");
    let mut xsubtitle = XMLElement::new("subtitle");
    xtitle.add_text(&title);
    xsubtitle.add_text(&subtitle);
    sitem.add_child(xtitle);
    sitem.add_child(xsubtitle);
    xitems.add_child(sitem);
  }
  println!("{}", xitems);
}
