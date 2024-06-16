/// Querying with ConnectorX and then trying to turn to Arrow or Polars dataframe
///
/// # Critical: Failing
/// The problem is that the Arrow ecosystem in Rust is fragmented and ConnectorX uses outdated versions of Polars to create the DataFrame.
/// This means that (a) the DataFrame is not compatible with a current version of Polars.  (programs with different names merely share a name)
/// (b) that the components of that DataFrame also do not match various crates.
///
/// Additionally, there are private components beneath that that are also hard to get at.
///
/// # Possible Solution:
/// Best solutin so far would be to rely on Arrow's memory layout being stanard and therefore using C-FFI to exchange the raw chunk of memory and take them as new Polars DataFrames
use clap::{Parser, ValueEnum};
use connectorx::{get_arrow2, prelude::*, sources::mysql::MySQLSource};
use itertools::Itertools;
use polars::prelude::*;
use tracing;
use tracing_subscriber::EnvFilter;

use crate::{args::{Args, Backend},
            connectorx_df, sqlx_trans, struct_to_dataframe,
            vecstruct_df::TestStruct};

pub fn sub_main(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    tracing::info!("ConnectorX ->>");
    let query_string = format!(r#"
                                                SELECT StudentID as id, 
                                                       FirstName as first_name, 
                                                       LastName as last_name, 
                                                       DateOfBirth as dob, 
                                                       School as school, 
                                                       Email as email
                                                FROM students 
                                                LIMIT {}
                                                "#,
                               args.lines.unwrap_or(3));

    let mut destination = Arrow2Destination::new();
    let db_source = SourceConn::try_from("mysql://root:root@127.0.0.1/university")?;

    let query = &[CXQuery::from(&query_string)];
    destination = get_arrow2(&db_source, None, query)?;
    let data = destination.arrow()?;

    println!("->> Data{:?}", data);

    let mut strx: Vec<TestStruct> = Vec::new();

    // itertool iterator style
    (0..10).cartesian_product((10_000..10_020))
           .for_each(|(h, w)| {
               strx.push(TestStruct { height: h,
                                      width:  w, })
           });

    // // native iterator style
    // (0..10).for_each(|h| {
    //            (10_000..10_020).for_each(|w| {
    //                                strx.push(TestStruct { height: h,
    //                                                       width:  w, })
    //                            })
    //        });

    // // for-loop style
    // for h in (0..10) {
    //     for w in (10_000..10_020) {
    //         strx.push(TestStruct { height: h,
    //                                width:  w, });
    //     }
    // }
    let df = struct_to_dataframe!(strx, [height, width]);
    println!("->> Dataframe{:?}", df);
    Ok(())
}
