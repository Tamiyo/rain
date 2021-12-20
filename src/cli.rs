use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "rain", about = "example")]
pub(crate) struct Opt {
    #[structopt(parse(from_os_str))]
    pub(crate) input: PathBuf,
}

pub(crate) fn parse_args() -> Opt {
    let opt = Opt::from_args();
    opt
}
