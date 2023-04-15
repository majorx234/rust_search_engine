// use search_engine::lexer::Lexer;
use crate::lexer::Lexer;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufWriter};
use std::path::{Path, PathBuf};

pub type DocFreq = HashMap<String, u32>;
pub type TermFreq = HashMap<String, u32>;
pub type TermFreqPerDoc = HashMap<PathBuf, (usize, TermFreq)>;

#[derive(Default, Deserialize, Serialize)]
pub struct TermIndex {
    pub term_freq_per_doc: TermFreqPerDoc,
    pub doc_freq: DocFreq,
}

impl TermIndex {
    fn new() -> Self {
        TermIndex {
            term_freq_per_doc: HashMap::new(),
            doc_freq: HashMap::new(),
        }
    }
    fn save_index_as_json(&self, json_file_path: &PathBuf) -> io::Result<()> {
        // saving index to json
        println!("Saving {}", json_file_path.to_str().unwrap());
        let index_file = File::create(json_file_path)?;
        serde_json::to_writer(BufWriter::new(index_file), &self).expect("serde works");
        Ok(())
    }
    fn compute_tf(term: &str, n: usize, doc_tf: &TermFreq) -> f32 {
        let n = n as f32;
        let m = doc_tf.get(term).cloned().unwrap_or(0) as f32;
        m / n
    }

    fn compute_idf(term: &str, n: usize, docs_tf: &DocFreq) -> f32 {
        let n = n as f32;
        let m = docs_tf.get(term).cloned().unwrap_or(1) as f32;
        (n / m).log10()
    }

    pub fn search_query(&self, query: &[char]) -> Result<Vec<(&Path, f32)>, ()> {
        let mut result = Vec::<(&Path, f32)>::new();
        let tokens = Lexer::new(query).collect::<Vec<_>>();
        for (path, (num_terms, tf_table)) in &self.term_freq_per_doc {
            let mut rank = 0f32;
            for term in &tokens {
                let tf_rank = Self::compute_tf(term, *num_terms, tf_table);
                let idf_rank =
                    Self::compute_idf(term, self.term_freq_per_doc.len(), &self.doc_freq);
                rank += tf_rank * idf_rank;
            }
            result.push((path, rank));
        }
        result.sort_by(|(_, rank1), (_, rank2)| rank1.partial_cmp(rank2).unwrap());
        result.reverse();
        Ok(result)
    }
}
