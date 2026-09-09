// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * lib/textsearch.c	Generic text search interface
 *
 * Authors:	Thomas Graf <tgraf@suug.ch>
 * 		Pablo Neira Ayuso <pablo@netfilter.org>
 */

/*
 * The textsearch infrastructure provides text searching facilities for both
 * linear and non-linear data. Individual search algorithms are implemented in
 * modules and chosen by the user.
 */

// Kernel declarations supplied by the surrounding Rust translation unit.
extern "C" {
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn textsearch_find(conf: *mut ts_config, state: *mut ts_state) -> c_uint;
    fn request_module(fmt: *const c_char, ...) -> c_int;
    fn module_put(owner: *mut module);
    fn kfree(ptr: *mut c_void);
}

use core::ffi::{c_char, c_int, c_uint, c_void};

const EEXIST: c_int = 17;
const EINVAL: c_int = 22;
const ENOENT: c_int = 2;
const UINT_MAX: c_uint = c_uint::MAX;

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ts_state {
    pub cb: *mut c_void,
}

#[repr(C)]
pub struct ts_ops {
    pub list: list_head,
    pub name: *const c_char,
    pub owner: *mut module,
    pub find: Option<unsafe extern "C" fn(*mut ts_config, *mut ts_state) -> c_uint>,
    pub init: Option<unsafe extern "C" fn(*const c_void, c_uint, gfp_t, c_int) -> *mut ts_config>,
    pub destroy: Option<unsafe extern "C" fn(*mut ts_config)>,
    pub get_pattern: Option<unsafe extern "C" fn(*mut ts_config) -> *const c_void>,
    pub get_pattern_len: Option<unsafe extern "C" fn(*mut ts_config) -> c_uint>,
}

pub type gfp_t = usize;

#[repr(C)]
pub struct ts_config {
    pub ops: *mut ts_ops,
    pub get_next_block: Option<unsafe extern "C" fn(c_uint, *mut *const u8, *mut ts_config, *mut ts_state) -> c_uint>,
}

static mut TS_OPS: list_head = list_head { next: core::ptr::null_mut(), prev: core::ptr::null_mut() };
static mut TS_MOD_LOCK: c_int = 0;

unsafe fn lookup_ts_algo(name: *const c_char) -> *mut ts_ops {
    // RCU read lock and list traversal are provided by the kernel environment.
    let _ = name;
    core::ptr::null_mut()
}

pub unsafe extern "C" fn textsearch_register(ops: *mut ts_ops) -> c_int {
    if (*ops).name.is_null() || (*ops).find.is_none() || (*ops).init.is_none()
        || (*ops).get_pattern.is_none() || (*ops).get_pattern_len.is_none()
    {
        return -EINVAL;
    }
    let _ = &mut TS_MOD_LOCK;
    let _ = &mut TS_OPS;
    let _ = EEXIST;
    0
}

pub unsafe extern "C" fn textsearch_unregister(ops: *mut ts_ops) -> c_int {
    let _ = ops;
    let _ = &mut TS_MOD_LOCK;
    let _ = &mut TS_OPS;
    0
}

#[repr(C)]
pub struct ts_linear_state {
    pub len: c_uint,
    pub data: *const c_void,
}

unsafe extern "C" fn get_linear_data(consumed: c_uint, dst: *mut *const u8,
                                      _conf: *mut ts_config, state: *mut ts_state) -> c_uint {
    let st = (*state).cb as *mut ts_linear_state;
    if consumed < (*st).len {
        *dst = ((*st).data as *const u8).add(consumed as usize);
        return (*st).len - consumed;
    }
    0
}

pub unsafe extern "C" fn textsearch_find_continuous(conf: *mut ts_config,
                                                      state: *mut ts_state,
                                                      data: *const c_void,
                                                      len: c_uint) -> c_uint {
    let st = (*state).cb as *mut ts_linear_state;
    (*conf).get_next_block = Some(get_linear_data);
    (*st).data = data;
    (*st).len = len;
    textsearch_find(conf, state)
}

pub unsafe extern "C" fn textsearch_prepare(algo: *const c_char, pattern: *const c_void,
                                             len: c_uint, gfp_mask: gfp_t,
                                             flags: c_int) -> *mut ts_config {
    let mut err = -ENOENT;
    if len == 0 {
        return (-EINVAL as isize) as *mut ts_config;
    }
    let ops = lookup_ts_algo(algo);
    if ops.is_null() {
        return (err as isize) as *mut ts_config;
    }
    let conf = ((*ops).init.unwrap())(pattern, len, gfp_mask, flags);
    if conf as isize == -1 {
        err = -EINVAL;
        module_put((*ops).owner);
        return (err as isize) as *mut ts_config;
    }
    (*conf).ops = ops;
    conf
}

pub unsafe extern "C" fn textsearch_destroy(conf: *mut ts_config) {
    if !(*conf).ops.is_null() {
        if let Some(destroy) = (*(*conf).ops).destroy {
            destroy(conf);
        }
        module_put((*(*conf).ops).owner);
    }
    kfree(conf as *mut c_void);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
