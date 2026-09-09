/* Intel e7xxx Memory Controller kernel module -- Rust translation. */

const EDAC_MOD_STR: &str = "e7xxx_edac";
const PCI_DEVICE_ID_INTEL_7205_0: u16 = 0x255d;
const PCI_DEVICE_ID_INTEL_7205_1_ERR: u16 = 0x2551;
const PCI_DEVICE_ID_INTEL_7500_0: u16 = 0x2540;
const PCI_DEVICE_ID_INTEL_7500_1_ERR: u16 = 0x2541;
const PCI_DEVICE_ID_INTEL_7501_0: u16 = 0x254c;
const PCI_DEVICE_ID_INTEL_7501_1_ERR: u16 = 0x2541;
const PCI_DEVICE_ID_INTEL_7505_0: u16 = 0x2550;
const PCI_DEVICE_ID_INTEL_7505_1_ERR: u16 = 0x2551;
const E7XXX_NR_CSROWS: usize = 8;
const E7XXX_NR_DIMMS: usize = 8;
const E7XXX_DRB: u16 = 0x60;
const E7XXX_DRA: u16 = 0x70;
const E7XXX_DRC: u16 = 0x7c;
const E7XXX_TOLM: u16 = 0xc4;
const E7XXX_REMAPBASE: u16 = 0xc6;
const E7XXX_REMAPLIMIT: u16 = 0xc8;
const E7XXX_DRAM_FERR: u16 = 0x80;
const E7XXX_DRAM_NERR: u16 = 0x82;
const E7XXX_DRAM_CELOG_ADD: u16 = 0xa0;
const E7XXX_DRAM_UELOG_ADD: u16 = 0xb0;
const E7XXX_DRAM_CELOG_SYNDROME: u16 = 0xd0;

#[repr(C)]
pub enum E7xxxChips { E7500 = 0, E7501, E7505, E7205 }

#[repr(C)]
pub struct E7xxxPvt {
    pub bridge_ck: *mut pci_dev,
    pub tolm: u32,
    pub remapbase: u32,
    pub remaplimit: u32,
    pub dev_info: *const E7xxxDevInfo,
}
#[repr(C)]
pub struct E7xxxDevInfo { pub err_dev: u16, pub ctl_name: *const i8 }
#[repr(C)]
pub struct E7xxxErrorInfo {
    pub dram_ferr: u8, pub dram_nerr: u8, pub dram_celog_add: u32,
    pub dram_celog_syndrome: u16, pub dram_uelog_add: u32,
}

static mut e7xxx_pci: *mut edac_pci_ctl_info = core::ptr::null_mut();
static e7xxx_devs: [E7xxxDevInfo; 4] = [
    E7xxxDevInfo { err_dev: PCI_DEVICE_ID_INTEL_7500_1_ERR, ctl_name: b"E7500\0".as_ptr() as *const i8 },
    E7xxxDevInfo { err_dev: PCI_DEVICE_ID_INTEL_7501_1_ERR, ctl_name: b"E7501\0".as_ptr() as *const i8 },
    E7xxxDevInfo { err_dev: PCI_DEVICE_ID_INTEL_7505_1_ERR, ctl_name: b"E7505\0".as_ptr() as *const i8 },
    E7xxxDevInfo { err_dev: PCI_DEVICE_ID_INTEL_7205_1_ERR, ctl_name: b"E7205\0".as_ptr() as *const i8 },
];

unsafe fn e7xxx_find_channel(syndrome: u16) -> i32 {
    edac_dbg(3, b"\n\0".as_ptr());
    if syndrome & 0xff00 == 0 { return 0; }
    if syndrome & 0x00ff == 0 { return 1; }
    if syndrome & 0xf000 == 0 || syndrome & 0x0f00 == 0 { return 0; }
    1
}

unsafe fn ctl_page_to_phys(mci: *mut mem_ctl_info, page: usize) -> usize {
    let pvt = (*mci).pvt_info as *mut E7xxxPvt;
    if page < (*pvt).tolm as usize || (page >= 0x100000 && page < (*pvt).remapbase as usize) { return page; }
    let remap = (page - (*pvt).tolm as usize) + (*pvt).remapbase as usize;
    if remap < (*pvt).remaplimit as usize { return remap; }
    e7xxx_printk(KERN_ERR, b"Invalid page %lx - out of range\n\0".as_ptr(), page);
    (*pvt).tolm as usize - 1
}

