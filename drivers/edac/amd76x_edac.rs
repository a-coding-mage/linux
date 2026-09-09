/*
 * AMD 76x Memory Controller kernel module
 * (C) 2003 Linux Networx (http://lnxi.com)
 * This file may be distributed under the terms of the
 * GNU General Public License.
 *
 * Written by Thayne Harbaugh
 * Based on work by Dan Hollis <goemon at anime dot net> and others.
 *	http://www.anime.net/~goemon/linux-ecc/
 *
 * $Id: edac_amd76x.c,v 1.4.2.5 2005/10/05 00:43:44 dsp_llnl Exp $
 */

// Translated dependencies: linux/module.h, linux/init.h, linux/pci.h,
// linux/pci_ids.h, linux/edac.h, and edac_module.h.

const EDAC_MOD_STR: &str = "amd76x_edac";
const AMD76X_NR_CSROWS: usize = 8;
const AMD76X_NR_DIMMS: usize = 4;
const AMD76X_ECC_MODE_STATUS: u32 = 0x48;
const AMD76X_DRAM_MODE_STATUS: u32 = 0x58;
const AMD76X_MEM_BASE_ADDR: u32 = 0xC0;

#[repr(C)]
struct amd76x_error_info {
    ecc_mode_status: u32,
}

#[repr(C)]
enum amd76x_chips {
    AMD761 = 0,
    AMD762,
}

#[repr(C)]
struct amd76x_dev_info {
    ctl_name: *const core::ffi::c_char,
}

static amd76x_devs: [amd76x_dev_info; 2] = [
    amd76x_dev_info { ctl_name: c"AMD761".as_ptr() },
    amd76x_dev_info { ctl_name: c"AMD762".as_ptr() },
];

static mut amd76x_pci: *mut edac_pci_ctl_info = core::ptr::null_mut();

unsafe fn amd76x_get_error_info(
    mci: *mut mem_ctl_info,
    info: *mut amd76x_error_info,
) {
    let pdev: *mut pci_dev = to_pci_dev((*mci).pdev);
    pci_read_config_dword(pdev, AMD76X_ECC_MODE_STATUS, &mut (*info).ecc_mode_status);

    if (*info).ecc_mode_status & BIT(8) != 0 {
        pci_write_bits32(pdev, AMD76X_ECC_MODE_STATUS, BIT(8) as u32, BIT(8) as u32);
    }
    if (*info).ecc_mode_status & BIT(9) != 0 {
        pci_write_bits32(pdev, AMD76X_ECC_MODE_STATUS, BIT(9) as u32, BIT(9) as u32);
    }
}

unsafe fn amd76x_process_error_info(
    mci: *mut mem_ctl_info,
    info: *mut amd76x_error_info,
    handle_errors: i32,
) -> i32 {
    let mut error_found = 0;
    let mut row: u32;

    if (*info).ecc_mode_status & BIT(8) != 0 {
        error_found = 1;
        if handle_errors != 0 {
            row = ((*info).ecc_mode_status >> 4) & 0xf;
            edac_mc_handle_error(HW_EVENT_ERR_UNCORRECTED, mci, 1,
                (*mci).csrows[row as usize].first_page, 0, 0, row, 0, -1,
                (*mci).ctl_name, c"".as_ptr());
        }
    }
    if (*info).ecc_mode_status & BIT(9) != 0 {
        error_found = 1;
        if handle_errors != 0 {
            row = (*info).ecc_mode_status & 0xf;
            edac_mc_handle_error(HW_EVENT_ERR_CORRECTED, mci, 1,
                (*mci).csrows[row as usize].first_page, 0, 0, row, 0, -1,
                (*mci).ctl_name, c"".as_ptr());
        }
    }
    error_found
}

unsafe fn amd76x_check(mci: *mut mem_ctl_info) {
    let mut info = amd76x_error_info { ecc_mode_status: 0 };
    amd76x_get_error_info(mci, &mut info);
    amd76x_process_error_info(mci, &mut info, 1);
}

