/* SPDX-License-Identifier: GPL-2.0 */

use std::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)]
pub struct machine {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct maps {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct perf_sample {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct thread {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct unwind_info {
    pub machine: *mut machine,
    pub thread: *mut thread,
    pub sample: *mut perf_sample,
    pub cursor: *mut c_void,
    pub ips: *mut u64,
    pub cur_ip: c_int,
    pub max_ips: c_int,
    pub unw_word_t_size: c_uint,
    pub e_machine: u16,
    pub best_effort: bool,
}

#[repr(C)]
pub struct libarch_unwind__dyn_info {
    pub start_ip: u64,
    pub end_ip: u64,
    pub segbase: u64,
    pub table_data: u64,
    pub table_len: u64,
}

#[repr(C)]
pub struct libarch_unwind__proc_info {
    _unused: [u8; 0],
}

unsafe extern "C" {
    pub fn __get_perf_regnum_for_unw_regnum_arm(unw_regnum: c_int) -> c_int;
    pub fn __get_perf_regnum_for_unw_regnum_arm64(unw_regnum: c_int) -> c_int;
    pub fn __get_perf_regnum_for_unw_regnum_loongarch(unw_regnum: c_int) -> c_int;
    pub fn __get_perf_regnum_for_unw_regnum_mips(unw_regnum: c_int) -> c_int;
    pub fn __get_perf_regnum_for_unw_regnum_ppc32(unw_regnum: c_int) -> c_int;
    pub fn __get_perf_regnum_for_unw_regnum_ppc64(unw_regnum: c_int) -> c_int;
    pub fn __get_perf_regnum_for_unw_regnum_riscv(unw_regnum: c_int) -> c_int;
    pub fn __get_perf_regnum_for_unw_regnum_s390(unw_regnum: c_int) -> c_int;
    pub fn __get_perf_regnum_for_unw_regnum_i386(unw_regnum: c_int) -> c_int;
    pub fn __get_perf_regnum_for_unw_regnum_x86_64(unw_regnum: c_int) -> c_int;
    pub fn get_perf_regnum_for_unw_regnum(e_machine: c_uint, unw_regnum: c_int) -> c_int;

    pub fn __libunwind_arch__flush_access_arm(maps: *mut maps);
    pub fn __libunwind_arch__flush_access_arm64(maps: *mut maps);
    pub fn __libunwind_arch__flush_access_loongarch(maps: *mut maps);
    pub fn __libunwind_arch__flush_access_mips(maps: *mut maps);
    pub fn __libunwind_arch__flush_access_ppc32(maps: *mut maps);
    pub fn __libunwind_arch__flush_access_ppc64(maps: *mut maps);
    pub fn __libunwind_arch__flush_access_riscv(maps: *mut maps);
    pub fn __libunwind_arch__flush_access_s390(maps: *mut maps);
    pub fn __libunwind_arch__flush_access_i386(maps: *mut maps);
    pub fn __libunwind_arch__flush_access_x86_64(maps: *mut maps);
    pub fn libunwind_arch__flush_access(maps: *mut maps);

    pub fn __libunwind_arch__finish_access_arm(maps: *mut maps);
    pub fn __libunwind_arch__finish_access_arm64(maps: *mut maps);
    pub fn __libunwind_arch__finish_access_loongarch(maps: *mut maps);
    pub fn __libunwind_arch__finish_access_mips(maps: *mut maps);
    pub fn __libunwind_arch__finish_access_ppc32(maps: *mut maps);
    pub fn __libunwind_arch__finish_access_ppc64(maps: *mut maps);
    pub fn __libunwind_arch__finish_access_riscv(maps: *mut maps);
    pub fn __libunwind_arch__finish_access_s390(maps: *mut maps);
    pub fn __libunwind_arch__finish_access_i386(maps: *mut maps);
    pub fn __libunwind_arch__finish_access_x86_64(maps: *mut maps);
    pub fn libunwind_arch__finish_access(maps: *mut maps);

    pub fn __libunwind_arch__create_addr_space_arm() -> *mut c_void;
    pub fn __libunwind_arch__create_addr_space_arm64() -> *mut c_void;
    pub fn __libunwind_arch__create_addr_space_loongarch() -> *mut c_void;
    pub fn __libunwind_arch__create_addr_space_mips() -> *mut c_void;
    pub fn __libunwind_arch__create_addr_space_ppc32() -> *mut c_void;
    pub fn __libunwind_arch__create_addr_space_ppc64() -> *mut c_void;
    pub fn __libunwind_arch__create_addr_space_riscv() -> *mut c_void;
    pub fn __libunwind_arch__create_addr_space_s390() -> *mut c_void;
    pub fn __libunwind_arch__create_addr_space_i386() -> *mut c_void;
    pub fn __libunwind_arch__create_addr_space_x86_64() -> *mut c_void;
    pub fn libunwind_arch__create_addr_space(e_machine: c_uint) -> *mut c_void;

    pub fn __libunwind__find_proc_info(
        as_: *mut c_void,
        ip: u64,
        pi: *mut c_void,
        need_unwind_info: c_int,
        arg: *mut c_void,
    ) -> c_int;
    pub fn __libunwind__access_mem(
        as_: *mut c_void,
        addr: u64,
        valp_word: *mut c_void,
        __write: c_int,
        arg: *mut c_void,
    ) -> c_int;
    pub fn __libunwind__access_reg(
        as_: *mut c_void,
        regnum: c_int,
        valp_word: *mut c_void,
        __write: c_int,
        arg: *mut c_void,
    ) -> c_int;

    pub fn __libunwind_arch__dwarf_search_unwind_table_arm(
        as_: *mut c_void,
        ip: u64,
        di: *mut libarch_unwind__dyn_info,
        pi: *mut c_void,
        need_unwind_info: c_int,
        arg: *mut c_void,
    ) -> c_int;
    pub fn __libunwind_arch__dwarf_search_unwind_table_arm64(
        as_: *mut c_void,
        ip: u64,
        di: *mut libarch_unwind__dyn_info,
        pi: *mut c_void,
        need_unwind_info: c_int,
        arg: *mut c_void,
    ) -> c_int;
    pub fn __libunwind_arch__dwarf_search_unwind_table_loongarch(
        as_: *mut c_void,
        ip: u64,
        di: *mut libarch_unwind__dyn_info,
        pi: *mut c_void,
        need_unwind_info: c_int,
        arg: *mut c_void,
    ) -> c_int;
    pub fn __libunwind_arch__dwarf_search_unwind_table_mips(
        as_: *mut c_void,
        ip: u64,
        di: *mut libarch_unwind__dyn_info,
        pi: *mut c_void,
        need_unwind_info: c_int,
        arg: *mut c_void,
    ) -> c_int;
    pub fn __libunwind_arch__dwarf_search_unwind_table_ppc32(
        as_: *mut c_void,
        ip: u64,
        di: *mut libarch_unwind__dyn_info,
        pi: *mut c_void,
        need_unwind_info: c_int,
        arg: *mut c_void,
    ) -> c_int;
    pub fn __libunwind_arch__dwarf_search_unwind_table_ppc64(
        as_: *mut c_void,
        ip: u64,
        di: *mut libarch_unwind__dyn_info,
        pi: *mut c_void,
        need_unwind_info: c_int,
        arg: *mut c_void,
    ) -> c_int;
    pub fn __libunwind_arch__dwarf_search_unwind_table_riscv(
        as_: *mut c_void,
        ip: u64,
        di: *mut libarch_unwind__dyn_info,
        pi: *mut c_void,
        need_unwind_info: c_int,
        arg: *mut c_void,
    ) -> c_int;
    pub fn __libunwind_arch__dwarf_search_unwind_table_s390(
        as_: *mut c_void,
        ip: u64,
        di: *mut libarch_unwind__dyn_info,
        pi: *mut c_void,
        need_unwind_info: c_int,
        arg: *mut c_void,
    ) -> c_int;
    pub fn __libunwind_arch__dwarf_search_unwind_table_i386(
        as_: *mut c_void,
        ip: u64,
        di: *mut libarch_unwind__dyn_info,
        pi: *mut c_void,
        need_unwind_info: c_int,
        arg: *mut c_void,
    ) -> c_int;
    pub fn __libunwind_arch__dwarf_search_unwind_table_x86_64(
        as_: *mut c_void,
        ip: u64,
        di: *mut libarch_unwind__dyn_info,
        pi: *mut c_void,
        need_unwind_info: c_int,
        arg: *mut c_void,
    ) -> c_int;
    pub fn libunwind_arch__dwarf_search_unwind_table(
        e_machine: c_uint,
        as_: *mut c_void,
        ip: u64,
        di: *mut libarch_unwind__dyn_info,
        pi: *mut c_void,
        need_unwind_info: c_int,
        arg: *mut c_void,
    ) -> c_int;

    pub fn __libunwind_arch__dwarf_find_debug_frame_arm(
        found: c_int,
        di_debug: *mut libarch_unwind__dyn_info,
        ip: u64,
        segbase: u64,
        obj_name: *const c_char,
        start: u64,
        end: u64,
    ) -> c_int;
    pub fn __libunwind_arch__dwarf_find_debug_frame_arm64(
        found: c_int,
        di_debug: *mut libarch_unwind__dyn_info,
        ip: u64,
        segbase: u64,
        obj_name: *const c_char,
        start: u64,
        end: u64,
    ) -> c_int;
    pub fn __libunwind_arch__dwarf_find_debug_frame_loongarch(
        found: c_int,
        di_debug: *mut libarch_unwind__dyn_info,
        ip: u64,
        segbase: u64,
        obj_name: *const c_char,
        start: u64,
        end: u64,
    ) -> c_int;
    pub fn __libunwind_arch__dwarf_find_debug_frame_mips(
        found: c_int,
        di_debug: *mut libarch_unwind__dyn_info,
        ip: u64,
        segbase: u64,
        obj_name: *const c_char,
        start: u64,
        end: u64,
    ) -> c_int;
    pub fn __libunwind_arch__dwarf_find_debug_frame_ppc32(
        found: c_int,
        di_debug: *mut libarch_unwind__dyn_info,
        ip: u64,
        segbase: u64,
        obj_name: *const c_char,
        start: u64,
        end: u64,
    ) -> c_int;
    pub fn __libunwind_arch__dwarf_find_debug_frame_ppc64(
        found: c_int,
        di_debug: *mut libarch_unwind__dyn_info,
        ip: u64,
        segbase: u64,
        obj_name: *const c_char,
        start: u64,
        end: u64,
    ) -> c_int;
    pub fn __libunwind_arch__dwarf_find_debug_frame_riscv(
        found: c_int,
        di_debug: *mut libarch_unwind__dyn_info,
        ip: u64,
        segbase: u64,
        obj_name: *const c_char,
        start: u64,
        end: u64,
    ) -> c_int;
    pub fn __libunwind_arch__dwarf_find_debug_frame_s390(
        found: c_int,
        di_debug: *mut libarch_unwind__dyn_info,
        ip: u64,
        segbase: u64,
        obj_name: *const c_char,
        start: u64,
        end: u64,
    ) -> c_int;
    pub fn __libunwind_arch__dwarf_find_debug_frame_i386(
        found: c_int,
        di_debug: *mut libarch_unwind__dyn_info,
        ip: u64,
        segbase: u64,
        obj_name: *const c_char,
        start: u64,
        end: u64,
    ) -> c_int;
    pub fn __libunwind_arch__dwarf_find_debug_frame_x86_64(
        found: c_int,
        di_debug: *mut libarch_unwind__dyn_info,
        ip: u64,
        segbase: u64,
        obj_name: *const c_char,
        start: u64,
        end: u64,
    ) -> c_int;
    pub fn libunwind_arch__dwarf_find_debug_frame(
        e_machine: c_uint,
        found: c_int,
        di_debug: *mut libarch_unwind__dyn_info,
        ip: u64,
        segbase: u64,
        obj_name: *const c_char,
        start: u64,
        end: u64,
    ) -> c_int;

    pub fn __libunwind_arch_unwind_info__new_arm(
        thread: *mut thread,
        sample: *mut perf_sample,
        max_stack: c_int,
        best_effort: bool,
        first_ip: u64,
    ) -> *mut unwind_info;
    pub fn __libunwind_arch_unwind_info__new_arm64(
        thread: *mut thread,
        sample: *mut perf_sample,
        max_stack: c_int,
        best_effort: bool,
        first_ip: u64,
    ) -> *mut unwind_info;
    pub fn __libunwind_arch_unwind_info__new_loongarch(
        thread: *mut thread,
        sample: *mut perf_sample,
        max_stack: c_int,
        best_effort: bool,
        first_ip: u64,
    ) -> *mut unwind_info;
    pub fn __libunwind_arch_unwind_info__new_mips(
        thread: *mut thread,
        sample: *mut perf_sample,
        max_stack: c_int,
        best_effort: bool,
        first_ip: u64,
    ) -> *mut unwind_info;
    pub fn __libunwind_arch_unwind_info__new_ppc32(
        thread: *mut thread,
        sample: *mut perf_sample,
        max_stack: c_int,
        best_effort: bool,
        first_ip: u64,
    ) -> *mut unwind_info;
    pub fn __libunwind_arch_unwind_info__new_ppc64(
        thread: *mut thread,
        sample: *mut perf_sample,
        max_stack: c_int,
        best_effort: bool,
        first_ip: u64,
    ) -> *mut unwind_info;
    pub fn __libunwind_arch_unwind_info__new_riscv(
        thread: *mut thread,
        sample: *mut perf_sample,
        max_stack: c_int,
        best_effort: bool,
        first_ip: u64,
    ) -> *mut unwind_info;
    pub fn __libunwind_arch_unwind_info__new_s390(
        thread: *mut thread,
        sample: *mut perf_sample,
        max_stack: c_int,
        best_effort: bool,
        first_ip: u64,
    ) -> *mut unwind_info;
    pub fn __libunwind_arch_unwind_info__new_i386(
        thread: *mut thread,
        sample: *mut perf_sample,
        max_stack: c_int,
        best_effort: bool,
        first_ip: u64,
    ) -> *mut unwind_info;
    pub fn __libunwind_arch_unwind_info__new_x86_64(
        thread: *mut thread,
        sample: *mut perf_sample,
        max_stack: c_int,
        best_effort: bool,
        first_ip: u64,
    ) -> *mut unwind_info;
    pub fn libunwind_arch_unwind_info__new(
        thread: *mut thread,
        sample: *mut perf_sample,
        max_stack: c_int,
        best_effort: bool,
        e_machine: u16,
        first_ip: u64,
    ) -> *mut unwind_info;

    pub fn libunwind_arch_unwind_info__delete(ui: *mut unwind_info);

    pub fn __libunwind_arch__unwind_step_arm(ui: *mut unwind_info) -> c_int;
    pub fn __libunwind_arch__unwind_step_arm64(ui: *mut unwind_info) -> c_int;
    pub fn __libunwind_arch__unwind_step_loongarch(ui: *mut unwind_info) -> c_int;
    pub fn __libunwind_arch__unwind_step_mips(ui: *mut unwind_info) -> c_int;
    pub fn __libunwind_arch__unwind_step_ppc32(ui: *mut unwind_info) -> c_int;
    pub fn __libunwind_arch__unwind_step_ppc64(ui: *mut unwind_info) -> c_int;
    pub fn __libunwind_arch__unwind_step_riscv(ui: *mut unwind_info) -> c_int;
    pub fn __libunwind_arch__unwind_step_s390(ui: *mut unwind_info) -> c_int;
    pub fn __libunwind_arch__unwind_step_i386(ui: *mut unwind_info) -> c_int;
    pub fn __libunwind_arch__unwind_step_x86_64(ui: *mut unwind_info) -> c_int;
    pub fn libunwind_arch__unwind_step(ui: *mut unwind_info) -> c_int;
}
