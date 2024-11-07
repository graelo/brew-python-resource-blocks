use reqwest::blocking::Client;
use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct PackageInfo {
    pub(crate) urls: Vec<UrlInfo>,
}

#[derive(Deserialize)]
pub(crate) struct UrlInfo {
    pub(crate) url: String,
    pub(crate) digests: Digests,
}

#[derive(Deserialize)]
pub(crate) struct Digests {
    pub(crate) sha256: String,
}

pub(crate) fn fetch_package_info(
    pkg_name: &str,
    version: &str,
) -> Result<PackageInfo, reqwest::Error> {
    let pypi_base_url = "https://pypi.org/pypi";
    let pkg_info_url = format!("{}/{}/{}/json", pypi_base_url, pkg_name, version);
    let client = Client::new();
    let response = client.get(&pkg_info_url).send()?;
    response.json::<PackageInfo>()
}
