/* Translated from the MIPS ELF header. */

pub const EF_MIPS_ARCH_1: u32 = 0x00000000;
pub const EF_MIPS_ARCH_2: u32 = 0x10000000;
pub const EF_MIPS_ARCH_3: u32 = 0x20000000;
pub const EF_MIPS_ARCH_4: u32 = 0x30000000;
pub const EF_MIPS_ARCH_5: u32 = 0x40000000;
pub const EF_MIPS_ARCH_32: u32 = 0x50000000;
pub const EF_MIPS_ARCH_64: u32 = 0x60000000;
pub const EF_MIPS_ARCH_32R2: u32 = 0x70000000;
pub const EF_MIPS_ARCH_64R2: u32 = 0x80000000;
pub const EF_MIPS_ABI_O32: u32 = 0x00001000;
pub const EF_MIPS_ABI_O64: u32 = 0x00002000;

pub const PT_MIPS_REGINFO: u32 = 0x70000000;
pub const PT_MIPS_RTPROC: u32 = 0x70000001;
pub const PT_MIPS_OPTIONS: u32 = 0x70000002;
pub const PT_MIPS_ABIFLAGS: u32 = 0x70000003;
pub const EF_MIPS_NOREORDER: u32 = 1;
pub const EF_MIPS_PIC: u32 = 2;
pub const EF_MIPS_CPIC: u32 = 4;
pub const EF_MIPS_ABI2: u32 = 0x20;
pub const EF_MIPS_OPTIONS_FIRST: u32 = 0x80;
pub const EF_MIPS_32BITMODE: u32 = 0x100;
pub const EF_MIPS_FP64: u32 = 0x200;
pub const EF_MIPS_NAN2008: u32 = 0x400;
pub const EF_MIPS_ABI: u32 = 0xf000;
pub const EF_MIPS_ARCH: u32 = 0xf0000000;

pub const DT_MIPS_RLD_VERSION: u32 = 0x70000001;
pub const DT_MIPS_TIME_STAMP: u32 = 0x70000002;
pub const DT_MIPS_ICHECKSUM: u32 = 0x70000003;
pub const DT_MIPS_IVERSION: u32 = 0x70000004;
pub const DT_MIPS_FLAGS: u32 = 0x70000005;
pub const RHF_NONE: u32 = 0;
pub const RHF_HARDWAY: u32 = 1;
pub const RHF_NOTPOT: u32 = 2;
pub const RHF_SGI_ONLY: u32 = 0x10;
pub const DT_MIPS_BASE_ADDRESS: u32 = 0x70000006;
pub const DT_MIPS_CONFLICT: u32 = 0x70000008;
pub const DT_MIPS_LIBLIST: u32 = 0x70000009;
pub const DT_MIPS_LOCAL_GOTNO: u32 = 0x7000000a;
pub const DT_MIPS_CONFLICTNO: u32 = 0x7000000b;
pub const DT_MIPS_LIBLISTNO: u32 = 0x70000010;
pub const DT_MIPS_SYMTABNO: u32 = 0x70000011;
pub const DT_MIPS_UNREFEXTNO: u32 = 0x70000012;
pub const DT_MIPS_GOTSYM: u32 = 0x70000013;
pub const DT_MIPS_HIPAGENO: u32 = 0x70000014;
pub const DT_MIPS_RLD_MAP: u32 = 0x70000016;

