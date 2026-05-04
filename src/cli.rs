use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "krdict")]
#[command(about = "Korean Dictionary API Client", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    // Optional export to JSON file
    #[arg(short, long, global = true, value_name = "FILE")]
    pub export: Option<PathBuf>,

    // Suppress stdout and stderr output
    #[arg(short, long, global = true)]
    pub quiet: bool,
}

#[derive(Clone, ValueEnum)]
pub enum SortMethod {
    Dict,
    Popular,
}

impl SortMethod {
    pub fn as_str(&self) -> &str {
        match self {
            SortMethod::Dict => "dict",
            SortMethod::Popular => "popular",
        }
    }
}

#[derive(Clone, ValueEnum)]
pub enum SearchPart {
    Word,
    Ip,
    Dfn,
    Exam,
}

impl SearchPart {
    pub fn as_str(&self) -> &str {
        match self {
            SearchPart::Word => "word",
            SearchPart::Ip => "ip",
            SearchPart::Dfn => "dfn",
            SearchPart::Exam => "exam",
        }
    }
}

#[derive(Clone, ValueEnum)]
pub enum SearchMethod {
    Exact,
    Include,
    Start,
    End,
}

impl SearchMethod {
    pub fn as_str(&self) -> &str {
        match self {
            SearchMethod::Exact => "exact",
            SearchMethod::Include => "include",
            SearchMethod::Start => "start",
            SearchMethod::End => "end",
        }
    }
}

#[derive(Clone, ValueEnum)]
pub enum ViewMethod {
    WordInfo,
    TargetCode,
}

impl ViewMethod {
    pub fn as_str(&self) -> &str {
        match self {
            ViewMethod::WordInfo => "word_info",
            ViewMethod::TargetCode => "target_code",
        }
    }
}

#[derive(Subcommand)]
pub enum Commands {
    /// Search the dictionary
    Search {
        /// Search term (UTF-8 encoded)
        #[arg(short, long)]
        query: String,

        /// Starting number for search (1-1000)
        #[arg(long, default_value = "1")]
        start: u32,

        /// Number of search results to display (10-100)
        #[arg(long, default_value = "10")]
        num: u32,

        /// Sort method
        #[arg(long, default_value = "dict")]
        sort: SortMethod,

        /// Search target
        #[arg(long, default_value = "word")]
        part: SearchPart,

        /// Enable multilingual translation
        #[arg(long)]
        translated: bool,

        /// Translation language (0-11, comma-separated)
        #[arg(long, default_value = "0")]
        trans_lang: String,

        /// Enable detailed search
        #[arg(long)]
        advanced: bool,

        /// What to find (1-10) - requires --advanced
        #[arg(long)]
        target: Option<u32>,

        /// Language (0-49) - requires target=4 (original language)
        #[arg(long)]
        lang: Option<u32>,

        /// Search method
        #[arg(long)]
        method: Option<SearchMethod>,

        /// Category 1 (comma-separated: word, phrase, expression)
        #[arg(long)]
        type1: Option<String>,

        /// Category 2 (comma-separated: native, chinese, loanword, hybrid)
        #[arg(long)]
        type2: Option<String>,

        /// Vocabulary by grade (comma-separated: level1, level2, level3)
        #[arg(long)]
        level: Option<String>,

        /// Parts of speech (0-15, comma-separated)
        #[arg(long)]
        pos: Option<String>,

        /// Multimedia information (0-6, comma-separated)
        #[arg(long)]
        multimedia: Option<String>,

        /// Start syllable count
        #[arg(long)]
        letter_s: Option<u32>,

        /// End syllable count
        #[arg(long)]
        letter_e: Option<u32>,

        /// Semantic category (0-153, comma-separated)
        #[arg(long)]
        sense_cat: Option<String>,

        /// Topic and situation categories (0-106, comma-separated)
        #[arg(long)]
        subject_cat: Option<String>,
    },
    /// View detailed dictionary content
    View {
        /// Search method
        #[arg(short, long, default_value = "word-info")]
        method: ViewMethod,

        /// Search term (word or target_code)
        #[arg(short, long)]
        query: String,

        /// Enable multilingual translation
        #[arg(long)]
        translated: bool,

        /// Translation language (0-11, comma-separated)
        #[arg(long, default_value = "0")]
        trans_lang: String,
    },
}