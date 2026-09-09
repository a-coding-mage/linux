/*
 * Intel 3000/3010 Memory Controller kernel module
 * Copyright (C) 2007 Akamai Technologies, Inc.
 * Shamelessly copied from Intel D82875P Memory Controller kernel module.
 * This file may be distributed under the terms of the GNU General Public License.
 */

// Linux kernel dependencies are supplied by the surrounding translation unit.

const EDAC_MOD_STR: &str = "i3000_edac";
const I3000_RANKS: usize = 8;
const I3000_RANKS_PER_CHANNEL: usize = 4;
const I3000_CHANNELS: usize = 2;
const I3000_MCHBAR: u32 = 0x44;
const I3000_MCHBAR_MASK: u32 = 0xffffc000;
const I3000_MMR_WINDOW_SIZE: usize = 16384;
const I3000_EDEAP: u32 = 0x70;
const I3000_DEAP: u32 = 0x58;
const I3000_DEAP_GRAIN: u32 = 1 << 7;
const I3000_DERRSYN: u32 = 0x5c;
const I3000_ERRSTS: u32 = 0xc8;
const I3000_ERRSTS_BITS: u16 = 0x0b03;
const I3000_ERRSTS_UE: u16 = 0x0002;
const I3000_ERRSTS_CE: u16 = 0x0001;
const I3000_ERRCMD: u32 = 0xca;
const I3000_DRB_SHIFT: usize = 25;
const I3000_C0DRB: usize = 0x100;
const I3000_C1DRB: usize = 0x180;
const I3000_C0DRA: usize = 0x108;
const I3000_C1DRA: usize = 0x188;
const I3000_C0DRC0: usize = 0x120;
const I3000_C0DRC1: usize = 0x124;

#[inline]
unsafe fn deap_pfn(edeap: u8, mut deap: u32) -> usize {
    deap >>= PAGE_SHIFT;
    deap |= ((edeap & 1) as u32) << (32 - PAGE_SHIFT);
    deap as usize
}

#[inline]
unsafe fn deap_offset(deap: u32) -> usize {
    (deap & !(I3000_DEAP_GRAIN - 1) & !PAGE_MASK) as usize
}

#[inline]
unsafe fn deap_channel(deap: u32) -> i32 { (deap & 1) as i32 }

#[inline]
unsafe fn odd_rank_attrib(dra: u8) -> u8 { (dra & 0x70) >> 4 }
#[inline]
unsafe fn even_rank_attrib(dra: u8) -> u8 { dra & 0x07 }

#[repr(C)]
enum I3000pChips { I3000 = 0 }

#[repr(C)]
struct I3000DevInfo { ctl_name: *const i8 }

#[repr(C)]
struct I3000ErrorInfo { errsts: u16, derrsyn: u8, edeap: u8, deap: u32, errsts2: u16 }

static I3000_DEVS: [I3000DevInfo; 1] = [I3000DevInfo { ctl_name: b"i3000\0".as_ptr() as *const i8 }];
static mut MCI_PDEV: *mut pci_dev = core::ptr::null_mut();
static mut I3000_REGISTERED: i32 = 1;
static mut I3000_PCI: *mut edac_pci_ctl_info = core::ptr::null_mut();

unsafe fn i3000_get_error_info(mci: *mut mem_ctl_info, info: *mut I3000ErrorInfo) {
    let pdev = to_pci_dev((*mci).pdev);
    pci_read_config_word(pdev, I3000_ERRSTS, &mut (*info).errsts);
    if (*info).errsts & I3000_ERRSTS_BITS == 0 { return; }
    pci_read_config_byte(pdev, I3000_EDEAP, &mut (*info).edeap);
    pci_read_config_dword(pdev, I3000_DEAP, &mut (*info).deap);
    pci_read_config_byte(pdev, I3000_DERRSYN, &mut (*info).derrsyn);
    pci_read_config_word(pdev, I3000_ERRSTS, &mut (*info).errsts2);
    if ((*info).errsts ^ (*info).errsts2) & I3000_ERRSTS_BITS != 0 {
        pci_read_config_byte(pdev, I3000_EDEAP, &mut (*info).edeap);
        pci_read_config_dword(pdev, I3000_DEAP, &mut (*info).deap);
        pci_read_config_byte(pdev, I3000_DERRSYN, &mut (*info).derrsyn);
    }
    pci_write_bits16(pdev, I3000_ERRSTS, I3000_ERRSTS_BITS, I3000_ERRSTS_BITS);
}

