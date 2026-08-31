/* SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause) */

/*
 * Dependencies from the original C header:
 * errno.h, fcntl.h, sched.h, stdatomic.h, stddef.h, stdlib.h, string.h,
 * unistd.h, sys/syscall.h, sys/types.h, and bpf/bpf.h.
 *
 * If TLD_FREE_DATA_ON_THREAD_EXIT is enabled, pthread.h is also required.
 */

use core::ffi::{c_char, c_int, c_long, c_void};
use core::mem;
use core::ptr;
use core::sync::atomic::{AtomicBool, AtomicPtr, AtomicU16, Ordering};

/*
 * OPTIONS
 *
 *   Define the option before including the header. Using different options in
 *   different translation units is strongly discouraged.
 *
 *   TLD_FREE_DATA_ON_THREAD_EXIT - Frees memory on thread exit automatically
 *
 *   Thread-specific memory for storing TLD is allocated lazily on the first call to
 *   tld_get_data(). The thread that calls it must also call tld_free() on thread exit
 *   to prevent memory leak. Pthread will be included if the option is defined. A pthread
 *   key will be registered with a destructor that calls tld_free(). Enabled only when
 *   the option is defined and TLD_DEFINE_KEY/tld_create_key() is called in the same
 *   translation unit.
 *
 *
 *   TLD_DYN_DATA_SIZE - The maximum size of memory allocated for TLDs created dynamically
 *   (default: 64 bytes)
 *
 *   A TLD can be defined statically using TLD_DEFINE_KEY() or created on the fly using
 *   tld_create_key(). As the total size of TLDs created with tld_create_key() cannot be
 *   possibly known statically, a memory area of size TLD_DYN_DATA_SIZE will be allocated
 *   for these TLDs. This additional memory is allocated for every thread that calls
 *   tld_get_data() even if no tld_create_key are actually called, so be mindful of
 *   potential memory wastage. Use TLD_DEFINE_KEY() whenever possible as just enough memory
 *   will be allocated for TLDs created with it.
 *
 *
 *   TLD_NAME_LEN - The maximum length of the name of a TLD (default: 62)
 *
 *   Setting TLD_NAME_LEN will affect the maximum number of TLDs a process can store,
 *   TLD_MAX_DATA_CNT. Must be consistent with task_local_data.bpf.h.
 *
 *
 *   TLD_DONT_ROUND_UP_DATA_SIZE - Don't round up memory size allocated for data if
 *   the memory allocator has low overhead aligned_alloc() implementation.
 *
 *   For some memory allocators, when calling aligned_alloc(alignment, size), size
 *   does not need to be an integral multiple of alignment and it can be fulfilled
 *   without using round_up(size, alignment) bytes of memory. Enable this option to
 *   reduce memory usage.
 */

pub type __s16 = i16;
pub type __u16 = u16;
pub type __u64 = u64;
pub type size_t = usize;
pub type pthread_key_t = u32;

pub const ENOMEM: c_int = 12;
pub const EEXIST: c_int = 17;
pub const E2BIG: c_int = 7;
pub const ENOSPC: c_int = 28;
pub const O_EXCL: c_int = 0o200;
pub const SYS_pidfd_open: c_long = 434;

pub const TLD_DYN_DATA_SIZE: __u16 = 64;
pub const TLD_NAME_LEN: usize = 62;

