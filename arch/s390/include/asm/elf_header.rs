/* SPDX-License-Identifier: GPL-2.0 */
/*
 *  S390 version
 *
 *  Derived from "include/asm-i386/elf.h"
 */

/* s390 relocations defined by the ABIs */
pub const R_390_NONE: u32 = 0; /* No reloc. */
pub const R_390_8: u32 = 1; /* Direct 8 bit. */
pub const R_390_12: u32 = 2; /* Direct 12 bit. */
pub const R_390_16: u32 = 3; /* Direct 16 bit. */
pub const R_390_32: u32 = 4; /* Direct 32 bit. */
pub const R_390_PC32: u32 = 5; /* PC relative 32 bit. */
pub const R_390_GOT12: u32 = 6; /* 12 bit GOT offset. */
pub const R_390_GOT32: u32 = 7; /* 32 bit GOT offset. */
pub const R_390_PLT32: u32 = 8; /* 32 bit PC relative PLT address. */
pub const R_390_COPY: u32 = 9; /* Copy symbol at runtime. */
pub const R_390_GLOB_DAT: u32 = 10; /* Create GOT entry. */
pub const R_390_JMP_SLOT: u32 = 11; /* Create PLT entry. */
pub const R_390_RELATIVE: u32 = 12; /* Adjust by program base. */
pub const R_390_GOTOFF32: u32 = 13; /* 32 bit offset to GOT. */
pub const R_390_GOTPC: u32 = 14; /* 32 bit PC rel. offset to GOT. */
pub const R_390_GOT16: u32 = 15; /* 16 bit GOT offset. */
pub const R_390_PC16: u32 = 16; /* PC relative 16 bit. */
pub const R_390_PC16DBL: u32 = 17; /* PC relative 16 bit shifted by 1. */
pub const R_390_PLT16DBL: u32 = 18; /* 16 bit PC rel. PLT shifted by 1. */
pub const R_390_PC32DBL: u32 = 19; /* PC relative 32 bit shifted by 1. */
pub const R_390_PLT32DBL: u32 = 20; /* 32 bit PC rel. PLT shifted by 1. */
pub const R_390_GOTPCDBL: u32 = 21; /* 32 bit PC rel. GOT shifted by 1. */
pub const R_390_64: u32 = 22; /* Direct 64 bit. */
pub const R_390_PC64: u32 = 23; /* PC relative 64 bit. */
pub const R_390_GOT64: u32 = 24; /* 64 bit GOT offset. */
pub const R_390_PLT64: u32 = 25; /* 64 bit PC relative PLT address. */
pub const R_390_GOTENT: u32 = 26; /* 32 bit PC rel. to GOT entry >> 1. */
pub const R_390_GOTOFF16: u32 = 27; /* 16 bit offset to GOT. */
pub const R_390_GOTOFF64: u32 = 28; /* 64 bit offset to GOT. */
pub const R_390_GOTPLT12: u32 = 29; /* 12 bit offset to jump slot. */
pub const R_390_GOTPLT16: u32 = 30; /* 16 bit offset to jump slot. */
pub const R_390_GOTPLT32: u32 = 31; /* 32 bit offset to jump slot. */
pub const R_390_GOTPLT64: u32 = 32; /* 64 bit offset to jump slot. */
pub const R_390_GOTPLTENT: u32 = 33; /* 32 bit rel. offset to jump slot. */
pub const R_390_PLTOFF16: u32 = 34; /* 16 bit offset from GOT to PLT. */
pub const R_390_PLTOFF32: u32 = 35; /* 32 bit offset from GOT to PLT. */
pub const R_390_PLTOFF64: u32 = 36; /* 16 bit offset from GOT to PLT. */
pub const R_390_TLS_LOAD: u32 = 37; /* Tag for load insn in TLS code. */
pub const R_390_TLS_GDCALL: u32 = 38;
pub const R_390_TLS_LDCALL: u32 = 39;
pub const R_390_TLS_GD32: u32 = 40;
pub const R_390_TLS_GD64: u32 = 41;
pub const R_390_TLS_GOTIE12: u32 = 42;
pub const R_390_TLS_GOTIE32: u32 = 43;
pub const R_390_TLS_GOTIE64: u32 = 44;
pub const R_390_TLS_LDM32: u32 = 45;
pub const R_390_TLS_LDM64: u32 = 46;
pub const R_390_TLS_IE32: u32 = 47;
pub const R_390_TLS_IE64: u32 = 48;
pub const R_390_TLS_IEENT: u32 = 49;
pub const R_390_TLS_LE32: u32 = 50;
pub const R_390_TLS_LE64: u32 = 51;
pub const R_390_TLS_LDO32: u32 = 52;
pub const R_390_TLS_LDO64: u32 = 53;
pub const R_390_TLS_DTPMOD: u32 = 54;
pub const R_390_TLS_DTPOFF: u32 = 55;
pub const R_390_TLS_TPOFF: u32 = 56;
pub const R_390_20: u32 = 57;
pub const R_390_GOT20: u32 = 58;
pub const R_390_GOTPLT20: u32 = 59;
pub const R_390_TLS_GOTIE20: u32 = 60;
/* Keep this the last entry. */
pub const R_390_NUM: u32 = 61;

