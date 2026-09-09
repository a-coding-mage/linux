/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the surrounding kernel translation.

/*
 * Avoid a literal load by emitting a sequence of ADD/LDR instructions with the
 * appropriate relocations. The combined sequence has a range of -/+ 256 MiB,
 * which should be sufficient for the core kernel as well as modules loaded
 * into the module region. (Not supported by LLD before release 14)
 */
macro_rules! LOAD_SYM_ARMV6 {
    ($reg:ident, $sym:ident) => {
        concat!(
            "\t.globl\t", stringify!($sym), "\n\t",
            "\t.reloc\t10f, R_ARM_ALU_PC_G0_NC, ", stringify!($sym), "\n\t",
            "\t.reloc\t11f, R_ARM_ALU_PC_G1_NC, ", stringify!($sym), "\n\t",
            "\t.reloc\t12f, R_ARM_LDR_PC_G2, ", stringify!($sym), "\n\t",
            "10:\tsub\t", stringify!($reg), ", pc, #8\n\t",
            "11:\tsub\t", stringify!($reg), ", ", stringify!($reg), ", #4\n\t",
            "12:\tldr\t", stringify!($reg), ", [", stringify!($reg), ", #0]\n\t",
        )
    };
}

#[inline]
pub fn arm_gen_nop() -> libc::c_ulong {
    #[cfg(CONFIG_THUMB2_KERNEL)]
    {
        return 0xf3af8000; /* nop.w */
    }
    #[cfg(not(CONFIG_THUMB2_KERNEL))]
    {
        0xe1a00000 /* mov r0, r0 */
    }
}

unsafe extern "C" {
    pub fn __arm_gen_branch(
        pc: libc::c_ulong,
        addr: libc::c_ulong,
        link: bool,
        warn: bool,
    ) -> libc::c_ulong;
}

#[inline]
pub unsafe fn arm_gen_branch(pc: libc::c_ulong, addr: libc::c_ulong) -> libc::c_ulong {
    unsafe { __arm_gen_branch(pc, addr, false, true) }
}

#[inline]
pub unsafe fn arm_gen_branch_link(
    pc: libc::c_ulong,
    addr: libc::c_ulong,
    warn: bool,
) -> libc::c_ulong {
    unsafe { __arm_gen_branch(pc, addr, true, warn) }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
