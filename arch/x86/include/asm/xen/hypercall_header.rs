/* Translated from hypercall.h. C includes and build-system symbols are supplied externally. */

use core::ffi::c_void;

pub struct xen_dm_op_buf;

extern "C" {
    pub fn xen_hypercall_func();
}

/* The original register-constraint macros are architecture/build dependent. */
#[cfg(target_pointer_width = "32")]
const _: &str = "hypercall arguments: eax/ebx/ecx/edx/esi/edi";
#[cfg(not(target_pointer_width = "32"))]
const _: &str = "hypercall arguments: rax/rdi/rsi/rdx/r10/r8";

/* __HYPERVISOR_* constants, Xen types, and trace_xen_mc_entry are external dependencies. */

#[inline]
pub unsafe fn xen_single_call(
    call: u32, a1: usize, a2: usize, a3: usize, a4: usize, a5: usize,
) -> isize {
    /* The C implementation invokes the static-call hypercall trampoline and clobbers all
     * unused argument registers and memory. The trampoline is supplied by the kernel. */
    let _ = (call, a1, a2, a3, a4, a5);
    core::hint::unreachable_unchecked()
}

#[inline(always)]
pub unsafe fn __xen_stac() {
    core::arch::asm!("stac", options(nostack, preserves_flags));
}

#[inline(always)]
pub unsafe fn __xen_clac() {
    core::arch::asm!("clac", options(nostack, preserves_flags));
}

#[inline]
pub unsafe fn privcmd_call(call: u32, a1: usize, a2: usize, a3: usize, a4: usize, a5: usize) -> isize {
    __xen_stac();
    let res = xen_single_call(call, a1, a2, a3, a4, a5);
    __xen_clac();
    res
}

macro_rules! _hypercall0 { ($ty:ty, $call:expr) => {{ xen_single_call($call, 0, 0, 0, 0, 0) as $ty }}; }
macro_rules! _hypercall1 { ($ty:ty, $call:expr, $a1:expr) => {{ xen_single_call($call, $a1 as usize, 0, 0, 0, 0) as $ty }}; }
macro_rules! _hypercall2 { ($ty:ty, $call:expr, $a1:expr, $a2:expr) => {{ xen_single_call($call, $a1 as usize, $a2 as usize, 0, 0, 0) as $ty }}; }
macro_rules! _hypercall3 { ($ty:ty, $call:expr, $a1:expr, $a2:expr, $a3:expr) => {{ xen_single_call($call, $a1 as usize, $a2 as usize, $a3 as usize, 0, 0) as $ty }}; }
macro_rules! _hypercall4 { ($ty:ty, $call:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr) => {{ xen_single_call($call, $a1 as usize, $a2 as usize, $a3 as usize, $a4 as usize, 0) as $ty }}; }

#[cfg(feature = "CONFIG_XEN_PV")]
#[inline]
pub unsafe fn HYPERVISOR_set_trap_table(table: *mut trap_info) -> i32 { _hypercall1!(i32, __HYPERVISOR_set_trap_table, table) }

#[inline(always)]
pub unsafe fn HYPERVISOR_sched_op(cmd: i32, arg: *mut c_void) -> i32 { _hypercall2!(i32, __HYPERVISOR_sched_op, cmd, arg) }

#[inline]
pub unsafe fn HYPERVISOR_set_timer_op(timeout: u64) -> isize {
    _hypercall2!(isize, __HYPERVISOR_set_timer_op, timeout as usize, (timeout >> 32) as usize)
}

#[inline]
pub unsafe fn HYPERVISOR_mca(mc_op: *mut xen_mc) -> i32 {
    (*mc_op).interface_version = XEN_MCA_INTERFACE_VERSION;
    _hypercall1!(i32, __HYPERVISOR_mca, mc_op)
}

#[inline]
pub unsafe fn HYPERVISOR_platform_op(op: *mut xen_platform_op) -> i32 {
    (*op).interface_version = XENPF_INTERFACE_VERSION;
    _hypercall1!(i32, __HYPERVISOR_platform_op, op)
}

#[inline] pub unsafe fn HYPERVISOR_memory_op(cmd: u32, arg: *mut c_void) -> isize { _hypercall2!(isize, memory_op, cmd, arg) }
#[inline] pub unsafe fn HYPERVISOR_multicall(call_list: *mut c_void, nr_calls: u32) -> i32 { _hypercall2!(i32, multicall, call_list, nr_calls) }
#[inline] pub unsafe fn HYPERVISOR_event_channel_op(cmd: i32, arg: *mut c_void) -> i32 { _hypercall2!(i32, event_channel_op, cmd, arg) }
#[inline(always)] pub unsafe fn HYPERVISOR_xen_version(cmd: i32, arg: *mut c_void) -> i32 { _hypercall2!(i32, xen_version, cmd, arg) }
#[inline] pub unsafe fn HYPERVISOR_console_io(cmd: i32, count: i32, str_: *mut i8) -> i32 { _hypercall3!(i32, console_io, cmd, count, str_) }
#[inline] pub unsafe fn HYPERVISOR_physdev_op(cmd: i32, arg: *mut c_void) -> i32 { _hypercall2!(i32, physdev_op, cmd, arg) }
#[inline] pub unsafe fn HYPERVISOR_grant_table_op(cmd: u32, uop: *mut c_void, count: u32) -> i32 { _hypercall3!(i32, grant_table_op, cmd, uop, count) }
#[inline] pub unsafe fn HYPERVISOR_vm_assist(cmd: u32, ty: u32) -> i32 { _hypercall2!(i32, vm_assist, cmd, ty) }
#[inline] pub unsafe fn HYPERVISOR_vcpu_op(cmd: i32, vcpuid: i32, extra_args: *mut c_void) -> i32 { _hypercall3!(i32, vcpu_op, cmd, vcpuid, extra_args) }
#[inline] pub unsafe fn HYPERVISOR_hvm_op(op: i32, arg: *mut c_void) -> usize { _hypercall2!(usize, hvm_op, op, arg) }
#[inline] pub unsafe fn HYPERVISOR_xenpmu_op(op: u32, arg: *mut c_void) -> i32 { _hypercall2!(i32, xenpmu_op, op, arg) }

#[inline]
pub unsafe fn HYPERVISOR_dm_op(dom: domid_t, nr_bufs: u32, bufs: *mut xen_dm_op_buf) -> i32 {
    __xen_stac();
    let ret = _hypercall3!(i32, dm_op, dom, nr_bufs, bufs);
    __xen_clac();
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
