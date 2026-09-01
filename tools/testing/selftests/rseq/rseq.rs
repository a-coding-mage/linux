// SPDX-License-Identifier: LGPL-2.1
/*
 * rseq.c
 *
 * Copyright (C) 2016 Mathieu Desnoyers <mathieu.desnoyers@efficios.com>
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; only
 * version 2.1 of the License.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
 * Lesser General Public License for more details.
 */

// C dependencies: errno.h, sched.h, stdio.h, stdlib.h, string.h, unistd.h,
// syscall.h, assert.h, signal.h, limits.h, dlfcn.h, stddef.h, sys/auxv.h,
// linux/auxvec.h, linux/compiler.h, kselftest.h, rseq.h.

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::MaybeUninit;
use core::ptr;

type ptrdiff_t = isize;

extern "C" {
    static mut errno: c_int;

    static mut __rseq_offset: ptrdiff_t;
    static mut __rseq_size: c_uint;
    static mut __rseq_flags: c_uint;

    fn syscall(num: c_long, ...) -> c_long;
    fn sched_getcpu() -> c_int;
    fn perror(s: *const c_char);
    fn abort() -> !;
    fn getauxval(type_: c_ulong) -> c_ulong;
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;

    fn rseq_current_cpu_raw() -> i32;
    fn rseq_thread_pointer() -> *mut c_void;
}

// External constants supplied by headers in the original translation unit.
extern "C" {
    static __NR_rseq: c_long;
    static __NR_getcpu: c_long;
    static AT_RSEQ_ALIGN: c_ulong;
    static AT_RSEQ_FEATURE_SIZE: c_ulong;
    static RSEQ_ABI_CPU_ID_UNINITIALIZED: i32;
    static RSEQ_ABI_FLAG_UNREGISTER: c_int;
    static RSEQ_SIG: u32;
}

#[repr(C)]
pub struct rseq_abi {
    pub cpu_id_start: u32,
    pub cpu_id: u32,
    pub rseq_cs: u64,
    pub flags: u32,
    // Further fields, if any, are supplied by the external rseq ABI definition.
}

static mut libc_rseq_offset_p: *const ptrdiff_t = unsafe { &__rseq_offset as *const ptrdiff_t };
static mut libc_rseq_size_p: *const c_uint = unsafe { &__rseq_size as *const c_uint };
static mut libc_rseq_flags_p: *const c_uint = unsafe { &__rseq_flags as *const c_uint };

/* Offset from the thread pointer to the rseq area. */
#[no_mangle]
pub static mut rseq_offset: ptrdiff_t = 0;

/*
 * Size of the registered rseq area. 0 if the registration was
 * unsuccessful.
 */
#[no_mangle]
pub static mut rseq_size: c_uint = !0u32;
static mut rseq_alloc_size: c_uint = 0;

/* Flags used during rseq registration.  */
#[no_mangle]
pub static mut rseq_flags: c_uint = 0;

static mut rseq_ownership: c_int = 0;

/* Allocate a large area for the TLS. */
const RSEQ_THREAD_AREA_ALLOC_SIZE: usize = 1024;

/* Original struct rseq feature size is 20 bytes. */
const ORIG_RSEQ_FEATURE_SIZE: c_uint = 20;

/* Original struct rseq allocation size is 32 bytes. */
const ORIG_RSEQ_ALLOC_SIZE: c_uint = 32;

/*
 * Use a union to ensure we allocate a TLS area of 1024 bytes to accommodate an
 * rseq registration that is larger than the current rseq ABI.
 */
#[repr(C)]
union rseq_tls {
    abi: rseq_abi,
    dummy: [c_char; RSEQ_THREAD_AREA_ALLOC_SIZE],
}

thread_local! {
    static __rseq: rseq_tls = rseq_tls {
        abi: rseq_abi {
            cpu_id_start: 0,
            cpu_id: unsafe { RSEQ_ABI_CPU_ID_UNINITIALIZED as u32 },
            rseq_cs: 0,
            flags: 0,
        },
    };
}

unsafe fn RSEQ_READ_ONCE_u32(p: *const c_uint) -> c_uint {
    ptr::read_volatile(p)
}

unsafe fn RSEQ_WRITE_ONCE_u32(p: *mut c_uint, v: c_uint) {
    ptr::write_volatile(p, v);
}

