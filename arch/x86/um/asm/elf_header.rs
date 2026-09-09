/*
 * Copyright (C) 2000 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
 * Licensed under the GPL
 */

// Dependencies supplied by the surrounding architecture headers:
// asm::user, skas, EM_*, ELFCLASS*, ELFDATA2LSB, TASK_SIZE, and register macros.

pub const CORE_DUMP_USE_REGSET: bool = true;

// The C header selects this branch with CONFIG_X86_32.
#[cfg(feature = "CONFIG_X86_32")]
mod x86_32 {
    pub const R_386_NONE: u32 = 0;
    pub const R_386_32: u32 = 1;
    pub const R_386_PC32: u32 = 2;
    pub const R_386_GOT32: u32 = 3;
    pub const R_386_PLT32: u32 = 4;
    pub const R_386_COPY: u32 = 5;
    pub const R_386_GLOB_DAT: u32 = 6;
    pub const R_386_JMP_SLOT: u32 = 7;
    pub const R_386_RELATIVE: u32 = 8;
    pub const R_386_GOTOFF: u32 = 9;
    pub const R_386_GOTPC: u32 = 10;
    pub const R_386_NUM: u32 = 11;

    pub const ELF_CLASS: u32 = ELFCLASS32;
    pub const ELF_DATA: u32 = ELFDATA2LSB;
    pub const ELF_ARCH: u32 = EM_386;
    pub const ELF_PLATFORM_FALLBACK: &str = "i586";

    macro_rules! elf_check_arch { ($x:expr) => { (($x).e_machine == EM_386) || (($x).e_machine == EM_486) }; }
    macro_rules! ELF_PLAT_INIT { ($regs:expr, $load_addr:expr) => {{ PT_REGS_BX!($regs) = 0; PT_REGS_CX!($regs) = 0; PT_REGS_DX!($regs) = 0; PT_REGS_SI!($regs) = 0; PT_REGS_DI!($regs) = 0; PT_REGS_BP!($regs) = 0; PT_REGS_AX!($regs) = 0; }}; }
    macro_rules! ELF_CORE_COPY_REGS { ($pr_reg:expr, $regs:expr) => {{
        $pr_reg[0] = PT_REGS_BX!($regs); $pr_reg[1] = PT_REGS_CX!($regs); $pr_reg[2] = PT_REGS_DX!($regs); $pr_reg[3] = PT_REGS_SI!($regs);
        $pr_reg[4] = PT_REGS_DI!($regs); $pr_reg[5] = PT_REGS_BP!($regs); $pr_reg[6] = PT_REGS_AX!($regs); $pr_reg[7] = PT_REGS_DS!($regs);
        $pr_reg[8] = PT_REGS_ES!($regs); $pr_reg[9] = PT_REGS_DS!($regs); $pr_reg[10] = PT_REGS_DS!($regs); $pr_reg[11] = PT_REGS_SYSCALL_NR!($regs);
        $pr_reg[12] = PT_REGS_IP!($regs); $pr_reg[13] = PT_REGS_CS!($regs); $pr_reg[14] = PT_REGS_EFLAGS!($regs); $pr_reg[15] = PT_REGS_SP!($regs); $pr_reg[16] = PT_REGS_SS!($regs);
    }}; }
}

#[cfg(not(feature = "CONFIG_X86_32"))]
mod x86_64 {
    pub const R_X86_64_NONE: u32 = 0; pub const R_X86_64_64: u32 = 1; pub const R_X86_64_PC32: u32 = 2; pub const R_X86_64_GOT32: u32 = 3;
    pub const R_X86_64_PLT32: u32 = 4; pub const R_X86_64_COPY: u32 = 5; pub const R_X86_64_GLOB_DAT: u32 = 6; pub const R_X86_64_JUMP_SLOT: u32 = 7;
    pub const R_X86_64_RELATIVE: u32 = 8; pub const R_X86_64_GOTPCREL: u32 = 9; pub const R_X86_64_32: u32 = 10; pub const R_X86_64_32S: u32 = 11;
    pub const R_X86_64_16: u32 = 12; pub const R_X86_64_PC16: u32 = 13; pub const R_X86_64_8: u32 = 14; pub const R_X86_64_PC8: u32 = 15; pub const R_X86_64_PC64: u32 = 24;
    pub const ELF_CLASS: u32 = ELFCLASS64; pub const ELF_DATA: u32 = ELFDATA2LSB; pub const ELF_ARCH: u32 = EM_X86_64;
    pub const ELF_PLATFORM_FALLBACK: &str = "x86_64";
    pub const ARCH_HAS_SETUP_ADDITIONAL_PAGES: i32 = 1;
    pub const AT_SYSINFO_EHDR: i32 = 33;
    pub const ELF_EXEC_PAGESIZE: usize = 4096;
    extern "C" { pub fn arch_setup_additional_pages(bprm: *mut linux_binprm, uses_interp: i32) -> i32; pub static mut um_vdso_addr: c_ulong; }
    pub struct linux_binprm;
    macro_rules! elf_check_arch { ($x:expr) => { ($x).e_machine == EM_X86_64 }; }
}

pub type elf_greg_t = c_ulong;
pub const ELF_NGREG: usize = core::mem::size_of::<user_regs_struct>() / core::mem::size_of::<elf_greg_t>();
pub type elf_gregset_t = [elf_greg_t; ELF_NGREG];
pub type elf_fpregset_t = user_i387_struct;
pub struct task_struct;
pub const ELF_ET_DYN_BASE: usize = TASK_SIZE / 3 * 2;
extern "C" { pub static mut elf_aux_hwcap: c_long; pub static mut elf_aux_platform: *mut c_char; }

macro_rules! ELF_HWCAP { () => { elf_aux_hwcap }; }
macro_rules! ELF_PLATFORM { () => { if !elf_aux_platform.is_null() { elf_aux_platform } else { ELF_PLATFORM_FALLBACK.as_ptr() as *mut c_char } }; }
macro_rules! SET_PERSONALITY { ($ex:expr) => {{ let _ = &$ex; }}; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
