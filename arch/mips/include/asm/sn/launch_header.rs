/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1992 - 1997, 2000 Silicon Graphics, Inc.
 * Copyright (C) 2000 by Colin Ngam
 */

// Dependencies supplied by asm/sn/types.h and asm/sn/addrs.h are external.

/*
 * The launch data structure resides at a fixed place in each node's memory
 * and is used to communicate between the master processor and the slave
 * processors.
 */

pub const LAUNCH_MAGIC: u64 = 0xaddbead2addbead3;
// CONFIG_SGI_IP27 controls these build-time constants.
#[cfg(feature = "CONFIG_SGI_IP27")]
pub const LAUNCH_SIZEOF: usize = 0x100;
#[cfg(feature = "CONFIG_SGI_IP27")]
pub const LAUNCH_PADSZ: usize = 0xa0;

pub const LAUNCH_OFF_MAGIC: usize = 0x00; // Struct offsets for assembly
pub const LAUNCH_OFF_BUSY: usize = 0x08;
pub const LAUNCH_OFF_CALL: usize = 0x10;
pub const LAUNCH_OFF_CALLC: usize = 0x18;
pub const LAUNCH_OFF_CALLPARM: usize = 0x20;
pub const LAUNCH_OFF_STACK: usize = 0x28;
pub const LAUNCH_OFF_GP: usize = 0x30;
pub const LAUNCH_OFF_BEVUTLB: usize = 0x38;
pub const LAUNCH_OFF_BEVNORMAL: usize = 0x40;
pub const LAUNCH_OFF_BEVECC: usize = 0x48;

pub type LaunchStateT = i32;
pub const LAUNCH_STATE_DONE: LaunchStateT = 0; // Return value of LAUNCH_POLL
pub const LAUNCH_STATE_SENT: LaunchStateT = 1;
pub const LAUNCH_STATE_RECD: LaunchStateT = 2;

pub type LaunchProcT = unsafe extern "C" fn(call_parm: u64);

#[repr(C)]
pub struct LaunchT {
    pub magic: u64,
    pub busy: u64,
    pub call_addr: Option<LaunchProcT>,
    pub call_addr_c: u64,
    pub call_parm: u64,
    pub stack_addr: *mut core::ffi::c_void,
    pub gp_addr: *mut core::ffi::c_void,
    pub bevutlb: *mut core::ffi::c_char,
    pub bevnormal: *mut core::ffi::c_char,
    pub bevecc: *mut core::ffi::c_char,
    pub pad: [core::ffi::c_char; 160],
}

/*
 * The launch routine is called only if the complement address is correct.
 * Before control is transferred to a routine, the complement address is
 * zeroed (invalidated) to prevent an accidental call from a spurious
 * interrupt.
 */

// PROM entry points are determined by IPxxprom/start.s.  The following
// wrappers preserve the C macros' call-through-an-address behavior.
pub unsafe fn launch_slave(
    nasid: i32,
    cpu: i32,
    call_addr: LaunchProcT,
    call_parm: u64,
    stack_addr: *mut core::ffi::c_void,
    gp_addr: *mut core::ffi::c_void,
) {
    let f: unsafe extern "C" fn(i32, i32, LaunchProcT, u64, *mut core::ffi::c_void, *mut core::ffi::c_void) =
        core::mem::transmute(IP27PROM_LAUNCHSLAVE);
    f(nasid, cpu, call_addr, call_parm, stack_addr, gp_addr);
}

pub unsafe fn launch_wait(nasid: i32, cpu: i32, timeout_msec: i32) {
    let f: unsafe extern "C" fn(i32, i32, i32) = core::mem::transmute(IP27PROM_WAITSLAVE);
    f(nasid, cpu, timeout_msec);
}

pub unsafe fn launch_poll(nasid: i32, cpu: i32) -> LaunchStateT {
    let f: unsafe extern "C" fn(i32, i32) -> LaunchStateT = core::mem::transmute(IP27PROM_POLLSLAVE);
    f(nasid, cpu)
}

pub unsafe fn launch_loop() {
    let f: unsafe extern "C" fn() = core::mem::transmute(IP27PROM_SLAVELOOP);
    f();
}

pub unsafe fn launch_flash() {
    let f: unsafe extern "C" fn() = core::mem::transmute(IP27PROM_FLASHLEDS);
    f();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
