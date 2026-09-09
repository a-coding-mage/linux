// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2001-2003 SuSE Labs.
 * Distributed under the GNU public license, v2.
 *
 * This is a GART driver for the AMD Opteron/Athlon64 on-CPU northbridge.
 * It also includes support for the AMD 8151 AGP bridge,
 * although it doesn't actually do much, as all the real
 * work is done in the northbridge(s).
 */

// Linux kernel headers provide the external types, constants, macros, and functions
// referenced below; build-time configuration conditions are retained as comments.

const NVIDIA_X86_64_0_APBASE: u32 = 0x10;
const NVIDIA_X86_64_1_APBASE1: u32 = 0x50;
const NVIDIA_X86_64_1_APLIMIT1: u32 = 0x54;
const NVIDIA_X86_64_1_APSIZE: u32 = 0xa8;
const NVIDIA_X86_64_1_APBASE2: u32 = 0xd8;
const NVIDIA_X86_64_1_APLIMIT2: u32 = 0xdc;
const ULI_X86_64_BASE_ADDR: u32 = 0x10;
const ULI_X86_64_HTT_FEA_REG: u32 = 0x50;
const ULI_X86_64_ENU_SCR_REG: u32 = 0x54;

static mut aperture_resource: *mut resource = core::ptr::null_mut();
static mut agp_try_unsupported: bool = true;
static mut agp_bridges_found: i32 = 0;

unsafe fn amd64_tlbflush(_temp: *mut agp_memory) { amd_flush_garts(); }

unsafe fn amd64_insert_memory(mem: *mut agp_memory, pg_start: off_t, type_: i32) -> i32 {
    let bridge = (*mem).bridge;
    let num_entries = agp_num_entries();
    if type_ != (*mem).type_ { return -EINVAL; }
    let mask_type = ((*(*bridge).driver).agp_type_to_mask_type)(bridge, type_);
    if mask_type != 0 { return -EINVAL; }
    if ((pg_start as usize).wrapping_add((*mem).page_count as usize)) > num_entries as usize { return -EINVAL; }
    let mut j = pg_start;
    while j < pg_start + (*mem).page_count as off_t {
        if !PGE_EMPTY(readl((*agp_bridge).gatt_table.offset(j as isize))) { return -EBUSY; }
        j += 1;
    }
    if !(*mem).is_flushed { global_cache_flush(); (*mem).is_flushed = true; }
    let mut i = 0;
    j = pg_start;
    while i < (*mem).page_count {
        let tmp = ((*(*bridge).driver).mask_memory)(agp_bridge, page_to_phys((*mem).pages.offset(i as isize)), mask_type);
        BUG_ON(tmp & 0xffffff0000000ffc_u64);
        let mut pte = ((tmp & 0x000000ff00000000_u64) >> 28) as u32;
        pte |= (tmp & 0x00000000fffff000_u64) as u32;
        pte |= GPTE_VALID | GPTE_COHERENT;
        writel(pte, (*agp_bridge).gatt_table.offset(j as isize));
        readl((*agp_bridge).gatt_table.offset(j as isize));
        i += 1; j += 1;
    }
    amd64_tlbflush(mem); 0
}

/* This hack alters the order element according to the size of a long. */
static mut amd64_aperture_sizes: [aper_size_info_32; 7] = [
    aper_size_info_32 { size:32, num_entries:8192, size_value: 3 + core::mem::size_of::<usize>() as u32 / 8, page_order:0 },
    aper_size_info_32 { size:64, num_entries:16384, size_value: 4 + core::mem::size_of::<usize>() as u32 / 8, page_order:1<<1 },
    aper_size_info_32 { size:128, num_entries:32768, size_value: 5 + core::mem::size_of::<usize>() as u32 / 8, page_order:1<<2 },
    aper_size_info_32 { size:256, num_entries:65536, size_value: 6 + core::mem::size_of::<usize>() as u32 / 8, page_order:(1<<1)|(1<<2) },
    aper_size_info_32 { size:512, num_entries:131072, size_value: 7 + core::mem::size_of::<usize>() as u32 / 8, page_order:1<<3 },
    aper_size_info_32 { size:1024, num_entries:262144, size_value: 8 + core::mem::size_of::<usize>() as u32 / 8, page_order:(1<<1)|(1<<3) },
    aper_size_info_32 { size:2048, num_entries:524288, size_value: 9 + core::mem::size_of::<usize>() as u32 / 8, page_order:(1<<2)|(1<<3) },
];

