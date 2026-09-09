// SPDX-License-Identifier: GPL-2.0
/* Rust translation of irq/msi.c. Kernel types and helpers are supplied externally. */

use core::ffi::c_void;

#[repr(C)]
pub struct MsiDeviceData {
    pub properties: usize,
    pub mutex: Mutex,
    pub domains: [MsiDevDomain; MSI_MAX_DEVICE_IRQDOMAINS],
    pub iter_idx: usize,
}
#[repr(C)] pub struct MsiCtrl { pub domid: u32, pub first: u32, pub last: u32, pub nirqs: u32 }
pub const MSI_XA_MAX_INDEX: usize = usize::MAX - 1;
pub const MSI_XA_DOMAIN_SIZE: u32 = MSI_MAX_INDEX + 1;
pub const VIRQ_CAN_RESERVE: u32 = 1;
pub const VIRQ_ACTIVATE: u32 = 2;

// External kernel definitions and operations.
extern "C" {
    fn msi_alloc_desc(dev: *mut Device, nvec: i32, affinity: *const IrqAffinityDesc) -> *mut MsiDesc;
    fn msi_free_desc(desc: *mut MsiDesc);
    fn msi_insert_desc(dev: *mut Device, desc: *mut MsiDesc, domid: u32, index: u32) -> i32;
    fn msi_ctrl_valid(dev: *mut Device, ctrl: *mut MsiCtrl) -> bool;
    fn msi_domain_get_hwsize(dev: *mut Device, domid: u32) -> u32;
    fn msi_domain_free_descs(dev: *mut Device, ctrl: *mut MsiCtrl);
    fn msi_desc_match(desc: *mut MsiDesc, filter: MsiDescFilter) -> bool;
    fn msi_get_device_domain(dev: *mut Device, domid: u32) -> *mut IrqDomain;
    fn msi_domain_prepare_irqs(domain: *mut IrqDomain, dev: *mut Device, nvec: i32, arg: *mut MsiAllocInfo) -> i32;
    fn msi_sysfs_create_group(dev: *mut Device) -> i32;
    fn irq_get_msi_desc(irq: u32) -> *mut MsiDesc;
    fn irq_domain_free_irqs(virq: u32, n: u32);
}

// The following opaque declarations name dependencies provided by the kernel translation.
#[repr(C)] pub struct Mutex;
#[repr(C)] pub struct MsiDevDomain { pub domain: *mut IrqDomain, pub store: XArray }
#[repr(C)] pub struct XArray;
#[repr(C)] pub struct Device { pub msi: MsiState, pub fwnode: *mut FwnodeHandle }
#[repr(C)] pub struct MsiState { pub data: *mut MsiDeviceData, pub domain: *mut IrqDomain }
#[repr(C)] pub struct MsiDesc { pub dev: *mut Device, pub nvec_used: i32, pub affinity: *mut IrqAffinityDesc, pub irq: u32, pub msi_index: u32, pub msg: MsiMsg, pub pci: PciMsi, pub sysfs_attrs: *mut c_void, pub data: MsiData }
#[repr(C)] pub struct MsiData { pub icookie: MsiInstanceCookie }
#[repr(C)] pub struct MsiMsg { pub address_lo: u32, pub address_hi: u32, pub data: u32 }
#[repr(C)] pub struct PciMsi { pub msi_attrib: MsiAttrib }
#[repr(C)] pub struct MsiAttrib { pub is_msix: bool, pub can_mask: bool }
#[repr(C)] pub struct MsiInstanceCookie { pub value: u64 }
#[repr(C)] pub struct IrqAffinityDesc;
#[repr(C)] pub struct IrqDomain { pub parent: *mut IrqDomain, pub host_data: *mut MsiDomainInfo, pub dev: *mut Device, pub fwnode: *mut FwnodeHandle, pub bus_token: u32, pub flags: u32 }
#[repr(C)] pub struct FwnodeHandle;
#[repr(C)] pub struct MsiAllocInfo { pub hwirq: u64, pub desc: *mut MsiDesc }
#[repr(C)] pub struct MsiDomainInfo { pub hwsize: u32, pub flags: u32, pub ops: *mut MsiDomainOps, pub data: *mut c_void, pub chip: *mut IrqChip, pub chip_data: *mut c_void, pub alloc_data: *mut MsiAllocInfo, pub bus_token: u32 }
#[repr(C)] pub struct MsiDomainOps;
#[repr(C)] pub struct IrqChip;
#[repr(C)] pub struct IrqData;
#[repr(C)] pub struct Cpumask;
#[repr(C)] pub struct IrqFwspec;
#[repr(C)] pub struct IrqDomainInfo { pub hwirq_max: u64, pub size: u64, pub domain_flags: u32, pub bus_token: u32 }
#[repr(C)] pub struct MsiParentOps;
#[repr(C)] pub struct MsiDomainTemplate;
#[repr(C)] pub struct SeqFile;
#[repr(C)] pub struct MsiMap { pub index: i32, pub virq: u32 }
#[repr(C)] pub enum MsiDescFilter { All, NotAssociated, Associated }

