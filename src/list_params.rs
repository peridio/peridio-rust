use serde::Serialize;

#[derive(Debug, Serialize, Default)]
pub struct ListParams {
    pub limit: Option<u8>,
    pub order: Option<String>,
    pub search: Option<String>,
    pub page: Option<String>,
}

impl ListParams {
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut query_params = Vec::new();
        if let Some(limit) = self.limit {
            query_params.push(("limit".to_string(), limit.to_string()));
        }
        if let Some(ref order) = self.order {
            query_params.push(("order".to_string(), order.to_string()));
        }
        if let Some(ref search) = self.search {
            query_params.push(("search".to_string(), search.to_string()));
        }
        if let Some(ref page) = self.page {
            query_params.push(("page".to_string(), page.to_string()));
        }
        query_params
    }
}
