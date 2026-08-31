// SPDX-License-Identifier: GPL-2.0
// Translated from perf/util/libunwind-arch/libunwind-i386.c.
// C includes intentionally omitted; referenced symbols are supplied by external
// translation units/headers in the original repository.

use core::ffi::{c_char, c_int, c_void};

extern "C" {
    fn pr_err(fmt: *const c_char, ...);
    fn pr_debug(fmt: *const c_char, ...);

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

    fn maps__addr_space(maps: *mut maps) -> *mut c_void;
    fn maps__machine(maps: *mut maps) -> *mut machine;
    fn thread__maps(thread: *mut thread) -> *mut maps;
    fn zalloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);

    fn unw_flush_cache(as_: unw_addr_space_t, lo: unw_word_t, hi: unw_word_t);
    fn unw_destroy_addr_space(as_: unw_addr_space_t);
    fn unw_create_addr_space(accessors: *mut unw_accessors_t, byte_order: c_int) -> unw_addr_space_t;
    fn unw_set_caching_policy(as_: unw_addr_space_t, policy: c_int) -> c_int;
    fn unw_init_remote(cursor: *mut unw_cursor_t, as_: unw_addr_space_t, arg: *mut c_void) -> c_int;
    fn unw_strerror(err_code: c_int) -> *const c_char;
    fn unw_step(cursor: *mut unw_cursor_t) -> c_int;
    fn unw_get_reg(cursor: *mut unw_cursor_t, regnum: c_int, valp: *mut u64) -> c_int;
    fn unw_is_signal_frame(cursor: *mut unw_cursor_t) -> c_int;

    fn dwarf_search_unwind_table(
        as_: unw_addr_space_t,
        ip: unw_word_t,
        di: *mut unw_dyn_info_t,
        pi: *mut unw_proc_info_t,
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

#[repr(C)]
pub struct maps {
    _private: [u8; 0],
}

#[repr(C)]
pub struct machine {
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
pub struct unw_proc_info_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct unw_cursor_t {
    _private: [u8; 0],
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
            usize,
            *mut unw_word_t,
            *mut c_void,
        ) -> c_int,
    >,
}

pub type unw_addr_space_t = *mut c_void;
pub type unw_word_t = u64;
pub type unw_regnum_t = c_int;
pub type unw_fpreg_t = u64;

const EINVAL: c_int = 22;
const UNW_ENOINFO: c_int = 10;
const UNW_EINVAL: c_int = 22;
const UNW_CACHE_GLOBAL: c_int = 1;
const UNW_INFO_FORMAT_REMOTE_TABLE: c_int = 1;
const UNW_REG_IP: c_int = 16;
const EM_I386: u16 = 3;

const PERF_REG_X86_AX: c_int = 0;
const PERF_REG_X86_DX: c_int = 1;
const PERF_REG_X86_CX: c_int = 2;
const PERF_REG_X86_BX: c_int = 3;
const PERF_REG_X86_SI: c_int = 4;
const PERF_REG_X86_DI: c_int = 5;
const PERF_REG_X86_BP: c_int = 6;
const PERF_REG_X86_SP: c_int = 7;
const PERF_REG_X86_IP: c_int = 8;

const UNW_X86_EAX: c_int = 0;
const UNW_X86_EDX: c_int = 1;
const UNW_X86_ECX: c_int = 2;
const UNW_X86_EBX: c_int = 3;
const UNW_X86_ESI: c_int = 4;
const UNW_X86_EDI: c_int = 5;
const UNW_X86_EBP: c_int = 6;
const UNW_X86_ESP: c_int = 7;
const UNW_X86_EIP: c_int = 8;

#[repr(C)]
pub struct libarch_unwind__dyn_info {
    pub start_ip: u64,
    pub end_ip: u64,
    pub segbase: u64,
    pub table_data: u64,
    pub table_len: u64,
}

#[repr(C)]
pub struct unwind_info {
    pub machine: *mut machine,
    pub thread: *mut thread,
    pub sample: *mut perf_sample,
    pub cursor: *mut unw_cursor_t,
    pub ips: *mut u64,
    pub cur_ip: c_int,
    pub max_ips: c_int,
    pub unw_word_t_size: usize,
    pub e_machine: u16,
    pub best_effort: bool,
}

#[repr(C)]
pub struct unw_dyn_info_t {
    pub format: c_int,
    pub start_ip: unw_word_t,
    pub end_ip: unw_word_t,
    pub u: unw_dyn_info_t_u,
}

#[repr(C)]
pub union unw_dyn_info_t_u {
    pub rti: unw_remote_table_info,
    pub ti: unw_table_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct unw_remote_table_info {
    pub segbase: unw_word_t,
    pub table_data: unw_word_t,
    pub table_len: unw_word_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct unw_table_info {
    pub segbase: unw_word_t,
    pub table_data: unw_word_t,
    pub table_len: unw_word_t,
}

pub unsafe extern "C" fn __get_perf_regnum_for_unw_regnum_i386(unw_regnum: c_int) -> c_int {
    // #ifndef HAVE_LIBUNWIND_X86_SUPPORT
    #[cfg(not(HAVE_LIBUNWIND_X86_SUPPORT))]
    {
        let _ = unw_regnum;
        return -EINVAL;
    }

    // #else
    #[cfg(HAVE_LIBUNWIND_X86_SUPPORT)]
    {
        static PERF_I386_REGNUMS: [c_int; 9] = [
            PERF_REG_X86_AX,
            PERF_REG_X86_DX,
            PERF_REG_X86_CX,
            PERF_REG_X86_BX,
            PERF_REG_X86_SI,
            PERF_REG_X86_DI,
            PERF_REG_X86_BP,
            PERF_REG_X86_SP,
            PERF_REG_X86_IP,
        ];

        if unw_regnum == UNW_X86_EAX {
            return PERF_REG_X86_AX;
        }

        if unw_regnum < 0
            || unw_regnum >= PERF_I386_REGNUMS.len() as c_int
            || PERF_I386_REGNUMS[unw_regnum as usize] == 0
        {
            pr_err(b"unwind: invalid reg id %d\n\0".as_ptr() as *const c_char, unw_regnum);
            return -EINVAL;
        }

        PERF_I386_REGNUMS[unw_regnum as usize]
    }
}

pub unsafe extern "C" fn __libunwind_arch__flush_access_i386(maps: *mut maps) {
    // #ifdef HAVE_LIBUNWIND_X86_SUPPORT
    #[cfg(HAVE_LIBUNWIND_X86_SUPPORT)]
    {
        unw_flush_cache(maps__addr_space(maps), 0, 0);
    }
    #[cfg(not(HAVE_LIBUNWIND_X86_SUPPORT))]
    {
        let _ = maps;
    }
}

pub unsafe extern "C" fn __libunwind_arch__finish_access_i386(maps: *mut maps) {
    // #ifdef HAVE_LIBUNWIND_X86_SUPPORT
    #[cfg(HAVE_LIBUNWIND_X86_SUPPORT)]
    {
        unw_destroy_addr_space(maps__addr_space(maps));
    }
    #[cfg(not(HAVE_LIBUNWIND_X86_SUPPORT))]
    {
        let _ = maps;
    }
}

// #ifdef HAVE_LIBUNWIND_X86_SUPPORT
#[cfg(HAVE_LIBUNWIND_X86_SUPPORT)]
unsafe extern "C" fn find_proc_info(
    as_: unw_addr_space_t,
    ip: unw_word_t,
    pi: *mut unw_proc_info_t,
    need_unwind_info: c_int,
    arg: *mut c_void,
) -> c_int {
    __libunwind__find_proc_info(as_, ip, pi, need_unwind_info, arg)
}

#[cfg(HAVE_LIBUNWIND_X86_SUPPORT)]
unsafe extern "C" fn put_unwind_info(
    as_: unw_addr_space_t,
    pi: *mut unw_proc_info_t,
    arg: *mut c_void,
) {
    let _ = (as_, pi, arg);
    pr_debug(b"unwind: put_unwind_info called\n\0".as_ptr() as *const c_char);
}

#[cfg(HAVE_LIBUNWIND_X86_SUPPORT)]
unsafe extern "C" fn get_dyn_info_list_addr(
    as_: unw_addr_space_t,
    dil_addr: *mut unw_word_t,
    arg: *mut c_void,
) -> c_int {
    let _ = (as_, dil_addr, arg);
    -UNW_ENOINFO
}

#[cfg(HAVE_LIBUNWIND_X86_SUPPORT)]
unsafe extern "C" fn access_mem(
    as_: unw_addr_space_t,
    addr: unw_word_t,
    valp: *mut unw_word_t,
    __write: c_int,
    arg: *mut c_void,
) -> c_int {
    __libunwind__access_mem(as_, addr, valp, __write, arg)
}

#[cfg(HAVE_LIBUNWIND_X86_SUPPORT)]
unsafe extern "C" fn access_reg(
    as_: unw_addr_space_t,
    regnum: unw_regnum_t,
    valp: *mut unw_word_t,
    __write: c_int,
    arg: *mut c_void,
) -> c_int {
    __libunwind__access_reg(as_, regnum, valp, __write, arg)
}

#[cfg(HAVE_LIBUNWIND_X86_SUPPORT)]
unsafe extern "C" fn access_fpreg(
    as_: unw_addr_space_t,
    num: unw_regnum_t,
    val: *mut unw_fpreg_t,
    __write: c_int,
    arg: *mut c_void,
) -> c_int {
    let _ = (as_, num, val, __write, arg);
    pr_err(b"unwind: access_fpreg unsupported\n\0".as_ptr() as *const c_char);
    -UNW_EINVAL
}

#[cfg(HAVE_LIBUNWIND_X86_SUPPORT)]
unsafe extern "C" fn resume(
    as_: unw_addr_space_t,
    cu: *mut unw_cursor_t,
    arg: *mut c_void,
) -> c_int {
    let _ = (as_, cu, arg);
    pr_err(b"unwind: resume unsupported\n\0".as_ptr() as *const c_char);
    -UNW_EINVAL
}

#[cfg(HAVE_LIBUNWIND_X86_SUPPORT)]
unsafe extern "C" fn get_proc_name(
    as_: unw_addr_space_t,
    addr: unw_word_t,
    bufp: *mut c_char,
    buf_len: usize,
    offp: *mut unw_word_t,
    arg: *mut c_void,
) -> c_int {
    let _ = (as_, addr, bufp, buf_len, offp, arg);
    pr_err(b"unwind: get_proc_name unsupported\n\0".as_ptr() as *const c_char);
    -UNW_EINVAL
}

pub unsafe extern "C" fn __libunwind_arch__create_addr_space_i386() -> *mut c_void {
    // #ifdef HAVE_LIBUNWIND_X86_SUPPORT
    #[cfg(HAVE_LIBUNWIND_X86_SUPPORT)]
    {
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
        let addr_space = unw_create_addr_space(&mut accessors, 0);
        unw_set_caching_policy(addr_space, UNW_CACHE_GLOBAL);
        return addr_space;
    }

    // #else
    #[cfg(not(HAVE_LIBUNWIND_X86_SUPPORT))]
    {
        core::ptr::null_mut()
    }
}

pub unsafe extern "C" fn __libunwind_arch__dwarf_search_unwind_table_i386(
    as_: *mut c_void,
    ip: u64,
    _di: *mut libarch_unwind__dyn_info,
    pi: *mut c_void,
    need_unwind_info: c_int,
    arg: *mut c_void,
) -> c_int {
    // #ifdef HAVE_LIBUNWIND_X86_SUPPORT
    #[cfg(HAVE_LIBUNWIND_X86_SUPPORT)]
    {
        let mut di = unw_dyn_info_t {
            format: UNW_INFO_FORMAT_REMOTE_TABLE,
            start_ip: (*_di).start_ip,
            end_ip: (*_di).end_ip,
            u: unw_dyn_info_t_u {
                rti: unw_remote_table_info {
                    segbase: (*_di).segbase,
                    table_data: (*_di).table_data,
                    table_len: (*_di).table_len,
                },
            },
        };
        let ret = dwarf_search_unwind_table(
            as_,
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
        return ret;
    }

    // #else
    #[cfg(not(HAVE_LIBUNWIND_X86_SUPPORT))]
    {
        let _ = (as_, ip, _di, pi, need_unwind_info, arg);
        -EINVAL
    }
}

pub unsafe extern "C" fn __libunwind_arch__dwarf_find_debug_frame_i386(
    found: c_int,
    _di: *mut libarch_unwind__dyn_info,
    ip: u64,
    segbase: u64,
    obj_name: *const c_char,
    start: u64,
    end: u64,
) -> c_int {
    // #if defined(HAVE_LIBUNWIND_X86_SUPPORT) && !defined(NO_LIBUNWIND_DEBUG_FRAME_X86)
    #[cfg(all(HAVE_LIBUNWIND_X86_SUPPORT, not(NO_LIBUNWIND_DEBUG_FRAME_X86)))]
    {
        let mut di = unw_dyn_info_t {
            format: UNW_INFO_FORMAT_REMOTE_TABLE,
            start_ip: (*_di).start_ip,
            end_ip: (*_di).end_ip,
            u: unw_dyn_info_t_u {
                rti: unw_remote_table_info {
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
        return ret;
    }

    // #else
    #[cfg(not(all(HAVE_LIBUNWIND_X86_SUPPORT, not(NO_LIBUNWIND_DEBUG_FRAME_X86))))]
    {
        let _ = (found, _di, ip, segbase, obj_name, start, end);
        -EINVAL
    }
}

#[cfg(HAVE_LIBUNWIND_X86_SUPPORT)]
#[repr(C)]
struct arch_unwind_info {
    ui: unwind_info,
    _cursor: unw_cursor_t,
    _ips: [u64; 0],
}

pub unsafe extern "C" fn __libunwind_arch_unwind_info__new_i386(
    thread: *mut thread,
    sample: *mut perf_sample,
    max_stack: c_int,
    best_effort: bool,
    first_ip: u64,
) -> *mut unwind_info {
    // #ifdef HAVE_LIBUNWIND_X86_SUPPORT
    #[cfg(HAVE_LIBUNWIND_X86_SUPPORT)]
    {
        let maps = thread__maps(thread);
        let addr_space = maps__addr_space(maps);
        let mut ret: c_int;

        if addr_space.is_null() {
            return core::ptr::null_mut();
        }

        let ui = zalloc(
            core::mem::size_of::<arch_unwind_info>()
                + core::mem::size_of::<u64>() * max_stack as usize,
        ) as *mut arch_unwind_info;
        if ui.is_null() {
            return core::ptr::null_mut();
        }

        (*ui).ui.machine = maps__machine(maps);
        (*ui).ui.thread = thread;
        (*ui).ui.sample = sample;
        (*ui).ui.cursor = &mut (*ui)._cursor;
        (*ui).ui.ips = (*ui)._ips.as_mut_ptr();
        *(*ui).ui.ips.add(0) = first_ip;
        (*ui).ui.cur_ip = 1;
        (*ui).ui.max_ips = max_stack;
        (*ui).ui.unw_word_t_size = core::mem::size_of::<unw_word_t>();
        (*ui).ui.e_machine = EM_I386;
        (*ui).ui.best_effort = best_effort;

        ret = unw_init_remote(&mut (*ui)._cursor, addr_space, &mut (*ui).ui as *mut _ as *mut c_void);
        if ret != 0 {
            if !best_effort {
                pr_err(b"libunwind: %s\n\0".as_ptr() as *const c_char, unw_strerror(ret));
            }
            free(ui as *mut c_void);
            return core::ptr::null_mut();
        }

        &mut (*ui).ui
    }

    // #else
    #[cfg(not(HAVE_LIBUNWIND_X86_SUPPORT))]
    {
        let _ = (thread, sample, max_stack, best_effort, first_ip);
        core::ptr::null_mut()
    }
}

pub unsafe extern "C" fn __libunwind_arch__unwind_step_i386(ui: *mut unwind_info) -> c_int {
    // #ifdef HAVE_LIBUNWIND_X86_SUPPORT
    #[cfg(HAVE_LIBUNWIND_X86_SUPPORT)]
    {
        let mut ret: c_int;

        if (*ui).cur_ip >= (*ui).max_ips {
            return 0;
        }

        ret = unw_step((*ui).cursor);
        if ret > 0 {
            let mut ip: u64 = 0;

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

    // #else
    #[cfg(not(HAVE_LIBUNWIND_X86_SUPPORT))]
    {
        let _ = ui;
        -EINVAL
    }
}
