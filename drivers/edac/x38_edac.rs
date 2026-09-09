/*
 * Intel X38 Memory Controller kernel module
 * Copyright (C) 2008 Cluster Computing, Inc.
 *
 * This file may be distributed under the terms of the
 * GNU General Public License.
 *
 * This file is based on i3200_edac.c
 */

/* Linux kernel dependencies: module, init, pci, pci_ids, edac,
 * io-64-nonatomic-lo-hi, and edac_module.h. */

const EDAC_MOD_STR: &str = "x38_edac";

const PCI_DEVICE_ID_INTEL_X38_HB: u32 = 0x29e0;

const X38_RANKS: usize = 8;
const X38_RANKS_PER_CHANNEL: usize = 4;
const X38_CHANNELS: usize = 2;

/* Intel X38 register addresses - device 0 function 0 - DRAM Controller */
const X38_MCHBAR_LOW: usize = 0x48;
const X38_MCHBAR_HIGH: usize = 0x4c;
const X38_MCHBAR_MASK: u64 = 0xfffffc000;
const X38_MMR_WINDOW_SIZE: usize = 16384;
const X38_TOM: usize = 0xa0;
const X38_TOM_MASK: u16 = 0x3ff;
const X38_TOM_SHIFT: usize = 26;
const X38_ERRSTS: usize = 0xc8;
const X38_ERRSTS_UE: u16 = 0x0002;
const X38_ERRSTS_CE: u16 = 0x0001;
const X38_ERRSTS_BITS: u16 = X38_ERRSTS_UE | X38_ERRSTS_CE;

/* Intel MMIO register space - device 0 function 0 - MMR space */
const X38_C0DRB: usize = 0x200;
const X38_C1DRB: usize = 0x600;
const X38_DRB_MASK: u16 = 0x3ff;
const X38_DRB_SHIFT: usize = 26;
const X38_C0ECCERRLOG: usize = 0x280;
const X38_C1ECCERRLOG: usize = 0x680;
const X38_ECCERRLOG_CE: u64 = 0x1;
const X38_ECCERRLOG_UE: u64 = 0x2;
const X38_ECCERRLOG_RANK_BITS: u64 = 0x18000000;
const X38_ECCERRLOG_SYNDROME_BITS: u64 = 0xff0000;
const X38_CAPID0: usize = 0xe0;

static mut x38_channel_num: i32 = 0;

unsafe fn how_many_channel(pdev: *mut pci_dev) -> i32 {
    let mut capid0_8b: u8 = 0;
    pci_read_config_byte(pdev, X38_CAPID0 + 8, &mut capid0_8b);
    if capid0_8b & 0x20 != 0 {
        edac_dbg(0, "In single channel mode\n");
        x38_channel_num = 1;
    } else {
        edac_dbg(0, "In dual channel mode\n");
        x38_channel_num = 2;
    }
    x38_channel_num
}

unsafe fn eccerrlog_syndrome(log: u64) -> usize {
    ((log & X38_ECCERRLOG_SYNDROME_BITS) >> 16) as usize
}

unsafe fn eccerrlog_row(channel: i32, log: u64) -> i32 {
    (((log & X38_ECCERRLOG_RANK_BITS) >> 27) as i32) |
        (channel * X38_RANKS_PER_CHANNEL as i32)
}

#[repr(i32)]
enum x38_chips { X38 = 0 }

#[repr(C)]
struct x38_dev_info { ctl_name: *const i8 }

#[repr(C)]
struct x38_error_info {
    errsts: u16,
    errsts2: u16,
    eccerrlog: [u64; X38_CHANNELS],
}

static x38_devs: [x38_dev_info; 1] = [x38_dev_info { ctl_name: b"x38\0".as_ptr() as *const i8 }];
static mut mci_pdev: *mut pci_dev = core::ptr::null_mut();
static mut x38_registered: i32 = 1;

unsafe fn x38_clear_error_info(mci: *mut mem_ctl_info) {
    let pdev = to_pci_dev((*mci).pdev);
    pci_write_bits16(pdev, X38_ERRSTS, X38_ERRSTS_BITS, X38_ERRSTS_BITS);
}

unsafe fn x38_get_and_clear_error_info(mci: *mut mem_ctl_info, info: *mut x38_error_info) {
    let pdev = to_pci_dev((*mci).pdev);
    let window = (*mci).pvt_info as *mut u8;
    pci_read_config_word(pdev, X38_ERRSTS, &mut (*info).errsts);
    if (*info).errsts & X38_ERRSTS_BITS == 0 { return; }
    (*info).eccerrlog[0] = lo_hi_readq(window.add(X38_C0ECCERRLOG));
    if x38_channel_num == 2 { (*info).eccerrlog[1] = lo_hi_readq(window.add(X38_C1ECCERRLOG)); }
    pci_read_config_word(pdev, X38_ERRSTS, &mut (*info).errsts2);
    if ((*info).errsts ^ (*info).errsts2) & X38_ERRSTS_BITS != 0 {
        (*info).eccerrlog[0] = lo_hi_readq(window.add(X38_C0ECCERRLOG));
        if x38_channel_num == 2 { (*info).eccerrlog[1] = lo_hi_readq(window.add(X38_C1ECCERRLOG)); }
    }
    x38_clear_error_info(mci);
}

