/* SPDX-License-Identifier: GPL-2.0 */

// Translated from tdx.h. The CONFIG_KVM_INTEL_TDX condition is preserved as
// comments because its value is supplied by the surrounding build.

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum KvmTdxState {
    TD_STATE_UNINITIALIZED = 0,
    TD_STATE_INITIALIZED,
    TD_STATE_RUNNABLE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum VcpuTdxState {
    VCPU_TD_STATE_UNINITIALIZED = 0,
    VCPU_TD_STATE_INITIALIZED,
}

#[repr(C)]
pub struct KvmTdx {
    pub kvm: Kvm,
    pub misc_cg: *mut MiscCg,
    pub hkid: i32,
    pub state: KvmTdxState,
    pub attributes: u64,
    pub xfam: u64,
    pub tsc_offset: u64,
    pub tsc_multiplier: u64,
    pub td: TdxTd,
    /* Scratch pointer used to pass the source page to tdx_mem_page_add().
     * Protected by slots_lock, and non-NULL only when mapping a private
     * pfn via tdx_gmem_post_populate(). */
    pub page_add_src: *mut Page,
    /* Prevent vCPUs from TD entry to ensure SEPT zap related SEAMCALLs do
     * not contend with tdh_vp_enter() and TDCALLs.
     * Set/unset is protected with kvm->mmu_lock. */
    pub wait_for_sept_zap: bool,
}

#[repr(C)]
pub struct VcpuTdx {
    pub vcpu: KvmVcpu,
    pub vt: VcpuVt,
    pub ext_exit_qualification: u64,
    pub exit_gpa: GpaT,
    pub vp_enter_args: TdxModuleArgs,
    pub vp: TdxVp,
    pub cpu_list: ListHead,
    pub vp_enter_ret: u64,
    pub state: VcpuTdxState,
    pub map_gpa_next: u64,
    pub map_gpa_end: u64,
}

extern "C" {
    pub fn tdx_hardware_setup() -> i32;
    pub fn tdx_hardware_unsetup();
    pub static mut enable_tdx: bool;

    pub fn tdh_vp_rd_failed(tdx: *mut VcpuTdx, uclass: *mut i8, field: u32, err: u64);
    pub fn tdh_vp_wr_failed(
        tdx: *mut VcpuTdx,
        uclass: *mut i8,
        op: *mut i8,
        field: u32,
        val: u64,
        err: u64,
    );

    pub fn tdh_mng_rd(td: *mut TdxTd, field: u64, data: *mut u64) -> u64;
    pub fn tdh_vp_rd(vp: *mut TdxVp, field: u64, data: *mut u64) -> u64;
    pub fn tdh_vp_wr(vp: *mut TdxVp, field: u64, val: u64, mask: u64) -> u64;
    pub fn pr_err(fmt: *const i8, ...);
}

pub unsafe fn td_tdcs_exec_read64(kvm_tdx: *mut KvmTdx, field: u32) -> u64 {
    let mut data: u64 = 0;
    let err = tdh_mng_rd(&mut (*kvm_tdx).td, TDCS_EXEC(field), &mut data);
    if err != 0 {
        pr_err(b"TDH_MNG_RD[EXEC.0x%x] failed: 0x%llx\0".as_ptr() as *const i8,
               field, err);
        return 0;
    }
    data
}

pub unsafe fn tdvps_vmcs_check(field: u32, bits: u8) {
    // TDX is 64bit only. HIGH field isn't supported.
    // BUILD_BUG_ON_MSG and BUILD_BUG_ON are compile-time checks in the C source.
    let _ = field;
    assert!(bits == 16 || bits == 32 || bits == 64);
}

pub unsafe fn tdvps_management_check(_field: u64, _bits: u8) {}
pub unsafe fn tdvps_state_non_arch_check(_field: u64, _bits: u8) {}

pub unsafe fn td_vmcs_read16(tdx: *mut VcpuTdx, field: u32) -> u16 {
    tdvps_vmcs_check(field, 16); let mut data = 0; let err = tdh_vp_rd(&mut (*tdx).vp, TDVPS_VMCS(field), &mut data);
    if err != 0 { tdh_vp_rd_failed(tdx, b"VMCS\0".as_ptr() as *mut i8, field, err); 0 } else { data as u16 }
}
pub unsafe fn td_vmcs_read32(tdx: *mut VcpuTdx, field: u32) -> u32 {
    tdvps_vmcs_check(field, 32); let mut data = 0; let err = tdh_vp_rd(&mut (*tdx).vp, TDVPS_VMCS(field), &mut data);
    if err != 0 { tdh_vp_rd_failed(tdx, b"VMCS\0".as_ptr() as *mut i8, field, err); 0 } else { data as u32 }
}
pub unsafe fn td_vmcs_read64(tdx: *mut VcpuTdx, field: u32) -> u64 {
    tdvps_vmcs_check(field, 64); let mut data = 0; let err = tdh_vp_rd(&mut (*tdx).vp, TDVPS_VMCS(field), &mut data);
    if err != 0 { tdh_vp_rd_failed(tdx, b"VMCS\0".as_ptr() as *mut i8, field, err); 0 } else { data }
}

// Macro-generated management/state accessors retain the same operations.
pub unsafe fn td_management_read8(tdx: *mut VcpuTdx, field: u32) -> u8 { tdvps_management_check(field as u64, 8); let mut d=0; let e=tdh_vp_rd(&mut (*tdx).vp, TDVPS_MANAGEMENT(field), &mut d); if e!=0 { tdh_vp_rd_failed(tdx,b"MANAGEMENT\0".as_ptr() as *mut i8,field,e); 0 } else { d as u8 } }
pub unsafe fn td_state_non_arch_read64(tdx: *mut VcpuTdx, field: u32) -> u64 { tdvps_state_non_arch_check(field as u64,64); let mut d=0; let e=tdh_vp_rd(&mut (*tdx).vp, TDVPS_STATE_NON_ARCH(field), &mut d); if e!=0 { tdh_vp_rd_failed(tdx,b"STATE_NON_ARCH\0".as_ptr() as *mut i8,field,e); 0 } else { d } }

macro_rules! tdx_write_accessors {
    ($check:ident, $encode:ident, $readty:ty, $write:ident, $setbit:ident, $clearbit:ident, $bits:expr, $class:expr) => {
        pub unsafe fn $write(tdx: *mut VcpuTdx, field: u32, val: $readty) {
            $check(field as u64, $bits);
            let err = tdh_vp_wr(&mut (*tdx).vp, $encode(field), val as u64,
                                if $bits == 64 { u64::MAX } else { (1u64 << $bits) - 1 });
            if err != 0 { tdh_vp_wr_failed(tdx, $class.as_ptr() as *mut i8, b" = \0".as_ptr() as *mut i8, field, val as u64, err); }
        }
        pub unsafe fn $setbit(tdx: *mut VcpuTdx, field: u32, bit: u64) {
            $check(field as u64, $bits); let err = tdh_vp_wr(&mut (*tdx).vp, $encode(field), bit, bit);
            if err != 0 { tdh_vp_wr_failed(tdx, $class.as_ptr() as *mut i8, b" |= \0".as_ptr() as *mut i8, field, bit, err); }
        }
        pub unsafe fn $clearbit(tdx: *mut VcpuTdx, field: u32, bit: u64) {
            $check(field as u64, $bits); let err = tdh_vp_wr(&mut (*tdx).vp, $encode(field), 0, bit);
            if err != 0 { tdh_vp_wr_failed(tdx, $class.as_ptr() as *mut i8, b" &= ~\0".as_ptr() as *mut i8, field, bit, err); }
        }
    };
}

tdx_write_accessors!(tdvps_vmcs_check, TDVPS_VMCS, u16, td_vmcs_write16, td_vmcs_setbit16, td_vmcs_clearbit16, 16, b"VMCS");
tdx_write_accessors!(tdvps_vmcs_check, TDVPS_VMCS, u32, td_vmcs_write32, td_vmcs_setbit32, td_vmcs_clearbit32, 32, b"VMCS");
tdx_write_accessors!(tdvps_vmcs_check, TDVPS_VMCS, u64, td_vmcs_write64, td_vmcs_setbit64, td_vmcs_clearbit64, 64, b"VMCS");
tdx_write_accessors!(tdvps_management_check, TDVPS_MANAGEMENT, u8, td_management_write8, td_management_setbit8, td_management_clearbit8, 8, b"MANAGEMENT");
tdx_write_accessors!(tdvps_state_non_arch_check, TDVPS_STATE_NON_ARCH, u64, td_state_non_arch_write64, td_state_non_arch_setbit64, td_state_non_arch_clearbit64, 64, b"STATE_NON_ARCH");

pub unsafe fn tdx_interrupt_allowed(_vcpu: *mut KvmVcpu) -> bool { true }
pub unsafe fn tdx_complete_emulated_msr(_vcpu: *mut KvmVcpu, _err: i32) -> i32 { 0 }

// External types and field-encoding helpers are supplied by the translated dependencies.
extern "Rust" {
    type Kvm; type MiscCg; type Page; type TdxTd; type KvmVcpu; type VcpuVt;
    type GpaT; type TdxModuleArgs; type TdxVp; type ListHead;
    fn TDCS_EXEC(field: u32) -> u64;
    fn TDVPS_VMCS(field: u32) -> u64;
    fn TDVPS_MANAGEMENT(field: u32) -> u64;
    fn TDVPS_STATE_NON_ARCH(field: u32) -> u64;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
