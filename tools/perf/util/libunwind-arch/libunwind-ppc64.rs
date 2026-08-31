// SPDX-License-Identifier: GPL-2.0-or-later
// Translated from perf/util/libunwind-arch/libunwind-ppc64.c.
// C includes referenced libunwind-arch.h, debug.h, maps.h, thread.h,
// arch/powerpc perf_regs.h, linux compiler/kernel/zalloc, elf.h, and errno.h.

use core::ffi::{c_char, c_int, c_void};

extern "C" {
    static mut UNW_PPC64_R0: c_int;
    static mut UNW_PPC64_R31: c_int;
    static mut UNW_PPC64_LR: c_int;
    static mut UNW_PPC64_CTR: c_int;
    static mut UNW_PPC64_XER: c_int;
    static mut UNW_PPC64_NIP: c_int;
    static mut PERF_REG_POWERPC_R0: c_int;
    static mut PERF_REG_POWERPC_LINK: c_int;
    static mut PERF_REG_POWERPC_CTR: c_int;
    static mut PERF_REG_POWERPC_XER: c_int;
    static mut PERF_REG_POWERPC_NIP: c_int;
}

const EINVAL: c_int = 22;
const UNW_ENOINFO: c_int = 10;
const UNW_EINVAL: c_int = 8;
const UNW_CACHE_GLOBAL: c_int = 1;
const UNW_INFO_FORMAT_REMOTE_TABLE: c_int = 1;
const UNW_REG_IP: c_int = 16;
const EM_PPC64: c_int = 21;

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
pub struct unw_remote_table_info {
    pub segbase: unw_word_t,
    pub table_data: unw_word_t,
    pub table_len: unw_word_t,
}

#[repr(C)]
pub union unw_dyn_info_union {
    pub rti: unw_remote_table_info,
    pub ti: unw_remote_table_info,
}

