// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) Sistina Software, Inc.  1997-2003 All rights reserved.
 * Copyright (C) 2004-2006 Red Hat, Inc.  All rights reserved.
 */

// Linux kernel headers and local headers from the C translation unit provide
// the types, constants, globals, and functions referenced below.

use core::ffi::c_void;

extern "C" {
    static mut gfs2_control_wq: *mut workqueue_struct;

    fn inode_init_once(inode: *mut inode);
    fn atomic_set(v: *mut atomic_t, i: i32);
    fn init_rwsem(sem: *mut rw_semaphore);
    fn INIT_LIST_HEAD(list: *mut list_head);
    fn gfs2_holder_mark_uninitialized(gh: *mut gfs2_holder);
    fn memset(s: *mut c_void, c: i32, n: usize) -> *mut c_void;
    fn RB_CLEAR_NODE(node: *mut rb_node);
    fn address_space_init_once(mapping: *mut address_space);
    fn gfs2_str2qstr(qstr: *mut qstr, s: *const i8);
    fn gfs2_quota_hash_init();
    fn gfs2_sys_init() -> i32;
    fn list_lru_init(lru: *mut list_lru) -> i32;
    fn gfs2_glock_init() -> i32;
    fn kmem_cache_create(name: *const i8, size: usize, align: usize, flags: u32,
                          ctor: Option<unsafe extern "C" fn(*mut c_void)>) -> *mut kmem_cache;
    fn gfs2_qd_shrinker_init() -> i32;
    fn alloc_workqueue(name: *const i8, flags: u32, max_active: u32) -> *mut workqueue_struct;
    fn mempool_create_page_pool(min_nr: i32, gfp_mask: u32) -> *mut mempool;
    fn gfs2_register_debugfs();
    fn register_filesystem(fs: *mut file_system_type) -> i32;
    fn unregister_filesystem(fs: *mut file_system_type) -> i32;
    fn mempool_destroy(pool: *mut mempool);
    fn destroy_workqueue(wq: *mut workqueue_struct);
    fn gfs2_qd_shrinker_exit();
    fn kmem_cache_destroy(cache: *mut kmem_cache);
    fn gfs2_glock_exit();
    fn list_lru_destroy(lru: *mut list_lru);
    fn gfs2_sys_uninit();
    fn gfs2_unregister_debugfs();
    fn rcu_barrier();

    static mut gfs2_qdot: qstr;
    static mut gfs2_qdotdot: qstr;
    static mut gfs2_qd_lru: list_lru;
    static mut gfs2_glock_cachep: *mut kmem_cache;
    static mut gfs2_glock_aspace_cachep: *mut kmem_cache;
    static mut gfs2_inode_cachep: *mut kmem_cache;
    static mut gfs2_bufdata_cachep: *mut kmem_cache;
    static mut gfs2_rgrpd_cachep: *mut kmem_cache;
    static mut gfs2_quotad_cachep: *mut kmem_cache;
    static mut gfs2_qadata_cachep: *mut kmem_cache;
    static mut gfs2_trans_cachep: *mut kmem_cache;
    static mut gfs2_recovery_wq: *mut workqueue_struct;
    static mut gfs2_freeze_wq: *mut workqueue_struct;
    static mut gfs2_page_pool: *mut mempool;
    static mut gfs2_fs_type: file_system_type;
    static mut gfs2meta_fs_type: file_system_type;
}

const SLAB_RECLAIM_ACCOUNT: u32 = 0;
const SLAB_ACCOUNT: u32 = 0;
const WQ_MEM_RECLAIM: u32 = 0;
const WQ_FREEZABLE: u32 = 0;
const WQ_PERCPU: u32 = 0;
const WQ_UNBOUND: u32 = 0;
const ENOMEM: i32 = 12;

unsafe extern "C" fn gfs2_init_inode_once(foo: *mut c_void) {
    let ip = foo as *mut gfs2_inode;
    inode_init_once(&mut (*ip).i_inode);
    atomic_set(&mut (*ip).i_sizehint, 0);
    init_rwsem(&mut (*ip).i_rw_mutex);
    INIT_LIST_HEAD(&mut (*ip).i_ordered);
    (*ip).i_qadata = core::ptr::null_mut();
    gfs2_holder_mark_uninitialized(&mut (*ip).i_rgd_gh);
    memset(&mut (*ip).i_res as *mut _ as *mut c_void, 0, core::mem::size_of_val(&(*ip).i_res));
    RB_CLEAR_NODE(&mut (*ip).i_res.rs_node);
    (*ip).i_hash_cache = core::ptr::null_mut();
    gfs2_holder_mark_uninitialized(&mut (*ip).i_iopen_gh);
}

unsafe extern "C" fn gfs2_init_glock_once(foo: *mut c_void) {
    let gl = foo as *mut gfs2_glock;
    INIT_LIST_HEAD(&mut (*gl).gl_holders);
    INIT_LIST_HEAD(&mut (*gl).gl_ail_list);
    atomic_set(&mut (*gl).gl_ail_count, 0);
    atomic_set(&mut (*gl).gl_revokes, 0);
}

