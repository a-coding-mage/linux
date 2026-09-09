/* SPDX-License-Identifier: GPL-2.0 */

/*
 * APIs which are relevant for device driver code for allocating and
 * freeing MSI interrupts and querying the associations between
 * hardware/software MSI indices and the Linux interrupt number.
 */

pub struct device;

/*
 * Per device interrupt domain related constants.
 */
#[repr(C)]
pub enum msi_domain_ids {
    MSI_DEFAULT_DOMAIN,
    MSI_MAX_DEVICE_IRQDOMAINS,
}

/**
 * union msi_instance_cookie - MSI instance cookie
 * @value:\tu64 value store
 * @ptr:\tPointer to usage site specific data
 *
 * This cookie is handed to the IMS allocation function and stored in the
 * MSI descriptor for the interrupt chip callbacks.
 *
 * The content of this cookie is MSI domain implementation defined.  For
 * PCI/IMS implementations this could be a PASID or a pointer to queue
 * memory.
 */
#[repr(C)]
pub union msi_instance_cookie {
    pub value: u64,
    pub ptr: *mut core::ffi::c_void,
}

/**
 * msi_map - Mapping between MSI index and Linux interrupt number
 * @index:\tThe MSI index, e.g. slot in the MSI-X table or
 *\t\ta software managed index if >= 0. If negative
 *\t\tthe allocation function failed and it contains
 *\t\tthe error code.
 * @virq:\tThe associated Linux interrupt number
 */
#[repr(C)]
pub struct msi_map {
    pub index: core::ffi::c_int,
    pub virq: core::ffi::c_int,
}

/*
 * Constant to be used for dynamic allocations when the allocation is any
 * free MSI index, which is either an entry in a hardware table or a
 * software managed index.
 */
pub const MSI_ANY_INDEX: u32 = u32::MAX;

unsafe extern "C" {
    pub fn msi_domain_get_virq(
        dev: *mut device,
        domid: u32,
        index: u32,
    ) -> u32;
}

/**
 * msi_get_virq - Lookup the Linux interrupt number for a MSI index on the default interrupt domain
 * @dev:\tDevice for which the lookup happens
 * @index:\tThe MSI index to lookup
 *
 * Return: The Linux interrupt number on success (> 0), 0 if not found
 */
#[inline]
pub unsafe fn msi_get_virq(dev: *mut device, index: u32) -> u32 {
    msi_domain_get_virq(dev, msi_domain_ids::MSI_DEFAULT_DOMAIN as u32, index)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
