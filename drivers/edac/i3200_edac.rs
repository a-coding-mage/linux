/*
 * Intel 3200/3210 Memory Controller kernel module
 * Copyright (C) 2008-2009 Akamai Technologies, Inc.
 * Portions by Hitoshi Mitake <h.mitake@gmail.com>.
 *
 * This file may be distributed under the terms of the
 * GNU General Public License.
 */

// Kernel dependencies supplied by the surrounding repository/build.

const EDAC_MOD_STR: &str = "i3200_edac";
const PCI_DEVICE_ID_INTEL_3200_HB: u16 = 0x29f0;
const I3200_DIMMS: usize = 4;
const I3200_RANKS: usize = 8;
const I3200_RANKS_PER_CHANNEL: usize = 4;
const I3200_CHANNELS: usize = 2;

const I3200_MCHBAR_LOW: usize = 0x48;
const I3200_MCHBAR_HIGH: usize = 0x4c;
const I3200_MCHBAR_MASK: u64 = 0xfffffc000;
const I3200_MMR_WINDOW_SIZE: usize = 16384;
const I3200_TOM: usize = 0xa0;
const I3200_TOM_MASK: u16 = 0x3ff;
const I3200_TOM_SHIFT: usize = 26;
const I3200_ERRSTS: usize = 0xc8;
const I3200_ERRSTS_UE: u16 = 0x0002;
const I3200_ERRSTS_CE: u16 = 0x0001;
const I3200_ERRSTS_BITS: u16 = I3200_ERRSTS_UE | I3200_ERRSTS_CE;
const I3200_C0DRB: usize = 0x200;
const I3200_C1DRB: usize = 0x600;
const I3200_DRB_MASK: u16 = 0x3ff;
const I3200_DRB_SHIFT: usize = 26;
const I3200_C0ECCERRLOG: usize = 0x280;
const I3200_C1ECCERRLOG: usize = 0x680;
const I3200_ECCERRLOG_CE: u64 = 0x1;
const I3200_ECCERRLOG_UE: u64 = 0x2;
const I3200_ECCERRLOG_RANK_BITS: u64 = 0x18000000;
const I3200_ECCERRLOG_RANK_SHIFT: usize = 27;
const I3200_ECCERRLOG_SYNDROME_BITS: u64 = 0xff0000;
const I3200_ECCERRLOG_SYNDROME_SHIFT: usize = 16;
const I3200_CAPID0: usize = 0xe0;

#[repr(C)]
pub struct i3200_priv {
    pub window: *mut core::ffi::c_void,
}

static mut nr_channels: i32 = 0;

unsafe fn how_many_channels(pdev: *mut pci_dev) -> i32 {
    let mut capid0_8b: u8 = 0;
    pci_read_config_byte(pdev, (I3200_CAPID0 + 8) as u32, &mut capid0_8b);
    let n_channels;
    if capid0_8b & 0x20 != 0 {
        edac_dbg(0, "In single channel mode\n");
        n_channels = 1;
    } else {
        edac_dbg(0, "In dual channel mode\n");
        n_channels = 2;
    }
    if capid0_8b & 0x10 != 0 {
        edac_dbg(0, "2 DIMMS per channel disabled\n");
    } else {
        edac_dbg(0, "2 DIMMS per channel enabled\n");
    }
    n_channels
}

unsafe fn eccerrlog_syndrome(log: u64) -> u64 {
    (log & I3200_ECCERRLOG_SYNDROME_BITS) >> I3200_ECCERRLOG_SYNDROME_SHIFT
}

unsafe fn eccerrlog_row(channel: i32, log: u64) -> u64 {
    let rank = (log & I3200_ECCERRLOG_RANK_BITS) >> I3200_ECCERRLOG_RANK_SHIFT;
    rank + (channel as u64 * I3200_RANKS_PER_CHANNEL as u64)
}

#[repr(C)]
pub struct i3200_dev_info { pub ctl_name: *const core::ffi::c_char }

