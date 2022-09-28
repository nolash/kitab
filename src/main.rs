use std::default;
use std::fs::{
    File,
    create_dir_all,
    metadata,
};
use std::io::Write;
use std::path::{
    Path,
    PathBuf,
};
use std::str::FromStr;
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
    warn,
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
    digests_from_path,
};
use kitab::digest::from_urn;
use kitab::digest::RecordDigest;
use kitab::digest::DigestType;


fn args_setup() -> ArgMatches<'static> {
    let mut o = App::new("kitab");
    o = o.version("0.0.1");
    o = o.author("Louis Holbrook <dev@holbrook.no>");

    o = o.arg(clap::Arg::with_name("store")
        .short("s")
        .long("store")
        .value_name("Store location")
        .takes_value(true)
        );
        
    let mut o_import = (
        SubCommand::with_name("import")
        .about("import information from file")
        .version("0.0.1")
        );
    o_import = o_import.arg(
        Arg::with_name("setdigest")
        .short("d")
        .long("digest")
        .help("Explicitly set digest")
        .multiple(true)
        .takes_value(true)
        .number_of_values(1)
        );
    o_import = o_import.arg(
        Arg::with_name("PATH")
        .help("Path to operate on")
        .required(true)
        );
    o = o.subcommand(o_import);

    let mut o_apply = (
        SubCommand::with_name("apply")
        .about("Apply metadata on matching files")
        .version("0.0.1")
        );
    o_apply = o_apply.arg(
        Arg::with_name("PATH")
        .help("Path to operate on")
        .required(true)
        .index(1)
        );
    o_apply = o_apply.arg(
        Arg::with_name("adddigest")
        .short("d")
        .long("digest")
        .help("Additional digest to store")
        .multiple(true)
        .takes_value(true)
        .number_of_values(1)
        );
    o = o.subcommand(o_apply);

//    let mut o_entry = (
//       SubCommand::with_name("new")
//        .about("add metadata for file")
//        .version("0.0.1")
//        );
//
//    o_entry = o_entry.arg(clap::Arg::with_name("validators")
//         .long("validator")
//         .value_name("Add given validator engine")
//         .multiple(true)
//         .takes_value(true)
//         );
//
//    o_entry = o_entry.arg(
//        Arg::with_name("PATH")
//        .help("Path to operate on")
//        .required(true)
//        .index(1)
//        );
//    o = o.subcommand(o_entry);

    o.get_matches()
}

