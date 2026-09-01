// SPDX-License-Identifier: GPL-2.0
// Translated from libunwind-s390.c. C include dependencies are expected to be
// supplied by the surrounding crate/bindings.

use core::ffi::{c_char, c_int, c_void};

type size_t = usize;
type uint64_t = u64;

const EINVAL: c_int = 22;
const EM_S390: c_int = 22;

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

#[allow(non_camel_case_types)]
type unw_addr_space_t = *mut c_void;
#[allow(non_camel_case_types)]
type unw_word_t = u64;
#[allow(non_camel_case_types)]
type unw_regnum_t = c_int;
#[allow(non_camel_case_types)]
type unw_fpreg_t = u64;

#[repr(C)]
pub struct unw_cursor_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct unw_proc_info_t {
    _private: [u8; 0],
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
    pub best_effort: bool,
}

#[repr(C)]
pub struct libarch_unwind__dyn_info {
    pub start_ip: unw_word_t,
    pub end_ip: unw_word_t,
    pub segbase: unw_word_t,
    pub table_data: unw_word_t,
    pub table_len: unw_word_t,
}

#[cfg(HAVE_LIBUNWIND_S390X_SUPPORT)]
const UNW_ENOINFO: c_int = 10;
#[cfg(HAVE_LIBUNWIND_S390X_SUPPORT)]
const UNW_EINVAL: c_int = 22;
#[cfg(HAVE_LIBUNWIND_S390X_SUPPORT)]
const UNW_CACHE_GLOBAL: c_int = 2;
#[cfg(HAVE_LIBUNWIND_S390X_SUPPORT)]
const UNW_INFO_FORMAT_REMOTE_TABLE: c_int = 1;
#[cfg(HAVE_LIBUNWIND_S390X_SUPPORT)]
const UNW_REG_IP: c_int = 16;

#[cfg(HAVE_LIBUNWIND_S390X_SUPPORT)]
const UNW_S390X_R0: c_int = 0;
#[cfg(HAVE_LIBUNWIND_S390X_SUPPORT)]
const UNW_S390X_R15: c_int = 15;
#[cfg(HAVE_LIBUNWIND_S390X_SUPPORT)]
const UNW_S390X_F0: c_int = 16;
#[cfg(HAVE_LIBUNWIND_S390X_SUPPORT)]
const UNW_S390X_F15: c_int = 31;
#[cfg(HAVE_LIBUNWIND_S390X_SUPPORT)]
const UNW_S390X_IP: c_int = 64;

#[cfg(HAVE_LIBUNWIND_S390X_SUPPORT)]
const PERF_REG_S390_R0: c_int = 0;
#[cfg(HAVE_LIBUNWIND_S390X_SUPPORT)]
const PERF_REG_S390_FP0: c_int = 16;
#[cfg(HAVE_LIBUNWIND_S390X_SUPPORT)]
const PERF_REG_S390_PC: c_int = 32;

#[cfg(HAVE_LIBUNWIND_S390X_SUPPORT)]
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

#[cfg(HAVE_LIBUNWIND_S390X_SUPPORT)]
#[repr(C)]
pub struct unw_dyn_info_rti {
    pub segbase: unw_word_t,
    pub table_data: unw_word_t,
    pub table_len: unw_word_t,
}

#[cfg(HAVE_LIBUNWIND_S390X_SUPPORT)]
#[repr(C)]
pub union unw_dyn_info_u {
    pub rti: unw_dyn_info_rti,
    pub ti: unw_dyn_info_rti,
}

#[cfg(HAVE_LIBUNWIND_S390X_SUPPORT)]
#[repr(C)]
pub struct unw_dyn_info_t {
    pub format: c_int,
    pub start_ip: unw_word_t,
    pub end_ip: unw_word_t,
    pub u: unw_dyn_info_u,
}

#[cfg(HAVE_LIBUNWIND_S390X_SUPPORT)]
#[repr(C)]
struct arch_unwind_info {
    ui: unwind_info,
    _cursor: unw_cursor_t,
    _ips: [uint64_t; 0],
}

