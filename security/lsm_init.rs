// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * LSM initialization functions
 */

// pr_fmt(fmt) prefixes messages with "LSM: " in the C source.
// Includes translated as external dependencies: linux/init.h,
// linux/lsm_hooks.h, and "lsm.h".

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

extern "C" {
    static mut __start_lsm_info: lsm_info;
    static mut __end_lsm_info: lsm_info;
    static mut __start_early_lsm_info: lsm_info;
    static mut __end_early_lsm_info: lsm_info;

    static mut lsm_debug: bool;
    static mut lsm_active_cnt: c_uint;
    static mut lsm_idlist: [*const lsm_id; MAX_LSM_COUNT as usize];
    static mut blob_sizes: lsm_blob_sizes;
    static mut lsm_file_cache: *mut c_void;
    static mut lsm_backing_file_cache: *mut c_void;
    static mut lsm_inode_cache: *mut c_void;
    static mut current: *mut task_struct;

    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn kstrdup(s: *const c_char, flags: gfp_t) -> *mut c_char;
    fn strsep(stringp: *mut *mut c_char, delim: *const c_char) -> *mut c_char;
    fn kfree(ptr: *const c_void);
    fn kmem_cache_create(
        name: *const c_char,
        size: c_uint,
        align: c_uint,
        flags: c_uint,
        ctor: *mut c_void,
    ) -> *mut c_void;
    fn lsm_cred_alloc(cred: *mut cred, gfp: gfp_t) -> c_int;
    fn unrcu_pointer(ptr: *mut cred) -> *mut cred;
    fn lsm_task_alloc(task: *mut task_struct) -> c_int;
    fn securityfs_init() -> c_int;
    fn call_blocking_lsm_notifier(val: c_uint, v: *mut c_void) -> c_int;
    fn __static_call_update(key: *mut c_void, trampoline: *mut c_void, func: *mut c_void);
    fn static_branch_enable(key: *mut c_void);
    fn panic(fmt: *const c_char, ...);
    fn pr_warn(fmt: *const c_char, ...);
    fn lsm_pr(fmt: *const c_char, ...);
    fn lsm_pr_cont(fmt: *const c_char, ...);
    fn lsm_pr_dbg(fmt: *const c_char, ...);
    fn WARN(condition: c_int, fmt: *const c_char, ...) -> c_int;
}

type gfp_t = c_uint;

const CONFIG_LSM: *const c_char = b"\0".as_ptr() as *const c_char;
const MAX_LSM_COUNT: c_uint = 0;
const GFP_KERNEL: gfp_t = 0;
const SLAB_PANIC: c_uint = 0;
const ENOSPC: c_int = 28;
const LSM_STARTED_ALL: c_uint = 0;
const LSM_FLAG_EXCLUSIVE: c_uint = 1 << 0;
const LSM_FLAG_LEGACY_MAJOR: c_uint = 1 << 1;
const LSM_ORDER_FIRST: c_int = 0;
const LSM_ORDER_MUTABLE: c_int = 1;
const LSM_ORDER_LAST: c_int = 2;

#[repr(C)]
pub struct lsm_id {
    pub name: *const c_char,
}

#[repr(C)]
pub struct lsm_blob_sizes {
    pub lbs_cred: c_uint,
    pub lbs_file: c_uint,
    pub lbs_backing_file: c_uint,
    pub lbs_ib: c_uint,
    pub lbs_inode: c_uint,
    pub lbs_ipc: c_uint,
    pub lbs_key: c_uint,
    pub lbs_msg_msg: c_uint,
    pub lbs_perf_event: c_uint,
    pub lbs_sock: c_uint,
    pub lbs_superblock: c_uint,
    pub lbs_task: c_uint,
    pub lbs_tun_dev: c_uint,
    pub lbs_xattr_count: c_uint,
    pub lbs_bdev: c_uint,
    pub lbs_bpf_map: c_uint,
    pub lbs_bpf_prog: c_uint,
    pub lbs_bpf_token: c_uint,
}

