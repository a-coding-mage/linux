/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from pci-tsm.h. Kernel-provided types and constants are external dependencies. */

#[repr(C)]
pub struct pci_tsm { pub pdev: *mut pci_dev, pub dsm_dev: *mut pci_dev, pub tsm_dev: *mut tsm_dev, pub tdi: *mut pci_tdi }
#[repr(C)] pub struct tsm_dev { _private: [u8; 0] }
#[repr(C)] pub struct kvm { _private: [u8; 0] }
#[repr(C)] pub struct pci_dev { pub devfn: u32, pub is_virtfn: bool, pub ide_cap: bool, pub devcap: u32 }
#[repr(C)] pub struct pci_doe_mb { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
pub type sockptr_t = usize;

#[repr(C)]
pub struct pci_tdi { pub pdev: *mut pci_dev, pub kvm: *mut kvm, pub tdi_id: u32 }

#[repr(C)]
pub struct pci_tsm_link_ops {
    pub probe: Option<unsafe extern "C" fn(*mut tsm_dev, *mut pci_dev) -> *mut pci_tsm>,
    pub remove: Option<unsafe extern "C" fn(*mut pci_tsm)>,
    pub connect: Option<unsafe extern "C" fn(*mut pci_dev) -> i32>,
    pub disconnect: Option<unsafe extern "C" fn(*mut pci_dev)>,
    pub bind: Option<unsafe extern "C" fn(*mut pci_dev, *mut kvm, u32) -> *mut pci_tdi>,
    pub unbind: Option<unsafe extern "C" fn(*mut pci_tdi)>,
    pub guest_req: Option<unsafe extern "C" fn(*mut pci_tdi, pci_tsm_req_scope, sockptr_t, usize, sockptr_t, usize, *mut u64) -> isize>,
}
#[repr(C)]
pub struct pci_tsm_devsec_ops {
    pub lock: Option<unsafe extern "C" fn(*mut tsm_dev, *mut pci_dev) -> *mut pci_tsm>,
    pub unlock: Option<unsafe extern "C" fn(*mut pci_tsm)>,
}
#[repr(C)]
pub struct pci_tsm_ops { pub link_ops: pci_tsm_link_ops, pub devsec_ops: pci_tsm_devsec_ops }

#[repr(C)]
pub struct pci_tsm_pf0 { pub base_tsm: pci_tsm, pub lock: mutex, pub doe_mb: *mut pci_doe_mb }

#[inline]
pub unsafe fn is_pci_tsm_pf0(pdev: *mut pci_dev) -> bool {
    if pdev.is_null() || !pci_is_pcie(pdev) || (*pdev).is_virtfn { return false; }
    match pci_pcie_type(pdev) {
        PCI_EXP_TYPE_ENDPOINT | PCI_EXP_TYPE_UPSTREAM | PCI_EXP_TYPE_RC_END => {
            if !((*pdev).ide_cap || ((*pdev).devcap & PCI_EXP_DEVCAP_TEE) != 0) { return false; }
        }
        _ => return false,
    }
    pci_func((*pdev).devfn) == 0
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum pci_tsm_req_scope { PCI_TSM_REQ_INFO = 0, PCI_TSM_REQ_STATE_CHANGE = 1, PCI_TSM_REQ_DEBUG_READ = 2, PCI_TSM_REQ_DEBUG_WRITE = 3 }

pub const PCI_EXP_TYPE_ENDPOINT: u32 = 0;
pub const PCI_EXP_TYPE_UPSTREAM: u32 = 5;
pub const PCI_EXP_TYPE_RC_END: u32 = 9;
pub const PCI_EXP_DEVCAP_TEE: u32 = 1 << 18;

extern "C" {
    fn pci_is_pcie(pdev: *mut pci_dev) -> bool;
    fn pci_pcie_type(pdev: *mut pci_dev) -> u32;
    fn pci_func(devfn: u32) -> u32;
    fn pci_tsm_register(tsm_dev: *mut tsm_dev) -> i32;
    fn pci_tsm_unregister(tsm_dev: *mut tsm_dev);
    fn pci_tsm_link_constructor(pdev: *mut pci_dev, tsm: *mut pci_tsm, tsm_dev: *mut tsm_dev) -> i32;
    fn pci_tsm_pf0_constructor(pdev: *mut pci_dev, tsm: *mut pci_tsm_pf0, tsm_dev: *mut tsm_dev) -> i32;
    fn pci_tsm_pf0_destructor(tsm: *mut pci_tsm_pf0);
    fn pci_tsm_doe_transfer(pdev: *mut pci_dev, type_: u8, req: *const core::ffi::c_void, req_sz: usize, resp: *mut core::ffi::c_void, resp_sz: usize) -> i32;
    fn pci_tsm_bind(pdev: *mut pci_dev, kvm: *mut kvm, tdi_id: u32) -> i32;
    fn pci_tsm_unbind(pdev: *mut pci_dev);
    fn pci_tsm_tdi_constructor(pdev: *mut pci_dev, tdi: *mut pci_tdi, kvm: *mut kvm, tdi_id: u32);
    fn pci_tsm_guest_req(pdev: *mut pci_dev, scope: pci_tsm_req_scope, req_in: sockptr_t, in_len: usize, req_out: sockptr_t, out_len: usize, tsm_code: *mut u64) -> isize;
}

#[inline] pub unsafe fn pci_tsm_register_stub(_: *mut tsm_dev) -> i32 { 0 }
#[inline] pub unsafe fn pci_tsm_unregister_stub(_: *mut tsm_dev) {}
#[inline] pub unsafe fn pci_tsm_bind_stub(_: *mut pci_dev, _: *mut kvm, _: u64) -> i32 { -6 }
#[inline] pub unsafe fn pci_tsm_unbind_stub(_: *mut pci_dev) {}
#[inline] pub unsafe fn pci_tsm_guest_req_stub(_: *mut pci_dev, _: pci_tsm_req_scope, _: sockptr_t, _: usize, _: sockptr_t, _: usize, _: *mut u64) -> isize { -6 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
