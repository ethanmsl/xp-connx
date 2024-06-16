//! Attempt to pull data with SQLx and then convert to DataFrame

use clap::{Parser, ValueEnum};
use connectorx::{prelude::*, sources::mysql::MySQLSource};
use polars::prelude::*;
use tracing;
use tracing_subscriber::EnvFilter;

use crate::{args::{Args, Backend},
            connectorx_df, sqlx_trans, struct_to_dataframe,
            vecstruct_df::TestStruct,
            Result};

pub fn sub_main(args: Args) -> Result<()> {
    tracing::info!("SQLx ->>");
    todo!();
}
