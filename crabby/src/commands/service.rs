use crate::logger;

pub fn service(_args : std::env::Args) {

    logger::log("Test generated.");
}