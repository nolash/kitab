use std::default;
use std::fs::{
    File,
    create_dir_all,
};
use std::io::Write;
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
use walkdir::WalkDir;
use hex;
use log::{
    debug,
    info,
};

use biblatex::EntryType;
use kitab::rdf::{
    read as rdf_read,
    read_all as rdf_read_all,
    write as rdf_write,
};
use kitab::biblatex::{
    read_all as biblatex_read_all,
};
use kitab::meta::{
    MetaData,
    digest_from_path,
};


fn args_setup() -> ArgMatches<'static> {
    let mut o = App::new("kitab");
    o = o.version("0.0.1");
    o = o.author("Louis Holbrook <dev@holbrook.no>");
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

    let mut o_apply = (
        SubCommand::with_name("apply")
        .about("import information from file")
        .version("0.0.1")
        );
    o_apply = o_apply.arg(
        Arg::with_name("PATH")
        .help("Path to operate on")
        .required(true)
        .index(1)
        );
    o = o.subcommand(o_apply);

    o.get_matches()
}

// commands
// kitab import <file> - attempt in order import rdf, import spec
    // kitab export <file> - export rdf/turtle
    // kitab apply <path> - recursively 

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

fn store(index_path: &Path, m: &MetaData) {
    let fp = index_path.join(m.fingerprint());
    create_dir_all(&index_path);
    debug!("writing record for title {} to {:?}", m.title(), &fp);

    let ff = File::create(&fp).unwrap();
    rdf_write(&m, &ff).unwrap();
    debug!("stored as rdf {:?}", fp);
}

fn exec_import_xattr(f: &Path, index_path: &Path) -> bool {
    let m = MetaData::from_xattr(f);
    match m.typ() {
        EntryType::Unknown(v) => {
            return false;
        },
        _ => {},
    };

    debug!("successfully processed xattr import source");

    info!("importing xattr source {:?}", &m);
    store(index_path, &m);
    true
}

fn exec_import_rdf(f: &Path, index_path: &Path) -> bool {
    let f = File::open(f).unwrap();
    let entries = match rdf_read_all(&f) {
        Ok(v) => {
            v
        },
        Err(e) => {
            return false;
        }
    };

    debug!("successfully processed rdf import source");

    for m in entries {
        info!("importing rdf source {:?}", &m);
        store(index_path, &m);
    }
    true
}

fn exec_import_biblatex(f: &Path, index_path: &Path) -> bool {
    let f = File::open(f).unwrap();
    let entries = match biblatex_read_all(&f) {
        Ok(v) => {
            v
        },
        Err(e) => {
            return false;
        }       
    };

    debug!("successfully processed biblatex import source");

    for m in entries {
        info!("importing biblatex source {:?}", &m);
        store(index_path, &m);    
    }

    true
}

fn exec_apply(p: &Path, index_path: &Path) -> bool {
    for entry in WalkDir::new(&p)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir()) {
            let ep = entry.path();
            let z = digest_from_path(ep);
            let z_hex = hex::encode(z);

            let fp = index_path.join(&z_hex);
            match fp.canonicalize() {
                Ok(v) => {
                    let f = File::open(&v).unwrap();
                    let m = rdf_read(f);
                    info!("apply {:?} -> {:?} for {:?}", entry, &m, z_hex);
                    m.to_xattr(&ep);
                },
                Err(e) => {
                    debug!("metadata not found for {:?} -> {:?}", entry, z_hex);
                },
            };
    }
    true
}

fn exec_import(p: &Path, index_path: &Path) {
    for entry in WalkDir::new(&p)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir()) {

        let fp = entry.path();
        debug!("processing {:?}", fp);
        if exec_import_xattr(fp, index_path) {
            continue;
        }
        if exec_import_rdf(fp, index_path) {
            continue;
        } 
        if exec_import_biblatex(fp, index_path) {
            continue;
        }
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
            info!("import from path {:?}", &p);
            return exec_import(&p, index_dir.as_path());
        },
        _ => {},
    }

    let mut r = true;
    match args.subcommand_matches("apply") {
        Some(v) => {
            let p = str_to_path(v);
            info!("apply from path {:?}", &p);
            if !exec_apply(p.as_path(), index_dir.as_path()) {
                r = false; 
            }
        },
        _ => {},
    }
}
