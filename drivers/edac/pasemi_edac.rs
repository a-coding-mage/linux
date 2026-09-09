// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2006-2007 PA Semi, Inc
 *
 * Author: Egor Martovetsky <egor@pasemi.com>
 * Maintained by: Olof Johansson <olof@lixom.net>
 *
 * Driver for the PWRficient onchip memory controllers
 */

// Linux kernel dependencies supplied by other translation units.

const MODULE_NAME: &str = "pasemi_edac";

const MCCFG_MCEN: u32 = 0x300;
const MCCFG_MCEN_MMC_EN: u32 = 0x00000001;
const MCCFG_ERRCOR: u32 = 0x388;
const MCCFG_ERRCOR_RNK_FAIL_DET_EN: u32 = 0x00000100;
const MCCFG_ERRCOR_ECC_GEN_EN: u32 = 0x00000010;
const MCCFG_ERRCOR_ECC_CRR_EN: u32 = 0x00000001;
const MCCFG_SCRUB: u32 = 0x384;
const MCCFG_SCRUB_RGLR_SCRB_EN: u32 = 0x00000001;
const MCDEBUG_ERRCTL1: u32 = 0x728;
const MCDEBUG_ERRCTL1_RFL_LOG_EN: u32 = 0x00080000;
const MCDEBUG_ERRCTL1_MBE_LOG_EN: u32 = 0x00040000;
const MCDEBUG_ERRCTL1_SBE_LOG_EN: u32 = 0x00020000;
const MCDEBUG_ERRSTA: u32 = 0x730;
const MCDEBUG_ERRSTA_RFL_STATUS: u32 = 0x00000004;
const MCDEBUG_ERRSTA_MBE_STATUS: u32 = 0x00000002;
const MCDEBUG_ERRSTA_SBE_STATUS: u32 = 0x00000001;
const MCDEBUG_ERRCNT1: u32 = 0x734;
const MCDEBUG_ERRCNT1_SBE_CNT_OVRFLO: u32 = 0x00000080;
const MCDEBUG_ERRLOG1A: u32 = 0x738;
const MCDEBUG_ERRLOG1A_MERR_TYPE_M: u32 = 0x30000000;
const MCDEBUG_ERRLOG1A_MERR_TYPE_NONE: u32 = 0x00000000;
const MCDEBUG_ERRLOG1A_MERR_TYPE_SBE: u32 = 0x10000000;
const MCDEBUG_ERRLOG1A_MERR_TYPE_MBE: u32 = 0x20000000;
const MCDEBUG_ERRLOG1A_MERR_TYPE_RFL: u32 = 0x30000000;
const MCDEBUG_ERRLOG1A_MERR_BA_M: u32 = 0x00700000;
const MCDEBUG_ERRLOG1A_MERR_BA_S: u32 = 20;
const MCDEBUG_ERRLOG1A_MERR_CS_M: u32 = 0x00070000;
const MCDEBUG_ERRLOG1A_MERR_CS_S: u32 = 16;
const MCDEBUG_ERRLOG1A_SYNDROME_M: u32 = 0x0000ffff;
const MCDRAM_RANKCFG: u32 = 0x114;
const MCDRAM_RANKCFG_EN: u32 = 0x00000001;
const MCDRAM_RANKCFG_TYPE_SIZE_M: u32 = 0x000001c0;
const MCDRAM_RANKCFG_TYPE_SIZE_S: u32 = 6;

const PASEMI_EDAC_NR_CSROWS: usize = 8;
const PASEMI_EDAC_NR_CHANS: usize = 1;
const PASEMI_EDAC_ERROR_GRAIN: u64 = 64;

static mut last_page_in_mmc: i32 = 0;
static mut system_mmc_id: i32 = 0;

unsafe fn pasemi_edac_get_error_info(mci: *mut mem_ctl_info) -> u32 {
    let pdev = to_pci_dev((*mci).pdev);
    let mut tmp: u32 = 0;
    pci_read_config_dword(pdev, MCDEBUG_ERRSTA, &mut tmp);
    tmp &= MCDEBUG_ERRSTA_RFL_STATUS | MCDEBUG_ERRSTA_MBE_STATUS | MCDEBUG_ERRSTA_SBE_STATUS;
    if tmp != 0 {
        if tmp & MCDEBUG_ERRSTA_SBE_STATUS != 0 {
            pci_write_config_dword(pdev, MCDEBUG_ERRCNT1, MCDEBUG_ERRCNT1_SBE_CNT_OVRFLO);
        }
        pci_write_config_dword(pdev, MCDEBUG_ERRSTA, tmp);
    }
    tmp
}

