pub mod lexer;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub type TermFreq = HashMap<String, u32>;
pub type TermFreqIndex = HashMap<PathBuf, TermFreq>;
