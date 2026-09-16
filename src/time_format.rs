/// Formats the given wall-clock time as a local time of day in the
/// `HH:mm:ss:SSS` format. Used to make benchmark start/end timestamps easier to
/// read than raw seconds since the Unix epoch.
#[doc(hidden)]
pub fn format_time_of_day(time: ::std::time::SystemTime) -> String {
    let duration = time
        .duration_since(::std::time::UNIX_EPOCH)
        .unwrap_or_default();
    let seconds = duration.as_secs() as ::libc::time_t;
    let millis = duration.subsec_millis();
    let mut tm: ::libc::tm = unsafe { ::std::mem::zeroed() };
    // SAFETY: `seconds` and `tm` are valid pointers for the duration of the call.
    unsafe {
        ::libc::localtime_r(&seconds, &mut tm);
    }
    format!(
        "{:02}:{:02}:{:02}:{:03}",
        tm.tm_hour, tm.tm_min, tm.tm_sec, millis
    )
}
