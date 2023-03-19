use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufWriter};
use std::path::{Path, PathBuf};

pub type DocFreq = HashMap<String, u32>;
pub type TermFreq = HashMap<String, u32>;
pub type TermFreqPerDoc = HashMap<PathBuf, TermFreq>;

#[derive(Default, Deserialize, Serialize)]
pub struct TermIndex {
    pub term_freq_per_doc: TermFreqPerDoc,
    pub doc_freq: DocFreq,
}

impl TermIndex {
    fn save_index_as_json(&self, json_file_path: &PathBuf) -> io::Result<()> {
        // saving index to json
        println!("Saving {}", json_file_path.to_str().unwrap());
        let index_file = File::create(json_file_path)?;
        serde_json::to_writer(BufWriter::new(index_file), &self).expect("serde works");
        Ok(())
    }
    pub fn compute_tf(self, t: &str, n: usize, d: &TermFreq) -> f32 {
        let n = n as f32;
        let m = d.get(t).cloned().unwrap_or(0) as f32;
        m / n
    }

    pub fn compute_idf(t: &str, n: usize, df: &DocFreq) -> f32 {
        let n = n as f32;
        let m = df.get(t).cloned().unwrap_or(1) as f32;
        (n / m).log10()
    }
}
