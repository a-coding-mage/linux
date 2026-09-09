/*
 * Intel 82860 Memory Controller kernel module
 * (C) 2005 Red Hat (http://www.redhat.com)
 * This file may be distributed under the terms of the
 * GNU General Public License.
 *
 * Written by Ben Woodard <woodard@redhat.com>
 * shamelessly copied from and based upon the edac_i82875 driver
 * by Thayne Harbaugh of Linux Networx. (http://lnxi.com)
 */

// C includes are supplied by the surrounding kernel bindings.

const EDAC_MOD_STR: &str = "i82860_edac";
const I82860_MCHCFG: u32 = 0x50;
const I82860_GBA: u32 = 0x60;
const I82860_GBA_MASK: u16 = 0x7FF;
const I82860_GBA_SHIFT: u32 = 24;
const I82860_ERRSTS: u32 = 0xC8;
const I82860_EAP: u32 = 0xE4;
const I82860_DERRCTL_STS: u32 = 0xE2;
const PCI_DEVICE_ID_INTEL_82860_0: u16 = 0x2531;

#[repr(C)]
enum i82860_chips { I82860 = 0 }

#[repr(C)]
struct i82860_dev_info { ctl_name: *const core::ffi::c_char }

#[repr(C)]
struct i82860_error_info { errsts: u16, eap: u32, derrsyn: u16, errsts2: u16 }

static I82860_DEVS: [i82860_dev_info; 1] = [i82860_dev_info {
    ctl_name: b"i82860\0".as_ptr() as *const core::ffi::c_char,
}];

static mut mci_pdev: *mut pci_dev = core::ptr::null_mut();
static mut i82860_pci: *mut edac_pci_ctl_info = core::ptr::null_mut();

unsafe fn i82860_get_error_info(mci: *mut mem_ctl_info, info: *mut i82860_error_info) {
    let pdev = to_pci_dev((*mci).pdev);
    pci_read_config_word(pdev, I82860_ERRSTS, &mut (*info).errsts);
    pci_read_config_dword(pdev, I82860_EAP, &mut (*info).eap);
    pci_read_config_word(pdev, I82860_DERRCTL_STS, &mut (*info).derrsyn);
    pci_read_config_word(pdev, I82860_ERRSTS, &mut (*info).errsts2);
    pci_write_bits16(pdev, I82860_ERRSTS, 0x0003, 0x0003);
    if (*info).errsts2 & 0x0003 == 0 { return; }
    if ((*info).errsts ^ (*info).errsts2) & 0x0003 != 0 {
        pci_read_config_dword(pdev, I82860_EAP, &mut (*info).eap);
        pci_read_config_word(pdev, I82860_DERRCTL_STS, &mut (*info).derrsyn);
    }
}

unsafe fn i82860_process_error_info(mci: *mut mem_ctl_info, info: *mut i82860_error_info, handle_errors: i32) -> i32 {
    if (*info).errsts2 & 0x0003 == 0 { return 0; }
    if handle_errors == 0 { return 1; }
    if ((*info).errsts ^ (*info).errsts2) & 0x0003 != 0 {
        edac_mc_handle_error(HW_EVENT_ERR_UNCORRECTED, mci, 1, 0, 0, 0, -1, -1, -1, cstr!("UE overwrote CE"), cstr!(""));
        (*info).errsts = (*info).errsts2;
    }
    (*info).eap >>= PAGE_SHIFT;
    let row = edac_mc_find_csrow_by_page(mci, (*info).eap);
    let dimm = (*(*mci).csrows.add(row as usize)).channels[0].dimm;
    if (*info).errsts & 0x0002 != 0 {
        edac_mc_handle_error(HW_EVENT_ERR_UNCORRECTED, mci, 1, (*info).eap, 0, 0, (*dimm).location[0], (*dimm).location[1], -1, cstr!("i82860 UE"), cstr!(""));
    } else {
        edac_mc_handle_error(HW_EVENT_ERR_CORRECTED, mci, 1, (*info).eap, 0, (*info).derrsyn, (*dimm).location[0], (*dimm).location[1], -1, cstr!("i82860 CE"), cstr!(""));
    }
    1
}

unsafe fn i82860_check(mci: *mut mem_ctl_info) {
    let mut info = core::mem::MaybeUninit::<i82860_error_info>::uninit();
    i82860_get_error_info(mci, info.as_mut_ptr());
    i82860_process_error_info(mci, info.as_mut_ptr(), 1);
}

unsafe fn i82860_init_csrows(mci: *mut mem_ctl_info, pdev: *mut pci_dev) {
    let mut last_cumul_size: u32 = 0;
    let mut mchcfg_ddim: u16 = 0;
    let mut value: u16 = 0;
    pci_read_config_word(pdev, I82860_MCHCFG, &mut mchcfg_ddim);
    mchcfg_ddim &= 0x180;
    for index in 0..(*mci).nr_csrows {
        let csrow = *(*mci).csrows.add(index as usize);
        let dimm = (*csrow).channels[0].dimm;
        pci_read_config_word(pdev, I82860_GBA + index as u32 * 2, &mut value);
        let cumul_size = ((value & I82860_GBA_MASK) as u32) << (I82860_GBA_SHIFT - PAGE_SHIFT);
        edac_dbg(3, cstr!("(%d) cumul_size 0x%x\n"), index, cumul_size);
        if cumul_size == last_cumul_size { continue; }
        (*csrow).first_page = last_cumul_size;
        (*csrow).last_page = cumul_size - 1;
        (*dimm).nr_pages = cumul_size - last_cumul_size;
        last_cumul_size = cumul_size;
        (*dimm).grain = 1 << 12;
        (*dimm).mtype = MEM_RMBS;
        (*dimm).dtype = DEV_UNKNOWN;
        (*dimm).edac_mode = if mchcfg_ddim != 0 { EDAC_SECDED } else { EDAC_NONE };
    }
}

