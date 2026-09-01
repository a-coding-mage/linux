/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright 2014, Michael Ellerman, IBM Corp.
 */

use core::arch::asm;
use core::ffi::c_ulong;

pub unsafe fn mfspr<const RN: u32>() -> c_ulong {
    let rval: c_ulong;

    unsafe {
        asm!(
            "mfspr {rval},{rn}",
            rval = out(reg) rval,
            rn = const RN,
        );
    }

    rval
}

pub unsafe fn mtspr<const RN: u32>(v: c_ulong) {
    unsafe {
        asm!(
            "mtspr {rn},{v}",
            rn = const RN,
            v = in(reg) v,
            options(nostack),
        );
    }
}

pub unsafe fn mb() {
    unsafe {
        asm!("sync", options(nostack));
    }
}

pub unsafe fn barrier() {
    unsafe {
        asm!("", options(nostack));
    }
}

pub const SPRN_HDEXCR_RO: u32 = 455; /* Userspace readonly view of SPRN_HDEXCR (471) */

pub const SPRN_MMCR2: u32 = 769;
pub const SPRN_MMCRA: u32 = 770;
pub const SPRN_MMCR0: u32 = 779;
pub const MMCR0_PMAO: u32 = 0x00000080;
pub const MMCR0_PMAE: u32 = 0x04000000;
pub const MMCR0_FC: u32 = 0x80000000;
pub const SPRN_EBBHR: u32 = 804;
pub const SPRN_EBBRR: u32 = 805;
pub const SPRN_BESCR: u32 = 806; /* Branch event status & control register */
pub const SPRN_BESCRS: u32 = 800; /* Branch event status & control set (1 bits set to 1) */
pub const SPRN_BESCRSU: u32 = 801; /* Branch event status & control set upper */
pub const SPRN_BESCRR: u32 = 802; /* Branch event status & control REset (1 bits set to 0) */
pub const SPRN_BESCRRU: u32 = 803; /* Branch event status & control REset upper */

pub const BESCR_PMEO: u64 = 0x1; /* PMU Event-based exception Occurred */
pub const BESCR_PME: c_ulong = (0x1 as c_ulong) << 32; /* PMU Event-based exception Enable */

pub const SPRN_PMC1: u32 = 771;
pub const SPRN_PMC2: u32 = 772;
pub const SPRN_PMC3: u32 = 773;
pub const SPRN_PMC4: u32 = 774;
pub const SPRN_PMC5: u32 = 775;
pub const SPRN_PMC6: u32 = 776;

pub const SPRN_SIAR: u32 = 780;
pub const SPRN_SDAR: u32 = 781;
pub const SPRN_SIER: u32 = 768;

pub const SPRN_DEXCR_RO: u32 = 812; /* Userspace readonly view of SPRN_DEXCR (828) */

pub const SPRN_TEXASR: u32 = 0x82; /* Transaction Exception and Status Register */
pub const SPRN_TFIAR: u32 = 0x81; /* Transaction Failure Inst Addr    */
pub const SPRN_TFHAR: u32 = 0x80; /* Transaction Failure Handler Addr */
pub const SPRN_TAR: u32 = 0x32f; /* Target Address Register */

pub const fn PVR_VER(pvr: c_ulong) -> c_ulong {
    (pvr >> 16) & 0xFFFF
}

pub const SPRN_PVR: u32 = 0x11F;

pub const fn PVR_CFG(pvr: c_ulong) -> c_ulong {
    (pvr >> 8) & 0xF
} /* Configuration field */

pub const fn PVR_MAJ(pvr: c_ulong) -> c_ulong {
    (pvr >> 4) & 0xF
} /* Major revision field */

pub const fn PVR_MIN(pvr: c_ulong) -> c_ulong {
    (pvr >> 0) & 0xF
} /* Minor revision field */

pub const SPRN_DSCR_PRIV: u32 = 0x11; /* Privilege State DSCR */
pub const SPRN_DSCR: u32 = 0x03; /* Data Stream Control Register */
pub const SPRN_PPR: u32 = 896; /* Program Priority Register */
pub const SPRN_AMR: u32 = 13; /* Authority Mask Register - problem state */