unsafe fn x38_process_error_info(mci: *mut mem_ctl_info, info: *mut x38_error_info) {
    if (*info).errsts & X38_ERRSTS_BITS == 0 { return; }
    if ((*info).errsts ^ (*info).errsts2) & X38_ERRSTS_BITS != 0 {
        edac_mc_handle_error(HW_EVENT_ERR_UNCORRECTED, mci, 1, 0, 0, 0, -1, -1, -1, b"UE overwrote CE\0".as_ptr() as *const i8, b"\0".as_ptr() as *const i8);
        (*info).errsts = (*info).errsts2;
    }
    for channel in 0..x38_channel_num {
        let log = (*info).eccerrlog[channel as usize];
        if log & X38_ECCERRLOG_UE != 0 {
            edac_mc_handle_error(HW_EVENT_ERR_UNCORRECTED, mci, 1, 0, 0, 0, eccerrlog_row(channel, log), -1, -1, b"x38 UE\0".as_ptr() as *const i8, b"\0".as_ptr() as *const i8);
        } else if log & X38_ECCERRLOG_CE != 0 {
            edac_mc_handle_error(HW_EVENT_ERR_CORRECTED, mci, 1, 0, 0, eccerrlog_syndrome(log) as i32, eccerrlog_row(channel, log), -1, -1, b"x38 CE\0".as_ptr() as *const i8, b"\0".as_ptr() as *const i8);
        }
    }
}

unsafe fn x38_check(mci: *mut mem_ctl_info) {
    let mut info = core::mem::MaybeUninit::<x38_error_info>::uninit();
    x38_get_and_clear_error_info(mci, info.as_mut_ptr());
    x38_process_error_info(mci, info.as_mut_ptr());
}

#[repr(C)]
union x38_mchbar_union { mchbar: u64, parts: [u32; 2] }

unsafe fn x38_map_mchbar(pdev: *mut pci_dev) -> *mut u8 {
    let mut u = x38_mchbar_union { mchbar: 0 };
    pci_read_config_dword(pdev, X38_MCHBAR_LOW, &mut u.parts[0]);
    pci_write_config_dword(pdev, X38_MCHBAR_LOW, u.parts[0] | 0x1);
    pci_read_config_dword(pdev, X38_MCHBAR_HIGH, &mut u.parts[1]);
    u.mchbar &= X38_MCHBAR_MASK;
    if u.mchbar != u.mchbar as resource_size_t as u64 {
        printk(KERN_ERR, b"x38: mmio space beyond accessible range (0x%llx)\n\0".as_ptr() as *const i8, u.mchbar);
        return core::ptr::null_mut();
    }
    let window = ioremap(u.mchbar, X38_MMR_WINDOW_SIZE);
    if window.is_null() { printk(KERN_ERR, b"x38: cannot map mmio space at 0x%llx\n\0".as_ptr() as *const i8, u.mchbar); }
    window
}

unsafe fn x38_get_drbs(window: *mut u8, drbs: &mut [[u16; X38_RANKS_PER_CHANNEL]; X38_CHANNELS]) {
    for i in 0..X38_RANKS_PER_CHANNEL {
        drbs[0][i] = readw(window.add(X38_C0DRB + 2 * i)) & X38_DRB_MASK;
        drbs[1][i] = readw(window.add(X38_C1DRB + 2 * i)) & X38_DRB_MASK;
    }
}

unsafe fn x38_is_stacked(pdev: *mut pci_dev, drbs: &[[u16; X38_RANKS_PER_CHANNEL]; X38_CHANNELS]) -> bool {
    let mut tom = 0u16;
    pci_read_config_word(pdev, X38_TOM, &mut tom);
    (drbs[X38_CHANNELS - 1][X38_RANKS_PER_CHANNEL - 1] == (tom & X38_TOM_MASK))
}

unsafe fn drb_to_nr_pages(drbs: &[[u16; X38_RANKS_PER_CHANNEL]; X38_CHANNELS], stacked: bool, channel: usize, rank: usize) -> usize {
    let mut n = drbs[channel][rank] as usize;
    if rank > 0 { n -= drbs[channel][rank - 1] as usize; }
    if stacked && channel == 1 && drbs[channel][rank] == drbs[channel][X38_RANKS_PER_CHANNEL - 1] { n -= drbs[0][X38_RANKS_PER_CHANNEL - 1] as usize; }
    n << (X38_DRB_SHIFT - PAGE_SHIFT)
}