unsafe fn amd76x_init_csrows(
    mci: *mut mem_ctl_info,
    pdev: *mut pci_dev,
    edac_mode: edac_type,
) {
    for index in 0..(*mci).nr_csrows as usize {
        let csrow = (*mci).csrows[index];
        let dimm = (*csrow).channels[0].dimm;
        let mut mba = 0u32;
        let mut dms = 0u32;
        pci_read_config_dword(pdev, AMD76X_MEM_BASE_ADDR + (index as u32 * 4), &mut mba);
        if mba & BIT(0) == 0 { continue; }
        let mba_base = mba & 0xff800000;
        let mba_mask = ((mba & 0xff80) << 16) | 0x7fffff;
        pci_read_config_dword(pdev, AMD76X_DRAM_MODE_STATUS, &mut dms);
        (*csrow).first_page = mba_base >> PAGE_SHIFT;
        (*dimm).nr_pages = (mba_mask + 1) >> PAGE_SHIFT;
        (*csrow).last_page = (*csrow).first_page + (*dimm).nr_pages - 1;
        (*csrow).page_mask = mba_mask >> PAGE_SHIFT;
        (*dimm).grain = (*dimm).nr_pages << PAGE_SHIFT;
        (*dimm).mtype = MEM_RDDR;
        (*dimm).dtype = if (dms >> index) & 1 != 0 { DEV_X4 } else { DEV_UNKNOWN };
        (*dimm).edac_mode = edac_mode;
    }
}

unsafe fn amd76x_probe1(pdev: *mut pci_dev, dev_idx: i32) -> i32 {
    let ems_modes = [EDAC_NONE, EDAC_EC, EDAC_SECDED, EDAC_SECDED];
    let mut ems = 0u32;
    pci_read_config_dword(pdev, AMD76X_ECC_MODE_STATUS, &mut ems);
    let ems_mode = ((ems >> 10) & 3) as usize;
    let mut layers = [edac_mc_layer { type_: EDAC_MC_LAYER_CHIP_SELECT, size: AMD76X_NR_CSROWS, is_virt_csrow: true },
        edac_mc_layer { type_: EDAC_MC_LAYER_CHANNEL, size: 1, is_virt_csrow: false }];
    let mci = edac_mc_alloc(0, layers.len(), layers.as_mut_ptr(), 0);
    if mci.is_null() { return -ENOMEM; }
    (*mci).pdev = &mut (*pdev).dev;
    (*mci).mtype_cap = MEM_FLAG_RDDR;
    (*mci).edac_ctl_cap = EDAC_FLAG_NONE | EDAC_FLAG_EC | EDAC_FLAG_SECDED;
    (*mci).edac_cap = if ems_mode != 0 { EDAC_FLAG_EC | EDAC_FLAG_SECDED } else { EDAC_FLAG_NONE };
    (*mci).mod_name = EDAC_MOD_STR.as_ptr() as *const _;
    (*mci).ctl_name = amd76x_devs[dev_idx as usize].ctl_name;
    (*mci).dev_name = pci_name(pdev);
    (*mci).edac_check = Some(amd76x_check);
    (*mci).ctl_page_to_phys = None;
    amd76x_init_csrows(mci, pdev, ems_modes[ems_mode]);
    let mut discard = amd76x_error_info { ecc_mode_status: 0 };
    amd76x_get_error_info(mci, &mut discard);
    if edac_mc_add_mc(mci) != 0 { edac_mc_free(mci); return -ENODEV; }
    amd76x_pci = edac_pci_create_generic_ctl(&mut (*pdev).dev, EDAC_MOD_STR.as_ptr() as *const _);
    0
}

unsafe fn amd76x_init_one(pdev: *mut pci_dev, ent: *const pci_device_id) -> i32 {
    amd76x_probe1(pdev, (*ent).driver_data as i32)
}

unsafe fn amd76x_remove_one(pdev: *mut pci_dev) {
    if !amd76x_pci.is_null() { edac_pci_release_generic_ctl(amd76x_pci); }
    let mci = edac_mc_del_mc(&mut (*pdev).dev);
    if !mci.is_null() { edac_mc_free(mci); }
}

static mut amd76x_pci_tbl: [pci_device_id; 3] = [
    pci_device_id { vendor: PCI_VENDOR_ID_AMD, device: FE_GATE_700C, driver_data: AMD762 as _ },
    pci_device_id { vendor: PCI_VENDOR_ID_AMD, device: FE_GATE_700E, driver_data: AMD761 as _ },
    pci_device_id::ZERO,
];

static mut amd76x_driver: pci_driver = pci_driver {
    name: EDAC_MOD_STR.as_ptr() as *const _,
    probe: Some(amd76x_init_one),
    remove: Some(amd76x_remove_one),
    id_table: amd76x_pci_tbl.as_ptr(),
};

unsafe fn amd76x_init() -> i32 {
    opstate_init();
    pci_register_driver(&mut amd76x_driver)
}

unsafe fn amd76x_exit() {
    pci_unregister_driver(&mut amd76x_driver);
}

// module_init(amd76x_init); module_exit(amd76x_exit);
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Linux Networx (http://lnxi.com) Thayne Harbaugh");
// MODULE_DESCRIPTION("MC support for AMD 76x memory controllers");
// module_param(edac_op_state, int, 0444);
// MODULE_PARM_DESC(edac_op_state, "EDAC Error Reporting state: 0=Poll,1=NMI");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
