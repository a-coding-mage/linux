// SPDX-License-Identifier: GPL-2.0
//
// Rust translation of perf/util/libunwind-arch/libunwind-x86_64.c.
// C include dependencies intentionally remain external to this translated file:
// libunwind-arch.h, debug.h, maps.h, thread.h, asm/perf_regs.h, libunwind-x86_64.h,
// linux/compiler.h, linux/kernel.h, linux/zalloc.h, elf.h, errno.h.

use core::ffi::{c_char, c_int, c_void};

const EINVAL: c_int = 22;

// These constants, types, and functions are supplied by the same external
// dependencies as in the original C source.
type size_t = usize;
type uint64_t = u64;
type unw_word_t = u64;
type unw_fpreg_t = u64;
type unw_regnum_t = c_int;
type unw_addr_space_t = *mut c_void;
type unw_cursor_t = c_void;
type unw_proc_info_t = c_void;

#[repr(C)]
pub struct maps {
    _private: [u8; 0],
}

#[repr(C)]
pub struct libarch_unwind__dyn_info {
    pub start_ip: unw_word_t,
    pub end_ip: unw_word_t,
    pub segbase: unw_word_t,
    pub table_data: unw_word_t,
    pub table_len: unw_word_t,
}

#[repr(C)]
pub struct unw_table_info_t {
    pub name_ptr: unw_word_t,
    pub segbase: unw_word_t,
    pub table_data: unw_word_t,
    pub table_len: unw_word_t,
}

#[repr(C)]
pub struct unw_remote_table_info_t {
    pub name_ptr: unw_word_t,
    pub segbase: unw_word_t,
    pub table_data: unw_word_t,
    pub table_len: unw_word_t,
}

#[repr(C)]
pub union unw_dyn_info_u {
    pub ti: core::mem::ManuallyDrop<unw_table_info_t>,
    pub rti: core::mem::ManuallyDrop<unw_remote_table_info_t>,
}

#[repr(C)]
pub struct unw_dyn_info_t {
    pub next: *mut unw_dyn_info_t,
    pub prev: *mut unw_dyn_info_t,
    pub format: c_int,
    pub start_ip: unw_word_t,
    pub end_ip: unw_word_t,
    pub gp: unw_word_t,
    pub u: unw_dyn_info_u,
}

#[repr(C)]
pub struct unw_accessors_t {
    pub find_proc_info: Option<
        unsafe extern "C" fn(
            unw_addr_space_t,
            unw_word_t,
            *mut unw_proc_info_t,
            c_int,
            *mut c_void,
        ) -> c_int,
    >,
    pub put_unwind_info:
        Option<unsafe extern "C" fn(unw_addr_space_t, *mut unw_proc_info_t, *mut c_void)>,
    pub get_dyn_info_list_addr:
        Option<unsafe extern "C" fn(unw_addr_space_t, *mut unw_word_t, *mut c_void) -> c_int>,
    pub access_mem: Option<
        unsafe extern "C" fn(
            unw_addr_space_t,
            unw_word_t,
            *mut unw_word_t,
            c_int,
            *mut c_void,
        ) -> c_int,
    >,
    pub access_reg: Option<
        unsafe extern "C" fn(
            unw_addr_space_t,
            unw_regnum_t,
            *mut unw_word_t,
            c_int,
            *mut c_void,
        ) -> c_int,
    >,
    pub access_fpreg: Option<
        unsafe extern "C" fn(
            unw_addr_space_t,
            unw_regnum_t,
            *mut unw_fpreg_t,
            c_int,
            *mut c_void,
        ) -> c_int,
    >,
    pub resume: Option<unsafe extern "C" fn(unw_addr_space_t, *mut unw_cursor_t, *mut c_void) -> c_int>,
    pub get_proc_name: Option<
        unsafe extern "C" fn(
            unw_addr_space_t,
            unw_word_t,
            *mut c_char,
            size_t,
            *mut unw_word_t,
            *mut c_void,
        ) -> c_int,
    >,
}

