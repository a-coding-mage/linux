// SPDX-License-Identifier: GPL-2.0-only

// Dependencies supplied by the surrounding kernel translation unit.

const WAIT_TABLE_BITS: usize = 8;
const WAIT_TABLE_SIZE: usize = 1 << WAIT_TABLE_BITS;

static mut BIT_WAIT_TABLE: [wait_queue_head_t; WAIT_TABLE_SIZE] = [unsafe { core::mem::zeroed() }; WAIT_TABLE_SIZE];

pub unsafe fn bit_waitqueue(word: *mut c_ulong, bit: c_int) -> *mut wait_queue_head_t {
    let shift: c_int = if BITS_PER_LONG == 32 { 5 } else { 6 };
    let val = (word as c_ulong).wrapping_shl(shift as u32) | bit as c_ulong;
    BIT_WAIT_TABLE.as_mut_ptr().add(hash_long(val, WAIT_TABLE_BITS as c_int) as usize)
}

pub unsafe fn wake_bit_function(
    wq_entry: *mut wait_queue_entry,
    mode: c_uint,
    sync: c_int,
    arg: *mut c_void,
) -> c_int {
    let key = arg as *mut wait_bit_key;
    let wait_bit = container_of!(wq_entry, wait_bit_queue_entry, wq_entry);

    if (*wait_bit).key.flags != (*key).flags
        || (*wait_bit).key.bit_nr != (*key).bit_nr
        || test_bit((*key).bit_nr, (*key).flags)
    {
        return 0;
    }

    autoremove_wake_function(wq_entry, mode, sync, key as *mut c_void)
}

pub unsafe fn __wait_on_bit(
    wq_head: *mut wait_queue_head_t,
    wbq_entry: *mut wait_bit_queue_entry,
    action: wait_bit_action_f,
    mode: c_uint,
) -> c_int {
    let mut ret: c_int = 0;
    loop {
        prepare_to_wait(wq_head, &mut (*wbq_entry).wq_entry, mode);
        if test_bit((*wbq_entry).key.bit_nr, (*wbq_entry).key.flags) {
            ret = action(&mut (*wbq_entry).key, mode);
        }
        if !(test_bit_acquire((*wbq_entry).key.bit_nr, (*wbq_entry).key.flags) && ret == 0) {
            break;
        }
    }
    finish_wait(wq_head, &mut (*wbq_entry).wq_entry);
    ret
}

pub unsafe fn out_of_line_wait_on_bit(
    word: *mut c_ulong, bit: c_int, action: wait_bit_action_f, mode: c_uint,
) -> c_int {
    let wq_head = bit_waitqueue(word, bit);
    let mut wq_entry = DEFINE_WAIT_BIT!(word, bit);
    __wait_on_bit(wq_head, &mut wq_entry, action, mode)
}

pub unsafe fn out_of_line_wait_on_bit_timeout(
    word: *mut c_ulong, bit: c_int, action: wait_bit_action_f, mode: c_uint, timeout: c_ulong,
) -> c_int {
    let wq_head = bit_waitqueue(word, bit);
    let mut wq_entry = DEFINE_WAIT_BIT!(word, bit);
    wq_entry.key.timeout = jiffies.wrapping_add(timeout);
    __wait_on_bit(wq_head, &mut wq_entry, action, mode)
}

pub unsafe fn __wait_on_bit_lock(
    wq_head: *mut wait_queue_head_t,
    wbq_entry: *mut wait_bit_queue_entry,
    action: wait_bit_action_f,
    mode: c_uint,
) -> c_int {
    let mut ret: c_int = 0;
    loop {
        prepare_to_wait_exclusive(wq_head, &mut (*wbq_entry).wq_entry, mode);
        if test_bit((*wbq_entry).key.bit_nr, (*wbq_entry).key.flags) {
            ret = action(&mut (*wbq_entry).key, mode);
            // See the comment in prepare_to_wait_event().
            // finish_wait() does not necessarily take wq_head->lock, but
            // test_and_set_bit() implies mb() which pairs with
            // smp_mb__after_atomic() before wake_up_page().
            if ret != 0 {
                finish_wait(wq_head, &mut (*wbq_entry).wq_entry);
            }
        }
        if !test_and_set_bit((*wbq_entry).key.bit_nr, (*wbq_entry).key.flags) {
            if ret == 0 {
                finish_wait(wq_head, &mut (*wbq_entry).wq_entry);
            }
            return 0;
        } else if ret != 0 {
            return ret;
        }
    }
}