/* HWCAP flags - for AT_HWCAP */
pub const HWCAP_NR_ESAN3: u32 = 0;
pub const HWCAP_NR_ZARCH: u32 = 1;
pub const HWCAP_NR_STFLE: u32 = 2;
pub const HWCAP_NR_MSA: u32 = 3;
pub const HWCAP_NR_LDISP: u32 = 4;
pub const HWCAP_NR_EIMM: u32 = 5;
pub const HWCAP_NR_DFP: u32 = 6;
pub const HWCAP_NR_HPAGE: u32 = 7;
pub const HWCAP_NR_ETF3EH: u32 = 8;
pub const HWCAP_NR_HIGH_GPRS: u32 = 9;
pub const HWCAP_NR_TE: u32 = 10;
pub const HWCAP_NR_VXRS: u32 = 11;
pub const HWCAP_NR_VXRS_BCD: u32 = 12;
pub const HWCAP_NR_VXRS_EXT: u32 = 13;
pub const HWCAP_NR_GS: u32 = 14;
pub const HWCAP_NR_VXRS_EXT2: u32 = 15;
pub const HWCAP_NR_VXRS_PDE: u32 = 16;
pub const HWCAP_NR_SORT: u32 = 17;
pub const HWCAP_NR_DFLT: u32 = 18;
pub const HWCAP_NR_VXRS_PDE2: u32 = 19;
pub const HWCAP_NR_NNPA: u32 = 20;
pub const HWCAP_NR_PCI_MIO: u32 = 21;
pub const HWCAP_NR_SIE: u32 = 22;
pub const HWCAP_NR_MAX: u32 = 23;

macro_rules! BIT { ($x:expr) => { 1usize << ($x) }; }
pub const HWCAP_ESAN3: usize = BIT!(HWCAP_NR_ESAN3);
pub const HWCAP_ZARCH: usize = BIT!(HWCAP_NR_ZARCH);
pub const HWCAP_STFLE: usize = BIT!(HWCAP_NR_STFLE);
pub const HWCAP_MSA: usize = BIT!(HWCAP_NR_MSA);
pub const HWCAP_LDISP: usize = BIT!(HWCAP_NR_LDISP);
pub const HWCAP_EIMM: usize = BIT!(HWCAP_NR_EIMM);
pub const HWCAP_DFP: usize = BIT!(HWCAP_NR_DFP);
pub const HWCAP_HPAGE: usize = BIT!(HWCAP_NR_HPAGE);
pub const HWCAP_ETF3EH: usize = BIT!(HWCAP_NR_ETF3EH);
pub const HWCAP_HIGH_GPRS: usize = BIT!(HWCAP_NR_HIGH_GPRS);
pub const HWCAP_TE: usize = BIT!(HWCAP_NR_TE);
pub const HWCAP_VXRS: usize = BIT!(HWCAP_NR_VXRS);
pub const HWCAP_VXRS_BCD: usize = BIT!(HWCAP_NR_VXRS_BCD);
pub const HWCAP_VXRS_EXT: usize = BIT!(HWCAP_NR_VXRS_EXT);
pub const HWCAP_GS: usize = BIT!(HWCAP_NR_GS);
pub const HWCAP_VXRS_EXT2: usize = BIT!(HWCAP_NR_VXRS_EXT2);
pub const HWCAP_VXRS_PDE: usize = BIT!(HWCAP_NR_VXRS_PDE);
pub const HWCAP_SORT: usize = BIT!(HWCAP_NR_SORT);
pub const HWCAP_DFLT: usize = BIT!(HWCAP_NR_DFLT);
pub const HWCAP_VXRS_PDE2: usize = BIT!(HWCAP_NR_VXRS_PDE2);
pub const HWCAP_NNPA: usize = BIT!(HWCAP_NR_NNPA);
pub const HWCAP_PCI_MIO: usize = BIT!(HWCAP_NR_PCI_MIO);
pub const HWCAP_SIE: usize = BIT!(HWCAP_NR_SIE);

pub const ELF_CLASS: u32 = ELFCLASS64;
pub const ELF_DATA: u32 = ELFDATA2MSB;
pub const ELF_ARCH: u32 = EM_S390;

pub type elf_fpregset_t = s390_fp_regs;
pub type elf_gregset_t = s390_regs;

/* This is used to ensure we don't load something for the wrong architecture. */
macro_rules! elf_check_arch { ($x:expr) => { (($x).e_machine == EM_S390 || ($x).e_machine == EM_S390_OLD) && ($x).e_ident[EI_CLASS] == ELF_CLASS }; }

/* For SVR4/S390 the function pointer to be registered with `atexit` is passed in R14. */
macro_rules! ELF_PLAT_INIT { ($r:expr, $load_addr:expr) => {{ ($r).gprs[14] = 0; }}; }

pub const CORE_DUMP_USE_REGSET: bool = true;
pub const ELF_EXEC_PAGESIZE: usize = PAGE_SIZE;
pub const ELF_ET_DYN_BASE: usize = (STACK_TOP / 3 * 2) & !((1usize << 32) - 1);

extern "C" {
    pub static mut elf_hwcap: usize;
    pub static mut elf_platform: [u8; ELF_PLATFORM_SIZE];
}
pub const ELF_PLATFORM_SIZE: usize = 8;

macro_rules! ELF_HWCAP { () => { elf_hwcap }; }
macro_rules! ELF_PLATFORM { () => { elf_platform }; }

macro_rules! SET_PERSONALITY { ($ex:expr) => {{ set_personality(PER_LINUX | (current.personality & !PER_MASK)); }}; }

pub const BRK_RND_MASK: usize = 0x1fff;
pub const MMAP_RND_MASK: usize = 0x3ff80;
pub const MMAP_ALIGN_MASK: usize = 0x7f;
pub const STACK_RND_MASK: usize = MMAP_RND_MASK;

macro_rules! ARCH_DLINFO { () => {{ NEW_AUX_ENT(AT_SYSINFO_EHDR, current.mm.context.vdso_base as usize); }}; }

#[repr(C)]
pub struct linux_binprm;

pub const ARCH_HAS_SETUP_ADDITIONAL_PAGES: i32 = 1;
extern "C" { pub fn arch_setup_additional_pages(bprm: *mut linux_binprm, uses_interp: i32) -> i32; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
