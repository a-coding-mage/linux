/* SPDX-License-Identifier: GPL-2.0 */

// Declarations corresponding to the Linux headers included by the C source
// are supplied by other translation units.

#[cfg(feature = "CONFIG_COREDUMP")]
#[repr(C)]
pub struct core_vma_metadata {
    pub start: ::core::ffi::c_ulong,
    pub end: ::core::ffi::c_ulong,
    pub flags: vm_flags_t,
    pub dump_size: ::core::ffi::c_ulong,
    pub pgoff: ::core::ffi::c_ulong,
    pub file: *mut file,
}

#[cfg(feature = "CONFIG_COREDUMP")]
#[repr(C)]
pub struct coredump_params {
    pub siginfo: *const kernel_siginfo_t,
    pub file: *mut file,
    pub limit: ::core::ffi::c_ulong,
    // MMF_DUMP_FILTER_* bits, snapshot of mm->flags at dump start.
    pub mm_flags: ::core::ffi::c_ulong,
    // Snapshot of dumpable at dump start.
    pub dumpable: task_dumpable,
    pub cpu: ::core::ffi::c_int,
    pub written: loff_t,
    pub pos: loff_t,
    pub to_skip: loff_t,
    pub vma_count: ::core::ffi::c_int,
    pub vma_data_size: usize,
    pub vma_meta: *mut core_vma_metadata,
    pub pid: *mut pid,
}

#[cfg(feature = "CONFIG_COREDUMP")]
unsafe extern "C" {
    pub static mut core_file_note_size_limit: ::core::ffi::c_uint;

    // These are the only things you should do on a core-file: use only these
    // functions to write out all the necessary info.
    pub fn dump_skip_to(cprm: *mut coredump_params, to: ::core::ffi::c_ulong);
    pub fn dump_skip(cprm: *mut coredump_params, nr: usize);
    pub fn dump_emit(
        cprm: *mut coredump_params,
        addr: *const ::core::ffi::c_void,
        nr: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    pub fn dump_align(cprm: *mut coredump_params, align: ::core::ffi::c_int)
        -> ::core::ffi::c_int;
    pub fn dump_user_range(
        cprm: *mut coredump_params,
        start: ::core::ffi::c_ulong,
        len: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_int;
    pub fn vfs_coredump(siginfo: *const kernel_siginfo_t);
}

// Logging for the coredump code, ratelimited. The TGID and comm fields are
// added to the message. The variadic formatting and kernel logging machinery
// are provided by the surrounding kernel translation.
#[cfg(feature = "CONFIG_COREDUMP")]
#[macro_export]
macro_rules! __COREDUMP_PRINTK {
    ($level:expr, $format:expr $(, $arg:expr)*) => {{
        unsafe {
            printk_ratelimited!($level, concat!("coredump: %d(%*pE): ", $format, "\n"),
                task_tgid_vnr(current), 0, current, $($arg),*);
        }
    }};
}

#[cfg(feature = "CONFIG_COREDUMP")]
#[macro_export]
macro_rules! coredump_report {
    ($fmt:expr $(, $arg:expr)*) => {
        $crate::__COREDUMP_PRINTK!(KERN_INFO, $fmt $(, $arg)*);
    };
}

#[cfg(feature = "CONFIG_COREDUMP")]
#[macro_export]
macro_rules! coredump_report_failure {
    ($fmt:expr $(, $arg:expr)*) => {
        $crate::__COREDUMP_PRINTK!(KERN_WARNING, $fmt $(, $arg)*);
    };
}

#[cfg(not(feature = "CONFIG_COREDUMP"))]
pub unsafe extern "C" fn vfs_coredump(_siginfo: *const kernel_siginfo_t) {}

#[cfg(not(feature = "CONFIG_COREDUMP"))]
#[macro_export]
macro_rules! coredump_report { ($($arg:tt)*) => {}; }

#[cfg(not(feature = "CONFIG_COREDUMP"))]
#[macro_export]
macro_rules! coredump_report_failure { ($($arg:tt)*) => {}; }

#[cfg(all(feature = "CONFIG_COREDUMP", feature = "CONFIG_SYSCTL"))]
unsafe extern "C" {
    pub fn validate_coredump_safety();
}

#[cfg(not(all(feature = "CONFIG_COREDUMP", feature = "CONFIG_SYSCTL")))]
pub unsafe extern "C" fn validate_coredump_safety() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