pub const R_MIPS_NONE: u32 = 0; pub const R_MIPS_16: u32 = 1; pub const R_MIPS_32: u32 = 2;
pub const R_MIPS_REL32: u32 = 3; pub const R_MIPS_26: u32 = 4; pub const R_MIPS_HI16: u32 = 5;
pub const R_MIPS_LO16: u32 = 6; pub const R_MIPS_GPREL16: u32 = 7; pub const R_MIPS_LITERAL: u32 = 8;
pub const R_MIPS_GOT16: u32 = 9; pub const R_MIPS_PC16: u32 = 10; pub const R_MIPS_CALL16: u32 = 11;
pub const R_MIPS_GPREL32: u32 = 12; pub const R_MIPS_UNUSED1: u32 = 13; pub const R_MIPS_UNUSED2: u32 = 14;
pub const R_MIPS_UNUSED3: u32 = 15; pub const R_MIPS_SHIFT5: u32 = 16; pub const R_MIPS_SHIFT6: u32 = 17;
pub const R_MIPS_64: u32 = 18; pub const R_MIPS_GOT_DISP: u32 = 19; pub const R_MIPS_GOT_PAGE: u32 = 20;
pub const R_MIPS_GOT_OFST: u32 = 21; pub const R_MIPS_GOTHI16: u32 = 22; pub const R_MIPS_GOTLO16: u32 = 23;
pub const R_MIPS_SUB: u32 = 24; pub const R_MIPS_INSERT_A: u32 = 25; pub const R_MIPS_INSERT_B: u32 = 26;
pub const R_MIPS_DELETE: u32 = 27; pub const R_MIPS_HIGHER: u32 = 28; pub const R_MIPS_HIGHEST: u32 = 29;
pub const R_MIPS_CALLHI16: u32 = 30; pub const R_MIPS_CALLLO16: u32 = 31;
pub const R_MIPS_PC21_S2: u32 = 60; pub const R_MIPS_PC26_S2: u32 = 61;
pub const R_MIPS_LOVENDOR: u32 = 100; pub const R_MIPS_HIVENDOR: u32 = 127; pub const R_MIPS_PC32: u32 = 248;

pub const SHN_MIPS_ACCOMON: u32 = 0xff00; pub const SHN_MIPS_TEXT: u32 = 0xff01;
pub const SHN_MIPS_DATA: u32 = 0xff02; pub const SHN_MIPS_SCOMMON: u32 = 0xff03; pub const SHN_MIPS_SUNDEFINED: u32 = 0xff04;
pub const SHT_MIPS_LIST: u32 = 0x70000000; pub const SHT_MIPS_CONFLICT: u32 = 0x70000002;
pub const SHT_MIPS_GPTAB: u32 = 0x70000003; pub const SHT_MIPS_UCODE: u32 = 0x70000004;
pub const SHT_MIPS_DEBUG: u32 = 0x70000005; pub const SHT_MIPS_REGINFO: u32 = 0x70000006;
pub const SHT_MIPS_PACKAGE: u32 = 0x70000007; pub const SHT_MIPS_PACKSYM: u32 = 0x70000008;
pub const SHT_MIPS_RELD: u32 = 0x70000009; pub const SHT_MIPS_IFACE: u32 = 0x7000000b;
pub const SHT_MIPS_CONTENT: u32 = 0x7000000c; pub const SHT_MIPS_OPTIONS: u32 = 0x7000000d;
pub const SHT_MIPS_SHDR: u32 = 0x70000010; pub const SHT_MIPS_FDESC: u32 = 0x70000011;
pub const SHT_MIPS_EXTSYM: u32 = 0x70000012; pub const SHT_MIPS_DENSE: u32 = 0x70000013;
pub const SHT_MIPS_PDESC: u32 = 0x70000014; pub const SHT_MIPS_LOCSYM: u32 = 0x70000015;
pub const SHT_MIPS_AUXSYM: u32 = 0x70000016; pub const SHT_MIPS_OPTSYM: u32 = 0x70000017;
pub const SHT_MIPS_LOCSTR: u32 = 0x70000018; pub const SHT_MIPS_LINE: u32 = 0x70000019;
pub const SHT_MIPS_RFDESC: u32 = 0x7000001a; pub const SHT_MIPS_DELTASYM: u32 = 0x7000001b;
pub const SHT_MIPS_DELTAINST: u32 = 0x7000001c; pub const SHT_MIPS_DELTACLASS: u32 = 0x7000001d;
pub const SHT_MIPS_DWARF: u32 = 0x7000001e; pub const SHT_MIPS_DELTADECL: u32 = 0x7000001f;
pub const SHT_MIPS_SYMBOL_LIB: u32 = 0x70000020; pub const SHT_MIPS_EVENTS: u32 = 0x70000021;
pub const SHT_MIPS_TRANSLATE: u32 = 0x70000022; pub const SHT_MIPS_PIXIE: u32 = 0x70000023;
pub const SHT_MIPS_XLATE: u32 = 0x70000024; pub const SHT_MIPS_XLATE_DEBUG: u32 = 0x70000025;
pub const SHT_MIPS_WHIRL: u32 = 0x70000026; pub const SHT_MIPS_EH_REGION: u32 = 0x70000027;
pub const SHT_MIPS_XLATE_OLD: u32 = 0x70000028; pub const SHT_MIPS_PDR_EXCEPTION: u32 = 0x70000029;
pub const SHF_MIPS_GPREL: u32 = 0x10000000; pub const SHF_MIPS_MERGE: u32 = 0x20000000;
pub const SHF_MIPS_ADDR: u32 = 0x40000000; pub const SHF_MIPS_STRING: u32 = 0x80000000;
pub const SHF_MIPS_NOSTRIP: u32 = 0x08000000; pub const SHF_MIPS_LOCAL: u32 = 0x04000000;
pub const SHF_MIPS_NAMES: u32 = 0x02000000; pub const SHF_MIPS_NODUPES: u32 = 0x01000000;

