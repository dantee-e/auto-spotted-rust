extern crate google_sheets4 as sheets4;
use google_sheets4::common::Client;
use sheets4::{Sheets, yup_oauth2, hyper_util, hyper_rustls};
use hyper_rustls::HttpsConnector;
use crate::sheet::hyper_util::client::legacy::connect::HttpConnector;
use serde_json::Value as JsonValue;

const CREDENTIALS_PATH: &str = "assets/cred/credentials.json";




pub async fn get_not_posted(hub: &Sheets<HttpsConnector<HttpConnector>>) -> Result<JsonValue, Box<dyn std::error::Error>> {
    fn empty_to_null(value: Option<&JsonValue>) -> JsonValue {
        match value {
            Some(JsonValue::String(s)) if s.is_empty() => JsonValue::Null,
            Some(v) => v.clone(),
            None => JsonValue::Null
        }
    }
    fn check_postado_banido(wor:&Vec<JsonValue>) -> bool {
        let postado = wor.get(4).unwrap_or(&JsonValue::Null);
        let banido = wor.get(5).unwrap_or(&JsonValue::Null);
        match (postado, banido) {
            (JsonValue::Null, JsonValue::Null) => true,
            _ => false
        }
    }

    let spreadsheet_id = "1EqixBaOLJsqUMSijDXMNZjJbQfvufi36N-qKdGUcJa8";
    let range = "A:F";

    let result = hub
        .spreadsheets()
        .values_get(&spreadsheet_id, range)
        .doit()
        .await;

    let rows = match result {
        Ok((_, value_range)) => value_range.values.unwrap_or_default(),
        Err(e) => {
            println!("Erro ao obter os valores: {:?}", e);
            return Ok(JsonValue::Array(vec![])); // Retorna um array vazio em caso de erro
        }
    };

    let mut json_rows = Vec::new();
    let mut index = 1;

    for row in rows {
        index+=1;
        if !check_postado_banido(&row){
            continue;
        }
        let mut json_row = serde_json::Map::new();
        json_row.insert("index".to_string(), JsonValue::String(index.clone().to_string()));
        json_row.insert("data/hora".to_string(), row.get(0).unwrap_or(&JsonValue::Null).clone());
        json_row.insert("mensagem".to_string(), row.get(1).unwrap_or(&JsonValue::Null).clone());
        json_row.insert("identificacao".to_string(), row.get(2).unwrap_or(&JsonValue::Null).clone());
        json_row.insert("link".to_string(), row.get(3).unwrap_or(&JsonValue::Null).clone());
        json_row.insert("postado".to_string(), empty_to_null(row.get(4)));
        json_row.insert("banido".to_string(), empty_to_null(row.get(5)));

        json_rows.push(JsonValue::Object(json_row));
    }
    let valor_retorno = JsonValue::Array(json_rows);

    
    Ok(valor_retorno)
}

pub async fn connect() -> Result<Sheets<HttpsConnector<HttpConnector>>, Box<dyn std::error::Error>> {
    let service_account_key = yup_oauth2::read_service_account_key(CREDENTIALS_PATH)
            .await
            .expect("Failed to read service account key file");
    // Cria o autenticador
    let auth = yup_oauth2::ServiceAccountAuthenticator::builder(service_account_key)
        .build()
        .await
        .expect("Failed to create the authenticator");

    let https = hyper_util::rt::TokioExecutor::new();
    let client: Client<HttpsConnector<HttpConnector>> = hyper_util::client::legacy::Client::builder(https)
        .build(
            hyper_rustls::HttpsConnectorBuilder::new()
            .with_native_roots()
            .unwrap()
            .https_or_http()
            .enable_http1()
            .build()
        );

    let hub = Sheets::new(client, auth);

    Ok(hub)
}