unsafe fn x38_probe1(pdev: *mut pci_dev, dev_idx: i32) -> i32 {
    let window = x38_map_mchbar(pdev);
    if window.is_null() { return -ENODEV; }
    let mut drbs = [[0u16; X38_RANKS_PER_CHANNEL]; X38_CHANNELS];
    x38_get_drbs(window, &mut drbs);
    how_many_channel(pdev);
    let stacked = x38_is_stacked(pdev, &drbs);
    /* FIXME: unconventional pvt_info usage */
    let mut layers = [edac_mc_layer { layer_type: EDAC_MC_LAYER_CHIP_SELECT, size: X38_RANKS, is_virt_csrow: true }, edac_mc_layer { layer_type: EDAC_MC_LAYER_CHANNEL, size: x38_channel_num as usize, is_virt_csrow: false }];
    let mci = edac_mc_alloc(0, 2, layers.as_mut_ptr(), 0);
    if mci.is_null() { iounmap(window); return -ENOMEM; }
    (*mci).pdev = &mut (*pdev).dev;
    (*mci).mtype_cap = MEM_FLAG_DDR2;
    (*mci).edac_ctl_cap = EDAC_FLAG_SECDED;
    (*mci).edac_cap = EDAC_FLAG_SECDED;
    (*mci).mod_name = EDAC_MOD_STR.as_ptr() as *const i8;
    (*mci).ctl_name = x38_devs[dev_idx as usize].ctl_name;
    (*mci).dev_name = pci_name(pdev);
    (*mci).edac_check = Some(x38_check);
    (*mci).ctl_page_to_phys = None;
    (*mci).pvt_info = window as *mut core::ffi::c_void;
    for i in 0..(*mci).nr_csrows {
        let nr_pages = drb_to_nr_pages(&drbs, stacked, i / X38_RANKS_PER_CHANNEL, i % X38_RANKS_PER_CHANNEL);
        if nr_pages == 0 { continue; }
        let csrow = *(*mci).csrows.add(i);
        for j in 0..x38_channel_num as usize {
            let dimm = (*(*csrow).channels.add(j)).dimm;
            (*dimm).nr_pages = nr_pages / x38_channel_num as usize;
            (*dimm).grain = nr_pages << PAGE_SHIFT;
            (*dimm).mtype = MEM_DDR2;
            (*dimm).dtype = DEV_UNKNOWN;
            (*dimm).edac_mode = EDAC_UNKNOWN;
        }
    }
    x38_clear_error_info(mci);
    if edac_mc_add_mc(mci) != 0 { edac_mc_free(mci); iounmap(window); return -ENODEV; }
    0
}

unsafe fn x38_init_one(pdev: *mut pci_dev, ent: *const pci_device_id) -> i32 {
    if pci_enable_device(pdev) < 0 { return -EIO; }
    let rc = x38_probe1(pdev, (*ent).driver_data);
    if mci_pdev.is_null() { mci_pdev = pci_dev_get(pdev); }
    rc
}

unsafe fn x38_remove_one(pdev: *mut pci_dev) {
    let mci = edac_mc_del_mc(&mut (*pdev).dev);
    if mci.is_null() { return; }
    iounmap((*mci).pvt_info as *mut u8);
    edac_mc_free(mci);
}

/* PCI table and module registration are supplied through the kernel bindings. */
static x38_pci_tbl: [pci_device_id; 2] = [pci_device_id { vendor: PCI_VENDOR_ID_INTEL, device: PCI_DEVICE_ID_INTEL_X38_HB, driver_data: x38_chips::X38 as i32 }, pci_device_id::default()];
static x38_driver: pci_driver = pci_driver { name: EDAC_MOD_STR.as_ptr() as *const i8, probe: Some(x38_init_one), remove: Some(x38_remove_one), id_table: x38_pci_tbl.as_ptr() };

unsafe fn x38_init() -> i32 {
    opstate_init();
    let mut pci_rc = pci_register_driver(&x38_driver);
    if pci_rc < 0 { pci_dev_put(mci_pdev); return pci_rc; }
    if mci_pdev.is_null() {
        x38_registered = 0;
        mci_pdev = pci_get_device(PCI_VENDOR_ID_INTEL, PCI_DEVICE_ID_INTEL_X38_HB, core::ptr::null_mut());
        if mci_pdev.is_null() { pci_unregister_driver(&x38_driver); return -ENODEV; }
        pci_rc = x38_init_one(mci_pdev, x38_pci_tbl.as_ptr());
        if pci_rc < 0 { pci_unregister_driver(&x38_driver); return -ENODEV; }
    }
    0
}

unsafe fn x38_exit() {
    pci_unregister_driver(&x38_driver);
    if x38_registered == 0 { x38_remove_one(mci_pdev); pci_dev_put(mci_pdev); }
}

/* module_init(x38_init); module_exit(x38_exit); */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
