use thiserror::Error;

pub mod dc_op;
pub mod transient;

#[derive(Error, Debug, PartialEq)]
pub enum RunnerError {
    #[error("at least one node is required to execute a runner")]
    ZeroNode,
    #[error("the circuit is malformed and cannot inverse the matrix")]
    MalformedCircuit,
}
