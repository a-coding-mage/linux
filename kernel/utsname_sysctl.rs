// SPDX-License-Identifier: GPL-2.0-only
/*
 *  Copyright (C) 2007
 *
 *  Author: Eric Biederman <ebiederm@xmision.com>
 */

// Translated dependencies supplied by the surrounding kernel build.

#[cfg(CONFIG_SYSCTL)]
unsafe fn get_uts(table: *const ctl_table) -> *mut core::ffi::c_void {
    let mut which = (*table).data as *mut u8;
    let uts_ns: *mut uts_namespace = (*current).nsproxy.as_ref().unwrap().uts_ns;

    which = (which as usize - (&init_uts_ns as *const uts_namespace as usize))
        .wrapping_add(uts_ns as usize) as *mut u8;

    which as *mut core::ffi::c_void
}

/*
 * Special case of dostring for the UTS structure. This has locks
 * to observe. Should this be in kernel/sys.c ????
 */
#[cfg(CONFIG_SYSCTL)]
unsafe fn proc_do_uts_string(
    table: *const ctl_table,
    write: i32,
    buffer: *mut core::ffi::c_void,
    lenp: *mut usize,
    ppos: *mut loff_t,
) -> i32 {
    let mut uts_table: ctl_table = core::ptr::read(table);
    let mut r: i32;
    let mut tmp_data = [0u8; __NEW_UTS_LEN as usize + 1];

    uts_table.data = tmp_data.as_mut_ptr() as *mut core::ffi::c_void;

    /*
     * Buffer the value in tmp_data so that proc_dostring() can be called
     * without holding any locks.
     * We also need to read the original value in the write==1 case to
     * support partial writes.
     */
    down_read(&raw const uts_sem);
    core::ptr::copy_nonoverlapping(
        get_uts(table) as *const u8,
        tmp_data.as_mut_ptr(),
        tmp_data.len(),
    );
    up_read(&raw const uts_sem);
    r = proc_dostring(&raw const uts_table, write, buffer, lenp, ppos);

    if write != 0 {
        /*
         * Write back the new value.
         * Note that, since we dropped uts_sem, the result can
         * theoretically be incorrect if there are two parallel writes
         * at non-zero offsets to the same sysctl.
         */
        add_device_randomness(tmp_data.as_ptr() as *const core::ffi::c_void, tmp_data.len());
        down_write(&raw const uts_sem);
        core::ptr::copy_nonoverlapping(
            tmp_data.as_ptr(),
            get_uts(table) as *mut u8,
            tmp_data.len(),
        );
        up_write(&raw const uts_sem);
        proc_sys_poll_notify((*table).poll);
    }

    r
}

#[cfg(not(CONFIG_SYSCTL))]
const proc_do_uts_string: Option<unsafe extern "C" fn(*const ctl_table, i32, *mut core::ffi::c_void, *mut usize, *mut loff_t) -> i32> = None;

static mut hostname_poll: ctl_table_poll = DEFINE_CTL_TABLE_POLL!();
static mut domainname_poll: ctl_table_poll = DEFINE_CTL_TABLE_POLL!();

// Note: update `enum uts_proc` to match any changes to this table
static uts_kern_table: [ctl_table; 6] = [
    ctl_table { procname: c"arch".as_ptr(), data: unsafe { &raw const init_uts_ns.name.machine as *const _ as *mut _ }, maxlen: core::mem::size_of_val(unsafe { &init_uts_ns.name.machine }), mode: 0o444, proc_handler: Some(proc_do_uts_string), poll: core::ptr::null_mut() },
    ctl_table { procname: c"ostype".as_ptr(), data: unsafe { &raw const init_uts_ns.name.sysname as *const _ as *mut _ }, maxlen: core::mem::size_of_val(unsafe { &init_uts_ns.name.sysname }), mode: 0o444, proc_handler: Some(proc_do_uts_string), poll: core::ptr::null_mut() },
    ctl_table { procname: c"osrelease".as_ptr(), data: unsafe { &raw const init_uts_ns.name.release as *const _ as *mut _ }, maxlen: core::mem::size_of_val(unsafe { &init_uts_ns.name.release }), mode: 0o444, proc_handler: Some(proc_do_uts_string), poll: core::ptr::null_mut() },
    ctl_table { procname: c"version".as_ptr(), data: unsafe { &raw const init_uts_ns.name.version as *const _ as *mut _ }, maxlen: core::mem::size_of_val(unsafe { &init_uts_ns.name.version }), mode: 0o444, proc_handler: Some(proc_do_uts_string), poll: core::ptr::null_mut() },
    ctl_table { procname: c"hostname".as_ptr(), data: unsafe { &raw const init_uts_ns.name.nodename as *const _ as *mut _ }, maxlen: core::mem::size_of_val(unsafe { &init_uts_ns.name.nodename }), mode: 0o644, proc_handler: Some(proc_do_uts_string), poll: unsafe { &raw mut hostname_poll } },
    ctl_table { procname: c"domainname".as_ptr(), data: unsafe { &raw const init_uts_ns.name.domainname as *const _ as *mut _ }, maxlen: core::mem::size_of_val(unsafe { &init_uts_ns.name.domainname }), mode: 0o644, proc_handler: Some(proc_do_uts_string), poll: unsafe { &raw mut domainname_poll } },
];

#[cfg(CONFIG_SYSCTL)]
unsafe fn uts_proc_notify(proc: uts_proc) {
    let table = &uts_kern_table[proc as usize] as *const ctl_table;
    proc_sys_poll_notify((*table).poll);
}

unsafe fn utsname_sysctl_init() -> i32 {
    register_sysctl(c"kernel".as_ptr(), uts_kern_table.as_ptr());
    0
}

// device_initcall(utsname_sysctl_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
