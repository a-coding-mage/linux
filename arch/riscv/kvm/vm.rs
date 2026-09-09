// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2019 Western Digital Corporation or its affiliates.
 *
 * Authors:
 *     Anup Patel <anup.patel@wdc.com>
 */

// Linux kernel dependencies and build-time configuration are supplied by the
// surrounding translation unit.

#[repr(C)]
pub struct KvmStatsDesc;
#[repr(C)]
pub struct KvmStatsHeader {
    pub name_size: u32,
    pub num_desc: usize,
    pub id_offset: usize,
    pub desc_offset: usize,
    pub data_offset: usize,
}

extern "C" {
    pub static kvm_vm_stats_desc: [KvmStatsDesc; 0];
    pub static kvm_vm_stats_header: KvmStatsHeader;

    fn kvm_riscv_mmu_alloc_pgd(kvm: *mut Kvm) -> i32;
    fn kvm_riscv_gstage_vmid_init(kvm: *mut Kvm) -> i32;
    fn kvm_riscv_mmu_free_pgd(kvm: *mut Kvm);
    fn kvm_riscv_aia_init_vm(kvm: *mut Kvm);
    fn kvm_riscv_guest_timer_init(kvm: *mut Kvm);
    fn kvm_destroy_vcpus(kvm: *mut Kvm);
    fn kvm_riscv_aia_destroy_vm(kvm: *mut Kvm);
    fn irqchip_in_kernel(kvm: *mut Kvm) -> bool;
    fn kvm_riscv_aia_inject_irq(kvm: *mut Kvm, irq: u32, level: i32) -> i32;
    fn kvm_riscv_aia_inject_msi(kvm: *mut Kvm, msi: *mut KvmMsi) -> i32;
    fn kvm_set_irq_routing(kvm: *mut Kvm, ents: *mut KvmIrqRoutingEntry, lines: u32, flags: u32) -> i32;
    fn kvm_riscv_aia_available() -> i32;
    fn num_online_cpus() -> u32;
    fn kvm_riscv_gstage_gpa_bits(levels: u32) -> i32;
    fn mutex_lock(lock: *mut core::ffi::c_void);
    fn mutex_unlock(lock: *mut core::ffi::c_void);
    fn kvm_are_all_memslots_empty(kvm: *mut Kvm) -> bool;
}

#[repr(C)]
pub struct Kvm { pub arch: KvmArch, pub created_vcpus: u32, pub lock: core::ffi::c_void, pub slots_lock: core::ffi::c_void }
#[repr(C)]
pub struct KvmArch { pub pgd_levels: u32, pub mp_state_reset: bool }
#[repr(C)]
pub struct File;
#[repr(C)]
pub struct KvmEnableCap { pub flags: u32, pub cap: u32, pub args: [u64; 4] }
#[repr(C)]
pub struct KvmIrqLevel { pub irq: u32, pub level: i32 }
#[repr(C)]
pub struct KvmMsi { pub address_lo: u32, pub address_hi: u32, pub data: u32, pub flags: u32, pub devid: u32 }
#[repr(C)]
pub struct KvmKernelIrqRoutingEntry {
    pub set: Option<unsafe extern "C" fn(*mut KvmKernelIrqRoutingEntry, *mut Kvm, i32, i32, bool) -> i32>,
    pub type_: u32,
    pub irqchip: KvmIrqchip,
    pub msi: KvmMsi,
}
#[repr(C)]
pub struct KvmIrqchip { pub irqchip: u32, pub pin: u32 }
#[repr(C)]
pub struct KvmIrqRoutingEntry {
    pub gsi: u32,
    pub type_: u32,
    pub irqchip: KvmIrqchip,
    pub msi: KvmMsi,
    pub flags: u32,
}

pub unsafe fn kvm_arch_init_vm(kvm: *mut Kvm, _type: usize) -> i32 {
    let mut r = kvm_riscv_mmu_alloc_pgd(kvm);
    if r != 0 { return r; }
    r = kvm_riscv_gstage_vmid_init(kvm);
    if r != 0 { kvm_riscv_mmu_free_pgd(kvm); return r; }
    kvm_riscv_aia_init_vm(kvm);
    kvm_riscv_guest_timer_init(kvm);
    0
}

pub unsafe fn kvm_arch_destroy_vm(kvm: *mut Kvm) {
    kvm_destroy_vcpus(kvm);
    kvm_riscv_aia_destroy_vm(kvm);
}

pub unsafe fn kvm_vm_ioctl_irq_line(kvm: *mut Kvm, irql: *mut KvmIrqLevel, _line_status: bool) -> i32 {
    if !irqchip_in_kernel(kvm) { return -6; }
    kvm_riscv_aia_inject_irq(kvm, (*irql).irq, (*irql).level)
}

pub unsafe extern "C" fn kvm_set_msi(e: *mut KvmKernelIrqRoutingEntry, kvm: *mut Kvm, _irq_source_id: i32, level: i32, _line_status: bool) -> i32 {
    if level == 0 { return -1; }
    let mut msi = (*e).msi;
    kvm_riscv_aia_inject_msi(kvm, &mut msi)
}

unsafe extern "C" fn kvm_riscv_set_irq(e: *mut KvmKernelIrqRoutingEntry, kvm: *mut Kvm, _irq_source_id: i32, level: i32, _line_status: bool) -> i32 {
    kvm_riscv_aia_inject_irq(kvm, (*e).irqchip.pin, level)
}

