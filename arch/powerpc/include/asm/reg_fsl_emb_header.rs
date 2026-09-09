/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Contains register definitions for the Freescale Embedded Performance
 * Monitor.
 */

/* The original declarations are guarded by __KERNEL__. */

/* Performance Monitor Registers */
#[inline(always)]
pub unsafe fn mfpmr(rn: u32) -> u32 {
    let rval: u32;
    core::arch::asm!(
        ".machine push",
        ".machine e300",
        "mfpmr {rval}, {rn}",
        ".machine pop",
        rval = out(reg) rval,
        rn = const rn,
    );
    rval
}

#[inline(always)]
pub unsafe fn mtpmr(rn: u32, val: u32) {
    core::arch::asm!(
        ".machine push",
        ".machine e300",
        "mtpmr {rn}, {val}",
        ".machine pop",
        rn = const rn,
        val = in(reg) val,
    );
}

/* Freescale Book E Performance Monitor APU Registers */
pub const PMRN_PMC0: u32 = 0x010; /* Performance Monitor Counter 0 */
pub const PMRN_PMC1: u32 = 0x011; /* Performance Monitor Counter 1 */
pub const PMRN_PMC2: u32 = 0x012; /* Performance Monitor Counter 2 */
pub const PMRN_PMC3: u32 = 0x013; /* Performance Monitor Counter 3 */
pub const PMRN_PMC4: u32 = 0x014; /* Performance Monitor Counter 4 */
pub const PMRN_PMC5: u32 = 0x015; /* Performance Monitor Counter 5 */
pub const PMRN_PMLCA0: u32 = 0x090; /* PM Local Control A0 */
pub const PMRN_PMLCA1: u32 = 0x091; /* PM Local Control A1 */
pub const PMRN_PMLCA2: u32 = 0x092; /* PM Local Control A2 */
pub const PMRN_PMLCA3: u32 = 0x093; /* PM Local Control A3 */
pub const PMRN_PMLCA4: u32 = 0x094; /* PM Local Control A4 */
pub const PMRN_PMLCA5: u32 = 0x095; /* PM Local Control A5 */

pub const PMLCA_FC: u32 = 0x80000000; /* Freeze Counter */
pub const PMLCA_FCS: u32 = 0x40000000; /* Freeze in Supervisor */
pub const PMLCA_FCU: u32 = 0x20000000; /* Freeze in User */
pub const PMLCA_FCM1: u32 = 0x10000000; /* Freeze when PMM==1 */
pub const PMLCA_FCM0: u32 = 0x08000000; /* Freeze when PMM==0 */
pub const PMLCA_CE: u32 = 0x04000000; /* Condition Enable */
pub const PMLCA_FGCS1: u32 = 0x00000002; /* Freeze in guest state */
pub const PMLCA_FGCS0: u32 = 0x00000001; /* Freeze in hypervisor state */

pub const PMLCA_EVENT_MASK: u32 = 0x01ff0000; /* Event field */
pub const PMLCA_EVENT_SHIFT: u32 = 16;

pub const PMRN_PMLCB0: u32 = 0x110; /* PM Local Control B0 */
pub const PMRN_PMLCB1: u32 = 0x111; /* PM Local Control B1 */
pub const PMRN_PMLCB2: u32 = 0x112; /* PM Local Control B2 */
pub const PMRN_PMLCB3: u32 = 0x113; /* PM Local Control B3 */
pub const PMRN_PMLCB4: u32 = 0x114; /* PM Local Control B4 */
pub const PMRN_PMLCB5: u32 = 0x115; /* PM Local Control B5 */

pub const PMLCB_THRESHMUL_MASK: u32 = 0x0700; /* Threshold Multiple Field */
pub const PMLCB_THRESHMUL_SHIFT: u32 = 8;
pub const PMLCB_THRESHOLD_MASK: u32 = 0x003f; /* Threshold Field */
pub const PMLCB_THRESHOLD_SHIFT: u32 = 0;

pub const PMRN_PMGC0: u32 = 0x190; /* PM Global Control 0 */
pub const PMGC0_FAC: u32 = 0x80000000; /* Freeze all Counters */
pub const PMGC0_PMIE: u32 = 0x40000000; /* Interrupt Enable */
pub const PMGC0_FCECE: u32 = 0x20000000; /* Freeze countes on Enabled Condition or Event */

pub const PMRN_UPMC0: u32 = 0x000; /* User Performance Monitor Counter 0 */
pub const PMRN_UPMC1: u32 = 0x001; /* User Performance Monitor Counter 1 */
pub const PMRN_UPMC2: u32 = 0x002; /* User Performance Monitor Counter 2 */
pub const PMRN_UPMC3: u32 = 0x003; /* User Performance Monitor Counter 3 */
pub const PMRN_UPMC4: u32 = 0x004; /* User Performance Monitor Counter 4 */
pub const PMRN_UPMC5: u32 = 0x005; /* User Performance Monitor Counter 5 */
pub const PMRN_UPMLCA0: u32 = 0x080; /* User PM Local Control A0 */
pub const PMRN_UPMLCA1: u32 = 0x081; /* User PM Local Control A1 */
pub const PMRN_UPMLCA2: u32 = 0x082; /* User PM Local Control A2 */
pub const PMRN_UPMLCA3: u32 = 0x083; /* User PM Local Control A3 */
pub const PMRN_UPMLCA4: u32 = 0x084; /* User PM Local Control A4 */
pub const PMRN_UPMLCA5: u32 = 0x085; /* User PM Local Control A5 */
pub const PMRN_UPMLCB0: u32 = 0x100; /* User PM Local Control B0 */
pub const PMRN_UPMLCB1: u32 = 0x101; /* User PM Local Control B1 */
pub const PMRN_UPMLCB2: u32 = 0x102; /* User PM Local Control B2 */
pub const PMRN_UPMLCB3: u32 = 0x103; /* User PM Local Control B3 */
pub const PMRN_UPMLCB4: u32 = 0x104; /* User PM Local Control B4 */
pub const PMRN_UPMLCB5: u32 = 0x105; /* User PM Local Control B5 */
pub const PMRN_UPMGC0: u32 = 0x180; /* User PM Global Control 0 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
