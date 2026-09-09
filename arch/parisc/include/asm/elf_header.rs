/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of the PA-RISC ELF header definitions. */

pub const EM_PARISC: u32 = 15;

pub const EF_PARISC_TRAPNIL: u32 = 0x0001_0000;
pub const EF_PARISC_EXT: u32 = 0x0002_0000;
pub const EF_PARISC_LSB: u32 = 0x0004_0000;
pub const EF_PARISC_WIDE: u32 = 0x0008_0000;
pub const EF_PARISC_NO_KABP: u32 = 0x0010_0000;
pub const EF_PARISC_LAZYSWAP: u32 = 0x0040_0000;
pub const EF_PARISC_ARCH: u32 = 0x0000_ffff;

pub const EFA_PARISC_1_0: u32 = 0x020b;
pub const EFA_PARISC_1_1: u32 = 0x0210;
pub const EFA_PARISC_2_0: u32 = 0x0214;

pub const SHN_PARISC_ANSI_COMMON: u32 = 0xff00;
pub const SHN_PARISC_HUGE_COMMON: u32 = 0xff01;
pub const SHT_PARISC_EXT: u32 = 0x7000_0000;
pub const SHT_PARISC_UNWIND: u32 = 0x7000_0001;
pub const SHT_PARISC_DOC: u32 = 0x7000_0002;
pub const SHF_PARISC_SHORT: u32 = 0x2000_0000;
pub const SHF_PARISC_HUGE: u32 = 0x4000_0000;
pub const SHF_PARISC_SBP: u32 = 0x8000_0000;
pub const STT_PARISC_MILLICODE: u32 = 13;

/* STT_HP_OPAQUE and STT_HP_STUB depend on the ELF STT_LOOS definition. */
pub const STT_HP_OPAQUE: u32 = STT_LOOS + 0x1;
pub const STT_HP_STUB: u32 = STT_LOOS + 0x2;

