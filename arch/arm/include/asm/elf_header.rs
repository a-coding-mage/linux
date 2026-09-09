/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding ARM headers:
// asm/auxvec.h, asm/hwcap.h, asm/ptrace.h, and asm/user.h.

pub struct pt_regs;
pub struct user_fp;
pub struct elf32_hdr {
    pub e_ident: [u8; 16],
    pub e_flags: u32,
}
pub struct linux_binprm;

/*
 * ELF register definitions..
 */
pub type elf_greg_t = ::core::ffi::c_ulong;
pub type elf_freg_t = [::core::ffi::c_ulong; 3];

pub const ELF_NGREG: usize = core::mem::size_of::<pt_regs>() / core::mem::size_of::<elf_greg_t>();
pub type elf_gregset_t = [elf_greg_t; ELF_NGREG];
pub type elf_fpregset_t = user_fp;

pub const EF_ARM_EABI_MASK: u32 = 0xff000000;
pub const EF_ARM_EABI_UNKNOWN: u32 = 0x00000000;
pub const EF_ARM_EABI_VER1: u32 = 0x01000000;
pub const EF_ARM_EABI_VER2: u32 = 0x02000000;
pub const EF_ARM_EABI_VER3: u32 = 0x03000000;
pub const EF_ARM_EABI_VER4: u32 = 0x04000000;
pub const EF_ARM_EABI_VER5: u32 = 0x05000000;

pub const EF_ARM_BE8: u32 = 0x00800000; // ABI 4,5
pub const EF_ARM_LE8: u32 = 0x00400000; // ABI 4,5
pub const EF_ARM_MAVERICK_FLOAT: u32 = 0x00000800; // ABI 0
pub const EF_ARM_VFP_FLOAT: u32 = 0x00000400; // ABI 0
pub const EF_ARM_SOFT_FLOAT: u32 = 0x00000200; // ABI 0
pub const EF_ARM_OLD_ABI: u32 = 0x00000100; // ABI 0
pub const EF_ARM_NEW_ABI: u32 = 0x00000080; // ABI 0
pub const EF_ARM_ALIGN8: u32 = 0x00000040; // ABI 0
pub const EF_ARM_PIC: u32 = 0x00000020; // ABI 0
pub const EF_ARM_MAPSYMSFIRST: u32 = 0x00000010; // ABI 2
pub const EF_ARM_APCS_FLOAT: u32 = 0x00000010; // ABI 0, floats in fp regs
pub const EF_ARM_DYNSYMSUSESEGIDX: u32 = 0x00000008; // ABI 2
pub const EF_ARM_APCS_26: u32 = 0x00000008; // ABI 0
pub const EF_ARM_SYMSARESORTED: u32 = 0x00000004; // ABI 1,2
pub const EF_ARM_INTERWORK: u32 = 0x00000004; // ABI 0
pub const EF_ARM_HASENTRY: u32 = 0x00000002; // All
pub const EF_ARM_RELEXEC: u32 = 0x00000001; // All

pub const R_ARM_NONE: i32 = 0;
pub const R_ARM_PC24: i32 = 1;
pub const R_ARM_ABS32: i32 = 2;
pub const R_ARM_REL32: i32 = 3;
pub const R_ARM_CALL: i32 = 28;
pub const R_ARM_JUMP24: i32 = 29;
pub const R_ARM_TARGET1: i32 = 38;
pub const R_ARM_V4BX: i32 = 40;
pub const R_ARM_PREL31: i32 = 42;
pub const R_ARM_MOVW_ABS_NC: i32 = 43;
pub const R_ARM_MOVT_ABS: i32 = 44;
pub const R_ARM_MOVW_PREL_NC: i32 = 45;
pub const R_ARM_MOVT_PREL: i32 = 46;
pub const R_ARM_ALU_PC_G0_NC: i32 = 57;
pub const R_ARM_ALU_PC_G1_NC: i32 = 59;
pub const R_ARM_LDR_PC_G2: i32 = 63;
pub const R_ARM_THM_CALL: i32 = 10;
pub const R_ARM_THM_JUMP24: i32 = 30;
pub const R_ARM_THM_MOVW_ABS_NC: i32 = 47;
pub const R_ARM_THM_MOVT_ABS: i32 = 48;
pub const R_ARM_THM_MOVW_PREL_NC: i32 = 49;
pub const R_ARM_THM_MOVT_PREL: i32 = 50;

/* These are used to set parameters in the core dumps. */
pub const ELF_CLASS: u32 = ELFCLASS32;
// __ARMEB__ selects ELFDATA2MSB; otherwise ELFDATA2LSB is selected.
pub const ELF_DATA: u32 = ELFDATA2LSB;
pub const ELF_ARCH: u32 = EM_ARM;

pub const ELF_PLATFORM_SIZE: usize = 8;
extern "C" {
    pub static mut elf_platform: [::core::ffi::c_char; ELF_PLATFORM_SIZE];
    pub fn elf_check_arch(x: *const elf32_hdr) -> ::core::ffi::c_int;
}
pub const ELFOSABI_ARM_FDPIC: u8 = 65; // ARM FDPIC platform
pub const ELF_FDPIC_CORE_EFLAGS: u32 = 0;

#[inline]
pub unsafe fn elf_check_fdpic(x: *const elf32_hdr) -> bool {
    (*x).e_ident[EI_OSABI as usize] == ELFOSABI_ARM_FDPIC
}

#[inline]
pub unsafe fn elf_check_const_displacement(x: *const elf32_hdr) -> u32 {
    (*x).e_flags & EF_ARM_PIC
}

#[inline]
pub const fn vmcore_elf64_check_arch<T>(_: T) -> i32 { 0 }

extern "C" {
    pub fn arm_elf_read_implies_exec(stk: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn elf_set_personality(ex: *const elf32_hdr);
    pub fn arch_setup_additional_pages(bprm: *mut linux_binprm, uses_interp: ::core::ffi::c_int)
        -> ::core::ffi::c_int;
}

#[inline]
pub unsafe fn elf_read_implies_exec(_ex: *const core::ffi::c_void, stk: ::core::ffi::c_int) -> ::core::ffi::c_int {
    arm_elf_read_implies_exec(stk)
}

#[inline]
pub unsafe fn SET_PERSONALITY(ex: *const elf32_hdr) {
    elf_set_personality(ex);
}

pub const CORE_DUMP_USE_REGSET: bool = true;
pub const ELF_EXEC_PAGESIZE: usize = 4096;
pub const ELF_ET_DYN_BASE: ::core::ffi::c_ulong = 0x400000;

/* When the program starts, a1 contains a pointer to a function to be
   registered with atexit, as per the SVR4 ABI.  A value of 0 means we
   have no such handler. */
#[inline]
pub unsafe fn ELF_PLAT_INIT(r: *mut pt_regs, _load_addr: usize) {
    (*r).ARM_r0 = 0;
}

#[inline]
pub unsafe fn ELF_FDPIC_PLAT_INIT(
    r: *mut pt_regs,
    exec_map_addr: usize,
    interp_map_addr: usize,
    dynamic_addr: usize,
) {
    (*r).ARM_r7 = exec_map_addr;
    (*r).ARM_r8 = interp_map_addr;
    (*r).ARM_r9 = dynamic_addr;
}

// CONFIG_MMU and CONFIG_VDSO provide ARCH_HAS_SETUP_ADDITIONAL_PAGES,
// linux_binprm, arch_setup_additional_pages, and ARCH_DLINFO when enabled.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
