/*
 * hypercall.h
 *
 * Linux-specific hypervisor handling.
 *
 * Stefano Stabellini <stefano.stabellini@eu.citrix.com>, Citrix, 2012
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU General Public License version 2
 * as published by the Free Software Foundation; or, when distributed
 * separately from the Linux kernel or incorporated into other
 * software packages, subject to the following license:
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this source file (the "Software"), to deal in the Software without
 * restriction, including without limitation the rights to use, copy, modify,
 * merge, publish, distribute, sublicense, and/or sell copies of the Software,
 * and to permit persons to whom the Software is furnished to do so, subject to
 * the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
 * FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
 * IN THE SOFTWARE.
 */

// C header guard: _ASM_ARM_XEN_HYPERCALL_H
// Dependencies supplied by the surrounding Xen/Linux translation.

#[repr(C)]
pub struct xen_dm_op_buf {
    _private: [u8; 0],
}

extern "C" {
    pub fn privcmd_call(
        call: u32,
        a1: usize,
        a2: usize,
        a3: usize,
        a4: usize,
        a5: usize,
    ) -> isize;
    pub fn HYPERVISOR_xen_version(cmd: i32, arg: *mut core::ffi::c_void) -> i32;
    pub fn HYPERVISOR_console_io(cmd: i32, count: i32, str_: *mut i8) -> i32;
    pub fn HYPERVISOR_grant_table_op(
        cmd: u32,
        uop: *mut core::ffi::c_void,
        count: u32,
    ) -> i32;
    pub fn HYPERVISOR_sched_op(cmd: i32, arg: *mut core::ffi::c_void) -> i32;
    pub fn HYPERVISOR_event_channel_op(cmd: i32, arg: *mut core::ffi::c_void) -> i32;
    pub fn HYPERVISOR_hvm_op(op: i32, arg: *mut core::ffi::c_void) -> usize;
    pub fn HYPERVISOR_memory_op(cmd: u32, arg: *mut core::ffi::c_void) -> i32;
    pub fn HYPERVISOR_physdev_op(cmd: i32, arg: *mut core::ffi::c_void) -> i32;
    pub fn HYPERVISOR_vcpu_op(cmd: i32, vcpuid: i32, extra_args: *mut core::ffi::c_void) -> i32;
    pub fn HYPERVISOR_vm_assist(cmd: u32, type_: u32) -> i32;
    pub fn HYPERVISOR_dm_op(
        domid: domid_t,
        nr_bufs: u32,
        bufs: *mut xen_dm_op_buf,
    ) -> i32;
    pub fn HYPERVISOR_platform_op_raw(arg: *mut core::ffi::c_void) -> i32;
    pub fn HYPERVISOR_multicall(calls: *mut multicall_entry, nr: u32) -> i32;
}

pub unsafe fn HYPERVISOR_platform_op(op: *mut xen_platform_op) -> i32 {
    (*op).interface_version = XENPF_INTERFACE_VERSION;
    HYPERVISOR_platform_op_raw(op.cast())
}

pub unsafe fn HYPERVISOR_suspend(start_info_mfn: usize) -> i32 {
    let mut r = sched_shutdown {
        reason: SHUTDOWN_suspend,
    };

    // start_info_mfn is unused on ARM
    let _ = start_info_mfn;
    HYPERVISOR_sched_op(SCHEDOP_shutdown, (&mut r as *mut sched_shutdown).cast())
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
