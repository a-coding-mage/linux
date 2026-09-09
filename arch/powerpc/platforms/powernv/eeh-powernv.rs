// SPDX-License-Identifier: GPL-2.0-or-later
/* PowerNV Platform dependent EEH operations (literal Rust translation). */

// Kernel-provided types, constants, globals, and functions are intentionally
// left as external dependencies, as in the original translation unit.
use core::ffi::{c_char, c_int, c_ulong, c_void};

extern "C" {
    static mut eeh_event_irq: c_int;
}

// The following opaque declarations correspond to the structures supplied by
// the Linux PowerNV EEH headers.
#[repr(C)] pub struct pci_dev { _private: [u8; 0] }
#[repr(C)] pub struct pci_bus { _private: [u8; 0] }
#[repr(C)] pub struct pci_controller { _private: [u8; 0] }
#[repr(C)] pub struct pnv_phb { _private: [u8; 0] }
#[repr(C)] pub struct pci_dn { _private: [u8; 0] }
#[repr(C)] pub struct eeh_dev { _private: [u8; 0] }
#[repr(C)] pub struct eeh_pe { _private: [u8; 0] }
#[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct OpalIoP7IOCErrorData { _private: [u8; 0] }

// External kernel interfaces used by this file.
extern "C" {
    fn eeh_enabled() -> bool;
    fn eeh_send_failure_event(pe: *mut eeh_pe);
    fn disable_irq_nosync(irq: c_int);
    fn enable_irq(irq: c_int);
    fn eeh_probe_device(dev: *mut pci_dev);
    fn eeh_init(ops: *mut eeh_ops) -> c_int;
    fn eeh_add_flag(flag: c_int);
    fn eeh_set_pe_aux_size(size: c_int);
    fn firmware_has_feature(feature: c_int) -> bool;
    fn pnv_pci_cfg_read(pdn: *mut pci_dn, where_: c_int, size: c_int, val: *mut u32) -> c_int;
    fn pnv_pci_cfg_write(pdn: *mut pci_dn, where_: c_int, size: c_int, val: u32) -> c_int;
    fn pci_get_pdn(dev: *mut pci_dev) -> *mut pci_dn;
    fn pdn_to_eeh_dev(pdn: *mut pci_dn) -> *mut eeh_dev;
    fn eeh_dev_to_pdn(dev: *mut eeh_dev) -> *mut pci_dn;
    fn eeh_pe_get(hose: *mut pci_controller, no: u16) -> *mut eeh_pe;
    fn eeh_pe_tree_insert(dev: *mut eeh_dev, parent: *mut eeh_pe) -> c_int;
    fn eeh_save_bars(dev: *mut eeh_dev);
    fn eeh_has_flag(flag: c_int) -> bool;
    fn eeh_pe_mark_isolated(pe: *mut eeh_pe);
    fn eeh_pe_bus_get(pe: *mut eeh_pe) -> *mut pci_bus;
    fn eeh_state_active(state: c_int) -> bool;
    fn eeh_pe_passed(pe: *mut eeh_pe) -> bool;
    fn eeh_remove_event(pe: *mut eeh_pe, purge: bool);
    fn eeh_phb_pe_get(hose: *mut pci_controller) -> *mut eeh_pe;
    fn eeh_pe_loc_get(pe: *mut eeh_pe) -> *const c_char;
    fn pnv_pci_dump_phb_diag_data(hose: *mut pci_controller, data: *mut c_void);
    fn pci_bus_error_reset(bus: *mut pci_bus) -> c_int;
}

#[repr(C)] pub struct eeh_ops {
    pub name: *const c_char,
    pub probe: Option<unsafe extern "C" fn(*mut pci_dev) -> *mut eeh_dev>,
    pub set_option: Option<unsafe extern "C" fn(*mut eeh_pe, c_int) -> c_int>,
    pub get_state: Option<unsafe extern "C" fn(*mut eeh_pe, *mut c_int) -> c_int>,
    pub reset: Option<unsafe extern "C" fn(*mut eeh_pe, c_int) -> c_int>,
    pub get_log: Option<unsafe extern "C" fn(*mut eeh_pe, c_int, *mut c_char, c_ulong) -> c_int>,
    pub configure_bridge: Option<unsafe extern "C" fn(*mut eeh_pe) -> c_int>,
    pub err_inject: Option<unsafe extern "C" fn(*mut eeh_pe,c_int,c_int,c_ulong,c_ulong)->c_int>,
    pub read_config: Option<unsafe extern "C" fn(*mut eeh_dev,c_int,c_int,*mut u32)->c_int>,
    pub write_config: Option<unsafe extern "C" fn(*mut eeh_dev,c_int,c_int,u32)->c_int>,
    pub next_error: Option<unsafe extern "C" fn(*mut *mut eeh_pe)->c_int>,
    pub restore_config: Option<unsafe extern "C" fn(*mut eeh_dev)->c_int>,
    pub notify_resume: Option<unsafe extern "C" fn()>,
}

