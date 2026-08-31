// SPDX-License-Identifier: GPL-2.0
// Translated from C. Original dependencies:
// "libunwind-arch.h", "../debug.h", "../maps.h", "../thread.h",
// "../../../arch/arm64/include/uapi/asm/perf_regs.h",
// <linux/compiler.h>, <linux/kernel.h>, <linux/zalloc.h>, <elf.h>, <errno.h>
// Optional libunwind dependency: <libunwind-aarch64.h>

use core::ffi::{c_char, c_int, c_void};

const EINVAL: c_int = 22;
const EM_AARCH64: c_int = 183;

extern "C" {
    static PERF_REG_ARM64_EXTENDED_MAX: c_int;

    fn pr_err(fmt: *const c_char, ...);
    fn pr_debug(fmt: *const c_char, ...);

    fn maps__addr_space(maps: *mut maps) -> *mut c_void;
    fn maps__machine(maps: *mut maps) -> *mut c_void;
    fn thread__maps(thread: *mut thread) -> *mut maps;
    fn zalloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
}

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
pub struct unwind_info {
    pub machine: *mut c_void,
    pub thread: *mut thread,
    pub sample: *mut perf_sample,
    pub cursor: *mut unw_cursor_t,
    pub ips: *mut u64,
    pub cur_ip: c_int,
    pub max_ips: c_int,
    pub unw_word_t_size: usize,
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

pub type unw_addr_space_t = *mut c_void;
pub type unw_word_t = u64;
pub type unw_regnum_t = c_int;
pub type unw_fpreg_t = u64;

#[repr(C)]
pub struct unw_cursor_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct unw_proc_info_t {
    _private: [u8; 0],
}

#[cfg(any(HAVE_LIBUNWIND_AARCH64_SUPPORT, HAVE_LIBUNWIND_ARM64_SUPPORT))]
#[repr(C)]
pub struct unw_remote_table_info {
    pub segbase: unw_word_t,
    pub table_data: unw_word_t,
    pub table_len: unw_word_t,
}

#[cfg(any(HAVE_LIBUNWIND_AARCH64_SUPPORT, HAVE_LIBUNWIND_ARM64_SUPPORT))]
#[repr(C)]
pub union unw_dyn_info_u {
    pub rti: unw_remote_table_info,
    pub ti: unw_remote_table_info,
}

#[cfg(any(HAVE_LIBUNWIND_AARCH64_SUPPORT, HAVE_LIBUNWIND_ARM64_SUPPORT))]
#[repr(C)]
pub struct unw_dyn_info_t {
    pub format: c_int,
    pub start_ip: unw_word_t,
    pub end_ip: unw_word_t,
    pub u: unw_dyn_info_u,
}

#[cfg(HAVE_LIBUNWIND_AARCH64_SUPPORT)]
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
        unsafe extern "C" fn(unw_addr_space_t, unw_word_t, *mut unw_word_t, c_int, *mut c_void) -> c_int,
    >,
    pub access_reg: Option<
        unsafe extern "C" fn(unw_addr_space_t, unw_regnum_t, *mut unw_word_t, c_int, *mut c_void) -> c_int,
    >,
    pub access_fpreg: Option<
        unsafe extern "C" fn(unw_addr_space_t, unw_regnum_t, *mut unw_fpreg_t, c_int, *mut c_void) -> c_int,
    >,
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

#[cfg(any(HAVE_LIBUNWIND_AARCH64_SUPPORT, HAVE_LIBUNWIND_ARM64_SUPPORT))]
extern "C" {
    static UNW_ENOINFO: c_int;
    static UNW_EINVAL: c_int;
    static UNW_INFO_FORMAT_REMOTE_TABLE: c_int;
    static UNW_CACHE_GLOBAL: c_int;
    static UNW_REG_IP: c_int;

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
    fn unw_set_caching_policy(as_: unw_addr_space_t, policy: c_int);
    fn unw_init_remote(cursor: *mut unw_cursor_t, as_: unw_addr_space_t, arg: *mut c_void) -> c_int;
    fn unw_strerror(ret: c_int) -> *const c_char;
    fn unw_step(cursor: *mut unw_cursor_t) -> c_int;
    fn unw_get_reg(cursor: *mut unw_cursor_t, regnum: c_int, valp: *mut u64) -> c_int;
    fn unw_is_signal_frame(cursor: *mut unw_cursor_t) -> c_int;
}

#[no_mangle]
pub unsafe extern "C" fn __get_perf_regnum_for_unw_regnum_arm64(unw_regnum: c_int) -> c_int {
    if unw_regnum < 0 || unw_regnum >= PERF_REG_ARM64_EXTENDED_MAX {
        pr_err(
            b"unwind: invalid reg id %d\n\0".as_ptr() as *const c_char,
            unw_regnum,
        );
        return -EINVAL;
    }
    unw_regnum
}

#[no_mangle]
pub unsafe extern "C" fn __libunwind_arch__flush_access_arm64(maps: *mut maps) {
    #[cfg(HAVE_LIBUNWIND_AARCH64_SUPPORT)]
    {
        unw_flush_cache(maps__addr_space(maps), 0, 0);
    }

    #[cfg(not(HAVE_LIBUNWIND_AARCH64_SUPPORT))]
    {
        let _ = maps;
    }
}

#[no_mangle]
pub unsafe extern "C" fn __libunwind_arch__finish_access_arm64(maps: *mut maps) {
    #[cfg(HAVE_LIBUNWIND_AARCH64_SUPPORT)]
    {
        unw_destroy_addr_space(maps__addr_space(maps));
    }

    #[cfg(not(HAVE_LIBUNWIND_AARCH64_SUPPORT))]
    {
        let _ = maps;
    }
}

#[cfg(HAVE_LIBUNWIND_AARCH64_SUPPORT)]
unsafe extern "C" fn find_proc_info(
    as_: unw_addr_space_t,
    ip: unw_word_t,
    pi: *mut unw_proc_info_t,
    need_unwind_info: c_int,
    arg: *mut c_void,
) -> c_int {
    __libunwind__find_proc_info(as_, ip, pi, need_unwind_info, arg)
}

#[cfg(HAVE_LIBUNWIND_AARCH64_SUPPORT)]
unsafe extern "C" fn put_unwind_info(
    as_: unw_addr_space_t,
    pi: *mut unw_proc_info_t,
    arg: *mut c_void,
) {
    let _ = as_;
    let _ = pi;
    let _ = arg;
    pr_debug(b"unwind: put_unwind_info called\n\0".as_ptr() as *const c_char);
}

#[cfg(HAVE_LIBUNWIND_AARCH64_SUPPORT)]
unsafe extern "C" fn get_dyn_info_list_addr(
    as_: unw_addr_space_t,
    dil_addr: *mut unw_word_t,
    arg: *mut c_void,
) -> c_int {
    let _ = as_;
    let _ = dil_addr;
    let _ = arg;
    -UNW_ENOINFO
}

#[cfg(HAVE_LIBUNWIND_AARCH64_SUPPORT)]
unsafe extern "C" fn access_mem(
    as_: unw_addr_space_t,
    addr: unw_word_t,
    valp: *mut unw_word_t,
    __write: c_int,
    arg: *mut c_void,
) -> c_int {
    __libunwind__access_mem(as_, addr, valp, __write, arg)
}

#[cfg(HAVE_LIBUNWIND_AARCH64_SUPPORT)]
unsafe extern "C" fn access_reg(
    as_: unw_addr_space_t,
    regnum: unw_regnum_t,
    valp: *mut unw_word_t,
    __write: c_int,
    arg: *mut c_void,
) -> c_int {
    __libunwind__access_reg(as_, regnum, valp, __write, arg)
}

#[cfg(HAVE_LIBUNWIND_AARCH64_SUPPORT)]
unsafe extern "C" fn access_fpreg(
    as_: unw_addr_space_t,
    num: unw_regnum_t,
    val: *mut unw_fpreg_t,
    __write: c_int,
    arg: *mut c_void,
) -> c_int {
    let _ = as_;
    let _ = num;
    let _ = val;
    let _ = __write;
    let _ = arg;
    pr_err(b"unwind: access_fpreg unsupported\n\0".as_ptr() as *const c_char);
    -UNW_EINVAL
}

#[cfg(HAVE_LIBUNWIND_AARCH64_SUPPORT)]
unsafe extern "C" fn resume(
    as_: unw_addr_space_t,
    cu: *mut unw_cursor_t,
    arg: *mut c_void,
) -> c_int {
    let _ = as_;
    let _ = cu;
    let _ = arg;
    pr_err(b"unwind: resume unsupported\n\0".as_ptr() as *const c_char);
    -UNW_EINVAL
}

#[cfg(HAVE_LIBUNWIND_AARCH64_SUPPORT)]
unsafe extern "C" fn get_proc_name(
    as_: unw_addr_space_t,
    addr: unw_word_t,
    bufp: *mut c_char,
    buf_len: usize,
    offp: *mut unw_word_t,
    arg: *mut c_void,
) -> c_int {
    let _ = as_;
    let _ = addr;
    let _ = bufp;
    let _ = buf_len;
    let _ = offp;
    let _ = arg;
    pr_err(b"unwind: get_proc_name unsupported\n\0".as_ptr() as *const c_char);
    -UNW_EINVAL
}

#[no_mangle]
pub unsafe extern "C" fn __libunwind_arch__create_addr_space_arm64() -> *mut c_void {
    #[cfg(HAVE_LIBUNWIND_AARCH64_SUPPORT)]
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
        let addr_space: unw_addr_space_t;

        addr_space = unw_create_addr_space(&mut ACCESSORS, 0);
        unw_set_caching_policy(addr_space, UNW_CACHE_GLOBAL);
        return addr_space;
    }

