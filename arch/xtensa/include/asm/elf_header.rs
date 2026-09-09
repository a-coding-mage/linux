/*
 * include/asm-xtensa/elf.h
 *
 * ELF register definitions
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2001 - 2005 Tensilica Inc.
 */

/* Dependencies supplied by the corresponding architecture and ELF headers. */

/* Xtensa processor ELF architecture-magic number */
pub const EM_XTENSA_OLD: u16 = 0xABC7;

/* Xtensa relocations defined by the ABIs */
pub const R_XTENSA_NONE: u32 = 0;
pub const R_XTENSA_32: u32 = 1;
pub const R_XTENSA_RTLD: u32 = 2;
pub const R_XTENSA_GLOB_DAT: u32 = 3;
pub const R_XTENSA_JMP_SLOT: u32 = 4;
pub const R_XTENSA_RELATIVE: u32 = 5;
pub const R_XTENSA_PLT: u32 = 6;
pub const R_XTENSA_OP0: u32 = 8;
pub const R_XTENSA_OP1: u32 = 9;
pub const R_XTENSA_OP2: u32 = 10;
pub const R_XTENSA_ASM_EXPAND: u32 = 11;
pub const R_XTENSA_ASM_SIMPLIFY: u32 = 12;
pub const R_XTENSA_GNU_VTINHERIT: u32 = 15;
pub const R_XTENSA_GNU_VTENTRY: u32 = 16;
pub const R_XTENSA_DIFF8: u32 = 17;
pub const R_XTENSA_DIFF16: u32 = 18;
pub const R_XTENSA_DIFF32: u32 = 19;
pub const R_XTENSA_SLOT0_OP: u32 = 20;
pub const R_XTENSA_SLOT1_OP: u32 = 21;
pub const R_XTENSA_SLOT2_OP: u32 = 22;
pub const R_XTENSA_SLOT3_OP: u32 = 23;
pub const R_XTENSA_SLOT4_OP: u32 = 24;
pub const R_XTENSA_SLOT5_OP: u32 = 25;
pub const R_XTENSA_SLOT6_OP: u32 = 26;
pub const R_XTENSA_SLOT7_OP: u32 = 27;
pub const R_XTENSA_SLOT8_OP: u32 = 28;
pub const R_XTENSA_SLOT9_OP: u32 = 29;
pub const R_XTENSA_SLOT10_OP: u32 = 30;
pub const R_XTENSA_SLOT11_OP: u32 = 31;
pub const R_XTENSA_SLOT12_OP: u32 = 32;
pub const R_XTENSA_SLOT13_OP: u32 = 33;
pub const R_XTENSA_SLOT14_OP: u32 = 34;
pub const R_XTENSA_SLOT0_ALT: u32 = 35;
pub const R_XTENSA_SLOT1_ALT: u32 = 36;
pub const R_XTENSA_SLOT2_ALT: u32 = 37;
pub const R_XTENSA_SLOT3_ALT: u32 = 38;
pub const R_XTENSA_SLOT4_ALT: u32 = 39;
pub const R_XTENSA_SLOT5_ALT: u32 = 40;
pub const R_XTENSA_SLOT6_ALT: u32 = 41;
pub const R_XTENSA_SLOT7_ALT: u32 = 42;
pub const R_XTENSA_SLOT8_ALT: u32 = 43;
pub const R_XTENSA_SLOT9_ALT: u32 = 44;
pub const R_XTENSA_SLOT10_ALT: u32 = 45;
pub const R_XTENSA_SLOT11_ALT: u32 = 46;
pub const R_XTENSA_SLOT12_ALT: u32 = 47;
pub const R_XTENSA_SLOT13_ALT: u32 = 48;
pub const R_XTENSA_SLOT14_ALT: u32 = 49;

/* ELF register definitions. This is needed for core dump support. */
pub type elf_greg_t = ::core::ffi::c_ulong;
pub type xtensa_gregset_t = user_pt_regs;
pub const ELF_NGREG: usize = ::core::mem::size_of::<xtensa_gregset_t>()
    / ::core::mem::size_of::<elf_greg_t>();
