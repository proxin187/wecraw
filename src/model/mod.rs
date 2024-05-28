use serde::{Serialize, Deserialize};

use std::collections::HashMap;
use std::cmp::Ordering;
use std::fs;

const SYMBOLS: [char; 14] = [' ', ',', '.', '_', '-', '/', '{', '}', '"', '\'', ';', ':', '\n', '\\'];


#[derive(Serialize, Deserialize)]
pub struct Document {
    pub terms: HashMap<String, usize>,
    pub count: usize,
}

impl Document {
    pub fn new() -> Document {
        Document {
            terms: HashMap::new(),
            count: 0,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Model {
    pub docs: HashMap<String, Document>,
    pub df: HashMap<String, usize>,
}

impl Model {
    pub fn new() -> Model {
        Model {
            docs: HashMap::new(),
            df: HashMap::new(),
        }
    }

    pub fn write(&mut self, file: &str) -> Result<(), Box<dyn std::error::Error>> {
        let data = serde_json::to_vec(&self)?;

        fs::write(file, &data)?;

        Ok(())
    }

    pub fn search(&self, query: &str) -> Result<Vec<(String, f64)>, Box<dyn std::error::Error>> {
        let mut result: Vec<(String, f64)> = Vec::new();

        for (url, document) in &self.docs {
            let mut rank = 0.0;

            for token in query.split(SYMBOLS) {
                let tf = document.terms.get(token).cloned().unwrap_or(0) as f64 / document.count as f64;
                let idf = (self.docs.len() as f64 / self.df.get(token).cloned().unwrap_or(1) as f64).log10();

                rank += tf * idf;
            }

            if !rank.is_nan() {
                result.push((url.clone(), rank));
            }
        }

        result.sort_by(|(_, rank1), (_, rank2)| rank1.partial_cmp(rank2).unwrap_or(Ordering::Equal));
        result.reverse();

        Ok(result.iter().filter(|(_, rank)| *rank != 0.0).map(|x| x.clone()).collect::<Vec<(String, f64)>>())
    }

    pub fn insert_document(&mut self, url: String, content: String) {
        let mut document = Document::new();

        for token in content.split(SYMBOLS) {
            if let Some(freq) = document.terms.get_mut(token) {
                *freq += 1;
            } else {
                document.terms.insert(token.to_string(), 1);
            }

            document.count += 1;
        }

        for term in document.terms.keys() {
            if let Some(freq) = self.df.get_mut(term) {
                *freq += 1;
            } else {
                self.df.insert(term.clone(), 1);
            }
        }

        self.docs.insert(url, document);
    }
}


