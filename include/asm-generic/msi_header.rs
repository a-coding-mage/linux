/* SPDX-License-Identifier: GPL-2.0 */

// The declarations below are enabled when CONFIG_GENERIC_MSI_IRQ is enabled
// in the C build configuration.
#[cfg(feature = "CONFIG_GENERIC_MSI_IRQ")]
pub const NUM_MSI_ALLOC_SCRATCHPAD_REGS: usize = 2;

#[cfg(feature = "CONFIG_GENERIC_MSI_IRQ")]
#[repr(C)]
pub struct msi_desc {
    _private: [u8; 0],
}

/**
 * struct msi_alloc_info - Default structure for MSI interrupt allocation.
 * @desc:        Pointer to msi descriptor
 * @hwirq:       Associated hw interrupt number in the domain
 * @scratchpad:  Storage for implementation specific scratch data
 *
 * Architectures can provide their own implementation by not including
 * asm-generic/msi.h into their arch specific header file.
 */
#[cfg(feature = "CONFIG_GENERIC_MSI_IRQ")]
#[repr(C)]
pub union msi_alloc_info_scratchpad {
    pub ul: ::core::ffi::c_ulong,
    pub ptr: *mut ::core::ffi::c_void,
}

#[cfg(feature = "CONFIG_GENERIC_MSI_IRQ")]
#[repr(C)]
pub struct msi_alloc_info {
    pub desc: *mut msi_desc,
    pub hwirq: irq_hw_number_t,
    pub flags: ::core::ffi::c_ulong,
    pub scratchpad: [msi_alloc_info_scratchpad; NUM_MSI_ALLOC_SCRATCHPAD_REGS],
}

#[cfg(feature = "CONFIG_GENERIC_MSI_IRQ")]
pub type msi_alloc_info_t = msi_alloc_info;

/* Device generating MSIs is proxying for another device */
#[cfg(feature = "CONFIG_GENERIC_MSI_IRQ")]
pub const MSI_ALLOC_FLAGS_PROXY_DEVICE: ::core::ffi::c_ulong = 1UL << 0;

#[cfg(feature = "CONFIG_GENERIC_MSI_IRQ")]
pub const MSI_ALLOC_FLAGS_FIXED_MSG_DATA: ::core::ffi::c_ulong = 1UL << 1;

#[cfg(feature = "CONFIG_GENERIC_MSI_IRQ")]
pub const GENERIC_MSI_DOMAIN_OPS: i32 = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