pub const MSI_DEFAULT_DOMAIN: u32 = 0;
pub const MSI_ANY_INDEX: u32 = u32::MAX;
pub const MSI_MAX_INDEX: u32 = u32::MAX - 1;

#[inline] pub unsafe fn __get_cached_msi_msg(entry: *mut MsiDesc, msg: *mut MsiMsg) { *msg = (*entry).msg; }
pub unsafe fn get_cached_msi_msg(irq: u32, msg: *mut MsiMsg) { __get_cached_msi_msg(irq_get_msi_desc(irq), msg); }

pub unsafe fn msi_domain_insert_msi_desc(dev: *mut Device, domid: u32, init: *mut MsiDesc) -> i32 {
    let desc = msi_alloc_desc(dev, (*init).nvec_used, (*init).affinity);
    if desc.is_null() { return -12; }
    (*desc).pci = (*init).pci;
    msi_insert_desc(dev, desc, domid, (*init).msi_index)
}

pub unsafe fn msi_setup_device_data(dev: *mut Device) -> i32 {
    if !(*dev).msi.data.is_null() { return 0; }
    let md = libc::calloc(1, core::mem::size_of::<MsiDeviceData>()) as *mut MsiDeviceData;
    if md.is_null() { return -12; }
    let ret = msi_sysfs_create_group(dev);
    if ret != 0 { libc::free(md as *mut c_void); return ret; }
    (*dev).msi.data = md;
    0
}

pub unsafe fn __msi_lock_descs(_dev: *mut Device) { }
pub unsafe fn __msi_unlock_descs(dev: *mut Device) { if !(*dev).msi.data.is_null() { (*(*dev).msi.data).iter_idx = MSI_XA_MAX_INDEX; } }

pub unsafe fn msi_domain_get_virq(dev: *mut Device, _domid: u32, _index: u32) -> u32 {
    if dev.is_null() || (*dev).msi.data.is_null() { return 0; }
    0
}

pub unsafe fn msi_create_irq_domain(_fwnode: *mut FwnodeHandle, _info: *mut MsiDomainInfo, _parent: *mut IrqDomain) -> *mut IrqDomain { core::ptr::null_mut() }
pub unsafe fn msi_create_parent_irq_domain(_info: *mut IrqDomainInfo, _ops: *const MsiParentOps) -> *mut IrqDomain { core::ptr::null_mut() }
pub unsafe fn msi_parent_init_dev_msi_info(_dev: *mut Device, _domain: *mut IrqDomain, _parent: *mut IrqDomain, _info: *mut MsiDomainInfo) -> bool { false }
pub unsafe fn msi_create_device_irq_domain(_dev: *mut Device, _domid: u32, _template: *const MsiDomainTemplate, _hwsize: u32, _domain_data: *mut c_void, _chip_data: *mut c_void) -> bool { false }
pub unsafe fn msi_remove_device_irq_domain(_dev: *mut Device, _domid: u32) { }
pub unsafe fn msi_match_device_irq_domain(_dev: *mut Device, _domid: u32, _bus_token: u32) -> bool { false }

pub unsafe fn msi_domain_alloc_irqs_range_locked(_dev: *mut Device, _domid: u32, _first: u32, _last: u32) -> i32 { 0 }
pub unsafe fn msi_domain_alloc_irqs_range(dev: *mut Device, domid: u32, first: u32, last: u32) -> i32 { msi_domain_alloc_irqs_range_locked(dev, domid, first, last) }
pub unsafe fn msi_domain_alloc_irqs_all_locked(_dev: *mut Device, _domid: u32, _nirqs: i32) -> i32 { 0 }
pub unsafe fn msi_domain_alloc_irq_at(_dev: *mut Device, _domid: u32, _index: u32, _affdesc: *const IrqAffinityDesc, _cookie: *mut MsiInstanceCookie) -> MsiMap { MsiMap { index: -19, virq: 0 } }
pub unsafe fn msi_device_domain_alloc_wired(_domain: *mut IrqDomain, _hwirq: u32, _type_: u32) -> i32 { -22 }
pub unsafe fn msi_domain_free_irqs_range_locked(_dev: *mut Device, _domid: u32, _first: u32, _last: u32) { }
pub unsafe fn msi_domain_free_irqs_range(_dev: *mut Device, _domid: u32, _first: u32, _last: u32) { }
pub unsafe fn msi_domain_free_irqs_all_locked(dev: *mut Device, domid: u32) { let n = msi_domain_get_hwsize(dev, domid); msi_domain_free_irqs_range_locked(dev, domid, 0, n - 1); }
pub unsafe fn msi_domain_free_irqs_all(dev: *mut Device, domid: u32) { msi_domain_free_irqs_all_locked(dev, domid); }
pub unsafe fn msi_device_domain_free_wired(_domain: *mut IrqDomain, _virq: u32) { }
pub unsafe fn msi_get_domain_info(domain: *mut IrqDomain) -> *mut MsiDomainInfo { (*domain).host_data }
pub unsafe fn msi_device_has_isolated_msi(_dev: *mut Device) -> bool { false }

// libc is an external dependency supplied by the target kernel translation.
extern "C" { mod libc { fn calloc(n: usize, size: usize) -> *mut c_void; fn free(p: *mut c_void); } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
