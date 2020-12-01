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
    let uuid = Uuid::new_v4();

    for _ in 0..opt.uuid_count {
        println!("{}", uuid);
    }
}
