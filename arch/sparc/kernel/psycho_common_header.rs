/* SPDX-License-Identifier: GPL-2.0 */

// U2P Programmer's Manual, page 13-55, configuration space
// address format:
//
//  32             24 23 16 15    11 10       8 7   2  1 0
// ---------------------------------------------------------
// |0 0 0 0 0 0 0 0 1| bus | device | function | reg | 0 0 |
// ---------------------------------------------------------

/* C preprocessor macros retained as Rust macros to preserve field access and
 * call-site evaluation semantics. */
macro_rules! PSYCHO_CONFIG_BASE {
    ($pbm:expr) => {
        unsafe { (*$pbm).config_space | (1 as ::core::ffi::c_ulong << 24) }
    };
}

macro_rules! PSYCHO_CONFIG_ENCODE {
    ($bus:expr, $devfn:expr, $reg:expr) => {
        (($bus as ::core::ffi::c_ulong << 16)
            | ($devfn as ::core::ffi::c_ulong << 8)
            | ($reg as ::core::ffi::c_ulong))
    };
}

#[inline]
pub unsafe fn psycho_pci_config_mkaddr(
    pbm: *mut pci_pbm_info,
    bus: u8,
    devfn: u32,
    where_: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_void {
    (PSYCHO_CONFIG_BASE!(pbm) | PSYCHO_CONFIG_ENCODE!(bus, devfn, where_))
        as *mut ::core::ffi::c_void
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum psycho_error_type {
    UE_ERR,
    CE_ERR,
    PCI_ERR,
}

extern "C" {
    pub fn psycho_check_iommu_error(
        pbm: *mut pci_pbm_info,
        afsr: ::core::ffi::c_ulong,
        afar: ::core::ffi::c_ulong,
        type_: psycho_error_type,
    );

    pub fn psycho_pcierr_intr(irq: ::core::ffi::c_int, dev_id: *mut ::core::ffi::c_void) -> irqreturn_t;

    pub fn psycho_iommu_init(
        pbm: *mut pci_pbm_info,
        tsbsize: ::core::ffi::c_int,
        dvma_offset: u32,
        dma_mask: u32,
        write_complete_offset: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_int;

    pub fn psycho_pbm_init_common(
        pbm: *mut pci_pbm_info,
        op: *mut platform_device,
        chip_name: *const ::core::ffi::c_char,
        chip_type: ::core::ffi::c_int,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
