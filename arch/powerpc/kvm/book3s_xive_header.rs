/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright 2017 Benjamin Herrenschmidt, IBM Corporation */

// CONFIG_KVM_XICS guarded declarations; dependencies are supplied externally.

pub const KVMPPC_XIVE_FIRST_IRQ: u32 = 0;
pub const KVMPPC_XIVE_NR_IRQS: u32 = KVMPPC_XICS_NR_IRQS;

#[repr(C)]
pub struct kvmppc_xive_irq_state {
    pub valid: bool,
    pub number: u32,
    pub ipi_number: u32,
    pub ipi_data: xive_irq_data,
    pub pt_number: u32,
    pub pt_data: *mut xive_irq_data,
    pub guest_priority: u8,
    pub saved_priority: u8,
    pub act_server: u32,
    pub act_priority: u8,
    pub in_eoi: bool,
    pub old_p: bool,
    pub old_q: bool,
    pub lsi: bool,
    pub asserted: bool,
    pub in_queue: bool,
    pub saved_p: bool,
    pub saved_q: bool,
    pub saved_scan_prio: u8,
    pub eisn: u32,
}

#[inline]
pub unsafe fn kvmppc_xive_select_irq(
    state: *mut kvmppc_xive_irq_state,
    out_hw_irq: *mut u32,
    out_xd: *mut *mut xive_irq_data,
) {
    if (*state).pt_number != 0 {
        if !out_hw_irq.is_null() { *out_hw_irq = (*state).pt_number; }
        if !out_xd.is_null() { *out_xd = (*state).pt_data; }
    } else {
        if !out_hw_irq.is_null() { *out_hw_irq = (*state).ipi_number; }
        if !out_xd.is_null() { *out_xd = &mut (*state).ipi_data; }
    }
}

#[repr(C)]
pub struct kvmppc_xive_src_block {
    pub lock: arch_spinlock_t,
    pub id: u16,
    pub irq_state: [kvmppc_xive_irq_state; KVMPPC_XICS_IRQ_PER_ICS as usize],
}

#[repr(C)]
pub struct kvmppc_xive_ops {
    pub reset_mapped: Option<unsafe extern "C" fn(*mut kvm, c_ulong) -> c_int>,
}

pub const KVMPPC_XIVE_FLAG_SINGLE_ESCALATION: u32 = 0x1;
pub const KVMPPC_XIVE_FLAG_SAVE_RESTORE: u32 = 0x2;

#[repr(C)]
pub struct kvmppc_xive {
    pub kvm: *mut kvm,
    pub dev: *mut kvm_device,
    pub dentry: *mut dentry,
    pub vp_base: u32,
    pub src_blocks: [*mut kvmppc_xive_src_block; (KVMPPC_XICS_MAX_ICS_ID + 1) as usize],
    pub max_sbid: u32,
    pub src_count: u32,
    pub saved_src_count: u32,
    pub delayed_irqs: u32,
    pub qmap: u8,
    pub q_order: u32,
    pub q_page_order: u32,
    pub flags: u8,
    pub nr_servers: u32,
    pub ops: *mut kvmppc_xive_ops,
    pub mapping: *mut address_space,
    pub mapping_lock: mutex,
    pub lock: mutex,
}

pub const KVMPPC_XIVE_Q_COUNT: usize = 8;

#[repr(C)]
pub struct kvmppc_xive_vcpu {
    pub xive: *mut kvmppc_xive,
    pub vcpu: *mut kvm_vcpu,
    pub valid: bool,
    pub server_num: u32,
    pub vp_id: u32,
    pub vp_chip_id: u32,
    pub vp_cam: u32,
    pub vp_ipi: u32,
    pub vp_ipi_data: xive_irq_data,
    pub cppr: u8,
    pub hw_cppr: u8,
    pub mfrr: u8,
    pub pending: u8,
    pub queues: [xive_q; KVMPPC_XIVE_Q_COUNT],
    pub esc_virq: [u32; KVMPPC_XIVE_Q_COUNT],
    pub esc_virq_names: [*mut c_char; KVMPPC_XIVE_Q_COUNT],
    pub delayed_irq: u32,
    pub stat_rm_h_xirr: u64,
    pub stat_rm_h_ipoll: u64,
    pub stat_rm_h_cppr: u64,
    pub stat_rm_h_eoi: u64,
    pub stat_rm_h_ipi: u64,
    pub stat_vm_h_xirr: u64,
    pub stat_vm_h_ipoll: u64,
    pub stat_vm_h_cppr: u64,
    pub stat_vm_h_eoi: u64,
    pub stat_vm_h_ipi: u64,
}

