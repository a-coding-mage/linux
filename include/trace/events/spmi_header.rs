/* SPDX-License-Identifier: GPL-2.0 */

// Translated from the Linux SPMI tracepoint header.  The original TRACE_EVENT
// declarations describe the entries recorded by the tracing subsystem.

#[repr(C)]
pub struct SpmiWriteBeginEntry {
    pub opcode: u8,
    pub sid: u8,
    pub addr: u16,
    pub len: u8,
    pub buf: *mut u8,
}

#[repr(C)]
pub struct SpmiWriteEndEntry {
    pub opcode: u8,
    pub sid: u8,
    pub addr: u16,
    pub ret: i32,
}

#[repr(C)]
pub struct SpmiReadBeginEntry {
    pub opcode: u8,
    pub sid: u8,
    pub addr: u16,
}

#[repr(C)]
pub struct SpmiReadEndEntry {
    pub opcode: u8,
    pub sid: u8,
    pub addr: u16,
    pub ret: i32,
    pub len: u8,
    pub buf: *mut u8,
}

#[repr(C)]
pub struct SpmiCmdEntry {
    pub opcode: u8,
    pub sid: u8,
    pub ret: i32,
}

/// Equivalent to the `spmi_write_begin` trace event's fast assignment.
pub unsafe fn spmi_write_begin(
    opcode: u8,
    sid: u8,
    addr: u16,
    len: u8,
    buf: *const u8,
    dynamic_buf: *mut u8,
) -> SpmiWriteBeginEntry {
    if len != 0 {
        core::ptr::copy_nonoverlapping(buf, dynamic_buf, len as usize);
    }
    SpmiWriteBeginEntry { opcode, sid, addr, len, buf: dynamic_buf }
}

/// Print format: `opc=%d sid=%02d addr=0x%04x len=%d buf=0x[%*phD]`.

/// Equivalent to the `spmi_write_end` trace event's fast assignment.
pub fn spmi_write_end(opcode: u8, sid: u8, addr: u16, ret: i32) -> SpmiWriteEndEntry {
    SpmiWriteEndEntry { opcode, sid, addr, ret }
}

/// Print format: `opc=%d sid=%02d addr=0x%04x ret=%d`.

/// Equivalent to the `spmi_read_begin` trace event's fast assignment.
pub fn spmi_read_begin(opcode: u8, sid: u8, addr: u16) -> SpmiReadBeginEntry {
    SpmiReadBeginEntry { opcode, sid, addr }
}

/// Print format: `opc=%d sid=%02d addr=0x%04x`.

/// Equivalent to the `spmi_read_end` trace event's fast assignment.
pub unsafe fn spmi_read_end(
    opcode: u8,
    sid: u8,
    addr: u16,
    ret: i32,
    len: u8,
    buf: *const u8,
    dynamic_buf: *mut u8,
) -> SpmiReadEndEntry {
    if len != 0 {
        core::ptr::copy_nonoverlapping(buf, dynamic_buf, len as usize);
    }
    SpmiReadEndEntry { opcode, sid, addr, ret, len, buf: dynamic_buf }
}

/// Print format: `opc=%d sid=%02d addr=0x%04x ret=%d len=%02d buf=0x[%*phD]`.

/// Equivalent to the `spmi_cmd` trace event's fast assignment.
pub fn spmi_cmd(opcode: u8, sid: u8, ret: i32) -> SpmiCmdEntry {
    SpmiCmdEntry { opcode, sid, ret }
}

/// Print format: `opc=%d sid=%02d ret=%d`.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
