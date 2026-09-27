/* SPDX-License-Identifier: GPL-2.0 */
/* pci_sun4v.h: SUN4V specific PCI controller support.
 *
 * Copyright (C) 2006 David S. Miller (davem@davemloft.net)
 */

/* C header guard: _PCI_SUN4V_H */

extern "C" {
    pub fn pci_sun4v_iommu_map(
        devhandle: ::kernel::ffi::c_ulong,
        tsbid: ::kernel::ffi::c_ulong,
        num_ttes: ::kernel::ffi::c_ulong,
        io_attributes: ::kernel::ffi::c_ulong,
        io_page_list_pa: ::kernel::ffi::c_ulong,
    ) -> ::kernel::ffi::c_long;
    pub fn pci_sun4v_iommu_demap(
        devhandle: ::kernel::ffi::c_ulong,
        tsbid: ::kernel::ffi::c_ulong,
        num_ttes: ::kernel::ffi::c_ulong,
    ) -> ::kernel::ffi::c_ulong;
    pub fn pci_sun4v_iommu_getmap(
        devhandle: ::kernel::ffi::c_ulong,
        tsbid: ::kernel::ffi::c_ulong,
        io_attributes: *mut ::kernel::ffi::c_ulong,
        real_address: *mut ::kernel::ffi::c_ulong,
    ) -> ::kernel::ffi::c_ulong;
    pub fn pci_sun4v_config_get(
        devhandle: ::kernel::ffi::c_ulong,
        pci_device: ::kernel::ffi::c_ulong,
        config_offset: ::kernel::ffi::c_ulong,
        size: ::kernel::ffi::c_ulong,
    ) -> ::kernel::ffi::c_ulong;
    pub fn pci_sun4v_config_put(
        devhandle: ::kernel::ffi::c_ulong,
        pci_device: ::kernel::ffi::c_ulong,
        config_offset: ::kernel::ffi::c_ulong,
        size: ::kernel::ffi::c_ulong,
        data: ::kernel::ffi::c_ulong,
    ) -> ::kernel::ffi::c_int;

    pub fn pci_sun4v_msiq_conf(devhandle: ::kernel::ffi::c_ulong, msiqid: ::kernel::ffi::c_ulong, msiq_paddr: ::kernel::ffi::c_ulong, num_entries: ::kernel::ffi::c_ulong) -> ::kernel::ffi::c_ulong;
    pub fn pci_sun4v_msiq_info(devhandle: ::kernel::ffi::c_ulong, msiqid: ::kernel::ffi::c_ulong, msiq_paddr: *mut ::kernel::ffi::c_ulong, num_entries: *mut ::kernel::ffi::c_ulong) -> ::kernel::ffi::c_ulong;
    pub fn pci_sun4v_msiq_getvalid(devhandle: ::kernel::ffi::c_ulong, msiqid: ::kernel::ffi::c_ulong, valid: *mut ::kernel::ffi::c_ulong) -> ::kernel::ffi::c_ulong;
    pub fn pci_sun4v_msiq_setvalid(devhandle: ::kernel::ffi::c_ulong, msiqid: ::kernel::ffi::c_ulong, valid: ::kernel::ffi::c_ulong) -> ::kernel::ffi::c_ulong;
    pub fn pci_sun4v_msiq_getstate(devhandle: ::kernel::ffi::c_ulong, msiqid: ::kernel::ffi::c_ulong, state: *mut ::kernel::ffi::c_ulong) -> ::kernel::ffi::c_ulong;
    pub fn pci_sun4v_msiq_setstate(devhandle: ::kernel::ffi::c_ulong, msiqid: ::kernel::ffi::c_ulong, state: ::kernel::ffi::c_ulong) -> ::kernel::ffi::c_ulong;
    pub fn pci_sun4v_msiq_gethead(devhandle: ::kernel::ffi::c_ulong, msiqid: ::kernel::ffi::c_ulong, head: *mut ::kernel::ffi::c_ulong) -> ::kernel::ffi::c_ulong;
    pub fn pci_sun4v_msiq_sethead(devhandle: ::kernel::ffi::c_ulong, msiqid: ::kernel::ffi::c_ulong, head: ::kernel::ffi::c_ulong) -> ::kernel::ffi::c_ulong;
    pub fn pci_sun4v_msiq_gettail(devhandle: ::kernel::ffi::c_ulong, msiqid: ::kernel::ffi::c_ulong, head: *mut ::kernel::ffi::c_ulong) -> ::kernel::ffi::c_ulong;
    pub fn pci_sun4v_msi_getvalid(devhandle: ::kernel::ffi::c_ulong, msinum: ::kernel::ffi::c_ulong, valid: *mut ::kernel::ffi::c_ulong) -> ::kernel::ffi::c_ulong;
    pub fn pci_sun4v_msi_setvalid(devhandle: ::kernel::ffi::c_ulong, msinum: ::kernel::ffi::c_ulong, valid: ::kernel::ffi::c_ulong) -> ::kernel::ffi::c_ulong;
    pub fn pci_sun4v_msi_getmsiq(devhandle: ::kernel::ffi::c_ulong, msinum: ::kernel::ffi::c_ulong, msiq: *mut ::kernel::ffi::c_ulong) -> ::kernel::ffi::c_ulong;
    pub fn pci_sun4v_msi_setmsiq(devhandle: ::kernel::ffi::c_ulong, msinum: ::kernel::ffi::c_ulong, msiq: ::kernel::ffi::c_ulong, msitype: ::kernel::ffi::c_ulong) -> ::kernel::ffi::c_ulong;
    pub fn pci_sun4v_msi_getstate(devhandle: ::kernel::ffi::c_ulong, msinum: ::kernel::ffi::c_ulong, state: *mut ::kernel::ffi::c_ulong) -> ::kernel::ffi::c_ulong;
    pub fn pci_sun4v_msi_setstate(devhandle: ::kernel::ffi::c_ulong, msinum: ::kernel::ffi::c_ulong, state: ::kernel::ffi::c_ulong) -> ::kernel::ffi::c_ulong;
    pub fn pci_sun4v_msg_getmsiq(devhandle: ::kernel::ffi::c_ulong, msinum: ::kernel::ffi::c_ulong, msiq: *mut ::kernel::ffi::c_ulong) -> ::kernel::ffi::c_ulong;
    pub fn pci_sun4v_msg_setmsiq(devhandle: ::kernel::ffi::c_ulong, msinum: ::kernel::ffi::c_ulong, msiq: ::kernel::ffi::c_ulong) -> ::kernel::ffi::c_ulong;
    pub fn pci_sun4v_msg_getvalid(devhandle: ::kernel::ffi::c_ulong, msinum: ::kernel::ffi::c_ulong, valid: *mut ::kernel::ffi::c_ulong) -> ::kernel::ffi::c_ulong;
    pub fn pci_sun4v_msg_setvalid(devhandle: ::kernel::ffi::c_ulong, msinum: ::kernel::ffi::c_ulong, valid: ::kernel::ffi::c_ulong) -> ::kernel::ffi::c_ulong;

    /* Sun4v HV IOMMU v2 APIs */
    pub fn pci_sun4v_iotsb_conf(devhandle: ::kernel::ffi::c_ulong, ra: ::kernel::ffi::c_ulong, table_size: ::kernel::ffi::c_ulong, page_size: ::kernel::ffi::c_ulong, dvma_base: ::kernel::ffi::c_ulong, iotsb_num: *mut u64) -> ::kernel::ffi::c_ulong;
    pub fn pci_sun4v_iotsb_bind(devhandle: ::kernel::ffi::c_ulong, iotsb_num: ::kernel::ffi::c_ulong, pci_device: ::kernel::ffi::c_uint) -> ::kernel::ffi::c_ulong;
    pub fn pci_sun4v_iotsb_map(devhandle: ::kernel::ffi::c_ulong, iotsb_num: ::kernel::ffi::c_ulong, iotsb_index_iottes: ::kernel::ffi::c_ulong, io_attributes: ::kernel::ffi::c_ulong, io_page_list_pa: ::kernel::ffi::c_ulong, mapped: *mut ::kernel::ffi::c_long) -> ::kernel::ffi::c_ulong;
    pub fn pci_sun4v_iotsb_demap(devhandle: ::kernel::ffi::c_ulong, iotsb_num: ::kernel::ffi::c_ulong, iotsb_index: ::kernel::ffi::c_ulong, iottes: ::kernel::ffi::c_ulong, demapped: *mut ::kernel::ffi::c_ulong) -> ::kernel::ffi::c_ulong;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