pub unsafe fn kvm_riscv_setup_default_irq_routing(kvm: *mut Kvm, lines: u32) -> i32 {
    let mut ents = vec![KvmIrqRoutingEntry { gsi: 0, type_: 0, irqchip: KvmIrqchip { irqchip: 0, pin: 0 }, msi: KvmMsi { address_lo: 0, address_hi: 0, data: 0, flags: 0, devid: 0 }, flags: 0 }; lines as usize];
    for i in 0..lines as usize {
        ents[i].gsi = i as u32;
        ents[i].type_ = KVM_IRQ_ROUTING_IRQCHIP;
        ents[i].irqchip.irqchip = 0;
        ents[i].irqchip.pin = i as u32;
    }
    kvm_set_irq_routing(kvm, ents.as_mut_ptr(), lines, 0)
}

pub unsafe fn kvm_arch_can_set_irq_routing(kvm: *mut Kvm) -> bool { irqchip_in_kernel(kvm) }

pub unsafe fn kvm_set_routing_entry(kvm: *mut Kvm, e: *mut KvmKernelIrqRoutingEntry, ue: *const KvmIrqRoutingEntry) -> i32 {
    let mut r = -22;
    match (*ue).type_ {
        KVM_IRQ_ROUTING_IRQCHIP => {
            (*e).set = Some(kvm_riscv_set_irq);
            (*e).irqchip = (*ue).irqchip;
            if (*e).irqchip.pin >= KVM_IRQCHIP_NUM_PINS || (*e).irqchip.irqchip >= KVM_NR_IRQCHIPS { return r; }
        }
        KVM_IRQ_ROUTING_MSI => { (*e).set = Some(kvm_set_msi); (*e).msi = (*ue).msi; (*e).msi.flags = (*ue).flags; }
        _ => return r,
    }
    r = 0; r
}

pub unsafe fn kvm_arch_set_irq_inatomic(e: *mut KvmKernelIrqRoutingEntry, kvm: *mut Kvm, irq_source_id: i32, level: i32, line_status: bool) -> i32 {
    if level == 0 { return -11; }
    match (*e).type_ {
        KVM_IRQ_ROUTING_MSI => kvm_set_msi(e, kvm, irq_source_id, level, line_status),
        KVM_IRQ_ROUTING_IRQCHIP => kvm_riscv_set_irq(e, kvm, irq_source_id, level, line_status),
        _ => -11,
    }
}

pub unsafe fn kvm_arch_irqchip_in_kernel(kvm: *mut Kvm) -> bool { irqchip_in_kernel(kvm) }

pub unsafe fn kvm_vm_ioctl_check_extension(kvm: *mut Kvm, ext: i64) -> i32 {
    match ext {
        KVM_CAP_IRQCHIP => kvm_riscv_aia_available(),
        KVM_CAP_IOEVENTFD | KVM_CAP_USER_MEMORY | KVM_CAP_DESTROY_MEMORY_REGION_WORKS | KVM_CAP_ONE_REG | KVM_CAP_READONLY_MEM | KVM_CAP_MP_STATE | KVM_CAP_IMMEDIATE_EXIT | KVM_CAP_SET_GUEST_DEBUG => 1,
        KVM_CAP_NR_VCPUS => core::cmp::min(num_online_cpus(), KVM_MAX_VCPUS) as i32,
        KVM_CAP_MAX_VCPUS => KVM_MAX_VCPUS as i32,
        KVM_CAP_NR_MEMSLOTS => KVM_USER_MEM_SLOTS as i32,
        KVM_CAP_VM_GPA_BITS => if kvm.is_null() { kvm_riscv_gstage_gpa_bits(kvm_riscv_gstage_max_pgd_levels) } else { kvm_riscv_gstage_gpa_bits((*kvm).arch.pgd_levels) },
        _ => 0,
    }
}

pub unsafe fn kvm_vm_ioctl_enable_cap(kvm: *mut Kvm, cap: *mut KvmEnableCap) -> i32 {
    if (*cap).flags != 0 { return -22; }
    match (*cap).cap {
        KVM_CAP_RISCV_MP_STATE_RESET => { (*kvm).arch.mp_state_reset = true; 0 }
        KVM_CAP_VM_GPA_BITS => {
            let gpa_bits = (*cap).args[0] as usize;
            let new_levels = if cfg!(target_pointer_width = "64") {
                if gpa_bits <= 41 { 3 } else if gpa_bits <= 50 { 4 } else if gpa_bits <= 59 { 5 } else { return -22; }
            } else if gpa_bits <= 34 { 2 } else { return -22; };
            if new_levels > kvm_riscv_gstage_max_pgd_levels { return -22; }
            mutex_lock(&mut (*kvm).lock as *mut _ as *mut core::ffi::c_void);
            mutex_lock(&mut (*kvm).slots_lock as *mut _ as *mut core::ffi::c_void);
            let r = if (*kvm).created_vcpus != 0 || !kvm_are_all_memslots_empty(kvm) { -16 } else { (*kvm).arch.pgd_levels = new_levels; 0 };
            mutex_unlock(&mut (*kvm).slots_lock as *mut _ as *mut core::ffi::c_void);
            mutex_unlock(&mut (*kvm).lock as *mut _ as *mut core::ffi::c_void);
            r
        }
        _ => -22,
    }
}

pub unsafe fn kvm_arch_vm_ioctl(_filp: *mut File, _ioctl: u32, _arg: usize) -> i32 { -22 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