#[repr(C)]
pub struct unw_dyn_info_t {
    pub format: c_int,
    pub start_ip: unw_word_t,
    pub end_ip: unw_word_t,
    pub u: unw_dyn_info_union,
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
    pub resume:
        Option<unsafe extern "C" fn(unw_addr_space_t, *mut unw_cursor_t, *mut c_void) -> c_int>,
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

extern "C" {
    fn pr_err(fmt: *const c_char, ...);
    fn pr_debug(fmt: *const c_char, ...);
    fn free(ptr: *mut c_void);
    fn zalloc(size: size_t) -> *mut c_void;
    fn maps__addr_space(maps: *mut maps) -> *mut c_void;
    fn maps__machine(maps: *mut maps) -> *mut machine;
    fn thread__maps(thread: *mut thread) -> *mut maps;
}

#[cfg(feature = "HAVE_LIBUNWIND_PPC64_SUPPORT")]
extern "C" {
    fn unw_flush_cache(as_: unw_addr_space_t, start: unw_word_t, end: unw_word_t);
    fn unw_destroy_addr_space(as_: unw_addr_space_t);
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
    fn unw_create_addr_space(accessors: *mut unw_accessors_t, byte_order: c_int) -> unw_addr_space_t;
    fn unw_set_caching_policy(as_: unw_addr_space_t, policy: c_int) -> c_int;
    fn unw_init_remote(
        cursor: *mut unw_cursor_t,
        as_: unw_addr_space_t,
        arg: *mut c_void,
    ) -> c_int;
    fn unw_strerror(ret: c_int) -> *const c_char;
    fn unw_step(cursor: *mut unw_cursor_t) -> c_int;
    fn unw_get_reg(cursor: *mut unw_cursor_t, regnum: unw_regnum_t, valp: *mut unw_word_t)
        -> c_int;
    fn unw_is_signal_frame(cursor: *mut unw_cursor_t) -> c_int;
}

#[no_mangle]
pub unsafe extern "C" fn __get_perf_regnum_for_unw_regnum_ppc64(unw_regnum: c_int) -> c_int {
    #[cfg(not(feature = "HAVE_LIBUNWIND_PPC64_SUPPORT"))]
    {
        let _ = unw_regnum;
        return -EINVAL;
    }

    #[cfg(feature = "HAVE_LIBUNWIND_PPC64_SUPPORT")]
    {
        if unw_regnum >= UNW_PPC64_R0 && unw_regnum <= UNW_PPC64_R31 {
            return unw_regnum - UNW_PPC64_R0 + PERF_REG_POWERPC_R0;
        }

        if unw_regnum == UNW_PPC64_LR {
            PERF_REG_POWERPC_LINK
        } else if unw_regnum == UNW_PPC64_CTR {
            PERF_REG_POWERPC_CTR
        } else if unw_regnum == UNW_PPC64_XER {
            PERF_REG_POWERPC_XER
        } else if unw_regnum == UNW_PPC64_NIP {
            PERF_REG_POWERPC_NIP
        } else {
            pr_err(b"unwind: invalid reg id %d\n\0".as_ptr() as *const c_char, unw_regnum);
            -EINVAL
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn __libunwind_arch__flush_access_ppc64(maps: *mut maps) {
    #[cfg(feature = "HAVE_LIBUNWIND_PPC64_SUPPORT")]
    {
        unw_flush_cache(maps__addr_space(maps) as unw_addr_space_t, 0, 0);
    }
    #[cfg(not(feature = "HAVE_LIBUNWIND_PPC64_SUPPORT"))]
    {
        let _ = maps;
    }
}

#[no_mangle]
pub unsafe extern "C" fn __libunwind_arch__finish_access_ppc64(maps: *mut maps) {
    #[cfg(feature = "HAVE_LIBUNWIND_PPC64_SUPPORT")]
    {
        unw_destroy_addr_space(maps__addr_space(maps) as unw_addr_space_t);
    }
    #[cfg(not(feature = "HAVE_LIBUNWIND_PPC64_SUPPORT"))]
    {
        let _ = maps;
    }
}

#[cfg(feature = "HAVE_LIBUNWIND_PPC64_SUPPORT")]
unsafe extern "C" fn find_proc_info(
    as_: unw_addr_space_t,
    ip: unw_word_t,
    pi: *mut unw_proc_info_t,
    need_unwind_info: c_int,
    arg: *mut c_void,
) -> c_int {
    __libunwind__find_proc_info(as_, ip, pi, need_unwind_info, arg)
}

#[cfg(feature = "HAVE_LIBUNWIND_PPC64_SUPPORT")]
unsafe extern "C" fn put_unwind_info(
    _as: unw_addr_space_t,
    _pi: *mut unw_proc_info_t,
    _arg: *mut c_void,
) {
    pr_debug(b"unwind: put_unwind_info called\n\0".as_ptr() as *const c_char);
}

#[cfg(feature = "HAVE_LIBUNWIND_PPC64_SUPPORT")]
unsafe extern "C" fn get_dyn_info_list_addr(
    _as: unw_addr_space_t,
    _dil_addr: *mut unw_word_t,
    _arg: *mut c_void,
) -> c_int {
    -UNW_ENOINFO
}

#[cfg(feature = "HAVE_LIBUNWIND_PPC64_SUPPORT")]
unsafe extern "C" fn access_mem(
    as_: unw_addr_space_t,
    addr: unw_word_t,
    valp: *mut unw_word_t,
    __write: c_int,
    arg: *mut c_void,
) -> c_int {
    __libunwind__access_mem(as_, addr, valp, __write, arg)
}

#[cfg(feature = "HAVE_LIBUNWIND_PPC64_SUPPORT")]
unsafe extern "C" fn access_reg(
    as_: unw_addr_space_t,
    regnum: unw_regnum_t,
    valp: *mut unw_word_t,
    __write: c_int,
    arg: *mut c_void,
) -> c_int {
    __libunwind__access_reg(as_, regnum, valp, __write, arg)
}

#[cfg(feature = "HAVE_LIBUNWIND_PPC64_SUPPORT")]
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

#[cfg(feature = "HAVE_LIBUNWIND_PPC64_SUPPORT")]
unsafe extern "C" fn resume(
    _as: unw_addr_space_t,
    _cu: *mut unw_cursor_t,
    _arg: *mut c_void,
) -> c_int {
    pr_err(b"unwind: resume unsupported\n\0".as_ptr() as *const c_char);
    -UNW_EINVAL
}

#[cfg(feature = "HAVE_LIBUNWIND_PPC64_SUPPORT")]
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
pub unsafe extern "C" fn __libunwind_arch__create_addr_space_ppc64() -> *mut c_void {
    #[cfg(feature = "HAVE_LIBUNWIND_PPC64_SUPPORT")]
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
        let addr_space: unw_addr_space_t = unw_create_addr_space(&mut ACCESSORS, 0);
        unw_set_caching_policy(addr_space, UNW_CACHE_GLOBAL);
        return addr_space as *mut c_void;
    }

    #[cfg(not(feature = "HAVE_LIBUNWIND_PPC64_SUPPORT"))]
    {
        core::ptr::null_mut()
    }
}

#[cfg(feature = "HAVE_LIBUNWIND_PPC64_SUPPORT")]
extern "C" {
    // C spelling: UNW_OBJ(dwarf_search_unwind_table)
    fn dwarf_search_unwind_table(
        as_: unw_addr_space_t,
        ip: unw_word_t,
        di: *mut unw_dyn_info_t,
        pi: *mut c_void,
        need_unwind_info: c_int,
        arg: *mut c_void,
    ) -> c_int;
}

#[no_mangle]
pub unsafe extern "C" fn __libunwind_arch__dwarf_search_unwind_table_ppc64(
    as_: *mut c_void,
    ip: uint64_t,
    _di: *mut libarch_unwind__dyn_info,
    pi: *mut c_void,
    need_unwind_info: c_int,
    arg: *mut c_void,
) -> c_int {
    #[cfg(feature = "HAVE_LIBUNWIND_PPC64_SUPPORT")]
    {
        let mut di = unw_dyn_info_t {
            format: UNW_INFO_FORMAT_REMOTE_TABLE,
            start_ip: (*_di).start_ip as unw_word_t,
            end_ip: (*_di).end_ip as unw_word_t,
            u: unw_dyn_info_union {
                rti: unw_remote_table_info {
                    segbase: (*_di).segbase as unw_word_t,
                    table_data: (*_di).table_data as unw_word_t,
                    table_len: (*_di).table_len as unw_word_t,
                },
            },
        };
        let ret = dwarf_search_unwind_table(
            as_ as unw_addr_space_t,
            ip as unw_word_t,
            &mut di,
            pi,
            need_unwind_info,
            arg,
        );

        (*_di).start_ip = di.start_ip as uint64_t;
        (*_di).end_ip = di.end_ip as uint64_t;
        (*_di).segbase = di.u.rti.segbase as uint64_t;
        (*_di).table_data = di.u.rti.table_data as uint64_t;
        (*_di).table_len = di.u.rti.table_len as uint64_t;
        return ret;
    }

    #[cfg(not(feature = "HAVE_LIBUNWIND_PPC64_SUPPORT"))]
    {
        let _ = (as_, ip, _di, pi, need_unwind_info, arg);
        -EINVAL
    }
}

#[cfg(all(
    feature = "HAVE_LIBUNWIND_PPC64_SUPPORT",
    not(feature = "NO_LIBUNWIND_DEBUG_FRAME_PPC64")
))]
extern "C" {
    // C spelling: UNW_OBJ(dwarf_find_debug_frame)
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

#[no_mangle]
pub unsafe extern "C" fn __libunwind_arch__dwarf_find_debug_frame_ppc64(
    found: c_int,
    _di: *mut libarch_unwind__dyn_info,
    ip: uint64_t,
    segbase: uint64_t,
    obj_name: *const c_char,
    start: uint64_t,
    end: uint64_t,
) -> c_int {
    #[cfg(all(
        feature = "HAVE_LIBUNWIND_PPC64_SUPPORT",
        not(feature = "NO_LIBUNWIND_DEBUG_FRAME_PPC64")
    ))]
    {
        let mut di = unw_dyn_info_t {
            format: UNW_INFO_FORMAT_REMOTE_TABLE,
            start_ip: (*_di).start_ip as unw_word_t,
            end_ip: (*_di).end_ip as unw_word_t,
            u: unw_dyn_info_union {
                rti: unw_remote_table_info {
                    segbase: (*_di).segbase as unw_word_t,
                    table_data: (*_di).table_data as unw_word_t,
                    table_len: (*_di).table_len as unw_word_t,
                },
            },
        };
        let ret = dwarf_find_debug_frame(
            found,
            &mut di,
            ip as unw_word_t,
            segbase as unw_word_t,
            obj_name,
            start as unw_word_t,
            end as unw_word_t,
        );

        (*_di).start_ip = di.start_ip as uint64_t;
        (*_di).end_ip = di.end_ip as uint64_t;
        (*_di).segbase = di.u.ti.segbase as uint64_t;
        (*_di).table_data = di.u.ti.table_data as uint64_t;
        (*_di).table_len = di.u.ti.table_len as uint64_t;
        return ret;
    }

    #[cfg(not(all(
        feature = "HAVE_LIBUNWIND_PPC64_SUPPORT",
        not(feature = "NO_LIBUNWIND_DEBUG_FRAME_PPC64")
    )))]
    {
        let _ = (found, _di, ip, segbase, obj_name, start, end);
        -EINVAL
    }
}

