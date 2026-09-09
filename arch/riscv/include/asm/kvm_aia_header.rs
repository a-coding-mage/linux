/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2021 Western Digital Corporation or its affiliates.
 * Copyright (C) 2022 Ventana Micro Systems Inc.
 *
 * Authors:
 *	Anup Patel <apatel@ventanamicro.com>
 */

// Dependencies supplied by the surrounding kernel translation.

#[repr(C)]
pub struct kvm_aia {
    /* In-kernel irqchip created */
    pub in_kernel: bool,
    /* In-kernel irqchip initialized */
    pub initialized: bool,
    /* Virtualization mode (Emulation, HW Accelerated, or Auto) */
    pub mode: u32,
    /* Number of MSIs */
    pub nr_ids: u32,
    /* Number of wired IRQs */
    pub nr_sources: u32,
    /* Number of group bits in IMSIC address */
    pub nr_group_bits: u32,
    /* Position of group bits in IMSIC address */
    pub nr_group_shift: u32,
    /* Number of hart bits in IMSIC address */
    pub nr_hart_bits: u32,
    /* Number of guest bits in IMSIC address */
    pub nr_guest_bits: u32,
    /* Guest physical address of APLIC */
    pub aplic_addr: gpa_t,
    /* Internal state of APLIC */
    pub aplic_state: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct kvm_vcpu_aia_csr {
    pub vsiselect: c_ulong,
    pub hviprio1: c_ulong,
    pub hviprio2: c_ulong,
    pub vsieh: c_ulong,
    pub hviph: c_ulong,
    pub hviprio1h: c_ulong,
    pub hviprio2h: c_ulong,
}

#[repr(C)]
pub struct kvm_vcpu_aia {
    /* CPU AIA CSR context of Guest VCPU */
    pub guest_csr: kvm_vcpu_aia_csr,
    /* Guest physical address of IMSIC for this VCPU */
    pub imsic_addr: gpa_t,
    /* HART index of IMSIC extacted from guest physical address */
    pub hart_index: u32,
    /* Internal state of IMSIC for this VCPU */
    pub imsic_state: *mut core::ffi::c_void,
}

pub const KVM_RISCV_AIA_UNDEF_ADDR: i32 = -1;

#[inline]
pub unsafe fn kvm_riscv_aia_initialized(k: *const kvm) -> bool {
    (*k).arch.aia.initialized
}

#[inline]
pub unsafe fn irqchip_in_kernel(k: *const kvm) -> bool {
    (*k).arch.aia.in_kernel
}

extern "C" {
    pub static mut kvm_riscv_aia_nr_hgei: atomic_t;
    pub static mut kvm_riscv_aia_max_ids: c_uint;
    pub static mut kvm_riscv_aia_available: static_key_false;
    pub static mut kvm_riscv_aia_device_ops: kvm_device_ops;

    pub fn kvm_riscv_vcpu_aia_imsic_has_interrupt(vcpu: *mut kvm_vcpu) -> bool;
    pub fn kvm_riscv_vcpu_aia_imsic_load(vcpu: *mut kvm_vcpu, cpu: c_int);
    pub fn kvm_riscv_vcpu_aia_imsic_put(vcpu: *mut kvm_vcpu);
    pub fn kvm_riscv_vcpu_aia_imsic_release(vcpu: *mut kvm_vcpu);
    pub fn kvm_riscv_vcpu_aia_imsic_update(vcpu: *mut kvm_vcpu) -> c_int;
    pub fn kvm_riscv_vcpu_aia_imsic_rmw(vcpu: *mut kvm_vcpu, isel: c_ulong, val: *mut c_ulong, new_val: c_ulong, wr_mask: c_ulong) -> c_int;
    pub fn kvm_riscv_aia_imsic_rw_attr(kvm: *mut kvm, type_: c_ulong, write: bool, val: *mut c_ulong) -> c_int;
    pub fn kvm_riscv_aia_imsic_has_attr(kvm: *mut kvm, type_: c_ulong) -> c_int;
    pub fn kvm_riscv_vcpu_aia_imsic_reset(vcpu: *mut kvm_vcpu);
    pub fn kvm_riscv_vcpu_aia_imsic_inject(vcpu: *mut kvm_vcpu, guest_index: u32, offset: u32, iid: u32) -> c_int;
    pub fn kvm_riscv_vcpu_aia_imsic_init(vcpu: *mut kvm_vcpu) -> c_int;
    pub fn kvm_riscv_vcpu_aia_imsic_cleanup(vcpu: *mut kvm_vcpu);
    pub fn kvm_riscv_aia_aplic_set_attr(kvm: *mut kvm, type_: c_ulong, v: u32) -> c_int;
    pub fn kvm_riscv_aia_aplic_get_attr(kvm: *mut kvm, type_: c_ulong, v: *mut u32) -> c_int;
    pub fn kvm_riscv_aia_aplic_has_attr(kvm: *mut kvm, type_: c_ulong) -> c_int;
    pub fn kvm_riscv_aia_aplic_inject(kvm: *mut kvm, source: u32, level: bool) -> c_int;
    pub fn kvm_riscv_aia_aplic_init(kvm: *mut kvm) -> c_int;
    pub fn kvm_riscv_aia_aplic_cleanup(kvm: *mut kvm);

    // Under CONFIG_32BIT these are external functions; otherwise they are empty inline functions.
    #[cfg(CONFIG_32BIT)] pub fn kvm_riscv_vcpu_aia_flush_interrupts(vcpu: *mut kvm_vcpu);
    #[cfg(CONFIG_32BIT)] pub fn kvm_riscv_vcpu_aia_sync_interrupts(vcpu: *mut kvm_vcpu);
    pub fn kvm_riscv_vcpu_aia_has_interrupts(vcpu: *mut kvm_vcpu, mask: u64) -> bool;
    pub fn kvm_riscv_vcpu_aia_update_hvip(vcpu: *mut kvm_vcpu);
    pub fn kvm_riscv_vcpu_aia_load(vcpu: *mut kvm_vcpu, cpu: c_int);
    pub fn kvm_riscv_vcpu_aia_put(vcpu: *mut kvm_vcpu);
    pub fn kvm_riscv_vcpu_aia_get_csr(vcpu: *mut kvm_vcpu, reg_num: c_ulong, out_val: *mut c_ulong) -> c_int;
    pub fn kvm_riscv_vcpu_aia_set_csr(vcpu: *mut kvm_vcpu, reg_num: c_ulong, val: c_ulong) -> c_int;
    pub fn kvm_riscv_vcpu_aia_rmw_topei(vcpu: *mut kvm_vcpu, csr_num: c_uint, val: *mut c_ulong, new_val: c_ulong, wr_mask: c_ulong) -> c_int;
    pub fn kvm_riscv_vcpu_aia_rmw_ireg(vcpu: *mut kvm_vcpu, csr_num: c_uint, val: *mut c_ulong, new_val: c_ulong, wr_mask: c_ulong) -> c_int;
    pub fn kvm_riscv_vcpu_aia_update(vcpu: *mut kvm_vcpu) -> c_int;
    pub fn kvm_riscv_vcpu_aia_reset(vcpu: *mut kvm_vcpu);
    pub fn kvm_riscv_vcpu_aia_init(vcpu: *mut kvm_vcpu);
    pub fn kvm_riscv_vcpu_aia_deinit(vcpu: *mut kvm_vcpu);
    pub fn kvm_riscv_aia_inject_msi_by_id(kvm: *mut kvm, hart_index: u32, guest_index: u32, iid: u32) -> c_int;
    pub fn kvm_riscv_aia_inject_msi(kvm: *mut kvm, msi: *mut kvm_msi) -> c_int;
    pub fn kvm_riscv_aia_inject_irq(kvm: *mut kvm, irq: c_uint, level: bool) -> c_int;
    pub fn kvm_riscv_aia_init_vm(kvm: *mut kvm);
    pub fn kvm_riscv_aia_destroy_vm(kvm: *mut kvm);
    pub fn kvm_riscv_aia_alloc_hgei(cpu: c_int, owner: *mut kvm_vcpu, hgei_va: *mut *mut core::ffi::c_void, hgei_pa: *mut phys_addr_t) -> c_int;
    pub fn kvm_riscv_aia_free_hgei(cpu: c_int, hgei: c_int);
    pub fn kvm_riscv_aia_pm_exit();
    pub fn kvm_riscv_aia_pm_enter();
    pub fn kvm_riscv_aia_enable();
    pub fn kvm_riscv_aia_disable();
    pub fn kvm_riscv_aia_init() -> c_int;
    pub fn kvm_riscv_aia_exit();
}

pub const KVM_RISCV_AIA_IMSIC_TOPEI: c_ulong = ISELECT_MASK + 1;

// KVM_RISCV_VCPU_AIA_CSR_FUNCS expands to CSR_SIREG/CSR_STOPEI descriptors;
// the descriptor type and constants are supplied by the surrounding translation.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
