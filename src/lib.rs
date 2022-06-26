#![crate_name = "kitab"]

pub mod meta;

pub mod dc;

pub mod store;

#[cfg(feature = "rdf")]
pub mod rdf;

#[cfg(test)]
mod tests {
    use env_logger;

    #[test]
    fn test_setup_env_logger() {
        env_logger::init();
    }
}
