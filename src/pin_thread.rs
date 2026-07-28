pub struct PinThread;

impl PinThread {
    pub fn try_pin(cpu_id: usize) -> Result<(), std::io::Error> {
        if Self::did_pin_thread(cpu_id) {
            Ok(())
        } else {
            Err(std::io::Error::last_os_error())
        }
    }

    fn did_pin_thread(cpu_id: usize) -> bool {
        use std::mem;
        unsafe {
            let mut cpu_set: libc::cpu_set_t = mem::zeroed();
            libc::CPU_SET(cpu_id, &mut cpu_set);
            let ret = libc::sched_setaffinity(0, mem::size_of::<libc::cpu_set_t>(), &cpu_set);
            ret == 0
        }
    }
}
