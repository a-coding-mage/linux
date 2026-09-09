/* SPDX-License-Identifier: GPL-2.0 */

/*
 * ELF register definitions.
 *
 * The C header includes asm/ptrace.h and asm/user.h; the corresponding types
 * and functions are supplied by those dependencies.
 */

/* 68k ELF relocation types */
pub const R_68K_NONE: i32 = 0;
pub const R_68K_32: i32 = 1;
pub const R_68K_16: i32 = 2;
pub const R_68K_8: i32 = 3;
pub const R_68K_PC32: i32 = 4;
pub const R_68K_PC16: i32 = 5;
pub const R_68K_PC8: i32 = 6;
pub const R_68K_GOT32: i32 = 7;
pub const R_68K_GOT16: i32 = 8;
pub const R_68K_GOT8: i32 = 9;
pub const R_68K_GOT32O: i32 = 10;
pub const R_68K_GOT16O: i32 = 11;
pub const R_68K_GOT8O: i32 = 12;
pub const R_68K_PLT32: i32 = 13;
pub const R_68K_PLT16: i32 = 14;
pub const R_68K_PLT8: i32 = 15;
pub const R_68K_PLT32O: i32 = 16;
pub const R_68K_PLT16O: i32 = 17;
pub const R_68K_PLT8O: i32 = 18;
pub const R_68K_COPY: i32 = 19;
pub const R_68K_GLOB_DAT: i32 = 20;
pub const R_68K_JMP_SLOT: i32 = 21;
pub const R_68K_RELATIVE: i32 = 22;

pub type elf_greg_t = ::core::ffi::c_ulong;
pub const ELF_NGREG: usize = ::core::mem::size_of::<user_regs_struct>() / ::core::mem::size_of::<elf_greg_t>();
pub type elf_gregset_t = [elf_greg_t; ELF_NGREG];
pub type elf_fpregset_t = user_m68kfp_struct;

/* Ensure that an object is for the correct architecture. */
#[macro_export]
macro_rules! elf_check_arch {
    ($x:expr) => {{ ($x).e_machine == EM_68K }};
}

/* Parameters used in core dumps. */
pub const ELF_CLASS: _ = ELFCLASS32;
pub const ELF_DATA: _ = ELFDATA2MSB;
pub const ELF_ARCH: _ = EM_68K;

/* The function pointer registered with atexit is passed in %a1. */
#[macro_export]
macro_rules! ELF_PLAT_INIT {
    ($r:expr, $load_addr:expr) => {{
        unsafe { (*$r).a1 = 0 };
    }};
}

#[macro_export]
macro_rules! ELF_FDPIC_PLAT_INIT {
    ($r:expr, $exec_map_addr:expr, $interp_map_addr:expr, $dynamic_addr:expr) => {{
        unsafe {
            (*$r).d3 = $exec_map_addr;
            (*$r).d4 = $interp_map_addr;
            (*$r).d5 = $dynamic_addr;
        }
    }};
}

/* CONFIG_SUN3 or CONFIG_COLDFIRE selects the 8192-byte page size. */
#[cfg(any(feature = "CONFIG_SUN3", feature = "CONFIG_COLDFIRE"))]
pub const ELF_EXEC_PAGESIZE: usize = 8192;
#[cfg(not(any(feature = "CONFIG_SUN3", feature = "CONFIG_COLDFIRE")))]
pub const ELF_EXEC_PAGESIZE: usize = 4096;

/* CONFIG_SUN3 selects the alternate ET_DYN load base. */
#[cfg(not(feature = "CONFIG_SUN3"))]
pub const ELF_ET_DYN_BASE: ::core::ffi::c_ulong = 0xD0000000;
#[cfg(feature = "CONFIG_SUN3")]
pub const ELF_ET_DYN_BASE: ::core::ffi::c_ulong = 0x0D800000;

#[macro_export]
macro_rules! ELF_CORE_COPY_REGS {
    ($pr_reg:expr, $regs:expr) => {{
        unsafe {
            ($pr_reg)[0] = (*$regs).d1;
            ($pr_reg)[1] = (*$regs).d2;
            ($pr_reg)[2] = (*$regs).d3;
            ($pr_reg)[3] = (*$regs).d4;
            ($pr_reg)[4] = (*$regs).d5;
            ($pr_reg)[7] = (*$regs).a0;
            ($pr_reg)[8] = (*$regs).a1;
            ($pr_reg)[9] = (*$regs).a2;
            ($pr_reg)[14] = (*$regs).d0;
            ($pr_reg)[15] = rdusp();
            ($pr_reg)[16] = (*$regs).orig_d0;
            ($pr_reg)[17] = (*$regs).sr;
            ($pr_reg)[18] = (*$regs).pc;
            ($pr_reg)[19] = ((*$regs).format << 12) | (*$regs).vector;
            let sw = ($regs as *mut switch_stack).sub(1);
            ($pr_reg)[5] = (*sw).d6;
            ($pr_reg)[6] = (*sw).d7;
            ($pr_reg)[10] = (*sw).a3;
            ($pr_reg)[11] = (*sw).a4;
            ($pr_reg)[12] = (*sw).a5;
            ($pr_reg)[13] = (*sw).a6;
        }
    }};
}

pub const ELF_HWCAP: i32 = 0;
pub const ELF_PLATFORM: *const core::ffi::c_void = core::ptr::null();
pub const ELF_FDPIC_CORE_EFLAGS: i32 = 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
