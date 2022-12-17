use std::error::Error;

use icu_datagen::{all_keys, datagen, CldrLocaleSubset, Out, SourceData};
use ra_bug::data_root;

fn main() -> Result<(), Box<dyn Error>> {
    simple_logger::SimpleLogger::new()
        .env()
        .with_level(log::LevelFilter::Info)
        .init()?;

    let source_data = SourceData::default()
        .with_cldr_latest(CldrLocaleSubset::Modern)?
        .with_icuexport_latest()?;

    let out = Out::Module {
        mod_directory: data_root().join("baked"),
        pretty: true,
        insert_feature_gates: true,
        use_separate_crates: true,
    };

    datagen(None, &all_keys(), &source_data, [out].into()).map_err(Into::into)
}
