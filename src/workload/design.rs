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
    fn from_ratios(
        total_ops: usize,
        lookup_hits: f64,
        lookup_misses: f64,
        inserts: f64,
        updates: f64,
        removes: f64,
    ) -> Self {
        Self {
            lookup_hits: (total_ops as f64 * lookup_hits) as usize,
            lookup_misses: (total_ops as f64 * lookup_misses) as usize,
            inserts: (total_ops as f64 * inserts) as usize,
            updates: (total_ops as f64 * updates) as usize,
            removes: (total_ops as f64 * removes) as usize,
        }
    }
    pub fn write_heavy(total_ops: usize) -> Self {
        Self::from_ratios(total_ops, 0.20, 0.0, 0.80, 0.0, 0.0)
    }
    pub fn high_churn(total_ops: usize) -> Self {
        Self::from_ratios(total_ops, 0.40, 0.10, 0.10, 0.30, 0.10)
    }
    pub fn balanced(total_ops: usize) -> Self {
        Self::from_ratios(total_ops, 0.75, 0.05, 0.05, 0.10, 0.05)
    }
    pub fn read_heavy(total_ops: usize) -> Self {
        Self::from_ratios(total_ops, 0.90, 0.05, 0.05, 0.0, 0.0)
    }
}
