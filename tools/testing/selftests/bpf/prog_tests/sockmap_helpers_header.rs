// Translated from testing/selftests/bpf/prog_tests/sockmap_helpers.h
// Dependency intent from C header: #include "socket_helpers.h"

pub const MAX_TEST_NAME: i32 = 80;

// C compound-literal helper intent:
// #define u32(v) ((u32){(v)})
// #define u64(v) ((u64){(v)})
#[macro_export]
macro_rules! u32 {
    ($v:expr) => {
        ($v) as u32
    };
}

#[macro_export]
macro_rules! u64 {
    ($v:expr) => {
        ($v) as u64
    };
}

// C attribute intent: #define __always_unused __attribute__((__unused__))

unsafe extern "C" {
    fn bpf_map_delete_elem(fd: i32, key: *const core::ffi::c_void) -> i32;
    fn bpf_map_lookup_elem(
        fd: i32,
        key: *const core::ffi::c_void,
        val: *mut core::ffi::c_void,
    ) -> i32;
    fn bpf_map_update_elem(
        fd: i32,
        key: *const core::ffi::c_void,
        val: *const core::ffi::c_void,
        flags: u64,
    ) -> i32;
    fn bpf_prog_attach(prog: i32, target: i32, type_: i32, flags: u32) -> i32;
    fn bpf_prog_detach2(prog: i32, target: i32, type_: i32) -> i32;
    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const pthread_attr_t,
        func: Option<unsafe extern "C" fn(*mut core::ffi::c_void) -> *mut core::ffi::c_void>,
        arg: *mut core::ffi::c_void,
    ) -> i32;
    fn pthread_join(thread: pthread_t, retval: *mut *mut core::ffi::c_void) -> i32;

    static mut errno: i32;
}

unsafe extern "C" {
    fn FAIL_ERRNO(msg: *const core::ffi::c_char);
}

// External pthread types supplied by translated dependencies.
pub type pthread_t = usize;
pub enum pthread_attr_t {}

#[macro_export]
macro_rules! xbpf_map_delete_elem {
    ($fd:expr, $key:expr) => {{
        let __ret = unsafe {
            bpf_map_delete_elem(
                ($fd),
                ($key) as *const core::ffi::c_void,
            )
        };
        if __ret < 0 {
            unsafe {
                FAIL_ERRNO(c"map_delete".as_ptr());
            }
        }
        __ret
    }};
}

#[macro_export]
macro_rules! xbpf_map_lookup_elem {
    ($fd:expr, $key:expr, $val:expr) => {{
        let __ret = unsafe {
            bpf_map_lookup_elem(
                ($fd),
                ($key) as *const core::ffi::c_void,
                ($val) as *mut core::ffi::c_void,
            )
        };
        if __ret < 0 {
            unsafe {
                FAIL_ERRNO(c"map_lookup".as_ptr());
            }
        }
        __ret
    }};
}

#[macro_export]
macro_rules! xbpf_map_update_elem {
    ($fd:expr, $key:expr, $val:expr, $flags:expr) => {{
        let __ret = unsafe {
            bpf_map_update_elem(
                ($fd),
                ($key) as *const core::ffi::c_void,
                ($val) as *const core::ffi::c_void,
                ($flags),
            )
        };
        if __ret < 0 {
            unsafe {
                FAIL_ERRNO(c"map_update".as_ptr());
            }
        }
        __ret
    }};
}

#[macro_export]
macro_rules! xbpf_prog_attach {
    ($prog:expr, $target:expr, $type:expr, $flags:expr) => {{
        let __ret = unsafe { bpf_prog_attach(($prog), ($target), ($type), ($flags)) };
        if __ret < 0 {
            unsafe {
                FAIL_ERRNO(concat!("prog_attach(", stringify!($type), ")\0").as_ptr() as *const core::ffi::c_char);
            }
        }
        __ret
    }};
}

#[macro_export]
macro_rules! xbpf_prog_detach2 {
    ($prog:expr, $target:expr, $type:expr) => {{
        let __ret = unsafe { bpf_prog_detach2(($prog), ($target), ($type)) };
        if __ret < 0 {
            unsafe {
                FAIL_ERRNO(concat!("prog_detach2(", stringify!($type), ")\0").as_ptr() as *const core::ffi::c_char);
            }
        }
        __ret
    }};
}

#[macro_export]
macro_rules! xpthread_create {
    ($thread:expr, $attr:expr, $func:expr, $arg:expr) => {{
        let __ret = unsafe { pthread_create(($thread), ($attr), ($func), ($arg)) };
        unsafe {
            errno = __ret;
        }
        if __ret != 0 {
            unsafe {
                FAIL_ERRNO(c"pthread_create".as_ptr());
            }
        }
        __ret
    }};
}

#[macro_export]
macro_rules! xpthread_join {
    ($thread:expr, $retval:expr) => {{
        let __ret = unsafe { pthread_join(($thread), ($retval)) };
        unsafe {
            errno = __ret;
        }
        if __ret != 0 {
            unsafe {
                FAIL_ERRNO(c"pthread_join".as_ptr());
            }
        }
        __ret
    }};
}

pub unsafe fn add_to_sockmap(mapfd: i32, fd1: i32, fd2: i32) -> i32 {
    let mut err: i32;

    err = xbpf_map_update_elem!(
        mapfd,
        &u32!(0) as *const u32,
        &u64!(fd1) as *const u64,
        BPF_NOEXIST
    );
    if err != 0 {
        return err;
    }

    xbpf_map_update_elem!(
        mapfd,
        &u32!(1) as *const u32,
        &u64!(fd2) as *const u64,
        BPF_NOEXIST
    )
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