pub const MIPS_ABI_FP_ANY: i32 = 0; pub const MIPS_ABI_FP_DOUBLE: i32 = 1; pub const MIPS_ABI_FP_SINGLE: i32 = 2;
pub const MIPS_ABI_FP_SOFT: i32 = 3; pub const MIPS_ABI_FP_OLD_64: i32 = 4; pub const MIPS_ABI_FP_XX: i32 = 5;
pub const MIPS_ABI_FP_64: i32 = 6; pub const MIPS_ABI_FP_64A: i32 = 7;

#[repr(C)]
pub struct mips_elf_abiflags_v0 { pub version: u16, pub isa_level: u8, pub isa_rev: u8, pub gpr_size: u8, pub cpr1_size: u8, pub cpr2_size: u8, pub fp_abi: u8, pub isa_ext: u32, pub ases: u32, pub flags1: u32, pub flags2: u32 }
pub const ELF_NGREG: usize = 45; pub const ELF_NFPREG: usize = 33;
pub type elf_greg_t = ::core::ffi::c_ulong; pub type elf_gregset_t = [elf_greg_t; ELF_NGREG];
pub type elf_fpreg_t = f64; pub type elf_fpregset_t = [elf_fpreg_t; ELF_NFPREG];

extern "C" { pub fn mips_dump_regs32(uregs: *mut u32, regs: *const pt_regs); pub fn mips_dump_regs64(uregs: *mut u64, regs: *const pt_regs); }
#[repr(C)] pub struct pt_regs { _private: [u8; 0] }

pub const ELF_ARCH: u32 = EM_MIPS;
pub const __MIPS_O32_FP64_MUST_BE_ZERO: u32 = EF_MIPS_FP64;
pub const CORE_DUMP_USE_REGSET: bool = true;
pub const ELF_EXEC_PAGESIZE: usize = PAGE_SIZE;
pub const ARCH_HAS_SETUP_ADDITIONAL_PAGES: i32 = 1;
pub const MIPS_ABI_FP_UNKNOWN: i32 = -1;

