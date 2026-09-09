/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright 2012 Michael Ellerman, IBM Corporation.
 * Copyright 2012 Benjamin Herrenschmidt, IBM Corporation
 */

/* CONFIG_KVM_XICS */

/*
 * We use a two-level tree to store interrupt source information.
 * There are up to 1024 ICS nodes, each of which can represent
 * 1024 sources.
 */
pub const KVMPPC_XICS_MAX_ICS_ID: u32 = 1023;
pub const KVMPPC_XICS_ICS_SHIFT: u32 = 10;
pub const KVMPPC_XICS_IRQ_PER_ICS: usize = 1usize << KVMPPC_XICS_ICS_SHIFT;
pub const KVMPPC_XICS_SRC_MASK: u32 = KVMPPC_XICS_IRQ_PER_ICS as u32 - 1;

/* Interrupt source numbers below this are reserved. */
pub const KVMPPC_XICS_FIRST_IRQ: u32 = 16;
pub const KVMPPC_XICS_NR_IRQS: u32 = (KVMPPC_XICS_MAX_ICS_ID + 1)
    * KVMPPC_XICS_IRQ_PER_ICS as u32;

/* Priority value to use for disabling an interrupt */
pub const MASKED: u8 = 0xff;
pub const PQ_PRESENTED: u8 = 1;
pub const PQ_QUEUED: u8 = 2;

#[repr(C)]
pub struct ics_irq_state {
    pub number: u32,
    pub server: u32,
    pub pq_state: u32,
    pub priority: u8,
    pub saved_priority: u8,
    pub resend: u8,
    pub masked_pending: u8,
    pub lsi: u8, /* level-sensitive interrupt */
    pub exists: u8,
    pub intr_cpu: i32,
    pub host_irq: u32,
}

#[repr(C)]
pub struct kvmppc_icp_state_fields {
    pub out_ee: u8,
    pub need_resend: u8,
    pub cppr: u8,
    pub mfrr: u8,
    pub pending_pri: u8,
    pub xisr: u32,
}

#[repr(C)]
pub union kvmppc_icp_state {
    pub raw: usize,
    pub fields: kvmppc_icp_state_fields,
}

pub const ICP_RESEND_MAP_SIZE: usize =
    (KVMPPC_XICS_MAX_ICS_ID as usize / (usize::BITS as usize) + 1);

#[repr(C)]
pub struct kvmppc_icp {
    pub vcpu: *mut kvm_vcpu,
    pub server_num: usize,
    pub state: kvmppc_icp_state,
    pub resend_map: [usize; ICP_RESEND_MAP_SIZE],
    /* Real mode might find something too hard, here's the action
     * it might request from virtual mode
     */
    pub rm_action: u32,
    pub rm_kick_target: *mut kvm_vcpu,
    pub rm_resend_icp: *mut kvmppc_icp,
    pub rm_reject: u32,
    pub rm_eoied_irq: u32,
    /* Counters for each reason we exited real mode */
    pub n_rm_kick_vcpu: usize,
    pub n_rm_check_resend: usize,
    pub n_rm_notify_eoi: usize,
    /* Counters for handling ICP processing in real mode */
    pub n_check_resend: usize,
    pub n_reject: usize,
    /* Debug stuff for real mode */
    pub rm_dbgstate: kvmppc_icp_state,
    pub rm_dbgtgt: *mut kvm_vcpu,
}

pub const XICS_RM_KICK_VCPU: u32 = 0x1;
pub const XICS_RM_CHECK_RESEND: u32 = 0x2;
pub const XICS_RM_NOTIFY_EOI: u32 = 0x8;

#[repr(C)]
pub struct kvmppc_ics {
    pub lock: arch_spinlock_t,
    pub icsid: u16,
    pub irq_state: [ics_irq_state; KVMPPC_XICS_IRQ_PER_ICS],
}

#[repr(C)]
pub struct kvmppc_xics {
    pub kvm: *mut kvm,
    pub dev: *mut kvm_device,
    pub dentry: *mut dentry,
    pub max_icsid: u32,
    pub real_mode: bool,
    pub real_mode_dbg: bool,
    pub err_noics: u32,
    pub err_noicp: u32,
    pub ics: [*mut kvmppc_ics; (KVMPPC_XICS_MAX_ICS_ID + 1) as usize],
}

pub unsafe fn kvmppc_xics_find_server(kvm: *mut kvm, nr: u32) -> *mut kvmppc_icp {
    let mut vcpu: *mut kvm_vcpu = core::ptr::null_mut();
    let mut i: usize = 0;
    kvm_for_each_vcpu!(i, vcpu, kvm, {
        if !(*vcpu).arch.icp.is_null() && nr as usize == (*(*vcpu).arch.icp).server_num {
            return (*vcpu).arch.icp;
        }
    });
    core::ptr::null_mut()
}

pub unsafe fn kvmppc_xics_find_ics(
    xics: *mut kvmppc_xics,
    irq: u32,
    source: *mut u16,
) -> *mut kvmppc_ics {
    let icsid = irq >> KVMPPC_XICS_ICS_SHIFT;
    let src = (irq & KVMPPC_XICS_SRC_MASK) as u16;
    if !source.is_null() {
        *source = src;
    }
    if icsid > KVMPPC_XICS_MAX_ICS_ID {
        return core::ptr::null_mut();
    }
    let ics = (*xics).ics[icsid as usize];
    if ics.is_null() {
        return core::ptr::null_mut();
    }
    ics
}

extern "C" {
    pub fn xics_rm_h_xirr(vcpu: *mut kvm_vcpu) -> usize;
    pub fn xics_rm_h_xirr_x(vcpu: *mut kvm_vcpu) -> usize;
    pub fn xics_rm_h_ipi(vcpu: *mut kvm_vcpu, server: usize, mfrr: usize) -> i32;
    pub fn xics_rm_h_cppr(vcpu: *mut kvm_vcpu, cppr: usize) -> i32;
    pub fn xics_rm_h_eoi(vcpu: *mut kvm_vcpu, xirr: usize) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
