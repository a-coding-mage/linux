// Translated from <linux/call_once.h>.
// Dependencies supplied by other files:
// linux/types.h, linux/mutex.h

pub const ONCE_NOT_STARTED: i32 = 0;
pub const ONCE_RUNNING: i32 = 1;
pub const ONCE_COMPLETED: i32 = 2;

#[repr(C)]
pub struct once {
    pub state: atomic_t,
    pub lock: mutex,
}

#[inline]
pub unsafe fn __once_init(
    once: *mut once,
    name: *const core::ffi::c_char,
    key: *mut lock_class_key,
) {
    atomic_set(&mut (*once).state, ONCE_NOT_STARTED);
    __mutex_init(&mut (*once).lock, name, key);
}

#[macro_export]
macro_rules! once_init {
    ($once:expr) => {{
        static mut __KEY: lock_class_key = lock_class_key::new();
        unsafe {
            __once_init(
                $once,
                concat!(stringify!($once), "\0").as_ptr() as *const core::ffi::c_char,
                &raw mut __KEY,
            );
        }
    }};
}

/*
 * call_once - Ensure a function has been called exactly once
 *
 * @once: Tracking struct
 * @cb: Function to be called
 *
 * If @once has never completed successfully before, call @cb and, if
 * it returns a zero or positive value, mark @once as completed.  Return
 * the value returned by @cb
 *
 * If @once has completed successfully before, return 0.
 *
 * The call to @cb is implicitly surrounded by a mutex, though for
 * efficiency the function avoids taking it after the first call.
 */
#[inline]
pub unsafe fn call_once(once: *mut once, cb: unsafe extern "C" fn(*mut once) -> i32) -> i32 {
    let r: i32;
    let state: i32;

    /* Pairs with atomic_set_release() below. */
    if atomic_read_acquire(&(*once).state) == ONCE_COMPLETED {
        return 0;
    }

    let _guard = guard_mutex(&mut (*once).lock);
    state = atomic_read(&(*once).state);
    if unlikely(state != ONCE_NOT_STARTED) {
        return if warn_on_once(state != ONCE_COMPLETED) {
            -EINVAL
        } else {
            0
        };
    }

    atomic_set(&mut (*once).state, ONCE_RUNNING);
    r = cb(once);
    if r < 0 {
        atomic_set(&mut (*once).state, ONCE_NOT_STARTED);
    } else {
        atomic_set_release(&mut (*once).state, ONCE_COMPLETED);
    }
    r
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
