use structopt::StructOpt;
use uuid::Uuid;

#[derive(Debug, StructOpt)]
#[structopt(name = "UUID generator")]
struct GeneratorSettings {
    /// Number of UUIDs to generate
    #[structopt(short = "n", default_value = "1")]
    uuid_count: usize,
}
fn main() {
    let opt = GeneratorSettings::from_args();

    for _ in 0..opt.uuid_count {
        println!("{}", Uuid::new_v4());
    }
}
