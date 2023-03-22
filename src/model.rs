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
    fn compute_tf(t: &str, n: usize, d: &TermFreq) -> f32 {
        let n = n as f32;
        let m = d.get(t).cloned().unwrap_or(0) as f32;
        m / n
    }

    fn compute_idf(t: &str, n: usize, df: &DocFreq) -> f32 {
        let n = n as f32;
        let m = df.get(t).cloned().unwrap_or(1) as f32;
        (n / m).log10()
    }

    pub fn search_query<'a>(self: &'a Self, query: &'a [char]) -> Result<Vec<(&'a Path, f32)>, ()> {
        let mut result = Vec::<(&Path, f32)>::new();
        let tokens = Lexer::new(&query).collect::<Vec<_>>();
        for (path, (n, tf_table)) in &self.term_freq_per_doc {
            let mut rank = 0f32;
            for token in &tokens {
                rank += Self::compute_tf(&token, *n, tf_table)
                    * Self::compute_idf(&token, self.term_freq_per_doc.len(), &self.doc_freq);
            }
            result.push((path, rank));
        }
        result.sort_by(|(_, rank1), (_, rank2)| rank1.partial_cmp(rank2).unwrap());
        result.reverse();
        Ok(result)
    }
}
