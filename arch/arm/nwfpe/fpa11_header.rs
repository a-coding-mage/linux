/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
    NetWinder Floating Point Emulator
    (c) Rebel.com, 1998-1999

    Direct questions, comments to Scott Bambrough <scottb@netwinder.org>

*/

/* C header dependencies are supplied by the surrounding translation unit:
 * linux/thread_info.h, fpsr.h, milieu.h, and softfloat.h.
 */

#[macro_export]
macro_rules! GET_FPA11 {
    () => {
        (&mut (*current_thread_info()).fpstate as *mut _ as *mut FPA11)
    };
}

/*
 * The processes registers are always at the very top of the 8K
 * stack+task struct.  Use the same method as 'current' uses to
 * reach them.
 */
#[macro_export]
macro_rules! GET_USERREG {
    () => {
        ((THREAD_START_SP + current_thread_info() as usize) as *mut pt_regs).wrapping_offset(-1)
    };
}

#[repr(C)]
pub struct roundingData {
    pub mode: i8,
    pub precision: i8,
    pub exception: i8,
}

pub const typeNone: u32 = 0x00;
pub const typeSingle: u32 = 0x01;
pub const typeDouble: u32 = 0x02;
pub const typeExtended: u32 = 0x03;

/*
 * This must be no more and no less than 12 bytes.
 */
#[cfg(feature = "CONFIG_FPE_NWFPE_XP")]
#[repr(C, packed(4))]
pub union FPREG {
    pub fSingle: f32,
    pub fDouble: f64,
    pub fExtended: floatx80,
}

#[cfg(not(feature = "CONFIG_FPE_NWFPE_XP"))]
#[repr(C, packed(4))]
pub union FPREG {
    pub fSingle: f32,
    pub fDouble: f64,
    pub padding: [u32; 3],
}

/*
 * FPA11 device model.
 *
 * This structure is exported to user space.  Do not re-order.
 * Only add new stuff to the end, and do not change the size of
 * any element.  Elements of this structure are used by user
 * space, and must match struct user_fp in <asm/user.h>.
 * We include the byte offsets below for documentation purposes.
 *
 * The size of this structure and FPREG are checked by fpmodule.c
 * on initialisation.  If the rules have been broken, NWFPE will
 * not initialise.
 */
#[repr(C, packed(4))]
pub struct FPA11 {
    /*   0 */ pub fpreg: [FPREG; 8], /* 8 floating point registers */
    /*  96 */ pub fpsr: FPSR, /* floating point status register */
    /* 100 */ pub fpcr: FPCR, /* floating point control register */
    /* 104 */ pub fType: [u8; 8], /* type of floating point value held in
                                      floating point registers.  One of
                                      none, single, double or extended. */
    /* 112 */ pub initflag: i32, /* this is special.  The kernel guarantees
                                    to set it to 0 when a thread is launched,
                                    so we can use it to detect whether this
                                    instance of the emulator needs to be
                                    initialised. */
}

unsafe extern "C" {
    pub fn SetRoundingMode(mode: u32) -> i8;
    pub fn SetRoundingPrecision(precision: u32) -> i8;
    pub fn nwfpe_init_fpa(fp: *mut fp_state);

    pub fn EmulateAll(opcode: u32) -> u32;

    pub fn EmulateCPDT(opcode: u32) -> u32;
    pub fn EmulateCPDO(opcode: u32) -> u32;
    pub fn EmulateCPRT(opcode: u32) -> u32;

    /* fpa11_cpdt.c */
    pub fn PerformLDF(opcode: u32) -> u32;
    pub fn PerformSTF(opcode: u32) -> u32;
    pub fn PerformLFM(opcode: u32) -> u32;
    pub fn PerformSFM(opcode: u32) -> u32;

    /* single_cpdo.c */
    pub fn SingleCPDO(roundData: *mut roundingData, opcode: u32, rFd: *mut FPREG) -> u32;
    /* double_cpdo.c */
    pub fn DoubleCPDO(roundData: *mut roundingData, opcode: u32, rFd: *mut FPREG) -> u32;
    /* extneded_cpdo.c */
    pub fn ExtendedCPDO(roundData: *mut roundingData, opcode: u32, rFd: *mut FPREG) -> u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
