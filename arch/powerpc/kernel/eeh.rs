// SPDX-License-Identifier: GPL-2.0-or-later
/* Direct Rust translation of powerpc/kernel/eeh.c. External kernel symbols are
 * intentionally left as dependencies supplied by the surrounding tree. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::{ffi::{c_char, c_int, c_uint, c_ulong, c_void}, ptr};

const EEH_MAX_FAILS: i32 = 2100000;
const PCI_BUS_RESET_WAIT_MSEC: i32 = 5 * 60 * 1000;
const EEH_PCI_REGS_LOG_LEN: usize = 8192;

#[repr(C)] pub struct eeh_dev { pub pe: *mut eeh_pe, pub pdev: *mut pci_dev, pub bdfn: u32, pub mode: i32, pub pcix_cap: i32, pub pcie_cap: i32, pub in_error: bool, pub config_space: [u32; 16] }
#[repr(C)] pub struct eeh_pe { pub phb: *mut pci_controller, pub parent: *mut eeh_pe, pub state: i32, pub type_: i32, pub addr: u32, pub check_count: i32, pub false_positives: u64, pub pass_dev_cnt: i32 }
#[repr(C)] pub struct pci_dev { pub vendor: u16, pub device: u16, pub subsystem_vendor: u16, pub subsystem_device: u16, pub error_state: i32, pub is_virtfn: bool, pub physfn: *mut pci_dev, pub resource: [resource; 17], pub dev: device }
#[repr(C)] pub struct resource { pub start: u64, pub flags: u64 }
#[repr(C)] pub struct device { pub archdata: archdata }
#[repr(C)] pub struct archdata { pub edev: *mut eeh_dev }
#[repr(C)] pub struct pci_controller { pub global_number: u32 }
#[repr(C)] pub struct device_node;
#[repr(C)] pub struct iommu_group;
#[repr(C)] pub struct notifier_block { pub notifier_call: Option<unsafe extern "C" fn(*mut notifier_block, c_ulong, *mut c_void) -> c_int> }
#[repr(C)] pub struct pci_device_id { pub vendor:u32, pub device:u32, pub subvendor:u32, pub subdevice:u32 }
#[repr(C)] pub struct eeh_ops { pub read_config: Option<unsafe extern "C" fn(*mut eeh_dev,i32,i32,*mut u32)->i32>, pub get_state: Option<unsafe extern "C" fn(*mut eeh_pe,*mut c_void)->i32>, pub set_option: Option<unsafe extern "C" fn(*mut eeh_pe,i32)->i32>, pub reset: Option<unsafe extern "C" fn(*mut eeh_pe,i32)->i32>, pub configure_bridge: Option<unsafe extern "C" fn(*mut eeh_pe)->i32>, pub get_log: Option<unsafe extern "C" fn(*mut eeh_pe,i32,*mut u8,usize)>, pub probe: Option<unsafe extern "C" fn(*mut pci_dev)->*mut eeh_dev>, pub restore_config: Option<unsafe extern "C" fn(*mut eeh_dev)> }
#[repr(C)] struct eeh_stats { no_device:u64,no_dn:u64,no_cfg_addr:u64,ignored_check:u64,total_mmio_ffs:u64,false_positives:u64,slot_resets:u64 }

extern "C" {
    static mut eeh_subsystem_flags:i32; static mut eeh_max_freezes:u32; static mut eeh_debugfs_no_recover:bool; static mut eeh_ops:*mut eeh_ops; static mut confirm_error_lock:c_void; static mut hose_list:c_void;
    fn eeh_has_flag(i32)->bool; fn eeh_add_flag(i32); fn eeh_clear_flag(i32); fn eeh_enabled()->bool; fn eeh_state_active(i32)->bool; fn eeh_dev_to_pci_dev(*mut eeh_dev)->*mut pci_dev; fn eeh_dev_to_pe(*mut eeh_dev)->*mut eeh_pe; fn pci_dev_to_eeh_dev(*mut pci_dev)->*mut eeh_dev;
    fn eeh_phb_pe_get(*mut pci_controller)->*mut eeh_pe; fn eeh_serialize_lock(*mut c_ulong); fn eeh_serialize_unlock(c_ulong); fn eeh_pe_mark_isolated(*mut eeh_pe); fn eeh_send_failure_event(*mut eeh_pe); fn __eeh_send_failure_event(*mut eeh_pe); fn eeh_wait_state(*mut eeh_pe,i32)->i32; fn eeh_pe_state_mark(*mut eeh_pe,i32); fn eeh_pe_state_clear(*mut eeh_pe,i32,bool); fn eeh_pe_passed(*mut eeh_pe)->bool; fn eeh_unfreeze_pe(*mut eeh_pe)->i32; fn eeh_pe_reset(*mut eeh_pe,i32,bool)->i32; fn eeh_pe_restore_bars(*mut eeh_pe); fn eeh_pe_reset_and_recover(*mut eeh_pe)->i32; fn eeh_pe_set_option(*mut eeh_pe,i32)->i32;
    fn ppc_find_vmap_phys(c_ulong)->c_ulong; fn eeh_addr_cache_get_dev(c_ulong)->*mut eeh_dev; fn eeh_addr_cache_init(); fn eeh_addr_cache_insert_dev(*mut pci_dev); fn eeh_addr_cache_rmv_dev(*mut pci_dev); fn eeh_event_init()->i32; fn eeh_phb_pe_create(*mut pci_controller); fn eeh_pe_tree_remove(*mut eeh_dev); fn eeh_sysfs_remove_device(*mut pci_dev); fn eeh_sysfs_add_device(*mut pci_dev); fn pci_reenable_device(*mut pci_dev)->i32; fn pci_set_power_state(*mut pci_dev,i32); fn pci_save_state(*mut pci_dev); fn pci_restore_state(*mut pci_dev); fn pci_write_config_word(*mut pci_dev,i32,u16); fn pci_read_config_word(*mut pci_dev,i32,*mut u16); fn pci_name(*mut pci_dev)->*const c_char;
}

static mut PCI_REGS_BUF:[u8;EEH_PCI_REGS_LOG_LEN]=[0;EEH_PCI_REGS_LOG_LEN]; static mut EEH_STATS:eeh_stats=eeh_stats{no_device:0,no_dn:0,no_cfg_addr:0,ignored_check:0,total_mmio_ffs:0,false_positives:0,slot_resets:0};

pub unsafe fn eeh_token_to_phys(token:c_ulong)->c_ulong { ppc_find_vmap_phys(token) }

pub unsafe extern "C" fn eeh_dev_check_failure(edev:*mut eeh_dev)->i32 {
    EEH_STATS.total_mmio_ffs+=1; if !eeh_enabled(){return 0} if edev.is_null(){EEH_STATS.no_dn+=1;return 0}
    let dev=eeh_dev_to_pci_dev(edev); let mut pe=eeh_dev_to_pe(edev); if pe.is_null(){EEH_STATS.ignored_check+=1;return 0}
    let mut flags=0; let mut ret=eeh_phb_check_failure(pe); if ret>0{return ret} if eeh_pe_passed(pe){return 0}
    eeh_serialize_lock(&mut flags); if (*pe).state & EEH_PE_ISOLATED != 0 {(*pe).check_count+=1; eeh_serialize_unlock(flags); return 1}
    ret=(*eeh_ops).get_state.unwrap()(pe,ptr::null_mut()); if ret<0 || (ret==EEH_STATE_NOT_SUPPORT && (*dev).error_state==PCI_CHANNEL_IO_PERM_FAILURE) || eeh_state_active(ret){EEH_STATS.false_positives+=1;(*pe).false_positives+=1;eeh_serialize_unlock(flags);return 0}
    let mut parent=(*pe).parent; while !parent.is_null(){if (*parent).type_ & EEH_PE_PHB!=0{break} ret=(*eeh_ops).get_state.unwrap()(parent,ptr::null_mut()); if ret>0&&!eeh_state_active(ret){pe=parent} parent=(*parent).parent}
    EEH_STATS.slot_resets+=1; eeh_pe_mark_isolated(pe); eeh_serialize_unlock(flags); eeh_send_failure_event(pe); 1
}

pub unsafe extern "C" fn eeh_check_failure(token:*const c_void)->i32 { let edev=eeh_addr_cache_get_dev(eeh_token_to_phys(token as c_ulong)); if edev.is_null(){EEH_STATS.no_device+=1;0}else{eeh_dev_check_failure(edev)} }

pub unsafe extern "C" fn eeh_pci_enable(pe:*mut eeh_pe,function:i32)->i32 { let active=match function{EEH_OPT_THAW_MMIO=>EEH_STATE_MMIO_ACTIVE|EEH_STATE_MMIO_ENABLED,EEH_OPT_THAW_DMA=>EEH_STATE_DMA_ACTIVE,EEH_OPT_DISABLE|EEH_OPT_ENABLE|EEH_OPT_FREEZE_PE=>0,_=>return -22}; if active!=0{let rc=(*eeh_ops).get_state.unwrap()(pe,ptr::null_mut());if rc<0{return rc}if rc==EEH_STATE_NOT_SUPPORT||rc&active!=0{return 0}} let rc=(*eeh_ops).set_option.unwrap()(pe,function);if rc!=0{return rc}if active!=0{let rc=eeh_wait_state(pe,PCI_BUS_RESET_WAIT_MSEC);if rc<0{return rc};if rc&active!=0{0}else{-5}}else{rc} }

pub unsafe extern "C" fn eeh_unfreeze_pe_rs(pe:*mut eeh_pe)->i32 { let r=eeh_pci_enable(pe,EEH_OPT_THAW_MMIO);if r!=0{return r}eeh_pci_enable(pe,EEH_OPT_THAW_DMA) }

pub unsafe extern "C" fn eeh_pe_reset_full_rs(pe:*mut eeh_pe,include_passed:bool)->i32 { let reset=EEH_PE_RESET|EEH_PE_CFG_BLOCKED;eeh_pe_state_mark(pe,reset);let mut ret=-5;for _ in 0..3{ret=eeh_pe_reset(pe,EEH_RESET_HOT,include_passed);if ret==0{ret=eeh_pe_reset(pe,EEH_RESET_DEACTIVATE,include_passed)}if ret==0{let s=eeh_wait_state(pe,PCI_BUS_RESET_WAIT_MSEC);if s>=0&&eeh_state_active(s){break}}}eeh_pe_state_clear(pe,reset,true);ret }

pub unsafe extern "C" fn eeh_pe_set_option_rs(pe:*mut eeh_pe,option:i32)->i32 {if pe.is_null(){return -19}match option{EEH_OPT_ENABLE=>if eeh_enabled(){eeh_pe_change_owner(pe)}else{-5},EEH_OPT_DISABLE=>0,EEH_OPT_THAW_MMIO|EEH_OPT_THAW_DMA|EEH_OPT_FREEZE_PE=>eeh_pci_enable(pe,option),_=>-22}}
pub unsafe extern "C" fn eeh_pe_get_state(pe:*mut eeh_pe)->i32 {if pe.is_null(){return -19}let r=(*eeh_ops).get_state.unwrap()(pe,ptr::null_mut());if r&EEH_STATE_RESET_ACTIVE!=0{EEH_PE_STATE_RESET}else if r&EEH_STATE_DMA_ENABLED!=0&&r&EEH_STATE_MMIO_ENABLED!=0{EEH_PE_STATE_NORMAL}else if r&EEH_STATE_DMA_ENABLED==0&&r&EEH_STATE_MMIO_ENABLED==0{EEH_PE_STATE_STOPPED_IO_DMA}else if r&EEH_STATE_DMA_ENABLED==0{EEH_PE_STATE_STOPPED_DMA}else{EEH_PE_STATE_UNAVAIL}}

unsafe fn eeh_phb_check_failure(pe:*mut eeh_pe)->i32 { if !eeh_has_flag(EEH_PROBE_MODE_DEV){return -1}let p=eeh_phb_pe_get((*pe).phb);if p.is_null(){return -17}let mut f=0;eeh_serialize_lock(&mut f);if (*p).state&EEH_PE_ISOLATED!=0{eeh_serialize_unlock(f);return 0}let r=(*eeh_ops).get_state.unwrap()(p,ptr::null_mut());if r<0||r==EEH_STATE_NOT_SUPPORT||eeh_state_active(r){eeh_serialize_unlock(f);return 0}eeh_pe_mark_isolated(p);eeh_serialize_unlock(f);eeh_send_failure_event(p);1 }
unsafe fn eeh_pe_change_owner(pe:*mut eeh_pe)->i32 {let r=(*eeh_ops).get_state.unwrap()(pe,ptr::null_mut());if r<0||r==EEH_STATE_NOT_SUPPORT||eeh_state_active(r){return 0}let r=eeh_unfreeze_pe_rs(pe);if r==0{eeh_pe_state_clear(pe,EEH_PE_ISOLATED,true)}r}

// Constants and declarations below are supplied by the kernel headers.
extern "C" { static EEH_FORCE_DISABLED:i32; static EEH_ENABLED:i32; static EEH_PROBE_MODE_DEV:i32; static EEH_PE_PHB:i32; static EEH_PE_ISOLATED:i32; static EEH_PE_RESET:i32; static EEH_PE_CFG_BLOCKED:i32; static EEH_OPT_THAW_MMIO:i32; static EEH_OPT_THAW_DMA:i32; static EEH_OPT_DISABLE:i32; static EEH_OPT_ENABLE:i32; static EEH_OPT_FREEZE_PE:i32; static EEH_RESET_HOT:i32; static EEH_RESET_DEACTIVATE:i32; static EEH_STATE_NOT_SUPPORT:i32; static EEH_STATE_MMIO_ACTIVE:i32; static EEH_STATE_MMIO_ENABLED:i32; static EEH_STATE_DMA_ACTIVE:i32; static EEH_STATE_RESET_ACTIVE:i32; static EEH_STATE_DMA_ENABLED:i32; static EEH_STATE_MMIO_ENABLED:i32; static PCI_CHANNEL_IO_PERM_FAILURE:i32; static EEH_PE_STATE_RESET:i32; static EEH_PE_STATE_NORMAL:i32; static EEH_PE_STATE_STOPPED_IO_DMA:i32; static EEH_PE_STATE_STOPPED_DMA:i32; static EEH_PE_STATE_UNAVAIL:i32; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
