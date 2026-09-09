// SPDX-License-Identifier: GPL-2.0-or-later
/* AFS client file system
 *
 * Copyright (C) 2002,5 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// Linux kernel headers and internal.h provide the declarations used here.
// CREATE_TRACE_POINTS

// MODULE_DESCRIPTION("AFS Client File System");
// MODULE_AUTHOR("Red Hat, Inc.");
// MODULE_LICENSE("GPL");

pub static mut afs_debug: u32 = 0;
// module_param_named(debug, afs_debug, uint, S_IWUSR | S_IRUGO);
// MODULE_PARM_DESC(debug, "AFS debugging mask");

static mut rootcell: *mut core::ffi::c_char = core::ptr::null_mut();

// module_param(rootcell, charp, 0);
// MODULE_PARM_DESC(rootcell, "root AFS cell name and VL server IP addr list");

pub static mut afs_wq: *mut workqueue_struct = core::ptr::null_mut();
static mut afs_proc_symlink: *mut proc_dir_entry = core::ptr::null_mut();

#[cfg(CONFIG_ALPHA)]
pub static afs_init_sysname: &[u8] = b"alpha_linux26\0";
#[cfg(all(not(CONFIG_ALPHA), CONFIG_X86_64))]
pub static afs_init_sysname: &[u8] = b"amd64_linux26\0";
#[cfg(all(not(CONFIG_ALPHA), not(CONFIG_X86_64), CONFIG_ARM))]
pub static afs_init_sysname: &[u8] = b"arm_linux26\0";
#[cfg(all(not(CONFIG_ALPHA), not(CONFIG_X86_64), not(CONFIG_ARM), CONFIG_ARM64))]
pub static afs_init_sysname: &[u8] = b"aarch64_linux26\0";
#[cfg(all(not(CONFIG_ALPHA), not(CONFIG_X86_64), not(CONFIG_ARM), not(CONFIG_ARM64), CONFIG_X86_32))]
pub static afs_init_sysname: &[u8] = b"i386_linux26\0";
#[cfg(all(not(CONFIG_ALPHA), not(CONFIG_X86_64), not(CONFIG_ARM), not(CONFIG_ARM64), not(CONFIG_X86_32), CONFIG_PPC64))]
pub static afs_init_sysname: &[u8] = b"ppc64_linux26\0";
#[cfg(all(not(CONFIG_ALPHA), not(CONFIG_X86_64), not(CONFIG_ARM), not(CONFIG_ARM64), not(CONFIG_X86_32), not(CONFIG_PPC64), CONFIG_PPC32))]
pub static afs_init_sysname: &[u8] = b"ppc_linux26\0";
#[cfg(all(not(CONFIG_ALPHA), not(CONFIG_X86_64), not(CONFIG_ARM), not(CONFIG_ARM64), not(CONFIG_X86_32), not(CONFIG_PPC64), not(CONFIG_PPC32), CONFIG_S390, CONFIG_64BIT))]
pub static afs_init_sysname: &[u8] = b"s390x_linux26\0";
#[cfg(all(not(CONFIG_ALPHA), not(CONFIG_X86_64), not(CONFIG_ARM), not(CONFIG_ARM64), not(CONFIG_X86_32), not(CONFIG_PPC64), not(CONFIG_PPC32), CONFIG_S390, not(CONFIG_64BIT)))]
pub static afs_init_sysname: &[u8] = b"s390_linux26\0";
#[cfg(all(not(CONFIG_ALPHA), not(CONFIG_X86_64), not(CONFIG_ARM), not(CONFIG_ARM64), not(CONFIG_X86_32), not(CONFIG_PPC64), not(CONFIG_PPC32), not(CONFIG_S390), CONFIG_SPARC64))]
pub static afs_init_sysname: &[u8] = b"sparc64_linux26\0";
#[cfg(all(not(CONFIG_ALPHA), not(CONFIG_X86_64), not(CONFIG_ARM), not(CONFIG_ARM64), not(CONFIG_X86_32), not(CONFIG_PPC64), not(CONFIG_PPC32), not(CONFIG_S390), not(CONFIG_SPARC64), CONFIG_SPARC32))]
pub static afs_init_sysname: &[u8] = b"sparc_linux26\0";
#[cfg(not(any(CONFIG_ALPHA, CONFIG_X86_64, CONFIG_ARM, CONFIG_ARM64, CONFIG_X86_32, CONFIG_PPC64, CONFIG_PPC32, CONFIG_S390, CONFIG_SPARC64, CONFIG_SPARC32)))]
pub static afs_init_sysname: &[u8] = b"unknown_linux26\0";

/* Initialise an AFS network namespace record. */
unsafe fn afs_net_init(net_ns: *mut net) -> i32 {
    let mut sysnames: *mut afs_sysnames;
    let net: *mut afs_net = afs_net(net_ns);
    let mut ret: i32;

    (*net).net = net_ns;
    (*net).live = true;
    generate_random_uuid((&mut (*net).uuid as *mut _).cast::<u8>());
    INIT_WORK(&mut (*net).charge_preallocation_work, afs_charge_preallocation);
    INIT_WORK(&mut (*net).rx_oob_work, afs_process_oob_queue);
    mutex_init(&mut (*net).socket_mutex);
    (*net).cells = RB_ROOT;
    idr_init(&mut (*net).cells_dyn_ino);
    init_rwsem(&mut (*net).cells_lock);
    mutex_init(&mut (*net).cells_alias_lock);
    mutex_init(&mut (*net).proc_cells_lock);
    INIT_HLIST_HEAD(&mut (*net).proc_cells);
    seqlock_init(&mut (*net).fs_lock);
    INIT_LIST_HEAD(&mut (*net).fs_probe_fast);
    INIT_LIST_HEAD(&mut (*net).fs_probe_slow);
    INIT_HLIST_HEAD(&mut (*net).fs_proc);
    INIT_WORK(&mut (*net).fs_prober, afs_fs_probe_dispatcher);
    timer_setup(&mut (*net).fs_probe_timer, afs_fs_probe_timer, 0);
    atomic_set(&mut (*net).servers_outstanding, 1);

    ret = -ENOMEM;
    sysnames = kzalloc_obj();
    if sysnames.is_null() { return ret; }
    (*sysnames).subs[0] = afs_init_sysname.as_ptr() as *mut core::ffi::c_char;
    (*sysnames).nr = 1;
    refcount_set(&mut (*sysnames).usage, 1);
    (*net).sysnames = sysnames;
    rwlock_init(&mut (*net).sysnames_lock);
    ret = afs_proc_init(net);
    if ret < 0 { afs_put_sysnames((*net).sysnames); idr_destroy(&mut (*net).cells_dyn_ino); (*net).live = false; return ret; }
    ret = afs_cell_init(net, rootcell);
    if ret < 0 { (*net).live = false; afs_proc_cleanup(net); afs_put_sysnames((*net).sysnames); idr_destroy(&mut (*net).cells_dyn_ino); return ret; }
    ret = afs_open_socket(net);
    if ret < 0 { (*net).live = false; afs_fs_probe_cleanup(net); afs_cell_purge(net); afs_wait_for_servers(net); (*net).live = false; afs_proc_cleanup(net); afs_put_sysnames((*net).sysnames); idr_destroy(&mut (*net).cells_dyn_ino); return ret; }
    0
}

