//! Error types.

use std::fmt::{self, Display};

use thiserror::Error;

use crate::rafsi::RafsiShape;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs, reason = "obvious")]
#[non_exhaustive]
/// Answers questions of the form "invalid what?" etc.
pub enum What {
    Cluster,
    Cmavo,
    ConsonantalSyllable,
    Diphthong,
    Gismu,
    Nucleus,
    Onset,
}

impl Display for What {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        #[allow(clippy::enum_glob_use, reason = "no silly named wildcards")] use What::*;
        f.write_str(match self {
            Cluster => "cluster",
            ConsonantalSyllable => "consonantal syllable",
            Diphthong => "diphthong",
            Nucleus => "nucleus",
            Onset => "onset",
            Cmavo => "cmavo",
            Gismu => "gismu",
        })
    }
}

/// An error.
#[derive(Debug, Error, PartialEq, Eq, Clone)]
#[allow(missing_docs, reason = "obvious")]
#[non_exhaustive]
pub enum Jvofli {
    #[error("{{{value}}} is not a valid {what}")]
    Invalid { what: What, value: String },
    #[error("no unit split makes {{{0}}} a valid stress location")]
    InvalidStressPosition(String),
    #[error("{{{0}}} may not be doubled when representing a glide both times")]
    LongGlide(char),
    #[error("{{{word}}} can't have {{{attempted}}} as a rafsi, because it's a {shape} rafsi")]
    LongRafsiAssignment { word: String, shape: RafsiShape, attempted: String },
    #[error(
        "{{'}} must be between two vowels, but has {} before it and {} after it",
        before.as_ref().map_or_else(|| "nothing".into(), |c| format!("{{{c}}}")),
        after.as_ref().map_or_else(|| "nothing".into(), |c| format!("{{{c}}}")),
    )]
    MisplacedApostrophe { before: Option<char>, after: Option<char> },
    #[error("{{{0}}} isn't a lojban character")]
    NonLojbanCharacter(char),
    #[error("{{{0}}} is only one syllable")]
    NotEnoughSyllables(String),
    #[error("{{{0}}} as an onglide can't be adjacent to consonants")]
    OnglideInCluster(char),
    #[error(
        "{{{word}}} already has the {shape} rafsi {{{existing}}}, so it can't also have \
         {{{attempted}}}"
    )]
    RafsiShapeTaken { word: String, shape: RafsiShape, existing: String, attempted: String },
    #[error("{{{0}}} is a CCV'V slinku'i, which is never valid")]
    CCVhV(String),
    #[error("{{{0}}} isn't stressable")]
    Unstressable(String),
    #[error("{{{0}}} is at the start of a pre-brivla but it isn't stressable")]
    UnstressablePreBrivlaStart(String),
    #[error("{{{0}}} is at the end of a pre-brivla but it isn't stressable")]
    UnstressablePreBrivlaEnd(String),
    #[error("you can't give {{{word}}} an empty rafsi")]
    EmptyRafsiAssignment { word: String },
}

pub(crate) fn invalid_from_pair(w: What, x: char, y: char) -> Jvofli {
    Jvofli::Invalid { what: w, value: format!("{x}{y}") }
}
pub(crate) fn invalid_cluster_from_triple(x: char, y: char, z: char) -> Jvofli {
    Jvofli::Invalid { what: What::Cluster, value: format!("{x}{y}{z}") }
}
