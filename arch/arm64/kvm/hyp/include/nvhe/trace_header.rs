/* SPDX-License-Identifier: GPL-2.0-only */

// Dependency intent from the C header:
// linux/trace_remote_event.h and asm/kvm_hyptrace.h

/// Return the pid of the vCPU currently running in the host context.
#[inline(always)]
pub unsafe fn __tracing_get_vcpu_pid(
    mut host_ctxt: *mut kvm_cpu_context,
) -> pid_t {
    if host_ctxt.is_null() {
        host_ctxt = host_data_ptr(host_ctxt);
    }

    let vcpu: *mut kvm_vcpu = (*host_ctxt).__hyp_running_vcpu;

    if !vcpu.is_null() {
        (*vcpu).arch.pid
    } else {
        0
    }
}

// C macro passthrough equivalents.
macro_rules! HE_PROTO { ($($args:tt)*) => { $($args)* }; }
macro_rules! HE_ASSIGN { ($($args:tt)*) => { $($args)* }; }

// HE_STRUCT is RE_STRUCT and he_field is re_field in the source build.
// These names are supplied by the remote-event dependency.

#[cfg(feature = "CONFIG_NVHE_EL2_TRACING")]
macro_rules! HYP_EVENT {
    ($name:ident, ($($proto:tt)*), $struct:item, ($($assign:tt)*), $printk:tt) => {
        $struct

        extern "C" {
            static mut concat_id_not_supported: hyp_event_id;
        }

        #[inline(always)]
        unsafe fn concat_trace_not_supported($($proto)*) {
            let _ = ($($assign)*);
            let _ = $printk;
        }
    };
}

#[cfg(feature = "CONFIG_NVHE_EL2_TRACING")]
extern "C" {
    pub fn tracing_reserve_entry(length: c_ulong) -> *mut c_void;
    pub fn tracing_commit_entry();

    pub fn __tracing_load(desc_va: c_ulong, desc_size: size_t) -> c_int;
    pub fn __tracing_unload();
    pub fn __tracing_enable(enable: bool) -> c_int;
    pub fn __tracing_swap_reader(cpu: c_uint) -> c_int;
    pub fn __tracing_update_clock(mult: u32, shift: u32, epoch_ns: u64, epoch_cyc: u64);
    pub fn __tracing_reset(cpu: c_uint) -> c_int;
    pub fn __tracing_enable_event(id: c_ushort, enable: bool) -> c_int;
}

#[cfg(not(feature = "CONFIG_NVHE_EL2_TRACING"))]
#[inline(always)]
pub unsafe fn tracing_reserve_entry(_length: c_ulong) -> *mut c_void {
    core::ptr::null_mut()
}

#[cfg(not(feature = "CONFIG_NVHE_EL2_TRACING"))]
#[inline(always)]
pub unsafe fn tracing_commit_entry() {}

#[cfg(not(feature = "CONFIG_NVHE_EL2_TRACING"))]
macro_rules! HYP_EVENT {
    ($name:ident, ($($proto:tt)*), $struct:item, ($($assign:tt)*), $printk:tt) => {
        #[inline(always)]
        fn $name($($proto)*) {}
    };
}

#[cfg(not(feature = "CONFIG_NVHE_EL2_TRACING"))]
#[inline(always)]
pub unsafe fn __tracing_load(_desc_va: c_ulong, _desc_size: size_t) -> c_int { -ENODEV }

#[cfg(not(feature = "CONFIG_NVHE_EL2_TRACING"))]
#[inline(always)]
pub unsafe fn __tracing_unload() {}

#[cfg(not(feature = "CONFIG_NVHE_EL2_TRACING"))]
#[inline(always)]
pub unsafe fn __tracing_enable(_enable: bool) -> c_int { -ENODEV }

#[cfg(not(feature = "CONFIG_NVHE_EL2_TRACING"))]
#[inline(always)]
pub unsafe fn __tracing_swap_reader(_cpu: c_uint) -> c_int { -ENODEV }

#[cfg(not(feature = "CONFIG_NVHE_EL2_TRACING"))]
#[inline(always)]
pub unsafe fn __tracing_update_clock(_mult: u32, _shift: u32, _epoch_ns: u64, _epoch_cyc: u64) {}

#[cfg(not(feature = "CONFIG_NVHE_EL2_TRACING"))]
#[inline(always)]
pub unsafe fn __tracing_reset(_cpu: c_uint) -> c_int { -ENODEV }

#[cfg(not(feature = "CONFIG_NVHE_EL2_TRACING"))]
#[inline(always)]
pub unsafe fn __tracing_enable_event(_id: c_ushort, _enable: bool) -> c_int { -ENODEV }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
