#![crate_name = "kitab"]

pub mod meta;

pub mod dc;

pub mod store;

pub mod rdf;

pub mod biblatex;

pub mod error;

pub mod digest;

#[cfg(test)]
mod tests {
    use env_logger;

    #[test]
    fn test_setup_env_logger() {
        env_logger::init();
    }
}
