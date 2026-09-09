/* SPDX-License-Identifier: GPL-2.0 */

// Dependency equivalent of: #include <linux/pid_namespace.h>

// These items are available from the surrounding kernel translation.

#[cfg(all(feature = "CONFIG_SYSCTL", feature = "CONFIG_MEMFD_CREATE"))]
unsafe fn pid_mfd_noexec_dointvec_minmax(
    table: *const ctl_table,
    write: ::core::ffi::c_int,
    buf: *mut ::core::ffi::c_void,
    lenp: *mut usize,
    ppos: *mut loff_t,
) -> ::core::ffi::c_int {
    let ns: *mut pid_namespace = task_active_pid_ns(current);
    let mut table_copy: ctl_table;
    let mut err: ::core::ffi::c_int;
    let mut scope: ::core::ffi::c_int;
    let mut parent_scope: ::core::ffi::c_int;

    if write != 0 && !ns_capable((*ns).user_ns, CAP_SYS_ADMIN) {
        return -(EPERM as ::core::ffi::c_int);
    }

    table_copy = *table;

    /* You cannot set a lower enforcement value than your parent. */
    parent_scope = pidns_memfd_noexec_scope((*ns).parent);
    /* Equivalent to pidns_memfd_noexec_scope(ns). */
    scope = max(READ_ONCE((*ns).memfd_noexec_scope), parent_scope);

    table_copy.data = &mut scope as *mut _ as *mut ::core::ffi::c_void;
    table_copy.extra1 = &mut parent_scope as *mut _ as *mut ::core::ffi::c_void;

    err = proc_dointvec_minmax(&mut table_copy, write, buf, lenp, ppos);
    if err == 0 && write != 0 {
        WRITE_ONCE((*ns).memfd_noexec_scope, scope);
    }
    err
}

#[cfg(all(feature = "CONFIG_SYSCTL", feature = "CONFIG_MEMFD_CREATE"))]
static mut pid_ns_ctl_table_vm: [ctl_table; 1] = [ctl_table {
    procname: "memfd_noexec",
    data: unsafe {
        &mut init_pid_ns.memfd_noexec_scope as *mut _ as *mut ::core::ffi::c_void
    },
    maxlen: ::core::mem::size_of_val(&init_pid_ns.memfd_noexec_scope),
    mode: 0o644,
    proc_handler: Some(pid_mfd_noexec_dointvec_minmax),
    extra1: SYSCTL_ZERO,
    extra2: SYSCTL_TWO,
}];

#[cfg(all(feature = "CONFIG_SYSCTL", feature = "CONFIG_MEMFD_CREATE"))]
#[inline]
unsafe fn register_pid_ns_sysctl_table_vm() {
    register_sysctl("vm", pid_ns_ctl_table_vm.as_ptr());
}

#[cfg(not(all(feature = "CONFIG_SYSCTL", feature = "CONFIG_MEMFD_CREATE")))]
#[inline]
unsafe fn register_pid_ns_sysctl_table_vm() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
