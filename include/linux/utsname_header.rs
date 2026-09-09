/* SPDX-License-Identifier: GPL-2.0 */

// Declarations supplied by the corresponding Linux kernel headers:
// linux/sched.h, linux/nsproxy.h, linux/ns_common.h, linux/err.h,
// and linux/uts_namespace.h.

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum uts_proc {
    UTS_PROC_ARCH,
    UTS_PROC_OSTYPE,
    UTS_PROC_OSRELEASE,
    UTS_PROC_VERSION,
    UTS_PROC_HOSTNAME,
    UTS_PROC_DOMAINNAME,
}

#[cfg(CONFIG_SYSCTL)]
extern "C" {
    pub fn uts_proc_notify(proc: uts_proc);
}

#[cfg(not(CONFIG_SYSCTL))]
#[inline]
pub unsafe fn uts_proc_notify(_proc: uts_proc) {}

#[inline]
pub unsafe fn utsname() -> *mut new_utsname {
    &mut (*(*current).nsproxy).uts_ns.name
}

#[inline]
pub unsafe fn init_utsname() -> *mut new_utsname {
    &mut init_uts_ns.name
}

extern "C" {
    pub static mut uts_sem: rw_semaphore;
    pub static mut current: *mut task_struct;
    pub static mut init_uts_ns: uts_namespace;
}

// External types supplied by the included Linux kernel headers.
extern "C" {
    type new_utsname;
    type rw_semaphore;
    type task_struct;
    type uts_namespace;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