unsafe fn pasemi_edac_process_error_info(mci: *mut mem_ctl_info, errsta: u32) {
    let pdev = to_pci_dev((*mci).pdev);
    let mut errlog1a: u32 = 0;
    pci_read_config_dword(pdev, MCDEBUG_ERRLOG1A, &mut errlog1a);
    let cs = ((errlog1a & MCDEBUG_ERRLOG1A_MERR_CS_M) >> MCDEBUG_ERRLOG1A_MERR_CS_S) as usize;
    if errsta & (MCDEBUG_ERRSTA_MBE_STATUS | MCDEBUG_ERRSTA_RFL_STATUS) != 0 {
        edac_mc_handle_error(HW_EVENT_ERR_UNCORRECTED, mci, 1, (*mci).csrows[cs].first_page,
                             0, 0, cs as i32, 0, -1, (*mci).ctl_name, "");
    }
    if errsta & MCDEBUG_ERRSTA_SBE_STATUS != 0 {
        edac_mc_handle_error(HW_EVENT_ERR_CORRECTED, mci, 1, (*mci).csrows[cs].first_page,
                             0, 0, cs as i32, 0, -1, (*mci).ctl_name, "");
    }
}

unsafe fn pasemi_edac_check(mci: *mut mem_ctl_info) {
    let errsta = pasemi_edac_get_error_info(mci);
    if errsta != 0 { pasemi_edac_process_error_info(mci, errsta); }
}

unsafe fn pasemi_edac_init_csrows(mci: *mut mem_ctl_info, pdev: *mut pci_dev,
                                  edac_mode: edac_type) -> i32 {
    for index in 0..(*mci).nr_csrows as usize {
        let csrow = (*mci).csrows[index];
        let dimm = (*csrow).channels[0].dimm;
        let mut rankcfg: u32 = 0;
        pci_read_config_dword(pdev, MCDRAM_RANKCFG + (index as u32 * 12), &mut rankcfg);
        if rankcfg & MCDRAM_RANKCFG_EN == 0 { continue; }
        (*dimm).nr_pages = match (rankcfg & MCDRAM_RANKCFG_TYPE_SIZE_M) >> MCDRAM_RANKCFG_TYPE_SIZE_S {
            0 => 128 << (20 - PAGE_SHIFT), 1 => 256 << (20 - PAGE_SHIFT),
            2 | 3 => 512 << (20 - PAGE_SHIFT), 4 => 1024 << (20 - PAGE_SHIFT),
            5 => 2048 << (20 - PAGE_SHIFT),
            _ => { edac_mc_printk(mci, KERN_ERR, "Unrecognized Rank Config. rankcfg=%u\n", rankcfg); return -EINVAL; }
        };
        (*csrow).first_page = last_page_in_mmc;
        (*csrow).last_page = (*csrow).first_page + (*dimm).nr_pages - 1;
        last_page_in_mmc += (*dimm).nr_pages;
        (*csrow).page_mask = 0;
        (*dimm).grain = PASEMI_EDAC_ERROR_GRAIN;
        (*dimm).mtype = MEM_DDR; (*dimm).dtype = DEV_UNKNOWN;
        (*dimm).edac_mode = edac_mode;
    }
    0
}

