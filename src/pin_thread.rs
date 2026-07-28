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
        if let Err(e) = Self::try_pin(cpu_id) {
            eprintln!("Warning: failed to pin thread to CPU {}: {}", cpu_id, e);
        }
    }

    /// Pin the current thread to a specific CPU core, returning a `Result`.
    ///
    /// On success, returns `Ok(())`. On platforms where pinning is not
    /// supported, returns `Err` with a `NotFound` kind error.
    pub fn try_pin(cpu_id: usize) -> Result<(), std::io::Error> {
        #[cfg(target_os = "linux")]
        {
            if Self::try_pin_impl(cpu_id) {
                Ok(())
            } else {
                Err(std::io::Error::last_os_error())
            }
        }
        #[cfg(not(target_os = "linux"))]
        {
            let _ = cpu_id;
            Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "thread pinning not supported on this platform",
            ))
        }
    }

    /// Low-level pinning implementation. Returns `true` on success.
    #[cfg(target_os = "linux")]
    fn try_pin_impl(cpu_id: usize) -> bool {
        use std::mem;
        unsafe {
            let mut cpu_set: libc::cpu_set_t = mem::zeroed();
            libc::CPU_SET(cpu_id, &mut cpu_set);
            let ret = libc::sched_setaffinity(0, mem::size_of::<libc::cpu_set_t>(), &cpu_set);
            ret == 0
        }
    }
}
