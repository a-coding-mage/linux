// SPDX-License-Identifier: GPL-2.0-or-later

// Translated from C implementation source. C includes:
// <stdio.h>, <stdlib.h>, <objtool/check.h>, <objtool/disas.h>,
// <objtool/elf.h>, <objtool/arch.h>, <objtool/warn.h>, <objtool/builtin.h>

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

pub type bool_ = bool;
pub type s64 = i64;
pub type u8 = u8;
pub type u32 = u32;

#[repr(C)]
pub struct elf {
    _private: [u8; 0],
}

#[repr(C)]
pub struct objtool_file {
    pub elf: *mut elf,
}

#[repr(C)]
pub struct data {
    pub d_buf: *mut c_void,
}

#[repr(C)]
pub struct section {
    pub data: *mut data,
}

#[repr(C)]
pub struct reloc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct instruction {
    pub len: u8,
    pub type_: insn_type,
    pub immediate: c_ulong,
    pub offset: c_ulong,
}

#[repr(C)]
pub struct cfi_reg {
    pub base: c_int,
    pub offset: c_int,
}

#[repr(C)]
pub struct cfi_init_state {
    pub regs: [cfi_reg; CFI_NUM_REGS],
    pub cfa: cfi_reg,
}

#[repr(C)]
pub struct disassemble_info {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum insn_type {
    INSN_OTHER = 0,
    INSN_CALL,
    INSN_JUMP_UNCONDITIONAL,
}

pub const CFI_NUM_REGS: usize = 33;
pub const CFI_UNDEFINED: c_int = -1;
pub const CFI_SP: c_int = 1;
pub const CFI_RA: usize = 32;
pub const CFI_CFA: c_int = -2;

pub const R_PPC_REL32: c_int = 26;
pub const R_PPC_ADDR32: c_int = 1;
pub const R_PPC_UADDR32: c_int = 24;
pub const R_PPC_PLT32: c_int = 27;
pub const R_PPC_PLTREL32: c_int = 32;

pub const bfd_arch_powerpc: c_int = 20;
pub const bfd_mach_ppc: c_ulong = 32;
pub const bfd_mach_ppc64: c_ulong = 64;

unsafe extern "C" {
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn exit(status: c_int) -> !;
    fn reloc_addend(reloc: *mut reloc) -> s64;
    fn reloc_type(reloc: *mut reloc) -> c_int;
    fn bswap_if_needed(elf: *mut elf, val: u32) -> u32;
    fn disas_info_init(
        dinfo: *mut disassemble_info,
        arch: c_int,
        mach32: c_ulong,
        mach64: c_ulong,
        endian: *mut c_void,
    ) -> c_int;
}

#[unsafe(no_mangle)]
pub static arch_reg_name: [*const c_char; CFI_NUM_REGS] = [
    c"r0".as_ptr(),
    c"sp".as_ptr(),
    c"r2".as_ptr(),
    c"r3".as_ptr(),
    c"r4".as_ptr(),
    c"r5".as_ptr(),
    c"r6".as_ptr(),
    c"r7".as_ptr(),
    c"r8".as_ptr(),
    c"r9".as_ptr(),
    c"r10".as_ptr(),
    c"r11".as_ptr(),
    c"r12".as_ptr(),
    c"r13".as_ptr(),
    c"r14".as_ptr(),
    c"r15".as_ptr(),
    c"r16".as_ptr(),
    c"r17".as_ptr(),
    c"r18".as_ptr(),
    c"r19".as_ptr(),
    c"r20".as_ptr(),
    c"r21".as_ptr(),
    c"r22".as_ptr(),
    c"r23".as_ptr(),
    c"r24".as_ptr(),
    c"r25".as_ptr(),
    c"r26".as_ptr(),
    c"r27".as_ptr(),
    c"r28".as_ptr(),
    c"r29".as_ptr(),
    c"r30".as_ptr(),
    c"r31".as_ptr(),
    c"ra".as_ptr(),
];

#[unsafe(no_mangle)]
pub unsafe extern "C" fn arch_ftrace_match(name: *const c_char) -> c_int {
    unsafe { (strcmp(name, c"_mcount".as_ptr()) == 0) as c_int }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn arch_insn_adjusted_addend(
    _insn: *mut instruction,
    reloc: *mut reloc,
) -> s64 {
    unsafe { reloc_addend(reloc) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn arch_callee_saved_reg(_reg: u8) -> bool {
    false
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn arch_decode_hint_reg(_sp_reg: u8, _base: *mut c_int) -> c_int {
    unsafe { exit(-1) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn arch_nop_insn(_len: c_int) -> *const c_char {
    unsafe { exit(-1) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn arch_ret_insn(_len: c_int) -> *const c_char {
    unsafe { exit(-1) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn arch_decode_instruction(
    file: *mut objtool_file,
    sec: *const section,
    offset: c_ulong,
    _maxlen: c_uint,
    insn: *mut instruction,
) -> c_int {
    let mut opcode: c_uint;
    let mut typ: insn_type;
    let mut imm: c_ulong;
    let ins: u32;

    ins = unsafe {
        bswap_if_needed(
            (*file).elf,
            *((*(*sec).data).d_buf.cast::<u8>().add(offset as usize).cast::<u32>()),
        )
    };
    opcode = ins >> 26;
    typ = insn_type::INSN_OTHER;
    imm = 0;

    match opcode {
        18 => {
            /* b[l][a] */
            if ins == 0x48000005 {
                /* bl .+4 */
                typ = insn_type::INSN_OTHER;
            } else if (ins & 1) != 0 {
                /* bl[a] */
                typ = insn_type::INSN_CALL;
            } else {
                /* b[a] */
                typ = insn_type::INSN_JUMP_UNCONDITIONAL;
            }

            imm = (ins & 0x3fffffc) as c_ulong;
            if (imm & 0x2000000) != 0 {
                imm = imm.wrapping_sub(0x4000000);
            }
            imm |= (ins & 2) as c_ulong; /* AA flag */
        }
        _ => {}
    }

    if opcode == 1 {
        unsafe {
            (*insn).len = 8;
        }
    } else {
        unsafe {
            (*insn).len = 4;
        }
    }

    unsafe {
        (*insn).type_ = typ;
        (*insn).immediate = imm;
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn arch_jump_destination(insn: *mut instruction) -> c_ulong {
    unsafe {
        if ((*insn).immediate & 2) != 0 {
            return (*insn).immediate & !2;
        }

        (*insn).offset.wrapping_add((*insn).immediate)
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn arch_pc_relative_reloc(_reloc: *mut reloc) -> bool {
    /*
     * The powerpc build only allows certain relocation types, see
     * relocs_check.sh, and none of those accepted are PC relative.
     */
    false
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn arch_initial_func_cfi_state(state: *mut cfi_init_state) {
    let mut i: c_int;

    i = 0;
    while i < CFI_NUM_REGS as c_int {
        unsafe {
            (*state).regs[i as usize].base = CFI_UNDEFINED;
            (*state).regs[i as usize].offset = 0;
        }
        i += 1;
    }

    /* initial CFA (call frame address) */
    unsafe {
        (*state).cfa.base = CFI_SP;
        (*state).cfa.offset = 0;
    }

    /* initial LR (return address) */
    unsafe {
        (*state).regs[CFI_RA].base = CFI_CFA;
        (*state).regs[CFI_RA].offset = 0;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn arch_reloc_size(reloc: *mut reloc) -> c_uint {
    match unsafe { reloc_type(reloc) } {
        R_PPC_REL32 | R_PPC_ADDR32 | R_PPC_UADDR32 | R_PPC_PLT32 | R_PPC_PLTREL32 => 4,
        _ => 8,
    }
}

// C source conditionally compiles this block when DISAS is defined.
#[cfg(DISAS)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn arch_disas_info_init(dinfo: *mut disassemble_info) -> c_int {
    unsafe {
        disas_info_init(
            dinfo,
            bfd_arch_powerpc,
            bfd_mach_ppc,
            bfd_mach_ppc64,
            core::ptr::null_mut(),
        )
    }
}
