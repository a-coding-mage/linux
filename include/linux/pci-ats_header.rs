/* SPDX-License-Identifier: GPL-2.0 */

/* Dependency supplied by linux/pci.h. */
#[repr(C)]
pub struct pci_dev {
    _private: [u8; 0],
}

/* Address Translation Service.
 * The CONFIG_PCI_ATS build condition is preserved from the C header.
 */
#[cfg(CONFIG_PCI_ATS)]
extern "C" {
    pub fn pci_ats_supported(dev: *mut pci_dev) -> bool;
    pub fn pci_enable_ats(dev: *mut pci_dev, ps: i32) -> i32;
    pub fn pci_prepare_ats(dev: *mut pci_dev, ps: i32) -> i32;
    pub fn pci_disable_ats(dev: *mut pci_dev);
    pub fn pci_ats_queue_depth(dev: *mut pci_dev) -> i32;
    pub fn pci_ats_page_aligned(dev: *mut pci_dev) -> i32;
    pub fn pci_ats_required(dev: *mut pci_dev) -> bool;
}

#[cfg(not(CONFIG_PCI_ATS))]
#[inline]
pub unsafe fn pci_ats_supported(_d: *mut pci_dev) -> bool { false }

#[cfg(not(CONFIG_PCI_ATS))]
#[inline]
pub unsafe fn pci_enable_ats(_d: *mut pci_dev, _ps: i32) -> i32 { -ENODEV }

#[cfg(not(CONFIG_PCI_ATS))]
#[inline]
pub unsafe fn pci_prepare_ats(_dev: *mut pci_dev, _ps: i32) -> i32 { -ENODEV }

#[cfg(not(CONFIG_PCI_ATS))]
#[inline]
pub unsafe fn pci_disable_ats(_d: *mut pci_dev) {}

#[cfg(not(CONFIG_PCI_ATS))]
#[inline]
pub unsafe fn pci_ats_queue_depth(_d: *mut pci_dev) -> i32 { -ENODEV }

#[cfg(not(CONFIG_PCI_ATS))]
#[inline]
pub unsafe fn pci_ats_page_aligned(_dev: *mut pci_dev) -> i32 { 0 }

#[cfg(not(CONFIG_PCI_ATS))]
#[inline]
pub unsafe fn pci_ats_required(_dev: *mut pci_dev) -> bool { false }

/* The CONFIG_PCI_PRI build condition is preserved from the C header. */
#[cfg(CONFIG_PCI_PRI)]
extern "C" {
    pub fn pci_enable_pri(pdev: *mut pci_dev, reqs: u32) -> i32;
    pub fn pci_disable_pri(pdev: *mut pci_dev);
    pub fn pci_reset_pri(pdev: *mut pci_dev) -> i32;
    pub fn pci_prg_resp_pasid_required(pdev: *mut pci_dev) -> i32;
    pub fn pci_pri_supported(pdev: *mut pci_dev) -> bool;
}

#[cfg(not(CONFIG_PCI_PRI))]
#[inline]
pub unsafe fn pci_pri_supported(_pdev: *mut pci_dev) -> bool { false }

/* The CONFIG_PCI_PASID build condition is preserved from the C header. */
#[cfg(CONFIG_PCI_PASID)]
extern "C" {
    pub fn pci_enable_pasid(pdev: *mut pci_dev, features: i32) -> i32;
    pub fn pci_disable_pasid(pdev: *mut pci_dev);
    pub fn pci_pasid_features(pdev: *mut pci_dev) -> i32;
    pub fn pci_max_pasids(pdev: *mut pci_dev) -> i32;
    pub fn pci_pasid_status(pdev: *mut pci_dev) -> i32;
}

#[cfg(not(CONFIG_PCI_PASID))]
#[inline]
pub unsafe fn pci_enable_pasid(_pdev: *mut pci_dev, _features: i32) -> i32 { -EINVAL }

#[cfg(not(CONFIG_PCI_PASID))]
#[inline]
pub unsafe fn pci_disable_pasid(_pdev: *mut pci_dev) {}

#[cfg(not(CONFIG_PCI_PASID))]
#[inline]
pub unsafe fn pci_pasid_features(_pdev: *mut pci_dev) -> i32 { -EINVAL }

#[cfg(not(CONFIG_PCI_PASID))]
#[inline]
pub unsafe fn pci_max_pasids(_pdev: *mut pci_dev) -> i32 { -EINVAL }

#[cfg(not(CONFIG_PCI_PASID))]
#[inline]
pub unsafe fn pci_pasid_status(_pdev: *mut pci_dev) -> i32 { -EINVAL }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
