/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from linux/wait_bit.h. */

#[repr(C)]
pub struct wait_bit_key {
    pub flags: *mut ::core::ffi::c_ulong,
    pub bit_nr: ::core::ffi::c_int,
    pub timeout: ::core::ffi::c_ulong,
}

#[repr(C)]
pub struct wait_bit_queue_entry {
    pub key: wait_bit_key,
    pub wq_entry: wait_queue_entry,
}

pub type wait_bit_action_f = unsafe extern "C" fn(*mut wait_bit_key, ::core::ffi::c_int) -> ::core::ffi::c_int;

extern "C" {
    pub fn __wake_up_bit(wq_head: *mut wait_queue_head, word: *mut ::core::ffi::c_ulong, bit: ::core::ffi::c_int);
    pub fn __wait_on_bit(wq_head: *mut wait_queue_head, wbq_entry: *mut wait_bit_queue_entry, action: wait_bit_action_f, mode: ::core::ffi::c_uint) -> ::core::ffi::c_int;
    pub fn __wait_on_bit_lock(wq_head: *mut wait_queue_head, wbq_entry: *mut wait_bit_queue_entry, action: wait_bit_action_f, mode: ::core::ffi::c_uint) -> ::core::ffi::c_int;
    pub fn wake_up_bit(word: *mut ::core::ffi::c_ulong, bit: ::core::ffi::c_int);
    pub fn out_of_line_wait_on_bit(word: *mut ::core::ffi::c_ulong, bit: ::core::ffi::c_int, action: wait_bit_action_f, mode: ::core::ffi::c_uint) -> ::core::ffi::c_int;
    pub fn out_of_line_wait_on_bit_timeout(word: *mut ::core::ffi::c_ulong, bit: ::core::ffi::c_int, action: wait_bit_action_f, mode: ::core::ffi::c_uint, timeout: ::core::ffi::c_ulong) -> ::core::ffi::c_int;
    pub fn out_of_line_wait_on_bit_lock(word: *mut ::core::ffi::c_ulong, bit: ::core::ffi::c_int, action: wait_bit_action_f, mode: ::core::ffi::c_uint) -> ::core::ffi::c_int;
    pub fn bit_waitqueue(word: *mut ::core::ffi::c_ulong, bit: ::core::ffi::c_int) -> *mut wait_queue_head;
    pub fn wait_bit_init();
    pub fn __var_wake_key(wq_entry: *mut wait_queue_entry, arg: *mut ::core::ffi::c_void) -> *mut wait_bit_key;
    pub fn wake_bit_function(wq_entry: *mut wait_queue_entry, mode: ::core::ffi::c_uint, sync: ::core::ffi::c_int, key: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    pub fn bit_wait(key: *mut wait_bit_key, mode: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn bit_wait_io(key: *mut wait_bit_key, mode: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn bit_wait_timeout(key: *mut wait_bit_key, mode: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn init_wait_var_entry(wbq_entry: *mut wait_bit_queue_entry, var: *mut ::core::ffi::c_void, flags: ::core::ffi::c_int);
    pub fn wake_up_var(var: *mut ::core::ffi::c_void);
    pub fn __var_waitqueue(p: *mut ::core::ffi::c_void) -> *mut wait_queue_head;
}

/* These types and functions are supplied by the Linux wait/atomic headers. */
#[repr(C)] pub struct wait_queue_head { _private: [u8; 0] }
#[repr(C)] pub struct wait_queue_entry { _private: [u8; 0] }
#[repr(C)] pub struct atomic_t { _private: [u8; 0] }

#[inline]
pub unsafe fn wait_on_bit(word: *mut ::core::ffi::c_ulong, bit: ::core::ffi::c_int, mode: ::core::ffi::c_uint) -> ::core::ffi::c_int {
    might_sleep();
    if !test_bit_acquire(bit, word) { return 0; }
    out_of_line_wait_on_bit(word, bit, bit_wait, mode)
}

#[inline]
pub unsafe fn wait_on_bit_io(word: *mut ::core::ffi::c_ulong, bit: ::core::ffi::c_int, mode: ::core::ffi::c_uint) -> ::core::ffi::c_int {
    might_sleep();
    if !test_bit_acquire(bit, word) { return 0; }
    out_of_line_wait_on_bit(word, bit, bit_wait_io, mode)
}

#[inline]
pub unsafe fn wait_on_bit_timeout(word: *mut ::core::ffi::c_ulong, bit: ::core::ffi::c_int, mode: ::core::ffi::c_uint, timeout: ::core::ffi::c_ulong) -> ::core::ffi::c_int {
    might_sleep();
    if !test_bit_acquire(bit, word) { return 0; }
    out_of_line_wait_on_bit_timeout(word, bit, bit_wait_timeout, mode, timeout)
}

#[inline]
pub unsafe fn wait_on_bit_action(word: *mut ::core::ffi::c_ulong, bit: ::core::ffi::c_int, action: wait_bit_action_f, mode: ::core::ffi::c_uint) -> ::core::ffi::c_int {
    might_sleep();
    if !test_bit_acquire(bit, word) { return 0; }
    out_of_line_wait_on_bit(word, bit, action, mode)
}

#[inline]
pub unsafe fn wait_on_bit_lock(word: *mut ::core::ffi::c_ulong, bit: ::core::ffi::c_int, mode: ::core::ffi::c_uint) -> ::core::ffi::c_int {
    might_sleep();
    if !test_and_set_bit(bit, word) { return 0; }
    out_of_line_wait_on_bit_lock(word, bit, bit_wait, mode)
}

#[inline]
pub unsafe fn wait_on_bit_lock_io(word: *mut ::core::ffi::c_ulong, bit: ::core::ffi::c_int, mode: ::core::ffi::c_uint) -> ::core::ffi::c_int {
    might_sleep();
    if !test_and_set_bit(bit, word) { return 0; }
    out_of_line_wait_on_bit_lock(word, bit, bit_wait_io, mode)
}

#[inline]
pub unsafe fn wait_on_bit_lock_action(word: *mut ::core::ffi::c_ulong, bit: ::core::ffi::c_int, action: wait_bit_action_f, mode: ::core::ffi::c_uint) -> ::core::ffi::c_int {
    might_sleep();
    if !test_and_set_bit(bit, word) { return 0; }
    out_of_line_wait_on_bit_lock(word, bit, action, mode)
}

#[inline]
pub unsafe fn clear_and_wake_up_bit(bit: ::core::ffi::c_int, word: *mut ::core::ffi::c_ulong) {
    clear_bit_unlock(bit, word);
    /* See wake_up_bit() for which memory barrier is needed. */
    smp_mb__after_atomic();
    wake_up_bit(word, bit);
}

#[inline]
pub unsafe fn test_and_clear_wake_up_bit(bit: ::core::ffi::c_int, word: *mut ::core::ffi::c_ulong) -> bool {
    if !test_and_clear_bit(bit, word) { return false; }
    wake_up_bit(word, bit);
    true
}

#[inline]
pub unsafe fn atomic_dec_and_wake_up(var: *mut atomic_t) -> bool {
    if !atomic_dec_and_test(var) { return false; }
    wake_up_var(var.cast());
    true
}

/* External operations referenced by the inline translations. */
extern "C" {
    fn might_sleep();
    fn test_bit_acquire(bit: ::core::ffi::c_int, word: *const ::core::ffi::c_ulong) -> bool;
    fn test_and_set_bit(bit: ::core::ffi::c_int, word: *mut ::core::ffi::c_ulong) -> bool;
    fn clear_bit_unlock(bit: ::core::ffi::c_int, word: *mut ::core::ffi::c_ulong);
    fn smp_mb__after_atomic();
    fn test_and_clear_bit(bit: ::core::ffi::c_int, word: *mut ::core::ffi::c_ulong) -> bool;
    fn atomic_dec_and_test(var: *mut atomic_t) -> bool;
}

/* Preprocessor-only wait-variable helpers retain their source-level intent. */
// __WAIT_BIT_KEY_INITIALIZER(word, bit) => wait_bit_key { flags: word, bit_nr: bit, timeout: 0 }
// DEFINE_WAIT_BIT and wait_var_event* macros require the surrounding kernel waitqueue types and statement expressions.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
