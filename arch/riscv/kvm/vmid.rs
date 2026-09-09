// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2019 Western Digital Corporation or its affiliates.
 *
 * Authors:
 *     Anup Patel <anup.patel@wdc.com>
 */

// Dependencies supplied by the surrounding kernel/RISC-V translation.

static mut vmid_version: ::core::ffi::c_ulong = 1;
static mut vmid_next: ::core::ffi::c_ulong = 0;
static mut vmid_bits: ::core::ffi::c_ulong = 0;
static mut vmid_lock: SpinLock = SpinLock::new();

extern "C" {
    fn csr_write(csr: ::core::ffi::c_ulong, value: ::core::ffi::c_ulong);
    fn csr_read(csr: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong;
    fn kvm_riscv_gstage_mode(levels: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong;
    fn kvm_riscv_local_hfence_gvma_all();
    fn fls_long(value: ::core::ffi::c_ulong) -> ::core::ffi::c_int;
    fn num_possible_cpus() -> ::core::ffi::c_ulong;
    fn on_each_cpu_mask(mask: *mut CpuMask, func: unsafe extern "C" fn(*mut ::core::ffi::c_void),
                        info: *mut ::core::ffi::c_void, wait: ::core::ffi::c_int);
    fn kvm_make_request(request: ::core::ffi::c_ulong, vcpu: *mut KvmVcpu);
    fn kvm_for_each_vcpu(kvm: *mut Kvm,
                         func: unsafe extern "C" fn(*mut KvmVcpu, *mut ::core::ffi::c_void),
                         data: *mut ::core::ffi::c_void);
}

// Values and types below are provided by the corresponding kernel headers.
const CSR_HGATP: ::core::ffi::c_ulong = 0;
const HGATP_MODE_SHIFT: ::core::ffi::c_ulong = 0;
const HGATP_VMID: ::core::ffi::c_ulong = 0;
const HGATP_VMID_SHIFT: ::core::ffi::c_ulong = 0;
const KVM_REQ_UPDATE_HGATP: ::core::ffi::c_ulong = 0;
static mut kvm_riscv_gstage_max_pgd_levels: ::core::ffi::c_ulong = 0;
static mut cpu_online_mask: CpuMask = CpuMask;

#[repr(C)]
pub struct KvmVmid {
    pub vmid_version: ::core::ffi::c_ulong,
    pub vmid: ::core::ffi::c_ulong,
}

#[repr(C)]
pub struct KvmArch {
    pub vmid: KvmVmid,
}

#[repr(C)]
pub struct Kvm {
    pub arch: KvmArch,
}

#[repr(C)]
pub struct KvmVcpu {
    pub kvm: *mut Kvm,
}

pub struct CpuMask;
pub struct SpinLock;
impl SpinLock {
    const fn new() -> Self { SpinLock }
    unsafe fn lock(&mut self) {}
    unsafe fn unlock(&mut self) {}
}

unsafe extern "C" fn __local_hfence_gvma_all(_info: *mut ::core::ffi::c_void) {
    kvm_riscv_local_hfence_gvma_all();
}

pub unsafe extern "C" fn kvm_riscv_gstage_vmid_detect() {
    /* Figure-out number of VMID bits in HW */
    csr_write(CSR_HGATP,
              (kvm_riscv_gstage_mode(kvm_riscv_gstage_max_pgd_levels) << HGATP_MODE_SHIFT)
                  | HGATP_VMID);
    vmid_bits = csr_read(CSR_HGATP);
    vmid_bits = (vmid_bits & HGATP_VMID) >> HGATP_VMID_SHIFT;
    vmid_bits = fls_long(vmid_bits) as ::core::ffi::c_ulong;
    csr_write(CSR_HGATP, 0);

    /* We polluted local TLB so flush all guest TLB */
    kvm_riscv_local_hfence_gvma_all();

    /* We don't use VMID bits if they are not sufficient */
    if (1 as ::core::ffi::c_ulong).wrapping_shl(vmid_bits as u32) < num_possible_cpus() {
        vmid_bits = 0;
    }
}

pub unsafe extern "C" fn kvm_riscv_gstage_vmid_bits() -> ::core::ffi::c_ulong {
    vmid_bits
}

pub unsafe extern "C" fn kvm_riscv_gstage_vmid_init(kvm: *mut Kvm) -> ::core::ffi::c_int {
    (*kvm).arch.vmid.vmid_version = 0;
    (*kvm).arch.vmid.vmid = 0;
    0
}

pub unsafe extern "C" fn kvm_riscv_gstage_vmid_ver_changed(vmid: *mut KvmVmid) -> bool {
    if vmid_bits == 0 {
        return false;
    }
    ::core::ptr::read_volatile(&(*vmid).vmid_version)
        != ::core::ptr::read_volatile(&vmid_version)
}

pub unsafe extern "C" fn kvm_riscv_gstage_vmid_update(vcpu: *mut KvmVcpu) {
    let vmid = &mut (*(*vcpu).kvm).arch.vmid as *mut KvmVmid;

    if !kvm_riscv_gstage_vmid_ver_changed(vmid) {
        return;
    }

    vmid_lock.lock();

    /* Re-check the version to ensure another VCPU has not allocated a valid VMID. */
    if !kvm_riscv_gstage_vmid_ver_changed(vmid) {
        vmid_lock.unlock();
        return;
    }

    /* First user of a new VMID version? */
    if vmid_next == 0 {
        vmid_version = vmid_version.wrapping_add(1);
        vmid_next = 1;

        /* Existing VMIDs are invalid; flush all guest TLBs on all host CPUs. */
        on_each_cpu_mask(&mut cpu_online_mask, __local_hfence_gvma_all,
                         ::core::ptr::null_mut(), 1);
    }

    (*vmid).vmid = vmid_next;
    vmid_next = vmid_next.wrapping_add(1);
    vmid_next &= (1u64.wrapping_shl(vmid_bits as u32)).wrapping_sub(1);

    ::core::ptr::write_volatile(&mut (*vmid).vmid_version, vmid_version);
    vmid_lock.unlock();

    unsafe extern "C" fn update_vcpu(v: *mut KvmVcpu, _data: *mut ::core::ffi::c_void) {
        kvm_make_request(KVM_REQ_UPDATE_HGATP, v);
    }
    kvm_for_each_vcpu((*vcpu).kvm, update_vcpu, ::core::ptr::null_mut());
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
