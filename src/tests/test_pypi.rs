use crate::repo::pypi::PyPiPackage;
use crate::repo::Package;

#[cfg(feature = "test_live")]
#[tokio::test]
async fn test_get_package() {
    use crate::repo::Repository;
    let pypi = crate::repo::pypi::PyPi::new();
    let res = pypi.get_package("requests").await.unwrap();
    println!("{:#?}", res);
    // assert!(false);
}

#[tokio::test]
async fn test_parse_package_mocked() {
    let pypipackage: PyPiPackage =
        serde_json::from_str(include_str!("data/pypi-requests.json")).unwrap();
    assert!(pypipackage.info.name == "requests");

    let package: Package = pypipackage.into();
    assert!(package.name == "requests");
}
