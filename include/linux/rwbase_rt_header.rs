// SPDX-License-Identifier: GPL-2.0-only

// Dependencies supplied by the corresponding Linux Rust bindings:
// use linux::atomic::atomic_t;
// use linux::rtmutex::rt_mutex_base;

pub const READER_BIAS: u32 = 1u32 << 31;
pub const WRITER_BIAS: u32 = 1u32 << 30;

#[repr(C)]
pub struct rwbase_rt {
    pub readers: atomic_t,
    pub rtmutex: rt_mutex_base,
}

#[macro_export]
macro_rules! __RWBASE_INITIALIZER {
    ($name:ident) => {
        $crate::rwbase_rt {
            readers: $crate::ATOMIC_INIT($crate::READER_BIAS),
            rtmutex: $crate::__RT_MUTEX_BASE_INITIALIZER!($name.rtmutex),
        }
    };
}

#[macro_export]
macro_rules! init_rwbase_rt {
    ($rwbase:expr) => {{
        unsafe {
            rt_mutex_base_init(&mut (*($rwbase as *mut rwbase_rt)).rtmutex);
            atomic_set(
                &mut (*($rwbase as *mut rwbase_rt)).readers,
                READER_BIAS as i32,
            );
        }
    }};
}

extern "C" {
    fn atomic_read(v: *const atomic_t) -> i32;
    fn atomic_set(v: *mut atomic_t, value: i32);
    fn rt_mutex_base_init(lock: *mut rt_mutex_base);
}

#[inline(always)]
pub unsafe fn rw_base_is_locked(rwb: *const rwbase_rt) -> bool {
    atomic_read(&(*rwb).readers) != READER_BIAS as i32
}

#[inline(always)]
pub unsafe fn rw_base_is_write_locked(rwb: *const rwbase_rt) -> bool {
    atomic_read(&(*rwb).readers) == WRITER_BIAS as i32
}

#[inline(always)]
pub unsafe fn rw_base_is_contended(rwb: *const rwbase_rt) -> bool {
    atomic_read(&(*rwb).readers) > 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
