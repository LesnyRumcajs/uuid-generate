use itertools::Itertools;
use structopt::StructOpt;
use uuid::Uuid;

#[derive(Debug, StructOpt)]
#[structopt(name = "UUID generator")]
struct GeneratorSettings {
    /// Number of UUIDs to generate
    #[structopt(short = "n", default_value = "1")]
    uuid_count: usize,
    /// Delimiter (if n > 1)
    #[structopt(short = "d", default_value = ",")]
    delimiter: String,
    /// End (Used to be newline, now defaults to emty sting)
    #[structopt(short = "e", default_value = "")]
    end: String,
}
fn main() {
    let opt = GeneratorSettings::from_args();

    let uuids = (0..opt.uuid_count).map(|_| Uuid::new_v4());
    print!("{}{}", uuids.into_iter().join(&opt.delimiter), opt.end)
}
