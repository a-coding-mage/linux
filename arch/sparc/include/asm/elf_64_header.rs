/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of asm/elf_64.h. */

/* Dependencies supplied by the surrounding kernel translation unit. */

/* ELF register definitions. */

/* Sparc section types. */
pub const STT_REGISTER: u32 = 13;

/* Sparc ELF relocation types. */
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
pub const R_SPARC_UA64: u32 = 54;

pub const HWCAP_SPARC_FLUSH: u32 = 0x00000001;
pub const HWCAP_SPARC_STBAR: u32 = 0x00000002;
pub const HWCAP_SPARC_SWAP: u32 = 0x00000004;
pub const HWCAP_SPARC_MULDIV: u32 = 0x00000008;
pub const HWCAP_SPARC_V9: u32 = 0x00000010;
pub const HWCAP_SPARC_ULTRA3: u32 = 0x00000020;
pub const HWCAP_SPARC_BLKINIT: u32 = 0x00000040;
pub const HWCAP_SPARC_N2: u32 = 0x00000080;

pub const AV_SPARC_MUL32: u32 = 0x00000100;
pub const AV_SPARC_DIV32: u32 = 0x00000200;
pub const AV_SPARC_FSMULD: u32 = 0x00000400;
pub const AV_SPARC_V8PLUS: u32 = 0x00000800;
pub const AV_SPARC_POPC: u32 = 0x00001000;
pub const AV_SPARC_VIS: u32 = 0x00002000;
pub const AV_SPARC_VIS2: u32 = 0x00004000;
pub const AV_SPARC_ASI_BLK_INIT: u32 = 0x00008000;
pub const AV_SPARC_FMAF: u32 = 0x00010000;
pub const AV_SPARC_VIS3: u32 = 0x00020000;
pub const AV_SPARC_HPC: u32 = 0x00040000;
pub const AV_SPARC_RANDOM: u32 = 0x00080000;
pub const AV_SPARC_TRANS: u32 = 0x00100000;
pub const AV_SPARC_FJFMAU: u32 = 0x00200000;
pub const AV_SPARC_IMA: u32 = 0x00400000;
pub const AV_SPARC_ASI_CACHE_SPARING: u32 = 0x00800000;
pub const AV_SPARC_PAUSE: u32 = 0x01000000;
pub const AV_SPARC_CBCOND: u32 = 0x02000000;
pub const HWCAP_SPARC_CRYPTO: u32 = 0x04000000;
pub const HWCAP_SPARC_ADI: u32 = 0x08000000;

pub const CORE_DUMP_USE_REGSET: bool = true;
pub const ELF_ARCH: u32 = EM_SPARCV9;
pub const ELF_CLASS: u32 = ELFCLASS64;
pub const ELF_DATA: u32 = ELFDATA2MSB;

pub type ElfGregT = ::core::ffi::c_ulong;
pub const ELF_NGREG: usize = 36;
pub type ElfGregsetT = [ElfGregT; ELF_NGREG];

#[repr(C)]
pub struct ElfFpregsetT {
    pub pr_regs: [::core::ffi::c_ulong; 32],
    pub pr_fsr: ::core::ffi::c_ulong,
    pub pr_gsr: ::core::ffi::c_ulong,
    pub pr_fprs: ::core::ffi::c_ulong,
}

pub type CompatElfGregT = ::core::ffi::c_uint;
pub const COMPAT_ELF_NGREG: usize = 38;
pub type CompatElfGregsetT = [CompatElfGregT; COMPAT_ELF_NGREG];

#[repr(C)]
pub union CompatElfFpRegs {
    pub pr_regs: [::core::ffi::c_uint; 32],
    pub pr_dregs: [::core::ffi::c_ulong; 16],
}

#[repr(C)]
pub struct CompatElfFpregsetT {
    pub pr_fr: CompatElfFpRegs,
    pub __unused: ::core::ffi::c_uint,
    pub pr_fsr: ::core::ffi::c_uint,
    pub pr_qcnt: u8,
    pub pr_q_entrysize: u8,
    pub pr_en: u8,
    pub pr_q: [::core::ffi::c_uint; 64],
}

/* UltraSparc extensions. Still unused, but will be eventually. */
#[repr(C)]
pub struct ElfXregsetT {
    pub pr_type: ::core::ffi::c_uint,
    pub pr_align: ::core::ffi::c_uint,
    pub pr_un: ElfXregsetUnion,
}

#[repr(C)]
pub union ElfXregsetUnion {
    pub pr_v8p: ElfV8p,
    pub pr_xfsr: ::core::ffi::c_uint,
    pub pr_fprs: ::core::ffi::c_uint,
    pub pr_xg: [::core::ffi::c_uint; 8],
    pub pr_xo: [::core::ffi::c_uint; 8],
    pub pr_tstate: ::core::ffi::c_ulong,
    pub pr_filler: [::core::ffi::c_uint; 8],
}

#[repr(C)]
pub struct ElfV8p {
    pub pr_xfr: ElfXfr,
}

#[repr(C)]
pub union ElfXfr {
    pub pr_regs: [::core::ffi::c_uint; 32],
    pub pr_dregs: [::core::ffi::c_ulong; 16],
    pub pr_qregs: [u128; 8],
}

#[macro_export]
macro_rules! elf_check_arch { ($x:expr) => { ($x).e_machine == $crate::ELF_ARCH }; }
#[macro_export]
macro_rules! compat_elf_check_arch { ($x:expr) => { ($x).e_machine == EM_SPARC || ($x).e_machine == EM_SPARC32PLUS }; }
pub use start_thread32 as compat_start_thread;
pub const ELF_EXEC_PAGESIZE: usize = PAGE_SIZE;
pub const ELF_ET_DYN_BASE: ::core::ffi::c_ulong = 0x0000010000000000;
pub const COMPAT_ELF_ET_DYN_BASE: ::core::ffi::c_ulong = 0x0000000070000000;

unsafe extern "C" {
    pub static mut sparc64_elf_hwcap: ::core::ffi::c_ulong;
    pub static mut vdso_enabled: ::core::ffi::c_uint;
    pub fn arch_setup_additional_pages(bprm: *mut LinuxBinprm, uses_interp: ::core::ffi::c_int) -> ::core::ffi::c_int;
}
pub const ELF_HWCAP: *mut ::core::ffi::c_ulong = unsafe { &raw mut sparc64_elf_hwcap };
pub const ELF_PLATFORM: *const core::ffi::c_char = core::ptr::null();

#[repr(C)]
pub struct LinuxBinprm;

/* SET_PERSONALITY and ARCH_DLINFO retain their C macro form and depend on surrounding kernel symbols. */
#[macro_export]
macro_rules! SET_PERSONALITY { ($ex:expr) => {{ if ($ex).e_ident[EI_CLASS] == ELFCLASS32 { set_thread_flag(TIF_32BIT); } else { clear_thread_flag(TIF_32BIT); } if personality(current->personality) != PER_LINUX32 { set_personality(PER_LINUX | (current->personality & !PER_MASK)); } }}; }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
