#[cfg(test)]
mod tests {
    use std::fs;
    use steamgriddb_dl::connectors::api_responses::*;
    use ureq::serde_json;

    #[test]
    fn deserialize_collection_info() {
        let json = fs::read_to_string("tests/resources/collection.json").unwrap();
        let deserialized = serde_json::from_str::<CollectionInfo>(&json);

        assert!(
            deserialized.is_ok(),
            "Error deserializing: {}",
            deserialized.unwrap_err()
        );
    }
    #[test]
    fn deserialize_collection_info_empty() {
        let json = fs::read_to_string("tests/resources/collection.empty.json").unwrap();
        let deserialized = serde_json::from_str::<CollectionInfo>(&json);

        assert!(
            deserialized.is_ok(),
            "Error deserializing: {}",
            deserialized.unwrap_err()
        );
    }
    #[test]
    fn deserialize_game_info() {
        let json = fs::read_to_string("tests/resources/game.json").unwrap();
        let deserialized = serde_json::from_str::<GameInfo>(&json);

        assert!(
            deserialized.is_ok(),
            "Error deserializing: {}",
            deserialized.unwrap_err()
        );
    }
}
