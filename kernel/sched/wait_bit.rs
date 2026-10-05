// SPDX-License-Identifier: GPL-2.0-only

use kernel::ffi::{c_int, c_long, c_uint, c_ulong, c_void};
use kernel::bindings::sched_waiting_native::*;
use super::wait::{
    __wake_up, autoremove_wake_function, finish_wait, prepare_to_wait,
    prepare_to_wait_exclusive,
};

// Preserve NULL until the native algorithm actually reaches an action call.
// The enclosing CFI signature still requires separate native qualification.
pub type WaitBitAction = wait_bit_action_f;

// The native data owner preserves configured __cacheline_aligned storage.

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bit_waitqueue(word: *mut c_ulong, bit: c_int) -> *mut wait_queue_head_t {
    // SAFETY: The caller upholds the native wait API pointer and locking contract.
    unsafe {
    let shift: c_int = if LUPOS_WAITING_BITS_PER_LONG == 32 { 5 } else { 6 };
    let val = (word as c_ulong).wrapping_shl(shift as u32) | bit as c_ulong;
    lupos_waiting_bit_table().add(lupos_waiting_hash_long(val, LUPOS_WAITING_WAIT_TABLE_BITS) as usize)

    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wake_bit_function(
    wq_entry: *mut wait_queue_entry,
    mode: c_uint,
    sync: c_int,
    arg: *mut c_void,
) -> c_int {
    // SAFETY: The caller upholds the native wait API pointer and locking contract.
    unsafe {
    let key = arg as *mut wait_bit_key;
    let wait_bit = lupos_waiting_bit_entry(wq_entry);

    if (*wait_bit).key.flags != (*key).flags
        || (*wait_bit).key.bit_nr != (*key).bit_nr
        || lupos_waiting_test_bit((*key).bit_nr, (*key).flags)
    {
        return 0;
    }

    autoremove_wake_function(wq_entry, mode, sync, key as *mut c_void)

    }
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".sched.text")]
pub unsafe extern "C" fn __wait_on_bit(
    wq_head: *mut wait_queue_head_t,
    wbq_entry: *mut wait_bit_queue_entry,
    action: WaitBitAction,
    mode: c_uint,
) -> c_int {
    // SAFETY: The caller upholds the native wait API pointer and locking contract.
    unsafe {
    let mut ret: c_int = 0;
    loop {
        prepare_to_wait(wq_head, &raw mut (*wbq_entry).wq_entry, mode as c_int);
        if lupos_waiting_test_bit((*wbq_entry).key.bit_nr, (*wbq_entry).key.flags) {
            ret = lupos_waiting_call_bit_action(action, &raw mut (*wbq_entry).key, mode as c_int);
        }
        if !(lupos_waiting_test_bit_acquire((*wbq_entry).key.bit_nr, (*wbq_entry).key.flags) && ret == 0) {
            break;
        }
    }
    finish_wait(wq_head, &raw mut (*wbq_entry).wq_entry);
    ret

    }
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".sched.text")]
pub unsafe extern "C" fn out_of_line_wait_on_bit(
    word: *mut c_ulong, bit: c_int, action: WaitBitAction, mode: c_uint,
) -> c_int {
    // SAFETY: The caller upholds the native wait API pointer and locking contract.
    unsafe {
    let wq_head = bit_waitqueue(word, bit);
    let mut wq_entry = core::mem::MaybeUninit::<wait_bit_queue_entry>::uninit();
    let wq_entry = wq_entry.as_mut_ptr();
    lupos_waiting_init_bit_entry(wq_entry, word, bit);
    __wait_on_bit(wq_head, wq_entry, action, mode)

    }
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".sched.text")]
pub unsafe extern "C" fn out_of_line_wait_on_bit_timeout(
    word: *mut c_ulong, bit: c_int, action: WaitBitAction, mode: c_uint, timeout: c_ulong,
) -> c_int {
    // SAFETY: The caller upholds the native wait API pointer and locking contract.
    unsafe {
    let wq_head = bit_waitqueue(word, bit);
    let mut wq_entry = core::mem::MaybeUninit::<wait_bit_queue_entry>::uninit();
    let wq_entry = wq_entry.as_mut_ptr();
    lupos_waiting_init_bit_entry(wq_entry, word, bit);
    (*wq_entry).key.timeout = lupos_waiting_read_jiffies().wrapping_add(timeout);
    __wait_on_bit(wq_head, wq_entry, action, mode)

    }
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".sched.text")]
pub unsafe extern "C" fn __wait_on_bit_lock(
    wq_head: *mut wait_queue_head_t,
    wbq_entry: *mut wait_bit_queue_entry,
    action: WaitBitAction,
    mode: c_uint,
) -> c_int {
    // SAFETY: The caller upholds the native wait API pointer and locking contract.
    unsafe {
    let mut ret: c_int = 0;
    loop {
        prepare_to_wait_exclusive(wq_head, &raw mut (*wbq_entry).wq_entry, mode as c_int);
        if lupos_waiting_test_bit((*wbq_entry).key.bit_nr, (*wbq_entry).key.flags) {
            ret = lupos_waiting_call_bit_action(action, &raw mut (*wbq_entry).key, mode as c_int);
            // See the comment in prepare_to_wait_event().
            // finish_wait() does not necessarily take wq_head->lock, but
            // test_and_set_bit() implies mb() which pairs with
            // smp_mb__after_atomic() before wake_up_page().
            if ret != 0 {
                finish_wait(wq_head, &raw mut (*wbq_entry).wq_entry);
            }
        }
        if !lupos_waiting_test_and_set_bit((*wbq_entry).key.bit_nr, (*wbq_entry).key.flags) {
            if ret == 0 {
                finish_wait(wq_head, &raw mut (*wbq_entry).wq_entry);
            }
            return 0;
        } else if ret != 0 {
            return ret;
        }
    }

    }
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".sched.text")]
pub unsafe extern "C" fn out_of_line_wait_on_bit_lock(
    word: *mut c_ulong, bit: c_int, action: WaitBitAction, mode: c_uint,
) -> c_int {
    // SAFETY: The caller upholds the native wait API pointer and locking contract.
    unsafe {
    let wq_head = bit_waitqueue(word, bit);
    let mut wq_entry = core::mem::MaybeUninit::<wait_bit_queue_entry>::uninit();
    let wq_entry = wq_entry.as_mut_ptr();
    lupos_waiting_init_bit_entry(wq_entry, word, bit);
    __wait_on_bit_lock(wq_head, wq_entry, action, mode)

    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __wake_up_bit(wq_head: *mut wait_queue_head_t, word: *mut c_ulong, bit: c_int) {
    // SAFETY: The caller upholds the native wait API pointer and locking contract.
    unsafe {
    let mut key = wait_bit_key { flags: word, bit_nr: bit, timeout: 0 };
    if lupos_waiting_waitqueue_active(wq_head) {
        __wake_up(wq_head, LUPOS_WAITING_TASK_NORMAL, 1, &mut key as *mut _ as *mut c_void);
    }

    }
}

/// wake_up_bit - wake up waiters on a bit
/// @word: the address containing the bit being waited on
/// @bit: the bit at that address being waited on
///
/// Wake up any process waiting in wait_on_bit() or similar for the given bit to be cleared.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn wake_up_bit(word: *mut c_ulong, bit: c_int) {
    // SAFETY: The caller upholds the native wait API pointer and locking contract.
    unsafe {
    __wake_up_bit(bit_waitqueue(word, bit), word, bit);

    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __var_waitqueue(p: *mut c_void) -> *mut wait_queue_head_t {
    // SAFETY: The caller upholds the native wait API pointer and locking contract.
    unsafe {
    lupos_waiting_bit_table().add(lupos_waiting_hash_ptr(p, LUPOS_WAITING_WAIT_TABLE_BITS) as usize)

    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __var_wake_key(wq_entry: *mut wait_queue_entry, arg: *mut c_void) -> *mut wait_bit_key {
    // SAFETY: The caller upholds the native wait API pointer and locking contract.
    unsafe {
    let key = arg as *mut wait_bit_key;
    let wbq_entry = lupos_waiting_bit_entry(wq_entry);
    if (*wbq_entry).key.flags != (*key).flags || (*wbq_entry).key.bit_nr != (*key).bit_nr {
        core::ptr::null_mut()
    } else {
        key
    }

    }
}

unsafe extern "C" fn var_wake_function(wq_entry: *mut wait_queue_entry, mode: c_uint, sync: c_int, arg: *mut c_void) -> c_int {
    // SAFETY: The caller upholds the native wait API pointer and locking contract.
    unsafe {
    let key = __var_wake_key(wq_entry, arg);
    if key.is_null() { return 0; }
    autoremove_wake_function(wq_entry, mode, sync, key as *mut c_void)

    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_wait_var_entry(wbq_entry: *mut wait_bit_queue_entry, var: *mut c_void, flags: c_int) {
    // SAFETY: The caller upholds the native wait API pointer and locking contract.
    unsafe {
    (*wbq_entry).key.flags = var.cast();
    (*wbq_entry).key.timeout = 0;
    (*wbq_entry).key.bit_nr = -1;
    (*wbq_entry).wq_entry.flags = flags as c_uint;
    (*wbq_entry).wq_entry.private = lupos_waiting_current().cast();
    (*wbq_entry).wq_entry.func = Some(var_wake_function);
    lupos_waiting_init_list_head(&raw mut (*wbq_entry).wq_entry.entry);

    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wake_up_var(var: *mut c_void) {
    // SAFETY: The caller upholds the native wait API pointer and locking contract.
    unsafe {
    __wake_up_bit(__var_waitqueue(var), var as *mut c_ulong, -1);

    }
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".sched.text")]
pub unsafe extern "C" fn bit_wait(_word: *mut wait_bit_key, mode: c_int) -> c_int {
    // SAFETY: The caller upholds the native wait API pointer and locking contract.
    unsafe {
    schedule();
    if lupos_waiting_signal_pending_state(mode, lupos_waiting_current()) { return -LUPOS_WAITING_EINTR; }
    0

    }
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".sched.text")]
pub unsafe extern "C" fn bit_wait_io(_word: *mut wait_bit_key, mode: c_int) -> c_int {
    // SAFETY: The caller upholds the native wait API pointer and locking contract.
    unsafe {
    io_schedule();
    if lupos_waiting_signal_pending_state(mode, lupos_waiting_current()) { return -LUPOS_WAITING_EINTR; }
    0

    }
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".sched.text")]
pub unsafe extern "C" fn bit_wait_timeout(word: *mut wait_bit_key, mode: c_int) -> c_int {
    // SAFETY: The caller upholds the native wait API pointer and locking contract.
    unsafe {
    let now = lupos_waiting_read_jiffies();
    if lupos_waiting_time_after_eq(now, (*word).timeout) { return -LUPOS_WAITING_EAGAIN; }
    schedule_timeout((*word).timeout.wrapping_sub(now) as c_long);
    if lupos_waiting_signal_pending_state(mode, lupos_waiting_current()) { return -LUPOS_WAITING_EINTR; }
    0

    }
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".init.text")]
pub unsafe extern "C" fn wait_bit_init() {
    // SAFETY: The caller upholds the native wait API pointer and locking contract.
    unsafe {
    for i in 0..LUPOS_WAITING_WAIT_TABLE_SIZE {
        lupos_waiting_init_bit_waitqueue_head(lupos_waiting_bit_table().add(i as usize));
    }

    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
