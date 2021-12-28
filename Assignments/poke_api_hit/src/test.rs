#[cfg(test)]
mod tests {
    use serde_json::Value::String;
    use std::collections::HashMap;
    use std::string::String as OtherString;
    #[test]
    fn url_success() {
        let url = "https://pokeapi.co/api/v2/pokemon-species/ditto";
        let res = reqwest::blocking::get(url).unwrap();
        assert_eq!(res.url().as_str(), url);
        assert_eq!(res.status(), reqwest::StatusCode::OK);
    }
    #[test]
    fn url_failure() {
        let url = "https://pokeapi.co/v2/pokemon-species/ditto";
        let res = reqwest::blocking::get(url).unwrap();
        assert_eq!(res.url().as_str(), url);
        assert_eq!(res.status(), reqwest::StatusCode::NOT_FOUND);
    }
    #[test]
    fn body_first_success() {
        let resp = String("purple".to_string());
        let content = reqwest::blocking::get("https://pokeapi.co/api/v2/pokemon-species/ditto")
            .unwrap()
            .json::<HashMap<OtherString, serde_json::Value>>()
            .unwrap();
        let resp_1 = &content["color"]["name"];
        assert_eq!(*resp_1, resp);
    }
    #[test]
    fn body_second_success() {
        let resp =  String(
            "Capable of copying\nan enemy's genetic\ncode to instantly\u{c}transform itself\ninto a duplicate\nof the enemy.".to_string(),
        );
        let content = reqwest::blocking::get("https://pokeapi.co/api/v2/pokemon-species/ditto")
            .unwrap()
            .json::<HashMap<OtherString, serde_json::Value>>()
            .unwrap();
        let resp_2 = &content["flavor_text_entries"][0]["flavor_text"];
        assert_eq!(*resp_2, resp);
    }
    #[test]
    fn body_third_success() {
        let resp = String("gold".to_string());
        let content = reqwest::blocking::get("https://pokeapi.co/api/v2/pokemon-species/ditto")
            .unwrap()
            .json::<HashMap<OtherString, serde_json::Value>>()
            .unwrap();
        let resp_3 = &content["flavor_text_entries"][3]["version"]["name"];
        assert_eq!(*resp_3, resp);
    }

    #[test]
    fn body_first_failure() {
        let content = reqwest::blocking::get("https://pokeapi.co/api/v2/pokemon-species/ditto")
            .unwrap()
            .json::<HashMap<OtherString, serde_json::Value>>()
            .unwrap();
        let resp_4 = &content["capture_rate"];
        assert_ne!(*resp_4, 70);
    }
    #[test]
    fn body_second_failure() {
        let content = reqwest::blocking::get("https://pokeapi.co/api/v2/pokemon-species/ditto")
            .unwrap()
            .json::<HashMap<OtherString, serde_json::Value>>()
            .unwrap();
        let resp_5 = &content["base_happiness"];
        assert_ne!(*resp_5, 35);
    }
}
