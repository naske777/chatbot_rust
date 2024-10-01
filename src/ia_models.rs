use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use strsim::levenshtein;

#[derive(Debug, Serialize, Deserialize)]
pub struct PatternMatchingChatbot {
    knowledge_base: HashMap<String, Vec<String>>, // Mapa de etiquetas a respuestas
    pattern_tag_pairs: Vec<(String, String)>, // (patrón, etiqueta)
}

#[derive(Debug, Serialize, Deserialize)]
struct QuestionAnswer {
    tag: String,
    patterns: Vec<String>,
    responses: Vec<String>,
}

impl PatternMatchingChatbot {
    // Constructor para inicializar el chatbot
    pub fn new() -> Self {
        PatternMatchingChatbot {
            knowledge_base: HashMap::new(),
            pattern_tag_pairs: Vec::new(),
        }
    }

    // Carga datos de entrenamiento desde un archivo JSON
    pub fn load_training_data(&mut self, file_path: &str) {
        let file = File::open(file_path).expect("No se pudo abrir el archivo");
        let reader = BufReader::new(file);
        let qas: Vec<QuestionAnswer> = serde_json::from_reader(reader).expect("No se pudo deserializar el archivo");

        // Poblar la base de conocimiento y los pares de patrones y etiquetas
        for qa in qas {
            for pattern in qa.patterns {
                self.pattern_tag_pairs.push((pattern, qa.tag.clone()));
            }
            self.knowledge_base.insert(qa.tag, qa.responses);
        }
    }

    // Encuentra la mejor respuesta para una pregunta dada
    pub fn find_best_response(&self, question: &str) -> Option<String> {
        let mut best_match = None;
        let mut best_score = usize::MAX;

        // Encuentra el patrón más cercano utilizando la distancia de Levenshtein
        for (pattern, tag) in &self.pattern_tag_pairs {
            let score = levenshtein(question, pattern);
            if score < best_score {
                best_score = score;
                best_match = Some(tag);
            }
        }

        // Si se encuentra una coincidencia, devuelve una respuesta aleatoria asociada a la etiqueta
        if let Some(best_tag) = best_match {
            if let Some(responses) = self.knowledge_base.get(best_tag) {
                let mut rng = rand::thread_rng();
                let response_index = rng.gen_range(0..responses.len());
                return responses.get(response_index).cloned();
            }
        }

        None
    }
}