/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (C) 2020-2022 Loongson Technology Corporation Limited */
// Translated from the C header. Included kernel dependencies are intentionally external.

/* The ABI of a file. */
pub const EF_LOONGARCH_ABI_LP64_SOFT_FLOAT: u32 = 0x1;
pub const EF_LOONGARCH_ABI_LP64_SINGLE_FLOAT: u32 = 0x2;
pub const EF_LOONGARCH_ABI_LP64_DOUBLE_FLOAT: u32 = 0x3;
pub const EF_LOONGARCH_ABI_ILP32_SOFT_FLOAT: u32 = 0x5;
pub const EF_LOONGARCH_ABI_ILP32_SINGLE_FLOAT: u32 = 0x6;
pub const EF_LOONGARCH_ABI_ILP32_DOUBLE_FLOAT: u32 = 0x7;

/* LoongArch relocation types used by the dynamic linker. */
pub const R_LARCH_NONE: u32 = 0;
pub const R_LARCH_32: u32 = 1;
pub const R_LARCH_64: u32 = 2;
pub const R_LARCH_RELATIVE: u32 = 3;
pub const R_LARCH_COPY: u32 = 4;
pub const R_LARCH_JUMP_SLOT: u32 = 5;
pub const R_LARCH_TLS_DTPMOD32: u32 = 6;
pub const R_LARCH_TLS_DTPMOD64: u32 = 7;
pub const R_LARCH_TLS_DTPREL32: u32 = 8;
pub const R_LARCH_TLS_DTPREL64: u32 = 9;
pub const R_LARCH_TLS_TPREL32: u32 = 10;
pub const R_LARCH_TLS_TPREL64: u32 = 11;
pub const R_LARCH_IRELATIVE: u32 = 12;
pub const R_LARCH_MARK_LA: u32 = 20;
pub const R_LARCH_MARK_PCREL: u32 = 21;
pub const R_LARCH_SOP_PUSH_PCREL: u32 = 22;
pub const R_LARCH_SOP_PUSH_ABSOLUTE: u32 = 23;
pub const R_LARCH_SOP_PUSH_DUP: u32 = 24;
pub const R_LARCH_SOP_PUSH_GPREL: u32 = 25;
pub const R_LARCH_SOP_PUSH_TLS_TPREL: u32 = 26;
pub const R_LARCH_SOP_PUSH_TLS_GOT: u32 = 27;
pub const R_LARCH_SOP_PUSH_TLS_GD: u32 = 28;
pub const R_LARCH_SOP_PUSH_PLT_PCREL: u32 = 29;
pub const R_LARCH_SOP_ASSERT: u32 = 30;
pub const R_LARCH_SOP_NOT: u32 = 31;
pub const R_LARCH_SOP_SUB: u32 = 32;
pub const R_LARCH_SOP_SL: u32 = 33;
pub const R_LARCH_SOP_SR: u32 = 34;
pub const R_LARCH_SOP_ADD: u32 = 35;
pub const R_LARCH_SOP_AND: u32 = 36;
pub const R_LARCH_SOP_IF_ELSE: u32 = 37;
pub const R_LARCH_SOP_POP_32_S_10_5: u32 = 38;
pub const R_LARCH_SOP_POP_32_U_10_12: u32 = 39;
pub const R_LARCH_SOP_POP_32_S_10_12: u32 = 40;
pub const R_LARCH_SOP_POP_32_S_10_16: u32 = 41;
pub const R_LARCH_SOP_POP_32_S_10_16_S2: u32 = 42;
pub const R_LARCH_SOP_POP_32_S_5_20: u32 = 43;
pub const R_LARCH_SOP_POP_32_S_0_5_10_16_S2: u32 = 44;
pub const R_LARCH_SOP_POP_32_S_0_10_10_16_S2: u32 = 45;
pub const R_LARCH_SOP_POP_32_U: u32 = 46;

