/* SPDX-License-Identifier: MIT */

// Translated from xen/interface/hvm/hvm_op.h.
// The original include and header guard are C-only declarations.

// Get/set subcommands: the second argument of the hypercall is a
// pointer to a xen_hvm_param struct.
pub const HVMOP_set_param: u32 = 0;
pub const HVMOP_get_param: u32 = 1;

#[repr(C)]
pub struct xen_hvm_param {
    pub domid: domid_t,    /* IN */
    pub index: u32,        /* IN */
    pub value: u64,        /* IN/OUT */
}

// DEFINE_GUEST_HANDLE_STRUCT(xen_hvm_param)
pub type xen_hvm_param_guest_handle = *mut xen_hvm_param;

// Hint from PV drivers for pagetable destruction.
pub const HVMOP_pagetable_dying: u32 = 9;

#[repr(C)]
pub struct xen_hvm_pagetable_dying {
    /* Domain with a pagetable about to be destroyed. */
    pub domid: domid_t,
    /* guest physical address of the toplevel pagetable dying */
    pub gpa: aligned_u64,
}

pub type xen_hvm_pagetable_dying_t = xen_hvm_pagetable_dying;
// DEFINE_GUEST_HANDLE_STRUCT(xen_hvm_pagetable_dying_t)
pub type xen_hvm_pagetable_dying_t_guest_handle = *mut xen_hvm_pagetable_dying_t;

#[repr(C)]
pub enum hvmmem_type_t {
    HVMMEM_ram_rw,             /* Normal read/write guest RAM */
    HVMMEM_ram_ro,             /* Read-only; writes are discarded */
    HVMMEM_mmio_dm,            /* Reads and write go to the device model */
}

pub const HVMOP_get_mem_type: u32 = 15;
/* Return hvmmem_type_t for the specified pfn. */
#[repr(C)]
pub struct xen_hvm_get_mem_type {
    /* Domain to be queried. */
    pub domid: domid_t,
    /* OUT variable. */
    pub mem_type: u16,
    pub pad: [u16; 2], /* align next field on 8-byte boundary */
    /* IN variable. */
    pub pfn: u64,
}

// DEFINE_GUEST_HANDLE_STRUCT(xen_hvm_get_mem_type)
pub type xen_hvm_get_mem_type_guest_handle = *mut xen_hvm_get_mem_type;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
/*
 * HVMOP_set_evtchn_upcall_vector: Set a <vector> that should be used for event
 *                                 channel upcalls on the specified <vcpu>. If set,
 *                                 this vector will be used in preference to the
 *                                 domain global callback via (see
 *                                 HVM_PARAM_CALLBACK_IRQ).
 */
pub const HVMOP_set_evtchn_upcall_vector: u32 = 23;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[repr(C)]
pub struct xen_hvm_evtchn_upcall_vector {
    pub vcpu: u32,
    pub vector: u8,
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub type xen_hvm_evtchn_upcall_vector_t = xen_hvm_evtchn_upcall_vector;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
// DEFINE_GUEST_HANDLE_STRUCT(xen_hvm_evtchn_upcall_vector_t)
pub type xen_hvm_evtchn_upcall_vector_t_guest_handle = *mut xen_hvm_evtchn_upcall_vector_t;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
