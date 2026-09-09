/* SPDX-License-Identifier: GPL-2.0 */
/* pci_sun4v.h: SUN4V specific PCI controller support.
 *
 * Copyright (C) 2006 David S. Miller (davem@davemloft.net)
 */

/* C header guard: _PCI_SUN4V_H */

extern "C" {
    pub fn pci_sun4v_iommu_map(
        devhandle: ::core::ffi::c_ulong,
        tsbid: ::core::ffi::c_ulong,
        num_ttes: ::core::ffi::c_ulong,
        io_attributes: ::core::ffi::c_ulong,
        io_page_list_pa: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_long;
    pub fn pci_sun4v_iommu_demap(
        devhandle: ::core::ffi::c_ulong,
        tsbid: ::core::ffi::c_ulong,
        num_ttes: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_ulong;
    pub fn pci_sun4v_iommu_getmap(
        devhandle: ::core::ffi::c_ulong,
        tsbid: ::core::ffi::c_ulong,
        io_attributes: *mut ::core::ffi::c_ulong,
        real_address: *mut ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_ulong;
    pub fn pci_sun4v_config_get(
        devhandle: ::core::ffi::c_ulong,
        pci_device: ::core::ffi::c_ulong,
        config_offset: ::core::ffi::c_ulong,
        size: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_ulong;
    pub fn pci_sun4v_config_put(
        devhandle: ::core::ffi::c_ulong,
        pci_device: ::core::ffi::c_ulong,
        config_offset: ::core::ffi::c_ulong,
        size: ::core::ffi::c_ulong,
        data: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_int;

    pub fn pci_sun4v_msiq_conf(devhandle: ::core::ffi::c_ulong, msiqid: ::core::ffi::c_ulong, msiq_paddr: ::core::ffi::c_ulong, num_entries: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong;
    pub fn pci_sun4v_msiq_info(devhandle: ::core::ffi::c_ulong, msiqid: ::core::ffi::c_ulong, msiq_paddr: *mut ::core::ffi::c_ulong, num_entries: *mut ::core::ffi::c_ulong) -> ::core::ffi::c_ulong;
    pub fn pci_sun4v_msiq_getvalid(devhandle: ::core::ffi::c_ulong, msiqid: ::core::ffi::c_ulong, valid: *mut ::core::ffi::c_ulong) -> ::core::ffi::c_ulong;
    pub fn pci_sun4v_msiq_setvalid(devhandle: ::core::ffi::c_ulong, msiqid: ::core::ffi::c_ulong, valid: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong;
    pub fn pci_sun4v_msiq_getstate(devhandle: ::core::ffi::c_ulong, msiqid: ::core::ffi::c_ulong, state: *mut ::core::ffi::c_ulong) -> ::core::ffi::c_ulong;
    pub fn pci_sun4v_msiq_setstate(devhandle: ::core::ffi::c_ulong, msiqid: ::core::ffi::c_ulong, state: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong;
    pub fn pci_sun4v_msiq_gethead(devhandle: ::core::ffi::c_ulong, msiqid: ::core::ffi::c_ulong, head: *mut ::core::ffi::c_ulong) -> ::core::ffi::c_ulong;
    pub fn pci_sun4v_msiq_sethead(devhandle: ::core::ffi::c_ulong, msiqid: ::core::ffi::c_ulong, head: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong;
    pub fn pci_sun4v_msiq_gettail(devhandle: ::core::ffi::c_ulong, msiqid: ::core::ffi::c_ulong, head: *mut ::core::ffi::c_ulong) -> ::core::ffi::c_ulong;
    pub fn pci_sun4v_msi_getvalid(devhandle: ::core::ffi::c_ulong, msinum: ::core::ffi::c_ulong, valid: *mut ::core::ffi::c_ulong) -> ::core::ffi::c_ulong;
    pub fn pci_sun4v_msi_setvalid(devhandle: ::core::ffi::c_ulong, msinum: ::core::ffi::c_ulong, valid: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong;
    pub fn pci_sun4v_msi_getmsiq(devhandle: ::core::ffi::c_ulong, msinum: ::core::ffi::c_ulong, msiq: *mut ::core::ffi::c_ulong) -> ::core::ffi::c_ulong;
    pub fn pci_sun4v_msi_setmsiq(devhandle: ::core::ffi::c_ulong, msinum: ::core::ffi::c_ulong, msiq: ::core::ffi::c_ulong, msitype: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong;
    pub fn pci_sun4v_msi_getstate(devhandle: ::core::ffi::c_ulong, msinum: ::core::ffi::c_ulong, state: *mut ::core::ffi::c_ulong) -> ::core::ffi::c_ulong;
    pub fn pci_sun4v_msi_setstate(devhandle: ::core::ffi::c_ulong, msinum: ::core::ffi::c_ulong, state: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong;
    pub fn pci_sun4v_msg_getmsiq(devhandle: ::core::ffi::c_ulong, msinum: ::core::ffi::c_ulong, msiq: *mut ::core::ffi::c_ulong) -> ::core::ffi::c_ulong;
    pub fn pci_sun4v_msg_setmsiq(devhandle: ::core::ffi::c_ulong, msinum: ::core::ffi::c_ulong, msiq: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong;
    pub fn pci_sun4v_msg_getvalid(devhandle: ::core::ffi::c_ulong, msinum: ::core::ffi::c_ulong, valid: *mut ::core::ffi::c_ulong) -> ::core::ffi::c_ulong;
    pub fn pci_sun4v_msg_setvalid(devhandle: ::core::ffi::c_ulong, msinum: ::core::ffi::c_ulong, valid: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong;

    /* Sun4v HV IOMMU v2 APIs */
    pub fn pci_sun4v_iotsb_conf(devhandle: ::core::ffi::c_ulong, ra: ::core::ffi::c_ulong, table_size: ::core::ffi::c_ulong, page_size: ::core::ffi::c_ulong, dvma_base: ::core::ffi::c_ulong, iotsb_num: *mut u64) -> ::core::ffi::c_ulong;
    pub fn pci_sun4v_iotsb_bind(devhandle: ::core::ffi::c_ulong, iotsb_num: ::core::ffi::c_ulong, pci_device: ::core::ffi::c_uint) -> ::core::ffi::c_ulong;
    pub fn pci_sun4v_iotsb_map(devhandle: ::core::ffi::c_ulong, iotsb_num: ::core::ffi::c_ulong, iotsb_index_iottes: ::core::ffi::c_ulong, io_attributes: ::core::ffi::c_ulong, io_page_list_pa: ::core::ffi::c_ulong, mapped: *mut ::core::ffi::c_long) -> ::core::ffi::c_ulong;
    pub fn pci_sun4v_iotsb_demap(devhandle: ::core::ffi::c_ulong, iotsb_num: ::core::ffi::c_ulong, iotsb_index: ::core::ffi::c_ulong, iottes: ::core::ffi::c_ulong, demapped: *mut ::core::ffi::c_ulong) -> ::core::ffi::c_ulong;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
