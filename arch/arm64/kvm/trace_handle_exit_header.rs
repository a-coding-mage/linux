/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies supplied by the surrounding kernel translation:
// linux/tracepoint.h, sys_regs.h, and trace/define_trace.h.
// TRACE_SYSTEM kvm

#[repr(C)]
pub struct KvmWfxArm64Entry {
    pub vcpu_pc: ::core::ffi::c_ulong,
    pub is_wfe: bool,
}

#[repr(C)]
pub struct KvmHvcArm64Entry {
    pub vcpu_pc: ::core::ffi::c_ulong,
    pub r0: ::core::ffi::c_ulong,
    pub imm: ::core::ffi::c_ulong,
}

/*
 * The dreg32 name is a leftover from a distant past. This will really
 * output a 64bit value...
 */
#[repr(C)]
pub struct KvmArmSetDreg32Entry {
    pub name: *const ::core::ffi::c_char,
    pub value: u64,
}

#[repr(C)]
pub struct KvmHandleSysRegEntry {
    pub hsr: ::core::ffi::c_ulong,
}

#[repr(C)]
pub struct KvmSysAccessEntry {
    pub vcpu_pc: ::core::ffi::c_ulong,
    pub is_write: bool,
    pub name: *const ::core::ffi::c_char,
    pub Op0: u8,
    pub Op1: u8,
    pub CRn: u8,
    pub CRm: u8,
    pub Op2: u8,
}

#[repr(C)]
pub struct KvmSetGuestDebugEntry {
    pub vcpu: *mut KvmVcpu,
    pub guest_debug: u32,
}

// External type supplied by the surrounding kernel translation.
#[repr(C)]
pub struct KvmVcpu {
    _private: [u8; 0],
}

// External types supplied by sys_regs.h.
#[repr(C)]
pub struct SysRegParams {
    pub is_write: bool,
}

#[repr(C)]
pub struct SysRegDesc {
    pub name: *const ::core::ffi::c_char,
    pub Op0: u8,
    pub Op1: u8,
    pub CRn: u8,
    pub CRm: u8,
    pub Op2: u8,
}

#[inline]
pub unsafe fn kvm_wfx_arm64(vcpu_pc: ::core::ffi::c_ulong, is_wfe: bool) -> KvmWfxArm64Entry {
    KvmWfxArm64Entry { vcpu_pc, is_wfe }
}

#[inline]
pub unsafe fn kvm_hvc_arm64(
    vcpu_pc: ::core::ffi::c_ulong,
    r0: ::core::ffi::c_ulong,
    imm: ::core::ffi::c_ulong,
) -> KvmHvcArm64Entry {
    KvmHvcArm64Entry { vcpu_pc, r0, imm }
}

#[inline]
pub unsafe fn kvm_arm_set_dreg32(name: *const ::core::ffi::c_char, value: u64) -> KvmArmSetDreg32Entry {
    KvmArmSetDreg32Entry { name, value }
}

#[inline]
pub unsafe fn kvm_handle_sys_reg(hsr: ::core::ffi::c_ulong) -> KvmHandleSysRegEntry {
    KvmHandleSysRegEntry { hsr }
}

#[inline]
pub unsafe fn kvm_sys_access(
    vcpu_pc: ::core::ffi::c_ulong,
    params: *mut SysRegParams,
    reg: *const SysRegDesc,
) -> KvmSysAccessEntry {
    KvmSysAccessEntry {
        vcpu_pc,
        is_write: (*params).is_write,
        name: (*reg).name,
        Op0: (*reg).Op0,
        Op1: (*reg).Op1,
        CRn: (*reg).CRn,
        CRm: (*reg).CRm,
        Op2: (*reg).Op2,
    }
}

#[inline]
pub unsafe fn kvm_set_guest_debug(vcpu: *mut KvmVcpu, guest_debug: u32) -> KvmSetGuestDebugEntry {
    KvmSetGuestDebugEntry { vcpu, guest_debug }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