pub unsafe fn set_amr(v: c_ulong) {
    unsafe {
        asm!(
            "isync",
            "mtspr 13,{v}",
            "isync",
            v = in(reg) v,
        );
    }
}

/* TEXASR register bits */
pub const TEXASR_FC: u64 = 0xFE00000000000000;
pub const TEXASR_FP: u64 = 0x0100000000000000;
pub const TEXASR_DA: u64 = 0x0080000000000000;
pub const TEXASR_NO: u64 = 0x0040000000000000;
pub const TEXASR_FO: u64 = 0x0020000000000000;
pub const TEXASR_SIC: u64 = 0x0010000000000000;
pub const TEXASR_NTC: u64 = 0x0008000000000000;
pub const TEXASR_TC: u64 = 0x0004000000000000;
pub const TEXASR_TIC: u64 = 0x0002000000000000;
pub const TEXASR_IC: u64 = 0x0001000000000000;
pub const TEXASR_IFC: u64 = 0x0000800000000000;
pub const TEXASR_ABT: u64 = 0x0000000100000000;
pub const TEXASR_SPD: u64 = 0x0000000080000000;
pub const TEXASR_HV: u64 = 0x0000000020000000;
pub const TEXASR_PR: u64 = 0x0000000010000000;
pub const TEXASR_FS: u64 = 0x0000000008000000;
pub const TEXASR_TE: u64 = 0x0000000004000000;
pub const TEXASR_ROT: u64 = 0x0000000002000000;

/* MSR register bits */
pub const MSR_HV: c_ulong = (1 as c_ulong) << 60; /* Hypervisor state */
pub const MSR_TS_S_LG: u32 = 33; /* Trans Mem state: Suspended */
pub const MSR_TS_T_LG: u32 = 34; /* Trans Mem state: Active */

pub const fn __MASK(x: u32) -> c_ulong {
    (1 as c_ulong) << x
}

/* macro to check TM MSR bits */
pub const MSR_TS_S: c_ulong = __MASK(MSR_TS_S_LG); /* Transaction Suspended */
pub const MSR_TS_T: c_ulong = __MASK(MSR_TS_T_LG); /* Transaction Transactional */

/* Vector Instructions */
pub const fn VSX_XX1(xs: u32, ra: u32, rb: u32) -> u32 {
    (((xs) & 0x1f) << 21) | ((ra) << 16) | ((rb) << 11) | (((xs) >> 5))
}

/* C macro emitted ".long (0x7c000798 | VSX_XX1(...))". */
pub const fn STXVD2X(xs: u32, ra: u32, rb: u32) -> u32 {
    0x7c000798 | VSX_XX1(xs, ra, rb)
}

/* C macro emitted ".long (0x7c000698 | VSX_XX1(...))". */
pub const fn LXVD2X(xs: u32, ra: u32, rb: u32) -> u32 {
    0x7c000698 | VSX_XX1(xs, ra, rb)
}

macro_rules! ASM_LOAD_GPR_IMMED {
    ($_asm_symbol_name_immed:ident) => {
        concat!(
            "li 14, %[", stringify!($_asm_symbol_name_immed), "];",
            "li 15, %[", stringify!($_asm_symbol_name_immed), "];",
            "li 16, %[", stringify!($_asm_symbol_name_immed), "];",
            "li 17, %[", stringify!($_asm_symbol_name_immed), "];",
            "li 18, %[", stringify!($_asm_symbol_name_immed), "];",
            "li 19, %[", stringify!($_asm_symbol_name_immed), "];",
            "li 20, %[", stringify!($_asm_symbol_name_immed), "];",
            "li 21, %[", stringify!($_asm_symbol_name_immed), "];",
            "li 22, %[", stringify!($_asm_symbol_name_immed), "];",
            "li 23, %[", stringify!($_asm_symbol_name_immed), "];",
            "li 24, %[", stringify!($_asm_symbol_name_immed), "];",
            "li 25, %[", stringify!($_asm_symbol_name_immed), "];",
            "li 26, %[", stringify!($_asm_symbol_name_immed), "];",
            "li 27, %[", stringify!($_asm_symbol_name_immed), "];",
            "li 28, %[", stringify!($_asm_symbol_name_immed), "];",
            "li 29, %[", stringify!($_asm_symbol_name_immed), "];",
            "li 30, %[", stringify!($_asm_symbol_name_immed), "];",
            "li 31, %[", stringify!($_asm_symbol_name_immed), "];",
        )
    };
}

