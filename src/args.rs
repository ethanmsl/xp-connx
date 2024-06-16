/// Args and other structs for use by Clap
use clap::{Parser, ValueEnum};

use crate::consts::{DOCKER_URI, NO_SYNC_DIR};

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Args {
    /// Which backend to use (sqlx or connx)
    pub backend: Backend,

    /// Number of lines to request
    pub lines: Option<u32>,
}

/// Which backend to use -- i.e. which  file in `src/bin` to run.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Backend {
    Sqlx,
    Connx,
}