unsafe extern "C" {
    fn getpagesize() -> c_int;
    fn aligned_alloc(alignment: size_t, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int;
    fn strscpy(dest: *mut c_char, src: *const c_char) -> isize;
    fn sched_yield() -> c_int;
    fn syscall(num: c_long, ...) -> c_long;
    fn sys_gettid() -> c_long;
    fn close(fd: c_int) -> c_int;
    fn bpf_map_update_elem(
        fd: c_int,
        key: *const c_void,
        value: *const c_void,
        flags: u64,
    ) -> c_int;
    fn __errno_location() -> *mut c_int;

    /*
     * TLD_FREE_DATA_ON_THREAD_EXIT dependency. The original C header only
     * declares and uses these when that option is defined.
     */
    fn pthread_key_create(
        key: *mut pthread_key_t,
        destructor: Option<unsafe extern "C" fn(*mut c_void)>,
    ) -> c_int;
    fn pthread_setspecific(key: pthread_key_t, value: *const c_void) -> c_int;
}

#[inline]
pub unsafe fn TLD_PAGE_SIZE() -> usize {
    unsafe { getpagesize() as usize }
}

#[inline]
pub unsafe fn TLD_PAGE_MASK() -> isize {
    unsafe { !((TLD_PAGE_SIZE() as isize) - 1) }
}

#[inline]
pub fn TLD_ROUND_MASK(x: usize, y: usize) -> usize {
    let _ = x;
    y.wrapping_sub(1)
}

#[inline]
pub fn TLD_ROUND_UP(x: usize, y: usize) -> usize {
    ((x.wrapping_sub(1)) | TLD_ROUND_MASK(x, y)).wrapping_add(1)
}

#[inline]
pub fn TLD_ROUND_UP_POWER_OF_TWO(x: usize) -> usize {
    1usize << (mem::size_of_val(&x) * 8 - (x.wrapping_sub(1)).leading_zeros() as usize)
}

#[inline]
pub unsafe fn TLD_MAX_DATA_CNT() -> usize {
    unsafe { TLD_PAGE_SIZE() / mem::size_of::<tld_metadata>() - 1 }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tld_key_t {
    pub off: __s16,
}

#[repr(C)]
pub struct tld_metadata {
    pub name: [c_char; TLD_NAME_LEN],
    pub size: AtomicU16, /* size of tld_data_u->data */
}

#[repr(C)]
pub struct tld_meta_u {
    pub cnt: AtomicU16,
    pub size: __u16,
    pub metadata: [tld_metadata; 0],
}

/*
 * The unused field ensures map_val.start > 0. On the BPF side, __tld_fetch_key()
 * calculates off by summing map_val.start and tld_key_t.off and treats off == 0
 * as key not cached.
 */
#[repr(C)]
pub struct tld_data_u {
    pub unused: __u64,
    pub data: [c_char; 0],
}

#[repr(C)]
pub struct tld_map_value {
    pub data: *mut c_void,
    pub meta: *mut tld_meta_u,
    pub start: __u16, /* offset of tld_data_u->data in a page */
}

#[unsafe(no_mangle)]
pub static tld_meta_p: AtomicPtr<tld_meta_u> = AtomicPtr::new(ptr::null_mut());

#[thread_local]
#[unsafe(no_mangle)]
pub static mut tld_data_p: *mut tld_data_u = ptr::null_mut();

/*
 * TLD_FREE_DATA_ON_THREAD_EXIT globals. The original C header declares these
 * only when the option is defined.
 */
#[unsafe(no_mangle)]
pub static tld_pthread_key_init: AtomicBool = AtomicBool::new(false);

#[unsafe(no_mangle)]
pub static mut tld_pthread_key: pthread_key_t = 0;

unsafe extern "C" fn __tld_thread_exit_handler(unused: *mut c_void) {
    let _ = unused;
    unsafe {
        tld_free();
    }
}

unsafe fn __tld_init_meta_p() -> c_int {
    let mut meta: *mut tld_meta_u;
    let mut uninit: *mut tld_meta_u = ptr::null_mut();
    let mut err: c_int = 0;

    meta = unsafe { aligned_alloc(TLD_PAGE_SIZE(), TLD_PAGE_SIZE()) as *mut tld_meta_u };
    if meta.is_null() {
        err = -ENOMEM;
        return err;
    }

    unsafe {
        memset(meta as *mut c_void, 0, TLD_PAGE_SIZE());
        (*meta).size = TLD_DYN_DATA_SIZE;
    }

    if tld_meta_p
        .compare_exchange(uninit, meta, Ordering::SeqCst, Ordering::SeqCst)
        .is_err()
    {
        unsafe {
            free(meta as *mut c_void);
        }
    }

    err
}

unsafe fn __tld_init_data_p(map_fd: c_int) -> c_int {
    let mut map_val: tld_map_value = unsafe { mem::zeroed() };
    let data: *mut tld_data_u;
    let mut err: c_int;
    let mut tid_fd: c_int = -1;
    let size: size_t;
    let size_pot: size_t;

    tid_fd = unsafe { syscall(SYS_pidfd_open, sys_gettid(), O_EXCL) as c_int };
    if tid_fd < 0 {
        err = unsafe { -*__errno_location() };
        return err;
    }

    /*
     * tld_meta_p->size = TLD_DYN_DATA_SIZE +
     *          total size of TLDs defined via TLD_DEFINE_KEY()
     */
    let meta = tld_meta_p.load(Ordering::SeqCst);
    size = unsafe { (*meta).size as usize + mem::size_of::<tld_data_u>() };
    size_pot = TLD_ROUND_UP_POWER_OF_TWO(size);

    /*
     * If TLD_DONT_ROUND_UP_DATA_SIZE is enabled in the C header:
     * data = aligned_alloc(size_pot, size)
     * Otherwise:
     * data = aligned_alloc(size_pot, size_pot)
     */
    data = unsafe { aligned_alloc(size_pot, size_pot) as *mut tld_data_u };
    if data.is_null() {
        err = -ENOMEM;
        unsafe {
            close(tid_fd);
        }
        return err;
    }

    /*
     * Always pass a page-aligned address to UPTR since the size of tld_map_value::data
     * is a page in BTF.
     */
    let page_mask = unsafe { TLD_PAGE_MASK() };
    map_val.data = ((page_mask & data as isize) as usize) as *mut c_void;
    map_val.start =
        (((!page_mask & data as isize) as usize) + mem::size_of::<tld_data_u>()) as __u16;
    map_val.meta = meta;

    err = unsafe {
        bpf_map_update_elem(
            map_fd,
            &tid_fd as *const c_int as *const c_void,
            &map_val as *const tld_map_value as *const c_void,
            0,
        )
    };
    if err != 0 {
        unsafe {
            free(data as *mut c_void);
            close(tid_fd);
        }
        return err;
    }

    unsafe {
        tld_data_p = data;
        /*
         * TLD_FREE_DATA_ON_THREAD_EXIT:
         * pthread_setspecific(tld_pthread_key, (void *)1);
         */
        pthread_setspecific(tld_pthread_key, 1usize as *const c_void);
    }

    if tid_fd >= 0 {
        unsafe {
            close(tid_fd);
        }
    }
    err
}

unsafe fn __tld_create_key(name: *const c_char, size: size_t, dyn_data: bool) -> tld_key_t {
    let mut err: c_int;
    let mut i: c_int;
    let mut sz: c_int;
    let mut off: c_int = 0;
    let mut uninit = false;
    let mut cnt: __u16;

    if tld_meta_p.load(Ordering::SeqCst).is_null() {
        err = unsafe { __tld_init_meta_p() };
        if err != 0 {
            return tld_key_t { off: err as __s16 };
        }
    }

    /*
     * TLD_FREE_DATA_ON_THREAD_EXIT:
     * Initialize the pthread key once and register __tld_thread_exit_handler.
     */
    if tld_pthread_key_init
        .compare_exchange(uninit, true, Ordering::SeqCst, Ordering::SeqCst)
        .is_ok()
    {
        err = unsafe { pthread_key_create(&raw mut tld_pthread_key, Some(__tld_thread_exit_handler)) };
        if err != 0 {
            return tld_key_t { off: err as __s16 };
        }
    }

    i = 0;
    while i < unsafe { TLD_MAX_DATA_CNT() as c_int } {
        loop {
            let meta = tld_meta_p.load(Ordering::SeqCst);
            cnt = unsafe { (*meta).cnt.load(Ordering::SeqCst) };
            if i < cnt as c_int {
                /* A metadata is not ready until size is updated with a non-zero value */
                loop {
                    sz = unsafe {
                        (*((*meta).metadata.as_ptr().add(i as usize))).size.load(Ordering::SeqCst)
                            as c_int
                    };
                    if sz != 0 {
                        break;
                    }
                    unsafe {
                        sched_yield();
                    }
                }

                if unsafe {
                    strncmp(
                        (*((*meta).metadata.as_ptr().add(i as usize))).name.as_ptr(),
                        name,
                        TLD_NAME_LEN,
                    )
                } == 0
                {
                    return tld_key_t {
                        off: (-EEXIST) as __s16,
                    };
                }

                off += TLD_ROUND_UP(sz as usize, 8) as c_int;
                break;
            }

            /*
             * TLD_DEFINE_KEY() is given memory upto a page while at most
             * TLD_DYN_DATA_SIZE is allocated for tld_create_key()
             */
            if dyn_data {
                if off + TLD_ROUND_UP(size, 8) as c_int > unsafe { (*meta).size as c_int }
                    || unsafe { (*meta).size as usize }
                        > unsafe { TLD_PAGE_SIZE() } - mem::size_of::<tld_data_u>()
                {
                    return tld_key_t {
                        off: (-E2BIG) as __s16,
                    };
                }
            } else {
                if off + TLD_ROUND_UP(size, 8) as c_int
                    > (unsafe { TLD_PAGE_SIZE() } - mem::size_of::<tld_data_u>()) as c_int
                {
                    return tld_key_t {
                        off: (-E2BIG) as __s16,
                    };
                }
                unsafe {
                    (*meta).size = ((*meta).size as usize + TLD_ROUND_UP(size, 8)) as __u16;
                }
            }

            /*
             * Only one tld_create_key() can increase the current cnt by one and
             * takes the latest available slot. Other threads will check again if a new
             * TLD can still be added, and then compete for the new slot after the
             * succeeding thread update the size.
             */
            if unsafe {
                (*meta)
                    .cnt
                    .compare_exchange(cnt, cnt.wrapping_add(1), Ordering::SeqCst, Ordering::SeqCst)
                    .is_err()
            } {
                continue;
            }

            unsafe {
                strscpy(
                    (*((*meta).metadata.as_mut_ptr().add(i as usize))).name.as_mut_ptr(),
                    name,
                );
                (*((*meta).metadata.as_ptr().add(i as usize)))
                    .size
                    .store(size as __u16, Ordering::SeqCst);
            }
            return tld_key_t { off: off as __s16 };
        }
        i += 1;
    }

    tld_key_t {
        off: (-ENOSPC) as __s16,
    }
}

/**
 * TLD_DEFINE_KEY() - Define a TLD and a global variable key associated with the TLD.
 *
 * @name: The name of the TLD
 * @size: The size of the TLD
 * @key: The variable name of the key. Cannot exceed TLD_NAME_LEN
 *
 * The macro can only be used in file scope.
 *
 * A global variable key of opaque type, tld_key_t, will be declared and initialized before
 * main() starts. Use tld_key_is_err() or tld_key_err_or_zero() later to check if the key
 * creation succeeded. Pass the key to tld_get_data() to get a pointer to the TLD.
 * bpf programs can also fetch the same key by name.
 *
 * The total size of TLDs created using TLD_DEFINE_KEY() cannot exceed a page. Just
 * enough memory will be allocated for each thread on the first call to tld_get_data().
 */
#[macro_export]
macro_rules! TLD_DEFINE_KEY {
    ($key:ident, $name:expr, $size:expr) => {
        static mut $key: $crate::tld_key_t = $crate::tld_key_t { off: 0 };
        /*
         * Original C uses __attribute__((constructor(101))) to initialize:
         * key = __tld_create_key(name, size, false);
         */
    };
}

/**
 * tld_create_key() - Create a TLD and return a key associated with the TLD.
 *
 * @name: The name the TLD
 * @size: The size of the TLD
 *
 * Return an opaque object key. Use tld_key_is_err() or tld_key_err_or_zero() to check
 * if the key creation succeeded. Pass the key to tld_get_data() to get a pointer to
 * locate the TLD. bpf programs can also fetch the same key by name.
 *
 * Use tld_create_key() only when a TLD needs to be created dynamically (e.g., @name is
 * not known statically or a TLD needs to be created conditionally)
 *
 * An additional TLD_DYN_DATA_SIZE bytes are allocated per-thread to accommodate TLDs
 * created dynamically with tld_create_key(). Since only a user page is pinned to the
 * kernel, when TLDs created with TLD_DEFINE_KEY() uses more than TLD_PAGE_SIZE -
 * TLD_DYN_DATA_SIZE, the buffer size will be limited to the rest of the page.
 */
pub unsafe fn tld_create_key(name: *const c_char, size: size_t) -> tld_key_t {
    unsafe { __tld_create_key(name, size, true) }
}

#[inline]
pub fn tld_key_is_err(key: tld_key_t) -> bool {
    key.off < 0
}

#[inline]
pub fn tld_key_err_or_zero(key: tld_key_t) -> c_int {
    if tld_key_is_err(key) {
        key.off as c_int
    } else {
        0
    }
}

/**
 * tld_get_data() - Get a pointer to the TLD associated with the given key of the
 * calling thread.
 *
 * @map_fd: A file descriptor of tld_data_map, the underlying BPF task local storage map
 * of task local data.
 * @key: A key object created by TLD_DEFINE_KEY() or tld_create_key().
 *
 * Return a pointer to the TLD if the key is valid; NULL if not enough memory for TLD
 * for this thread, or the key is invalid. The returned pointer is guaranteed to be 8-byte
 * aligned.
 *
 * Threads that call tld_get_data() must call tld_free() on exit to prevent
 * memory leak if TLD_FREE_DATA_ON_THREAD_EXIT is not defined.
 */
pub unsafe fn tld_get_data(map_fd: c_int, key: tld_key_t) -> *mut c_void {
    if tld_meta_p.load(Ordering::SeqCst).is_null() {
        return ptr::null_mut();
    }

    /* tld_data_p is allocated on the first invocation of tld_get_data() */
    unsafe {
        if tld_data_p.is_null() && __tld_init_data_p(map_fd) != 0 {
            return ptr::null_mut();
        }

        (*tld_data_p)
            .data
            .as_mut_ptr()
            .offset(key.off as isize) as *mut c_void
    }
}

/**
 * tld_free() - Free task local data memory of the calling thread
 *
 * For the calling thread, all pointers to TLDs acquired before will become invalid.
 *
 * Users must call tld_free() on thread exit to prevent memory leak. Alternatively,
 * define TLD_FREE_DATA_ON_THREAD_EXIT and a thread exit handler will be registered
 * to free the memory automatically. Calling tld_free() before thread exit is
 * undefined behavior, which may lead to null-pointer dereference.
 */
pub unsafe fn tld_free() {
    unsafe {
        if !tld_data_p.is_null() {
            free(tld_data_p as *mut c_void);
            tld_data_p = ptr::null_mut();
        }
    }
}
