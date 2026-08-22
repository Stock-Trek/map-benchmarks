use crate::workload::op::WorkloadOp;

#[derive(Clone, Copy, Debug)]
pub struct WorkItem<K> {
    pub op: WorkloadOp,
    pub key: K,
}
