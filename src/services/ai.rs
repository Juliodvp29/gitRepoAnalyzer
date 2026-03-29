use reqwest::Client;
use serde_json::json;

const GEMINI_URL: &str = "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash-lite:generateContent";

pub struct AiService {
    client: Client,
    api_key: String,
}

impl AiService {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
        }
    }

    pub async fn summarize(&self, context: &str) -> Result<String, String> {
        let prompt = format!(
            "Eres un asistente que analiza repositorios de código. \
             Basándote en esta información del repositorio, genera un resumen \
             conciso (máximo 3 oraciones) explicando qué hace el proyecto, \
             su estado y una recomendación:\n\n{}",
            context
        );

        let body = json!({
            "contents": [{
                "parts": [{
                    "text": prompt
                }]
            }]
        });

        let response = self.client
            .post(format!("{}?key={}", GEMINI_URL, self.api_key))
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("Error al llamar a Gemini: {}", e))?;

        let json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| format!("Error al parsear respuesta: {}", e))?;

        tracing::info!("Respuesta de Gemini: {}", json);

        let text = json["candidates"][0]["content"]["parts"][0]["text"]
            .as_str()
            .unwrap_or("No se pudo generar un resumen.")
            .to_string();

        Ok(text)
    }
}