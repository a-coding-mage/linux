/* SPDX-License-Identifier: GPL-2.0 */

// Declarations corresponding to the Linux headers included by the C source
// are supplied by other translation units.

#[cfg(CONFIG_COREDUMP)]
#[repr(C)]
pub struct core_vma_metadata {
    pub start: ::kernel::ffi::c_ulong,
    pub end: ::kernel::ffi::c_ulong,
    pub flags: vm_flags_t,
    pub dump_size: ::kernel::ffi::c_ulong,
    pub pgoff: ::kernel::ffi::c_ulong,
    pub file: *mut file,
}

#[cfg(CONFIG_COREDUMP)]
#[repr(C)]
pub struct coredump_params {
    pub siginfo: *const kernel_siginfo_t,
    pub file: *mut file,
    pub limit: ::kernel::ffi::c_ulong,
    // MMF_DUMP_FILTER_* bits, snapshot of mm->flags at dump start.
    pub mm_flags: ::kernel::ffi::c_ulong,
    // Snapshot of dumpable at dump start.
    pub dumpable: task_dumpable,
    pub cpu: ::kernel::ffi::c_int,
    pub written: loff_t,
    pub pos: loff_t,
    pub to_skip: loff_t,
    pub vma_count: ::kernel::ffi::c_int,
    pub vma_data_size: usize,
    pub vma_meta: *mut core_vma_metadata,
    pub pid: *mut pid,
}

#[cfg(CONFIG_COREDUMP)]
unsafe extern "C" {
    pub static mut core_file_note_size_limit: ::kernel::ffi::c_uint;

    // These are the only things you should do on a core-file: use only these
    // functions to write out all the necessary info.
    pub fn dump_skip_to(cprm: *mut coredump_params, to: ::kernel::ffi::c_ulong);
    pub fn dump_skip(cprm: *mut coredump_params, nr: usize);
    pub fn dump_emit(
        cprm: *mut coredump_params,
        addr: *const ::kernel::ffi::c_void,
        nr: ::kernel::ffi::c_int,
    ) -> ::kernel::ffi::c_int;
    pub fn dump_align(cprm: *mut coredump_params, align: ::kernel::ffi::c_int)
        -> ::kernel::ffi::c_int;
    pub fn dump_user_range(
        cprm: *mut coredump_params,
        start: ::kernel::ffi::c_ulong,
        len: ::kernel::ffi::c_ulong,
    ) -> ::kernel::ffi::c_int;
    pub fn vfs_coredump(siginfo: *const kernel_siginfo_t);
}

// Logging for the coredump code, ratelimited. The TGID and comm fields are
// added to the message. The variadic formatting and kernel logging machinery
// are provided by the surrounding kernel translation.
#[cfg(CONFIG_COREDUMP)]
#[macro_export]
macro_rules! __COREDUMP_PRINTK {
    ($level:expr, $format:expr $(, $arg:expr)*) => {{
        unsafe {
            printk_ratelimited!($level, concat!("coredump: %d(%*pE): ", $format, "\n"),
                task_tgid_vnr(current), 0, current, $($arg),*);
        }
    }};
}

#[cfg(CONFIG_COREDUMP)]
#[macro_export]
macro_rules! coredump_report {
    ($fmt:expr $(, $arg:expr)*) => {
        $crate::__COREDUMP_PRINTK!(KERN_INFO, $fmt $(, $arg)*);
    };
}

#[cfg(CONFIG_COREDUMP)]
#[macro_export]
macro_rules! coredump_report_failure {
    ($fmt:expr $(, $arg:expr)*) => {
        $crate::__COREDUMP_PRINTK!(KERN_WARNING, $fmt $(, $arg)*);
    };
}

#[cfg(not(CONFIG_COREDUMP))]
pub unsafe extern "C" fn vfs_coredump(_siginfo: *const kernel_siginfo_t) {}

#[cfg(not(CONFIG_COREDUMP))]
#[macro_export]
macro_rules! coredump_report { ($($arg:tt)*) => {}; }

#[cfg(not(CONFIG_COREDUMP))]
#[macro_export]
macro_rules! coredump_report_failure { ($($arg:tt)*) => {}; }

#[cfg(all(CONFIG_COREDUMP, CONFIG_SYSCTL))]
unsafe extern "C" {
    pub fn validate_coredump_safety();
}

#[cfg(not(all(CONFIG_COREDUMP, CONFIG_SYSCTL)))]
pub unsafe extern "C" fn validate_coredump_safety() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