    #[cfg(not(HAVE_LIBUNWIND_AARCH64_SUPPORT))]
    {
        core::ptr::null_mut()
    }
}

#[cfg(HAVE_LIBUNWIND_ARM64_SUPPORT)]
extern "C" {
    // C used UNW_OBJ(dwarf_search_unwind_table) and then #defined dwarf_search_unwind_table.
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
pub unsafe extern "C" fn __libunwind_arch__dwarf_search_unwind_table_arm64(
    as_: *mut c_void,
    ip: u64,
    _di: *mut libarch_unwind__dyn_info,
    pi: *mut c_void,
    need_unwind_info: c_int,
    arg: *mut c_void,
) -> c_int {
    #[cfg(HAVE_LIBUNWIND_ARM64_SUPPORT)]
    {
        let mut di = unw_dyn_info_t {
            format: UNW_INFO_FORMAT_REMOTE_TABLE,
            start_ip: (*_di).start_ip,
            end_ip: (*_di).end_ip,
            u: unw_dyn_info_u {
                rti: unw_remote_table_info {
                    segbase: (*_di).segbase,
                    table_data: (*_di).table_data,
                    table_len: (*_di).table_len,
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

        (*_di).start_ip = di.start_ip;
        (*_di).end_ip = di.end_ip;
        (*_di).segbase = di.u.rti.segbase;
        (*_di).table_data = di.u.rti.table_data;
        (*_di).table_len = di.u.rti.table_len;
        return ret;
    }

    #[cfg(not(HAVE_LIBUNWIND_ARM64_SUPPORT))]
    {
        let _ = as_;
        let _ = ip;
        let _ = _di;
        let _ = pi;
        let _ = need_unwind_info;
        let _ = arg;
        -EINVAL
    }
}

#[cfg(all(HAVE_LIBUNWIND_ARM64_SUPPORT, not(NO_LIBUNWIND_DEBUG_FRAME_ARM64)))]
extern "C" {
    // C used UNW_OBJ(dwarf_find_debug_frame) and then #defined dwarf_find_debug_frame.
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
pub unsafe extern "C" fn __libunwind_arch__dwarf_find_debug_frame_arm64(
    found: c_int,
    _di: *mut libarch_unwind__dyn_info,
    ip: u64,
    segbase: u64,
    obj_name: *const c_char,
    start: u64,
    end: u64,
) -> c_int {
    #[cfg(all(HAVE_LIBUNWIND_ARM64_SUPPORT, not(NO_LIBUNWIND_DEBUG_FRAME_ARM64)))]
    {
        let mut di = unw_dyn_info_t {
            format: UNW_INFO_FORMAT_REMOTE_TABLE,
            start_ip: (*_di).start_ip,
            end_ip: (*_di).end_ip,
            u: unw_dyn_info_u {
                rti: unw_remote_table_info {
                    segbase: (*_di).segbase,
                    table_data: (*_di).table_data,
                    table_len: (*_di).table_len,
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

        (*_di).start_ip = di.start_ip;
        (*_di).end_ip = di.end_ip;
        (*_di).segbase = di.u.ti.segbase;
        (*_di).table_data = di.u.ti.table_data;
        (*_di).table_len = di.u.ti.table_len;
        return ret;
    }

    #[cfg(not(all(HAVE_LIBUNWIND_ARM64_SUPPORT, not(NO_LIBUNWIND_DEBUG_FRAME_ARM64))))]
    {
        let _ = found;
        let _ = _di;
        let _ = ip;
        let _ = segbase;
        let _ = obj_name;
        let _ = start;
        let _ = end;
        -EINVAL
    }
}

#[cfg(HAVE_LIBUNWIND_ARM64_SUPPORT)]
#[repr(C)]
struct arch_unwind_info {
    ui: unwind_info,
    _cursor: unw_cursor_t,
    _ips: [u64; 0],
}

#[no_mangle]
pub unsafe extern "C" fn __libunwind_arch_unwind_info__new_arm64(
    thread: *mut thread,
    sample: *mut perf_sample,
    max_stack: c_int,
    best_effort: bool,
    first_ip: u64,
) -> *mut unwind_info {
    #[cfg(HAVE_LIBUNWIND_ARM64_SUPPORT)]
    {
        let maps = thread__maps(thread);
        let addr_space = maps__addr_space(maps);
        let ui: *mut arch_unwind_info;
        let ret: c_int;

        if addr_space.is_null() {
            return core::ptr::null_mut();
        }

        ui = zalloc(
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
        (*ui).ui.e_machine = EM_AARCH64;
        (*ui).ui.best_effort = best_effort;

        ret = unw_init_remote(&mut (*ui)._cursor, addr_space as unw_addr_space_t, &mut (*ui).ui as *mut _ as *mut c_void);
        if ret != 0 {
            if !best_effort {
                pr_err(b"libunwind: %s\n\0".as_ptr() as *const c_char, unw_strerror(ret));
            }
            free(ui as *mut c_void);
            return core::ptr::null_mut();
        }

        return &mut (*ui).ui;
    }

    #[cfg(not(HAVE_LIBUNWIND_ARM64_SUPPORT))]
    {
        let _ = thread;
        let _ = sample;
        let _ = max_stack;
        let _ = best_effort;
        let _ = first_ip;
        core::ptr::null_mut()
    }
}

#[no_mangle]
pub unsafe extern "C" fn __libunwind_arch__unwind_step_arm64(ui: *mut unwind_info) -> c_int {
    #[cfg(HAVE_LIBUNWIND_ARM64_SUPPORT)]
    {
        let ret: c_int;

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
        return ret;
    }

    #[cfg(not(HAVE_LIBUNWIND_ARM64_SUPPORT))]
    {
        let _ = ui;
        -EINVAL
    }
}
