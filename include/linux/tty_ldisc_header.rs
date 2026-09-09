/* SPDX-License-Identifier: GPL-2.0 */

/* Translated from linux/tty_ldisc.h. */

#[repr(C)]
pub struct ld_semaphore {
    pub count: atomic_long_t,
    pub wait_lock: raw_spinlock_t,
    pub wait_readers: ::core::ffi::c_uint,
    pub read_wait: list_head,
    pub write_wait: list_head,
    /* Present only when CONFIG_DEBUG_LOCK_ALLOC is enabled. */
    #[cfg(CONFIG_DEBUG_LOCK_ALLOC)]
    pub dep_map: lockdep_map,
}

pub unsafe extern "C" fn __init_ldsem(
    sem: *mut ld_semaphore,
    name: *const ::core::ffi::c_char,
    key: *mut lock_class_key,
);

/* C macro init_ldsem(sem), including its function-local static key. */
#[macro_export]
macro_rules! init_ldsem {
    ($sem:expr) => {{
        static mut __key: lock_class_key = unsafe { ::core::mem::zeroed() };
        unsafe {
            __init_ldsem(
                ($sem),
                concat!(stringify!($sem), "\0").as_ptr() as *const ::core::ffi::c_char,
                &mut __key,
            );
        }
    }};
}

pub unsafe extern "C" fn ldsem_down_read(
    sem: *mut ld_semaphore,
    timeout: ::core::ffi::c_long,
) -> ::core::ffi::c_int;
pub unsafe extern "C" fn ldsem_down_read_trylock(sem: *mut ld_semaphore) -> ::core::ffi::c_int;
pub unsafe extern "C" fn ldsem_down_write(
    sem: *mut ld_semaphore,
    timeout: ::core::ffi::c_long,
) -> ::core::ffi::c_int;
pub unsafe extern "C" fn ldsem_up_read(sem: *mut ld_semaphore);
pub unsafe extern "C" fn ldsem_up_write(sem: *mut ld_semaphore);

#[cfg(CONFIG_DEBUG_LOCK_ALLOC)]
pub unsafe extern "C" fn ldsem_down_read_nested(
    sem: *mut ld_semaphore,
    subclass: ::core::ffi::c_int,
    timeout: ::core::ffi::c_long,
) -> ::core::ffi::c_int;
#[cfg(CONFIG_DEBUG_LOCK_ALLOC)]
pub unsafe extern "C" fn ldsem_down_write_nested(
    sem: *mut ld_semaphore,
    subclass: ::core::ffi::c_int,
    timeout: ::core::ffi::c_long,
) -> ::core::ffi::c_int;

#[cfg(not(CONFIG_DEBUG_LOCK_ALLOC))]
#[macro_export]
macro_rules! ldsem_down_read_nested { ($sem:expr, $subclass:expr, $timeout:expr) => { ldsem_down_read($sem, $timeout) }; }
#[cfg(not(CONFIG_DEBUG_LOCK_ALLOC))]
#[macro_export]
macro_rules! ldsem_down_write_nested { ($sem:expr, $subclass:expr, $timeout:expr) => { ldsem_down_write($sem, $timeout) }; }

#[repr(C)]
pub struct tty_ldisc_ops {
    pub name: *mut ::core::ffi::c_char,
    pub num: ::core::ffi::c_int,
    pub open: Option<unsafe extern "C" fn(tty: *mut tty_struct) -> ::core::ffi::c_int>,
    pub close: Option<unsafe extern "C" fn(tty: *mut tty_struct)>,
    pub flush_buffer: Option<unsafe extern "C" fn(tty: *mut tty_struct)>,
    pub read: Option<unsafe extern "C" fn(tty: *mut tty_struct, file: *mut file, buf: *mut u8, nr: usize, cookie: *mut *mut ::core::ffi::c_void, offset: ::core::ffi::c_ulong) -> ssize_t>,
    pub write: Option<unsafe extern "C" fn(tty: *mut tty_struct, file: *mut file, buf: *const u8, nr: usize) -> ssize_t>,
    pub ioctl: Option<unsafe extern "C" fn(tty: *mut tty_struct, cmd: ::core::ffi::c_uint, arg: ::core::ffi::c_ulong) -> ::core::ffi::c_int>,
    pub compat_ioctl: Option<unsafe extern "C" fn(tty: *mut tty_struct, cmd: ::core::ffi::c_uint, arg: ::core::ffi::c_ulong) -> ::core::ffi::c_int>,
    pub set_termios: Option<unsafe extern "C" fn(tty: *mut tty_struct, old: *const ktermios)>,
    pub poll: Option<unsafe extern "C" fn(tty: *mut tty_struct, file: *mut file, wait: *mut poll_table_struct) -> __poll_t>,
    pub hangup: Option<unsafe extern "C" fn(tty: *mut tty_struct)>,
    pub receive_buf: Option<unsafe extern "C" fn(tty: *mut tty_struct, cp: *const u8, fp: *const u8, count: usize)>,
    pub write_wakeup: Option<unsafe extern "C" fn(tty: *mut tty_struct)>,
    pub dcd_change: Option<unsafe extern "C" fn(tty: *mut tty_struct, active: bool)>,
    pub receive_buf2: Option<unsafe extern "C" fn(tty: *mut tty_struct, cp: *const u8, fp: *const u8, count: usize) -> usize>,
    pub lookahead_buf: Option<unsafe extern "C" fn(tty: *mut tty_struct, cp: *const u8, fp: *const u8, count: usize)>,
    pub owner: *mut module,
}

#[repr(C)]
pub struct tty_ldisc {
    pub ops: *const tty_ldisc_ops,
    pub tty: *mut tty_struct,
}

/* MODULE_ALIAS_LDISC(ldisc) expands to MODULE_ALIAS("tty-ldisc-" __stringify(ldisc)). */
#[macro_export]
macro_rules! MODULE_ALIAS_LDISC { ($ldisc:ident) => { MODULE_ALIAS!(concat!("tty-ldisc-", stringify!($ldisc))) }; }

extern "C" {
    pub static tty_ldiscs_seq_ops: seq_operations;
    pub fn tty_ldisc_ref(tty: *mut tty_struct) -> *mut tty_ldisc;
    pub fn tty_ldisc_deref(ld: *mut tty_ldisc);
    pub fn tty_ldisc_ref_wait(tty: *mut tty_struct) -> *mut tty_ldisc;
    pub fn tty_ldisc_flush(tty: *mut tty_struct);
    pub fn tty_register_ldisc(new_ldisc: *const tty_ldisc_ops) -> ::core::ffi::c_int;
    pub fn tty_unregister_ldisc(ldisc: *const tty_ldisc_ops);
    pub fn tty_set_ldisc(tty: *mut tty_struct, disc: ::core::ffi::c_int) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