#[repr(C)]
pub struct lsm_info {
    pub id: *const lsm_id,
    pub enabled: *mut bool,
    pub flags: c_uint,
    pub order: c_int,
    pub blobs: *mut lsm_blob_sizes,
    pub init: unsafe extern "C" fn() -> c_int,
    pub initcall_pure: Option<unsafe extern "C" fn() -> c_int>,
    pub initcall_early: Option<unsafe extern "C" fn() -> c_int>,
    pub initcall_core: Option<unsafe extern "C" fn() -> c_int>,
    pub initcall_subsys: Option<unsafe extern "C" fn() -> c_int>,
    pub initcall_fs: Option<unsafe extern "C" fn() -> c_int>,
    pub initcall_device: Option<unsafe extern "C" fn() -> c_int>,
    pub initcall_late: Option<unsafe extern "C" fn() -> c_int>,
    pub initcall_late_sync: Option<unsafe extern "C" fn() -> c_int>,
}

#[repr(C)]
pub struct security_hook_list {
    pub lsmid: *const lsm_id,
    pub scalls: *mut lsm_static_call,
    pub hook: security_hook,
}

#[repr(C)]
pub struct security_hook {
    pub lsm_func_addr: *mut c_void,
}

#[repr(C)]
pub struct lsm_static_call {
    pub hl: *mut security_hook_list,
    pub key: *mut c_void,
    pub trampoline: *mut c_void,
    pub active: *mut c_void,
}