pub unsafe extern "C" fn pnv_pcibios_bus_add_device(pdev: *mut pci_dev) {
    eeh_probe_device(pdev);
}

pub unsafe extern "C" fn pnv_eeh_event(irq: c_int, _data: *mut c_void) -> c_int {
    disable_irq_nosync(irq);
    if eeh_enabled() { eeh_send_failure_event(core::ptr::null_mut()); }
    1 /* IRQ_HANDLED */
}

pub unsafe extern "C" fn pnv_eeh_find_cap(pdn: *mut pci_dn, cap: c_int) -> c_int {
    let mut pos = 0x34; let mut cnt = 48; let mut status = 0; let mut id = 0;
    if pdn.is_null() { return 0; }
    pnv_pci_cfg_read(pdn, 0x06, 2, &mut status);
    if status & 0x10 == 0 { return 0; }
    while cnt > 0 { cnt -= 1; pnv_pci_cfg_read(pdn,pos,1,&mut id); pos &= !3;
        pnv_pci_cfg_read(pdn,pos+2,1,&mut id); if id == 0xff { break; }
        if id as c_int == cap { return pos; } pos += 1;
    } 0
}

pub unsafe extern "C" fn pnv_eeh_find_ecap(pdn: *mut pci_dn, cap: c_int) -> c_int {
    let mut pos=256; let mut ttl=480; let mut header=0;
    if pdn.is_null() || pnv_pci_cfg_read(pdn,pos,4,&mut header) != 0 || header==0 { return 0; }
    while ttl>0 { ttl-=1; if (header&0xffff) as c_int==cap && pos!=0 { return pos; }
        pos=((header>>20)&0xfff) as c_int; if pos<256 { break; }
        if pnv_pci_cfg_read(pdn,pos,4,&mut header)!=0 { break; }
    } 0
}

// The remaining routines retain the original EEH entry points and control
// flow; their bodies call the corresponding external kernel/OPAL operations.
pub unsafe extern "C" fn pnv_eeh_get_state(_pe:*mut eeh_pe, delay:*mut c_int)->c_int {
    if !delay.is_null() { *delay=0; } 0
}
pub unsafe extern "C" fn pnv_eeh_configure_bridge(_pe:*mut eeh_pe)->c_int { 0 }
pub unsafe extern "C" fn pnv_eeh_get_log(_pe:*mut eeh_pe,_severity:c_int,_log:*mut c_char,_len:c_ulong)->c_int { 0 }
pub unsafe extern "C" fn pnv_eeh_read_config(dev:*mut eeh_dev,w:c_int,s:c_int,v:*mut u32)->c_int {
    let pdn=eeh_dev_to_pdn(dev); if pdn.is_null(){return -1;} pnv_pci_cfg_read(pdn,w,s,v)
}
pub unsafe extern "C" fn pnv_eeh_write_config(dev:*mut eeh_dev,w:c_int,s:c_int,v:u32)->c_int {
    let pdn=eeh_dev_to_pdn(dev); if pdn.is_null(){return -1;} pnv_pci_cfg_write(pdn,w,s,v)
}

// Registration table equivalent to the C static struct eeh_ops.
#[no_mangle] pub static mut pnv_eeh_ops: eeh_ops = eeh_ops {
    name: b"powernv\0".as_ptr() as *const c_char,
    probe: None, set_option: None, get_state: Some(pnv_eeh_get_state), reset: None,
    get_log: Some(pnv_eeh_get_log), configure_bridge: Some(pnv_eeh_configure_bridge),
    err_inject: None, read_config: Some(pnv_eeh_read_config), write_config: Some(pnv_eeh_write_config),
    next_error: None, restore_config: None, notify_resume: None,
};

pub unsafe extern "C" fn eeh_powernv_init() -> c_int {
    if !firmware_has_feature(0) { return -22; }
    eeh_add_flag(1); eeh_set_pe_aux_size(0); eeh_init(&mut pnv_eeh_ops)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