unsafe fn i82860_probe1(pdev: *mut pci_dev, dev_idx: i32) -> i32 {
    let layers = [
        edac_mc_layer { type_: EDAC_MC_LAYER_CHANNEL, size: 2, is_virt_csrow: true },
        edac_mc_layer { type_: EDAC_MC_LAYER_SLOT, size: 8, is_virt_csrow: true },
    ];
    let mci = edac_mc_alloc(0, layers.len(), layers.as_ptr(), 0);
    if mci.is_null() { return -ENOMEM; }
    (*mci).pdev = &mut (*pdev).dev;
    (*mci).mtype_cap = MEM_FLAG_DDR;
    (*mci).edac_ctl_cap = EDAC_FLAG_NONE | EDAC_FLAG_SECDED;
    (*mci).edac_cap = EDAC_FLAG_SECDED;
    (*mci).mod_name = EDAC_MOD_STR.as_ptr() as *const _;
    (*mci).ctl_name = I82860_DEVS[dev_idx as usize].ctl_name;
    (*mci).dev_name = pci_name(pdev);
    (*mci).edac_check = Some(i82860_check);
    (*mci).ctl_page_to_phys = None;
    i82860_init_csrows(mci, pdev);
    let mut discard = core::mem::MaybeUninit::<i82860_error_info>::uninit();
    i82860_get_error_info(mci, discard.as_mut_ptr());
    if edac_mc_add_mc(mci) != 0 { edac_mc_free(mci); return -ENODEV; }
    i82860_pci = edac_pci_create_generic_ctl(&mut (*pdev).dev, EDAC_MOD_STR.as_ptr() as *const _);
    0
}

unsafe fn i82860_init_one(pdev: *mut pci_dev, ent: *const pci_device_id) -> i32 {
    edac_dbg(0, cstr!("\n"));
    i82860_printk!(KERN_INFO, cstr!("i82860 init one\n"));
    if pci_enable_device(pdev) < 0 { return -EIO; }
    let rc = i82860_probe1(pdev, (*ent).driver_data);
    if rc == 0 { mci_pdev = pci_dev_get(pdev); }
    rc
}

unsafe fn i82860_remove_one(pdev: *mut pci_dev) {
    edac_dbg(0, cstr!("\n"));
    if !i82860_pci.is_null() { edac_pci_release_generic_ctl(i82860_pci); }
    let mci = edac_mc_del_mc(&mut (*pdev).dev);
    if mci.is_null() { return; }
    edac_mc_free(mci);
}

static I82860_PCI_TBL: [pci_device_id; 2] = [
    pci_device_id { vendor: PCI_VENDOR_ID_INTEL, device: PCI_DEVICE_ID_INTEL_82860_0, driver_data: I82860 as i64 },
    pci_device_id::zero(),
];

static mut I82860_DRIVER: pci_driver = pci_driver {
    name: EDAC_MOD_STR.as_ptr() as *const _,
    probe: Some(i82860_init_one), remove: Some(i82860_remove_one), id_table: I82860_PCI_TBL.as_ptr(),
};

unsafe fn i82860_init() -> i32 {
    edac_dbg(3, cstr!("\n"));
    opstate_init();
    let mut pci_rc = pci_register_driver(&mut I82860_DRIVER);
    if pci_rc < 0 { pci_dev_put(mci_pdev); return pci_rc; }
    if mci_pdev.is_null() {
        mci_pdev = pci_get_device(PCI_VENDOR_ID_INTEL, PCI_DEVICE_ID_INTEL_82860_0, core::ptr::null_mut());
        if mci_pdev.is_null() { pci_rc = -ENODEV; pci_unregister_driver(&mut I82860_DRIVER); pci_dev_put(mci_pdev); return pci_rc; }
        pci_rc = i82860_init_one(mci_pdev, I82860_PCI_TBL.as_ptr());
        if pci_rc < 0 { pci_rc = -ENODEV; pci_unregister_driver(&mut I82860_DRIVER); pci_dev_put(mci_pdev); return pci_rc; }
    }
    0
}

unsafe fn i82860_exit() { edac_dbg(3, cstr!("\n")); pci_unregister_driver(&mut I82860_DRIVER); pci_dev_put(mci_pdev); }

// module_init(i82860_init); module_exit(i82860_exit);
// MODULE_LICENSE("GPL"); MODULE_AUTHOR("Red Hat Inc. (http://www.redhat.com) Ben Woodard <woodard@redhat.com>");
// MODULE_DESCRIPTION("ECC support for Intel 82860 memory hub controllers");
// module_param(edac_op_state, int, 0444);
// MODULE_PARM_DESC(edac_op_state, "EDAC Error Reporting state: 0=Poll,1=NMI");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
