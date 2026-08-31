// SPDX-License-Identifier: GPL-2.0
//
// Rust translation of perf/util/libunwind-arch/libunwind-arch.c.
// C includes intentionally remain external dependencies:
// "libunwind-arch.h", "../debug.h", "../maps.h", <elf.h>, <errno.h>.

use core::ffi::{c_char, c_int, c_uint, c_void};

const EM_NONE: c_uint = 0;
const EM_386: c_uint = 3;
const EM_MIPS: c_uint = 8;
const EM_PPC: c_uint = 20;
const EM_PPC64: c_uint = 21;
const EM_S390: c_uint = 22;
const EM_ARM: c_uint = 40;
const EM_X86_64: c_uint = 62;
const EM_AARCH64: c_uint = 183;
const EM_RISCV: c_uint = 243;
const EM_LOONGARCH: c_uint = 258;

const EINVAL: c_int = 22;

#[repr(C)]
pub struct maps {
    _private: [u8; 0],
}

#[repr(C)]
pub struct libarch_unwind__dyn_info {
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
    pub e_machine: u16,
}

unsafe extern "C" {
    fn pr_err(fmt: *const c_char, ...);
    fn maps__e_machine(maps: *mut maps) -> c_uint;
    fn free(ptr: *mut c_void);

    fn __get_perf_regnum_for_unw_regnum_arm(unw_regnum: c_int) -> c_int;
    fn __get_perf_regnum_for_unw_regnum_arm64(unw_regnum: c_int) -> c_int;
    fn __get_perf_regnum_for_unw_regnum_loongarch(unw_regnum: c_int) -> c_int;
    fn __get_perf_regnum_for_unw_regnum_mips(unw_regnum: c_int) -> c_int;
    fn __get_perf_regnum_for_unw_regnum_ppc32(unw_regnum: c_int) -> c_int;
    fn __get_perf_regnum_for_unw_regnum_ppc64(unw_regnum: c_int) -> c_int;
    fn __get_perf_regnum_for_unw_regnum_riscv(unw_regnum: c_int) -> c_int;
    fn __get_perf_regnum_for_unw_regnum_s390(unw_regnum: c_int) -> c_int;
    fn __get_perf_regnum_for_unw_regnum_i386(unw_regnum: c_int) -> c_int;
    fn __get_perf_regnum_for_unw_regnum_x86_64(unw_regnum: c_int) -> c_int;

    fn __libunwind_arch__flush_access_arm(maps: *mut maps);
    fn __libunwind_arch__flush_access_arm64(maps: *mut maps);
    fn __libunwind_arch__flush_access_loongarch(maps: *mut maps);
    fn __libunwind_arch__flush_access_mips(maps: *mut maps);
    fn __libunwind_arch__flush_access_ppc32(maps: *mut maps);
    fn __libunwind_arch__flush_access_ppc64(maps: *mut maps);
    fn __libunwind_arch__flush_access_riscv(maps: *mut maps);
    fn __libunwind_arch__flush_access_s390(maps: *mut maps);
    fn __libunwind_arch__flush_access_i386(maps: *mut maps);
    fn __libunwind_arch__flush_access_x86_64(maps: *mut maps);

    fn __libunwind_arch__finish_access_arm(maps: *mut maps);
    fn __libunwind_arch__finish_access_arm64(maps: *mut maps);
    fn __libunwind_arch__finish_access_loongarch(maps: *mut maps);
    fn __libunwind_arch__finish_access_mips(maps: *mut maps);
    fn __libunwind_arch__finish_access_ppc32(maps: *mut maps);
    fn __libunwind_arch__finish_access_ppc64(maps: *mut maps);
    fn __libunwind_arch__finish_access_riscv(maps: *mut maps);
    fn __libunwind_arch__finish_access_s390(maps: *mut maps);
    fn __libunwind_arch__finish_access_i386(maps: *mut maps);
    fn __libunwind_arch__finish_access_x86_64(maps: *mut maps);

    fn __libunwind_arch__create_addr_space_arm() -> *mut c_void;
    fn __libunwind_arch__create_addr_space_arm64() -> *mut c_void;
    fn __libunwind_arch__create_addr_space_loongarch() -> *mut c_void;
    fn __libunwind_arch__create_addr_space_mips() -> *mut c_void;
    fn __libunwind_arch__create_addr_space_ppc32() -> *mut c_void;
    fn __libunwind_arch__create_addr_space_ppc64() -> *mut c_void;
    fn __libunwind_arch__create_addr_space_riscv() -> *mut c_void;
    fn __libunwind_arch__create_addr_space_s390() -> *mut c_void;
    fn __libunwind_arch__create_addr_space_i386() -> *mut c_void;
    fn __libunwind_arch__create_addr_space_x86_64() -> *mut c_void;

    fn __libunwind_arch__dwarf_search_unwind_table_arm(
        as_: *mut c_void,
        ip: u64,
        di: *mut libarch_unwind__dyn_info,
        pi: *mut c_void,
        need_unwind_info: c_int,
        arg: *mut c_void,
    ) -> c_int;
    fn __libunwind_arch__dwarf_search_unwind_table_arm64(
        as_: *mut c_void,
        ip: u64,
        di: *mut libarch_unwind__dyn_info,
        pi: *mut c_void,
        need_unwind_info: c_int,
        arg: *mut c_void,
    ) -> c_int;
    fn __libunwind_arch__dwarf_search_unwind_table_loongarch(
        as_: *mut c_void,
        ip: u64,
        di: *mut libarch_unwind__dyn_info,
        pi: *mut c_void,
        need_unwind_info: c_int,
        arg: *mut c_void,
    ) -> c_int;
    fn __libunwind_arch__dwarf_search_unwind_table_mips(
        as_: *mut c_void,
        ip: u64,
        di: *mut libarch_unwind__dyn_info,
        pi: *mut c_void,
        need_unwind_info: c_int,
        arg: *mut c_void,
    ) -> c_int;
    fn __libunwind_arch__dwarf_search_unwind_table_ppc32(
        as_: *mut c_void,
        ip: u64,
        di: *mut libarch_unwind__dyn_info,
        pi: *mut c_void,
        need_unwind_info: c_int,
        arg: *mut c_void,
    ) -> c_int;
    fn __libunwind_arch__dwarf_search_unwind_table_ppc64(
        as_: *mut c_void,
        ip: u64,
        di: *mut libarch_unwind__dyn_info,
        pi: *mut c_void,
        need_unwind_info: c_int,
        arg: *mut c_void,
    ) -> c_int;
    fn __libunwind_arch__dwarf_search_unwind_table_riscv(
        as_: *mut c_void,
        ip: u64,
        di: *mut libarch_unwind__dyn_info,
        pi: *mut c_void,
        need_unwind_info: c_int,
        arg: *mut c_void,
    ) -> c_int;
    fn __libunwind_arch__dwarf_search_unwind_table_s390(
        as_: *mut c_void,
        ip: u64,
        di: *mut libarch_unwind__dyn_info,
        pi: *mut c_void,
        need_unwind_info: c_int,
        arg: *mut c_void,
    ) -> c_int;
    fn __libunwind_arch__dwarf_search_unwind_table_i386(
        as_: *mut c_void,
        ip: u64,
        di: *mut libarch_unwind__dyn_info,
        pi: *mut c_void,
        need_unwind_info: c_int,
        arg: *mut c_void,
    ) -> c_int;
    fn __libunwind_arch__dwarf_search_unwind_table_x86_64(
        as_: *mut c_void,
        ip: u64,
        di: *mut libarch_unwind__dyn_info,
        pi: *mut c_void,
        need_unwind_info: c_int,
        arg: *mut c_void,
    ) -> c_int;

    fn __libunwind_arch__dwarf_find_debug_frame_arm(
        found: c_int,
        di_debug: *mut libarch_unwind__dyn_info,
        ip: u64,
        segbase: u64,
        obj_name: *const c_char,
        start: u64,
        end: u64,
    ) -> c_int;
    fn __libunwind_arch__dwarf_find_debug_frame_arm64(
        found: c_int,
        di_debug: *mut libarch_unwind__dyn_info,
        ip: u64,
        segbase: u64,
        obj_name: *const c_char,
        start: u64,
        end: u64,
    ) -> c_int;
    fn __libunwind_arch__dwarf_find_debug_frame_loongarch(
        found: c_int,
        di_debug: *mut libarch_unwind__dyn_info,
        ip: u64,
        segbase: u64,
        obj_name: *const c_char,
        start: u64,
        end: u64,
    ) -> c_int;
    fn __libunwind_arch__dwarf_find_debug_frame_mips(
        found: c_int,
        di_debug: *mut libarch_unwind__dyn_info,
        ip: u64,
        segbase: u64,
        obj_name: *const c_char,
        start: u64,
        end: u64,
    ) -> c_int;
    fn __libunwind_arch__dwarf_find_debug_frame_ppc32(
        found: c_int,
        di_debug: *mut libarch_unwind__dyn_info,
        ip: u64,
        segbase: u64,
        obj_name: *const c_char,
        start: u64,
        end: u64,
    ) -> c_int;
    fn __libunwind_arch__dwarf_find_debug_frame_ppc64(
        found: c_int,
        di_debug: *mut libarch_unwind__dyn_info,
        ip: u64,
        segbase: u64,
        obj_name: *const c_char,
        start: u64,
        end: u64,
    ) -> c_int;
    fn __libunwind_arch__dwarf_find_debug_frame_riscv(
        found: c_int,
        di_debug: *mut libarch_unwind__dyn_info,
        ip: u64,
        segbase: u64,
        obj_name: *const c_char,
        start: u64,
        end: u64,
    ) -> c_int;
    fn __libunwind_arch__dwarf_find_debug_frame_s390(
        found: c_int,
        di_debug: *mut libarch_unwind__dyn_info,
        ip: u64,
        segbase: u64,
        obj_name: *const c_char,
        start: u64,
        end: u64,
    ) -> c_int;
    fn __libunwind_arch__dwarf_find_debug_frame_i386(
        found: c_int,
        di_debug: *mut libarch_unwind__dyn_info,
        ip: u64,
        segbase: u64,
        obj_name: *const c_char,
        start: u64,
        end: u64,
    ) -> c_int;
    fn __libunwind_arch__dwarf_find_debug_frame_x86_64(
        found: c_int,
        di_debug: *mut libarch_unwind__dyn_info,
        ip: u64,
        segbase: u64,
        obj_name: *const c_char,
        start: u64,
        end: u64,
    ) -> c_int;

    fn __libunwind_arch_unwind_info__new_arm(
        thread: *mut thread,
        sample: *mut perf_sample,
        max_stack: c_int,
        best_effort: bool,
        first_ip: u64,
    ) -> *mut unwind_info;
    fn __libunwind_arch_unwind_info__new_arm64(
        thread: *mut thread,
        sample: *mut perf_sample,
        max_stack: c_int,
        best_effort: bool,
        first_ip: u64,
    ) -> *mut unwind_info;
    fn __libunwind_arch_unwind_info__new_loongarch(
        thread: *mut thread,
        sample: *mut perf_sample,
        max_stack: c_int,
        best_effort: bool,
        first_ip: u64,
    ) -> *mut unwind_info;
    fn __libunwind_arch_unwind_info__new_mips(
        thread: *mut thread,
        sample: *mut perf_sample,
        max_stack: c_int,
        best_effort: bool,
        first_ip: u64,
    ) -> *mut unwind_info;
    fn __libunwind_arch_unwind_info__new_ppc32(
        thread: *mut thread,
        sample: *mut perf_sample,
        max_stack: c_int,
        best_effort: bool,
        first_ip: u64,
    ) -> *mut unwind_info;
    fn __libunwind_arch_unwind_info__new_ppc64(
        thread: *mut thread,
        sample: *mut perf_sample,
        max_stack: c_int,
        best_effort: bool,
        first_ip: u64,
    ) -> *mut unwind_info;
    fn __libunwind_arch_unwind_info__new_riscv(
        thread: *mut thread,
        sample: *mut perf_sample,
        max_stack: c_int,
        best_effort: bool,
        first_ip: u64,
    ) -> *mut unwind_info;
    fn __libunwind_arch_unwind_info__new_s390(
        thread: *mut thread,
        sample: *mut perf_sample,
        max_stack: c_int,
        best_effort: bool,
        first_ip: u64,
    ) -> *mut unwind_info;
    fn __libunwind_arch_unwind_info__new_i386(
        thread: *mut thread,
        sample: *mut perf_sample,
        max_stack: c_int,
        best_effort: bool,
        first_ip: u64,
    ) -> *mut unwind_info;
    fn __libunwind_arch_unwind_info__new_x86_64(
        thread: *mut thread,
        sample: *mut perf_sample,
        max_stack: c_int,
        best_effort: bool,
        first_ip: u64,
    ) -> *mut unwind_info;

    fn __libunwind_arch__unwind_step_arm(ui: *mut unwind_info) -> c_int;
    fn __libunwind_arch__unwind_step_arm64(ui: *mut unwind_info) -> c_int;
    fn __libunwind_arch__unwind_step_loongarch(ui: *mut unwind_info) -> c_int;
    fn __libunwind_arch__unwind_step_mips(ui: *mut unwind_info) -> c_int;
    fn __libunwind_arch__unwind_step_ppc32(ui: *mut unwind_info) -> c_int;
    fn __libunwind_arch__unwind_step_ppc64(ui: *mut unwind_info) -> c_int;
    fn __libunwind_arch__unwind_step_riscv(ui: *mut unwind_info) -> c_int;
    fn __libunwind_arch__unwind_step_s390(ui: *mut unwind_info) -> c_int;
    fn __libunwind_arch__unwind_step_i386(ui: *mut unwind_info) -> c_int;
    fn __libunwind_arch__unwind_step_x86_64(ui: *mut unwind_info) -> c_int;
}

