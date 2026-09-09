/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation:
// linux/types.h, linux/list.h, linux/kernel.h, linux/err.h, linux/slab.h

use core::ffi::c_void;

pub struct module;

pub const TS_AUTOLOAD: i32 = 1; // Automatically load textsearch modules when needed
pub const TS_IGNORECASE: i32 = 2; // Searches string case insensitively

/**
 * struct ts_state - search state
 * @offset: offset for next match
 * @cb: control buffer, for persistent variables of get_next_block()
 */
#[repr(C)]
pub struct ts_state {
    pub offset: u32,
    pub cb: [i8; 48],
}

/**
 * struct ts_ops - search module operations
 * @name: name of search algorithm
 * @init: initialization function to prepare a search
 * @find: find the next occurrence of the pattern
 * @destroy: destroy algorithm specific parts of a search configuration
 * @get_pattern: return head of pattern
 * @get_pattern_len: return length of pattern
 * @owner: module reference to algorithm
 * @list: list to search
 */
#[repr(C)]
pub struct ts_ops {
    pub name: *const i8,
    pub init: Option<unsafe extern "C" fn(*const c_void, u32, gfp_t, i32) -> *mut ts_config>,
    pub find: Option<unsafe extern "C" fn(*mut ts_config, *mut ts_state) -> u32>,
    pub destroy: Option<unsafe extern "C" fn(*mut ts_config)>,
    pub get_pattern: Option<unsafe extern "C" fn(*mut ts_config) -> *mut c_void>,
    pub get_pattern_len: Option<unsafe extern "C" fn(*mut ts_config) -> u32>,
    pub owner: *mut module,
    pub list: list_head,
}

/**
 * struct ts_config - search configuration
 * @ops: operations of chosen algorithm
 * @flags: flags
 * @get_next_block: callback to fetch the next block to search in
 * @finish: callback to finalize a search
 */
#[repr(C)]
pub struct ts_config {
    pub ops: *mut ts_ops,
    pub flags: i32,

    /**
     * @get_next_block: fetch next block of data
     * @consumed: number of bytes consumed by the caller
     * @dst: destination buffer
     * @conf: search configuration
     * @state: search state
     *
     * Called repeatedly until 0 is returned. Must assign the
     * head of the next block of data to &*dst and return the length
     * of the block or 0 if at the end. consumed == 0 indicates
     * a new search. May store/read persistent values in state->cb.
     */
    pub get_next_block: Option<unsafe extern "C" fn(u32, *mut *const u8, *mut ts_config, *mut ts_state) -> u32>,

    /**
     * @finish: finalize/clean a series of get_next_block() calls
     * @conf: search configuration
     * @state: search state
     *
     * Called after the last use of get_next_block(), may be used
     * to cleanup any leftovers.
     */
    pub finish: Option<unsafe extern "C" fn(*mut ts_config, *mut ts_state)>,
}

/**
 * textsearch_next - continue searching for a pattern
 * @conf: search configuration
 * @state: search state
 *
 * Continues a search looking for more occurrences of the pattern.
 * textsearch_find() must be called to find the first occurrence
 * in order to reset the state.
 *
 * Returns the position of the next occurrence of the pattern or
 * UINT_MAX if not match was found.
 */
#[inline]
pub unsafe fn textsearch_next(conf: *mut ts_config, state: *mut ts_state) -> u32 {
    let ret = ((*(*conf).ops).find.unwrap())(conf, state);

    if let Some(finish) = (*conf).finish {
        finish(conf, state);
    }

    ret
}

/**
 * textsearch_find - start searching for a pattern
 * @conf: search configuration
 * @state: search state
 *
 * Returns the position of first occurrence of the pattern or
 * UINT_MAX if no match was found.
 */
#[inline]
pub unsafe fn textsearch_find(conf: *mut ts_config, state: *mut ts_state) -> u32 {
    (*state).offset = 0;
    textsearch_next(conf, state)
}

/**
 * textsearch_get_pattern - return head of the pattern
 * @conf: search configuration
 */
#[inline]
pub unsafe fn textsearch_get_pattern(conf: *mut ts_config) -> *mut c_void {
    ((*(*conf).ops).get_pattern.unwrap())(conf)
}

/**
 * textsearch_get_pattern_len - return length of the pattern
 * @conf: search configuration
 */
#[inline]
pub unsafe fn textsearch_get_pattern_len(conf: *mut ts_config) -> u32 {
    ((*(*conf).ops).get_pattern_len.unwrap())(conf)
}

unsafe extern "C" {
    pub fn textsearch_register(ops: *mut ts_ops) -> i32;
    pub fn textsearch_unregister(ops: *mut ts_ops) -> i32;
    pub fn textsearch_prepare(pattern: *const i8, blob: *const c_void, len: u32, gfp: gfp_t, flags: i32) -> *mut ts_config;
    pub fn textsearch_destroy(conf: *mut ts_config);
    pub fn textsearch_find_continuous(conf: *mut ts_config, state: *mut ts_state, buf: *const c_void, len: u32) -> u32;
}

pub const TS_PRIV_ALIGNTO: usize = 8;

#[inline]
pub const fn TS_PRIV_ALIGN(len: usize) -> usize {
    (len + TS_PRIV_ALIGNTO - 1) & !(TS_PRIV_ALIGNTO - 1)
}

#[inline]
pub unsafe fn alloc_ts_config(payload: usize, gfp_mask: gfp_t) -> *mut ts_config {
    let conf = kzalloc(TS_PRIV_ALIGN(core::mem::size_of::<ts_config>()) + payload, gfp_mask);
    if conf.is_null() {
        return ERR_PTR(-ENOMEM) as *mut ts_config;
    }
    conf as *mut ts_config
}

#[inline]
pub unsafe fn ts_config_priv(conf: *mut ts_config) -> *mut c_void {
    (conf as *mut u8).add(TS_PRIV_ALIGN(core::mem::size_of::<ts_config>())) as *mut c_void
}

// Supplied by linux/types.h, linux/err.h, and linux/slab.h.
extern "C" {
    pub fn kzalloc(size: usize, flags: gfp_t) -> *mut c_void;
    pub fn ERR_PTR(error: isize) -> *mut c_void;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
