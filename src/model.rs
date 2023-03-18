use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub type DocFreq = HashMap<String, u32>;
pub type TermFreq = HashMap<String, u32>;
pub type TermFreqPerDoc = HashMap<PathBuf, TermFreq>;
