// SPDX-License-Identifier: GPL-2.0
//
// C headers supplied by the surrounding Xen and x86 implementation are
// intentionally not reproduced here; their symbols are external dependencies.

use core::ffi::c_void;

extern "C" {
    fn HYPERVISOR_xen_version(cmd: u32, arg: *mut c_void) -> i32;
    fn HYPERVISOR_sched_op(cmd: u32, arg: *mut c_void) -> i32;
    fn HYPERVISOR_vcpu_op(cmd: u32, vcpu: u32, arg: *mut c_void) -> i32;
    fn irqs_disabled() -> bool;
    fn xen_vcpu_nr(cpu: u32) -> u32;
    fn smp_processor_id() -> u32;
    fn xen_init_IRQ();
    fn BUG();
    fn paravirt_ret0() -> usize;
    fn paravirt_nop() -> usize;
    fn BUG_func() -> usize;
}

const SCHEDOP_BLOCK: u32 = 2;
const VCPUOP_DOWN: u32 = 1;

#[repr(C)]
pub struct PvIrqOps {
    pub save_fl: unsafe extern "C" fn() -> usize,
    pub irq_disable: unsafe extern "C" fn() -> usize,
    pub irq_enable: unsafe extern "C" fn() -> usize,
    pub safe_halt: unsafe extern "C" fn(),
    pub halt: unsafe extern "C" fn(),
}

#[repr(C)]
pub struct PvOps {
    pub irq: PvIrqOps,
}

#[repr(C)]
pub struct X86IrqInit {
    pub intr_init: unsafe extern "C" fn(),
}

#[repr(C)]
pub struct X86Init {
    pub irqs: X86IrqInit,
}

extern "C" {
    static mut pv_ops: PvOps;
    static mut x86_init: X86Init;
}

/*
 * Force a proper event-channel callback from Xen after clearing the
 * callback mask. We do this in a very simple manner, by making a call
 * down into Xen. The pending flag will be checked by Xen on return.
 */
#[no_mangle]
pub unsafe extern "C" fn xen_force_evtchn_callback() {
    let _ = HYPERVISOR_xen_version(0, core::ptr::null_mut());
}

unsafe extern "C" fn xen_safe_halt() {
    /* Blocking includes an implicit local_irq_enable(). */
    if HYPERVISOR_sched_op(SCHEDOP_BLOCK, core::ptr::null_mut()) != 0 {
        BUG();
    }
}

unsafe extern "C" fn xen_halt() {
    if irqs_disabled() {
        HYPERVISOR_vcpu_op(
            VCPUOP_DOWN,
            xen_vcpu_nr(smp_processor_id()),
            core::ptr::null_mut(),
        );
    } else {
        xen_safe_halt();
    }
}

#[no_mangle]
pub unsafe extern "C" fn xen_init_irq_ops() {
    /* Initial interrupt flag handling only called while interrupts off. */
    pv_ops.irq.save_fl = core::mem::transmute(paravirt_ret0 as unsafe extern "C" fn() -> usize);
    pv_ops.irq.irq_disable = core::mem::transmute(paravirt_nop as unsafe extern "C" fn() -> usize);
    pv_ops.irq.irq_enable = core::mem::transmute(BUG_func as unsafe extern "C" fn() -> usize);
    pv_ops.irq.safe_halt = xen_safe_halt;
    pv_ops.irq.halt = xen_halt;

    x86_init.irqs.intr_init = xen_init_IRQ;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