// The remaining PCI driver/module declarations retain the kernel ABI and are
// expressed using the corresponding external kernel types and functions.
unsafe fn pasemi_edac_probe(pdev: *mut pci_dev, _ent: *const pci_device_id) -> i32 {
    let mut mci: *mut mem_ctl_info = core::ptr::null_mut();
    let mut mcen = 0; pci_read_config_dword(pdev, MCCFG_MCEN, &mut mcen);
    if mcen & MCCFG_MCEN_MMC_EN == 0 { return -ENODEV; }
    let mut errctl1 = 0; pci_read_config_dword(pdev, MCDEBUG_ERRCTL1, &mut errctl1);
    errctl1 |= MCDEBUG_ERRCTL1_SBE_LOG_EN | MCDEBUG_ERRCTL1_MBE_LOG_EN | MCDEBUG_ERRCTL1_RFL_LOG_EN;
    pci_write_config_dword(pdev, MCDEBUG_ERRCTL1, errctl1);
    let layers = [edac_mc_layer { type_: EDAC_MC_LAYER_CHIP_SELECT, size: PASEMI_EDAC_NR_CSROWS, is_virt_csrow: true },
                  edac_mc_layer { type_: EDAC_MC_LAYER_CHANNEL, size: PASEMI_EDAC_NR_CHANS, is_virt_csrow: false }];
    mci = edac_mc_alloc(system_mmc_id as usize, layers.len(), layers.as_ptr(), 0);
    if mci.is_null() { return -ENOMEM; }
    let mut errcor = 0; pci_read_config_dword(pdev, MCCFG_ERRCOR, &mut errcor);
    errcor |= MCCFG_ERRCOR_RNK_FAIL_DET_EN | MCCFG_ERRCOR_ECC_GEN_EN | MCCFG_ERRCOR_ECC_CRR_EN;
    (*mci).pdev = &mut (*pdev).dev;
    (*mci).mtype_cap = MEM_FLAG_DDR | MEM_FLAG_RDDR;
    (*mci).edac_ctl_cap = EDAC_FLAG_NONE | EDAC_FLAG_EC | EDAC_FLAG_SECDED;
    (*mci).edac_cap = if errcor & MCCFG_ERRCOR_ECC_GEN_EN != 0 { if errcor & MCCFG_ERRCOR_ECC_CRR_EN != 0 { EDAC_FLAG_EC | EDAC_FLAG_SECDED } else { EDAC_FLAG_EC } } else { EDAC_FLAG_NONE };
    (*mci).mod_name = MODULE_NAME; (*mci).dev_name = pci_name(pdev); (*mci).ctl_name = "pasemi,pwrficient-mc";
    (*mci).edac_check = Some(pasemi_edac_check); (*mci).ctl_page_to_phys = None;
    let mut scrub = 0; pci_read_config_dword(pdev, MCCFG_SCRUB, &mut scrub);
    (*mci).scrub_cap = SCRUB_FLAG_HW_PROG | SCRUB_FLAG_HW_SRC;
    (*mci).scrub_mode = if errcor & MCCFG_ERRCOR_ECC_CRR_EN != 0 { SCRUB_FLAG_HW_SRC } else { 0 } | if scrub & MCCFG_SCRUB_RGLR_SCRB_EN != 0 { SCRUB_FLAG_HW_PROG } else { 0 };
    if pasemi_edac_init_csrows(mci, pdev, if (*mci).edac_cap & EDAC_FLAG_SECDED != 0 { EDAC_SECDED } else if (*mci).edac_cap & EDAC_FLAG_EC != 0 { EDAC_EC } else { EDAC_NONE }) != 0 { edac_mc_free(mci); return -ENODEV; }
    pasemi_edac_get_error_info(mci); if edac_mc_add_mc(mci) != 0 { edac_mc_free(mci); return -ENODEV; } 0
}

unsafe fn pasemi_edac_remove(pdev: *mut pci_dev) { let mci = edac_mc_del_mc(&mut (*pdev).dev); if !mci.is_null() { edac_mc_free(mci); } }

static pasemi_edac_pci_tbl: [pci_device_id; 2] = [PCI_DEVICE(PCI_VENDOR_ID_PASEMI, 0xa00a), pci_device_id::default()];
static mut pasemi_edac_driver: pci_driver = pci_driver { name: MODULE_NAME, probe: Some(pasemi_edac_probe), remove: Some(pasemi_edac_remove), id_table: pasemi_edac_pci_tbl.as_ptr() };

unsafe fn pasemi_edac_init() -> i32 { opstate_init(); pci_register_driver(&mut pasemi_edac_driver) }
unsafe fn pasemi_edac_exit() { pci_unregister_driver(&mut pasemi_edac_driver); }

// module_init(pasemi_edac_init); module_exit(pasemi_edac_exit);
// MODULE_LICENSE("GPL"); MODULE_AUTHOR("Egor Martovetsky <egor@pasemi.com>");
// MODULE_DESCRIPTION("MC support for PA Semi PWRficient memory controller");
// module_param(edac_op_state, int, 0444);
// MODULE_PARM_DESC(edac_op_state, "EDAC Error Reporting state: 0=Poll,1=NMI");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