#[repr(C)]
pub struct i3200_error_info {
    pub errsts: u16,
    pub errsts2: u16,
    pub eccerrlog: [u64; I3200_CHANNELS],
}

static i3200_devs: [i3200_dev_info; 1] = [i3200_dev_info { ctl_name: b"i3200\0".as_ptr() as *const _ }];
static mut mci_pdev: *mut pci_dev = core::ptr::null_mut();
static mut i3200_registered: i32 = 1;

unsafe fn i3200_clear_error_info(mci: *mut mem_ctl_info) {
    let pdev = to_pci_dev((*mci).pdev);
    pci_write_bits16(pdev, I3200_ERRSTS as u32, I3200_ERRSTS_BITS, I3200_ERRSTS_BITS);
}

unsafe fn i3200_get_and_clear_error_info(mci: *mut mem_ctl_info, info: *mut i3200_error_info) {
    let pdev = to_pci_dev((*mci).pdev);
    let priv_ = (*mci).pvt_info as *mut i3200_priv;
    let window = (*priv_).window;
    pci_read_config_word(pdev, I3200_ERRSTS as u32, &mut (*info).errsts);
    if (*info).errsts & I3200_ERRSTS_BITS == 0 { return; }
    (*info).eccerrlog[0] = readq(window.add(I3200_C0ECCERRLOG));
    if nr_channels == 2 { (*info).eccerrlog[1] = readq(window.add(I3200_C1ECCERRLOG)); }
    pci_read_config_word(pdev, I3200_ERRSTS as u32, &mut (*info).errsts2);
    if ((*info).errsts ^ (*info).errsts2) & I3200_ERRSTS_BITS != 0 {
        (*info).eccerrlog[0] = readq(window.add(I3200_C0ECCERRLOG));
        if nr_channels == 2 { (*info).eccerrlog[1] = readq(window.add(I3200_C1ECCERRLOG)); }
    }
    i3200_clear_error_info(mci);
}

unsafe fn i3200_process_error_info(mci: *mut mem_ctl_info, info: *mut i3200_error_info) {
    if (*info).errsts & I3200_ERRSTS_BITS == 0 { return; }
    if ((*info).errsts ^ (*info).errsts2) & I3200_ERRSTS_BITS != 0 {
        edac_mc_handle_error(HW_EVENT_ERR_UNCORRECTED, mci, 1, 0, 0, 0, -1, -1, -1, "UE overwrote CE", "");
        (*info).errsts = (*info).errsts2;
    }
    for channel in 0..nr_channels {
        let log = (*info).eccerrlog[channel as usize];
        if log & I3200_ECCERRLOG_UE != 0 {
            edac_mc_handle_error(HW_EVENT_ERR_UNCORRECTED, mci, 1, 0, 0, 0, eccerrlog_row(channel, log) as i32, -1, -1, "i3000 UE", "");
        } else if log & I3200_ECCERRLOG_CE != 0 {
            edac_mc_handle_error(HW_EVENT_ERR_CORRECTED, mci, 1, 0, 0, eccerrlog_syndrome(log) as i32, eccerrlog_row(channel, log) as i32, -1, -1, "i3000 CE", "");
        }
    }
}

unsafe fn i3200_check(mci: *mut mem_ctl_info) {
    let mut info = i3200_error_info { errsts: 0, errsts2: 0, eccerrlog: [0; I3200_CHANNELS] };
    i3200_get_and_clear_error_info(mci, &mut info);
    i3200_process_error_info(mci, &mut info);
}

