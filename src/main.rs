use std::path::PathBuf;
use std::process::exit;
use structopt::StructOpt;

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
        hashes: Option<PathBuf>,
    },
    Patch {
        #[structopt(parse(from_os_str))]
        source: PathBuf,

        #[structopt(parse(from_os_str))]
        patch: PathBuf,

        #[structopt(parse(from_os_str))]
        output: PathBuf,
    },
}

fn handle_diff(source: PathBuf, modded: PathBuf, output: PathBuf, is_text: bool) {
    let source = prcx::open(source).unwrap();
    let modded = prcx::open(modded).unwrap();
    let diff = prcx::generate_patch(&source, &modded).unwrap();
    match diff {
        Some(diff) => {
            if is_text {
                let mut file = std::io::BufWriter::new(std::fs::File::create(output).unwrap());
                prcx::write_xml(&diff, &mut file).unwrap();
            } else {
                prcx::save(output, &diff).unwrap();
            }
        }
        None => println!("No differences were found between the two files"),
    }
}

fn handle_patch(source: PathBuf, patch: PathBuf, output: PathBuf) {
    let mut source = prcx::open(source).unwrap();
    let patch = if let Ok(patch) = prcx::open(&patch) {
        patch
    } else {
        prcx::read_xml(&mut std::io::BufReader::new(
            std::fs::File::open(patch).unwrap(),
        ))
        .unwrap()
    };
    prcx::apply_patch(&patch, &mut source).unwrap();
    prcx::save(output, &source).unwrap();
}

fn main() {
    let opt = Opt::from_args();
    match opt {
        Opt::Diff {
            source,
            modded,
            output,
            out_type,
            hashes,
        } => {
            if let Some(hashes) = hashes {
                let labels = prcx::hash40::label_map::LabelMap::read_custom_labels(hashes).unwrap();
                prcx::hash40::Hash40::label_map()
                    .lock()
                    .unwrap()
                    .add_custom_labels(labels.into_iter());
            }
            let is_text = if out_type == "xml" {
                true
            } else if out_type == "bin" {
                false
            } else {
                eprintln!("Output type must be either \"xml\" or \"bin\"");
                exit(1);
            };
            handle_diff(source, modded, output, is_text);
        }
        Opt::Patch {
            source,
            patch,
            output,
        } => handle_patch(source, patch, output),
    }
}
