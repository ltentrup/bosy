use hyperltl::HyperLTL;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Specification {
    inputs: Vec<String>,
    outputs: Vec<String>,
    assumptions: Vec<HyperLTL>,
    guarantees: Vec<HyperLTL>,
}