// The remaining kernel-facing routines retain the C implementation's interfaces and are
// expressed with external types/functions supplied by the surrounding kernel bindings.
unsafe fn i3200_map_mchbar(pdev: *mut pci_dev) -> *mut core::ffi::c_void {
    let mut low = 0u32;
    let mut high = 0u32;
    pci_read_config_dword(pdev, I3200_MCHBAR_LOW as u32, &mut low);
    pci_read_config_dword(pdev, I3200_MCHBAR_HIGH as u32, &mut high);
    let mchbar = (((high as u64) << 32) | low as u64) & I3200_MCHBAR_MASK;
    if mchbar != mchbar as resource_size_t { printk(KERN_ERR, "i3200: mmio space beyond accessible range (0x%llx)\n", mchbar); return core::ptr::null_mut(); }
    let window = ioremap(mchbar as resource_size_t, I3200_MMR_WINDOW_SIZE);
    if window.is_null() { printk(KERN_ERR, "i3200: cannot map mmio space at 0x%llx\n", mchbar); }
    window
}

unsafe fn i3200_get_drbs(window: *mut core::ffi::c_void, drbs: &mut [[u16; I3200_RANKS_PER_CHANNEL]; I3200_CHANNELS]) {
    for i in 0..I3200_RANKS_PER_CHANNEL {
        drbs[0][i] = readw(window.add(I3200_C0DRB + 2 * i)) & I3200_DRB_MASK;
        drbs[1][i] = readw(window.add(I3200_C1DRB + 2 * i)) & I3200_DRB_MASK;
        edac_dbg(0, "drb[0][%d] = %d, drb[1][%d] = %d\n", i, drbs[0][i], i, drbs[1][i]);
    }
}

unsafe fn i3200_is_stacked(pdev: *mut pci_dev, drbs: &[[u16; I3200_RANKS_PER_CHANNEL]; I3200_CHANNELS]) -> bool {
    let mut tom = 0u16;
    pci_read_config_word(pdev, I3200_TOM as u32, &mut tom);
    (drbs[I3200_CHANNELS - 1][I3200_RANKS_PER_CHANNEL - 1] == (tom & I3200_TOM_MASK))
}

unsafe fn drb_to_nr_pages(drbs: &[[u16; I3200_RANKS_PER_CHANNEL]; I3200_CHANNELS], stacked: bool, channel: usize, rank: usize) -> usize {
    let mut n = drbs[channel][rank] as usize;
    if n == 0 { return 0; }
    if rank > 0 { n -= drbs[channel][rank - 1] as usize; }
    if stacked && channel == 1 && drbs[channel][rank] == drbs[channel][I3200_RANKS_PER_CHANNEL - 1] { n -= drbs[0][I3200_RANKS_PER_CHANNEL - 1] as usize; }
    n << (I3200_DRB_SHIFT - PAGE_SHIFT)
}

unsafe fn i3200_probe1(pdev: *mut pci_dev, dev_idx: usize) -> i32 {
    edac_dbg(0, "MC:\n");
    let window = i3200_map_mchbar(pdev);
    if window.is_null() { return -ENODEV; }
    let mut drbs = [[0u16; I3200_RANKS_PER_CHANNEL]; I3200_CHANNELS];
    i3200_get_drbs(window, &mut drbs);
    nr_channels = how_many_channels(pdev);
    let mut layers = [edac_mc_layer { layer_type: EDAC_MC_LAYER_CHIP_SELECT, size: I3200_DIMMS, is_virt_csrow: true }, edac_mc_layer { layer_type: EDAC_MC_LAYER_CHANNEL, size: nr_channels as usize, is_virt_csrow: false }];
    let mci = edac_mc_alloc(0, 2, layers.as_mut_ptr(), core::mem::size_of::<i3200_priv>());
    if mci.is_null() { iounmap(window); return -ENOMEM; }
    (*mci).pdev = &mut (*pdev).dev;
    (*mci).mtype_cap = MEM_FLAG_DDR2;
    (*mci).edac_ctl_cap = EDAC_FLAG_SECDED;
    (*mci).edac_cap = EDAC_FLAG_SECDED;
    (*mci).mod_name = EDAC_MOD_STR.as_ptr() as *const _;
    (*mci).ctl_name = i3200_devs[dev_idx].ctl_name;
    (*mci).dev_name = pci_name(pdev);
    (*mci).edac_check = Some(i3200_check);
    let priv_ = (*mci).pvt_info as *mut i3200_priv;
    (*priv_).window = window;
    let stacked = i3200_is_stacked(pdev, &drbs);
    for i in 0..I3200_DIMMS {
        for j in 0..nr_channels as usize {
            let nr_pages = drb_to_nr_pages(&drbs, stacked, j, i);
            if nr_pages == 0 { continue; }
            let dimm = edac_get_dimm(mci, i, j, 0);
            (*dimm).nr_pages = nr_pages;
            (*dimm).grain = nr_pages << PAGE_SHIFT;
            (*dimm).mtype = MEM_DDR2;
            (*dimm).dtype = DEV_UNKNOWN;
            (*dimm).edac_mode = EDAC_UNKNOWN;
        }
    }
    i3200_clear_error_info(mci);
    if edac_mc_add_mc(mci) != 0 { edac_mc_free(mci); iounmap(window); return -ENODEV; }
    0
}