unsafe fn sys_rseq(rseq_abi_p: *mut rseq_abi, rseq_len: u32, flags: c_int, sig: u32) -> c_int {
    syscall(__NR_rseq, rseq_abi_p, rseq_len, flags, sig) as c_int
}

unsafe fn sys_getcpu(cpu: *mut c_uint, node: *mut c_uint) -> c_int {
    syscall(__NR_getcpu, cpu, node, ptr::null_mut::<c_void>()) as c_int
}

#[no_mangle]
pub unsafe extern "C" fn rseq_available() -> bool {
    let rc: c_int;

    rc = sys_rseq(ptr::null_mut(), 0, 0, 0);
    if rc != -1 {
        abort();
    }
    match errno {
        38 => false, /* ENOSYS */
        22 => true,  /* EINVAL */
        _ => abort(),
    }
}

/*
 * Return the feature size supported by the kernel.
 *
 * Depending on the value returned by getauxval(AT_RSEQ_FEATURE_SIZE):
 *
 *   0: Return ORIG_RSEQ_FEATURE_SIZE (20)
 * > 0: Return the value from getauxval(AT_RSEQ_FEATURE_SIZE).
 *
 * It should never return a value below ORIG_RSEQ_FEATURE_SIZE.
 */
unsafe fn get_rseq_kernel_feature_size() -> c_uint {
    let auxv_rseq_feature_size: c_ulong;
    let auxv_rseq_align: c_ulong;

    auxv_rseq_align = getauxval(AT_RSEQ_ALIGN);
    assert!(!(auxv_rseq_align != 0 && auxv_rseq_align > RSEQ_THREAD_AREA_ALLOC_SIZE as c_ulong));

    auxv_rseq_feature_size = getauxval(AT_RSEQ_FEATURE_SIZE);
    assert!(
        !(auxv_rseq_feature_size != 0
            && auxv_rseq_feature_size > RSEQ_THREAD_AREA_ALLOC_SIZE as c_ulong)
    );
    if auxv_rseq_feature_size != 0 {
        auxv_rseq_feature_size as c_uint
    } else {
        ORIG_RSEQ_FEATURE_SIZE
    }
}

