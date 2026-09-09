/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of x86/include/asm/elf.h. */

pub type ElfGregT = core::ffi::c_ulong;
pub const ELF_NGREG: usize = core::mem::size_of::<UserRegsStruct>() / core::mem::size_of::<ElfGregT>();
pub type ElfGregsetT = [ElfGregT; ELF_NGREG];
pub type ElfFpregsetT = UserI387Struct;

#[cfg(target_arch = "x86")]
pub const R_386_NONE: i32 = 0;
#[cfg(target_arch = "x86")]
pub const R_386_32: i32 = 1;
#[cfg(target_arch = "x86")]
pub const R_386_PC32: i32 = 2;
#[cfg(target_arch = "x86")]
pub const R_386_GOT32: i32 = 3;
#[cfg(target_arch = "x86")]
pub const R_386_PLT32: i32 = 4;
#[cfg(target_arch = "x86")]
pub const R_386_COPY: i32 = 5;
#[cfg(target_arch = "x86")]
pub const R_386_GLOB_DAT: i32 = 6;
#[cfg(target_arch = "x86")]
pub const R_386_JMP_SLOT: i32 = 7;
#[cfg(target_arch = "x86")]
pub const R_386_RELATIVE: i32 = 8;
#[cfg(target_arch = "x86")]
pub const R_386_GOTOFF: i32 = 9;
#[cfg(target_arch = "x86")]
pub const R_386_GOTPC: i32 = 10;
#[cfg(target_arch = "x86")]
pub const R_386_NUM: i32 = 11;

#[cfg(target_arch = "x86_64")]
pub const R_X86_64_NONE: i32 = 0;
#[cfg(target_arch = "x86_64")]
pub const R_X86_64_64: i32 = 1;
#[cfg(target_arch = "x86_64")]
pub const R_X86_64_PC32: i32 = 2;
#[cfg(target_arch = "x86_64")]
pub const R_X86_64_GOT32: i32 = 3;
#[cfg(target_arch = "x86_64")]
pub const R_X86_64_PLT32: i32 = 4;
#[cfg(target_arch = "x86_64")]
pub const R_X86_64_COPY: i32 = 5;
#[cfg(target_arch = "x86_64")]
pub const R_X86_64_GLOB_DAT: i32 = 6;
#[cfg(target_arch = "x86_64")]
pub const R_X86_64_JUMP_SLOT: i32 = 7;
#[cfg(target_arch = "x86_64")]
pub const R_X86_64_RELATIVE: i32 = 8;
#[cfg(target_arch = "x86_64")]
pub const R_X86_64_GOTPCREL: i32 = 9;
#[cfg(target_arch = "x86_64")]
pub const R_X86_64_32: i32 = 10;
#[cfg(target_arch = "x86_64")]
pub const R_X86_64_32S: i32 = 11;
#[cfg(target_arch = "x86_64")]
pub const R_X86_64_16: i32 = 12;
#[cfg(target_arch = "x86_64")]
pub const R_X86_64_PC16: i32 = 13;
#[cfg(target_arch = "x86_64")]
pub const R_X86_64_8: i32 = 14;
#[cfg(target_arch = "x86_64")]
pub const R_X86_64_PC8: i32 = 15;
#[cfg(target_arch = "x86_64")]
pub const R_X86_64_PC64: i32 = 24;

extern "C" {
    pub static mut vdso64_enabled: u32;
    pub static mut vdso32_enabled: u32;
    pub static mut elf_hwcap2: u32;
    pub static mut va_align: VaAlignment;
    pub fn compat_start_thread(regs: *mut PtRegs, new_ip: u32, new_sp: u32, x32: bool);
    pub fn set_personality_ia32(value: bool);
    pub fn set_personality_64bit();
    pub fn arch_setup_additional_pages(bprm: *mut LinuxBinprm, uses_interp: i32) -> i32;
    pub fn compat_arch_setup_additional_pages(bprm: *mut LinuxBinprm, uses_interp: i32, x32: bool) -> i32;
    pub fn arch_syscall_is_vdso_sigreturn(regs: *mut PtRegs) -> bool;
    pub fn task_size_32bit() -> core::ffi::c_ulong;
    pub fn task_size_64bit(full_addr_space: i32) -> core::ffi::c_ulong;
    pub fn get_mmap_base(is_legacy: i32) -> core::ffi::c_ulong;
    pub fn mmap_address_hint_valid(addr: core::ffi::c_ulong, len: core::ffi::c_ulong) -> bool;
    pub fn get_sigframe_size() -> core::ffi::c_ulong;
    pub fn mmap_is_ia32() -> i32;
}

#[repr(C)]
pub struct UserRegsStruct;
pub struct UserI387Struct;
pub struct PtRegs;
pub struct ThreadStruct;
pub struct LinuxBinprm;
pub struct TaskStruct;

#[cfg(target_arch = "x86_64")]
#[inline]
pub unsafe fn elf_check_arch_ia32(x: *const ElfHeader) -> bool {
    (*x).e_machine == EM_386 || (*x).e_machine == EM_486
}

pub const CORE_DUMP_USE_REGSET: bool = true;
pub const ELF_EXEC_PAGESIZE: usize = 4096;
pub const AT_SYSINFO: u32 = 32;

#[inline]
pub unsafe fn elf_read_implies_exec(executable_stack: i32) -> bool {
    mmap_is_ia32() != 0 && executable_stack == EXSTACK_DEFAULT
}

#[cfg(target_arch = "x86_64")]
pub const STACK_RND_MASK_32BIT: usize = 0x7ff;
#[cfg(target_arch = "x86_64")]
pub const STACK_RND_MASK_64BIT: usize = 0x3fffff;
#[cfg(target_arch = "x86")]
pub const STACK_RND_MASK: usize = 0x7ff;

pub const ELF_HWCAP2: usize = 0; // (elf_hwcap2); supplied by the kernel dependency.
pub const ARCH_HAS_SETUP_ADDITIONAL_PAGES: i32 = 1;
pub const ALIGN_VA_32: i32 = 1 << 0;
pub const ALIGN_VA_64: i32 = 1 << 1;

#[repr(C)]
pub struct VaAlignment {
    pub flags: i32,
    pub mask: core::ffi::c_ulong,
    pub bits: core::ffi::c_ulong,
}

/* External types and constants below are supplied by the included kernel headers. */
extern "C" {
    pub static mut force_personality32: i32;
}

pub const EXSTACK_DEFAULT: i32 = 0;
pub const EM_386: u16 = 3;
pub const EM_486: u16 = 6;
pub const EM_X86_64: u16 = 62;

#[repr(C)]
pub struct ElfHeader {
    pub e_machine: u16,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
