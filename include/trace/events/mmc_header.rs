/* SPDX-License-Identifier: GPL-2.0 */
// Translated from trace/events/mmc.h.  The Linux tracepoint machinery and
// mmc types are supplied by external dependencies.

use core::ffi::c_char;

#[repr(C)]
pub struct MmcCommand {
    pub opcode: u32,
    pub arg: u32,
    pub flags: u32,
    pub retries: u32,
    pub error: i32,
    pub resp: [u32; 4],
}

#[repr(C)]
pub struct MmcData {
    pub blksz: u32,
    pub blocks: u32,
    pub blk_addr: u32,
    pub flags: u32,
    pub bytes_xfered: u32,
    pub error: i32,
}

#[repr(C)]
pub struct MmcRequest {
    pub cmd: *mut MmcCommand,
    pub stop: *mut MmcCommand,
    pub sbc: *mut MmcCommand,
    pub data: *mut MmcData,
    pub tag: i32,
}

#[repr(C)]
pub struct MmcHost {
    pub can_retune: u32,
    pub doing_retune: u32,
    pub retune_now: u32,
    pub need_retune: i32,
    pub hold_retune: i32,
    pub retune_period: u32,
    pub name: *const c_char,
}

#[repr(C)]
pub struct MmcRequestStartEntry {
    pub cmd_opcode: u32, pub cmd_arg: u32, pub cmd_flags: u32, pub cmd_retries: u32,
    pub stop_opcode: u32, pub stop_arg: u32, pub stop_flags: u32, pub stop_retries: u32,
    pub sbc_opcode: u32, pub sbc_arg: u32, pub sbc_flags: u32, pub sbc_retries: u32,
    pub blocks: u32, pub blk_addr: u32, pub blksz: u32, pub data_flags: u32,
    pub tag: i32, pub can_retune: u32, pub doing_retune: u32, pub retune_now: u32,
    pub need_retune: i32, pub hold_retune: i32, pub retune_period: u32,
    pub mrq: *mut MmcRequest, pub name: *const c_char,
}

#[repr(C)]
pub struct MmcRequestDoneEntry {
    pub cmd_opcode: u32, pub cmd_err: i32, pub cmd_resp: [u32; 4], pub cmd_retries: u32,
    pub stop_opcode: u32, pub stop_err: i32, pub stop_resp: [u32; 4], pub stop_retries: u32,
    pub sbc_opcode: u32, pub sbc_err: i32, pub sbc_resp: [u32; 4], pub sbc_retries: u32,
    pub bytes_xfered: u32, pub data_err: i32, pub tag: i32, pub can_retune: u32,
    pub doing_retune: u32, pub retune_now: u32, pub need_retune: i32, pub hold_retune: i32,
    pub retune_period: u32, pub mrq: *mut MmcRequest, pub name: *const c_char,
}

#[inline]
pub unsafe fn mmc_request_start_assign(e: &mut MmcRequestStartEntry, host: *mut MmcHost, mrq: *mut MmcRequest) {
    let r = &*mrq;
    let c = |p: *mut MmcCommand| if p.is_null() { None } else { Some(&*p) };
    let d = if r.data.is_null() { None } else { Some(&*r.data) };
    e.cmd_opcode = c(r.cmd).map_or(0, |x| x.opcode); e.cmd_arg = c(r.cmd).map_or(0, |x| x.arg);
    e.cmd_flags = c(r.cmd).map_or(0, |x| x.flags); e.cmd_retries = c(r.cmd).map_or(0, |x| x.retries);
    e.stop_opcode = c(r.stop).map_or(0, |x| x.opcode); e.stop_arg = c(r.stop).map_or(0, |x| x.arg);
    e.stop_flags = c(r.stop).map_or(0, |x| x.flags); e.stop_retries = c(r.stop).map_or(0, |x| x.retries);
    e.sbc_opcode = c(r.sbc).map_or(0, |x| x.opcode); e.sbc_arg = c(r.sbc).map_or(0, |x| x.arg);
    e.sbc_flags = c(r.sbc).map_or(0, |x| x.flags); e.sbc_retries = c(r.sbc).map_or(0, |x| x.retries);
    e.blksz = d.map_or(0, |x| x.blksz); e.blocks = d.map_or(0, |x| x.blocks); e.blk_addr = d.map_or(0, |x| x.blk_addr);
    e.data_flags = d.map_or(0, |x| x.flags); e.tag = r.tag; let h = &*host;
    e.can_retune = h.can_retune; e.doing_retune = h.doing_retune; e.retune_now = h.retune_now;
    e.need_retune = h.need_retune; e.hold_retune = h.hold_retune; e.retune_period = h.retune_period;
    e.name = h.name; e.mrq = mrq;
}

#[inline]
pub unsafe fn mmc_request_done_assign(e: &mut MmcRequestDoneEntry, host: *mut MmcHost, mrq: *mut MmcRequest) {
    let r = &*mrq; let c = |p: *mut MmcCommand| if p.is_null() { None } else { Some(&*p) };
    for (dst, p) in [(&mut e.cmd_opcode, r.cmd), (&mut e.stop_opcode, r.stop), (&mut e.sbc_opcode, r.sbc)] { *dst = c(p).map_or(0, |x| x.opcode); }
    if let Some(x) = c(r.cmd) { e.cmd_err=x.error; e.cmd_resp=x.resp; e.cmd_retries=x.retries; } else { e.cmd_err=0; e.cmd_resp=[0;4]; e.cmd_retries=0; }
    if let Some(x) = c(r.stop) { e.stop_err=x.error; e.stop_resp=x.resp; e.stop_retries=x.retries; } else { e.stop_err=0; e.stop_resp=[0;4]; e.stop_retries=0; }
    if let Some(x) = c(r.sbc) { e.sbc_err=x.error; e.sbc_resp=x.resp; e.sbc_retries=x.retries; } else { e.sbc_err=0; e.sbc_resp=[0;4]; e.sbc_retries=0; }
    if let Some(x) = r.data.as_ref() { e.bytes_xfered=x.bytes_xfered; e.data_err=x.error; } else { e.bytes_xfered=0; e.data_err=0; }
    e.tag=r.tag; let h=&*host; e.can_retune=h.can_retune; e.doing_retune=h.doing_retune; e.retune_now=h.retune_now; e.need_retune=h.need_retune; e.hold_retune=h.hold_retune; e.retune_period=h.retune_period; e.name=h.name; e.mrq=mrq;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