pub const R_PARISC_NONE: u32 = 0;
pub const R_PARISC_DIR32: u32 = 1;
pub const R_PARISC_DIR21L: u32 = 2;
pub const R_PARISC_DIR17R: u32 = 3;
pub const R_PARISC_DIR17F: u32 = 4;
pub const R_PARISC_DIR14R: u32 = 6;
pub const R_PARISC_PCREL32: u32 = 9;
pub const R_PARISC_PCREL21L: u32 = 10;
pub const R_PARISC_PCREL17R: u32 = 11;
pub const R_PARISC_PCREL17F: u32 = 12;
pub const R_PARISC_PCREL14R: u32 = 14;
pub const R_PARISC_DPREL21L: u32 = 18;
pub const R_PARISC_DPREL14R: u32 = 22;
pub const R_PARISC_GPREL21L: u32 = 26;
pub const R_PARISC_GPREL14R: u32 = 30;
pub const R_PARISC_LTOFF21L: u32 = 34;
pub const R_PARISC_LTOFF14R: u32 = 38;
pub const R_PARISC_SECREL32: u32 = 41;
pub const R_PARISC_SEGBASE: u32 = 48;
pub const R_PARISC_SEGREL32: u32 = 49;
pub const R_PARISC_PLTOFF21L: u32 = 50;
pub const R_PARISC_PLTOFF14R: u32 = 54;
pub const R_PARISC_LTOFF_FPTR32: u32 = 57;
pub const R_PARISC_LTOFF_FPTR21L: u32 = 58;
pub const R_PARISC_LTOFF_FPTR14R: u32 = 62;
pub const R_PARISC_FPTR64: u32 = 64;
pub const R_PARISC_PLABEL32: u32 = 65;
pub const R_PARISC_PCREL64: u32 = 72;
pub const R_PARISC_PCREL22F: u32 = 74;
pub const R_PARISC_PCREL14WR: u32 = 75;
pub const R_PARISC_PCREL14DR: u32 = 76;
pub const R_PARISC_PCREL16F: u32 = 77;
pub const R_PARISC_PCREL16WF: u32 = 78;
pub const R_PARISC_PCREL16DF: u32 = 79;
pub const R_PARISC_DIR64: u32 = 80;
pub const R_PARISC_DIR14WR: u32 = 83;
pub const R_PARISC_DIR14DR: u32 = 84;
pub const R_PARISC_DIR16F: u32 = 85;
pub const R_PARISC_DIR16WF: u32 = 86;
pub const R_PARISC_DIR16DF: u32 = 87;
pub const R_PARISC_GPREL64: u32 = 88;
pub const R_PARISC_GPREL14WR: u32 = 91;
pub const R_PARISC_GPREL14DR: u32 = 92;
pub const R_PARISC_GPREL16F: u32 = 93;
pub const R_PARISC_GPREL16WF: u32 = 94;
pub const R_PARISC_GPREL16DF: u32 = 95;
pub const R_PARISC_LTOFF64: u32 = 96;
pub const R_PARISC_LTOFF14WR: u32 = 99;
pub const R_PARISC_LTOFF14DR: u32 = 100;
pub const R_PARISC_LTOFF16F: u32 = 101;
pub const R_PARISC_LTOFF16WF: u32 = 102;
pub const R_PARISC_LTOFF16DF: u32 = 103;
pub const R_PARISC_SECREL64: u32 = 104;
pub const R_PARISC_SEGREL64: u32 = 112;
pub const R_PARISC_PLTOFF14WR: u32 = 115;
pub const R_PARISC_PLTOFF14DR: u32 = 116;
pub const R_PARISC_PLTOFF16F: u32 = 117;
pub const R_PARISC_PLTOFF16WF: u32 = 118;
pub const R_PARISC_PLTOFF16DF: u32 = 119;
pub const R_PARISC_LTOFF_FPTR64: u32 = 120;
pub const R_PARISC_LTOFF_FPTR14WR: u32 = 123;
pub const R_PARISC_LTOFF_FPTR14DR: u32 = 124;
pub const R_PARISC_LTOFF_FPTR16F: u32 = 125;
pub const R_PARISC_LTOFF_FPTR16WF: u32 = 126;
pub const R_PARISC_LTOFF_FPTR16DF: u32 = 127;
pub const R_PARISC_LORESERVE: u32 = 128;
pub const R_PARISC_COPY: u32 = 128;
pub const R_PARISC_IPLT: u32 = 129;
pub const R_PARISC_EPLT: u32 = 130;
pub const R_PARISC_TPREL32: u32 = 153;
pub const R_PARISC_TPREL21L: u32 = 154;
pub const R_PARISC_TPREL14R: u32 = 158;
pub const R_PARISC_LTOFF_TP21L: u32 = 162;
pub const R_PARISC_LTOFF_TP14R: u32 = 166;
pub const R_PARISC_LTOFF_TP14F: u32 = 167;
pub const R_PARISC_TPREL64: u32 = 216;
pub const R_PARISC_TPREL14WR: u32 = 219;
pub const R_PARISC_TPREL14DR: u32 = 220;
pub const R_PARISC_TPREL16F: u32 = 221;
pub const R_PARISC_TPREL16WF: u32 = 222;
pub const R_PARISC_TPREL16DF: u32 = 223;
pub const R_PARISC_LTOFF_TP64: u32 = 224;
pub const R_PARISC_LTOFF_TP14WR: u32 = 227;
pub const R_PARISC_LTOFF_TP14DR: u32 = 228;
pub const R_PARISC_LTOFF_TP16F: u32 = 229;
pub const R_PARISC_LTOFF_TP16WF: u32 = 230;
pub const R_PARISC_LTOFF_TP16DF: u32 = 231;
pub const R_PARISC_HIRESERVE: u32 = 255;

