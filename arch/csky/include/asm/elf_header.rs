/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding architecture headers:
// asm/ptrace.h, abi/regdef.h, and abi/elf.h.

pub const ELF_ARCH: u32 = EM_CSKY;
pub const EM_CSKY_OLD: u32 = 39;

/* CSKY Relocations */
pub const R_CSKY_NONE: u32 = 0;
pub const R_CSKY_32: u32 = 1;
pub const R_CSKY_PCIMM8BY4: u32 = 2;
pub const R_CSKY_PCIMM11BY2: u32 = 3;
pub const R_CSKY_PCIMM4BY2: u32 = 4;
pub const R_CSKY_PC32: u32 = 5;
pub const R_CSKY_PCRELJSR_IMM11BY2: u32 = 6;
pub const R_CSKY_GNU_VTINHERIT: u32 = 7;
pub const R_CSKY_GNU_VTENTRY: u32 = 8;
pub const R_CSKY_RELATIVE: u32 = 9;
pub const R_CSKY_COPY: u32 = 10;
pub const R_CSKY_GLOB_DAT: u32 = 11;
pub const R_CSKY_JUMP_SLOT: u32 = 12;
pub const R_CSKY_ADDR_HI16: u32 = 24;
pub const R_CSKY_ADDR_LO16: u32 = 25;
pub const R_CSKY_PCRELJSR_IMM26BY2: u32 = 40;

pub type elf_greg_t = ::core::ffi::c_ulong;
pub type elf_fpregset_t = user_fp;

/*
 * In gdb/bfd elf32-csky.c, csky_elf_grok_prstatus() use fixed size of
 * elf_prstatus. It's 148 for abiv1 and 220 for abiv2, the size is enough
 * for coredump and no need full sizeof(struct pt_regs).
 */
pub const ELF_NGREG: usize =
    (::core::mem::size_of::<pt_regs>() / ::core::mem::size_of::<elf_greg_t>()) - 2;

pub type elf_gregset_t = [elf_greg_t; ELF_NGREG];

/* This is used to ensure we don't load something for the wrong architecture. */
#[inline]
pub unsafe fn elf_check_arch(x: *const Elf32_Ehdr) -> bool {
    (*x).e_machine == ELF_ARCH || (*x).e_machine == EM_CSKY_OLD
}

/* These are used to set parameters in the core dumps. */
pub const ELF_EXEC_PAGESIZE: u32 = 4096;
pub const ELF_CLASS: u8 = ELFCLASS32;

#[macro_export]
macro_rules! ELF_PLAT_INIT {
    ($r:expr, $load_addr:expr) => {{
        unsafe { (*$r).a0 = 0; }
    }};
}

#[cfg(__cskyBE__)]
pub const ELF_DATA: u8 = ELFDATA2MSB;
#[cfg(not(__cskyBE__))]
pub const ELF_DATA: u8 = ELFDATA2LSB;

/* This is the location that an ET_DYN program is loaded if exec'ed. */
pub const ELF_ET_DYN_BASE: ::core::ffi::c_ulong = 0x0;

/* Similar, but for a thread other than current. */
#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn dump_task_regs(tsk: *mut task_struct, elf_regs: *mut elf_gregset_t) -> ::core::ffi::c_int;
}

#[inline]
pub unsafe fn ELF_CORE_COPY_TASK_REGS(
    tsk: *mut task_struct,
    elf_regs: *mut elf_gregset_t,
) -> ::core::ffi::c_int {
    dump_task_regs(tsk, elf_regs)
}

pub const ELF_HWCAP: u32 = 0;

/*
 * This yields a string that ld.so will use to load implementation specific
 * libraries for optimization. This is more specific in intent than poking
 * at uname or /proc/cpuinfo.
 */
pub const ELF_PLATFORM: *const ::core::ffi::c_void = ::core::ptr::null();

#[inline]
pub unsafe fn SET_PERSONALITY(_ex: usize) {
    set_personality(PER_LINUX);
}

pub const ARCH_HAS_SETUP_ADDITIONAL_PAGES: u32 = 1;

#[repr(C)]
pub struct linux_binprm {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn arch_setup_additional_pages(
        bprm: *mut linux_binprm,
        uses_interp: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