extern "C" {
    pub static mut elf_hwcap: u32;
    pub static __elf_platform: *const ::core::ffi::c_char;
    pub static __elf_base_platform: *const ::core::ffi::c_char;
    pub static mut mips_use_nan_legacy: bool;
    pub static mut mips_use_nan_2008: bool;
    pub fn arch_setup_additional_pages(bprm: *mut linux_binprm, uses_interp: i32) -> i32;
    pub fn arch_elf_pt_proc(ehdr: *mut ::core::ffi::c_void, phdr: *mut ::core::ffi::c_void, elf: *mut file, is_interp: bool, state: *mut arch_elf_state) -> i32;
    pub fn arch_check_elf(ehdr: *mut ::core::ffi::c_void, has_interpreter: bool, interp_ehdr: *mut ::core::ffi::c_void, state: *mut arch_elf_state) -> i32;
    pub fn mips_set_personality_nan(state: *mut arch_elf_state);
    pub fn mips_set_personality_fp(state: *mut arch_elf_state);
    pub fn mips_elf_read_implies_exec(elf_ex: *mut ::core::ffi::c_void, exstack: i32) -> i32;
    pub static mut mips_abi: mips_abi;
    pub static mut mips_abi_32: mips_abi;
    pub static mut mips_abi_n32: mips_abi;
}
#[repr(C)] pub struct mips_abi { _private: [u8; 0] }
#[repr(C)] pub struct linux_binprm { _private: [u8; 0] }
#[repr(C)] pub struct file { _private: [u8; 0] }
#[repr(C)] pub struct arch_elf_state { pub nan_2008: i32, pub fp_abi: i32, pub interp_fp_abi: i32, pub overall_fp_mode: i32 }
pub const PAGE_SIZE: usize = 4096;
pub const EM_MIPS: u32 = 8;

pub const ELFCLASS32: u8 = 1; pub const ELFCLASS64: u8 = 2;
pub const EI_CLASS: usize = 4; pub const READ_IMPLIES_EXEC: usize = 0x0400000;
pub const PER_LINUX: u32 = 0; pub const PER_LINUX32: u32 = 8;

/* Build-time configuration selects these aliases in the original header. */
#[macro_export] macro_rules! mips_elf_check_machine { ($x:expr) => { ($x).e_machine == $crate::elf_header::EM_MIPS }; }
#[macro_export] macro_rules! vmcore_elf32_check_arch { ($x:expr) => { mips_elf_check_machine!($x) }; }
#[macro_export] macro_rules! vmcore_elf64_check_arch { ($x:expr) => { mips_elf_check_machine!($x) }; }
#[macro_export] macro_rules! elf_read_implies_exec { ($ex:expr, $stk:expr) => { unsafe { mips_elf_read_implies_exec(&mut $ex as *mut _, $stk) } }; }

#[macro_export] macro_rules! ELF_PLAT_INIT {
    ($r:expr, $load_addr:expr) => {{
        $r.regs[1] = 0; $r.regs[2] = 0; $r.regs[3] = 0; $r.regs[4] = 0;
        $r.regs[5] = 0; $r.regs[6] = 0; $r.regs[7] = 0; $r.regs[8] = 0;
        $r.regs[9] = 0; $r.regs[10] = 0; $r.regs[11] = 0; $r.regs[12] = 0;
        $r.regs[13] = 0; $r.regs[14] = 0; $r.regs[15] = 0; $r.regs[16] = 0;
        $r.regs[17] = 0; $r.regs[18] = 0; $r.regs[19] = 0; $r.regs[20] = 0;
        $r.regs[21] = 0; $r.regs[22] = 0; $r.regs[23] = 0; $r.regs[24] = 0;
        $r.regs[25] = 0; $r.regs[26] = 0; $r.regs[27] = 0; $r.regs[28] = 0;
        $r.regs[30] = 0; $r.regs[31] = 0;
    }};
}

pub const ELF_ET_DYN_BASE: usize = (TASK_SIZE / 3) * 2;
pub const TASK_SIZE: usize = 0;
#[macro_export] macro_rules! INIT_ARCH_ELF_STATE { () => { $crate::elf_header::arch_elf_state { nan_2008: -1, fp_abi: $crate::elf_header::MIPS_ABI_FP_UNKNOWN, interp_fp_abi: $crate::elf_header::MIPS_ABI_FP_UNKNOWN, overall_fp_mode: -1 } }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
