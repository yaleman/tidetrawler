use crate::repo::npm::NpmSearchResponse;

#[test]
fn test_npm_search_parse() {
    let response: NpmSearchResponse =
        serde_json::from_str(include_str!("data/npm-search-api.json")).unwrap();

    assert!(response.csrftoken == "HzK2YfrBNUkVR6r4M6h0clCwOmcCSnGDU_YanLgOvRT".to_string())
}
