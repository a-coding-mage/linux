// SPDX-License-Identifier: GPL-2.0
//
// Rust translation of perf/util/libunwind-arch/libunwind-riscv.c.
// C includes removed; declarations below are supplied by the surrounding build.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_void};
use core::mem;
use core::ptr;

type size_t = usize;
type uint64_t = u64;
type bool_ = bool;
type unw_word_t = u64;
type unw_regnum_t = c_int;
type unw_fpreg_t = u64;
type unw_addr_space_t = *mut c_void;
type unw_cursor_t = c_void;

#[repr(C)]
pub struct maps {
    _private: [u8; 0],
}

#[repr(C)]
pub struct thread {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_sample {
    _private: [u8; 0],
}

#[repr(C)]
pub struct machine {
    _private: [u8; 0],
}

#[repr(C)]
pub struct unw_proc_info_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct unw_dyn_info_t {
    pub format: c_int,
    pub start_ip: unw_word_t,
    pub end_ip: unw_word_t,
    pub u: unw_dyn_info_u,
}

#[repr(C)]
pub union unw_dyn_info_u {
    pub rti: unw_dyn_remote_table_info_t,
    pub ti: unw_dyn_table_info_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct unw_dyn_remote_table_info_t {
    pub segbase: unw_word_t,
    pub table_data: unw_word_t,
    pub table_len: unw_word_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct unw_dyn_table_info_t {
    pub segbase: unw_word_t,
    pub table_data: unw_word_t,
    pub table_len: unw_word_t,
}

#[repr(C)]
pub struct libarch_unwind__dyn_info {
    pub start_ip: uint64_t,
    pub end_ip: uint64_t,
    pub segbase: uint64_t,
    pub table_data: uint64_t,
    pub table_len: uint64_t,
}

#[repr(C)]
pub struct unwind_info {
    pub machine: *mut machine,
    pub thread: *mut thread,
    pub sample: *mut perf_sample,
    pub cursor: *mut unw_cursor_t,
    pub ips: *mut uint64_t,
    pub cur_ip: c_int,
    pub max_ips: c_int,
    pub unw_word_t_size: size_t,
    pub e_machine: c_int,
    pub best_effort: bool_,
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
    pub access_mem:
        Option<unsafe extern "C" fn(unw_addr_space_t, unw_word_t, *mut unw_word_t, c_int, *mut c_void) -> c_int>,
    pub access_reg:
        Option<unsafe extern "C" fn(unw_addr_space_t, unw_regnum_t, *mut unw_word_t, c_int, *mut c_void) -> c_int>,
    pub access_fpreg:
        Option<unsafe extern "C" fn(unw_addr_space_t, unw_regnum_t, *mut unw_fpreg_t, c_int, *mut c_void) -> c_int>,
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

const EINVAL: c_int = 22;
const UNW_ENOINFO: c_int = 10;
const UNW_EINVAL: c_int = 8;
const UNW_CACHE_GLOBAL: c_int = 2;
const UNW_INFO_FORMAT_REMOTE_TABLE: c_int = 1;
const UNW_REG_IP: c_int = 16;
const EM_RISCV: c_int = 243;
const UNW_RISCV_X1: c_int = 1;
const UNW_RISCV_X31: c_int = 31;
const UNW_RISCV_PC: c_int = 32;
const PERF_REG_RISCV_RA: c_int = 1;
const PERF_REG_RISCV_PC: c_int = 0;

unsafe extern "C" {
    fn pr_err(fmt: *const c_char, ...);
    fn pr_debug(fmt: *const c_char, ...);
    fn free(ptr: *mut c_void);
    fn zalloc(size: size_t) -> *mut c_void;
    fn maps__addr_space(maps: *mut maps) -> *mut c_void;
    fn maps__machine(maps: *mut maps) -> *mut machine;
    fn thread__maps(thread: *mut thread) -> *mut maps;

    #[cfg(HAVE_LIBUNWIND_RISCV_SUPPORT)]
    fn __libunwind__find_proc_info(
        as_: unw_addr_space_t,
        ip: unw_word_t,
        pi: *mut unw_proc_info_t,
        need_unwind_info: c_int,
        arg: *mut c_void,
    ) -> c_int;
    #[cfg(HAVE_LIBUNWIND_RISCV_SUPPORT)]
    fn __libunwind__access_mem(
        as_: unw_addr_space_t,
        addr: unw_word_t,
        valp: *mut unw_word_t,
        __write: c_int,
        arg: *mut c_void,
    ) -> c_int;
    #[cfg(HAVE_LIBUNWIND_RISCV_SUPPORT)]
    fn __libunwind__access_reg(
        as_: unw_addr_space_t,
        regnum: unw_regnum_t,
        valp: *mut unw_word_t,
        __write: c_int,
        arg: *mut c_void,
    ) -> c_int;
    #[cfg(HAVE_LIBUNWIND_RISCV_SUPPORT)]
    fn unw_flush_cache(as_: unw_addr_space_t, start: unw_word_t, end: unw_word_t);
    #[cfg(HAVE_LIBUNWIND_RISCV_SUPPORT)]
    fn unw_destroy_addr_space(as_: unw_addr_space_t);
    #[cfg(HAVE_LIBUNWIND_RISCV_SUPPORT)]
    fn unw_create_addr_space(accessors: *mut unw_accessors_t, byte_order: c_int) -> unw_addr_space_t;
    #[cfg(HAVE_LIBUNWIND_RISCV_SUPPORT)]
    fn unw_set_caching_policy(as_: unw_addr_space_t, policy: c_int) -> c_int;
    #[cfg(HAVE_LIBUNWIND_RISCV_SUPPORT)]
    fn dwarf_search_unwind_table(
        as_: unw_addr_space_t,
        ip: unw_word_t,
        di: *mut unw_dyn_info_t,
        pi: *mut unw_proc_info_t,
        need_unwind_info: c_int,
        arg: *mut c_void,
    ) -> c_int;
    #[cfg(all(HAVE_LIBUNWIND_RISCV_SUPPORT, not(NO_LIBUNWIND_DEBUG_FRAME_RISCV)))]
    fn dwarf_find_debug_frame(
        found: c_int,
        di_debug: *mut unw_dyn_info_t,
        ip: unw_word_t,
        segbase: unw_word_t,
        obj_name: *const c_char,
        start: unw_word_t,
        end: unw_word_t,
    ) -> c_int;
    #[cfg(HAVE_LIBUNWIND_RISCV_SUPPORT)]
    fn unw_init_remote(cu: *mut unw_cursor_t, as_: unw_addr_space_t, arg: *mut c_void) -> c_int;
    #[cfg(HAVE_LIBUNWIND_RISCV_SUPPORT)]
    fn unw_strerror(err_code: c_int) -> *const c_char;
    #[cfg(HAVE_LIBUNWIND_RISCV_SUPPORT)]
    fn unw_step(cu: *mut unw_cursor_t) -> c_int;
    #[cfg(HAVE_LIBUNWIND_RISCV_SUPPORT)]
    fn unw_get_reg(cu: *mut unw_cursor_t, regnum: c_int, valp: *mut unw_word_t) -> c_int;
    #[cfg(HAVE_LIBUNWIND_RISCV_SUPPORT)]
    fn unw_is_signal_frame(cu: *mut unw_cursor_t) -> c_int;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __get_perf_regnum_for_unw_regnum_riscv(unw_regnum: c_int) -> c_int {
    #[cfg(not(HAVE_LIBUNWIND_RISCV_SUPPORT))]
    {
        return -EINVAL;
    }

    #[cfg(HAVE_LIBUNWIND_RISCV_SUPPORT)]
    {
        if unw_regnum >= UNW_RISCV_X1 && unw_regnum <= UNW_RISCV_X31 {
            return unw_regnum - UNW_RISCV_X1 + PERF_REG_RISCV_RA;
        }
        if unw_regnum == UNW_RISCV_PC {
            return PERF_REG_RISCV_PC;
        }
        pr_err(c"unwind: invalid reg id %d\n".as_ptr(), unw_regnum);
        -EINVAL
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __libunwind_arch__flush_access_riscv(maps: *mut maps) {
    #[cfg(HAVE_LIBUNWIND_RISCV_SUPPORT)]
    {
        unw_flush_cache(maps__addr_space(maps) as unw_addr_space_t, 0, 0);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __libunwind_arch__finish_access_riscv(maps: *mut maps) {
    #[cfg(HAVE_LIBUNWIND_RISCV_SUPPORT)]
    {
        unw_destroy_addr_space(maps__addr_space(maps) as unw_addr_space_t);
    }
}

#[cfg(HAVE_LIBUNWIND_RISCV_SUPPORT)]
unsafe extern "C" fn find_proc_info(
    as_: unw_addr_space_t,
    ip: unw_word_t,
    pi: *mut unw_proc_info_t,
    need_unwind_info: c_int,
    arg: *mut c_void,
) -> c_int {
    __libunwind__find_proc_info(as_, ip, pi, need_unwind_info, arg)
}

#[cfg(HAVE_LIBUNWIND_RISCV_SUPPORT)]
unsafe extern "C" fn put_unwind_info(
    _as: unw_addr_space_t,
    _pi: *mut unw_proc_info_t,
    _arg: *mut c_void,
) {
    pr_debug(c"unwind: put_unwind_info called\n".as_ptr());
}

#[cfg(HAVE_LIBUNWIND_RISCV_SUPPORT)]
unsafe extern "C" fn get_dyn_info_list_addr(
    _as: unw_addr_space_t,
    _dil_addr: *mut unw_word_t,
    _arg: *mut c_void,
) -> c_int {
    -UNW_ENOINFO
}

#[cfg(HAVE_LIBUNWIND_RISCV_SUPPORT)]
unsafe extern "C" fn access_mem(
    as_: unw_addr_space_t,
    addr: unw_word_t,
    valp: *mut unw_word_t,
    __write: c_int,
    arg: *mut c_void,
) -> c_int {
    __libunwind__access_mem(as_, addr, valp, __write, arg)
}

#[cfg(HAVE_LIBUNWIND_RISCV_SUPPORT)]
unsafe extern "C" fn access_reg(
    as_: unw_addr_space_t,
    regnum: unw_regnum_t,
    valp: *mut unw_word_t,
    __write: c_int,
    arg: *mut c_void,
) -> c_int {
    __libunwind__access_reg(as_, regnum, valp, __write, arg)
}

#[cfg(HAVE_LIBUNWIND_RISCV_SUPPORT)]
unsafe extern "C" fn access_fpreg(
    _as: unw_addr_space_t,
    _num: unw_regnum_t,
    _val: *mut unw_fpreg_t,
    _write: c_int,
    _arg: *mut c_void,
) -> c_int {
    pr_err(c"unwind: access_fpreg unsupported\n".as_ptr());
    -UNW_EINVAL
}

#[cfg(HAVE_LIBUNWIND_RISCV_SUPPORT)]
unsafe extern "C" fn resume(
    _as: unw_addr_space_t,
    _cu: *mut unw_cursor_t,
    _arg: *mut c_void,
) -> c_int {
    pr_err(c"unwind: resume unsupported\n".as_ptr());
    -UNW_EINVAL
}

#[cfg(HAVE_LIBUNWIND_RISCV_SUPPORT)]
unsafe extern "C" fn get_proc_name(
    _as: unw_addr_space_t,
    _addr: unw_word_t,
    _bufp: *mut c_char,
    _buf_len: size_t,
    _offp: *mut unw_word_t,
    _arg: *mut c_void,
) -> c_int {
    pr_err(c"unwind: get_proc_name unsupported\n".as_ptr());
    -UNW_EINVAL
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __libunwind_arch__create_addr_space_riscv() -> *mut c_void {
    #[cfg(not(HAVE_LIBUNWIND_RISCV_SUPPORT))]
    {
        return ptr::null_mut();
    }

    #[cfg(HAVE_LIBUNWIND_RISCV_SUPPORT)]
    {
        static mut accessors: unw_accessors_t = unw_accessors_t {
            find_proc_info: Some(find_proc_info),
            put_unwind_info: Some(put_unwind_info),
            get_dyn_info_list_addr: Some(get_dyn_info_list_addr),
            access_mem: Some(access_mem),
            access_reg: Some(access_reg),
            access_fpreg: Some(access_fpreg),
            resume: Some(resume),
            get_proc_name: Some(get_proc_name),
        };
        let addr_space = unw_create_addr_space(&mut accessors, 0);
        unw_set_caching_policy(addr_space, UNW_CACHE_GLOBAL);
        addr_space as *mut c_void
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __libunwind_arch__dwarf_search_unwind_table_riscv(
    as_: *mut c_void,
    ip: uint64_t,
    _di: *mut libarch_unwind__dyn_info,
    pi: *mut c_void,
    need_unwind_info: c_int,
    arg: *mut c_void,
) -> c_int {
    #[cfg(not(HAVE_LIBUNWIND_RISCV_SUPPORT))]
    {
        return -EINVAL;
    }

    #[cfg(HAVE_LIBUNWIND_RISCV_SUPPORT)]
    {
        let mut di = unw_dyn_info_t {
            format: UNW_INFO_FORMAT_REMOTE_TABLE,
            start_ip: (*_di).start_ip,
            end_ip: (*_di).end_ip,
            u: unw_dyn_info_u {
                rti: unw_dyn_remote_table_info_t {
                    segbase: (*_di).segbase,
                    table_data: (*_di).table_data,
                    table_len: (*_di).table_len,
                },
            },
        };
        let ret = dwarf_search_unwind_table(
            as_ as unw_addr_space_t,
            ip,
            &mut di,
            pi as *mut unw_proc_info_t,
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
pub unsafe extern "C" fn __libunwind_arch__dwarf_find_debug_frame_riscv(
    found: c_int,
    _di: *mut libarch_unwind__dyn_info,
    ip: uint64_t,
    segbase: uint64_t,
    obj_name: *const c_char,
    start: uint64_t,
    end: uint64_t,
) -> c_int {
    #[cfg(not(all(HAVE_LIBUNWIND_RISCV_SUPPORT, not(NO_LIBUNWIND_DEBUG_FRAME_RISCV))))]
    {
        return -EINVAL;
    }

    #[cfg(all(HAVE_LIBUNWIND_RISCV_SUPPORT, not(NO_LIBUNWIND_DEBUG_FRAME_RISCV)))]
    {
        let mut di = unw_dyn_info_t {
            format: UNW_INFO_FORMAT_REMOTE_TABLE,
            start_ip: (*_di).start_ip,
            end_ip: (*_di).end_ip,
            u: unw_dyn_info_u {
                rti: unw_dyn_remote_table_info_t {
                    segbase: (*_di).segbase,
                    table_data: (*_di).table_data,
                    table_len: (*_di).table_len,
                },
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

#[cfg(HAVE_LIBUNWIND_RISCV_SUPPORT)]
#[repr(C)]
struct arch_unwind_info {
    ui: unwind_info,
    _cursor: unw_cursor_t,
    _ips: [uint64_t; 0],
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __libunwind_arch_unwind_info__new_riscv(
    thread: *mut thread,
    sample: *mut perf_sample,
    max_stack: c_int,
    best_effort: bool_,
    first_ip: uint64_t,
) -> *mut unwind_info {
    #[cfg(not(HAVE_LIBUNWIND_RISCV_SUPPORT))]
    {
        return ptr::null_mut();
    }

    #[cfg(HAVE_LIBUNWIND_RISCV_SUPPORT)]
    {
        let maps = thread__maps(thread);
        let addr_space = maps__addr_space(maps);
        let mut ret: c_int;

        if addr_space.is_null() {
            return ptr::null_mut();
        }

        let ui = zalloc(
            mem::size_of::<arch_unwind_info>()
                + mem::size_of::<uint64_t>() * max_stack as usize,
        ) as *mut arch_unwind_info;
        if ui.is_null() {
            return ptr::null_mut();
        }

        (*ui).ui.machine = maps__machine(maps);
        (*ui).ui.thread = thread;
        (*ui).ui.sample = sample;
        (*ui).ui.cursor = &mut (*ui)._cursor;
        (*ui).ui.ips = (*ui)._ips.as_mut_ptr();
        *(*ui).ui.ips.add(0) = first_ip;
        (*ui).ui.cur_ip = 1;
        (*ui).ui.max_ips = max_stack;
        (*ui).ui.unw_word_t_size = mem::size_of::<unw_word_t>();
        (*ui).ui.e_machine = EM_RISCV;
        (*ui).ui.best_effort = best_effort;

        ret = unw_init_remote(&mut (*ui)._cursor, addr_space as unw_addr_space_t, &mut (*ui).ui as *mut _ as *mut c_void);
        if ret != 0 {
            if !best_effort {
                pr_err(c"libunwind: %s\n".as_ptr(), unw_strerror(ret));
            }
            free(ui as *mut c_void);
            return ptr::null_mut();
        }

        &mut (*ui).ui
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __libunwind_arch__unwind_step_riscv(ui: *mut unwind_info) -> c_int {
    #[cfg(not(HAVE_LIBUNWIND_RISCV_SUPPORT))]
    {
        return -EINVAL;
    }

    #[cfg(HAVE_LIBUNWIND_RISCV_SUPPORT)]
    {
        let ret: c_int;

        if (*ui).cur_ip >= (*ui).max_ips {
            return 0;
        }

        ret = unw_step((*ui).cursor);
        if ret > 0 {
            let mut ip: uint64_t = 0;

            unw_get_reg((*ui).cursor, UNW_REG_IP, &mut ip);

            if unw_is_signal_frame((*ui).cursor) <= 0 {
                /*
                 * Decrement the IP for any non-activation frames. This
                 * is required to properly find the srcline for caller
                 * frames.  See also the documentation for
                 * dwfl_frame_pc(), which this code tries to replicate.
                 */
                ip = ip.wrapping_sub(1);
            }
            *(*ui).ips.add((*ui).cur_ip as usize) = ip;
            (*ui).cur_ip += 1;
        }
        ret
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