pub const PA_PLABEL_FDESC: u32 = 0x02;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Elf32_Fdesc { pub addr: u32, pub gp: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Elf64_Fdesc { pub dummy: [u64; 2], pub addr: u64, pub gp: u64 }

/* CONFIG_64BIT selects Elf64_Fdesc; the enclosing build supplies that choice. */
#[cfg(target_pointer_width = "64")]
pub type Elf_Fdesc = Elf64_Fdesc;
#[cfg(not(target_pointer_width = "64"))]
pub type Elf_Fdesc = Elf32_Fdesc;

pub const PT_HP_TLS: u32 = PT_LOOS + 0x0;
pub const PT_HP_CORE_NONE: u32 = PT_LOOS + 0x1;
pub const PT_HP_CORE_VERSION: u32 = PT_LOOS + 0x2;
pub const PT_HP_CORE_KERNEL: u32 = PT_LOOS + 0x3;
pub const PT_HP_CORE_COMM: u32 = PT_LOOS + 0x4;
pub const PT_HP_CORE_PROC: u32 = PT_LOOS + 0x5;
pub const PT_HP_CORE_LOADABLE: u32 = PT_LOOS + 0x6;
pub const PT_HP_CORE_STACK: u32 = PT_LOOS + 0x7;
pub const PT_HP_CORE_SHM: u32 = PT_LOOS + 0x8;
pub const PT_HP_CORE_MMF: u32 = PT_LOOS + 0x9;
pub const PT_HP_PARALLEL: u32 = PT_LOOS + 0x10;
pub const PT_HP_FASTBIND: u32 = PT_LOOS + 0x11;
pub const PT_HP_OPT_ANNOT: u32 = PT_LOOS + 0x12;
pub const PT_HP_HSL_ANNOT: u32 = PT_LOOS + 0x13;
pub const PT_HP_STACK: u32 = PT_LOOS + 0x14;
pub const PT_PARISC_ARCHEXT: u32 = 0x7000_0000;
pub const PT_PARISC_UNWIND: u32 = 0x7000_0001;
pub const PF_PARISC_SBP: u32 = 0x0800_0000;
pub const PF_HP_PAGE_SIZE: u32 = 0x0010_0000;
pub const PF_HP_FAR_SHARED: u32 = 0x0020_0000;
pub const PF_HP_NEAR_SHARED: u32 = 0x0040_0000;
pub const PF_HP_CODE: u32 = 0x0100_0000;
pub const PF_HP_MODIFY: u32 = 0x0200_0000;
pub const PF_HP_LAZYSWAP: u32 = 0x0400_0000;
pub const PF_HP_SBP: u32 = 0x0800_0000;
pub const ELF_PLATFORM: &str = "PARISC";

pub type elf_greg_t = libc::c_ulong;
pub const ELF_NGREG: usize = 80;
pub type elf_gregset_t = [elf_greg_t; ELF_NGREG];
pub const ELF_NFPREG: usize = 32;
pub type elf_fpreg_t = f64;
pub type elf_fpregset_t = [elf_fpreg_t; ELF_NFPREG];

pub const ELF_DATA: u32 = ELFDATA2MSB;
pub const ELF_ARCH: u32 = EM_PARISC;
pub const ELF_OSABI: u32 = ELFOSABI_LINUX;
pub const ELF_EXEC_PAGESIZE: usize = 4096;
pub const ELF_HWCAP: u32 = 0;
pub const STACK_RND_MASK: usize = 0x7ff;
pub const ARCH_HAS_SETUP_ADDITIONAL_PAGES: u32 = 1;

/* These names are supplied by the ELF and kernel headers in the full build. */
pub const ELF_CLASS: u8 = {
    #[cfg(target_pointer_width = "64")]
    { ELFCLASS64 }
    #[cfg(not(target_pointer_width = "64"))]
    { ELFCLASS32 }
};

pub const ELF_ET_DYN_BASE: usize = TASK_UNMAPPED_BASE + 0x0100_0000;

/* ELF_CLASS, SET_PERSONALITY, core-register copying, and VDSO macros retain
 * their source conditional/build-context dependencies and are represented by
 * the declarations below for use by the surrounding kernel translation. */
extern "C" {
    pub fn arch_setup_additional_pages(bprm: *mut linux_binprm, executable_stack: i32) -> i32;
}

#[repr(C)]
pub struct linux_binprm { _private: [u8; 0] }

#[repr(C)]
pub struct task_struct { _private: [u8; 0] }
#[repr(C)]
pub struct pt_regs { _private: [u8; 0] }

/* C macro: ((x)->e_machine == EM_PARISC &&
 *           (x)->e_ident[EI_CLASS] == ELF_CLASS). */
#[inline]
pub unsafe fn elf_check_arch(x: *const ElfHeaderLike) -> bool {
    (*x).e_machine == EM_PARISC && (*x).e_ident[EI_CLASS_INDEX] == ELF_CLASS
}

#[inline]
pub unsafe fn compat_elf_check_arch(x: *const ElfHeaderLike) -> bool {
    (*x).e_machine == EM_PARISC && (*x).e_ident[EI_CLASS_INDEX] == ELFCLASS32
}

#[repr(C)]
pub struct ElfHeaderLike {
    pub e_ident: [u8; 16],
    pub e_machine: u32,
}

pub const EI_CLASS_INDEX: usize = EI_CLASS as usize;

/* ELF_CORE_COPY_REGS is a source-level register-layout operation.  The
 * register structure and mfctl intrinsic are supplied by the kernel build. */
pub const CORE_DUMP_USE_REGSET: bool = true;

/* ELF_PLAT_INIT(_r, load_addr): _r->gr[23] = 0. */
pub unsafe fn elf_plat_init(gr: *mut elf_greg_t) {
    *gr.add(23) = 0;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