unsafe extern "C" fn gfs2_init_gl_aspace_once(foo: *mut c_void) {
    let gla = foo as *mut gfs2_glock_aspace;
    gfs2_init_glock_once(&mut (*gla).glock as *mut _ as *mut c_void);
    address_space_init_once(&mut (*gla).mapping);
}

unsafe fn init_gfs2_fs() -> i32 {
    let mut error: i32;
    gfs2_str2qstr(&mut gfs2_qdot, b".\0".as_ptr() as *const i8);
    gfs2_str2qstr(&mut gfs2_qdotdot, b"..\0".as_ptr() as *const i8);
    gfs2_quota_hash_init();
    error = gfs2_sys_init();
    if error != 0 { return error; }
    error = list_lru_init(&mut gfs2_qd_lru);
    if error != 0 { gfs2_sys_uninit(); return error; }
    error = gfs2_glock_init();
    if error != 0 { list_lru_destroy(&mut gfs2_qd_lru); gfs2_sys_uninit(); return error; }
    error = -ENOMEM;
    gfs2_glock_cachep = kmem_cache_create(b"gfs2_glock\0".as_ptr() as *const i8, core::mem::size_of::<gfs2_glock>(), 0, SLAB_RECLAIM_ACCOUNT, Some(gfs2_init_glock_once));
    if gfs2_glock_cachep.is_null() { gfs2_glock_exit(); list_lru_destroy(&mut gfs2_qd_lru); gfs2_sys_uninit(); return error; }
    gfs2_glock_aspace_cachep = kmem_cache_create(b"gfs2_glock(aspace)\0".as_ptr() as *const i8, core::mem::size_of::<gfs2_glock_aspace>(), 0, 0, Some(gfs2_init_gl_aspace_once));
    if gfs2_glock_aspace_cachep.is_null() { kmem_cache_destroy(gfs2_glock_cachep); gfs2_glock_exit(); list_lru_destroy(&mut gfs2_qd_lru); gfs2_sys_uninit(); return error; }
    gfs2_inode_cachep = kmem_cache_create(b"gfs2_inode\0".as_ptr() as *const i8, core::mem::size_of::<gfs2_inode>(), 0, SLAB_RECLAIM_ACCOUNT | SLAB_ACCOUNT, Some(gfs2_init_inode_once));
    if gfs2_inode_cachep.is_null() { kmem_cache_destroy(gfs2_glock_aspace_cachep); kmem_cache_destroy(gfs2_glock_cachep); gfs2_glock_exit(); list_lru_destroy(&mut gfs2_qd_lru); gfs2_sys_uninit(); return error; }
    gfs2_bufdata_cachep = kmem_cache_create(b"gfs2_bufdata\0".as_ptr() as *const i8, core::mem::size_of::<gfs2_bufdata>(), 0, 0, None);
    if gfs2_bufdata_cachep.is_null() { kmem_cache_destroy(gfs2_inode_cachep); kmem_cache_destroy(gfs2_glock_aspace_cachep); kmem_cache_destroy(gfs2_glock_cachep); gfs2_glock_exit(); list_lru_destroy(&mut gfs2_qd_lru); gfs2_sys_uninit(); return error; }
    gfs2_rgrpd_cachep = kmem_cache_create(b"gfs2_rgrpd\0".as_ptr() as *const i8, core::mem::size_of::<gfs2_rgrpd>(), 0, 0, None);
    if gfs2_rgrpd_cachep.is_null() { kmem_cache_destroy(gfs2_bufdata_cachep); kmem_cache_destroy(gfs2_inode_cachep); kmem_cache_destroy(gfs2_glock_aspace_cachep); kmem_cache_destroy(gfs2_glock_cachep); gfs2_glock_exit(); list_lru_destroy(&mut gfs2_qd_lru); gfs2_sys_uninit(); return error; }
    gfs2_quotad_cachep = kmem_cache_create(b"gfs2_quotad\0".as_ptr() as *const i8, core::mem::size_of::<gfs2_quota_data>(), 0, SLAB_RECLAIM_ACCOUNT, None);
    if gfs2_quotad_cachep.is_null() { kmem_cache_destroy(gfs2_rgrpd_cachep); kmem_cache_destroy(gfs2_bufdata_cachep); kmem_cache_destroy(gfs2_inode_cachep); kmem_cache_destroy(gfs2_glock_aspace_cachep); kmem_cache_destroy(gfs2_glock_cachep); gfs2_glock_exit(); list_lru_destroy(&mut gfs2_qd_lru); gfs2_sys_uninit(); return error; }
    gfs2_qadata_cachep = kmem_cache_create(b"gfs2_qadata\0".as_ptr() as *const i8, core::mem::size_of::<gfs2_qadata>(), 0, 0, None);
    if gfs2_qadata_cachep.is_null() { kmem_cache_destroy(gfs2_quotad_cachep); kmem_cache_destroy(gfs2_rgrpd_cachep); kmem_cache_destroy(gfs2_bufdata_cachep); kmem_cache_destroy(gfs2_inode_cachep); kmem_cache_destroy(gfs2_glock_aspace_cachep); kmem_cache_destroy(gfs2_glock_cachep); gfs2_glock_exit(); list_lru_destroy(&mut gfs2_qd_lru); gfs2_sys_uninit(); return error; }
    gfs2_trans_cachep = kmem_cache_create(b"gfs2_trans\0".as_ptr() as *const i8, core::mem::size_of::<gfs2_trans>(), 0, 0, None);
    if gfs2_trans_cachep.is_null() { kmem_cache_destroy(gfs2_qadata_cachep); kmem_cache_destroy(gfs2_quotad_cachep); kmem_cache_destroy(gfs2_rgrpd_cachep); kmem_cache_destroy(gfs2_bufdata_cachep); kmem_cache_destroy(gfs2_inode_cachep); kmem_cache_destroy(gfs2_glock_aspace_cachep); kmem_cache_destroy(gfs2_glock_cachep); gfs2_glock_exit(); list_lru_destroy(&mut gfs2_qd_lru); gfs2_sys_uninit(); return error; }
    error = gfs2_qd_shrinker_init();
    if error != 0 { kmem_cache_destroy(gfs2_trans_cachep); return error; }
    error = -ENOMEM;
    gfs2_recovery_wq = alloc_workqueue(b"gfs2_recovery\0".as_ptr() as *const i8, WQ_MEM_RECLAIM | WQ_FREEZABLE | WQ_PERCPU, 0);
    if gfs2_recovery_wq.is_null() { gfs2_qd_shrinker_exit(); kmem_cache_destroy(gfs2_trans_cachep); return error; }
    gfs2_control_wq = alloc_workqueue(b"gfs2_control\0".as_ptr() as *const i8, WQ_UNBOUND | WQ_FREEZABLE, 0);
    if gfs2_control_wq.is_null() { destroy_workqueue(gfs2_recovery_wq); gfs2_qd_shrinker_exit(); kmem_cache_destroy(gfs2_trans_cachep); return error; }
    gfs2_freeze_wq = alloc_workqueue(b"gfs2_freeze\0".as_ptr() as *const i8, WQ_PERCPU, 0);
    if gfs2_freeze_wq.is_null() { destroy_workqueue(gfs2_control_wq); destroy_workqueue(gfs2_recovery_wq); gfs2_qd_shrinker_exit(); kmem_cache_destroy(gfs2_trans_cachep); return error; }
    gfs2_page_pool = mempool_create_page_pool(64, 0);
    if gfs2_page_pool.is_null() { destroy_workqueue(gfs2_freeze_wq); destroy_workqueue(gfs2_control_wq); destroy_workqueue(gfs2_recovery_wq); gfs2_qd_shrinker_exit(); kmem_cache_destroy(gfs2_trans_cachep); return error; }
    gfs2_register_debugfs();
    error = register_filesystem(&mut gfs2_fs_type);
    if error != 0 { mempool_destroy(gfs2_page_pool); destroy_workqueue(gfs2_freeze_wq); destroy_workqueue(gfs2_control_wq); destroy_workqueue(gfs2_recovery_wq); gfs2_qd_shrinker_exit(); kmem_cache_destroy(gfs2_trans_cachep); return error; }
    error = register_filesystem(&mut gfs2meta_fs_type);
    if error != 0 { unregister_filesystem(&mut gfs2_fs_type); mempool_destroy(gfs2_page_pool); destroy_workqueue(gfs2_freeze_wq); destroy_workqueue(gfs2_control_wq); destroy_workqueue(gfs2_recovery_wq); gfs2_qd_shrinker_exit(); kmem_cache_destroy(gfs2_trans_cachep); return error; }
    0
}