extern "C" {
    fn pr_err(fmt: *const c_char, ...);
    fn pr_debug(fmt: *const c_char, ...);
    fn free(ptr: *mut c_void);
    fn zalloc(size: size_t) -> *mut c_void;
    fn thread__maps(thread: *mut thread) -> *mut maps;
    fn maps__addr_space(maps: *mut maps) -> *mut c_void;
    fn maps__machine(maps: *mut maps) -> *mut machine;

    #[cfg(HAVE_LIBUNWIND_S390X_SUPPORT)]
    fn unw_flush_cache(as_: unw_addr_space_t, lo: unw_word_t, hi: unw_word_t);
    #[cfg(HAVE_LIBUNWIND_S390X_SUPPORT)]
    fn unw_destroy_addr_space(as_: unw_addr_space_t);
    #[cfg(HAVE_LIBUNWIND_S390X_SUPPORT)]
    fn __libunwind__find_proc_info(
        as_: unw_addr_space_t,
        ip: unw_word_t,
        pi: *mut unw_proc_info_t,
        need_unwind_info: c_int,
        arg: *mut c_void,
    ) -> c_int;
    #[cfg(HAVE_LIBUNWIND_S390X_SUPPORT)]
    fn __libunwind__access_mem(
        as_: unw_addr_space_t,
        addr: unw_word_t,
        valp: *mut unw_word_t,
        __write: c_int,
        arg: *mut c_void,
    ) -> c_int;
    #[cfg(HAVE_LIBUNWIND_S390X_SUPPORT)]
    fn __libunwind__access_reg(
        as_: unw_addr_space_t,
        regnum: unw_regnum_t,
        valp: *mut unw_word_t,
        __write: c_int,
        arg: *mut c_void,
    ) -> c_int;
    #[cfg(HAVE_LIBUNWIND_S390X_SUPPORT)]
    fn unw_create_addr_space(accessors: *mut unw_accessors_t, byte_order: c_int) -> unw_addr_space_t;
    #[cfg(HAVE_LIBUNWIND_S390X_SUPPORT)]
    fn unw_set_caching_policy(as_: unw_addr_space_t, policy: c_int);
    #[cfg(HAVE_LIBUNWIND_S390X_SUPPORT)]
    fn dwarf_search_unwind_table(
        as_: unw_addr_space_t,
        ip: unw_word_t,
        di: *mut unw_dyn_info_t,
        pi: *mut unw_proc_info_t,
        need_unwind_info: c_int,
        arg: *mut c_void,
    ) -> c_int;
    #[cfg(all(HAVE_LIBUNWIND_S390X_SUPPORT, not(NO_LIBUNWIND_DEBUG_FRAME_S390X)))]
    fn dwarf_find_debug_frame(
        found: c_int,
        di_debug: *mut unw_dyn_info_t,
        ip: unw_word_t,
        segbase: unw_word_t,
        obj_name: *const c_char,
        start: unw_word_t,
        end: unw_word_t,
    ) -> c_int;
    #[cfg(HAVE_LIBUNWIND_S390X_SUPPORT)]
    fn unw_init_remote(cu: *mut unw_cursor_t, as_: unw_addr_space_t, arg: *mut c_void) -> c_int;
    #[cfg(HAVE_LIBUNWIND_S390X_SUPPORT)]
    fn unw_strerror(ret: c_int) -> *const c_char;
    #[cfg(HAVE_LIBUNWIND_S390X_SUPPORT)]
    fn unw_step(cu: *mut unw_cursor_t) -> c_int;
    #[cfg(HAVE_LIBUNWIND_S390X_SUPPORT)]
    fn unw_get_reg(cu: *mut unw_cursor_t, regnum: c_int, valp: *mut uint64_t) -> c_int;
    #[cfg(HAVE_LIBUNWIND_S390X_SUPPORT)]
    fn unw_is_signal_frame(cu: *mut unw_cursor_t) -> c_int;
}