unsafe fn process_ce(mci: *mut mem_ctl_info, info: *mut E7xxxErrorInfo) {
    let page = ((*info).dram_celog_add >> 6) as usize;
    let syndrome = (*info).dram_celog_syndrome;
    let row = edac_mc_find_csrow_by_page(mci, page);
    let channel = e7xxx_find_channel(syndrome);
    edac_mc_handle_error(HW_EVENT_ERR_CORRECTED, mci, 1, page, 0, syndrome, row, channel, -1, b"e7xxx CE\0".as_ptr(), b"\0".as_ptr());
}
unsafe fn process_ce_no_info(mci: *mut mem_ctl_info) { edac_mc_handle_error(HW_EVENT_ERR_CORRECTED, mci, 1, 0, 0, 0, -1, -1, -1, b"e7xxx CE log register overflow\0".as_ptr(), b"\0".as_ptr()); }
unsafe fn process_ue(mci: *mut mem_ctl_info, info: *mut E7xxxErrorInfo) {
    let page = ((*info).dram_uelog_add >> 6) as usize;
    let row = edac_mc_find_csrow_by_page(mci, page);
    edac_mc_handle_error(HW_EVENT_ERR_UNCORRECTED, mci, 1, page, 0, 0, row, -1, -1, b"e7xxx UE\0".as_ptr(), b"\0".as_ptr());
}
unsafe fn process_ue_no_info(mci: *mut mem_ctl_info) { edac_mc_handle_error(HW_EVENT_ERR_UNCORRECTED, mci, 1, 0, 0, 0, -1, -1, -1, b"e7xxx UE log register overflow\0".as_ptr(), b"\0".as_ptr()); }

unsafe fn e7xxx_get_error_info(mci: *mut mem_ctl_info, info: *mut E7xxxErrorInfo) {
    let pvt = (*mci).pvt_info as *mut E7xxxPvt;
    pci_read_config_byte((*pvt).bridge_ck, E7XXX_DRAM_FERR, &mut (*info).dram_ferr);
    pci_read_config_byte((*pvt).bridge_ck, E7XXX_DRAM_NERR, &mut (*info).dram_nerr);
    if (*info).dram_ferr & 1 != 0 || (*info).dram_nerr & 1 != 0 {
        pci_read_config_dword((*pvt).bridge_ck, E7XXX_DRAM_CELOG_ADD, &mut (*info).dram_celog_add);
        pci_read_config_word((*pvt).bridge_ck, E7XXX_DRAM_CELOG_SYNDROME, &mut (*info).dram_celog_syndrome);
    }
    if (*info).dram_ferr & 2 != 0 || (*info).dram_nerr & 2 != 0 { pci_read_config_dword((*pvt).bridge_ck, E7XXX_DRAM_UELOG_ADD, &mut (*info).dram_uelog_add); }
    if (*info).dram_ferr & 3 != 0 { pci_write_bits8((*pvt).bridge_ck, E7XXX_DRAM_FERR, 3, 3); }
    if (*info).dram_nerr & 3 != 0 { pci_write_bits8((*pvt).bridge_ck, E7XXX_DRAM_NERR, 3, 3); }
}

unsafe fn e7xxx_process_error_info(mci: *mut mem_ctl_info, info: *mut E7xxxErrorInfo, handle_errors: i32) -> i32 {
    let mut found = 0;
    if (*info).dram_ferr & 1 != 0 { found = 1; if handle_errors != 0 { process_ce(mci, info); } }
    if (*info).dram_ferr & 2 != 0 { found = 1; if handle_errors != 0 { process_ue(mci, info); } }
    if (*info).dram_nerr & 1 != 0 { found = 1; if handle_errors != 0 { if (*info).dram_ferr & 1 != 0 { process_ce_no_info(mci); } else { process_ce(mci, info); } } }
    if (*info).dram_nerr & 2 != 0 { found = 1; if handle_errors != 0 { if (*info).dram_ferr & 2 != 0 { process_ue_no_info(mci); } else { process_ue(mci, info); } } }
    found
}
unsafe fn e7xxx_check(mci: *mut mem_ctl_info) { let mut info = core::mem::MaybeUninit::<E7xxxErrorInfo>::uninit(); e7xxx_get_error_info(mci, info.as_mut_ptr()); e7xxx_process_error_info(mci, info.as_mut_ptr(), 1); }
unsafe fn dual_channel_active(drc: u32, dev_idx: i32) -> i32 { if dev_idx == E7xxxChips::E7501 as i32 { ((drc >> 22) & 1) as i32 } else { 1 } }
unsafe fn drb_granularity(drc: u32, dev_idx: i32) -> i32 { if dev_idx == E7xxxChips::E7501 as i32 { ((drc >> 18) & 3) as i32 } else { 1 } }

