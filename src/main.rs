/// Exploratory App
/// Checking basic Querying of databases
/// and runing dynamic filters
use clap::{Parser, ValueEnum};
use connectorx::{prelude::*, sources::mysql::MySQLSource};
use polars::prelude::*;
use tracing;
use tracing_subscriber::EnvFilter;
use xp_connx::{args::{Args, Backend},
               connectorx_df, sqlx_trans, struct_to_dataframe,
               vecstruct_df::TestStruct};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().with_env_filter(EnvFilter::from_default_env())
                             .init();
    let args = Args::parse();
    tracing::debug!("->> args {:?}", args);

    match args.backend {
        Backend::Sqlx => {
            tracing::info!("->> Sqlx");
            sqlx_trans::sub_main(args)?;
        }
        Backend::Connx => {
            tracing::info!("->> Connx");
            connectorx_df::sub_main(args)?;
        }
    }

    Ok(())
}
