#[derive(Clone, Copy, Debug)]
pub struct WorkloadDesign {
    pub lookup_hits: usize,
    pub lookup_misses: usize,
    pub inserts: usize,
    pub updates: usize,
    pub removes: usize,
}

impl WorkloadDesign {
    pub fn total_ops(&self) -> usize {
        self.lookup_hits + self.lookup_misses + self.inserts + self.updates + self.removes
    }
    pub fn write_heavy(total_ops: usize) -> Self {
        Self {
            lookup_hits: (total_ops as f64 * 0.20) as usize,
            lookup_misses: 0,
            inserts: (total_ops as f64 * 0.80) as usize,
            updates: 0,
            removes: 0,
        }
    }
    pub fn high_churn(total_ops: usize) -> Self {
        Self {
            lookup_hits: (total_ops as f64 * 0.40) as usize,
            lookup_misses: (total_ops as f64 * 0.10) as usize,
            inserts: (total_ops as f64 * 0.10) as usize,
            updates: (total_ops as f64 * 0.30) as usize,
            removes: (total_ops as f64 * 0.10) as usize,
        }
    }
    pub fn balanced(total_ops: usize) -> Self {
        Self {
            lookup_hits: (total_ops as f64 * 0.75) as usize,
            lookup_misses: (total_ops as f64 * 0.05) as usize,
            inserts: (total_ops as f64 * 0.05) as usize,
            updates: (total_ops as f64 * 0.10) as usize,
            removes: (total_ops as f64 * 0.05) as usize,
        }
    }
    pub fn read_heavy(total_ops: usize) -> Self {
        Self {
            lookup_hits: (total_ops as f64 * 0.90) as usize,
            lookup_misses: (total_ops as f64 * 0.05) as usize,
            inserts: (total_ops as f64 * 0.05) as usize,
            updates: 0,
            removes: 0,
        }
    }
}
