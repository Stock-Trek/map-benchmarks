/// A no-size struct that provides thread pinning functionality.
///
/// Pinning a thread to a specific CPU core can reduce effects from OS scheduling
/// in benchmarks. This is a best-effort operation. On platforms where pinning is
/// not supported, a warning is printed and the method returns without error.
pub struct PinThread;

impl PinThread {
    /// Pin the current thread to a specific CPU core.
    ///
    /// This is a best-effort operation. On platforms where pinning is not
    /// supported, a warning is printed and the method returns without error.
    pub fn pin(cpu_id: usize) {
        #[cfg(target_os = "linux")]
        {
            use std::mem;
            unsafe {
                let mut cpu_set: libc::cpu_set_t = mem::zeroed();
                libc::CPU_SET(cpu_id, &mut cpu_set);
                let ret = libc::sched_setaffinity(0, mem::size_of::<libc::cpu_set_t>(), &cpu_set);
                if ret != 0 {
                    eprintln!(
                        "Warning: failed to pin thread to CPU {}: {}",
                        cpu_id,
                        std::io::Error::last_os_error()
                    );
                }
            }
        }
        #[cfg(not(target_os = "linux"))]
        {
            let _ = cpu_id;
            eprintln!("Warning: thread pinning not supported on this platform");
        }
    }
}
