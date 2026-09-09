/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Machine check exception header file.
 *
 * Copyright 2013 IBM Corporation
 * Author: Mahesh Salgaonkar <mahesh@linux.vnet.ibm.com>
 */

#[repr(i32)]
pub enum MCE_Version {
    MCE_V1 = 1,
}

#[repr(i32)]
pub enum MCE_Severity {
    MCE_SEV_NO_ERROR = 0,
    MCE_SEV_WARNING = 1,
    MCE_SEV_SEVERE = 2,
    MCE_SEV_FATAL = 3,
}

#[repr(i32)]
pub enum MCE_Disposition {
    MCE_DISPOSITION_RECOVERED = 0,
    MCE_DISPOSITION_NOT_RECOVERED = 1,
}

#[repr(i32)]
pub enum MCE_Initiator {
    MCE_INITIATOR_UNKNOWN = 0,
    MCE_INITIATOR_CPU = 1,
    MCE_INITIATOR_PCI = 2,
    MCE_INITIATOR_ISA = 3,
    MCE_INITIATOR_MEMORY = 4,
    MCE_INITIATOR_POWERMGM = 5,
}

#[repr(i32)]
pub enum MCE_ErrorType {
    MCE_ERROR_TYPE_UNKNOWN = 0,
    MCE_ERROR_TYPE_UE = 1,
    MCE_ERROR_TYPE_SLB = 2,
    MCE_ERROR_TYPE_ERAT = 3,
    MCE_ERROR_TYPE_TLB = 4,
    MCE_ERROR_TYPE_USER = 5,
    MCE_ERROR_TYPE_RA = 6,
    MCE_ERROR_TYPE_LINK = 7,
    MCE_ERROR_TYPE_DCACHE = 8,
    MCE_ERROR_TYPE_ICACHE = 9,
}

#[repr(i32)]
pub enum MCE_ErrorClass {
    MCE_ECLASS_UNKNOWN = 0,
    MCE_ECLASS_HARDWARE,
    MCE_ECLASS_HARD_INDETERMINATE,
    MCE_ECLASS_SOFTWARE,
    MCE_ECLASS_SOFT_INDETERMINATE,
}

#[repr(i32)]
pub enum MCE_UeErrorType {
    MCE_UE_ERROR_INDETERMINATE = 0,
    MCE_UE_ERROR_IFETCH = 1,
    MCE_UE_ERROR_PAGE_TABLE_WALK_IFETCH = 2,
    MCE_UE_ERROR_LOAD_STORE = 3,
    MCE_UE_ERROR_PAGE_TABLE_WALK_LOAD_STORE = 4,
}

#[repr(i32)]
pub enum MCE_SlbErrorType { MCE_SLB_ERROR_INDETERMINATE = 0, MCE_SLB_ERROR_PARITY = 1, MCE_SLB_ERROR_MULTIHIT = 2 }
#[repr(i32)]
pub enum MCE_EratErrorType { MCE_ERAT_ERROR_INDETERMINATE = 0, MCE_ERAT_ERROR_PARITY = 1, MCE_ERAT_ERROR_MULTIHIT = 2 }
#[repr(i32)]
pub enum MCE_TlbErrorType { MCE_TLB_ERROR_INDETERMINATE = 0, MCE_TLB_ERROR_PARITY = 1, MCE_TLB_ERROR_MULTIHIT = 2 }
#[repr(i32)]
pub enum MCE_UserErrorType { MCE_USER_ERROR_INDETERMINATE = 0, MCE_USER_ERROR_TLBIE = 1, MCE_USER_ERROR_SCV = 2 }

#[repr(i32)]
pub enum MCE_RaErrorType {
    MCE_RA_ERROR_INDETERMINATE = 0,
    MCE_RA_ERROR_IFETCH = 1,
    MCE_RA_ERROR_IFETCH_FOREIGN = 2,
    MCE_RA_ERROR_PAGE_TABLE_WALK_IFETCH = 3,
    MCE_RA_ERROR_PAGE_TABLE_WALK_IFETCH_FOREIGN = 4,
    MCE_RA_ERROR_LOAD = 5,
    MCE_RA_ERROR_STORE = 6,
    MCE_RA_ERROR_PAGE_TABLE_WALK_LOAD_STORE = 7,
    MCE_RA_ERROR_PAGE_TABLE_WALK_LOAD_STORE_FOREIGN = 8,
    MCE_RA_ERROR_LOAD_STORE_FOREIGN = 9,
}