pub type elf_gregset_t = [elf_greg_t; ELF_NGREG];

pub const ELF_NFPREG: usize = 18;
pub type elf_fpreg_t = u32;
pub type elf_fpregset_t = [elf_fpreg_t; ELF_NFPREG];

/* This is used to ensure we don't load something for the wrong architecture. */
macro_rules! elf_check_arch {
    ($x:expr) => {
        (($x).e_machine == EM_XTENSA) || (($x).e_machine == EM_XTENSA_OLD)
    };
}

pub const ELFOSABI_XTENSA_FDPIC: u8 = 65;
macro_rules! elf_check_fdpic {
    ($x:expr) => {
        ($x).e_ident[EI_OSABI] == ELFOSABI_XTENSA_FDPIC
    };
}
pub const ELF_FDPIC_CORE_EFLAGS: u32 = 0;

/* These are used to set parameters in the core dumps. */
/* ELF_DATA depends on the target byte-order configuration (__XTENSA_EL__/__XTENSA_EB__). */
#[cfg(__XTENSA_EL__)]
pub const ELF_DATA: u8 = ELFDATA2LSB;
#[cfg(__XTENSA_EB__)]
pub const ELF_DATA: u8 = ELFDATA2MSB;

pub const ELF_CLASS: u8 = ELFCLASS32;
pub const ELF_ARCH: u16 = EM_XTENSA;
pub const ELF_EXEC_PAGESIZE: usize = PAGE_SIZE;
/* CORE_DUMP_USE_REGSET */

pub const ELF_ET_DYN_BASE: usize = (2 * TASK_SIZE) / 3;
pub const ELF_HWCAP: u32 = 0;
pub const ELF_PLATFORM: *const core::ffi::c_char = core::ptr::null();

macro_rules! ELF_PLAT_INIT {
    ($r:expr, $load_addr:expr) => {{
        let r = $r;
        r.areg[0] = 0;
        r.areg[2] = 0; r.areg[3] = 0;
        r.areg[4] = 0; r.areg[5] = 0;
        r.areg[6] = 0; r.areg[7] = 0;
        r.areg[8] = 0; r.areg[9] = 0;
        r.areg[10] = 0; r.areg[11] = 0;
        r.areg[12] = 0; r.areg[13] = 0;
        r.areg[14] = 0; r.areg[15] = 0;
    }};
}

macro_rules! ELF_FDPIC_PLAT_INIT {
    ($r:expr, $exec_map_addr:expr, $interp_map_addr:expr, $dynamic_addr:expr) => {{
        let r = $r;
        r.areg[4] = $exec_map_addr;
        r.areg[5] = $interp_map_addr;
        r.areg[6] = $dynamic_addr;
    }};
}

#[repr(C)]
pub struct elf_xtregs_t {
    pub opt: xtregs_opt_t,
    pub user: xtregs_user_t,
    #[cfg(XTENSA_HAVE_COPROCESSORS)]
    pub cp0: xtregs_cp0_t,
    #[cfg(XTENSA_HAVE_COPROCESSORS)]
    pub cp1: xtregs_cp1_t,
    #[cfg(XTENSA_HAVE_COPROCESSORS)]
    pub cp2: xtregs_cp2_t,
    #[cfg(XTENSA_HAVE_COPROCESSORS)]
    pub cp3: xtregs_cp3_t,
    #[cfg(XTENSA_HAVE_COPROCESSORS)]
    pub cp4: xtregs_cp4_t,
    #[cfg(XTENSA_HAVE_COPROCESSORS)]
    pub cp5: xtregs_cp5_t,
    #[cfg(XTENSA_HAVE_COPROCESSORS)]
    pub cp6: xtregs_cp6_t,
    #[cfg(XTENSA_HAVE_COPROCESSORS)]
    pub cp7: xtregs_cp7_t,
}

macro_rules! SET_PERSONALITY {
    ($ex:expr) => {
        set_personality(PER_LINUX_32BIT | (current.personality & (!PER_MASK)))
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