unsafe fn amd64_fetch_size() -> i32 {
    let dev = (*node_to_amd_nb(0)).misc;
    if dev.is_null() { return 0; }
    let mut temp = 0_u32;
    pci_read_config_dword(dev, AMD64_GARTAPERTURECTL, &mut temp);
    temp &= 0xe;
    for i in 0..(*(*agp_bridge).driver).num_aperture_sizes {
        if temp == amd64_aperture_sizes[i as usize].size_value {
            (*agp_bridge).previous_size = &mut amd64_aperture_sizes[i as usize] as *mut _ as *mut _;
            (*agp_bridge).current_size = (*agp_bridge).previous_size;
            (*agp_bridge).aperture_size_idx = i;
            return amd64_aperture_sizes[i as usize].size as i32;
        }
    }
    0
}

unsafe fn amd64_configure(hammer: *mut pci_dev, gatt_table: u64) -> u64 {
    let mut tmp = 0_u32;
    pci_read_config_dword(hammer, AMD64_GARTAPERTUREBASE, &mut tmp);
    let aperturebase = (tmp as u64) << 25;
    let aper_base = aperturebase & PCI_BASE_ADDRESS_MEM_MASK as u64;
    enable_gart_translation(hammer, gatt_table); aper_base
}

static amd_8151_sizes: [aper_size_info_32; 7] = [
    aper_size_info_32 {size:2048,num_entries:524288,size_value:9,page_order:0}, aper_size_info_32 {size:1024,num_entries:262144,size_value:8,page_order:0x400}, aper_size_info_32 {size:512,num_entries:131072,size_value:7,page_order:0x600}, aper_size_info_32 {size:256,num_entries:65536,size_value:6,page_order:0x700}, aper_size_info_32 {size:128,num_entries:32768,size_value:5,page_order:0x720}, aper_size_info_32 {size:64,num_entries:16384,size_value:4,page_order:0x730}, aper_size_info_32 {size:32,num_entries:8192,size_value:3,page_order:0x738},
];

unsafe fn amd_8151_configure() -> i32 {
    let gatt_bus = virt_to_phys((*agp_bridge).gatt_table_real);
    if !amd_nb_has_feature(AMD_NB_GART) { return 0; }
    for i in 0..amd_nb_num() { (*agp_bridge).gart_bus_addr = amd64_configure((*node_to_amd_nb(i)).misc, gatt_bus); }
    amd_flush_garts(); 0
}

unsafe fn amd64_cleanup() {
    if !amd_nb_has_feature(AMD_NB_GART) { return; }
    for i in 0..amd_nb_num() { let dev = (*node_to_amd_nb(i)).misc; let mut tmp=0; pci_read_config_dword(dev, AMD64_GARTAPERTURECTL, &mut tmp); tmp &= !GARTEN; pci_write_config_dword(dev, AMD64_GARTAPERTURECTL, tmp); }
}

/* Some basic sanity checks for the aperture. */
unsafe fn agp_aperture_valid(aper: u64, size: u32) -> i32 {
    if !aperture_valid(aper, size, 32*1024*1024) { return 0; }
    if aperture_resource.is_null() { aperture_resource = request_mem_region(aper, size, b"aperture\0".as_ptr() as *const _); if aperture_resource.is_null() { printk(KERN_ERR, b"Aperture conflicts with PCI mapping.\n\0".as_ptr()); return 0; } }
    1
}

