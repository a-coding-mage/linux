/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies: <asm/ptrace.h>, <uapi/asm/vm86.h>

/*
 * This is the (kernel) stack-layout when we have done a "SAVE_ALL" from vm86
 * mode - the main change is that the old segment descriptors aren't
 * useful any more and are forced to be zero by the kernel (and the
 * hardware when a trap occurs), and the real segment descriptors are
 * at the end of the structure. Look at ptrace.h to see the "normal"
 * setup. For user space layout see 'struct vm86_regs' above.
 */
#[repr(C)]
pub struct kernel_vm86_regs {
    /* normal regs, with special meaning for the segment descriptors.. */
    pub pt: pt_regs,
    /* these are specific to v86 mode: */
    pub es: u16,
    pub __esh: u16,
    pub ds: u16,
    pub __dsh: u16,
    pub fs: u16,
    pub __fsh: u16,
    pub gs: u16,
    pub __gsh: u16,
}

#[repr(C)]
pub struct vm86 {
    pub user_vm86: *mut vm86plus_struct,
    pub regs32: pt_regs,
    pub veflags: c_ulong,
    pub veflags_mask: c_ulong,
    pub saved_sp0: c_ulong,

    pub flags: c_ulong,
    pub cpu_type: c_ulong,
    pub int_revectored: revectored_struct,
    pub int21_revectored: revectored_struct,
    pub vm86plus: vm86plus_info_struct,
}

#[cfg(feature = "CONFIG_VM86")]
extern "C" {
    pub fn handle_vm86_fault(regs: *mut kernel_vm86_regs, error_code: c_long);
    pub fn handle_vm86_trap(
        regs: *mut kernel_vm86_regs,
        error_code: c_long,
        trapno: c_int,
    ) -> c_int;
    pub fn save_v86_state(regs: *mut kernel_vm86_regs, error_code: c_int);
    pub fn release_vm86_irqs(task: *mut task_struct);
}

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[cfg(feature = "CONFIG_VM86")]
pub const FIRST_VM86_IRQ: c_int = 3;
#[cfg(feature = "CONFIG_VM86")]
pub const LAST_VM86_IRQ: c_int = 15;

#[cfg(feature = "CONFIG_VM86")]
#[inline]
pub const unsafe fn invalid_vm86_irq(irq: c_int) -> c_int {
    (irq < FIRST_VM86_IRQ || irq > LAST_VM86_IRQ) as c_int
}

#[cfg(not(feature = "CONFIG_VM86"))]
#[inline]
pub unsafe fn handle_vm86_trap(
    _a: *mut kernel_vm86_regs,
    _b: c_long,
    _c: c_int,
) -> c_int {
    0
}

#[cfg(not(feature = "CONFIG_VM86"))]
#[inline]
pub unsafe fn save_v86_state(_a: *mut kernel_vm86_regs, _b: c_int) {}

#[cfg(not(feature = "CONFIG_VM86"))]
#[inline]
pub unsafe fn handle_vm86_fault(_a: *mut kernel_vm86_regs, _b: c_long) {}

// The C free_vm86() macro accesses thread_struct::vm86 and calls kfree().
// Its exact field type and allocator are supplied by the including kernel.
#[macro_export]
macro_rules! free_vm86 {
    ($task:expr) => {{
        let __t = $task;
        unsafe {
            if !(*__t).vm86.is_null() {
                kfree((*__t).vm86);
                (*__t).vm86 = core::ptr::null_mut();
            }
        }
    }};
}

#[cfg(not(feature = "CONFIG_VM86"))]
#[macro_export]
macro_rules! free_vm86 {
    ($task:expr) => {{
        let _ = $task;
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
