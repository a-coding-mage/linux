/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from linux/jump_label_ratelimit.h. */
/* Dependencies supplied by linux/jump_label.h and linux/workqueue.h remain external. */

/* CONFIG_JUMP_LABEL conditional preserved from the source header. */
#[cfg(CONFIG_JUMP_LABEL)]
#[repr(C)]
pub struct static_key_deferred {
    pub key: static_key,
    pub timeout: ::core::ffi::c_ulong,
    pub work: delayed_work,
}

#[cfg(CONFIG_JUMP_LABEL)]
#[repr(C)]
pub struct static_key_true_deferred {
    pub key: static_key_true,
    pub timeout: ::core::ffi::c_ulong,
    pub work: delayed_work,
}

#[cfg(CONFIG_JUMP_LABEL)]
#[repr(C)]
pub struct static_key_false_deferred {
    pub key: static_key_false,
    pub timeout: ::core::ffi::c_ulong,
    pub work: delayed_work,
}

#[cfg(CONFIG_JUMP_LABEL)]
extern "C" {
    pub fn __static_key_slow_dec_deferred(
        key: *mut static_key,
        work: *mut delayed_work,
        timeout: ::core::ffi::c_ulong,
    );
    pub fn __static_key_deferred_flush(key: *mut ::core::ffi::c_void, work: *mut delayed_work);
    pub fn jump_label_rate_limit(key: *mut static_key_deferred, rl: ::core::ffi::c_ulong);
    pub fn jump_label_update_timeout(work: *mut work_struct);
}

#[cfg(CONFIG_JUMP_LABEL)]
#[inline]
pub unsafe fn static_key_slow_dec_deferred(x: *mut static_key_deferred) {
    __static_key_slow_dec_deferred(
        &mut (*x).key,
        &mut (*x).work,
        (*x).timeout,
    );
}

#[cfg(CONFIG_JUMP_LABEL)]
#[inline]
pub unsafe fn static_branch_slow_dec_deferred(x: *mut static_key_true_deferred) {
    __static_key_slow_dec_deferred(
        &mut (*x).key.key,
        &mut (*x).work,
        (*x).timeout,
    );
}

#[cfg(CONFIG_JUMP_LABEL)]
#[inline]
pub unsafe fn static_key_deferred_flush(x: *mut ::core::ffi::c_void) {
    __static_key_deferred_flush(x, &mut (*(x as *mut static_key_deferred)).work);
}

/* DEFINE_STATIC_KEY_DEFERRED_TRUE/FALSE expand to C static initializers. */

#[cfg(not(CONFIG_JUMP_LABEL))]
#[repr(C)]
pub struct static_key_deferred {
    pub key: static_key,
}

#[cfg(not(CONFIG_JUMP_LABEL))]
#[repr(C)]
pub struct static_key_true_deferred {
    pub key: static_key_true,
}

#[cfg(not(CONFIG_JUMP_LABEL))]
#[repr(C)]
pub struct static_key_false_deferred {
    pub key: static_key_false,
}

#[cfg(not(CONFIG_JUMP_LABEL))]
#[inline]
pub unsafe fn static_branch_slow_dec_deferred(x: *mut static_key_true_deferred) {
    static_branch_dec(&mut (*x).key);
}

#[cfg(not(CONFIG_JUMP_LABEL))]
#[inline]
pub unsafe fn static_key_slow_dec_deferred(key: *mut static_key_deferred) {
    STATIC_KEY_CHECK_USE(key);
    static_key_slow_dec(&mut (*key).key);
}

#[cfg(not(CONFIG_JUMP_LABEL))]
#[inline]
pub unsafe fn static_key_deferred_flush(key: *mut ::core::ffi::c_void) {
    STATIC_KEY_CHECK_USE(key);
}

#[cfg(not(CONFIG_JUMP_LABEL))]
#[inline]
pub unsafe fn jump_label_rate_limit(
    key: *mut static_key_deferred,
    rl: ::core::ffi::c_ulong,
) {
    STATIC_KEY_CHECK_USE(key);
}

#[inline]
pub unsafe fn static_branch_deferred_inc(x: *mut static_key_true) {
    static_branch_inc(&mut (*x).key);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
