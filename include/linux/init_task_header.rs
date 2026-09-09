/* SPDX-License-Identifier: GPL-2.0 */

// Declarations supplied by the corresponding Linux kernel dependencies.

unsafe extern "C" {
    pub static mut init_files: files_struct;
    pub static mut init_fs: fs_struct;
    pub static mut userspace_init_fs: *mut fs_struct;
    pub static mut init_nsproxy: nsproxy;
}

// When CONFIG_VIRT_CPU_ACCOUNTING_NATIVE is not enabled, initialize the
// previous CPU time lock and structure.  The native-accounting configuration
// intentionally expands this initializer to nothing.
#[cfg(not(CONFIG_VIRT_CPU_ACCOUNTING_NATIVE))]
macro_rules! INIT_PREV_CPUTIME {
    ($x:ident) => {
        .prev_cputime = {
            .lock = __RAW_SPIN_LOCK_UNLOCKED!($x.prev_cputime.lock),
        },
    };
}

#[cfg(CONFIG_VIRT_CPU_ACCOUNTING_NATIVE)]
macro_rules! INIT_PREV_CPUTIME {
    ($x:ident) => {};
}

pub const INIT_TASK_COMM: &str = "swapper";

/* Attach to the thread_info data structure for proper alignment */
// C: #define __init_thread_info __section(".data..init_thread_info")
#[allow(non_upper_case_globals)]
pub const __init_thread_info: &str = ".data..init_thread_info";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