macro_rules! ASM_LOAD_FPR {
    ($_asm_symbol_name_addr:ident) => {
        concat!(
            "lfd 0, 0(%[", stringify!($_asm_symbol_name_addr), "]);",
            "lfd 1, 0(%[", stringify!($_asm_symbol_name_addr), "]);",
            "lfd 2, 0(%[", stringify!($_asm_symbol_name_addr), "]);",
            "lfd 3, 0(%[", stringify!($_asm_symbol_name_addr), "]);",
            "lfd 4, 0(%[", stringify!($_asm_symbol_name_addr), "]);",
            "lfd 5, 0(%[", stringify!($_asm_symbol_name_addr), "]);",
            "lfd 6, 0(%[", stringify!($_asm_symbol_name_addr), "]);",
            "lfd 7, 0(%[", stringify!($_asm_symbol_name_addr), "]);",
            "lfd 8, 0(%[", stringify!($_asm_symbol_name_addr), "]);",
            "lfd 9, 0(%[", stringify!($_asm_symbol_name_addr), "]);",
            "lfd 10, 0(%[", stringify!($_asm_symbol_name_addr), "]);",
            "lfd 11, 0(%[", stringify!($_asm_symbol_name_addr), "]);",
            "lfd 12, 0(%[", stringify!($_asm_symbol_name_addr), "]);",
            "lfd 13, 0(%[", stringify!($_asm_symbol_name_addr), "]);",
            "lfd 14, 0(%[", stringify!($_asm_symbol_name_addr), "]);",
            "lfd 15, 0(%[", stringify!($_asm_symbol_name_addr), "]);",
            "lfd 16, 0(%[", stringify!($_asm_symbol_name_addr), "]);",
            "lfd 17, 0(%[", stringify!($_asm_symbol_name_addr), "]);",
            "lfd 18, 0(%[", stringify!($_asm_symbol_name_addr), "]);",
            "lfd 19, 0(%[", stringify!($_asm_symbol_name_addr), "]);",
            "lfd 20, 0(%[", stringify!($_asm_symbol_name_addr), "]);",
            "lfd 21, 0(%[", stringify!($_asm_symbol_name_addr), "]);",
            "lfd 22, 0(%[", stringify!($_asm_symbol_name_addr), "]);",
            "lfd 23, 0(%[", stringify!($_asm_symbol_name_addr), "]);",
            "lfd 24, 0(%[", stringify!($_asm_symbol_name_addr), "]);",
            "lfd 25, 0(%[", stringify!($_asm_symbol_name_addr), "]);",
            "lfd 26, 0(%[", stringify!($_asm_symbol_name_addr), "]);",
            "lfd 27, 0(%[", stringify!($_asm_symbol_name_addr), "]);",
            "lfd 28, 0(%[", stringify!($_asm_symbol_name_addr), "]);",
            "lfd 29, 0(%[", stringify!($_asm_symbol_name_addr), "]);",
            "lfd 30, 0(%[", stringify!($_asm_symbol_name_addr), "]);",
            "lfd 31, 0(%[", stringify!($_asm_symbol_name_addr), "]);",
        )
    };
}

pub(crate) use ASM_LOAD_FPR;
pub(crate) use ASM_LOAD_GPR_IMMED;

/* Declarations excluded in C when __ASSEMBLER__ is defined. */
unsafe extern "C" {
    pub fn store_gpr(addr: *mut c_ulong);
    pub fn load_gpr(addr: *mut c_ulong);
    pub fn store_fpr(addr: *mut f64);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