const UNSUPPORTED_ELF_MACHINE_FMT: &[u8] = b"ELF MACHINE %x is not supported.\n\0";

#[no_mangle]
pub unsafe extern "C" fn get_perf_regnum_for_unw_regnum(
    e_machine: c_uint,
    unw_regnum: c_int,
) -> c_int {
    unsafe {
        match e_machine {
            EM_ARM => __get_perf_regnum_for_unw_regnum_arm(unw_regnum),
            EM_AARCH64 => __get_perf_regnum_for_unw_regnum_arm64(unw_regnum),
            EM_LOONGARCH => __get_perf_regnum_for_unw_regnum_loongarch(unw_regnum),
            EM_MIPS => __get_perf_regnum_for_unw_regnum_mips(unw_regnum),
            EM_PPC => __get_perf_regnum_for_unw_regnum_ppc32(unw_regnum),
            EM_PPC64 => __get_perf_regnum_for_unw_regnum_ppc64(unw_regnum),
            EM_RISCV => __get_perf_regnum_for_unw_regnum_riscv(unw_regnum),
            EM_S390 => __get_perf_regnum_for_unw_regnum_s390(unw_regnum),
            EM_386 => __get_perf_regnum_for_unw_regnum_i386(unw_regnum),
            EM_X86_64 => __get_perf_regnum_for_unw_regnum_x86_64(unw_regnum),
            _ => {
                pr_err(UNSUPPORTED_ELF_MACHINE_FMT.as_ptr() as *const c_char, e_machine);
                -EINVAL
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn libunwind_arch__flush_access(maps: *mut maps) {
    let e_machine = unsafe { maps__e_machine(maps) };

    unsafe {
        match e_machine {
            EM_NONE => {
                // No libunwind info on the maps.
            }
            EM_ARM => __libunwind_arch__flush_access_arm(maps),
            EM_AARCH64 => __libunwind_arch__flush_access_arm64(maps),
            EM_LOONGARCH => __libunwind_arch__flush_access_loongarch(maps),
            EM_MIPS => __libunwind_arch__flush_access_mips(maps),
            EM_PPC => __libunwind_arch__flush_access_ppc32(maps),
            EM_PPC64 => __libunwind_arch__flush_access_ppc64(maps),
            EM_RISCV => __libunwind_arch__flush_access_riscv(maps),
            EM_S390 => __libunwind_arch__flush_access_s390(maps),
            EM_386 => __libunwind_arch__flush_access_i386(maps),
            EM_X86_64 => __libunwind_arch__flush_access_x86_64(maps),
            _ => pr_err(UNSUPPORTED_ELF_MACHINE_FMT.as_ptr() as *const c_char, e_machine),
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn libunwind_arch__finish_access(maps: *mut maps) {
    let e_machine = unsafe { maps__e_machine(maps) };

    unsafe {
        match e_machine {
            EM_NONE => {
                // No libunwind info on the maps.
            }
            EM_ARM => __libunwind_arch__finish_access_arm(maps),
            EM_AARCH64 => __libunwind_arch__finish_access_arm64(maps),
            EM_LOONGARCH => __libunwind_arch__finish_access_loongarch(maps),
            EM_MIPS => __libunwind_arch__finish_access_mips(maps),
            EM_PPC => __libunwind_arch__finish_access_ppc32(maps),
            EM_PPC64 => __libunwind_arch__finish_access_ppc64(maps),
            EM_RISCV => __libunwind_arch__finish_access_riscv(maps),
            EM_S390 => __libunwind_arch__finish_access_s390(maps),
            EM_386 => __libunwind_arch__finish_access_i386(maps),
            EM_X86_64 => __libunwind_arch__finish_access_x86_64(maps),
            _ => pr_err(UNSUPPORTED_ELF_MACHINE_FMT.as_ptr() as *const c_char, e_machine),
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn libunwind_arch__create_addr_space(e_machine: c_uint) -> *mut c_void {
    unsafe {
        match e_machine {
            EM_ARM => __libunwind_arch__create_addr_space_arm(),
            EM_AARCH64 => __libunwind_arch__create_addr_space_arm64(),
            EM_LOONGARCH => __libunwind_arch__create_addr_space_loongarch(),
            EM_MIPS => __libunwind_arch__create_addr_space_mips(),
            EM_PPC => __libunwind_arch__create_addr_space_ppc32(),
            EM_PPC64 => __libunwind_arch__create_addr_space_ppc64(),
            EM_RISCV => __libunwind_arch__create_addr_space_riscv(),
            EM_S390 => __libunwind_arch__create_addr_space_s390(),
            EM_386 => __libunwind_arch__create_addr_space_i386(),
            EM_X86_64 => __libunwind_arch__create_addr_space_x86_64(),
            _ => {
                pr_err(UNSUPPORTED_ELF_MACHINE_FMT.as_ptr() as *const c_char, e_machine);
                core::ptr::null_mut()
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn libunwind_arch__dwarf_search_unwind_table(
    e_machine: c_uint,
    as_: *mut c_void,
    ip: u64,
    di: *mut libarch_unwind__dyn_info,
    pi: *mut c_void,
    need_unwind_info: c_int,
    arg: *mut c_void,
) -> c_int {
    unsafe {
        match e_machine {
            EM_ARM => __libunwind_arch__dwarf_search_unwind_table_arm(
                as_,
                ip,
                di,
                pi,
                need_unwind_info,
                arg,
            ),
            EM_AARCH64 => __libunwind_arch__dwarf_search_unwind_table_arm64(
                as_,
                ip,
                di,
                pi,
                need_unwind_info,
                arg,
            ),
            EM_LOONGARCH => __libunwind_arch__dwarf_search_unwind_table_loongarch(
                as_,
                ip,
                di,
                pi,
                need_unwind_info,
                arg,
            ),
            EM_MIPS => __libunwind_arch__dwarf_search_unwind_table_mips(
                as_,
                ip,
                di,
                pi,
                need_unwind_info,
                arg,
            ),
            EM_PPC => __libunwind_arch__dwarf_search_unwind_table_ppc32(
                as_,
                ip,
                di,
                pi,
                need_unwind_info,
                arg,
            ),
            EM_PPC64 => __libunwind_arch__dwarf_search_unwind_table_ppc64(
                as_,
                ip,
                di,
                pi,
                need_unwind_info,
                arg,
            ),
            EM_RISCV => __libunwind_arch__dwarf_search_unwind_table_riscv(
                as_,
                ip,
                di,
                pi,
                need_unwind_info,
                arg,
            ),
            EM_S390 => __libunwind_arch__dwarf_search_unwind_table_s390(
                as_,
                ip,
                di,
                pi,
                need_unwind_info,
                arg,
            ),
            EM_386 => __libunwind_arch__dwarf_search_unwind_table_i386(
                as_,
                ip,
                di,
                pi,
                need_unwind_info,
                arg,
            ),
            EM_X86_64 => __libunwind_arch__dwarf_search_unwind_table_x86_64(
                as_,
                ip,
                di,
                pi,
                need_unwind_info,
                arg,
            ),
            _ => {
                pr_err(UNSUPPORTED_ELF_MACHINE_FMT.as_ptr() as *const c_char, e_machine);
                -EINVAL
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn libunwind_arch__dwarf_find_debug_frame(
    e_machine: c_uint,
    found: c_int,
    di_debug: *mut libarch_unwind__dyn_info,
    ip: u64,
    segbase: u64,
    obj_name: *const c_char,
    start: u64,
    end: u64,
) -> c_int {
    unsafe {
        match e_machine {
            EM_ARM => __libunwind_arch__dwarf_find_debug_frame_arm(
                found, di_debug, ip, segbase, obj_name, start, end,
            ),
            EM_AARCH64 => __libunwind_arch__dwarf_find_debug_frame_arm64(
                found, di_debug, ip, segbase, obj_name, start, end,
            ),
            EM_LOONGARCH => __libunwind_arch__dwarf_find_debug_frame_loongarch(
                found, di_debug, ip, segbase, obj_name, start, end,
            ),
            EM_MIPS => __libunwind_arch__dwarf_find_debug_frame_mips(
                found, di_debug, ip, segbase, obj_name, start, end,
            ),
            EM_PPC => __libunwind_arch__dwarf_find_debug_frame_ppc32(
                found, di_debug, ip, segbase, obj_name, start, end,
            ),
            EM_PPC64 => __libunwind_arch__dwarf_find_debug_frame_ppc64(
                found, di_debug, ip, segbase, obj_name, start, end,
            ),
            EM_RISCV => __libunwind_arch__dwarf_find_debug_frame_riscv(
                found, di_debug, ip, segbase, obj_name, start, end,
            ),
            EM_S390 => __libunwind_arch__dwarf_find_debug_frame_s390(
                found, di_debug, ip, segbase, obj_name, start, end,
            ),
            EM_386 => __libunwind_arch__dwarf_find_debug_frame_i386(
                found, di_debug, ip, segbase, obj_name, start, end,
            ),
            EM_X86_64 => __libunwind_arch__dwarf_find_debug_frame_x86_64(
                found, di_debug, ip, segbase, obj_name, start, end,
            ),
            _ => {
                pr_err(UNSUPPORTED_ELF_MACHINE_FMT.as_ptr() as *const c_char, e_machine);
                -EINVAL
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn libunwind_arch_unwind_info__new(
    thread: *mut thread,
    sample: *mut perf_sample,
    max_stack: c_int,
    best_effort: bool,
    e_machine: u16,
    first_ip: u64,
) -> *mut unwind_info {
    unsafe {
        match e_machine as c_uint {
            EM_ARM => __libunwind_arch_unwind_info__new_arm(
                thread,
                sample,
                max_stack,
                best_effort,
                first_ip,
            ),
            EM_AARCH64 => __libunwind_arch_unwind_info__new_arm64(
                thread,
                sample,
                max_stack,
                best_effort,
                first_ip,
            ),
            EM_LOONGARCH => __libunwind_arch_unwind_info__new_loongarch(
                thread,
                sample,
                max_stack,
                best_effort,
                first_ip,
            ),
            EM_MIPS => __libunwind_arch_unwind_info__new_mips(
                thread,
                sample,
                max_stack,
                best_effort,
                first_ip,
            ),
            EM_PPC => __libunwind_arch_unwind_info__new_ppc32(
                thread,
                sample,
                max_stack,
                best_effort,
                first_ip,
            ),
            EM_PPC64 => __libunwind_arch_unwind_info__new_ppc64(
                thread,
                sample,
                max_stack,
                best_effort,
                first_ip,
            ),
            EM_RISCV => __libunwind_arch_unwind_info__new_riscv(
                thread,
                sample,
                max_stack,
                best_effort,
                first_ip,
            ),
            EM_S390 => __libunwind_arch_unwind_info__new_s390(
                thread,
                sample,
                max_stack,
                best_effort,
                first_ip,
            ),
            EM_386 => __libunwind_arch_unwind_info__new_i386(
                thread,
                sample,
                max_stack,
                best_effort,
                first_ip,
            ),
            EM_X86_64 => __libunwind_arch_unwind_info__new_x86_64(
                thread,
                sample,
                max_stack,
                best_effort,
                first_ip,
            ),
            _ => {
                pr_err(
                    UNSUPPORTED_ELF_MACHINE_FMT.as_ptr() as *const c_char,
                    e_machine as c_uint,
                );
                core::ptr::null_mut()
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn libunwind_arch_unwind_info__delete(ui: *mut unwind_info) {
    unsafe {
        free(ui as *mut c_void);
    }
}

#[no_mangle]
pub unsafe extern "C" fn libunwind_arch__unwind_step(ui: *mut unwind_info) -> c_int {
    let e_machine = unsafe { (*ui).e_machine };

    unsafe {
        match e_machine as c_uint {
            EM_ARM => __libunwind_arch__unwind_step_arm(ui),
            EM_AARCH64 => __libunwind_arch__unwind_step_arm64(ui),
            EM_LOONGARCH => __libunwind_arch__unwind_step_loongarch(ui),
            EM_MIPS => __libunwind_arch__unwind_step_mips(ui),
            EM_PPC => __libunwind_arch__unwind_step_ppc32(ui),
            EM_PPC64 => __libunwind_arch__unwind_step_ppc64(ui),
            EM_RISCV => __libunwind_arch__unwind_step_riscv(ui),
            EM_S390 => __libunwind_arch__unwind_step_s390(ui),
            EM_386 => __libunwind_arch__unwind_step_i386(ui),
            EM_X86_64 => __libunwind_arch__unwind_step_x86_64(ui),
            _ => {
                pr_err(
                    UNSUPPORTED_ELF_MACHINE_FMT.as_ptr() as *const c_char,
                    e_machine as c_uint,
                );
                -EINVAL
            }
        }
    }
}