pub unsafe fn out_of_line_wait_on_bit_lock(
    word: *mut c_ulong, bit: c_int, action: wait_bit_action_f, mode: c_uint,
) -> c_int {
    let wq_head = bit_waitqueue(word, bit);
    let mut wq_entry = DEFINE_WAIT_BIT!(word, bit);
    __wait_on_bit_lock(wq_head, &mut wq_entry, action, mode)
}

pub unsafe fn __wake_up_bit(wq_head: *mut wait_queue_head_t, word: *mut c_ulong, bit: c_int) {
    let mut key = WAIT_BIT_KEY_INITIALIZER!(word, bit);
    if waitqueue_active(wq_head) {
        __wake_up(wq_head, TASK_NORMAL, 1, &mut key as *mut _ as *mut c_void);
    }
}

/// wake_up_bit - wake up waiters on a bit
/// @word: the address containing the bit being waited on
/// @bit: the bit at that address being waited on
///
/// Wake up any process waiting in wait_on_bit() or similar for the given bit to be cleared.
pub unsafe fn wake_up_bit(word: *mut c_ulong, bit: c_int) {
    __wake_up_bit(bit_waitqueue(word, bit), word, bit);
}

pub unsafe fn __var_waitqueue(p: *mut c_void) -> *mut wait_queue_head_t {
    BIT_WAIT_TABLE.as_mut_ptr().add(hash_ptr(p, WAIT_TABLE_BITS as c_int) as usize)
}

pub unsafe fn __var_wake_key(wq_entry: *mut wait_queue_entry, arg: *mut c_void) -> *mut wait_bit_key {
    let key = arg as *mut wait_bit_key;
    let wbq_entry = container_of!(wq_entry, wait_bit_queue_entry, wq_entry);
    if (*wbq_entry).key.flags != (*key).flags || (*wbq_entry).key.bit_nr != (*key).bit_nr {
        core::ptr::null_mut()
    } else {
        key
    }
}

unsafe fn var_wake_function(wq_entry: *mut wait_queue_entry, mode: c_uint, sync: c_int, arg: *mut c_void) -> c_int {
    let key = __var_wake_key(wq_entry, arg);
    if key.is_null() { return 0; }
    autoremove_wake_function(wq_entry, mode, sync, key as *mut c_void)
}

pub unsafe fn init_wait_var_entry(wbq_entry: *mut wait_bit_queue_entry, var: *mut c_void, flags: c_int) {
    (*wbq_entry).key.flags = var;
    (*wbq_entry).key.bit_nr = -1;
    (*wbq_entry).wq_entry.flags = flags;
    (*wbq_entry).wq_entry.private = current;
    (*wbq_entry).wq_entry.func = Some(var_wake_function);
    (*wbq_entry).wq_entry.entry = LIST_HEAD_INIT!((*wbq_entry).wq_entry.entry);
}

pub unsafe fn wake_up_var(var: *mut c_void) {
    __wake_up_bit(__var_waitqueue(var), var as *mut c_ulong, -1);
}

pub unsafe fn bit_wait(word: *mut wait_bit_key, mode: c_int) -> c_int {
    schedule();
    if signal_pending_state(mode, current) { return -EINTR; }
    0
}

pub unsafe fn bit_wait_io(word: *mut wait_bit_key, mode: c_int) -> c_int {
    io_schedule();
    if signal_pending_state(mode, current) { return -EINTR; }
    0
}

pub unsafe fn bit_wait_timeout(word: *mut wait_bit_key, mode: c_int) -> c_int {
    let now = READ_ONCE!(jiffies);
    if time_after_eq(now, (*word).timeout) { return -EAGAIN; }
    schedule_timeout((*word).timeout.wrapping_sub(now));
    if signal_pending_state(mode, current) { return -EINTR; }
    0
}

pub unsafe fn wait_bit_init() {
    for i in 0..WAIT_TABLE_SIZE {
        init_waitqueue_head(BIT_WAIT_TABLE.as_mut_ptr().add(i));
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
