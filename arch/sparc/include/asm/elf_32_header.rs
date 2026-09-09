/* SPDX-License-Identifier: GPL-2.0 */

/* ELF register definitions. */

/* Sparc section types */
pub const STT_REGISTER: u32 = 13;

/* Sparc ELF relocation types */
pub const R_SPARC_NONE: u32 = 0;
pub const R_SPARC_8: u32 = 1;
pub const R_SPARC_16: u32 = 2;
pub const R_SPARC_32: u32 = 3;
pub const R_SPARC_DISP8: u32 = 4;
pub const R_SPARC_DISP16: u32 = 5;
pub const R_SPARC_DISP32: u32 = 6;
pub const R_SPARC_WDISP30: u32 = 7;
pub const R_SPARC_WDISP22: u32 = 8;
pub const R_SPARC_HI22: u32 = 9;
pub const R_SPARC_22: u32 = 10;
pub const R_SPARC_13: u32 = 11;
pub const R_SPARC_LO10: u32 = 12;
pub const R_SPARC_GOT10: u32 = 13;
pub const R_SPARC_GOT13: u32 = 14;
pub const R_SPARC_GOT22: u32 = 15;
pub const R_SPARC_PC10: u32 = 16;
pub const R_SPARC_PC22: u32 = 17;
pub const R_SPARC_WPLT30: u32 = 18;
pub const R_SPARC_COPY: u32 = 19;
pub const R_SPARC_GLOB_DAT: u32 = 20;
pub const R_SPARC_JMP_SLOT: u32 = 21;
pub const R_SPARC_RELATIVE: u32 = 22;
pub const R_SPARC_UA32: u32 = 23;
pub const R_SPARC_PLT32: u32 = 24;
pub const R_SPARC_HIPLT22: u32 = 25;
pub const R_SPARC_LOPLT10: u32 = 26;
pub const R_SPARC_PCPLT32: u32 = 27;
pub const R_SPARC_PCPLT22: u32 = 28;
pub const R_SPARC_PCPLT10: u32 = 29;
pub const R_SPARC_10: u32 = 30;
pub const R_SPARC_11: u32 = 31;
pub const R_SPARC_64: u32 = 32;
pub const R_SPARC_OLO10: u32 = 33;
pub const R_SPARC_WDISP16: u32 = 40;
pub const R_SPARC_WDISP19: u32 = 41;
pub const R_SPARC_7: u32 = 43;
pub const R_SPARC_5: u32 = 44;
pub const R_SPARC_6: u32 = 45;

/* Bits present in AT_HWCAP, primarily for Sparc32. */
pub const HWCAP_SPARC_FLUSH: u32 = 1;
pub const HWCAP_SPARC_STBAR: u32 = 2;
pub const HWCAP_SPARC_SWAP: u32 = 4;
pub const HWCAP_SPARC_MULDIV: u32 = 8;
pub const HWCAP_SPARC_V9: u32 = 16;
pub const HWCAP_SPARC_ULTRA3: u32 = 32;

pub const CORE_DUMP_USE_REGSET: bool = true;

pub type elf_greg_t = u32;
pub const ELF_NGREG: usize = 38;
pub type elf_gregset_t = [elf_greg_t; ELF_NGREG];

#[repr(C)]
pub union ElfFpRegs {
    pub pr_regs: [u32; 32],
    pub pr_dregs: [f64; 16],
}

#[repr(C)]
pub struct elf_fpregset_t {
    pub pr_fr: ElfFpRegs,
    pub __unused: u32,
    pub pr_fsr: u32,
    pub pr_qcnt: u8,
    pub pr_q_entrysize: u8,
    pub pr_en: u8,
    pub pr_q: [u32; 64],
}

/* This is used to ensure we don't load something for the wrong architecture. */
#[macro_export]
macro_rules! elf_check_arch {
    ($x:expr) => { ($x).e_machine == EM_SPARC };
}

/* These are used to set parameters in the core dumps. */
pub const ELF_ARCH: u16 = EM_SPARC;
pub const ELF_CLASS: u8 = ELFCLASS32;
pub const ELF_DATA: u8 = ELFDATA2MSB;
pub const ELF_EXEC_PAGESIZE: usize = 4096;

/* This is the location that an ET_DYN program is loaded if exec'ed. */
pub const ELF_ET_DYN_BASE: usize = TASK_UNMAPPED_BASE;

/* Most sun4m's have them all. */
pub const ELF_HWCAP: u32 = HWCAP_SPARC_FLUSH
    | HWCAP_SPARC_STBAR
    | HWCAP_SPARC_SWAP
    | HWCAP_SPARC_MULDIV;

/* This yields a string that ld.so will use to load implementation specific libraries. */
pub const ELF_PLATFORM: *const core::ffi::c_void = core::ptr::null();

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
