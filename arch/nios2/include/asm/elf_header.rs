/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2011 Tobias Klauser <tklauser@distanz.ch>
 */

// Dependency intent: symbols and types from <uapi/asm/elf.h> are supplied externally.

/* This is used to ensure we don't load something for the wrong architecture. */
#[macro_export]
macro_rules! elf_check_arch {
    ($x:expr) => { ($x).e_machine == EM_ALTERA_NIOS2 };
}

#[macro_export]
macro_rules! ELF_PLAT_INIT {
    ($_r:tt, $load_addr:tt) => {};
}

pub const CORE_DUMP_USE_REGSET: bool = true;
pub const ELF_EXEC_PAGESIZE: usize = 4096;

/* This is the location that an ET_DYN program is loaded if exec'ed. */
pub const ELF_ET_DYN_BASE: usize = 0xD000_0000;

/* regs is struct pt_regs, pr_reg is elf_gregset_t. */
pub const ARCH_HAS_SETUP_ADDITIONAL_PAGES: i32 = 1;

#[repr(C)]
pub struct linux_binprm {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn arch_setup_additional_pages(
        bprm: *mut linux_binprm,
        uses_interp: i32,
    ) -> i32;
}

#[macro_export]
macro_rules! ELF_CORE_COPY_REGS {
    ($pr_reg:expr, $regs:expr) => {{
        $pr_reg[0] = (*$regs).r8;
        $pr_reg[1] = (*$regs).r9;
        $pr_reg[2] = (*$regs).r10;
        $pr_reg[3] = (*$regs).r11;
        $pr_reg[4] = (*$regs).r12;
        $pr_reg[5] = (*$regs).r13;
        $pr_reg[6] = (*$regs).r14;
        $pr_reg[7] = (*$regs).r15;
        $pr_reg[8] = (*$regs).r1;
        $pr_reg[9] = (*$regs).r2;
        $pr_reg[10] = (*$regs).r3;
        $pr_reg[11] = (*$regs).r4;
        $pr_reg[12] = (*$regs).r5;
        $pr_reg[13] = (*$regs).r6;
        $pr_reg[14] = (*$regs).r7;
        $pr_reg[15] = (*$regs).orig_r2;
        $pr_reg[16] = (*$regs).ra;
        $pr_reg[17] = (*$regs).fp;
        $pr_reg[18] = (*$regs).sp;
        $pr_reg[19] = (*$regs).gp;
        $pr_reg[20] = (*$regs).estatus;
        $pr_reg[21] = (*$regs).ea;
        $pr_reg[22] = (*$regs).orig_r7;
        let sw = (($regs as *mut switch_stack).offset(-1));
        $pr_reg[23] = (*sw).r16;
        $pr_reg[24] = (*sw).r17;
        $pr_reg[25] = (*sw).r18;
        $pr_reg[26] = (*sw).r19;
        $pr_reg[27] = (*sw).r20;
        $pr_reg[28] = (*sw).r21;
        $pr_reg[29] = (*sw).r22;
        $pr_reg[30] = (*sw).r23;
        $pr_reg[31] = (*sw).fp;
        $pr_reg[32] = (*sw).gp;
        $pr_reg[33] = (*sw).ra;
    }};
}

pub const ELF_HWCAP: u32 = 0;
pub const ELF_PLATFORM: *const core::ffi::c_void = core::ptr::null();

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