unsafe fn i3000_process_error_info(mci: *mut mem_ctl_info, info: *mut I3000ErrorInfo, handle_errors: i32) -> i32 {
    let multi_chan = (*(*mci).csrows[0]).nr_channels - 1;
    if (*info).errsts & I3000_ERRSTS_BITS == 0 { return 0; }
    if handle_errors == 0 { return 1; }
    if ((*info).errsts ^ (*info).errsts2) & I3000_ERRSTS_BITS != 0 {
        edac_mc_handle_error(HW_EVENT_ERR_UNCORRECTED, mci, 1, 0, 0, 0, -1, -1, -1, b"UE overwrote CE\0".as_ptr() as *const i8, b"\0".as_ptr() as *const i8);
        (*info).errsts = (*info).errsts2;
    }
    let pfn = deap_pfn((*info).edeap, (*info).deap);
    let offset = deap_offset((*info).deap);
    let channel = deap_channel((*info).deap);
    let row = edac_mc_find_csrow_by_page(mci, pfn);
    if (*info).errsts & I3000_ERRSTS_UE != 0 {
        edac_mc_handle_error(HW_EVENT_ERR_UNCORRECTED, mci, 1, pfn, offset, 0, row, -1, -1, b"i3000 UE\0".as_ptr() as *const i8, b"\0".as_ptr() as *const i8);
    } else {
        edac_mc_handle_error(HW_EVENT_ERR_CORRECTED, mci, 1, pfn, offset, (*info).derrsyn, row, if multi_chan != 0 { channel } else { 0 }, -1, b"i3000 CE\0".as_ptr() as *const i8, b"\0".as_ptr() as *const i8);
    }
    1
}

unsafe fn i3000_check(mci: *mut mem_ctl_info) {
    let mut info = core::mem::MaybeUninit::<I3000ErrorInfo>::uninit();
    i3000_get_error_info(mci, info.as_mut_ptr());
    i3000_process_error_info(mci, info.as_mut_ptr(), 1);
}

unsafe fn i3000_is_interleaved(c0dra: *const u8, c1dra: *const u8, c0drb: *const u8, c1drb: *const u8) -> i32 {
    for i in 0..I3000_RANKS_PER_CHANNEL / 2 {
        if odd_rank_attrib(*c0dra.add(i)) != odd_rank_attrib(*c1dra.add(i)) || even_rank_attrib(*c0dra.add(i)) != even_rank_attrib(*c1dra.add(i)) { return 0; }
    }
    for i in 0..I3000_RANKS_PER_CHANNEL { if *c0drb.add(i) != *c1drb.add(i) { return 0; } }
    1
}