unsafe extern "C" {
    fn pr_err(fmt: *const c_char, ...);
    fn pr_debug(fmt: *const c_char, ...);

    fn maps__addr_space(maps: *mut maps) -> unw_addr_space_t;

    fn __libunwind__find_proc_info(
        as_: unw_addr_space_t,
        ip: unw_word_t,
        pi: *mut unw_proc_info_t,
        need_unwind_info: c_int,
        arg: *mut c_void,
    ) -> c_int;
    fn __libunwind__access_mem(
        as_: unw_addr_space_t,
        addr: unw_word_t,
        valp: *mut unw_word_t,
        __write: c_int,
        arg: *mut c_void,
    ) -> c_int;
    fn __libunwind__access_reg(
        as_: unw_addr_space_t,
        regnum: unw_regnum_t,
        valp: *mut unw_word_t,
        __write: c_int,
        arg: *mut c_void,
    ) -> c_int;

    fn unw_flush_cache(as_: unw_addr_space_t, lo: unw_word_t, hi: unw_word_t);
    fn unw_destroy_addr_space(as_: unw_addr_space_t);
    fn unw_create_addr_space(accessors: *mut unw_accessors_t, byte_order: c_int) -> unw_addr_space_t;
    fn unw_set_caching_policy(as_: unw_addr_space_t, policy: c_int) -> c_int;

    fn dwarf_search_unwind_table(
        as_: unw_addr_space_t,
        ip: unw_word_t,
        di: *mut unw_dyn_info_t,
        pi: *mut c_void,
        need_unwind_info: c_int,
        arg: *mut c_void,
    ) -> c_int;

    fn dwarf_find_debug_frame(
        found: c_int,
        di_debug: *mut unw_dyn_info_t,
        ip: unw_word_t,
        segbase: unw_word_t,
        obj_name: *const c_char,
        start: unw_word_t,
        end: unw_word_t,
    ) -> c_int;
}

// Present only when HAVE_LIBUNWIND_X86_64_SUPPORT is defined in the C build.
const UNW_X86_64_RAX: c_int = 0;
const UNW_X86_64_RDX: c_int = 1;
const UNW_X86_64_RCX: c_int = 2;
const UNW_X86_64_RBX: c_int = 3;
const UNW_X86_64_RSI: c_int = 4;
const UNW_X86_64_RDI: c_int = 5;
const UNW_X86_64_RBP: c_int = 6;
const UNW_X86_64_RSP: c_int = 7;
const UNW_X86_64_RIP: c_int = 16;
const UNW_X86_64_R8: c_int = 8;
const UNW_X86_64_R9: c_int = 9;
const UNW_X86_64_R10: c_int = 10;
const UNW_X86_64_R11: c_int = 11;
const UNW_X86_64_R12: c_int = 12;
const UNW_X86_64_R13: c_int = 13;
const UNW_X86_64_R14: c_int = 14;
const UNW_X86_64_R15: c_int = 15;

const PERF_REG_X86_AX: c_int = 0;
const PERF_REG_X86_BX: c_int = 1;
const PERF_REG_X86_CX: c_int = 2;
const PERF_REG_X86_DX: c_int = 3;
const PERF_REG_X86_SI: c_int = 4;
const PERF_REG_X86_DI: c_int = 5;
const PERF_REG_X86_BP: c_int = 6;
const PERF_REG_X86_SP: c_int = 7;
const PERF_REG_X86_IP: c_int = 8;
const PERF_REG_X86_R8: c_int = 9;
const PERF_REG_X86_R9: c_int = 10;
const PERF_REG_X86_R10: c_int = 11;
const PERF_REG_X86_R11: c_int = 12;
const PERF_REG_X86_R12: c_int = 13;
const PERF_REG_X86_R13: c_int = 14;
const PERF_REG_X86_R14: c_int = 15;
const PERF_REG_X86_R15: c_int = 16;

