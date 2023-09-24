use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;

#[no_mangle]
pub fn translate(
    text: &str,
    from: &str,
    to: &str,
    _detect: &str,
    _needs: HashMap<String, String>,
) -> Result<Value, Box<dyn Error>> {
    let client = reqwest::blocking::ClientBuilder::new().build()?;

    const URL: &str = "https://translate.volcengine.com/crx/translate/v1";

    let mut body = HashMap::new();
    body.insert("source_language", from);
    body.insert("target_language", to);
    body.insert("text", text);

    let res: Value = client
        .post(URL)
        .header("Content-Type", "application/json")
        .json(&body)
        .send()?
        .json()?;

    fn parse_result(res: Value) -> Option<String> {
        println!("{res:?}");
        let result = res.as_object()?.get("translation")?.as_str()?.to_string();

        Some(result)
    }
    if let Some(result) = parse_result(res) {
        return Ok(Value::String(result));
    } else {
        return Err("Response Parse Error".into());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn try_request() {
        let needs = HashMap::new();
        let result = translate("你好", "detect", "en", "", needs).unwrap();
        println!("{result}");
    }
}