#[repr(C)]
pub struct rcu_head {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cred {
    _private: [u8; 0],
}

#[repr(C)]
pub struct task_struct {
    pub cred: *mut cred,
}

/* LSM enabled constants. */
static mut lsm_enabled_true: bool = true;
static mut lsm_enabled_false: bool = false;

/* Number of "early" LSMs */
static mut lsm_count_early: c_uint = 0;

/* Build and boot-time LSM ordering. */
static lsm_order_builtin: *const c_char = CONFIG_LSM;
static mut lsm_order_cmdline: *const c_char = ptr::null();
static mut lsm_order_legacy: *const c_char = ptr::null();

/* Ordered list of LSMs to initialize. */
static mut lsm_order: [*mut lsm_info; MAX_LSM_COUNT as usize + 1] =
    [ptr::null_mut(); MAX_LSM_COUNT as usize + 1];
static mut lsm_exclusive: *mut lsm_info = ptr::null_mut();

unsafe fn align(value: c_uint, align_to: usize) -> c_uint {
    let mask = align_to as c_uint - 1;
    (value + mask) & !mask
}

unsafe fn lsm_initcall(initcall: unsafe fn(*mut lsm_info) -> Option<unsafe extern "C" fn() -> c_int>, level: *const c_char) -> c_int {
    let mut _r: c_int;
    let mut _rc: c_int = 0;
    let mut _lp = lsm_order.as_mut_ptr();

    while !(*_lp).is_null() {
        let _l = *_lp;
        let call = initcall(_l);
        if call.is_none() {
            _lp = _lp.add(1);
            continue;
        }
        lsm_pr_dbg(
            b"running %s %s initcall\0".as_ptr() as *const c_char,
            (*(*_l).id).name,
            level,
        );
        _r = call.unwrap()();
        if _r != 0 {
            pr_warn(
                b"failed LSM %s %s initcall with errno %d\n\0".as_ptr() as *const c_char,
                (*(*_l).id).name,
                level,
                _r,
            );
            if _rc == 0 {
                _rc = _r;
            }
        }
        _lp = _lp.add(1);
    }

    _rc
}

/**
 * lsm_choose_security - Legacy "major" LSM selection
 * @str: kernel command line parameter
 */
unsafe extern "C" fn lsm_choose_security(str_: *mut c_char) -> c_int {
    lsm_order_legacy = str_;
    1
}
// __setup("security=", lsm_choose_security);

/**
 * lsm_choose_lsm - Modern LSM selection
 * @str: kernel command line parameter
 */
unsafe extern "C" fn lsm_choose_lsm(str_: *mut c_char) -> c_int {
    lsm_order_cmdline = str_;
    1
}
// __setup("lsm=", lsm_choose_lsm);

/**
 * lsm_debug_enable - Enable LSM framework debugging
 * @str: kernel command line parameter
 *
 * Currently we only provide debug info during LSM initialization, but we may
 * want to expand this in the future.
 */
unsafe extern "C" fn lsm_debug_enable(_str: *mut c_char) -> c_int {
    lsm_debug = true;
    1
}
// __setup("lsm.debug", lsm_debug_enable);

/**
 * lsm_enabled_set - Mark a LSM as enabled
 * @lsm: LSM definition
 * @enabled: enabled flag
 */
unsafe fn lsm_enabled_set(lsm: *mut lsm_info, enabled: bool) {
    /*
     * When an LSM hasn't configured an enable variable, we can use
     * a hard-coded location for storing the default enabled state.
     */
    if (*lsm).enabled.is_null()
        || (*lsm).enabled == &mut lsm_enabled_true
        || (*lsm).enabled == &mut lsm_enabled_false
    {
        (*lsm).enabled = if enabled {
            &mut lsm_enabled_true
        } else {
            &mut lsm_enabled_false
        };
    } else {
        *(*lsm).enabled = enabled;
    }
}

/**
 * lsm_is_enabled - Determine if a LSM is enabled
 * @lsm: LSM definition
 */
unsafe fn lsm_is_enabled(lsm: *mut lsm_info) -> bool {
    if !(*lsm).enabled.is_null() {
        *(*lsm).enabled
    } else {
        false
    }
}

/**
 * lsm_order_exists - Determine if a LSM exists in the ordered list
 * @lsm: LSM definition
 */
unsafe fn lsm_order_exists(lsm: *mut lsm_info) -> bool {
    let mut check = lsm_order.as_mut_ptr();

    while !(*check).is_null() {
        if *check == lsm {
            return true;
        }
        check = check.add(1);
    }

    false
}

/**
 * lsm_order_append - Append a LSM to the ordered list
 * @lsm: LSM definition
 * @src: source of the addition
 *
 * Append @lsm to the enabled LSM array after ensuring that it hasn't been
 * explicitly disabled, is a duplicate entry, or would run afoul of the
 * LSM_FLAG_EXCLUSIVE logic.
 */
unsafe fn lsm_order_append(lsm: *mut lsm_info, src: *const c_char) {
    /* Ignore duplicate selections. */
    if lsm_order_exists(lsm) {
        return;
    }

    /* Skip explicitly disabled LSMs. */
    if !(*lsm).enabled.is_null() && !lsm_is_enabled(lsm) {
        lsm_pr_dbg(
            b"skip previously disabled LSM %s:%s\n\0".as_ptr() as *const c_char,
            src,
            (*(*lsm).id).name,
        );
        return;
    }

    if lsm_active_cnt == MAX_LSM_COUNT {
        pr_warn(
            b"exceeded maximum LSM count on %s:%s\n\0".as_ptr() as *const c_char,
            src,
            (*(*lsm).id).name,
        );
        lsm_enabled_set(lsm, false);
        return;
    }

    if ((*lsm).flags & LSM_FLAG_EXCLUSIVE) != 0 {
        if !lsm_exclusive.is_null() {
            lsm_pr_dbg(
                b"skip exclusive LSM conflict %s:%s\n\0".as_ptr() as *const c_char,
                src,
                (*(*lsm).id).name,
            );
            lsm_enabled_set(lsm, false);
            return;
        } else {
            lsm_pr_dbg(
                b"select exclusive LSM %s:%s\n\0".as_ptr() as *const c_char,
                src,
                (*(*lsm).id).name,
            );
            lsm_exclusive = lsm;
        }
    }

    lsm_enabled_set(lsm, true);
    lsm_order[lsm_active_cnt as usize] = lsm;
    lsm_idlist[lsm_active_cnt as usize] = (*lsm).id;
    lsm_active_cnt += 1;

    lsm_pr_dbg(
        b"enabling LSM %s:%s\n\0".as_ptr() as *const c_char,
        src,
        (*(*lsm).id).name,
    );
}

/**
 * lsm_order_parse - Parse the comma delimited LSM list
 * @list: LSM list
 * @src: source of the list
 */
unsafe fn lsm_order_parse(list: *const c_char, src: *const c_char) {
    let mut lsm: *mut lsm_info;
    let mut sep: *mut c_char;
    let mut name: *mut c_char;
    let mut next: *mut c_char;

    /* Handle any Legacy LSM exclusions if one was specified. */
    if !lsm_order_legacy.is_null() {
        /*
         * To match the original "security=" behavior, this explicitly
         * does NOT fallback to another Legacy Major if the selected
         * one was separately disabled: disable all non-matching
         * Legacy Major LSMs.
         */
        lsm = &mut __start_lsm_info;
        while lsm < &mut __end_lsm_info {
            if ((*lsm).flags & LSM_FLAG_LEGACY_MAJOR) != 0
                && strcmp((*(*lsm).id).name, lsm_order_legacy) != 0
            {
                lsm_enabled_set(lsm, false);
                lsm_pr_dbg(
                    b"skip legacy LSM conflict %s:%s\n\0".as_ptr() as *const c_char,
                    src,
                    (*(*lsm).id).name,
                );
            }
            lsm = lsm.add(1);
        }
    }

    /* LSM_ORDER_FIRST */
    lsm = &mut __start_lsm_info;
    while lsm < &mut __end_lsm_info {
        if (*lsm).order == LSM_ORDER_FIRST {
            lsm_order_append(lsm, b"first\0".as_ptr() as *const c_char);
        }
        lsm = lsm.add(1);
    }

    /* Normal or "mutable" LSMs */
    sep = kstrdup(list, GFP_KERNEL);
    next = sep;
    /* Walk the list, looking for matching LSMs. */
    loop {
        name = strsep(&mut next, b",\0".as_ptr() as *const c_char);
        if name.is_null() {
            break;
        }
        lsm = &mut __start_lsm_info;
        while lsm < &mut __end_lsm_info {
            if strcmp((*(*lsm).id).name, name) == 0 && (*lsm).order == LSM_ORDER_MUTABLE {
                lsm_order_append(lsm, src);
            }
            lsm = lsm.add(1);
        }
    }
    kfree(sep as *const c_void);

    /* Legacy LSM if specified. */
    if !lsm_order_legacy.is_null() {
        lsm = &mut __start_lsm_info;
        while lsm < &mut __end_lsm_info {
            if strcmp((*(*lsm).id).name, lsm_order_legacy) == 0 {
                lsm_order_append(lsm, src);
            }
            lsm = lsm.add(1);
        }
    }

    /* LSM_ORDER_LAST */
    lsm = &mut __start_lsm_info;
    while lsm < &mut __end_lsm_info {
        if (*lsm).order == LSM_ORDER_LAST {
            lsm_order_append(lsm, b"last\0".as_ptr() as *const c_char);
        }
        lsm = lsm.add(1);
    }

    /* Disable all LSMs not previously enabled. */
    lsm = &mut __start_lsm_info;
    while lsm < &mut __end_lsm_info {
        if lsm_order_exists(lsm) {
            lsm = lsm.add(1);
            continue;
        }
        lsm_enabled_set(lsm, false);
        lsm_pr_dbg(
            b"skip disabled LSM %s:%s\n\0".as_ptr() as *const c_char,
            src,
            (*(*lsm).id).name,
        );
        lsm = lsm.add(1);
    }
}

/**
 * lsm_blob_size_update - Update the LSM blob size and offset information
 * @sz_req: the requested additional blob size
 * @sz_cur: the existing blob size
 */
unsafe fn lsm_blob_size_update(sz_req: *mut c_uint, sz_cur: *mut c_uint) {
    let offset: c_uint;

    if *sz_req == 0 {
        return;
    }

    offset = align(*sz_cur, size_of::<*mut c_void>());
    *sz_cur = offset + *sz_req;
    *sz_req = offset;
}

/**
 * lsm_prepare - Prepare the LSM framework for a new LSM
 * @lsm: LSM definition
 */
unsafe fn lsm_prepare(lsm: *mut lsm_info) {
    let mut blobs: *mut lsm_blob_sizes = (*lsm).blobs;

    if blobs.is_null() {
        return;
    }

    /* Register the LSM blob sizes. */
    blobs = (*lsm).blobs;
    lsm_blob_size_update(&mut (*blobs).lbs_cred, &mut blob_sizes.lbs_cred);
    lsm_blob_size_update(&mut (*blobs).lbs_file, &mut blob_sizes.lbs_file);
    lsm_blob_size_update(&mut (*blobs).lbs_backing_file, &mut blob_sizes.lbs_backing_file);
    lsm_blob_size_update(&mut (*blobs).lbs_ib, &mut blob_sizes.lbs_ib);
    /* inode blob gets an rcu_head in addition to LSM blobs. */
    if (*blobs).lbs_inode != 0 && blob_sizes.lbs_inode == 0 {
        blob_sizes.lbs_inode = size_of::<rcu_head>() as c_uint;
    }
    lsm_blob_size_update(&mut (*blobs).lbs_inode, &mut blob_sizes.lbs_inode);
    lsm_blob_size_update(&mut (*blobs).lbs_ipc, &mut blob_sizes.lbs_ipc);
    lsm_blob_size_update(&mut (*blobs).lbs_key, &mut blob_sizes.lbs_key);
    lsm_blob_size_update(&mut (*blobs).lbs_msg_msg, &mut blob_sizes.lbs_msg_msg);
    lsm_blob_size_update(&mut (*blobs).lbs_perf_event, &mut blob_sizes.lbs_perf_event);
    lsm_blob_size_update(&mut (*blobs).lbs_sock, &mut blob_sizes.lbs_sock);
    lsm_blob_size_update(&mut (*blobs).lbs_superblock, &mut blob_sizes.lbs_superblock);
    lsm_blob_size_update(&mut (*blobs).lbs_task, &mut blob_sizes.lbs_task);
    lsm_blob_size_update(&mut (*blobs).lbs_tun_dev, &mut blob_sizes.lbs_tun_dev);
    lsm_blob_size_update(&mut (*blobs).lbs_xattr_count, &mut blob_sizes.lbs_xattr_count);
    lsm_blob_size_update(&mut (*blobs).lbs_bdev, &mut blob_sizes.lbs_bdev);
    lsm_blob_size_update(&mut (*blobs).lbs_bpf_map, &mut blob_sizes.lbs_bpf_map);
    lsm_blob_size_update(&mut (*blobs).lbs_bpf_prog, &mut blob_sizes.lbs_bpf_prog);
    lsm_blob_size_update(&mut (*blobs).lbs_bpf_token, &mut blob_sizes.lbs_bpf_token);
}

/**
 * lsm_init_single - Initialize a given LSM
 * @lsm: LSM definition
 */
unsafe fn lsm_init_single(lsm: *mut lsm_info) {
    let ret: c_int;

    if !lsm_is_enabled(lsm) {
        return;
    }

    lsm_pr_dbg(
        b"initializing %s\n\0".as_ptr() as *const c_char,
        (*(*lsm).id).name,
    );
    ret = ((*lsm).init)();
    WARN(
        ret,
        b"%s failed to initialize: %d\n\0".as_ptr() as *const c_char,
        (*(*lsm).id).name,
        ret,
    );
}

/**
 * lsm_static_call_init - Initialize a LSM's static calls
 * @hl: LSM hook list
 */
unsafe fn lsm_static_call_init(hl: *mut security_hook_list) -> c_int {
    let mut scall: *mut lsm_static_call = (*hl).scalls;
    let mut i: c_int;

    i = 0;
    while i < MAX_LSM_COUNT as c_int {
        /* Update the first static call that is not used yet */
        if (*scall).hl.is_null() {
            __static_call_update((*scall).key, (*scall).trampoline, (*hl).hook.lsm_func_addr);
            (*scall).hl = hl;
            static_branch_enable((*scall).active);
            return 0;
        }
        scall = scall.add(1);
        i += 1;
    }

    -ENOSPC
}

/**
 * security_add_hooks - Add a LSM's hooks to the LSM framework's hook lists
 * @hooks: LSM hooks to add
 * @count: number of hooks to add
 * @lsmid: identification information for the LSM
 *
 * Each LSM has to register its hooks with the LSM framework.
 */
pub unsafe extern "C" fn security_add_hooks(
    hooks: *mut security_hook_list,
    count: c_int,
    lsmid: *const lsm_id,
) {
    let mut i: c_int;

    i = 0;
    while i < count {
        (*hooks.add(i as usize)).lsmid = lsmid;
        if lsm_static_call_init(hooks.add(i as usize)) != 0 {
            panic(
                b"exhausted LSM callback slots with LSM %s\n\0".as_ptr() as *const c_char,
                (*lsmid).name,
            );
        }
        i += 1;
    }
}

/**
 * early_security_init - Initialize the early LSMs
 */
pub unsafe extern "C" fn early_security_init() -> c_int {
    let mut lsm: *mut lsm_info;

    /* NOTE: lsm_pr_dbg() doesn't work here as lsm_debug is not yet set */

    lsm = &mut __start_early_lsm_info;
    while lsm < &mut __end_early_lsm_info {
        lsm_enabled_set(lsm, true);
        lsm_order_append(lsm, b"early\0".as_ptr() as *const c_char);
        lsm_prepare(lsm);
        lsm_init_single(lsm);
        lsm_count_early += 1;
        lsm = lsm.add(1);
    }

    0
}

/**
 * security_init - Initializes the LSM framework
 *
 * This should be called early in the kernel initialization sequence.
 */
pub unsafe extern "C" fn security_init() -> c_int {
    let mut cnt: c_uint;
    let mut lsm: *mut *mut lsm_info;

    if lsm_debug {
        let mut i: *mut lsm_info;

        cnt = 0;
        lsm_pr(b"available LSMs: \0".as_ptr() as *const c_char);
        i = &mut __start_early_lsm_info;
        while i < &mut __end_early_lsm_info {
            lsm_pr_cont(
                b"%s%s(E)\0".as_ptr() as *const c_char,
                if cnt != 0 { b",\0".as_ptr() } else { b"\0".as_ptr() },
                (*(*i).id).name,
            );
            cnt += 1;
            i = i.add(1);
        }
        i = &mut __start_lsm_info;
        while i < &mut __end_lsm_info {
            lsm_pr_cont(
                b"%s%s\0".as_ptr() as *const c_char,
                if cnt != 0 { b",\0".as_ptr() } else { b"\0".as_ptr() },
                (*(*i).id).name,
            );
            cnt += 1;
            i = i.add(1);
        }
        lsm_pr_cont(b"\n\0".as_ptr() as *const c_char);

        lsm_pr(
            b"built-in LSM config: %s\n\0".as_ptr() as *const c_char,
            lsm_order_builtin,
        );

        lsm_pr(
            b"legacy LSM parameter: %s\n\0".as_ptr() as *const c_char,
            lsm_order_legacy,
        );
        lsm_pr(
            b"boot LSM parameter: %s\n\0".as_ptr() as *const c_char,
            lsm_order_cmdline,
        );

        /* see the note about lsm_pr_dbg() in early_security_init() */
        i = &mut __start_early_lsm_info;
        while i < &mut __end_early_lsm_info {
            lsm_pr(
                b"enabled LSM early:%s\n\0".as_ptr() as *const c_char,
                (*(*i).id).name,
            );
            i = i.add(1);
        }
    }

    if !lsm_order_cmdline.is_null() {
        if !lsm_order_legacy.is_null() {
            lsm_order_legacy = ptr::null();
        }
        lsm_order_parse(lsm_order_cmdline, b"cmdline\0".as_ptr() as *const c_char);
    } else {
        lsm_order_parse(lsm_order_builtin, b"builtin\0".as_ptr() as *const c_char);
    }

    lsm = lsm_order.as_mut_ptr();
    while !(*lsm).is_null() {
        lsm_prepare(*lsm);
        lsm = lsm.add(1);
    }

    if lsm_debug {
        lsm_pr(b"blob(cred) size %d\n\0".as_ptr() as *const c_char, blob_sizes.lbs_cred);
        lsm_pr(b"blob(file) size %d\n\0".as_ptr() as *const c_char, blob_sizes.lbs_file);
        lsm_pr(
            b"blob(backing_file) size %d\n\0".as_ptr() as *const c_char,
            blob_sizes.lbs_backing_file,
        );
        lsm_pr(b"blob(ib) size %d\n\0".as_ptr() as *const c_char, blob_sizes.lbs_ib);
        lsm_pr(b"blob(inode) size %d\n\0".as_ptr() as *const c_char, blob_sizes.lbs_inode);
        lsm_pr(b"blob(ipc) size %d\n\0".as_ptr() as *const c_char, blob_sizes.lbs_ipc);
        lsm_pr(b"blob(key) size %d\n\0".as_ptr() as *const c_char, blob_sizes.lbs_key);
        lsm_pr(b"blob(msg_msg)_size %d\n\0".as_ptr() as *const c_char, blob_sizes.lbs_msg_msg);
        lsm_pr(b"blob(sock) size %d\n\0".as_ptr() as *const c_char, blob_sizes.lbs_sock);
        lsm_pr(
            b"blob(superblock) size %d\n\0".as_ptr() as *const c_char,
            blob_sizes.lbs_superblock,
        );
        lsm_pr(
            b"blob(perf_event) size %d\n\0".as_ptr() as *const c_char,
            blob_sizes.lbs_perf_event,
        );
        lsm_pr(b"blob(task) size %d\n\0".as_ptr() as *const c_char, blob_sizes.lbs_task);
        lsm_pr(b"blob(tun_dev) size %d\n\0".as_ptr() as *const c_char, blob_sizes.lbs_tun_dev);
        lsm_pr(
            b"blob(xattr) count %d\n\0".as_ptr() as *const c_char,
            blob_sizes.lbs_xattr_count,
        );
        lsm_pr(b"blob(bdev) size %d\n\0".as_ptr() as *const c_char, blob_sizes.lbs_bdev);
        lsm_pr(b"blob(bpf_map) size %d\n\0".as_ptr() as *const c_char, blob_sizes.lbs_bpf_map);
        lsm_pr(b"blob(bpf_prog) size %d\n\0".as_ptr() as *const c_char, blob_sizes.lbs_bpf_prog);
        lsm_pr(
            b"blob(bpf_token) size %d\n\0".as_ptr() as *const c_char,
            blob_sizes.lbs_bpf_token,
        );
    }

    if blob_sizes.lbs_file != 0 {
        lsm_file_cache = kmem_cache_create(
            b"lsm_file_cache\0".as_ptr() as *const c_char,
            blob_sizes.lbs_file,
            0,
            SLAB_PANIC,
            ptr::null_mut(),
        );
    }
    if blob_sizes.lbs_backing_file != 0 {
        lsm_backing_file_cache = kmem_cache_create(
            b"lsm_backing_file_cache\0".as_ptr() as *const c_char,
            blob_sizes.lbs_backing_file,
            0,
            SLAB_PANIC,
            ptr::null_mut(),
        );
    }
    if blob_sizes.lbs_inode != 0 {
        lsm_inode_cache = kmem_cache_create(
            b"lsm_inode_cache\0".as_ptr() as *const c_char,
            blob_sizes.lbs_inode,
            0,
            SLAB_PANIC,
            ptr::null_mut(),
        );
    }

    if lsm_cred_alloc(unrcu_pointer((*current).cred), GFP_KERNEL) != 0 {
        panic(b"early LSM cred alloc failed\n\0".as_ptr() as *const c_char);
    }
    if lsm_task_alloc(current) != 0 {
        panic(b"early LSM task alloc failed\n\0".as_ptr() as *const c_char);
    }

    cnt = 0;
    lsm = lsm_order.as_mut_ptr();
    while !(*lsm).is_null() {
        /* skip the "early" LSMs as they have already been setup */
        if cnt < lsm_count_early {
            cnt += 1;
            lsm = lsm.add(1);
            continue;
        }
        cnt += 1;
        lsm_init_single(*lsm);
        lsm = lsm.add(1);
    }

    0
}

unsafe fn initcall_pure(lsm: *mut lsm_info) -> Option<unsafe extern "C" fn() -> c_int> {
    (*lsm).initcall_pure
}

unsafe fn initcall_early(lsm: *mut lsm_info) -> Option<unsafe extern "C" fn() -> c_int> {
    (*lsm).initcall_early
}

unsafe fn initcall_core(lsm: *mut lsm_info) -> Option<unsafe extern "C" fn() -> c_int> {
    (*lsm).initcall_core
}

unsafe fn initcall_subsys(lsm: *mut lsm_info) -> Option<unsafe extern "C" fn() -> c_int> {
    (*lsm).initcall_subsys
}

unsafe fn initcall_fs(lsm: *mut lsm_info) -> Option<unsafe extern "C" fn() -> c_int> {
    (*lsm).initcall_fs
}

unsafe fn initcall_device(lsm: *mut lsm_info) -> Option<unsafe extern "C" fn() -> c_int> {
    (*lsm).initcall_device
}

unsafe fn initcall_late(lsm: *mut lsm_info) -> Option<unsafe extern "C" fn() -> c_int> {
    (*lsm).initcall_late
}

unsafe fn initcall_late_sync(lsm: *mut lsm_info) -> Option<unsafe extern "C" fn() -> c_int> {
    (*lsm).initcall_late_sync
}

/**
 * security_initcall_pure - Run the LSM pure initcalls
 */
unsafe extern "C" fn security_initcall_pure() -> c_int {
    lsm_initcall(initcall_pure, b"pure\0".as_ptr() as *const c_char)
}
// pure_initcall(security_initcall_pure);

/**
 * security_initcall_early - Run the LSM early initcalls
 */
unsafe extern "C" fn security_initcall_early() -> c_int {
    lsm_initcall(initcall_early, b"early\0".as_ptr() as *const c_char)
}
// early_initcall(security_initcall_early);

/**
 * security_initcall_core - Run the LSM core initcalls
 */
unsafe extern "C" fn security_initcall_core() -> c_int {
    let rc_sfs: c_int;
    let rc_lsm: c_int;

    rc_sfs = securityfs_init();
    rc_lsm = lsm_initcall(initcall_core, b"core\0".as_ptr() as *const c_char);

    if rc_sfs != 0 { rc_sfs } else { rc_lsm }
}
// core_initcall(security_initcall_core);

/**
 * security_initcall_subsys - Run the LSM subsys initcalls
 */
unsafe extern "C" fn security_initcall_subsys() -> c_int {
    lsm_initcall(initcall_subsys, b"subsys\0".as_ptr() as *const c_char)
}
// subsys_initcall(security_initcall_subsys);

/**
 * security_initcall_fs - Run the LSM fs initcalls
 */
unsafe extern "C" fn security_initcall_fs() -> c_int {
    lsm_initcall(initcall_fs, b"fs\0".as_ptr() as *const c_char)
}
// fs_initcall(security_initcall_fs);

/**
 * security_initcall_device - Run the LSM device initcalls
 */
unsafe extern "C" fn security_initcall_device() -> c_int {
    lsm_initcall(initcall_device, b"device\0".as_ptr() as *const c_char)
}
// device_initcall(security_initcall_device);

/**
 * security_initcall_late - Run the LSM late initcalls
 */
unsafe extern "C" fn security_initcall_late() -> c_int {
    lsm_initcall(initcall_late, b"late\0".as_ptr() as *const c_char)
}
// late_initcall(security_initcall_late);

/**
 * security_initcall_late_sync - Run the LSM late initcalls sync
 */
unsafe extern "C" fn security_initcall_late_sync() -> c_int {
    let rc: c_int;

    rc = lsm_initcall(initcall_late_sync, b"late_sync\0".as_ptr() as *const c_char);
    lsm_pr_dbg(b"all enabled LSMs fully activated\n\0".as_ptr() as *const c_char);
    call_blocking_lsm_notifier(LSM_STARTED_ALL, ptr::null_mut());

    rc
}
// late_initcall_sync(security_initcall_late_sync);


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
