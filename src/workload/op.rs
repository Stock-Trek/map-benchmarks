#[derive(Clone, Copy, Debug)]
pub enum WorkloadOp {
    Lookup,
    Insert,
    Remove,
}
