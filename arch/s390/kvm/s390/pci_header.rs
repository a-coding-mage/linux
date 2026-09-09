/* SPDX-License-Identifier: GPL-2.0 */
/*
 * s390 kvm PCI passthrough support
 *
 * Copyright IBM Corp. 2022
 *
 *    Author(s): Matthew Rosato <mjrosato@linux.ibm.com>
 */

// Dependencies supplied by the corresponding kernel headers are intentionally
// left as external Rust types and symbols.

#[repr(C)]
pub struct kvm_zdev {
    pub zdev: *mut zpci_dev,
    pub kvm: *mut kvm,
    pub fib: zpci_fib,
    pub entry: list_head,
    pub user_account: *mut user_struct,
    pub mm_account: *mut mm_struct,
}

#[repr(C)]
pub struct zpci_gaite {
    pub gisa: u32,
    pub gisc: u8,
    pub count: u8,
    pub reserved: u8,
    pub aisbo: u8,
    pub aisb: u64,
}

#[repr(C)]
pub struct zpci_aift {
    pub gait: *mut zpci_gaite,
    pub sbv: *mut airq_iv,
    pub kzdev: *mut *mut kvm_zdev,
    pub gait_lock: spinlock_t, // Protects the gait, used during AEN forward
    pub aift_lock: mutex, // Protects the other structures in aift
}

extern "C" {
    pub static mut aift: *mut zpci_aift;

    pub fn kvm_s390_pci_aen_init(nisc: u8) -> i32;
    pub fn kvm_s390_pci_aen_exit();

    pub fn kvm_s390_pci_init_list(kvm: *mut kvm);
    pub fn kvm_s390_pci_clear_list(kvm: *mut kvm);

    pub fn kvm_s390_pci_zpci_op(kvm: *mut kvm, args: *mut kvm_s390_zpci_op) -> i32;

    pub fn kvm_s390_pci_init() -> i32;
    pub fn kvm_s390_pci_exit();
}

#[inline]
pub unsafe fn kvm_s390_pci_si_to_kvm(aift: *mut zpci_aift, si: usize) -> *mut kvm {
    // IS_ENABLED(CONFIG_VFIO_PCI_ZDEV_KVM) is a build-time kernel condition.
    if !IS_ENABLED(CONFIG_VFIO_PCI_ZDEV_KVM) || (*aift).kzdev.is_null() || (*(*aift).kzdev.add(si)).is_null() {
        core::ptr::null_mut()
    } else {
        (**(*aift).kzdev.add(si)).kvm
    }
}

#[inline]
pub unsafe fn kvm_s390_pci_interp_allowed() -> bool {
    let mut cpu_id: cpuid = core::mem::zeroed();

    get_cpu_id(&mut cpu_id);
    match cpu_id.machine {
        0x2817 | 0x2818 | 0x2827 | 0x2828 | 0x2964 | 0x2965 => {
            /* No SHM on certain machines */
            false
        }
        _ => {
            IS_ENABLED(CONFIG_VFIO_PCI_ZDEV_KVM)
                && sclp.has_zpci_lsi
                && sclp.has_aeni
                && sclp.has_aisi
                && sclp.has_aisii
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
