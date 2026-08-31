/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::{c_int, c_uint, c_void};
use core::mem;
use core::ptr;

// Dependencies from: sample.h, elf.h, linux/zalloc.h, asm/insn.h,
// debug.h, evsel.h, and thread.h.

pub const EM_386: u16 = 3;
pub const EM_68K: u16 = 4;
pub const EM_MIPS: u16 = 8;
pub const EM_PARISC: u16 = 15;
pub const EM_SPARC: u16 = 2;
pub const EM_PPC: u16 = 20;
pub const EM_PPC64: u16 = 21;
pub const EM_ARM: u16 = 40;
pub const EM_SH: u16 = 42;
pub const EM_SPARCV9: u16 = 43;
pub const EM_X86_64: u16 = 62;
pub const EM_ALPHA: u16 = 0x9026;
pub const EM_ARC: u16 = 93;
pub const EM_XTENSA: u16 = 94;
pub const EM_AARCH64: u16 = 183;
pub const EM_RISCV: u16 = 243;

// Local fallbacks from the C preprocessor guards.
pub const EM_CSKY: u16 = 252;
pub const EM_LOONGARCH: u16 = 258;

pub const INSN_MODE_32: c_int = 0;
pub const INSN_MODE_64: c_int = 1;

unsafe extern "C" {
    static MAX_INSN: c_int;

    fn evsel__put(evsel: *mut evsel);
    fn zalloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn pr_err(fmt: *const i8, ...);
    fn thread__e_machine(thread: *mut thread, machine: *mut machine, e_flags: *mut c_uint) -> u16;
    fn thread__memcpy(
        thread: *mut thread,
        machine: *mut machine,
        buf: *mut u8,
        ip: u64,
        len: c_int,
        is64bit: *mut bool,
    ) -> c_int;
    fn insn_decode(insn: *mut insn, buf: *const u8, len: c_int, mode: c_int) -> c_int;
}

#[repr(C)]
pub struct evsel {
    _private: [u8; 0],
}

#[repr(C)]
pub struct thread {
    _private: [u8; 0],
}

#[repr(C)]
pub struct machine {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regs_dump {
    _private: [u8; 0],
}

#[repr(C)]
pub struct callchain_cursor {
    _private: [u8; 0],
}

#[repr(C)]
pub struct insn {
    pub length: c_int,
}

#[repr(C)]
pub struct perf_sample {
    pub evsel: *mut evsel,
    pub user_regs: *mut regs_dump,
    pub intr_regs: *mut regs_dump,
    pub merged_callchain: bool,
    pub callchain: *mut callchain_cursor,
    pub ip: u64,
    pub insn_len: c_int,
    pub insn: *mut u8,
}

unsafe fn zfree<T>(ptr_to_ptr: *mut *mut T) {
    if !(*ptr_to_ptr).is_null() {
        free(*ptr_to_ptr as *mut c_void);
        *ptr_to_ptr = ptr::null_mut();
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_sample__init(sample: *mut perf_sample, all: bool) {
    if all {
        ptr::write_bytes(sample as *mut u8, 0, mem::size_of::<perf_sample>());
    } else {
        (*sample).evsel = ptr::null_mut();
        (*sample).user_regs = ptr::null_mut();
        (*sample).intr_regs = ptr::null_mut();
        (*sample).merged_callchain = false;
        (*sample).callchain = ptr::null_mut();
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_sample__exit(sample: *mut perf_sample) {
    evsel__put((*sample).evsel);
    (*sample).evsel = ptr::null_mut();
    zfree(&mut (*sample).user_regs);
    zfree(&mut (*sample).intr_regs);
    if (*sample).merged_callchain {
        zfree(&mut (*sample).callchain);
        (*sample).merged_callchain = false;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_sample__user_regs(sample: *mut perf_sample) -> *mut regs_dump {
    if (*sample).user_regs.is_null() {
        (*sample).user_regs = zalloc(mem::size_of::<regs_dump>()) as *mut regs_dump;
        if (*sample).user_regs.is_null() {
            pr_err(c"Failure to allocate sample user_regs".as_ptr());
        }
    }
    (*sample).user_regs
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_sample__intr_regs(sample: *mut perf_sample) -> *mut regs_dump {
    if (*sample).intr_regs.is_null() {
        (*sample).intr_regs = zalloc(mem::size_of::<regs_dump>()) as *mut regs_dump;
        if (*sample).intr_regs.is_null() {
            pr_err(c"Failure to allocate sample intr_regs".as_ptr());
        }
    }
    (*sample).intr_regs
}

unsafe fn elf_machine_max_instruction_length(e_machine: u16) -> c_int {
    match e_machine {
        // Fixed 4-byte (32-bit) architectures
        EM_AARCH64
        | EM_PPC
        | EM_PPC64
        | EM_MIPS
        | EM_SPARC
        | EM_SPARCV9
        | EM_ALPHA
        | EM_LOONGARCH
        | EM_PARISC
        | EM_SH => 4,

        // Variable length or mixed-mode architectures
        EM_ARM => 4,   // Variable due to Thumb/Thumb-2
        EM_RISCV => 4, // Variable due to Compressed (C) extension
        EM_CSKY => 4,  // Variable (16 or 32 bit)
        EM_ARC => 4,   // Variable (ARCompact)
        EM_S390 => 6,  // Variable (2, 4, or 6 bytes)
        EM_68K => 10,
        EM_386 | EM_X86_64 => 15,
        EM_XTENSA => 16, // Variable (FLIX)
        _ => MAX_INSN,
    }
}

pub const EM_S390: u16 = 22;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_sample__fetch_insn(
    sample: *mut perf_sample,
    thread: *mut thread,
    machine: *mut machine,
) {
    let ret: c_int;
    let mut len: c_int;
    let mut is64bit = false;
    let e_machine: u16;

    if (*sample).ip == 0 || (*sample).insn_len != 0 {
        return;
    }

    e_machine = thread__e_machine(thread, machine, ptr::null_mut());
    len = elf_machine_max_instruction_length(e_machine);
    len = thread__memcpy(
        thread,
        machine,
        (*sample).insn,
        (*sample).ip,
        len,
        &mut is64bit,
    );
    if len <= 0 {
        return;
    }

    (*sample).insn_len = len;

    if e_machine == EM_386 || e_machine == EM_X86_64 {
        // Refine the x86 instruction length with the decoder.
        let mut insn: insn = mem::zeroed();

        ret = insn_decode(
            &mut insn,
            (*sample).insn,
            len,
            if is64bit { INSN_MODE_64 } else { INSN_MODE_32 },
        );
        if ret >= 0 && insn.length <= len {
            (*sample).insn_len = insn.length;
        }
    }
}
