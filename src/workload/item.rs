use crate::workload::op::WorkloadOp;

#[derive(Clone, Copy, Debug)]
pub struct WorkItem {
    pub op: WorkloadOp,
    pub key: u64,
}
