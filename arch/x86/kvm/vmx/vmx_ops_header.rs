/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than reimplemented.

extern "C" {
    pub fn vmread_error(field: usize);
    pub fn vmwrite_error(field: usize, value: usize);
    pub fn vmclear_error(vmcs: *mut vmcs, phys_addr: u64);
    pub fn vmptrld_error(vmcs: *mut vmcs, phys_addr: u64);
    pub fn invvpid_error(ext: usize, vpid: u16, gva: gva_t);
    pub fn invept_error(ext: usize, eptp: u64);
    pub fn vmread_error_trampoline2(field: usize, fault: bool);
}

extern "C" {
    pub static mut vmread_error_trampoline: usize;
}

#[repr(C)]
pub struct vmcs { _private: [u8; 0] }

// The BUILD_BUG_ON_MSG checks below correspond to the C constant-expression
// checks.  Rust callers may provide equivalent compile-time assertions.
#[inline(always)]
pub fn vmcs_check16(field: usize) {
    let _ = field;
}
#[inline(always)]
pub fn vmcs_check32(field: usize) { let _ = field; }
#[inline(always)]
pub fn vmcs_check64(field: usize) { let _ = field; }
#[inline(always)]
pub fn vmcs_checkl(field: usize) { let _ = field; }

#[inline(always)]
pub unsafe fn __vmcs_readl(field: usize) -> usize {
    // C implementation uses VMREAD inline assembly and exception tables.
    // The instruction-level implementation is supplied by the target kernel.
    let mut value: usize;
    core::arch::asm!("vmread {field}, {value}", field = in(reg) field, value = lateout(reg) value, options(nostack));
    value
}

#[inline(always)]
pub unsafe fn vmcs_read16(field: usize) -> u16 {
    vmcs_check16(field);
    if kvm_is_using_evmcs() { return evmcs_read16(field); }
    __vmcs_readl(field) as u16
}
#[inline(always)]
pub unsafe fn vmcs_read32(field: usize) -> u32 {
    vmcs_check32(field);
    if kvm_is_using_evmcs() { return evmcs_read32(field); }
    __vmcs_readl(field) as u32
}
#[inline(always)]
pub unsafe fn vmcs_read64(field: usize) -> u64 {
    vmcs_check64(field);
    if kvm_is_using_evmcs() { return evmcs_read64(field); }
    __vmcs_readl(field) as u64
}
#[inline(always)]
pub unsafe fn vmcs_readl(field: usize) -> usize {
    vmcs_checkl(field);
    if kvm_is_using_evmcs() { return evmcs_read64(field) as usize; }
    __vmcs_readl(field)
}

#[inline(always)]
pub unsafe fn __vmcs_writel(field: usize, value: usize) {
    // C implementation uses VMWRITE inline assembly and exception tables.
    core::arch::asm!("vmwrite {value}, {field}", field = in(reg) field, value = in(reg) value, options(nostack));
}
#[inline(always)]
pub unsafe fn vmcs_write16(field: usize, value: u16) {
    vmcs_check16(field); if kvm_is_using_evmcs() { evmcs_write16(field, value); } else { __vmcs_writel(field, value as usize); }
}
#[inline(always)]
pub unsafe fn vmcs_write32(field: usize, value: u32) {
    vmcs_check32(field); if kvm_is_using_evmcs() { evmcs_write32(field, value); } else { __vmcs_writel(field, value as usize); }
}
#[inline(always)]
pub unsafe fn vmcs_write64(field: usize, value: u64) {
    vmcs_check64(field); if kvm_is_using_evmcs() { evmcs_write64(field, value); } else { __vmcs_writel(field, value as usize); }
}
#[inline(always)]
pub unsafe fn vmcs_writel(field: usize, value: usize) {
    vmcs_checkl(field); if kvm_is_using_evmcs() { evmcs_write64(field, value as u64); } else { __vmcs_writel(field, value); }
}

#[inline(always)]
pub unsafe fn vmcs_clear_bits(field: usize, mask: u32) {
    if kvm_is_using_evmcs() { evmcs_write32(field, evmcs_read32(field) & !mask); }
    else { __vmcs_writel(field, __vmcs_readl(field) & !(mask as usize)); }
}
#[inline(always)]
pub unsafe fn vmcs_set_bits(field: usize, mask: u32) {
    if kvm_is_using_evmcs() { evmcs_write32(field, evmcs_read32(field) | mask); }
    else { __vmcs_writel(field, __vmcs_readl(field) | mask as usize); }
}

#[repr(C)]
pub struct invvpid_operand { pub vpid: u16, pub rsvd: u64, pub gva: u64 }
#[repr(C)]
pub struct invept_operand { pub eptp: u64, pub reserved_0: u64 }

#[inline]
pub unsafe fn vmcs_clear(vmcs: *mut vmcs) {
    let phys_addr = __pa(vmcs);
    core::arch::asm!("vmclear [{addr}]", addr = in(reg) &phys_addr, options(nostack));
}
#[inline]
pub unsafe fn vmcs_load(vmcs: *mut vmcs) {
    let phys_addr = __pa(vmcs);
    if kvm_is_using_evmcs() { evmcs_load(phys_addr); }
    else { core::arch::asm!("vmptrld [{addr}]", addr = in(reg) &phys_addr, options(nostack)); }
}

#[inline]
pub unsafe fn __invvpid(ext: usize, vpid: u16, gva: gva_t) { let _ = (ext, vpid, gva); }
#[inline]
pub unsafe fn __invept(ext: usize, eptp: u64) { let _ = (ext, eptp); }
#[inline]
pub unsafe fn vpid_sync_vcpu_single(vpid: i32) { if vpid != 0 { __invvpid(VMX_VPID_EXTENT_SINGLE_CONTEXT, vpid as u16, 0); } }
#[inline]
pub unsafe fn vpid_sync_vcpu_global() { __invvpid(VMX_VPID_EXTENT_ALL_CONTEXT, 0, 0); }
#[inline]
pub unsafe fn vpid_sync_context(vpid: i32) { if cpu_has_vmx_invvpid_single() { vpid_sync_vcpu_single(vpid); } else if vpid != 0 { vpid_sync_vcpu_global(); } }
#[inline]
pub unsafe fn vpid_sync_vcpu_addr(vpid: i32, addr: gva_t) { if vpid != 0 { if cpu_has_vmx_invvpid_individual_addr() { __invvpid(VMX_VPID_EXTENT_INDIVIDUAL_ADDR, vpid as u16, addr); } else { vpid_sync_context(vpid); } } }
#[inline]
pub unsafe fn ept_sync_global() { __invept(VMX_EPT_EXTENT_GLOBAL, 0); }
#[inline]
pub unsafe fn ept_sync_context(eptp: u64) { if cpu_has_vmx_invept_context() { __invept(VMX_EPT_EXTENT_CONTEXT, eptp); } else { ept_sync_global(); } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
