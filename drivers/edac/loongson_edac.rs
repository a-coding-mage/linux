// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2024 Loongson Technology Corporation Limited.
 */

// Linux kernel dependencies supplied by other files.

const ECC_CS_COUNT_REG: usize = 0x18;

#[repr(C)]
struct loongson_edac_pvt {
    ecc_base: *mut core::ffi::c_void,

    /*
     * The ECC register in this controller records the number of errors
     * encountered since reset and cannot be zeroed so in order to be able
     * to report the error count at each check, this records the previous
     * register state.
     */
    last_ce_count: i32,
}

unsafe fn read_ecc(mci: *mut mem_ctl_info) -> i32 {
    let pvt = (*mci).pvt_info as *mut loongson_edac_pvt;
    let ecc: u64 = readq((*pvt).ecc_base.cast::<u8>().add(ECC_CS_COUNT_REG));
    /* cs0 -- cs3 */
    let mut cs = (ecc & 0xff) as i32;
    cs += ((ecc >> 8) & 0xff) as i32;
    cs += ((ecc >> 16) & 0xff) as i32;
    cs += ((ecc >> 24) & 0xff) as i32;

    cs
}

unsafe extern "C" fn edac_check(mci: *mut mem_ctl_info) {
    let pvt = (*mci).pvt_info as *mut loongson_edac_pvt;
    let new = read_ecc(mci);
    let add = new - (*pvt).last_ce_count;
    (*pvt).last_ce_count = new;
    if add <= 0 {
        return;
    }

    edac_mc_handle_error(
        HW_EVENT_ERR_CORRECTED,
        mci,
        add,
        0,
        0,
        0,
        0,
        0,
        -1,
        c"error".as_ptr(),
        c"".as_ptr(),
    );
}

unsafe fn dimm_config_init(mci: *mut mem_ctl_info) {
    let mut size: u32;
    let npages: u32;

    /* size not used */
    size = u32::MAX;
    npages = MiB_TO_PAGES(size);

    let dimm = edac_get_dimm(mci, 0, 0, 0);
    (*dimm).nr_pages = npages;
    snprintf(
        (*dimm).label.as_mut_ptr(),
        core::mem::size_of_val(&(*dimm).label),
        c"MC#%uChannel#%u_DIMM#%u".as_ptr(),
        (*mci).mc_idx,
        0,
        0,
    );
    (*dimm).grain = 8;
}

unsafe fn pvt_init(mci: *mut mem_ctl_info, vbase: *mut core::ffi::c_void) {
    let pvt = (*mci).pvt_info as *mut loongson_edac_pvt;

    (*pvt).ecc_base = vbase;
    (*pvt).last_ce_count = read_ecc(mci);
}

unsafe extern "C" fn edac_probe(pdev: *mut platform_device) -> i32 {
    let mut layers: [edac_mc_layer; 2] = core::mem::zeroed();
    let mci: *mut mem_ctl_info;
    let vbase: *mut core::ffi::c_void;
    let ret: i32;

    vbase = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR(vbase) {
        return PTR_ERR(vbase);
    }

    layers[0].type_ = EDAC_MC_LAYER_CHANNEL;
    layers[0].size = 1;
    layers[0].is_virt_csrow = false;
    layers[1].type_ = EDAC_MC_LAYER_SLOT;
    layers[1].size = 1;
    layers[1].is_virt_csrow = true;
    mci = edac_mc_alloc(
        0,
        layers.len(),
        layers.as_mut_ptr(),
        core::mem::size_of::<loongson_edac_pvt>(),
    );
    if mci.is_null() {
        return -ENOMEM;
    }

    (*mci).mc_idx = edac_device_alloc_index();
    (*mci).mtype_cap = MEM_FLAG_RDDR4;
    (*mci).edac_ctl_cap = EDAC_FLAG_NONE;
    (*mci).edac_cap = EDAC_FLAG_NONE;
    (*mci).mod_name = c"loongson_edac.c".as_ptr();
    (*mci).ctl_name = c"loongson_edac_ctl".as_ptr();
    (*mci).dev_name = c"loongson_edac_dev".as_ptr();
    (*mci).ctl_page_to_phys = None;
    (*mci).pdev = &mut (*pdev).dev;
    (*mci).error_desc.grain = 8;
    (*mci).edac_check = Some(edac_check);

    pvt_init(mci, vbase);
    dimm_config_init(mci);

    ret = edac_mc_add_mc(mci);
    if ret != 0 {
        edac_dbg(0, c"MC: failed edac_mc_add_mc()\n".as_ptr());
        edac_mc_free(mci);
        return ret;
    }
    edac_op_state = EDAC_OPSTATE_POLL;

    0
}

unsafe extern "C" fn edac_remove(pdev: *mut platform_device) {
    let mci = edac_mc_del_mc(&mut (*pdev).dev);

    if !mci.is_null() {
        edac_mc_free(mci);
    }
}

#[repr(C)]
struct acpi_device_id {
    id: *const core::ffi::c_char,
    driver_data: usize,
}

static loongson_edac_acpi_match: [acpi_device_id; 2] = [
    acpi_device_id { id: c"LOON0010".as_ptr(), driver_data: 0 },
    acpi_device_id { id: core::ptr::null(), driver_data: 0 },
];

static mut loongson_edac_driver: platform_driver = platform_driver {
    probe: Some(edac_probe),
    remove: Some(edac_remove),
    driver: device_driver {
        name: c"loongson-mc-edac".as_ptr(),
        acpi_match_table: loongson_edac_acpi_match.as_ptr(),
    },
};

// MODULE_DEVICE_TABLE(acpi, loongson_edac_acpi_match);
// module_platform_driver(loongson_edac_driver);
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Zhao Qunqin <zhaoqunqin@loongson.cn>");
// MODULE_DESCRIPTION("EDAC driver for loongson memory controller");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
