// SPDX-License-Identifier: GPL-2.0
/* Copyright(c) 2016 - 2018 Intel Corporation. All rights reserved. */
// Linux kernel dependencies and build-time module macros are supplied by the
// surrounding translation unit.

unsafe fn __dax_pmem_probe(dev: *mut device) -> *mut dev_dax {
    let mut range: range;
    let mut rc: i32;
    let mut id: i32;
    let mut region_id: i32;
    let mut offset: resource_size_t;
    let mut pfn_sb: *mut nd_pfn_sb;
    let mut data: dev_dax_data;
    let mut nsio: *mut nd_namespace_io;
    let mut dax_region: *mut dax_region;
    let mut pgmap: dev_pagemap = core::mem::zeroed();
    let mut ndns: *mut nd_namespace_common;
    let nd_dax: *mut nd_dax = to_nd_dax(dev);
    let nd_pfn: *mut nd_pfn = &mut (*nd_dax).nd_pfn;
    let nd_region: *mut nd_region = to_nd_region((*dev).parent);

    ndns = nvdimm_namespace_common_probe(dev);
    if IS_ERR(ndns) {
        return ERR_CAST(ndns);
    }

    /* parse the 'pfn' info block via ->rw_bytes */
    rc = devm_namespace_enable(dev, ndns, nd_info_block_reserve());
    if rc != 0 {
        return ERR_PTR(rc);
    }
    rc = nvdimm_setup_pfn(nd_pfn, &mut pgmap);
    if rc != 0 {
        return ERR_PTR(rc);
    }
    devm_namespace_disable(dev, ndns);

    /* reserve the metadata area, device-dax will reserve the data */
    pfn_sb = (*nd_pfn).pfn_sb;
    offset = le64_to_cpu((*pfn_sb).dataoff);
    nsio = to_nd_namespace_io(&mut (*ndns).dev);
    if devm_request_mem_region(
        dev,
        (*nsio).res.start,
        offset,
        dev_name(&(*ndns).dev),
    ) == core::ptr::null_mut() {
        dev_warn(dev, "could not reserve metadata\n");
        return ERR_PTR(-EBUSY);
    }

    rc = sscanf(
        dev_name(&(*ndns).dev),
        "namespace%d.%d",
        &mut region_id,
        &mut id,
    );
    if rc != 2 {
        return ERR_PTR(-EINVAL);
    }

    /* adjust the dax_region range to the start of data */
    range = pgmap.range;
    range.start = range.start.wrapping_add(offset);
    dax_region = alloc_dax_region(
        dev,
        region_id,
        &mut range,
        (*nd_region).target_node,
        le32_to_cpu((*pfn_sb).align),
        IORESOURCE_DAX_STATIC,
    );
    if dax_region.is_null() {
        return ERR_PTR(-ENOMEM);
    }

    data = dev_dax_data {
        dax_region,
        id,
        pgmap: &mut pgmap,
        size: range_len(&range),
        memmap_on_memory: false,
    };

    devm_create_dev_dax(&mut data)
}

unsafe fn dax_pmem_probe(dev: *mut device) -> i32 {
    PTR_ERR_OR_ZERO(__dax_pmem_probe(dev))
}

static mut dax_pmem_driver: nd_device_driver = nd_device_driver {
    probe: Some(dax_pmem_probe),
    drv: driver {
        name: "dax_pmem",
    },
    type_: ND_DRIVER_DAX_PMEM,
};

unsafe fn dax_pmem_init() -> i32 {
    nd_driver_register(&mut dax_pmem_driver)
}

unsafe fn dax_pmem_exit() {
    driver_unregister(&mut dax_pmem_driver.drv);
}

// module_init(dax_pmem_init);
// module_exit(dax_pmem_exit);
// MODULE_DESCRIPTION("PMEM DAX: direct access to persistent memory");
// MODULE_LICENSE("GPL v2");
// MODULE_AUTHOR("Intel Corporation");
// MODULE_ALIAS_ND_DEVICE(ND_DEVICE_DAX_PMEM);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
