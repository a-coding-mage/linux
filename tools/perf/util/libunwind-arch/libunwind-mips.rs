// SPDX-License-Identifier: GPL-2.0
// Translated from perf/util/libunwind-arch/libunwind-mips.c.
// C includes intentionally omitted; the referenced types, constants, and
// functions are supplied by the surrounding repository bindings.

use core::ffi::{c_char, c_int, c_void};

#[no_mangle]
pub unsafe extern "C" fn __get_perf_regnum_for_unw_regnum_mips(
    unw_regnum: c_int,
) -> c_int {
    // #ifndef HAVE_LIBUNWIND_MIPS_SUPPORT
    #[cfg(not(HAVE_LIBUNWIND_MIPS_SUPPORT))]
    {
        return -EINVAL;
    }

    // #ifdef HAVE_LIBUNWIND_MIPS_SUPPORT
    #[cfg(HAVE_LIBUNWIND_MIPS_SUPPORT)]
    {
        match unw_regnum {
            r if r >= UNW_MIPS_R1 && r <= UNW_MIPS_R25 => {
                return unw_regnum - UNW_MIPS_R1 + PERF_REG_MIPS_R1;
            }
            r if r >= UNW_MIPS_R28 && r <= UNW_MIPS_R31 => {
                return unw_regnum - UNW_MIPS_R28 + PERF_REG_MIPS_R28;
            }
            UNW_MIPS_PC => {
                return PERF_REG_MIPS_PC;
            }
            _ => {
                pr_err(
                    b"unwind: invalid reg id %d\n\0".as_ptr() as *const c_char,
                    unw_regnum,
                );
                return -EINVAL;
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn __libunwind_arch__flush_access_mips(maps: *mut maps) {
    // #ifdef HAVE_LIBUNWIND_MIPS_SUPPORT
    #[cfg(HAVE_LIBUNWIND_MIPS_SUPPORT)]
    {
        unw_flush_cache(maps__addr_space(maps), 0, 0);
    }
}

#[no_mangle]
pub unsafe extern "C" fn __libunwind_arch__finish_access_mips(maps: *mut maps) {
    // #ifdef HAVE_LIBUNWIND_MIPS_SUPPORT
    #[cfg(HAVE_LIBUNWIND_MIPS_SUPPORT)]
    {
        unw_destroy_addr_space(maps__addr_space(maps));
    }
}

// #ifdef HAVE_LIBUNWIND_MIPS_SUPPORT
#[cfg(HAVE_LIBUNWIND_MIPS_SUPPORT)]
unsafe extern "C" fn find_proc_info(
    as_: unw_addr_space_t,
    ip: unw_word_t,
    pi: *mut unw_proc_info_t,
    need_unwind_info: c_int,
    arg: *mut c_void,
) -> c_int {
    __libunwind__find_proc_info(as_, ip, pi, need_unwind_info, arg)
}

#[cfg(HAVE_LIBUNWIND_MIPS_SUPPORT)]
unsafe extern "C" fn put_unwind_info(
    _as: unw_addr_space_t,
    _pi: *mut unw_proc_info_t,
    _arg: *mut c_void,
) {
    pr_debug(b"unwind: put_unwind_info called\n\0".as_ptr() as *const c_char);
}

#[cfg(HAVE_LIBUNWIND_MIPS_SUPPORT)]
unsafe extern "C" fn get_dyn_info_list_addr(
    _as: unw_addr_space_t,
    _dil_addr: *mut unw_word_t,
    _arg: *mut c_void,
) -> c_int {
    -UNW_ENOINFO
}

#[cfg(HAVE_LIBUNWIND_MIPS_SUPPORT)]
unsafe extern "C" fn access_mem(
    as_: unw_addr_space_t,
    addr: unw_word_t,
    valp: *mut unw_word_t,
    __write: c_int,
    arg: *mut c_void,
) -> c_int {
    __libunwind__access_mem(as_, addr, valp, __write, arg)
}

#[cfg(HAVE_LIBUNWIND_MIPS_SUPPORT)]
unsafe extern "C" fn access_reg(
    as_: unw_addr_space_t,
    regnum: unw_regnum_t,
    valp: *mut unw_word_t,
    __write: c_int,
    arg: *mut c_void,
) -> c_int {
    __libunwind__access_reg(as_, regnum, valp, __write, arg)
}

#[cfg(HAVE_LIBUNWIND_MIPS_SUPPORT)]
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

#[cfg(HAVE_LIBUNWIND_MIPS_SUPPORT)]
unsafe extern "C" fn resume(
    _as: unw_addr_space_t,
    _cu: *mut unw_cursor_t,
    _arg: *mut c_void,
) -> c_int {
    pr_err(b"unwind: resume unsupported\n\0".as_ptr() as *const c_char);
    -UNW_EINVAL
}

#[cfg(HAVE_LIBUNWIND_MIPS_SUPPORT)]
unsafe extern "C" fn get_proc_name(
    _as: unw_addr_space_t,
    _addr: unw_word_t,
    _bufp: *mut c_char,
    _buf_len: usize,
    _offp: *mut unw_word_t,
    _arg: *mut c_void,
) -> c_int {
    pr_err(b"unwind: get_proc_name unsupported\n\0".as_ptr() as *const c_char);
    -UNW_EINVAL
}

#[no_mangle]
pub unsafe extern "C" fn __libunwind_arch__create_addr_space_mips() -> *mut c_void {
    // #ifdef HAVE_LIBUNWIND_MIPS_SUPPORT
    #[cfg(HAVE_LIBUNWIND_MIPS_SUPPORT)]
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
        return addr_space as *mut c_void;
    }

    // #else
    #[cfg(not(HAVE_LIBUNWIND_MIPS_SUPPORT))]
    {
        return core::ptr::null_mut();
    }
}

// #ifdef HAVE_LIBUNWIND_MIPS_SUPPORT
// C used UNW_OBJ(dwarf_search_unwind_table) and then macro-renamed it.
#[cfg(HAVE_LIBUNWIND_MIPS_SUPPORT)]
extern "C" {
    fn dwarf_search_unwind_table(
        as_: unw_addr_space_t,
        ip: unw_word_t,
        di: *mut unw_dyn_info_t,
        pi: *mut unw_proc_info_t,
        need_unwind_info: c_int,
        arg: *mut c_void,
    ) -> c_int;
}

#[no_mangle]
pub unsafe extern "C" fn __libunwind_arch__dwarf_search_unwind_table_mips(
    as_: *mut c_void,
    ip: u64,
    _di: *mut libarch_unwind__dyn_info,
    pi: *mut c_void,
    need_unwind_info: c_int,
    arg: *mut c_void,
) -> c_int {
    // #ifdef HAVE_LIBUNWIND_MIPS_SUPPORT
    #[cfg(HAVE_LIBUNWIND_MIPS_SUPPORT)]
    {
        let mut di = unw_dyn_info_t {
            format: UNW_INFO_FORMAT_REMOTE_TABLE,
            start_ip: (*_di).start_ip,
            end_ip: (*_di).end_ip,
            u: unw_dyn_info_t__bindgen_ty_1 {
                rti: unw_dyn_remote_table_info {
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
    #[cfg(not(HAVE_LIBUNWIND_MIPS_SUPPORT))]
    {
        return -EINVAL;
    }
}

// #if defined(HAVE_LIBUNWIND_MIPS_SUPPORT) && !defined(NO_LIBUNWIND_DEBUG_FRAME_MIPS)
// C used UNW_OBJ(dwarf_find_debug_frame) and then macro-renamed it.
#[cfg(all(HAVE_LIBUNWIND_MIPS_SUPPORT, not(NO_LIBUNWIND_DEBUG_FRAME_MIPS)))]
extern "C" {
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
pub unsafe extern "C" fn __libunwind_arch__dwarf_find_debug_frame_mips(
    found: c_int,
    _di: *mut libarch_unwind__dyn_info,
    ip: u64,
    segbase: u64,
    obj_name: *const c_char,
    start: u64,
    end: u64,
) -> c_int {
    // #if defined(HAVE_LIBUNWIND_MIPS_SUPPORT) && !defined(NO_LIBUNWIND_DEBUG_FRAME_MIPS)
    #[cfg(all(HAVE_LIBUNWIND_MIPS_SUPPORT, not(NO_LIBUNWIND_DEBUG_FRAME_MIPS)))]
    {
        let mut di = unw_dyn_info_t {
            format: UNW_INFO_FORMAT_REMOTE_TABLE,
            start_ip: (*_di).start_ip,
            end_ip: (*_di).end_ip,
            u: unw_dyn_info_t__bindgen_ty_1 {
                rti: unw_dyn_remote_table_info {
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

    // #else
    #[cfg(not(all(HAVE_LIBUNWIND_MIPS_SUPPORT, not(NO_LIBUNWIND_DEBUG_FRAME_MIPS))))]
    {
        return -EINVAL;
    }
}

#[cfg(HAVE_LIBUNWIND_MIPS_SUPPORT)]
#[repr(C)]
struct arch_unwind_info {
    ui: unwind_info,
    _cursor: unw_cursor_t,
    _ips: [u64; 0],
}

#[no_mangle]
pub unsafe extern "C" fn __libunwind_arch_unwind_info__new_mips(
    thread: *mut thread,
    sample: *mut perf_sample,
    max_stack: c_int,
    best_effort: bool,
    first_ip: u64,
) -> *mut unwind_info {
    // #ifdef HAVE_LIBUNWIND_MIPS_SUPPORT
    #[cfg(HAVE_LIBUNWIND_MIPS_SUPPORT)]
    {
        let maps: *mut maps = thread__maps(thread);
        let addr_space: *mut c_void = maps__addr_space(maps);
        let mut ui: *mut arch_unwind_info;
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
        (*ui).ui.e_machine = EM_MIPS;
        (*ui).ui.best_effort = best_effort;

        ret = unw_init_remote(&mut (*ui)._cursor, addr_space as unw_addr_space_t, &mut (*ui).ui);
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

    // #else
    #[cfg(not(HAVE_LIBUNWIND_MIPS_SUPPORT))]
    {
        return core::ptr::null_mut();
    }
}

#[no_mangle]
pub unsafe extern "C" fn __libunwind_arch__unwind_step_mips(ui: *mut unwind_info) -> c_int {
    // #ifdef HAVE_LIBUNWIND_MIPS_SUPPORT
    #[cfg(HAVE_LIBUNWIND_MIPS_SUPPORT)]
    {
        let ret: c_int;

        if (*ui).cur_ip >= (*ui).max_ips {
            return 0;
        }

        ret = unw_step((*ui).cursor);
        if ret > 0 {
            let mut ip: u64 = 0;

            unw_get_reg((*ui).cursor, UNW_REG_IP, &mut ip as *mut u64 as *mut unw_word_t);

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

    // #else
    #[cfg(not(HAVE_LIBUNWIND_MIPS_SUPPORT))]
    {
        return -EINVAL;
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