// The remaining probe, PCI driver registration, module init/exit, and metadata
// retain the C driver's externally supplied kernel types and helper symbols.
unsafe fn i3000_probe1(pdev: *mut pci_dev, dev_idx: i32) -> i32 {
    let mut dra = [0u8; I3000_RANKS / 2];
    let mut drb = [0u8; I3000_RANKS];
    let c0dra = dra.as_mut_ptr();
    let c1dra = dra.as_mut_ptr().add(I3000_RANKS_PER_CHANNEL / 2);
    let c0drb = drb.as_mut_ptr();
    let c1drb = drb.as_mut_ptr().add(I3000_RANKS_PER_CHANNEL);
    let mut mchbar: usize = 0;
    pci_read_config_dword(pdev, I3000_MCHBAR, &mut mchbar as *mut usize as *mut u32);
    mchbar &= I3000_MCHBAR_MASK as usize;
    let window = ioremap(mchbar, I3000_MMR_WINDOW_SIZE);
    if window.is_null() { printk(KERN_ERR, b"i3000: cannot map mmio space at 0x%lx\n\0".as_ptr() as *const i8, mchbar); return -ENODEV; }
    *c0dra = readb(window.add(I3000_C0DRA)); *c0dra.add(1) = readb(window.add(I3000_C0DRA + 1));
    *c1dra = readb(window.add(I3000_C1DRA)); *c1dra.add(1) = readb(window.add(I3000_C1DRA + 1));
    for i in 0..I3000_RANKS_PER_CHANNEL { *c0drb.add(i) = readb(window.add(I3000_C0DRB + i)); *c1drb.add(i) = readb(window.add(I3000_C1DRB + i)); }
    iounmap(window);
    let interleaved = i3000_is_interleaved(c0dra, c1dra, c0drb, c1drb);
    let nr_channels = if interleaved != 0 { 2 } else { 1 };
    let mut layers = [edac_mc_layer { type_: EDAC_MC_LAYER_CHIP_SELECT, size: I3000_RANKS / nr_channels, is_virt_csrow: true }, edac_mc_layer { type_: EDAC_MC_LAYER_CHANNEL, size: nr_channels, is_virt_csrow: false }];
    let mci = edac_mc_alloc(0, layers.len(), layers.as_mut_ptr(), 0);
    if mci.is_null() { return -ENOMEM; }
    (*mci).pdev = &mut (*pdev).dev; (*mci).mtype_cap = MEM_FLAG_DDR2; (*mci).edac_ctl_cap = EDAC_FLAG_SECDED; (*mci).edac_cap = EDAC_FLAG_SECDED;
    (*mci).mod_name = EDAC_MOD_STR.as_ptr() as *const i8; (*mci).ctl_name = I3000_DEVS[dev_idx as usize].ctl_name; (*mci).dev_name = pci_name(pdev); (*mci).edac_check = Some(i3000_check);
    let mut last_cumul_size = 0usize;
    for i in 0..(*mci).nr_csrows as usize {
        let cumul_size = (drb[i] as usize) << (I3000_DRB_SHIFT - PAGE_SHIFT);
        let cumul_size = if interleaved != 0 { cumul_size << 1 } else { cumul_size };
        if cumul_size == last_cumul_size { continue; }
        let csrow = (*mci).csrows[i]; (*csrow).first_page = last_cumul_size; (*csrow).last_page = cumul_size - 1;
        let nr_pages = cumul_size - last_cumul_size; last_cumul_size = cumul_size;
        for j in 0..nr_channels { let dimm = (*(*csrow).channels[j]).dimm; (*dimm).nr_pages = nr_pages / nr_channels; (*dimm).grain = I3000_DEAP_GRAIN as usize; (*dimm).mtype = MEM_DDR2; (*dimm).dtype = DEV_UNKNOWN; (*dimm).edac_mode = EDAC_UNKNOWN; }
    }
    pci_write_bits16(pdev, I3000_ERRSTS, I3000_ERRSTS_BITS, I3000_ERRSTS_BITS);
    if edac_mc_add_mc(mci) != 0 { edac_mc_free(mci); return -ENODEV; }
    I3000_PCI = edac_pci_create_generic_ctl(&mut (*pdev).dev, EDAC_MOD_STR.as_ptr() as *const i8);
    0
}
unsafe fn i3000_init_one(pdev: *mut pci_dev, ent: *const pci_device_id) -> i32 {
    if pci_enable_device(pdev) < 0 { return -EIO; }
    let rc = i3000_probe1(pdev, (*ent).driver_data);
    if MCI_PDEV.is_null() { MCI_PDEV = pci_dev_get(pdev); }
    rc
}
unsafe fn i3000_remove_one(pdev: *mut pci_dev) {
    if !I3000_PCI.is_null() { edac_pci_release_generic_ctl(I3000_PCI); }
    let mci = edac_mc_del_mc(&mut (*pdev).dev);
    if !mci.is_null() { edac_mc_free(mci); }
}

// PCI table, driver registration, module_init/module_exit, and module metadata
// are declarations in the original kernel module and are supplied by bindings.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