unsafe fn exit_gfs2_fs() {
    gfs2_qd_shrinker_exit(); gfs2_glock_exit(); gfs2_unregister_debugfs();
    unregister_filesystem(&mut gfs2_fs_type); unregister_filesystem(&mut gfs2meta_fs_type);
    destroy_workqueue(gfs2_recovery_wq); destroy_workqueue(gfs2_control_wq); destroy_workqueue(gfs2_freeze_wq);
    list_lru_destroy(&mut gfs2_qd_lru); rcu_barrier(); mempool_destroy(gfs2_page_pool);
    kmem_cache_destroy(gfs2_trans_cachep); kmem_cache_destroy(gfs2_qadata_cachep); kmem_cache_destroy(gfs2_quotad_cachep);
    kmem_cache_destroy(gfs2_rgrpd_cachep); kmem_cache_destroy(gfs2_bufdata_cachep); kmem_cache_destroy(gfs2_inode_cachep);
    kmem_cache_destroy(gfs2_glock_aspace_cachep); kmem_cache_destroy(gfs2_glock_cachep); gfs2_sys_uninit();
}

// MODULE_DESCRIPTION("Global File System");
// MODULE_AUTHOR("Red Hat, Inc.");
// MODULE_LICENSE("GPL");
// module_init(init_gfs2_fs);
// module_exit(exit_gfs2_fs);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