unsafe fn fix_northbridge(nb: *mut pci_dev, agp: *mut pci_dev, cap: u16) -> i32 {
    let mut nb_order=0_u32; let mut nb_base=0_u32; pci_read_config_dword(nb, AMD64_GARTAPERTURECTL, &mut nb_order); nb_order=(nb_order>>1)&7; pci_read_config_dword(nb, AMD64_GARTAPERTUREBASE, &mut nb_base); let nb_aper=(nb_base as u64)<<25;
    let mut apsize=0_u16; pci_read_config_word(agp, cap+0x14, &mut apsize);
    if apsize==0xffff { return if agp_aperture_valid(nb_aper,(32*1024*1024)<<nb_order)==1 {0} else {-1}; }
    apsize &= 0xfff; if apsize&0xff != 0 { apsize |= 0xf00; }
    let mut order = 7 - hweight16(apsize) as i32; let aper = pci_bus_address(agp, AGP_APERTURE_BAR);
    if order >= 0 && aper + (32_u64 << (20 + order as u32)) > 0x100000000 { order=nb_order as i32; }
    if nb_order as i32 >= order && agp_aperture_valid(nb_aper,(32*1024*1024)<<nb_order)==1 { return 0; }
    if order < 0 || agp_aperture_valid(aper,(32*1024*1024)<<order as u32)==0 { return -1; }
    gart_set_size_and_enable(nb, order); pci_write_config_dword(nb, AMD64_GARTAPERTUREBASE, (aper>>25) as u32); 0
}

unsafe fn cache_nbs(pdev:*mut pci_dev, cap:u32)->i32 { if amd_nb_num()==0 || !amd_nb_has_feature(AMD_NB_GART) {return -ENODEV;} for i in 0..amd_nb_num() { let dev=(*node_to_amd_nb(i)).misc; if fix_northbridge(dev,pdev,cap as u16)<0 { dev_err(&(*dev).dev,b"no usable aperture found\0".as_ptr()); return -1; } } 0 }

unsafe fn amd8151_init(pdev:*mut pci_dev, bridge:*mut agp_bridge_data) { let revstring=match (*pdev).revision {0x01=>"A0",0x02=>"A1",0x11=>"B0",0x12=>"B1",0x13=>"B2",0x14=>"B3",_=>"??"}; dev_info(&(*pdev).dev,revstring.as_ptr()); if (*pdev).revision<0x13 {(*bridge).major_version=3;(*bridge).minor_version=0;} }

// The remaining driver registration and device-management declarations retain the
// source control flow; their kernel callback structure fields are supplied externally.
unsafe fn agp_amd64_probe(pdev:*mut pci_dev, _ent:*const pci_device_id)->i32 { if agp_bridges_found!=0{return -ENODEV;} let cap=pci_find_capability(pdev,PCI_CAP_ID_AGP); if cap==0{return -ENODEV;} let bridge=agp_alloc_bridge(); if bridge.is_null(){return -ENOMEM;} (*bridge).driver=&amd_8151_driver; (*bridge).dev=pdev; (*bridge).capndx=cap as u8; if cache_nbs(pdev,cap)<0 {agp_put_bridge(bridge);return -ENODEV;} pci_set_drvdata(pdev,bridge); let err=agp_add_bridge(bridge); if err<0{return err;} agp_bridges_found+=1; 0 }
unsafe fn agp_amd64_remove(pdev:*mut pci_dev) { let bridge=pci_get_drvdata(pdev); release_mem_region(virt_to_phys((*bridge).gatt_table_real),amd64_aperture_sizes[(*bridge).aperture_size_idx as usize].size); agp_remove_bridge(bridge); agp_put_bridge(bridge); agp_bridges_found-=1; }
unsafe fn agp_amd64_resume(dev:*mut device)->i32 { let pdev=to_pci_dev(dev); if (*pdev).vendor==PCI_VENDOR_ID_NVIDIA {nforce3_agp_init(pdev);} amd_8151_configure() }

// C module metadata, PCI ID table, PM operations, and registration macros are
// represented as external kernel declarations because their definitions are supplied by the kernel integration layer.
extern "C" { static mut amd_8151_driver: agp_bridge_driver; fn nforce3_agp_init(pdev:*mut pci_dev)->i32; fn pci_register_driver(driver:*mut pci_driver)->i32; fn pci_unregister_driver(driver:*mut pci_driver); }

#[no_mangle]
pub unsafe extern "C" fn agp_amd64_init() -> i32 { if agp_off{return -EINVAL;} let err=pci_register_driver(&mut agp_amd64_pci_driver); if err<0{return err;} if agp_bridges_found==0 && !agp_try_unsupported && !agp_try_unsupported_boot { pci_unregister_driver(&mut agp_amd64_pci_driver); return -ENODEV; } err }

static mut agp_amd64_pci_driver: pci_driver = pci_driver { name: b"agpgart-amd64\0".as_ptr(), ..pci_driver::ZERO };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