unsafe fn i3200_init_one(pdev: *mut pci_dev, ent: *const pci_device_id) -> i32 {
    edac_dbg(0, "MC:\n");
    if pci_enable_device(pdev) < 0 { return -EIO; }
    let rc = i3200_probe1(pdev, (*ent).driver_data as usize);
    if mci_pdev.is_null() { mci_pdev = pci_dev_get(pdev); }
    rc
}

unsafe fn i3200_remove_one(pdev: *mut pci_dev) {
    edac_dbg(0, "\n");
    let mci = edac_mc_del_mc(&mut (*pdev).dev);
    if mci.is_null() { return; }
    let priv_ = (*mci).pvt_info as *mut i3200_priv;
    iounmap((*priv_).window);
    edac_mc_free(mci);
    pci_disable_device(pdev);
}

static i3200_pci_tbl: [pci_device_id; 2] = [
    pci_device_id { vendor: PCI_VENDOR_ID_INTEL, device: PCI_DEVICE_ID_INTEL_3200_HB, driver_data: I3200 as usize },
    pci_device_id { vendor: 0, device: 0, driver_data: 0 },
];

unsafe fn i3200_init() -> i32 {
    edac_dbg(3, "MC:\n");
    opstate_init();
    let mut pci_rc = pci_register_driver(&i3200_driver);
    if pci_rc < 0 { pci_dev_put(mci_pdev); return pci_rc; }
    if mci_pdev.is_null() {
        i3200_registered = 0;
        mci_pdev = pci_get_device(PCI_VENDOR_ID_INTEL, PCI_DEVICE_ID_INTEL_3200_HB, core::ptr::null_mut());
        if mci_pdev.is_null() { pci_unregister_driver(&i3200_driver); pci_dev_put(mci_pdev); return -ENODEV; }
        pci_rc = i3200_init_one(mci_pdev, i3200_pci_tbl.as_ptr());
        if pci_rc < 0 { pci_unregister_driver(&i3200_driver); pci_dev_put(mci_pdev); return -ENODEV; }
    }
    0
}

unsafe fn i3200_exit() {
    edac_dbg(3, "MC:\n");
    pci_unregister_driver(&i3200_driver);
    if i3200_registered == 0 { i3200_remove_one(mci_pdev); pci_dev_put(mci_pdev); }
}

static mut i3200_driver: pci_driver = pci_driver { name: EDAC_MOD_STR.as_ptr() as *const _, probe: Some(i3200_init_one), remove: Some(i3200_remove_one), id_table: i3200_pci_tbl.as_ptr() };

// module_init(i3200_init); module_exit(i3200_exit);
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Akamai Technologies, Inc.");
// MODULE_DESCRIPTION("MC support for Intel 3200 memory hub controllers");
// module_param(edac_op_state, int, 0444);
// MODULE_PARM_DESC(edac_op_state, "EDAC Error Reporting state: 0=Poll,1=NMI");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
