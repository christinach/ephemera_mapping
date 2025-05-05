
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Response {
    pub data: Vec<Item>,
}

#[derive(Deserialize, Debug)]
pub struct Item {
    pub id: String,
    pub attributes: Attributes,
}

#[derive(Deserialize, Debug)]
pub struct Attributes {
    #[serde(rename = "sort_title")]
    pub title_sort: String,
    // alternative -> other_title_display
    #[serde(rename = "transliterated_title")]
    pub other_title_display: Vec<String>,
    // creator -> author_display, author, author_s, author_sort, author_roles_1display, author_citation_display
    #[serde(rename = "creator")]
    creator: Vec<String>,
}

pub struct CatalogClient {
    pub url: String,
}

impl Default for CatalogClient {
    fn default() -> Self {
        Self::new()
    }
}

impl CatalogClient {
    pub fn new() -> Self {
        let figgy_url = std::env::var("FIGGY_URL");
        CatalogClient {
            url: figgy_url.unwrap()
        }
    }

    pub async fn get_data(&self) -> Result<Response, reqwest::Error> {
        let response = reqwest::get(&self.url).await?;
        let data: Response = response.json().await?;
        Ok(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_data() {
        std::env::set_var("FIGGY_URL", "https://figgy-staging.princeton.edu/catalog.json?f%5Bephemera_project_ssim%5D%5B%5D=Born+Digital+Monographs%2C+Serials%26Series+Reports&f%5Bhuman_readable_type_ssim%5D%5B%5D=Ephemera+Folder&f%5Bstate_ssim%5D%5B%5D=complete&per_page=100&q=");
        let client = CatalogClient::default();
        let result = client.get_data().await;
        assert!(result.is_ok());
    }
}