#[no_mangle]
pub unsafe extern "C" fn __get_perf_regnum_for_unw_regnum_s390(unw_regnum: c_int) -> c_int {
    #[cfg(not(HAVE_LIBUNWIND_S390X_SUPPORT))]
    {
        let _ = unw_regnum;
        return -EINVAL;
    }

    #[cfg(HAVE_LIBUNWIND_S390X_SUPPORT)]
    {
        match unw_regnum {
            UNW_S390X_R0..=UNW_S390X_R15 => unw_regnum - UNW_S390X_R0 + PERF_REG_S390_R0,
            UNW_S390X_F0..=UNW_S390X_F15 => unw_regnum - UNW_S390X_F0 + PERF_REG_S390_FP0,
            UNW_S390X_IP => PERF_REG_S390_PC,
            _ => {
                pr_err(
                    b"unwind: invalid reg id %d\n\0".as_ptr() as *const c_char,
                    unw_regnum,
                );
                -EINVAL
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn __libunwind_arch__flush_access_s390(maps: *mut maps) {
    #[cfg(HAVE_LIBUNWIND_S390X_SUPPORT)]
    {
        unw_flush_cache(maps__addr_space(maps), 0, 0);
    }

    #[cfg(not(HAVE_LIBUNWIND_S390X_SUPPORT))]
    {
        let _ = maps;
    }
}

#[no_mangle]
pub unsafe extern "C" fn __libunwind_arch__finish_access_s390(maps: *mut maps) {
    #[cfg(HAVE_LIBUNWIND_S390X_SUPPORT)]
    {
        unw_destroy_addr_space(maps__addr_space(maps));
    }

    #[cfg(not(HAVE_LIBUNWIND_S390X_SUPPORT))]
    {
        let _ = maps;
    }
}

#[cfg(HAVE_LIBUNWIND_S390X_SUPPORT)]
unsafe extern "C" fn find_proc_info(
    as_: unw_addr_space_t,
    ip: unw_word_t,
    pi: *mut unw_proc_info_t,
    need_unwind_info: c_int,
    arg: *mut c_void,
) -> c_int {
    __libunwind__find_proc_info(as_, ip, pi, need_unwind_info, arg)
}

#[cfg(HAVE_LIBUNWIND_S390X_SUPPORT)]
unsafe extern "C" fn put_unwind_info(
    _as: unw_addr_space_t,
    _pi: *mut unw_proc_info_t,
    _arg: *mut c_void,
) {
    pr_debug(b"unwind: put_unwind_info called\n\0".as_ptr() as *const c_char);
}

#[cfg(HAVE_LIBUNWIND_S390X_SUPPORT)]
unsafe extern "C" fn get_dyn_info_list_addr(
    _as: unw_addr_space_t,
    _dil_addr: *mut unw_word_t,
    _arg: *mut c_void,
) -> c_int {
    -UNW_ENOINFO
}

#[cfg(HAVE_LIBUNWIND_S390X_SUPPORT)]
unsafe extern "C" fn access_mem(
    as_: unw_addr_space_t,
    addr: unw_word_t,
    valp: *mut unw_word_t,
    __write: c_int,
    arg: *mut c_void,
) -> c_int {
    __libunwind__access_mem(as_, addr, valp, __write, arg)
}

#[cfg(HAVE_LIBUNWIND_S390X_SUPPORT)]
unsafe extern "C" fn access_reg(
    as_: unw_addr_space_t,
    regnum: unw_regnum_t,
    valp: *mut unw_word_t,
    __write: c_int,
    arg: *mut c_void,
) -> c_int {
    __libunwind__access_reg(as_, regnum, valp, __write, arg)
}

#[cfg(HAVE_LIBUNWIND_S390X_SUPPORT)]
unsafe extern "C" fn access_fpreg(
    _as: unw_addr_space_t,
    _num: unw_regnum_t,
    _val: *mut unw_fpreg_t,
    _write: c_int,
    _arg: *mut c_void,
) -> c_int {
    pr_err(b"unwind: access_fpreg unsupported\n\0".as_ptr() as *const c_char);
    -UNW_EINVAL
}

#[cfg(HAVE_LIBUNWIND_S390X_SUPPORT)]
unsafe extern "C" fn resume(
    _as: unw_addr_space_t,
    _cu: *mut unw_cursor_t,
    _arg: *mut c_void,
) -> c_int {
    pr_err(b"unwind: resume unsupported\n\0".as_ptr() as *const c_char);
    -UNW_EINVAL
}

#[cfg(HAVE_LIBUNWIND_S390X_SUPPORT)]
unsafe extern "C" fn get_proc_name(
    _as: unw_addr_space_t,
    _addr: unw_word_t,
    _bufp: *mut c_char,
    _buf_len: size_t,
    _offp: *mut unw_word_t,
    _arg: *mut c_void,
) -> c_int {
    pr_err(b"unwind: get_proc_name unsupported\n\0".as_ptr() as *const c_char);
    -UNW_EINVAL
}

#[no_mangle]
pub unsafe extern "C" fn __libunwind_arch__create_addr_space_s390() -> *mut c_void {
    #[cfg(HAVE_LIBUNWIND_S390X_SUPPORT)]
    {
        static mut ACCESSORS: unw_accessors_t = unw_accessors_t {
            find_proc_info: Some(find_proc_info),
            put_unwind_info: Some(put_unwind_info),
            get_dyn_info_list_addr: Some(get_dyn_info_list_addr),
            access_mem: Some(access_mem),
            access_reg: Some(access_reg),
            access_fpreg: Some(access_fpreg),
            resume: Some(resume),
            get_proc_name: Some(get_proc_name),
        };
        let addr_space = unw_create_addr_space(&mut ACCESSORS, 0);
        unw_set_caching_policy(addr_space, UNW_CACHE_GLOBAL);
        return addr_space;
    }

    #[cfg(not(HAVE_LIBUNWIND_S390X_SUPPORT))]
    {
        core::ptr::null_mut()
    }
}

#[no_mangle]
pub unsafe extern "C" fn __libunwind_arch__dwarf_search_unwind_table_s390(
    as_: *mut c_void,
    ip: uint64_t,
    _di: *mut libarch_unwind__dyn_info,
    pi: *mut c_void,
    need_unwind_info: c_int,
    arg: *mut c_void,
) -> c_int {
    #[cfg(HAVE_LIBUNWIND_S390X_SUPPORT)]
    {
        let mut di = unw_dyn_info_t {
            format: UNW_INFO_FORMAT_REMOTE_TABLE,
            start_ip: (*_di).start_ip,
            end_ip: (*_di).end_ip,
            u: unw_dyn_info_u {
                rti: unw_dyn_info_rti {
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

    #[cfg(not(HAVE_LIBUNWIND_S390X_SUPPORT))]
    {
        let _ = (as_, ip, _di, pi, need_unwind_info, arg);
        -EINVAL
    }
}

#[no_mangle]
pub unsafe extern "C" fn __libunwind_arch__dwarf_find_debug_frame_s390(
    found: c_int,
    _di: *mut libarch_unwind__dyn_info,
    ip: uint64_t,
    segbase: uint64_t,
    obj_name: *const c_char,
    start: uint64_t,
    end: uint64_t,
) -> c_int {
    #[cfg(all(HAVE_LIBUNWIND_S390X_SUPPORT, not(NO_LIBUNWIND_DEBUG_FRAME_S390X)))]
    {
        let mut di = unw_dyn_info_t {
            format: UNW_INFO_FORMAT_REMOTE_TABLE,
            start_ip: (*_di).start_ip,
            end_ip: (*_di).end_ip,
            u: unw_dyn_info_u {
                rti: unw_dyn_info_rti {
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

    #[cfg(not(all(HAVE_LIBUNWIND_S390X_SUPPORT, not(NO_LIBUNWIND_DEBUG_FRAME_S390X))))]
    {
        let _ = (found, _di, ip, segbase, obj_name, start, end);
        -EINVAL
    }
}

#[no_mangle]
pub unsafe extern "C" fn __libunwind_arch_unwind_info__new_s390(
    thread: *mut thread,
    sample: *mut perf_sample,
    max_stack: c_int,
    best_effort: bool,
    first_ip: uint64_t,
) -> *mut unwind_info {
    #[cfg(HAVE_LIBUNWIND_S390X_SUPPORT)]
    {
        let maps = thread__maps(thread);
        let addr_space = maps__addr_space(maps);
        let mut ret: c_int;

        if addr_space.is_null() {
            return core::ptr::null_mut();
        }

        let ui = zalloc(
            core::mem::size_of::<arch_unwind_info>()
                + core::mem::size_of::<uint64_t>() * max_stack as usize,
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
        (*ui).ui.e_machine = EM_S390;
        (*ui).ui.best_effort = best_effort;

        ret = unw_init_remote(&mut (*ui)._cursor, addr_space, &mut (*ui).ui as *mut _ as *mut c_void);
        if ret != 0 {
            if !best_effort {
                pr_err(
                    b"libunwind: %s\n\0".as_ptr() as *const c_char,
                    unw_strerror(ret),
                );
            }
            free(ui as *mut c_void);
            return core::ptr::null_mut();
        }

        return &mut (*ui).ui;
    }

    #[cfg(not(HAVE_LIBUNWIND_S390X_SUPPORT))]
    {
        let _ = (thread, sample, max_stack, best_effort, first_ip);
        core::ptr::null_mut()
    }
}

#[no_mangle]
pub unsafe extern "C" fn __libunwind_arch__unwind_step_s390(ui: *mut unwind_info) -> c_int {
    #[cfg(HAVE_LIBUNWIND_S390X_SUPPORT)]
    {
        let mut ret: c_int;

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
        return ret;
    }

    #[cfg(not(HAVE_LIBUNWIND_S390X_SUPPORT))]
    {
        let _ = ui;
        -EINVAL
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
