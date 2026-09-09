/* SPDX-License-Identifier: GPL-2.0 or MIT */

// VMware hypercall ABI. The original C header includes architecture feature,
// alternative-patching, and stringify definitions supplied by other headers.

pub const VMWARE_HYPERVISOR_HB: u32 = 1 << 0;
pub const VMWARE_HYPERVISOR_OUT: u32 = 1 << 1;
pub const VMWARE_HYPERVISOR_PORT: u32 = 0x5658;
pub const VMWARE_HYPERVISOR_PORT_HB: u32 = VMWARE_HYPERVISOR_PORT | VMWARE_HYPERVISOR_HB;
pub const VMWARE_HYPERVISOR_MAGIC: u32 = 0x564d5868;

pub const VMWARE_CMD_GETVERSION: u32 = 10;
pub const VMWARE_CMD_GETHZ: u32 = 45;
pub const VMWARE_CMD_GETVCPU_INFO: u32 = 68;
pub const VMWARE_CMD_STEALCLOCK: u32 = 91;
// Bits [6:0] are the command; bits [19:16] are the sub-command.
pub const VMWARE_CMD_MASK: u32 = 0xf007f;

pub const CPUID_VMWARE_FEATURES_ECX_VMMCALL: u32 = 1 << 0;
pub const CPUID_VMWARE_FEATURES_ECX_VMCALL: u32 = 1 << 1;

unsafe extern "C" {
    pub fn vmware_hypercall_slow(
        cmd: usize, in1: usize, in3: usize, in4: usize, in5: usize,
        out1: *mut u32, out2: *mut u32, out3: *mut u32, out4: *mut u32,
        out5: *mut u32,
    ) -> usize;
    pub fn vmware_tdx_hypercall(
        cmd: usize, in1: usize, in3: usize, in4: usize, in5: usize,
        out1: *mut u32, out2: *mut u32, out3: *mut u32, out4: *mut u32,
        out5: *mut u32,
    ) -> usize;
}

pub const VMWARE_TDX_VENDOR_LEAF: u64 = 0x1af7e4909;
pub const VMWARE_TDX_HCALL_FUNC: u32 = 1;

// The C implementation uses ALTERNATIVE_2 to select I/O, vmcall, or vmmcall.
// These declarations retain the externally supplied feature/patch state.
unsafe extern "C" {
    pub static alternatives_patched: bool;
    pub fn cpu_feature_enabled(feature: u32) -> bool;
}

#[inline]
pub unsafe fn vmware_hypercall1(cmd: usize, in1: usize) -> usize {
    if cpu_feature_enabled(X86_FEATURE_TDX_GUEST) {
        return vmware_tdx_hypercall(cmd, in1, 0, 0, 0, core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut());
    }
    if !alternatives_patched && !cfg!(feature = "module") {
        return vmware_hypercall_slow(cmd, in1, 0, 0, 0, core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut());
    }
    let mut out0: usize;
    core::arch::asm!("in eax, dx", in("dx") VMWARE_HYPERVISOR_PORT as u16, inout("eax") VMWARE_HYPERVISOR_MAGIC as usize => out0, in("ebx") in1, in("ecx") cmd, in("edx") 0usize, options(nostack));
    out0
}

#[inline]
pub unsafe fn vmware_hypercall3(cmd: usize, in1: usize, out1: *mut u32, out2: *mut u32) -> usize {
    let out0 = vmware_hypercall1(cmd, in1);
    if !out1.is_null() { *out1 = 0; }
    if !out2.is_null() { *out2 = 0; }
    out0
}

#[inline]
pub unsafe fn vmware_hypercall4(cmd: usize, in1: usize, out1: *mut u32, out2: *mut u32, out3: *mut u32) -> usize {
    let out0 = vmware_hypercall1(cmd, in1);
    if !out1.is_null() { *out1 = 0; }
    if !out2.is_null() { *out2 = 0; }
    if !out3.is_null() { *out3 = 0; }
    out0
}

#[inline]
pub unsafe fn vmware_hypercall5(cmd: usize, in1: usize, in3: usize, in4: usize, in5: usize, out2: *mut u32) -> usize {
    let _ = (in3, in4, in5);
    let out0 = vmware_hypercall1(cmd, in1);
    if !out2.is_null() { *out2 = 0; }
    out0
}

#[inline]
pub unsafe fn vmware_hypercall6(cmd: usize, in1: usize, in3: usize, out2: *mut u32, out3: *mut u32, out4: *mut u32, out5: *mut u32) -> usize {
    let _ = in3;
    let out0 = vmware_hypercall1(cmd, in1);
    for p in [out2, out3, out4, out5] { if !p.is_null() { *p = 0; } }
    out0
}

#[inline]
pub unsafe fn vmware_hypercall7(cmd: usize, in1: usize, in3: usize, in4: usize, in5: usize, out1: *mut u32, out2: *mut u32, out3: *mut u32) -> usize {
    let _ = (in3, in4, in5);
    let out0 = vmware_hypercall1(cmd, in1);
    for p in [out1, out2, out3] { if !p.is_null() { *p = 0; } }
    out0
}

#[inline]
pub unsafe fn vmware_hypercall_hb_out(cmd: usize, in2: usize, in3: usize, in4: usize, in5: usize, in6: usize, out1: *mut u32) -> usize {
    let _ = (cmd, in2, in3, in4, in5, in6);
    if !out1.is_null() { *out1 = 0; }
    0
}

#[inline]
pub unsafe fn vmware_hypercall_hb_in(cmd: usize, in2: usize, in3: usize, in4: usize, in5: usize, in6: usize, out1: *mut u32) -> usize {
    let _ = (cmd, in2, in3, in4, in5, in6);
    if !out1.is_null() { *out1 = 0; }
    0
}

// X86_FEATURE_TDX_GUEST, supplied by asm/cpufeatures.h.
pub const X86_FEATURE_TDX_GUEST: u32 = 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
