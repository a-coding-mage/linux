/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/* Indicate the register numbers for a number of the specific
   debug registers. Registers 0-3 contain the addresses we wish to trap on */
pub const DR_FIRSTADDR: u64 = 0; /* u_debugreg[DR_FIRSTADDR] */
pub const DR_LASTADDR: u64 = 3; /* u_debugreg[DR_LASTADDR] */

pub const DR_STATUS: u64 = 6; /* u_debugreg[DR_STATUS] */
pub const DR_CONTROL: u64 = 7; /* u_debugreg[DR_CONTROL] */

/* Define bits in DR6 which are set to 1 by default.
 *
 * This is also the DR6 architectural value following Power-up, Reset or INIT.
 *
 * Note, with the introduction of Bus Lock Detection (BLD) and Restricted
 * Transactional Memory (RTM), the DR6 register has been modified:
 *
 * 1) BLD flag (bit 11) is no longer reserved to 1 if the CPU supports
 *    Bus Lock Detection. The assertion of a bus lock could clear it.
 *
 * 2) RTM flag (bit 16) is no longer reserved to 1 if the CPU supports
 *    restricted transactional memory. #DB occurred inside an RTM region
 *    could clear it.
 *
 * Apparently, DR6.BLD and DR6.RTM are active low bits.
 *
 * As a result, DR6_RESERVED is an incorrect name now, but it is kept for
 * compatibility.
 */
pub const DR6_RESERVED: u64 = 0xFFFF0FF0;

pub const DR_TRAP0: u64 = 0x1; /* db0 */
pub const DR_TRAP1: u64 = 0x2; /* db1 */
pub const DR_TRAP2: u64 = 0x4; /* db2 */
pub const DR_TRAP3: u64 = 0x8; /* db3 */
pub const DR_TRAP_BITS: u64 = DR_TRAP0 | DR_TRAP1 | DR_TRAP2 | DR_TRAP3;

pub const DR_BUS_LOCK: u64 = 0x800; /* bus_lock */
pub const DR_STEP: u64 = 0x4000; /* single-step */
pub const DR_SWITCH: u64 = 0x8000; /* task switch */

/* Now define a bunch of things for manipulating the control register.
   The top two bytes of the control register consist of 4 fields of 4
   bits - each field corresponds to one of the four debug registers,
   and indicates what types of access we trap on, and how large the data
   field is that we are looking at */
pub const DR_CONTROL_SHIFT: u64 = 16; /* Skip this many bits in ctl register */
pub const DR_CONTROL_SIZE: u64 = 4; /* 4 control bits per register */

pub const DR_RW_EXECUTE: u64 = 0x0; /* Settings for the access types to trap on */
pub const DR_RW_WRITE: u64 = 0x1;
pub const DR_RW_READ: u64 = 0x3;

pub const DR_LEN_1: u64 = 0x0; /* Settings for data length to trap on */
pub const DR_LEN_2: u64 = 0x4;
pub const DR_LEN_4: u64 = 0xC;
pub const DR_LEN_8: u64 = 0x8;

/* The low byte to the control register determine which registers are
   enabled. There are 4 fields of two bits. One bit is "local", meaning
   that the processor will reset the bit after a task switch and the other
   is global meaning that we have to explicitly reset the bit. With linux,
   you can use either one, since we explicitly zero the register when we enter
   kernel mode. */
pub const DR_LOCAL_ENABLE_SHIFT: u64 = 0; /* Extra shift to the local enable bit */
pub const DR_GLOBAL_ENABLE_SHIFT: u64 = 1; /* Extra shift to the global enable bit */
pub const DR_LOCAL_ENABLE: u64 = 0x1; /* Local enable for reg 0 */
pub const DR_GLOBAL_ENABLE: u64 = 0x2; /* Global enable for reg 0 */
pub const DR_ENABLE_SIZE: u64 = 2; /* 2 enable bits per register */

pub const DR_LOCAL_ENABLE_MASK: u64 = 0x55; /* Set local bits for all 4 regs */
pub const DR_GLOBAL_ENABLE_MASK: u64 = 0xAA; /* Set global bits for all 4 regs */

/* The second byte to the control register has a few special things.
   We can slow the instruction pipeline for instructions coming via the
   gdt or the ldt if we want to. I am not sure why this is an advantage */
#[cfg(target_arch = "x86")]
pub const DR_CONTROL_RESERVED: u64 = 0xFC00; /* Reserved by Intel */
#[cfg(not(target_arch = "x86"))]
pub const DR_CONTROL_RESERVED: u64 = 0xFFFFFFFF0000FC00; /* Reserved */

pub const DR_LOCAL_SLOWDOWN: u64 = 0x100; /* Local slow the pipeline */
pub const DR_GLOBAL_SLOWDOWN: u64 = 0x200; /* Global slow the pipeline */

/* HW breakpoint additions */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
