use std::default;
use std::fs::File;
use std::path::{
    Path,
    PathBuf,
};
use env_logger;
use clap::{
    App, 
    Arg,
    ArgMatches,
    SubCommand,
};
use directories::{
    BaseDirs,
};
use log::{
    debug,
    info,
};

#[cfg(feature = "rdf")]
use kitab::rdf::read as rdf_read;


fn args_setup() -> ArgMatches<'static> {
    let mut o = App::new("kitab");
    o = o.version("0.0.1");
    o = o.author("Louis Holbrook <dev@holbrook.no>");
    o = o.arg(
        Arg::with_name("file")
            .long("file")
            .short("f")
            .value_name("File to match records against")
            .takes_value(true)
            );
    let mut o_import = (
        SubCommand::with_name("import")
        .about("import information from file")
        .version("0.0.1")
        );
    o_import = o_import.arg(
        Arg::with_name("PATH")
        .help("Path to operate on")
        .required(true)
        .index(1)
        );

    o = o.subcommand(o_import);

    o.get_matches()
}

// commands
// kitab import <file> - attempt in order import rdf, import spec
// kitab export <file> - export rdf/turtle
// kitab scan <path> - recursively 

fn resolve_directory(args: &ArgMatches) -> PathBuf {
    match BaseDirs::new() {
        Some(v) => {
            let d = v.data_dir();
            d.join("kitab")
                .join("idx")
        },
        _ => {
            PathBuf::from(".")
                .join(".kitab")
                .join("/idx")
        },
    }
}

fn str_to_path(args: &ArgMatches) -> PathBuf {
    let mut p_canon: PathBuf;
    match args.value_of("PATH") {
        Some(v) => {
            let p = &Path::new(v);
            match p.canonicalize() {
                Ok(v) => {
                    p_canon = v.clone();
                },
                Err(e) => {
                    panic!("path error: {}", e);
                },
            };
        },
        None => {
            panic!("path required"); 
        },
    }
    p_canon
}

fn exec_import(f: &Path, index_path: &Path) {
    #[cfg(feature = "rdf")]
    {
        let f = File::open(f).unwrap();
        let m = rdf_read(&f);
        
        let fp = index_path.join(m.fingerprint());
        debug!("writing record for title {} to {:?}", m.title(), &fp);
    }
}

fn main() {
    env_logger::init();

    let args = args_setup();

    let index_dir = resolve_directory(&args);
    info!("have index directory {:?}", &index_dir);
    
    match args.subcommand_matches("import") {
        Some(v) => {
            let p = str_to_path(v);
            info!("have path {:?}", &p);
            return exec_import(p.as_path(), index_dir.as_path());
        },
        _ => {},
    }
}
