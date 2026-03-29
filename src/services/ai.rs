use reqwest::Client;
use serde_json::json;
use crate::models::AiAnalysis;

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

    pub async fn analyze(&self, context: &str) -> Result<AiAnalysis, String> {
        let prompt_template = std::env::var("AI_PROMPT")
            .expect("AI_PROMPT not found in environment");
        let prompt = prompt_template.replace("{}", context);

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

        let raw_text = json["candidates"][0]["content"]["parts"][0]["text"]
            .as_str()
            .unwrap_or("")
            .trim()
            .to_string();

        let clean = raw_text
            .trim_start_matches("```json")
            .trim_start_matches("```")
            .trim_end_matches("```")
            .trim()
            .to_string();

        let analysis: AiAnalysis = serde_json::from_str(&clean)
            .map_err(|e| format!("Error al parsear JSON de Gemini: {}. Respuesta: {}", e, clean))?;

        Ok(analysis)
    }
}