unsafe fn e7xxx_init_csrows(mci: *mut mem_ctl_info, pdev: *mut pci_dev, dev_idx: i32, drc: u32) {
    let mut dra = 0u32; pci_read_config_dword(pdev, E7XXX_DRA, &mut dra);
    let chan = dual_channel_active(drc, dev_idx); let gran = drb_granularity(drc, dev_idx); let ddim = ((drc >> 20) & 3) != 0;
    let mut last = 0usize;
    for index in 0..(*mci).nr_csrows as usize {
        let mem_dev = ((dra >> (index * 4 + 3)) & 1) != 0; let csrow = *(*mci).csrows.add(index); let mut value = 0u8;
        pci_read_config_byte(pdev, E7XXX_DRB + index as u16, &mut value);
        let cumul = (value as usize) << (25 + gran - PAGE_SHIFT); if cumul == last { continue; }
        (*csrow).first_page = last; (*csrow).last_page = cumul - 1; let nr_pages = cumul - last; last = cumul;
        let mode = if !ddim { EDAC_NONE } else if chan != 0 && mem_dev { (*mci).edac_cap |= EDAC_FLAG_S4ECD4ED; EDAC_S4ECD4ED } else { (*mci).edac_cap |= EDAC_FLAG_SECDED; EDAC_SECDED };
        for j in 0..=(chan as usize) { let dimm = (*(*csrow).channels.add(j)).dimm; (*dimm).nr_pages = nr_pages / (chan as usize + 1); (*dimm).grain = 1 << 12; (*dimm).mtype = MEM_RDDR; (*dimm).dtype = if mem_dev { DEV_X4 } else { DEV_X8 }; (*dimm).edac_mode = mode; }
    }
}
unsafe fn e7xxx_probe1(pdev: *mut pci_dev, dev_idx: i32) -> i32 {
    let mut drc=0u32; pci_read_config_dword(pdev,E7XXX_DRC,&mut drc); let chan=dual_channel_active(drc,dev_idx);
    let mut layers=[edac_mc_layer{type_:EDAC_MC_LAYER_CHIP_SELECT,size:E7XXX_NR_CSROWS,is_virt_csrow:true},edac_mc_layer{type_:EDAC_MC_LAYER_CHANNEL,size:(chan+1) as usize,is_virt_csrow:false}];
    let mci=edac_mc_alloc(0,2,layers.as_mut_ptr(),core::mem::size_of::<E7xxxPvt>()); if mci.is_null(){return -12;}
    (*mci).mtype_cap=MEM_FLAG_RDDR; (*mci).edac_ctl_cap=EDAC_FLAG_NONE|EDAC_FLAG_SECDED|EDAC_FLAG_S4ECD4ED; (*mci).mod_name=EDAC_MOD_STR.as_ptr() as *const i8; (*mci).pdev=&mut (*pdev).dev;
    let pvt=(*mci).pvt_info as *mut E7xxxPvt; (*pvt).dev_info=&e7xxx_devs[dev_idx as usize]; (*pvt).bridge_ck=pci_get_device(PCI_VENDOR_ID_INTEL,(*pvt).dev_info).cast(); if (*pvt).bridge_ck.is_null(){edac_mc_free(mci);return -19;}
    e7xxx_init_csrows(mci,pdev,dev_idx,drc); let mut v=0u16; pci_read_config_word(pdev,E7XXX_TOLM,&mut v);(*pvt).tolm=(v as u32)<<4; pci_read_config_word(pdev,E7XXX_REMAPBASE,&mut v);(*pvt).remapbase=(v as u32)<<14; pci_read_config_word(pdev,E7XXX_REMAPLIMIT,&mut v);(*pvt).remaplimit=(v as u32)<<14; let mut discard=core::mem::MaybeUninit::uninit();e7xxx_get_error_info(mci,discard.as_mut_ptr()); if edac_mc_add_mc(mci)!=0{pci_dev_put((*pvt).bridge_ck);edac_mc_free(mci);return -19;} 0
}
unsafe fn e7xxx_init_one(pdev:*mut pci_dev,ent:*const pci_device_id)->i32{if pci_enable_device(pdev)!=0{-5}else{e7xxx_probe1(pdev,(*ent).driver_data)}}
unsafe fn e7xxx_remove_one(pdev:*mut pci_dev){if !e7xxx_pci.is_null(){edac_pci_release_generic_ctl(e7xxx_pci);}let mci=edac_mc_del_mc(&mut (*pdev).dev);if !mci.is_null(){let pvt=(*mci).pvt_info as *mut E7xxxPvt;pci_dev_put((*pvt).bridge_ck);edac_mc_free(mci);}}

#[repr(C)] struct pci_device_id { pub driver_data: i32 }
#[repr(C)] struct pci_driver { pub name:*const i8, pub probe:unsafe fn(*mut pci_dev,*const pci_device_id)->i32, pub remove:unsafe fn(*mut pci_dev) }
static e7xxx_pci_tbl:[pci_device_id;5]=[pci_device_id{driver_data:E7205 as i32},pci_device_id{driver_data:E7500 as i32},pci_device_id{driver_data:E7501 as i32},pci_device_id{driver_data:E7505 as i32},pci_device_id{driver_data:0}];
static mut e7xxx_driver:pci_driver=pci_driver{name:b"e7xxx_edac\0".as_ptr() as *const i8,probe:e7xxx_init_one,remove:e7xxx_remove_one};
unsafe fn e7xxx_init()->i32{opstate_init();pci_register_driver(&mut e7xxx_driver)}
unsafe fn e7xxx_exit(){pci_unregister_driver(&mut e7xxx_driver);}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
