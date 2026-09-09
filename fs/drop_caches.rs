// SPDX-License-Identifier: GPL-2.0
/*
 * Implement the manual drop-all-pagecache function
 */

// C dependencies supplied by the kernel headers and internal.h are intentionally
// left as external Rust symbols/types.

use core::ffi::{c_char, c_int, c_void};

static mut SYSCTL_DROP_CACHES: c_int = 0;

unsafe fn drop_pagecache_sb(sb: *mut super_block, _unused: *mut c_void) {
    let mut inode: *mut inode;
    let mut toput_inode: *mut inode = core::ptr::null_mut();

    spin_lock(&mut (*sb).s_inode_list_lock);
    list_for_each_entry!(inode, (*sb).s_inodes, i_sb_list) {
        spin_lock(&mut (*inode).i_lock);
        /*
         * We must skip inodes in unusual state. We may also skip
         * inodes without pages but we deliberately won't in case
         * we need to reschedule to avoid softlockups.
         */
        if (inode_state_read(inode) & (I_FREEING | I_WILL_FREE | I_NEW)) != 0
            || (mapping_empty((*inode).i_mapping) && !need_resched())
        {
            spin_unlock(&mut (*inode).i_lock);
            continue;
        }
        __iget(inode);
        spin_unlock(&mut (*inode).i_lock);
        spin_unlock(&mut (*sb).s_inode_list_lock);

        invalidate_mapping_pages((*inode).i_mapping, 0, -1);
        iput(toput_inode);
        toput_inode = inode;

        cond_resched();
        spin_lock(&mut (*sb).s_inode_list_lock);
    }
    spin_unlock(&mut (*sb).s_inode_list_lock);
    iput(toput_inode);
}

unsafe fn drop_caches_sysctl_handler(
    table: *const ctl_table,
    write: c_int,
    buffer: *mut c_void,
    length: *mut usize,
    ppos: *mut loff_t,
) -> c_int {
    let mut ret: c_int;

    ret = proc_dointvec_minmax(table, write, buffer, length, ppos);
    if ret != 0 {
        return ret;
    }
    if write != 0 {
        static mut STFU: c_int = 0;

        if (SYSCTL_DROP_CACHES & 1) != 0 {
            lru_add_drain_all();
            iterate_supers(Some(drop_pagecache_sb), core::ptr::null_mut());
            count_vm_event(DROP_PAGECACHE);
        }
        if (SYSCTL_DROP_CACHES & 2) != 0 {
            drop_slab();
            count_vm_event(DROP_SLAB);
        }
        if STFU == 0 {
            pr_info!(
                "%s (%d): drop_caches: %d\n",
                (*current).comm,
                task_pid_nr(current),
                SYSCTL_DROP_CACHES
            );
        }
        STFU |= SYSCTL_DROP_CACHES & 4;
    }
    0
}

static DROP_CACHES_TABLE: [ctl_table; 1] = [ctl_table {
    procname: b"drop_caches\0".as_ptr() as *const c_char,
    data: unsafe { &raw mut SYSCTL_DROP_CACHES as *mut c_void },
    maxlen: core::mem::size_of::<c_int>(),
    mode: 0o200,
    proc_handler: Some(drop_caches_sysctl_handler),
    extra1: SYSCTL_ONE,
    extra2: SYSCTL_FOUR,
}];

unsafe fn init_vm_drop_caches_sysctls() -> c_int {
    register_sysctl_init(
        b"vm\0".as_ptr() as *const c_char,
        DROP_CACHES_TABLE.as_ptr(),
    );
    0
}

// Equivalent to fs_initcall(init_vm_drop_caches_sysctls).
fs_initcall!(init_vm_drop_caches_sysctls);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