const UNW_ENOINFO: c_int = 10;
const UNW_EINVAL: c_int = 8;
const UNW_CACHE_GLOBAL: c_int = 1;
const UNW_INFO_FORMAT_REMOTE_TABLE: c_int = 3;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __get_perf_regnum_for_unw_regnum_x86_64(unw_regnum: c_int) -> c_int {
    // Without HAVE_LIBUNWIND_X86_64_SUPPORT, the C implementation returns -EINVAL.
    const PERF_X86_64_REGNUMS: [c_int; 17] = [
        PERF_REG_X86_AX,
        PERF_REG_X86_DX,
        PERF_REG_X86_CX,
        PERF_REG_X86_BX,
        PERF_REG_X86_SI,
        PERF_REG_X86_DI,
        PERF_REG_X86_BP,
        PERF_REG_X86_SP,
        PERF_REG_X86_R8,
        PERF_REG_X86_R9,
        PERF_REG_X86_R10,
        PERF_REG_X86_R11,
        PERF_REG_X86_R12,
        PERF_REG_X86_R13,
        PERF_REG_X86_R14,
        PERF_REG_X86_R15,
        PERF_REG_X86_IP,
    ];

    if unw_regnum == UNW_X86_64_RAX {
        return PERF_REG_X86_AX;
    }

    if unw_regnum < 0
        || unw_regnum as usize >= PERF_X86_64_REGNUMS.len()
        || PERF_X86_64_REGNUMS[unw_regnum as usize] == 0
    {
        unsafe {
            pr_err(c"unwind: invalid reg id %d\n".as_ptr(), unw_regnum);
        }
        return -EINVAL;
    }

    PERF_X86_64_REGNUMS[unw_regnum as usize]
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __libunwind_arch__flush_access_x86_64(maps: *mut maps) {
    // Present only when HAVE_LIBUNWIND_X86_64_SUPPORT is defined.
    unsafe {
        unw_flush_cache(maps__addr_space(maps), 0, 0);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __libunwind_arch__finish_access_x86_64(maps: *mut maps) {
    // Present only when HAVE_LIBUNWIND_X86_64_SUPPORT is defined.
    unsafe {
        unw_destroy_addr_space(maps__addr_space(maps));
    }
}

unsafe extern "C" fn find_proc_info(
    as_: unw_addr_space_t,
    ip: unw_word_t,
    pi: *mut unw_proc_info_t,
    need_unwind_info: c_int,
    arg: *mut c_void,
) -> c_int {
    unsafe { __libunwind__find_proc_info(as_, ip, pi, need_unwind_info, arg) }
}

unsafe extern "C" fn put_unwind_info(
    _as: unw_addr_space_t,
    _pi: *mut unw_proc_info_t,
    _arg: *mut c_void,
) {
    unsafe {
        pr_debug(c"unwind: put_unwind_info called\n".as_ptr());
    }
}

unsafe extern "C" fn get_dyn_info_list_addr(
    _as: unw_addr_space_t,
    _dil_addr: *mut unw_word_t,
    _arg: *mut c_void,
) -> c_int {
    -UNW_ENOINFO
}

unsafe extern "C" fn access_mem(
    as_: unw_addr_space_t,
    addr: unw_word_t,
    valp: *mut unw_word_t,
    __write: c_int,
    arg: *mut c_void,
) -> c_int {
    unsafe { __libunwind__access_mem(as_, addr, valp, __write, arg) }
}

unsafe extern "C" fn access_reg(
    as_: unw_addr_space_t,
    regnum: unw_regnum_t,
    valp: *mut unw_word_t,
    __write: c_int,
    arg: *mut c_void,
) -> c_int {
    unsafe { __libunwind__access_reg(as_, regnum, valp, __write, arg) }
}

unsafe extern "C" fn access_fpreg(
    _as: unw_addr_space_t,
    _num: unw_regnum_t,
    _val: *mut unw_fpreg_t,
    ___write: c_int,
    _arg: *mut c_void,
) -> c_int {
    unsafe {
        pr_err(c"unwind: access_fpreg unsupported\n".as_ptr());
    }
    -UNW_EINVAL
}

unsafe extern "C" fn resume(
    _as: unw_addr_space_t,
    _cu: *mut unw_cursor_t,
    _arg: *mut c_void,
) -> c_int {
    unsafe {
        pr_err(c"unwind: resume unsupported\n".as_ptr());
    }
    -UNW_EINVAL
}

unsafe extern "C" fn get_proc_name(
    _as: unw_addr_space_t,
    _addr: unw_word_t,
    _bufp: *mut c_char,
    _buf_len: size_t,
    _offp: *mut unw_word_t,
    _arg: *mut c_void,
) -> c_int {
    unsafe {
        pr_err(c"unwind: get_proc_name unsupported\n".as_ptr());
    }
    -UNW_EINVAL
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __libunwind_arch__create_addr_space_x86_64() -> *mut c_void {
    // Without HAVE_LIBUNWIND_X86_64_SUPPORT, the C implementation returns NULL.
    let mut accessors = unw_accessors_t {
        find_proc_info: Some(find_proc_info),
        put_unwind_info: Some(put_unwind_info),
        get_dyn_info_list_addr: Some(get_dyn_info_list_addr),
        access_mem: Some(access_mem),
        access_reg: Some(access_reg),
        access_fpreg: Some(access_fpreg),
        resume: Some(resume),
        get_proc_name: Some(get_proc_name),
    };
    let addr_space = unsafe { unw_create_addr_space(&mut accessors, 0) };

    unsafe {
        unw_set_caching_policy(addr_space, UNW_CACHE_GLOBAL);
    }
    addr_space
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __libunwind_arch__dwarf_search_unwind_table_x86_64(
    as_: *mut c_void,
    ip: uint64_t,
    _di: *mut libarch_unwind__dyn_info,
    pi: *mut c_void,
    need_unwind_info: c_int,
    arg: *mut c_void,
) -> c_int {
    // Without HAVE_LIBUNWIND_X86_64_SUPPORT, the C implementation returns -EINVAL.
    unsafe {
        let mut di = unw_dyn_info_t {
            next: core::ptr::null_mut(),
            prev: core::ptr::null_mut(),
            format: UNW_INFO_FORMAT_REMOTE_TABLE,
            start_ip: (*_di).start_ip,
            end_ip: (*_di).end_ip,
            gp: 0,
            u: unw_dyn_info_u {
                rti: core::mem::ManuallyDrop::new(unw_remote_table_info_t {
                    name_ptr: 0,
                    segbase: (*_di).segbase,
                    table_data: (*_di).table_data,
                    table_len: (*_di).table_len,
                }),
            },
        };
        let ret = dwarf_search_unwind_table(
            as_ as unw_addr_space_t,
            ip,
            &mut di,
            pi,
            need_unwind_info,
            arg,
        );

        (*_di).start_ip = di.start_ip;
        (*_di).end_ip = di.end_ip;
        (*_di).segbase = di.u.rti.segbase;
        (*_di).table_data = di.u.rti.table_data;
        (*_di).table_len = di.u.rti.table_len;
        ret
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __libunwind_arch__dwarf_find_debug_frame_x86_64(
    found: c_int,
    _di: *mut libarch_unwind__dyn_info,
    ip: uint64_t,
    segbase: uint64_t,
    obj_name: *const c_char,
    start: uint64_t,
    end: uint64_t,
) -> c_int {
    // Active only when HAVE_LIBUNWIND_X86_64_SUPPORT is defined and
    // NO_LIBUNWIND_DEBUG_FRAME_X86_64 is not defined. Otherwise C returns -EINVAL.
    unsafe {
        let mut di = unw_dyn_info_t {
            next: core::ptr::null_mut(),
            prev: core::ptr::null_mut(),
            format: UNW_INFO_FORMAT_REMOTE_TABLE,
            start_ip: (*_di).start_ip,
            end_ip: (*_di).end_ip,
            gp: 0,
            u: unw_dyn_info_u {
                rti: core::mem::ManuallyDrop::new(unw_remote_table_info_t {
                    name_ptr: 0,
                    segbase: (*_di).segbase,
                    table_data: (*_di).table_data,
                    table_len: (*_di).table_len,
                }),
            },
        };
        let ret = dwarf_find_debug_frame(found, &mut di, ip, segbase, obj_name, start, end);

        (*_di).start_ip = di.start_ip;
        (*_di).end_ip = di.end_ip;
        (*_di).segbase = di.u.ti.segbase;
        (*_di).table_data = di.u.ti.table_data;
        (*_di).table_len = di.u.ti.table_len;
        ret
    }
}
