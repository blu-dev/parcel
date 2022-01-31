use structopt::StructOpt;
use std::path::PathBuf;
use std::process::exit;

#[derive(Debug, StructOpt)]
enum Opt {
    Diff {
        #[structopt(parse(from_os_str))]
        source: PathBuf,
        
        #[structopt(parse(from_os_str))]
        modded: PathBuf,

        #[structopt(parse(from_os_str))]
        output: PathBuf,

        #[structopt(short = "t", long = "type", default_value = "bin")]
        out_type: String,

        #[structopt(short, long)]
        hashes: Option<PathBuf>
    },
    Patch {
        #[structopt(parse(from_os_str))]
        source: PathBuf,

        #[structopt(parse(from_os_str))]
        patch: PathBuf,

        #[structopt(parse(from_os_str))]
        output: PathBuf
    }
}

fn handle_diff(source: PathBuf, modded: PathBuf, output: PathBuf, is_text: bool) {
    let source = prcx::prc::open(source).unwrap();
    let modded = prcx::prc::open(modded).unwrap();
    let diff = prcx::diff::Diff::generate(&source, &modded);
    diff.save(output, is_text).unwrap();
}

fn handle_patch(source: PathBuf, patch: PathBuf, output: PathBuf) {
    let mut source = prcx::prc::open(source).unwrap();
    let patch = if let Ok(patch) = prcx::diff::Diff::open(&patch) {
        patch
    } else {
        prcx::diff::Diff::open_bin(patch).unwrap()
    };
    patch.apply(&mut source);
    prcx::prc::save(output, &source).unwrap();
}

fn main() {
    let opt = Opt::from_args();
    match opt {
        Opt::Diff { source, modded, output, out_type, hashes } => {
            if let Some(hashes) = hashes {
                let file = std::fs::read_to_string(hashes).unwrap();
                let hashes = file.lines().map(|x| x.trim().split(",").last().unwrap()).collect();
                prcx::hash::add_hashes(hashes);
            }
            let is_text = if out_type == "text" {
                true
            } else if out_type == "bin" {
                false
            } else {
                eprintln!("Output type must be either \"test\" or \"bin\"");
                exit(1);
            };
            handle_diff(source, modded, output, is_text);
        },
        Opt::Patch { source, patch, output } => handle_patch(source, patch, output)
    }
}