#[inline]
pub unsafe fn kvmppc_xive_find_server(kvm: *mut kvm, nr: u32) -> *mut kvm_vcpu {
    let mut vcpu: *mut kvm_vcpu = core::ptr::null_mut();
    let mut i: c_ulong = 0;
    kvm_for_each_vcpu!(i, vcpu, kvm, {
        if !(*vcpu).arch.xive_vcpu.is_null() && nr == (*(*vcpu).arch.xive_vcpu).server_num { return vcpu; }
    });
    core::ptr::null_mut()
}

#[inline]
pub unsafe fn kvmppc_xive_find_source(xive: *mut kvmppc_xive, irq: u32, source: *mut u16) -> *mut kvmppc_xive_src_block {
    let bid = irq >> KVMPPC_XICS_ICS_SHIFT;
    let src = (irq & KVMPPC_XICS_SRC_MASK) as u16;
    if !source.is_null() { *source = src; }
    if bid > KVMPPC_XICS_MAX_ICS_ID { return core::ptr::null_mut(); }
    (*xive).src_blocks[bid as usize]
}

#[inline]
pub unsafe fn kvmppc_xive_vp(xive: *mut kvmppc_xive, server: u32) -> u32 {
    (*xive).vp_base + kvmppc_pack_vcpu_id((*xive).kvm, server)
}

#[inline]
pub unsafe fn kvmppc_xive_vp_in_use(kvm: *mut kvm, vp_id: u32) -> bool {
    let mut vcpu: *mut kvm_vcpu = core::ptr::null_mut();
    let mut i: c_ulong = 0;
    kvm_for_each_vcpu!(i, vcpu, kvm, {
        if !(*vcpu).arch.xive_vcpu.is_null() && vp_id == (*(*vcpu).arch.xive_vcpu).vp_id { return true; }
    });
    false
}

#[inline]
pub fn xive_prio_from_guest(prio: u8) -> u8 { if prio == 0xff || prio < 6 { prio } else { 6 } }
#[inline]
pub fn xive_prio_to_guest(prio: u8) -> u8 { prio }

#[inline]
pub unsafe fn __xive_read_eq(qpage: *mut __be32, msk: u32, idx: *mut u32, toggle: *mut u32) -> u32 {
    if qpage.is_null() { return 0; }
    let cur = be32_to_cpup(qpage.add(*idx as usize));
    if (cur >> 31) == *toggle { return 0; }
    *idx = (*idx + 1) & msk;
    if *idx == 0 { *toggle ^= 1; }
    cur & 0x7fffffff
}

extern "C" {
    pub fn kvmppc_xive_disable_vcpu_interrupts(vcpu: *mut kvm_vcpu);
    pub fn kvmppc_xive_debug_show_queues(m: *mut seq_file, vcpu: *mut kvm_vcpu) -> c_int;
    pub fn kvmppc_xive_debug_show_sources(m: *mut seq_file, sb: *mut kvmppc_xive_src_block);
    pub fn kvmppc_xive_create_src_block(xive: *mut kvmppc_xive, irq: c_int) -> *mut kvmppc_xive_src_block;
    pub fn kvmppc_xive_free_sources(sb: *mut kvmppc_xive_src_block);
    pub fn kvmppc_xive_select_target(kvm: *mut kvm, server: *mut u32, prio: u8) -> c_int;
    pub fn kvmppc_xive_attach_escalation(vcpu: *mut kvm_vcpu, prio: u8, single_escalation: bool) -> c_int;
    pub fn kvmppc_xive_get_device(kvm: *mut kvm, type_: u32) -> *mut kvmppc_xive;
    pub fn xive_cleanup_single_escalation(vcpu: *mut kvm_vcpu, irq: c_int);
    pub fn kvmppc_xive_compute_vp_id(xive: *mut kvmppc_xive, cpu: u32, vp: *mut u32) -> c_int;
    pub fn kvmppc_xive_set_nr_servers(xive: *mut kvmppc_xive, addr: u64) -> c_int;
    pub fn kvmppc_xive_check_save_restore(vcpu: *mut kvm_vcpu) -> bool;
}

#[inline]
pub unsafe fn kvmppc_xive_has_single_escalation(xive: *mut kvmppc_xive) -> bool {
    ((*xive).flags as u32 & KVMPPC_XIVE_FLAG_SINGLE_ESCALATION) != 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