/* Values 47..139 are deliberately listed to preserve the complete ABI table. */
pub const R_LARCH_ADD8: u32 = 47; pub const R_LARCH_ADD16: u32 = 48; pub const R_LARCH_ADD24: u32 = 49; pub const R_LARCH_ADD32: u32 = 50; pub const R_LARCH_ADD64: u32 = 51;
pub const R_LARCH_SUB8: u32 = 52; pub const R_LARCH_SUB16: u32 = 53; pub const R_LARCH_SUB24: u32 = 54; pub const R_LARCH_SUB32: u32 = 55; pub const R_LARCH_SUB64: u32 = 56;
pub const R_LARCH_GNU_VTINHERIT: u32 = 57; pub const R_LARCH_GNU_VTENTRY: u32 = 58;
pub const R_LARCH_B16: u32 = 64; pub const R_LARCH_B21: u32 = 65; pub const R_LARCH_B26: u32 = 66;
pub const R_LARCH_ABS_HI20: u32 = 67; pub const R_LARCH_ABS_LO12: u32 = 68; pub const R_LARCH_ABS64_LO20: u32 = 69; pub const R_LARCH_ABS64_HI12: u32 = 70;
pub const R_LARCH_PCALA_HI20: u32 = 71; pub const R_LARCH_PCALA_LO12: u32 = 72; pub const R_LARCH_PCALA64_LO20: u32 = 73; pub const R_LARCH_PCALA64_HI12: u32 = 74;
pub const R_LARCH_GOT_PC_HI20: u32 = 75; pub const R_LARCH_GOT_PC_LO12: u32 = 76; pub const R_LARCH_GOT64_PC_LO20: u32 = 77; pub const R_LARCH_GOT64_PC_HI12: u32 = 78;
pub const R_LARCH_GOT_HI20: u32 = 79; pub const R_LARCH_GOT_LO12: u32 = 80; pub const R_LARCH_GOT64_LO20: u32 = 81; pub const R_LARCH_GOT64_HI12: u32 = 82;
pub const R_LARCH_TLS_LE_HI20: u32 = 83; pub const R_LARCH_TLS_LE_LO12: u32 = 84; pub const R_LARCH_TLS_LE64_LO20: u32 = 85; pub const R_LARCH_TLS_LE64_HI12: u32 = 86;
pub const R_LARCH_TLS_IE_PC_HI20: u32 = 87; pub const R_LARCH_TLS_IE_PC_LO12: u32 = 88; pub const R_LARCH_TLS_IE64_PC_LO20: u32 = 89; pub const R_LARCH_TLS_IE64_PC_HI12: u32 = 90;
pub const R_LARCH_TLS_IE_HI20: u32 = 91; pub const R_LARCH_TLS_IE_LO12: u32 = 92; pub const R_LARCH_TLS_IE64_LO20: u32 = 93; pub const R_LARCH_TLS_IE64_HI12: u32 = 94;
pub const R_LARCH_TLS_LD_PC_HI20: u32 = 95; pub const R_LARCH_TLS_LD_HI20: u32 = 96; pub const R_LARCH_TLS_GD_PC_HI20: u32 = 97; pub const R_LARCH_TLS_GD_HI20: u32 = 98;
pub const R_LARCH_32_PCREL: u32 = 99; pub const R_LARCH_RELAX: u32 = 100; pub const R_LARCH_DELETE: u32 = 101; pub const R_LARCH_ALIGN: u32 = 102; pub const R_LARCH_PCREL20_S2: u32 = 103; pub const R_LARCH_CFA: u32 = 104; pub const R_LARCH_ADD6: u32 = 105; pub const R_LARCH_SUB6: u32 = 106; pub const R_LARCH_ADD_ULEB128: u32 = 107; pub const R_LARCH_SUB_ULEB128: u32 = 108; pub const R_LARCH_64_PCREL: u32 = 109; pub const R_LARCH_CALL36: u32 = 110;
pub const R_LARCH_TLS_DESC_PC_HI20: u32 = 111; pub const R_LARCH_TLS_DESC_PC_LO12: u32 = 112; pub const R_LARCH_TLS_DESC64_PC_LO20: u32 = 113; pub const R_LARCH_TLS_DESC64_PC_HI12: u32 = 114; pub const R_LARCH_TLS_DESC_HI20: u32 = 115; pub const R_LARCH_TLS_DESC_LO12: u32 = 116; pub const R_LARCH_TLS_DESC64_LO20: u32 = 117; pub const R_LARCH_TLS_DESC64_HI12: u32 = 118; pub const R_LARCH_TLS_DESC_LD: u32 = 119; pub const R_LARCH_TLS_DESC_CALL: u32 = 120;
pub const R_LARCH_TLS_LE_HI20_R: u32 = 121; pub const R_LARCH_TLS_LE_ADD_R: u32 = 122; pub const R_LARCH_TLS_LE_LO12_R: u32 = 123; pub const R_LARCH_TLS_LD_PCREL20_S2: u32 = 124; pub const R_LARCH_TLS_GD_PCREL20_S2: u32 = 125; pub const R_LARCH_TLS_DESC_PCREL20_S2: u32 = 126; pub const R_LARCH_CALL30: u32 = 127; pub const R_LARCH_PCADD_HI20: u32 = 128; pub const R_LARCH_PCADD_LO12: u32 = 129; pub const R_LARCH_GOT_PCADD_HI20: u32 = 130; pub const R_LARCH_GOT_PCADD_LO12: u32 = 131; pub const R_LARCH_TLS_IE_PCADD_HI20: u32 = 132; pub const R_LARCH_TLS_IE_PCADD_LO12: u32 = 133; pub const R_LARCH_TLS_LD_PCADD_HI20: u32 = 134; pub const R_LARCH_TLS_LD_PCADD_LO12: u32 = 135; pub const R_LARCH_TLS_GD_PCADD_HI20: u32 = 136; pub const R_LARCH_TLS_GD_PCADD_LO12: u32 = 137; pub const R_LARCH_TLS_DESC_PCADD_HI20: u32 = 138; pub const R_LARCH_TLS_DESC_PCADD_LO12: u32 = 139;

