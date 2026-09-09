/* SPDX-License-Identifier: GPL-2.0 */

pub const PSW_I: u32 = 0x00000001;
pub const PSW_D: u32 = 0x00000002;
pub const PSW_P: u32 = 0x00000004;
pub const PSW_Q: u32 = 0x00000008;

pub const PSW_R: u32 = 0x00000010;
pub const PSW_F: u32 = 0x00000020;
pub const PSW_G: u32 = 0x00000040; /* PA1.x only */
pub const PSW_O: u32 = 0x00000080; /* PA2.0 only */

/* ssm/rsm instructions number PSW_W and PSW_E differently */
pub const PSW_SM_I: u32 = PSW_I; /* Enable External Interrupts */
pub const PSW_SM_D: u32 = PSW_D;
pub const PSW_SM_P: u32 = PSW_P;
pub const PSW_SM_Q: u32 = PSW_Q; /* Enable Interrupt State Collection */
pub const PSW_SM_R: u32 = PSW_R; /* Enable Recover Counter Trap */
pub const PSW_SM_W: u32 = 0x200; /* PA2.0 only : Enable Wide Mode */

pub const PSW_SM_QUIET: u32 = PSW_SM_R + PSW_SM_Q + PSW_SM_P + PSW_SM_D + PSW_SM_I;

pub const PSW_CB: u32 = 0x0000ff00;

pub const PSW_M: u32 = 0x00010000;
pub const PSW_V: u32 = 0x00020000;
pub const PSW_C: u32 = 0x00040000;
pub const PSW_B: u32 = 0x00080000;

pub const PSW_X: u32 = 0x00100000;
pub const PSW_N: u32 = 0x00200000;
pub const PSW_L: u32 = 0x00400000;
pub const PSW_H: u32 = 0x00800000;

pub const PSW_T: u32 = 0x01000000;
pub const PSW_S: u32 = 0x02000000;
pub const PSW_E: u32 = 0x04000000;
pub const PSW_W: u32 = 0x08000000; /* PA2.0 only */
pub const PSW_W_BIT: u32 = 36; /* PA2.0 only */

pub const PSW_Z: u32 = 0x40000000; /* PA1.x only */
pub const PSW_Y: u32 = 0x80000000; /* PA1.x only */

/* CONFIG_64BIT is a build-time C condition; Rust cfg preserves its intent. */
#[cfg(feature = "CONFIG_64BIT")]
pub const PSW_HI_CB: u32 = 0x000000ff; /* PA2.0 only */

#[cfg(feature = "CONFIG_64BIT")]
pub const USER_PSW_HI_MASK: u32 = PSW_HI_CB;
#[cfg(feature = "CONFIG_64BIT")]
pub const WIDE_PSW: u32 = PSW_W;
#[cfg(not(feature = "CONFIG_64BIT"))]
pub const WIDE_PSW: u32 = 0;

/* Used when setting up for rfi */
pub const KERNEL_PSW: u32 = WIDE_PSW | PSW_C | PSW_Q | PSW_P | PSW_D;
pub const REAL_MODE_PSW: u32 = WIDE_PSW | PSW_Q;
pub const USER_PSW_MASK: u32 = WIDE_PSW | PSW_T | PSW_N | PSW_X | PSW_B | PSW_V | PSW_CB;
pub const USER_PSW: u32 = PSW_C | PSW_Q | PSW_P | PSW_D | PSW_I;

/* The program status word as bitfields. */
#[repr(C)]
pub struct pa_psw {
    pub y: u32,
    pub z: u32,
    pub rv: u32,
    pub w: u32,
    pub e: u32,
    pub s: u32,
    pub t: u32,
    pub h: u32,
    pub l: u32,
    pub n: u32,
    pub x: u32,
    pub b: u32,
    pub c: u32,
    pub v: u32,
    pub m: u32,
    pub cb: u32,
    pub o: u32,
    pub g: u32,
    pub f: u32,
    pub r: u32,
    pub q: u32,
    pub p: u32,
    pub d: u32,
    pub i: u32,
}

/* TASK_PT_PSW is supplied by the containing architecture definitions. */
#[cfg(feature = "CONFIG_64BIT")]
#[inline]
pub unsafe fn pa_psw(task: *mut u8) -> *mut pa_psw {
    task.add(TASK_PT_PSW + 4) as *mut pa_psw
}

#[cfg(not(feature = "CONFIG_64BIT"))]
#[inline]
pub unsafe fn pa_psw(task: *mut u8) -> *mut pa_psw {
    task.add(TASK_PT_PSW) as *mut pa_psw
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