#[no_mangle]
pub unsafe extern "C" fn __rseq_register_current_thread(nolibc: bool, legacy: bool) -> c_int {
    let size: c_uint;
    let rc: c_int;

    if rseq_ownership == 0 {
        /* Treat libc's ownership as a successful registration. */
        return if nolibc { -16 } else { 0 }; /* -EBUSY */
    }

    /* The minimal allocation size is 32, which is the legacy allocation size */
    size = get_rseq_kernel_feature_size();
    if legacy || size < ORIG_RSEQ_ALLOC_SIZE {
        rseq_alloc_size = ORIG_RSEQ_ALLOC_SIZE;
    } else {
        rseq_alloc_size = size;
    }

    rc = __rseq.with(|r| sys_rseq((&(*r).abi) as *const rseq_abi as *mut rseq_abi, rseq_alloc_size, 0, RSEQ_SIG));
    if rc != 0 {
        /*
         * After at least one thread has registered successfully
         * (rseq_size > 0), the registration of other threads should
         * never fail.
         */
        if RSEQ_READ_ONCE_u32(&rseq_size as *const c_uint) > 0 {
            /* Incoherent success/failure within process. */
            abort();
        }
        return -1;
    }
    assert!(rseq_current_cpu_raw() >= 0);

    /*
     * The first thread to register sets the rseq_size to mimic the libc
     * behavior.
     */
    if RSEQ_READ_ONCE_u32(&rseq_size as *const c_uint) == 0 {
        RSEQ_WRITE_ONCE_u32(&mut rseq_size as *mut c_uint, size);
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn rseq_unregister_current_thread() -> c_int {
    let rc: c_int;

    if rseq_ownership == 0 {
        /* Treat libc's ownership as a successful unregistration. */
        return 0;
    }
    rc = __rseq.with(|r| {
        sys_rseq(
            (&(*r).abi) as *const rseq_abi as *mut rseq_abi,
            rseq_alloc_size,
            RSEQ_ABI_FLAG_UNREGISTER,
            RSEQ_SIG,
        )
    });
    if rc != 0 {
        return -1;
    }
    0
}

unsafe extern "C" fn rseq_init() {
    /*
     * If the libc's registered rseq size isn't already valid, it may be
     * because the binary is dynamically linked and not necessarily due to
     * libc not having registered a restartable sequence.  Try to find the
     * symbols if that's the case.
     */
    if libc_rseq_size_p.is_null() || *libc_rseq_size_p == 0 {
        libc_rseq_offset_p = dlsym(-1isize as *mut c_void, b"__rseq_offset\0".as_ptr() as *const c_char)
            as *const ptrdiff_t; /* RTLD_NEXT */
        libc_rseq_size_p = dlsym(-1isize as *mut c_void, b"__rseq_size\0".as_ptr() as *const c_char)
            as *const c_uint; /* RTLD_NEXT */
        libc_rseq_flags_p = dlsym(-1isize as *mut c_void, b"__rseq_flags\0".as_ptr() as *const c_char)
            as *const c_uint; /* RTLD_NEXT */
    }
    if !libc_rseq_size_p.is_null()
        && !libc_rseq_offset_p.is_null()
        && !libc_rseq_flags_p.is_null()
        && *libc_rseq_size_p != 0
    {
        let libc_rseq_size: c_uint;

        /* rseq registration owned by glibc */
        rseq_offset = *libc_rseq_offset_p;
        libc_rseq_size = *libc_rseq_size_p;
        rseq_flags = *libc_rseq_flags_p;

        /*
         * Previous versions of glibc expose the value
         * 32 even though the kernel only supported 20
         * bytes initially. Therefore treat 32 as a
         * special-case. glibc 2.40 exposes a 20 bytes
         * __rseq_size without using getauxval(3) to
         * query the supported size, while still allocating a 32
         * bytes area. Also treat 20 as a special-case.
         *
         * Special-cases are handled by using the following
         * value as active feature set size:
         *
         *   rseq_size = min(32, get_rseq_kernel_feature_size())
         */
        match libc_rseq_size {
            ORIG_RSEQ_FEATURE_SIZE | ORIG_RSEQ_ALLOC_SIZE => {
                let rseq_kernel_feature_size: c_uint = get_rseq_kernel_feature_size();

                if rseq_kernel_feature_size < ORIG_RSEQ_ALLOC_SIZE {
                    rseq_size = rseq_kernel_feature_size;
                } else {
                    rseq_size = ORIG_RSEQ_ALLOC_SIZE;
                }
            }
            _ => {
                /* Otherwise just use the __rseq_size from libc as rseq_size. */
                rseq_size = libc_rseq_size;
            }
        }
        return;
    }
    rseq_ownership = 1;

    /* Calculate the offset of the rseq area from the thread pointer. */
    __rseq.with(|r| {
        rseq_offset = (&(*r).abi as *const rseq_abi as *const c_void as isize)
            - (rseq_thread_pointer() as isize);
    });

    /* rseq flags are deprecated, always set to 0. */
    rseq_flags = 0;

    /*
     * Set the size to 0 until at least one thread registers to mimic the
     * libc behavior.
     */
    rseq_size = 0;
}

unsafe extern "C" fn rseq_exit() {
    if rseq_ownership == 0 {
        return;
    }
    rseq_offset = 0;
    rseq_size = !0u32;
    rseq_ownership = 0;
}

#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
static RSEQ_INIT_ARRAY: unsafe extern "C" fn() = rseq_init;

#[used]
#[cfg_attr(target_os = "linux", link_section = ".fini_array")]
static RSEQ_FINI_ARRAY: unsafe extern "C" fn() = rseq_exit;

#[no_mangle]
pub unsafe extern "C" fn rseq_fallback_current_cpu() -> i32 {
    let cpu: i32;

    cpu = sched_getcpu();
    if cpu < 0 {
        perror(b"sched_getcpu()\0".as_ptr() as *const c_char);
        abort();
    }
    cpu
}

#[no_mangle]
pub unsafe extern "C" fn rseq_fallback_current_node() -> i32 {
    let mut cpu_id: u32 = MaybeUninit::<u32>::uninit().assume_init();
    let mut node_id: u32 = MaybeUninit::<u32>::uninit().assume_init();
    let ret: c_int;

    ret = sys_getcpu(&mut cpu_id as *mut u32, &mut node_id as *mut u32);
    if ret != 0 {
        perror(b"sys_getcpu()\0".as_ptr() as *const c_char);
        return ret;
    }
    node_id as i32
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