pub const ELF_NGREG: usize = 45;
pub const ELF_NFPREG: usize = 34;
pub type elf_greg_t = usize;
pub type elf_gregset_t = [elf_greg_t; ELF_NGREG];
pub type elf_fpreg_t = f64;
pub type elf_fpregset_t = [elf_fpreg_t; ELF_NFPREG];

extern "C" {
    pub fn loongarch_dump_regs32(uregs: *mut u32, regs: *const crate::pt_regs);
    pub fn loongarch_dump_regs64(uregs: *mut u64, regs: *const crate::pt_regs);
    pub static mut elf_hwcap: u32;
    pub static __elf_platform: *const core::ffi::c_char;
    pub fn arch_setup_additional_pages(bprm: *mut crate::linux_binprm, uses_interp: i32) -> i32;
    pub fn arch_elf_pt_proc(ehdr: *mut core::ffi::c_void, phdr: *mut core::ffi::c_void, elf: *mut crate::file, is_interp: bool, state: *mut arch_elf_state) -> i32;
    pub fn arch_check_elf(ehdr: *mut core::ffi::c_void, has_interpreter: bool, interp_ehdr: *mut core::ffi::c_void, state: *mut arch_elf_state) -> i32;
}

#[repr(C)]
pub struct arch_elf_state { pub fp_abi: i32, pub interp_fp_abi: i32 }
pub const LOONGARCH_ABI_FP_ANY: i32 = 0;
// `elf_check_arch`, SET_PERSONALITY2, ELF_PLAT_INIT, ARCH_DLINFO and related
// macros remain build-configuration-dependent kernel operations and are represented
// by their declarations above and by the constants below.
pub const CORE_DUMP_USE_REGSET: bool = true;
pub const ARCH_HAS_SETUP_ADDITIONAL_PAGES: i32 = 1;
pub const ELF_DATA: u32 = crate::ELFDATA2LSB;
pub const ELF_ARCH: u32 = crate::EM_LOONGARCH;
pub const ELF_EXEC_PAGESIZE: usize = crate::PAGE_SIZE;
pub const ELF_ET_DYN_BASE_DIVISOR: usize = 3;
pub const ELF_HWCAP: u32 = unsafe { elf_hwcap };
pub const LOONGARCH_ABI_FP_ANY_STATE: arch_elf_state = arch_elf_state {
    fp_abi: LOONGARCH_ABI_FP_ANY,
    interp_fp_abi: LOONGARCH_ABI_FP_ANY,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
