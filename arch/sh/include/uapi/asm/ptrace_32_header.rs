/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/*
 * GCC defines register number like this:
 * -----------------------------
 *   0 - 15 are integer registers
 *   17 - 22 are control/special registers
 *   24 - 39 fp registers
 *   40 - 47 xd registers
 *   48 -    fpscr register
 * -----------------------------
 *
 * We follows above, except:
 *   16 --- program counter (PC)
 *   22 --- syscall #
 *   23 --- floating point communication register
 */
pub const REG_REG0: i32 = 0;
pub const REG_REG15: i32 = 15;

pub const REG_PC: i32 = 16;

pub const REG_PR: i32 = 17;
pub const REG_SR: i32 = 18;
pub const REG_GBR: i32 = 19;
pub const REG_MACH: i32 = 20;
pub const REG_MACL: i32 = 21;

pub const REG_SYSCALL: i32 = 22;

pub const REG_FPREG0: i32 = 23;
pub const REG_FPREG15: i32 = 38;
pub const REG_XFREG0: i32 = 39;
pub const REG_XFREG15: i32 = 54;

pub const REG_FPSCR: i32 = 55;
pub const REG_FPUL: i32 = 56;

/*
 * This struct defines the way the registers are stored on the
 * kernel stack during a system call or other kernel entry.
 */
#[repr(C)]
pub struct pt_regs {
    pub regs: [u32; 16],
    pub pc: u32,
    pub pr: u32,
    pub sr: u32,
    pub gbr: u32,
    pub mach: u32,
    pub macl: u32,
    pub tra: i32,
}

/*
 * This struct defines the way the DSP registers are stored on the
 * kernel stack during a system call or other kernel entry.
 */
#[repr(C)]
pub struct pt_dspregs {
    pub a1: u32,
    pub a0g: u32,
    pub a1g: u32,
    pub m0: u32,
    pub m1: u32,
    pub a0: u32,
    pub x0: u32,
    pub x1: u32,
    pub y0: u32,
    pub y1: u32,
    pub dsr: u32,
    pub rs: u32,
    pub re: u32,
    pub mod_: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
