// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2020 ARM Limited

// C dependencies: fcntl.h, sched.h, signal.h, stdio.h, stdlib.h, time.h,
// unistd.h, linux/auxvec.h, sys/auxv.h, sys/mman.h, sys/prctl.h,
// asm/hwcap.h, kselftest.h, mte_common_util.h, mte_def.h.

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};
use core::ptr;

pub type size_t = usize;
pub type ssize_t = isize;
pub type uintptr_t = usize;

pub const SA_EXPOSE_TAGBITS: c_int = 0x00000800;
pub const INIT_BUFFER_SIZE: usize = 256;

// Constants supplied by Linux/selftest headers in the original C file.
pub const SIGSEGV: c_int = 11;
pub const SIGBUS: c_int = 7;
pub const SA_SIGINFO: c_int = 4;
pub const SEGV_MTEAERR: c_int = 8;
pub const SEGV_MTESERR: c_int = 9;
pub const KSFT_FAIL: c_int = 1;
pub const EINVAL: c_int = 22;
pub const PROT_READ: c_int = 0x1;
pub const PROT_WRITE: c_int = 0x2;
pub const PROT_MTE: c_int = 0x20;
pub const MAP_SHARED: c_int = 0x01;
pub const MAP_PRIVATE: c_int = 0x02;
pub const MAP_ANONYMOUS: c_int = 0x20;
pub const SEEK_SET: c_int = 0;
pub const AT_HWCAP2: c_ulong = 26;
pub const AT_HWCAP3: c_ulong = 29;
pub const HWCAP2_MTE: c_ulong = 1 << 18;
pub const HWCAP3_MTE_FAR: c_ulong = 1 << 0;
pub const HWCAP3_MTE_STORE_ONLY: c_ulong = 1 << 1;
pub const PR_SET_TAGGED_ADDR_CTRL: c_int = 55;
pub const PR_GET_TAGGED_ADDR_CTRL: c_int = 56;
pub const PR_TAGGED_ADDR_ENABLE: c_ulong = 1 << 0;
pub const PR_MTE_TCF_SHIFT: c_ulong = 1;
pub const PR_MTE_TCF_NONE: c_ulong = 0 << PR_MTE_TCF_SHIFT;
pub const PR_MTE_TCF_SYNC: c_ulong = 1 << PR_MTE_TCF_SHIFT;
pub const PR_MTE_TCF_ASYNC: c_ulong = 2 << PR_MTE_TCF_SHIFT;
pub const PR_MTE_TAG_SHIFT: c_ulong = 3;
pub const PR_MTE_STORE_ONLY: c_ulong = 1 << 19;

// Constants and helpers supplied by mte_common_util.h/mte_def.h in the
// original source. Numeric values are header-owned in the C repository.
unsafe extern "C" {
    static USE_MALLOC: c_int;
    static USE_MMAP: c_int;
    static USE_MPROTECT: c_int;
    static MTE_NONE_ERR: c_int;
    static MTE_SYNC_ERR: c_int;
    static MTE_ASYNC_ERR: c_int;
    static MT_ALIGN_GRANULE: c_ulong;
    static MT_ATAG_MASK: c_ulong;
    static MT_INCLUDE_TAG_MASK: c_ulong;
    static MTE_ALLOW_NON_ZERO_TAG: c_ulong;
    static MT_PSTATE_TCO_EN: c_uint;
    static MT_PSTATE_TCO_DIS: c_uint;
}

pub type c_uint = u32;

#[repr(C)]
pub struct sigset_t {
    __val: [c_ulong; 16],
}

