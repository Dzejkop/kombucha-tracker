use cmd_lib::run_cmd;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
enum Opt {
    StartAll,
    InstallDeps,
}

fn main() {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "xtask=trace,$RUST_LOG");
    pretty_env_logger::init();

    let args = Opt::from_args();

    match args {
        Opt::StartAll => run_cmd!("").unwrap(),
        Opt::InstallDeps => run_cmd!("").unwrap(),
    }
}