/* Clean up and destroy an AFS network namespace record. */
unsafe fn afs_net_exit(net_ns: *mut net) {
    let net = afs_net(net_ns);
    (*net).live = false;
    afs_fs_probe_cleanup(net); afs_cell_purge(net); afs_wait_for_servers(net);
    afs_close_socket(net); afs_proc_cleanup(net); afs_put_sysnames((*net).sysnames);
    idr_destroy(&mut (*net).cells_dyn_ino);
    kfree_rcu(rcu_access_pointer((*net).address_prefs), rcu);
}

// The remaining module registration and teardown are kernel macros/calls;
// their direct declarations and ordering are preserved here.
unsafe fn afs_init() -> i32 {
    let mut ret = -ENOMEM;
    printk(KERN_INFO, "kAFS: Red Hat AFS client v0.1 registering.\n");
    afs_wq = alloc_workqueue(b"afs\0".as_ptr(), WQ_PERCPU, 0);
    if afs_wq.is_null() { return ret; }
    afs_async_calls = alloc_workqueue(b"kafsd\0".as_ptr(), WQ_MEM_RECLAIM | WQ_UNBOUND, 0);
    if afs_async_calls.is_null() { destroy_workqueue(afs_wq); return ret; }
    afs_lock_manager = alloc_workqueue(b"kafs_lockd\0".as_ptr(), WQ_MEM_RECLAIM | WQ_PERCPU, 0);
    if afs_lock_manager.is_null() { destroy_workqueue(afs_async_calls); destroy_workqueue(afs_wq); return ret; }
    ret = register_pernet_device(&mut afs_net_ops);
    if ret < 0 { destroy_workqueue(afs_lock_manager); destroy_workqueue(afs_async_calls); destroy_workqueue(afs_wq); rcu_barrier(); return ret; }
    ret = afs_fs_init();
    if ret < 0 { unregister_pernet_device(&mut afs_net_ops); destroy_workqueue(afs_lock_manager); destroy_workqueue(afs_async_calls); destroy_workqueue(afs_wq); rcu_barrier(); return ret; }
    afs_proc_symlink = proc_symlink(b"fs/afs\0".as_ptr(), core::ptr::null_mut(), b"../self/net/afs\0".as_ptr());
    if afs_proc_symlink.is_null() { afs_fs_exit(); unregister_pernet_device(&mut afs_net_ops); destroy_workqueue(afs_lock_manager); destroy_workqueue(afs_async_calls); destroy_workqueue(afs_wq); rcu_barrier(); return -ENOMEM; }
    ret
}

unsafe fn afs_exit() {
    printk(KERN_INFO, "kAFS: Red Hat AFS client v0.1 unregistering.\n");
    proc_remove(afs_proc_symlink); afs_fs_exit(); unregister_pernet_device(&mut afs_net_ops);
    destroy_workqueue(afs_lock_manager); destroy_workqueue(afs_async_calls); destroy_workqueue(afs_wq);
    afs_clean_up_permit_cache(); rcu_barrier();
}

// late_initcall(afs_init);
// module_exit(afs_exit);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