#[repr(C)]
pub struct siginfo_t {
    _prefix: [u8; 16],
    pub si_code: c_int,
    _pad: [u8; 12],
    pub si_addr: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sigaction {
    pub sa_sigaction: Option<unsafe extern "C" fn(c_int, *mut siginfo_t, *mut c_void)>,
    pub sa_mask: sigset_t,
    pub sa_flags: c_int,
    pub sa_restorer: Option<unsafe extern "C" fn()>,
}

#[repr(C)]
pub struct mcontext_t {
    _pad0: [u8; 184],
    pub pc: c_ulong,
}

#[repr(C)]
pub struct ucontext_t {
    _pad0: [u8; 176],
    pub uc_mcontext: mcontext_t,
}

#[repr(C)]
pub struct mte_fault_cxt {
    pub fault_valid: bool,
    pub trig_addr: uintptr_t,
    pub trig_range: ssize_t,
    pub trig_si_code: c_int,
}

unsafe extern "C" {
    fn sigaction(signum: c_int, act: *const sigaction, oldact: *mut sigaction) -> c_int;
    fn sigemptyset(set: *mut sigset_t) -> c_int;
    fn sched_yield() -> c_int;
    fn malloc(size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn mmap(
        addr: *mut c_void,
        length: size_t,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: c_long,
    ) -> *mut c_void;
    fn munmap(addr: *mut c_void, length: size_t) -> c_int;
    fn mprotect(addr: *mut c_void, len: size_t, prot: c_int) -> c_int;
    fn lseek(fd: c_int, offset: c_long, whence: c_int) -> c_long;
    fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    fn random() -> c_long;
    fn srandom(seed: c_uint);
    fn time(tloc: *mut c_long) -> c_long;
    fn exit(status: c_int) -> !;
    fn getauxval(type_: c_ulong) -> c_ulong;
    fn prctl(option: c_int, arg2: c_ulong, arg3: c_ulong, arg4: c_ulong, arg5: c_ulong) -> c_int;
    fn mkstemp(template: *mut c_char) -> c_int;
    fn unlink(pathname: *const c_char) -> c_int;

    fn ksft_print_msg(fmt: *const c_char, ...);
    fn ksft_perror(msg: *const c_char);
    fn ksft_exit_skip(fmt: *const c_char, ...) -> !;

    fn MT_FETCH_TAG(addr: c_ulong) -> u8;
    fn MT_FETCH_ATAG(addr: c_ulong) -> u8;
    fn MT_CLEAR_TAGS(addr: c_ulong) -> c_ulong;
    fn MT_ALIGN_UP(size: size_t) -> c_int;
    fn MT_CLEAR_TAG(addr: c_ulong) -> c_ulong;
    fn MT_SET_ATAG(addr: c_ulong, atag: u8) -> c_ulong;
    fn MT_CLEAR_ATAG(addr: c_ulong) -> c_ulong;

    fn mte_insert_random_tag(ptr: *mut c_void) -> *mut c_void;
    fn mte_set_tag_address_range(ptr: *mut c_void, size: c_int);
    fn mte_clear_tag_address_range(ptr: *mut c_void, size: size_t);
    fn mte_get_pstate_tco() -> c_uint;
    fn mte_disable_pstate_tco();
    fn mte_enable_pstate_tco();
}

#[unsafe(no_mangle)]
pub static mut cur_mte_cxt: mte_fault_cxt = mte_fault_cxt {
    fault_valid: false,
    trig_addr: 0,
    trig_range: 0,
    trig_si_code: 0,
};
#[unsafe(no_mangle)]
pub static mut mtefar_support: bool = false;
#[unsafe(no_mangle)]
pub static mut mtestonly_support: bool = false;
static mut mte_cur_mode: c_uint = 0;
static mut mte_cur_pstate_tco: c_uint = 0;
static mut mte_cur_stonly: bool = false;

const MAP_FAILED: *mut c_void = !0usize as *mut c_void;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mte_default_handler(signum: c_int, si: *mut siginfo_t, uc: *mut c_void) {
    let mut sa: sigaction = core::mem::zeroed();
    let mut addr = (*si).si_addr as c_ulong;
    let si_tag: u8;
    let si_atag: u8;

    sigaction(signum, ptr::null(), &mut sa);

    if (sa.sa_flags & SA_EXPOSE_TAGBITS) != 0 {
        si_tag = MT_FETCH_TAG(addr);
        si_atag = MT_FETCH_ATAG(addr);
        addr = MT_CLEAR_TAGS(addr);
    } else {
        si_tag = 0;
        si_atag = 0;
    }

    if signum == SIGSEGV {
        // DEBUG-only ksft_print_msg from the C source is intentionally left conditional
        // to the original build configuration.
        if (*si).si_code == SEGV_MTEAERR {
            if cur_mte_cxt.trig_si_code == (*si).si_code {
                cur_mte_cxt.fault_valid = true;
            } else {
                ksft_print_msg(
                    c"Got unexpected SEGV_MTEAERR at pc=%llx, fault addr=%lx\n".as_ptr(),
                    (*(uc as *mut ucontext_t)).uc_mcontext.pc,
                    addr,
                );
            }
            return;
        } else if (*si).si_code == SEGV_MTESERR {
            /* Compare the context for precise error */
            if (!mtefar_support && si_atag != 0)
                || (si_atag != MT_FETCH_ATAG(cur_mte_cxt.trig_addr as c_ulong))
            {
                ksft_print_msg(
                    c"Invalid MTE synchronous exception caught for address tag! si_tag=%x, si_atag: %x\n"
                        .as_ptr(),
                    si_tag as c_int,
                    si_atag as c_int,
                );
                exit(KSFT_FAIL);
            }

            if cur_mte_cxt.trig_si_code == (*si).si_code
                && ((cur_mte_cxt.trig_range >= 0
                    && addr >= MT_CLEAR_TAGS(cur_mte_cxt.trig_addr as c_ulong)
                    && addr
                        <= MT_CLEAR_TAGS(cur_mte_cxt.trig_addr as c_ulong)
                            .wrapping_add(cur_mte_cxt.trig_range as c_ulong))
                    || (cur_mte_cxt.trig_range < 0
                        && addr <= MT_CLEAR_TAGS(cur_mte_cxt.trig_addr as c_ulong)
                        && addr
                            >= MT_CLEAR_TAGS(cur_mte_cxt.trig_addr as c_ulong)
                                .wrapping_add(cur_mte_cxt.trig_range as c_ulong)))
            {
                cur_mte_cxt.fault_valid = true;
                /* Adjust the pc by 4 */
                (*(uc as *mut ucontext_t)).uc_mcontext.pc =
                    (*(uc as *mut ucontext_t)).uc_mcontext.pc.wrapping_add(4);
            } else {
                ksft_print_msg(c"Invalid MTE synchronous exception caught!\n".as_ptr());
                exit(1);
            }
        } else {
            ksft_print_msg(c"Unknown SIGSEGV exception caught!\n".as_ptr());
            exit(1);
        }
    } else if signum == SIGBUS {
        ksft_print_msg(
            c"INFO: SIGBUS signal at pc=%llx, fault addr=%lx, si_code=%x\n".as_ptr(),
            (*(uc as *mut ucontext_t)).uc_mcontext.pc,
            addr,
            (*si).si_code,
        );
        if (cur_mte_cxt.trig_range >= 0
            && addr >= MT_CLEAR_TAGS(cur_mte_cxt.trig_addr as c_ulong)
            && addr
                <= MT_CLEAR_TAGS(cur_mte_cxt.trig_addr as c_ulong)
                    .wrapping_add(cur_mte_cxt.trig_range as c_ulong))
            || (cur_mte_cxt.trig_range < 0
                && addr <= MT_CLEAR_TAGS(cur_mte_cxt.trig_addr as c_ulong)
                && addr
                    >= MT_CLEAR_TAGS(cur_mte_cxt.trig_addr as c_ulong)
                        .wrapping_add(cur_mte_cxt.trig_range as c_ulong))
        {
            cur_mte_cxt.fault_valid = true;
            /* Adjust the pc by 4 */
            (*(uc as *mut ucontext_t)).uc_mcontext.pc =
                (*(uc as *mut ucontext_t)).uc_mcontext.pc.wrapping_add(4);
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mte_register_signal(
    signal: c_int,
    handler: Option<unsafe extern "C" fn(c_int, *mut siginfo_t, *mut c_void)>,
    export_tags: bool,
) {
    let mut sa: sigaction = core::mem::zeroed();

    sa.sa_sigaction = handler;
    sa.sa_flags = SA_SIGINFO;

    if export_tags && signal == SIGSEGV {
        sa.sa_flags |= SA_EXPOSE_TAGBITS;
    }

    sigemptyset(&mut sa.sa_mask);
    sigaction(signal, &sa, ptr::null_mut());
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mte_wait_after_trig() {
    sched_yield();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mte_insert_tags(ptr: *mut c_void, size: size_t) -> *mut c_void {
    let tag_ptr: *mut c_void;
    let align_size: c_int;

    if ptr.is_null() || ((ptr as c_ulong) & MT_ALIGN_GRANULE) != 0 {
        ksft_print_msg(c"FAIL: Addr=%p: invalid\n".as_ptr(), ptr);
        return ptr::null_mut();
    }
    align_size = MT_ALIGN_UP(size);
    tag_ptr = mte_insert_random_tag(ptr);
    mte_set_tag_address_range(tag_ptr, align_size);
    tag_ptr
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mte_clear_tags(mut ptr: *mut c_void, mut size: size_t) {
    if ptr.is_null() || ((ptr as c_ulong) & MT_ALIGN_GRANULE) != 0 {
        ksft_print_msg(c"FAIL: Addr=%p: invalid\n".as_ptr(), ptr);
        return;
    }
    size = MT_ALIGN_UP(size) as size_t;
    ptr = MT_CLEAR_TAG(ptr as c_ulong) as *mut c_void;
    mte_clear_tag_address_range(ptr, size);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mte_insert_atag(ptr: *mut c_void) -> *mut c_void {
    let atag: u8;

    atag = if mtefar_support {
        ((random() as c_ulong % MT_ATAG_MASK) + 1) as u8
    } else {
        0
    };
    MT_SET_ATAG(ptr as c_ulong, atag) as *mut c_void
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mte_clear_atag(ptr: *mut c_void) -> *mut c_void {
    MT_CLEAR_ATAG(ptr as c_ulong) as *mut c_void
}

unsafe fn __mte_allocate_memory_range(
    size: size_t,
    mem_type: c_int,
    mapping: c_int,
    range_before: size_t,
    range_after: size_t,
    tags: bool,
    fd: c_int,
) -> *mut c_void {
    let mut ptr: *mut c_void;
    let mut prot_flag: c_int;
    let mut map_flag: c_int;
    let entire_size = size.wrapping_add(range_before).wrapping_add(range_after);

    if mem_type == USE_MALLOC {
        return (malloc(entire_size) as *mut u8).add(range_before) as *mut c_void;
    } else if mem_type == USE_MMAP || mem_type == USE_MPROTECT {
    } else {
        ksft_print_msg(c"FAIL: Invalid allocate request\n".as_ptr());
        return ptr::null_mut();
    }

    prot_flag = PROT_READ | PROT_WRITE;
    if mem_type == USE_MMAP {
        prot_flag |= PROT_MTE;
    }

    map_flag = mapping;
    if fd == -1 {
        map_flag = MAP_ANONYMOUS | map_flag;
    }
    if (mapping & MAP_SHARED) == 0 {
        map_flag |= MAP_PRIVATE;
    }
    ptr = mmap(ptr::null_mut(), entire_size, prot_flag, map_flag, fd, 0);
    if ptr == MAP_FAILED {
        ksft_perror(c"mmap()".as_ptr());
        return ptr::null_mut();
    }
    if mem_type == USE_MPROTECT {
        if mprotect(ptr, entire_size, prot_flag | PROT_MTE) != 0 {
            ksft_perror(c"mprotect(PROT_MTE)".as_ptr());
            munmap(ptr, size);
            return ptr::null_mut();
        }
    }
    if tags {
        ptr = mte_insert_tags((ptr as *mut u8).add(range_before) as *mut c_void, size);
    }
    ptr
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mte_allocate_memory_tag_range(
    size: size_t,
    mem_type: c_int,
    mapping: c_int,
    range_before: size_t,
    range_after: size_t,
) -> *mut c_void {
    __mte_allocate_memory_range(size, mem_type, mapping, range_before, range_after, true, -1)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mte_allocate_memory(
    size: size_t,
    mem_type: c_int,
    mapping: c_int,
    tags: bool,
) -> *mut c_void {
    __mte_allocate_memory_range(size, mem_type, mapping, 0, 0, tags, -1)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mte_allocate_file_memory(
    size: size_t,
    mem_type: c_int,
    mapping: c_int,
    tags: bool,
    fd: c_int,
) -> *mut c_void {
    let mut index: c_int;
    let buffer: [c_char; INIT_BUFFER_SIZE] = [0; INIT_BUFFER_SIZE];

    if mem_type != USE_MPROTECT && mem_type != USE_MMAP {
        ksft_print_msg(c"FAIL: Invalid mmap file request\n".as_ptr());
        return ptr::null_mut();
    }
    /* Initialize the file for mappable size */
    lseek(fd, 0, SEEK_SET);
    index = INIT_BUFFER_SIZE as c_int;
    while (index as size_t) < size {
        if write(fd, buffer.as_ptr() as *const c_void, INIT_BUFFER_SIZE) != INIT_BUFFER_SIZE as ssize_t
        {
            ksft_perror(c"initialising buffer".as_ptr());
            return ptr::null_mut();
        }
        index += INIT_BUFFER_SIZE as c_int;
    }
    index -= INIT_BUFFER_SIZE as c_int;
    if write(
        fd,
        buffer.as_ptr() as *const c_void,
        size.wrapping_sub(index as size_t),
    ) != size.wrapping_sub(index as size_t) as ssize_t
    {
        ksft_perror(c"initialising buffer".as_ptr());
        return ptr::null_mut();
    }
    __mte_allocate_memory_range(size, mem_type, mapping, 0, 0, tags, fd)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mte_allocate_file_memory_tag_range(
    size: size_t,
    mem_type: c_int,
    mapping: c_int,
    range_before: size_t,
    range_after: size_t,
    fd: c_int,
) -> *mut c_void {
    let mut index: c_int;
    let buffer: [c_char; INIT_BUFFER_SIZE] = [0; INIT_BUFFER_SIZE];
    let map_size: c_int = size.wrapping_add(range_before).wrapping_add(range_after) as c_int;

    if mem_type != USE_MPROTECT && mem_type != USE_MMAP {
        ksft_print_msg(c"FAIL: Invalid mmap file request\n".as_ptr());
        return ptr::null_mut();
    }
    /* Initialize the file for mappable size */
    lseek(fd, 0, SEEK_SET);
    index = INIT_BUFFER_SIZE as c_int;
    while index < map_size {
        if write(fd, buffer.as_ptr() as *const c_void, INIT_BUFFER_SIZE) != INIT_BUFFER_SIZE as ssize_t
        {
            ksft_perror(c"initialising buffer".as_ptr());
            return ptr::null_mut();
        }
        index += INIT_BUFFER_SIZE as c_int;
    }
    index -= INIT_BUFFER_SIZE as c_int;
    if write(
        fd,
        buffer.as_ptr() as *const c_void,
        (map_size - index) as size_t,
    ) != (map_size - index) as ssize_t
    {
        ksft_perror(c"initialising buffer".as_ptr());
        return ptr::null_mut();
    }
    __mte_allocate_memory_range(size, mem_type, mapping, range_before, range_after, true, fd)
}

unsafe fn __mte_free_memory_range(
    ptr: *mut c_void,
    size: size_t,
    mem_type: c_int,
    range_before: size_t,
    range_after: size_t,
    tags: bool,
) {
    if mem_type == USE_MALLOC {
        free((ptr as *mut u8).sub(range_before) as *mut c_void);
    } else if mem_type == USE_MMAP || mem_type == USE_MPROTECT {
        if tags {
            mte_clear_tags(ptr, size);
        }
        munmap(
            (ptr as *mut u8).sub(range_before) as *mut c_void,
            size.wrapping_add(range_before).wrapping_add(range_after),
        );
    } else {
        ksft_print_msg(c"FAIL: Invalid free request\n".as_ptr());
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mte_free_memory_tag_range(
    ptr: *mut c_void,
    size: size_t,
    mem_type: c_int,
    range_before: size_t,
    range_after: size_t,
) {
    __mte_free_memory_range(ptr, size, mem_type, range_before, range_after, true);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mte_free_memory(ptr: *mut c_void, size: size_t, mem_type: c_int, tags: bool) {
    __mte_free_memory_range(ptr, size, mem_type, 0, 0, tags);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mte_initialize_current_context(
    mode: c_int,
    ptr: uintptr_t,
    range: ssize_t,
) {
    cur_mte_cxt.fault_valid = false;
    cur_mte_cxt.trig_addr = ptr;
    cur_mte_cxt.trig_range = range;
    if mode == MTE_SYNC_ERR {
        cur_mte_cxt.trig_si_code = SEGV_MTESERR;
    } else if mode == MTE_ASYNC_ERR {
        cur_mte_cxt.trig_si_code = SEGV_MTEAERR;
    } else {
        cur_mte_cxt.trig_si_code = 0;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mte_switch_mode(
    mte_option: c_int,
    incl_mask: c_ulong,
    stonly: bool,
) -> c_int {
    let mut en: c_ulong;

    if mte_option == MTE_NONE_ERR || mte_option == MTE_SYNC_ERR || mte_option == MTE_ASYNC_ERR {
    } else {
        ksft_print_msg(c"FAIL: Invalid MTE option %x\n".as_ptr(), mte_option);
        return -EINVAL;
    }

    if (incl_mask & !MT_INCLUDE_TAG_MASK) != 0 {
        ksft_print_msg(c"FAIL: Invalid incl_mask %lx\n".as_ptr(), incl_mask);
        return -EINVAL;
    }

    en = PR_TAGGED_ADDR_ENABLE;
    if mte_option == MTE_SYNC_ERR {
        en |= PR_MTE_TCF_SYNC;
    } else if mte_option == MTE_ASYNC_ERR {
        en |= PR_MTE_TCF_ASYNC;
    } else if mte_option == MTE_NONE_ERR {
        en |= PR_MTE_TCF_NONE;
    }

    if mtestonly_support && stonly {
        en |= PR_MTE_STORE_ONLY;
    }

    en |= incl_mask << PR_MTE_TAG_SHIFT;
    /* Enable address tagging ABI, mte error reporting mode and tag inclusion mask. */
    if prctl(PR_SET_TAGGED_ADDR_CTRL, en, 0, 0, 0) != 0 {
        ksft_print_msg(c"FAIL:prctl PR_SET_TAGGED_ADDR_CTRL for mte mode\n".as_ptr());
        return -EINVAL;
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mte_default_setup() -> c_int {
    let hwcaps2: c_ulong = getauxval(AT_HWCAP2);
    let hwcaps3: c_ulong = getauxval(AT_HWCAP3);
    let en: c_ulong = 0;
    let ret: c_int;

    /* To generate random address tag */
    srandom(time(ptr::null_mut()) as c_uint);

    if (hwcaps2 & HWCAP2_MTE) == 0 {
        ksft_exit_skip(c"MTE features unavailable\n".as_ptr());
    }

    mtefar_support = (hwcaps3 & HWCAP3_MTE_FAR) != 0;

    if (hwcaps3 & HWCAP3_MTE_STORE_ONLY) != 0 {
        mtestonly_support = true;
    }

    /* Get current mte mode */
    ret = prctl(PR_GET_TAGGED_ADDR_CTRL, en, 0, 0, 0);
    if ret < 0 {
        ksft_print_msg(
            c"FAIL:prctl PR_GET_TAGGED_ADDR_CTRL with error =%d\n".as_ptr(),
            ret,
        );
        return KSFT_FAIL;
    }
    if (ret as c_ulong & PR_MTE_TCF_SYNC) != 0 {
        mte_cur_mode = MTE_SYNC_ERR as c_uint;
    } else if (ret as c_ulong & PR_MTE_TCF_ASYNC) != 0 {
        mte_cur_mode = MTE_ASYNC_ERR as c_uint;
    } else if (ret as c_ulong & PR_MTE_TCF_NONE) != 0 {
        mte_cur_mode = MTE_NONE_ERR as c_uint;
    }

    mte_cur_stonly = (ret as c_ulong & PR_MTE_STORE_ONLY) != 0;

    mte_cur_pstate_tco = mte_get_pstate_tco();
    /* Disable PSTATE.TCO */
    mte_disable_pstate_tco();
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mte_restore_setup() {
    mte_switch_mode(mte_cur_mode as c_int, MTE_ALLOW_NON_ZERO_TAG, mte_cur_stonly);
    if mte_cur_pstate_tco == MT_PSTATE_TCO_EN {
        mte_enable_pstate_tco();
    } else if mte_cur_pstate_tco == MT_PSTATE_TCO_DIS {
        mte_disable_pstate_tco();
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn create_temp_file() -> c_int {
    let fd: c_int;
    let mut filename = *b"/dev/shm/tmp_XXXXXX\0";

    /* Create a file in the tmpfs filesystem */
    fd = mkstemp(filename.as_mut_ptr() as *mut c_char);
    if fd == -1 {
        ksft_perror(filename.as_ptr() as *const c_char);
        ksft_print_msg(c"FAIL: Unable to open temporary file\n".as_ptr());
        return 0;
    }
    unlink(filename.as_ptr() as *const c_char);
    fd
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