#[repr(i32)]
pub enum MCE_LinkErrorType {
    MCE_LINK_ERROR_INDETERMINATE = 0,
    MCE_LINK_ERROR_IFETCH_TIMEOUT = 1,
    MCE_LINK_ERROR_PAGE_TABLE_WALK_IFETCH_TIMEOUT = 2,
    MCE_LINK_ERROR_LOAD_TIMEOUT = 3,
    MCE_LINK_ERROR_STORE_TIMEOUT = 4,
    MCE_LINK_ERROR_PAGE_TABLE_WALK_LOAD_STORE_TIMEOUT = 5,
}

#[repr(C)]
pub struct machine_check_event_ue_error {
    pub ue_error_type: u8,
    pub effective_address_provided: u8,
    pub physical_address_provided: u8,
    pub ignore_event: u8,
    pub reserved_1: [u8; 4],
    pub effective_address: u64,
    pub physical_address: u64,
    pub reserved_2: [u8; 8],
}

#[repr(C)]
pub struct machine_check_event_address_error {
    pub error_type: u8,
    pub effective_address_provided: u8,
    pub reserved_1: [u8; 6],
    pub effective_address: u64,
    pub reserved_2: [u8; 16],
}

#[repr(C)]
pub union machine_check_event_union {
    pub ue_error: machine_check_event_ue_error,
    pub slb_error: machine_check_event_address_error,
    pub erat_error: machine_check_event_address_error,
    pub tlb_error: machine_check_event_address_error,
    pub user_error: machine_check_event_address_error,
    pub ra_error: machine_check_event_address_error,
    pub link_error: machine_check_event_address_error,
}

#[repr(C)]
pub struct machine_check_event {
    pub version: u8,
    pub in_use: u8,
    pub severity: u8,
    pub initiator: u8,
    pub error_type: u8,
    pub error_class: u8,
    pub disposition: u8,
    pub sync_error: bool,
    pub cpu: u16,
    pub gpr3: u64,
    pub srr0: u64,
    pub srr1: u64,
    pub u: machine_check_event_union,
}

#[repr(C)]
pub union mce_error_info_union {
    pub ue_error_type: u8,
    pub slb_error_type: u8,
    pub erat_error_type: u8,
    pub tlb_error_type: u8,
    pub user_error_type: u8,
    pub ra_error_type: u8,
    pub link_error_type: u8,
}

#[repr(C)]
pub struct mce_error_info {
    pub error_type: u8,
    pub u: mce_error_info_union,
    pub severity: u8,
    pub initiator: u8,
    pub error_class: u8,
    pub sync_error: bool,
    pub ignore_event: bool,
}

pub const MAX_MC_EVT: usize = 10;
pub const MCE_EVENT_RELEASE: bool = true;
pub const MCE_EVENT_DONTRELEASE: bool = false;

#[repr(C)]
pub struct mce_info {
    pub mce_nest_count: i32,
    pub mce_event: [machine_check_event; MAX_MC_EVT],
    pub mce_queue_count: i32,
    pub mce_event_queue: [machine_check_event; MAX_MC_EVT],
    pub mce_ue_count: i32,
    pub mce_ue_event_queue: [machine_check_event; MAX_MC_EVT],
}

#[repr(C)]
pub struct pt_regs;
#[repr(C)]
pub struct notifier_block;

unsafe extern "C" {
    pub fn save_mce_event(regs: *mut pt_regs, handled: i64, mce_err: *mut mce_error_info,
                          nip: u64, addr: u64, phys_addr: u64);
    pub fn get_mce_event(mce: *mut machine_check_event, release: bool) -> i32;
    pub fn release_mce_event();
    pub fn machine_check_queue_event();
    pub fn machine_check_print_event_info(evt: *mut machine_check_event, user_mode: bool,
                                          in_guest: bool);
    pub fn addr_to_pfn(regs: *mut pt_regs, addr: u64) -> u64;
    pub fn mce_common_process_ue(regs: *mut pt_regs, mce_err: *mut mce_error_info);
    pub fn mce_irq_work_queue();
    pub fn mce_register_notifier(nb: *mut notifier_block) -> i32;
    pub fn mce_unregister_notifier(nb: *mut notifier_block) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