// commands
// kitab import <file> - attempt in order import rdf, import spec
// kitab apply <path> - recursively 

    fn resolve_directory(args: &ArgMatches) -> PathBuf {
        let r = match args.value_of("store") {
            Some(v) => {
                v
            },
            _ => {
                ""
            },
        };
        if r.len() != 0 {
            return PathBuf::from(r)
        }
        

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

fn exec_import_xattr(f: &Path, index_path: &Path, digests: &Vec<RecordDigest>) -> bool {
    let mut m = match MetaData::from_xattr(f) {
        Ok(r) => {
            r
        }
        Err(e) => {
            return false;
        }
    };

    debug!("successfully processed xattr import source");

    info!("importing xattr source {:?}", &m);

    let mut digest_types: Vec<DigestType> = vec!();

    for v in digests.iter() {
        match v {
            RecordDigest::EmptyWithType(digest_typ) => {
                digest_types.push(*digest_typ);
            },
            RecordDigest::Empty => {
                digest_types.push(DigestType::Sha512);
            },
            _ => {
                warn!("digest specifier {:?} is invalid in xattr import context.", v);
            },
        };
    }

    for v in digests_from_path(f, &digest_types) {
        m.set_fingerprint(v);
        store(index_path, &m);
    }
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

fn exec_import_biblatex(f: &Path, index_path: &Path, digests: &Vec<RecordDigest>) -> bool {
    let f = File::open(f).unwrap();
    let entries = match biblatex_read_all(&f, digests) {
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

fn exec_apply(p: &Path, index_path: &Path, mut extra_digest_types: Vec<DigestType>) -> bool {
    let mut digest_types: Vec<DigestType> = vec!(DigestType::Sha512);
    digest_types.append(&mut extra_digest_types);
    for entry in WalkDir::new(&p)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir()) {
            let ep = entry.path();
            for digest in digests_from_path(ep, &digest_types) {
                let z_hex = hex::encode(digest.fingerprint());

                let fp = index_path.join(&z_hex);
                match fp.canonicalize() {
                    Ok(v) => {
                        let f = File::open(&v).unwrap();
                        let m = rdf_read(f);
                        info!("apply {:?} -> {:?}", entry, &m);
                        m.to_xattr(&ep);
                    },
                    Err(e) => {
                        debug!("metadata not found for {:?} -> {:?}", entry, z_hex);
                    },
                };
            }
    }
    true
}

fn exec_import(p: &Path, index_path: &Path, digests: Vec<RecordDigest>) {
    for entry in WalkDir::new(&p)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir()) {

        let fp = entry.path();
        debug!("attempt xattr import {:?}", fp);
        if exec_import_xattr(fp, index_path, &digests) {
            continue;
        }

        let st = entry.metadata().unwrap();
        if st.len() > 1048576 {
            warn!("skipping metadata content probe for file >1MB");
            continue;
        }

        debug!("attempt rdf import {:?}", fp);
        if exec_import_rdf(fp, index_path) { 
            continue;
        } 

        debug!("attempt biblatex import {:?}", fp);
        if exec_import_biblatex(fp, index_path, &digests) {
            continue;
        }
    }
}

fn exec_entry(p: &Path, index_path: &Path) -> bool {
    if !p.is_file() {
        return false; 
    }
    true
}

fn main() {
    env_logger::init();

    let args = args_setup();

    let index_dir = resolve_directory(&args);
    info!("have index directory {:?}", &index_dir);
   
    match args.subcommand_matches("import") {
        Some(arg) => {
            let p = str_to_path(&arg);
            let mut digests: Vec<RecordDigest> = Vec::new();
            match arg.values_of("setdigest") {
                Some(r) => {
                    for digest_str in r {
                        match from_urn(&digest_str) {
                            Ok(digest) => {
                                info!("using digest {}", digest_str);
                                digests.push(digest);
                            },
                            Err(e) => {
                                let digest_type = match DigestType::from_str(digest_str) {
                                    Ok(v) => {
                                        v
                                    },
                                    Err(e) => {
                                        panic!("invalid digest specifier: {:?}", e);
                                    },
                                };
                                let digest_empty = RecordDigest::EmptyWithType(digest_type);
                                digests.push(digest_empty);
                            },
                        }
                    }
                },
                None => {},
            };
            info!("import from path {:?}", &p);
            return exec_import(&p, index_dir.as_path(), digests);
        },
        _ => {},
    };


    let mut r = true;
    match args.subcommand_matches("apply") {
        Some(arg) => {
            let p = str_to_path(&arg);
            let mut digests: Vec<DigestType> = Vec::new();
            match arg.values_of("adddigest") {
                Some(r) => {
                    for digest_str in r {
                        match DigestType::from_str(digest_str.clone()) {
                            Ok(digest) => {
                                info!("using digest type {}", digest_str);
                                digests.push(digest);
                            },
                            Err(e) => {
                                panic!("invalid digest URN: {:?}", e);
                            },
                        }
                    }
                },
                None => {},
            };

            info!("apply from path {:?}", &p);
            if !exec_apply(p.as_path(), index_dir.as_path(), digests) {
                r = false; 
            }
        },
        _ => {},
    }

//    match args.subcommand_matches("new") {
//        Some(v) => {
//            let p = str_to_path(v);
//            info!("new metadata for path {:?}", &p);
//            if !exec_entry(p.as_path(), index_dir.as_path()) {
//                r = false; 
//            }
//        },
//        _ => {},
//    }
}
