pub mod answers;
pub mod dict;
pub mod options;
pub mod rules;
pub mod tests;

pub use answers::Answers;
pub use dict::{Dict, DictError};
pub use options::Options;
use rules::DictRegistry;
pub use tests::{Test, TestSuite};

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref DICT_REGISTRY: DictRegistry = {
         include!(concat!(env!("OUT_DIR"), "/dict_registry.rs"));
    };
}