#[cfg(feature = "HAVE_LIBUNWIND_PPC64_SUPPORT")]
#[repr(C)]
struct arch_unwind_info {
    ui: unwind_info,
    _cursor: unw_cursor_t,
    _ips: [uint64_t; 0],
}

#[no_mangle]
pub unsafe extern "C" fn __libunwind_arch_unwind_info__new_ppc64(
    thread: *mut thread,
    sample: *mut perf_sample,
    max_stack: c_int,
    best_effort: bool_,
    first_ip: uint64_t,
) -> *mut unwind_info {
    #[cfg(feature = "HAVE_LIBUNWIND_PPC64_SUPPORT")]
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
        (*ui).ui.e_machine = EM_PPC64;
        (*ui).ui.best_effort = best_effort;

        ret = unw_init_remote(&mut (*ui)._cursor, addr_space as unw_addr_space_t, &mut (*ui).ui as *mut _ as *mut c_void);
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

    #[cfg(not(feature = "HAVE_LIBUNWIND_PPC64_SUPPORT"))]
    {
        let _ = (thread, sample, max_stack, best_effort, first_ip);
        core::ptr::null_mut()
    }
}

#[no_mangle]
pub unsafe extern "C" fn __libunwind_arch__unwind_step_ppc64(ui: *mut unwind_info) -> c_int {
    #[cfg(feature = "HAVE_LIBUNWIND_PPC64_SUPPORT")]
    {
        let ret: c_int;

        if (*ui).cur_ip >= (*ui).max_ips {
            return 0;
        }

        ret = unw_step((*ui).cursor);
        if ret > 0 {
            let mut ip: uint64_t = 0;

            unw_get_reg((*ui).cursor, UNW_REG_IP, &mut ip as *mut _ as *mut unw_word_t);

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

    #[cfg(not(feature = "HAVE_LIBUNWIND_PPC64_SUPPORT"))]
    {
        let _ = ui;
        -EINVAL
    }
}
