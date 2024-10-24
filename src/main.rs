mod my_window;
mod imagens;
mod sheet;
mod definitions;

use serde_json::Value as JsonValue;

use definitions::SpottedStruct;

#[tokio::main]
async fn main() {
    // conectando com o google sheets e obtendo as entradas ainda nao postadas
    let mut nao_postados: JsonValue = JsonValue::Null; // algo assim
   
    match sheet::connect().await{
        Ok(hub) => {
            println!("A conexao com o google sheets foi bem sucedida");
            match sheet::get_not_posted(&hub).await {
                Ok(data) => {
                    nao_postados = data; // Atribua o valor retornado
                },
                Err(e) => println!("Erro ao obter postagens não postadas: {e}"),
            }
        },
        Err(e) => println!("Erro conectando com o google sheets: {e}")
    };
    
    // para cada entrada nao postada, cria um struct e envia para a funcao 
    // de criar janela
    if let JsonValue::Array(posts) = &nao_postados {
        for post in posts {
            let mut send_to_window = SpottedStruct::new();
            if let JsonValue::Object(map) = post {
                if let Some(JsonValue::String(id)) = map.get("id"){
                    send_to_window.id = id.clone();
                }
                if let Some(JsonValue::String(mensagem)) = map.get("mensagem"){
                    send_to_window.mensagem = mensagem.clone();
                }
                if let Some(JsonValue::String(identificacao)) = map.get("identificacao"){
                    send_to_window.identificacao = identificacao.clone();
                }
            }
        }
    }
}