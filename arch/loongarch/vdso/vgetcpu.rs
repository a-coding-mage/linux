// SPDX-License-Identifier: GPL-2.0-only
/*
 * Fast user context implementation of getcpu()
 */

// Dependency supplied by the architecture vdso headers.
extern "C" {
    static vdso_u_arch_data: VdsoUArchData;
}

#[repr(C)]
pub struct VdsoUArchData {
    pub pdata: *const VdsoArchData,
}

#[repr(C)]
pub struct VdsoArchData {
    pub node: core::ffi::c_uint,
}

#[inline(always)]
unsafe fn read_cpu_id() -> i32 {
    let mut cpu_id: i32;

    #[cfg(CONFIG_64BIT)]
    core::arch::asm!(
        "rdtime.d $zero, {cpu_id}",
        cpu_id = out(reg) cpu_id,
        options(nostack)
    );

    #[cfg(not(CONFIG_64BIT))]
    core::arch::asm!(
        "rdtimel.w $zero, {cpu_id}",
        cpu_id = out(reg) cpu_id,
        options(nostack)
    );

    cpu_id
}

pub unsafe fn __vdso_getcpu(
    cpu: *mut core::ffi::c_uint,
    node: *mut core::ffi::c_uint,
    _unused: *mut core::ffi::c_void,
) -> i32 {
    let cpu_id: i32;

    cpu_id = read_cpu_id();

    if !cpu.is_null() {
        *cpu = cpu_id as core::ffi::c_uint;
    }

    if !node.is_null() {
        *node = (*vdso_u_arch_data.pdata.add(cpu_id as usize)).node;
    }

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
