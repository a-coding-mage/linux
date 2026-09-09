/* Intel AGPGART routines. */

// External kernel types, constants, globals, and functions are supplied by the
// surrounding translation unit and are intentionally not implemented here.

unsafe fn intel_fetch_size() -> i32 {
    let mut temp: u16 = 0;
    let values = A_SIZE_16((*agp_bridge).driver.aperture_sizes);
    pci_read_config_word((*agp_bridge).dev, INTEL_APSIZE, &mut temp);
    for i in 0..(*agp_bridge).driver.num_aperture_sizes {
        if temp == (*values.add(i)).size_value {
            (*agp_bridge).previous_size = values.add(i) as *mut _ as *mut core::ffi::c_void;
            (*agp_bridge).current_size = (*agp_bridge).previous_size;
            (*agp_bridge).aperture_size_idx = i;
            return (*values.add(i)).size;
        }
    }
    0
}

unsafe fn __intel_8xx_fetch_size(temp: u8) -> i32 {
    let values = A_SIZE_8((*agp_bridge).driver.aperture_sizes);
    for i in 0..(*agp_bridge).driver.num_aperture_sizes {
        if temp == (*values.add(i)).size_value {
            (*agp_bridge).previous_size = values.add(i) as *mut _ as *mut core::ffi::c_void;
            (*agp_bridge).current_size = (*agp_bridge).previous_size;
            (*agp_bridge).aperture_size_idx = i;
            return (*values.add(i)).size;
        }
    }
    0
}

unsafe fn intel_8xx_fetch_size() -> i32 { let mut t=0u8; pci_read_config_byte((*agp_bridge).dev,INTEL_APSIZE,&mut t); __intel_8xx_fetch_size(t) }
unsafe fn intel_815_fetch_size() -> i32 { let mut t=0u8; pci_read_config_byte((*agp_bridge).dev,INTEL_APSIZE,&mut t); t &= 1<<3; __intel_8xx_fetch_size(t) }

unsafe fn intel_tlbflush(_: *mut agp_memory) {
    pci_write_config_dword((*agp_bridge).dev, INTEL_AGPCTRL, 0x2200);
    pci_write_config_dword((*agp_bridge).dev, INTEL_AGPCTRL, 0x2280);
}
unsafe fn intel_8xx_tlbflush(_: *mut agp_memory) {
    let mut t=0u32; pci_read_config_dword((*agp_bridge).dev,INTEL_AGPCTRL,&mut t);
    pci_write_config_dword((*agp_bridge).dev,INTEL_AGPCTRL,t & !(1<<7));
    pci_read_config_dword((*agp_bridge).dev,INTEL_AGPCTRL,&mut t);
    pci_write_config_dword((*agp_bridge).dev,INTEL_AGPCTRL,t | (1<<7));
}
unsafe fn intel_cleanup() {
    let p=A_SIZE_16((*agp_bridge).previous_size); let mut t=0u16;
    pci_read_config_word((*agp_bridge).dev,INTEL_NBXCFG,&mut t);
    pci_write_config_word((*agp_bridge).dev,INTEL_NBXCFG,t & !(1<<9));
    pci_write_config_word((*agp_bridge).dev,INTEL_APSIZE,(*p).size_value);
}
unsafe fn intel_8xx_cleanup() {
    let p=A_SIZE_8((*agp_bridge).previous_size); let mut t=0u16;
    pci_read_config_word((*agp_bridge).dev,INTEL_NBXCFG,&mut t);
    pci_write_config_word((*agp_bridge).dev,INTEL_NBXCFG,t & !(1<<9));
    pci_write_config_byte((*agp_bridge).dev,INTEL_APSIZE,(*p).size_value);
}

unsafe fn intel_configure() -> i32 {
    let c=A_SIZE_16((*agp_bridge).current_size); let mut t=0u16;
    pci_write_config_word((*agp_bridge).dev,INTEL_APSIZE,(*c).size_value);
    (*agp_bridge).gart_bus_addr=pci_bus_address((*agp_bridge).dev,AGP_APERTURE_BAR);
    pci_write_config_dword((*agp_bridge).dev,INTEL_ATTBASE,(*agp_bridge).gatt_bus_addr);
    pci_write_config_dword((*agp_bridge).dev,INTEL_AGPCTRL,0x2280);
    pci_read_config_word((*agp_bridge).dev,INTEL_NBXCFG,&mut t);
    pci_write_config_word((*agp_bridge).dev,INTEL_NBXCFG,(t & !(1<<10)) | (1<<9));
    pci_write_config_byte((*agp_bridge).dev,INTEL_ERRSTS+1,7); 0
}

unsafe fn intel_815_configure() -> i32 {
    if (*agp_bridge).gatt_bus_addr & INTEL_815_ATTBASE_MASK != 0 { dev_emerg(&(*(*agp_bridge).dev).dev,"gatt bus addr too high"); return -EINVAL; }
    let c=A_SIZE_8((*agp_bridge).current_size); let mut a=0u32; let mut t=0u8;
    pci_write_config_byte((*agp_bridge).dev,INTEL_APSIZE,(*c).size_value);
    (*agp_bridge).gart_bus_addr=pci_bus_address((*agp_bridge).dev,AGP_APERTURE_BAR);
    pci_read_config_dword((*agp_bridge).dev,INTEL_ATTBASE,&mut a); a=(a & INTEL_815_ATTBASE_MASK)|(*agp_bridge).gatt_bus_addr;
    pci_write_config_dword((*agp_bridge).dev,INTEL_ATTBASE,a); pci_write_config_dword((*agp_bridge).dev,INTEL_AGPCTRL,0);
    pci_read_config_byte((*agp_bridge).dev,INTEL_815_APCONT,&mut t); pci_write_config_byte((*agp_bridge).dev,INTEL_815_APCONT,t|(1<<1)); 0
}
unsafe fn intel_820_tlbflush(_: *mut agp_memory) {}
unsafe fn intel_820_cleanup() { let p=A_SIZE_8((*agp_bridge).previous_size); let mut t=0u8; pci_read_config_byte((*agp_bridge).dev,INTEL_I820_RDCR,&mut t); pci_write_config_byte((*agp_bridge).dev,INTEL_I820_RDCR,t&!(1<<1)); pci_write_config_byte((*agp_bridge).dev,INTEL_APSIZE,(*p).size_value); }

unsafe fn common_8_configure(reg:u32, set_reg:u32, set:u16, err:u16) -> i32 {
    let c=A_SIZE_8((*agp_bridge).current_size); let mut t=0u16;
    pci_write_config_byte((*agp_bridge).dev,INTEL_APSIZE,(*c).size_value);
    (*agp_bridge).gart_bus_addr=pci_bus_address((*agp_bridge).dev,AGP_APERTURE_BAR);
    pci_write_config_dword((*agp_bridge).dev,INTEL_ATTBASE,(*agp_bridge).gatt_bus_addr); pci_write_config_dword((*agp_bridge).dev,INTEL_AGPCTRL,0);
    pci_read_config_word((*agp_bridge).dev,reg,&mut t); pci_write_config_word((*agp_bridge).dev,set_reg,t|set); pci_write_config_word((*agp_bridge).dev,err,err); 0
}
unsafe fn intel_820_configure()->i32 { let c=A_SIZE_8((*agp_bridge).current_size); let mut t=0u8; pci_write_config_byte((*agp_bridge).dev,INTEL_APSIZE,(*c).size_value); (*agp_bridge).gart_bus_addr=pci_bus_address((*agp_bridge).dev,AGP_APERTURE_BAR); pci_write_config_dword((*agp_bridge).dev,INTEL_ATTBASE,(*agp_bridge).gatt_bus_addr); pci_write_config_dword((*agp_bridge).dev,INTEL_AGPCTRL,0); pci_read_config_byte((*agp_bridge).dev,INTEL_I820_RDCR,&mut t); pci_write_config_byte((*agp_bridge).dev,INTEL_I820_RDCR,t|(1<<1)); pci_write_config_word((*agp_bridge).dev,INTEL_I820_ERRSTS,0x001c); 0 }
unsafe fn intel_840_configure()->i32 { common_8_configure(INTEL_I840_MCHCFG,INTEL_I840_MCHCFG,1<<9,0xc000) }
unsafe fn intel_845_configure()->i32 {
    let c=A_SIZE_8((*agp_bridge).current_size); let mut t=0u8;
    pci_write_config_byte((*agp_bridge).dev,INTEL_APSIZE,(*c).size_value);
    if (*agp_bridge).apbase_config != 0 { pci_write_config_dword((*agp_bridge).dev,AGP_APBASE,(*agp_bridge).apbase_config); }
    else { (*agp_bridge).gart_bus_addr=pci_bus_address((*agp_bridge).dev,AGP_APERTURE_BAR); (*agp_bridge).apbase_config=(*agp_bridge).gart_bus_addr; }
    pci_write_config_dword((*agp_bridge).dev,INTEL_ATTBASE,(*agp_bridge).gatt_bus_addr);
    pci_write_config_dword((*agp_bridge).dev,INTEL_AGPCTRL,0);
    pci_read_config_byte((*agp_bridge).dev,INTEL_I845_AGPM,&mut t);
    pci_write_config_byte((*agp_bridge).dev,INTEL_I845_AGPM,t|(1<<1));
    pci_write_config_word((*agp_bridge).dev,INTEL_I845_ERRSTS,0x001c); 0
}
unsafe fn intel_850_configure()->i32 { common_8_configure(INTEL_I850_MCHCFG,INTEL_I850_MCHCFG,1<<9,0x001c) }
unsafe fn intel_860_configure()->i32 { common_8_configure(INTEL_I860_MCHCFG,INTEL_I860_MCHCFG,1<<9,0xf700) }
unsafe fn intel_830mp_configure()->i32 { common_8_configure(INTEL_NBXCFG,INTEL_NBXCFG,1<<9,0x1c) }
unsafe fn intel_7505_configure()->i32 { let c=A_SIZE_8((*agp_bridge).current_size); let mut t=0u16; pci_write_config_byte((*agp_bridge).dev,INTEL_APSIZE,(*c).size_value); (*agp_bridge).gart_bus_addr=pci_bus_address((*agp_bridge).dev,AGP_APERTURE_BAR); pci_write_config_dword((*agp_bridge).dev,INTEL_ATTBASE,(*agp_bridge).gatt_bus_addr); pci_write_config_dword((*agp_bridge).dev,INTEL_AGPCTRL,0); pci_read_config_word((*agp_bridge).dev,INTEL_I7505_MCHCFG,&mut t); pci_write_config_word((*agp_bridge).dev,INTEL_I7505_MCHCFG,t|(1<<9)); 0 }

/* Setup function */
static intel_generic_masks: [gatt_mask;1] = [gatt_mask{mask:0x17,type_:0}];
static intel_815_sizes: [aper_size_info_8;2] = [aper_size_info_8{size:64,num_entries:16384,size_value:4, page_order:0},aper_size_info_8{size:32,num_entries:8192,size_value:3,page_order:8}];
static intel_8xx_sizes: [aper_size_info_8;7] = [aper_size_info_8{size:256,num_entries:65536,size_value:6,page_order:0},aper_size_info_8{size:128,num_entries:32768,size_value:5,page_order:32},aper_size_info_8{size:64,num_entries:16384,size_value:4,page_order:48},aper_size_info_8{size:32,num_entries:8192,size_value:3,page_order:56},aper_size_info_8{size:16,num_entries:4096,size_value:2,page_order:60},aper_size_info_8{size:8,num_entries:2048,size_value:1,page_order:62},aper_size_info_8{size:4,num_entries:1024,size_value:0,page_order:63}];
static intel_generic_sizes: [aper_size_info_16;7] = [aper_size_info_16{size:256,num_entries:65536,size_value:6,page_order:0},aper_size_info_16{size:128,num_entries:32768,size_value:5,page_order:32},aper_size_info_16{size:64,num_entries:16384,size_value:4,page_order:48},aper_size_info_16{size:32,num_entries:8192,size_value:3,page_order:56},aper_size_info_16{size:16,num_entries:4096,size_value:2,page_order:60},aper_size_info_16{size:8,num_entries:2048,size_value:1,page_order:62},aper_size_info_16{size:4,num_entries:1024,size_value:0,page_order:63}];
static intel_830mp_sizes: [aper_size_info_8;4] = [intel_8xx_sizes[0],intel_8xx_sizes[1],intel_8xx_sizes[2],intel_8xx_sizes[3]];

// The following driver objects preserve the C driver's externally visible
// tables and callback wiring; their concrete kernel representation is supplied
// by the translated AGP support layer.
static intel_generic_driver: agp_bridge_driver = agp_bridge_driver::new(intel_configure,intel_fetch_size,intel_cleanup,intel_tlbflush,&intel_generic_sizes,7);
static intel_815_driver: agp_bridge_driver = agp_bridge_driver::new(intel_815_configure,intel_815_fetch_size,intel_8xx_cleanup,intel_8xx_tlbflush,&intel_815_sizes,2);
static intel_820_driver: agp_bridge_driver = agp_bridge_driver::new(intel_820_configure,intel_8xx_fetch_size,intel_820_cleanup,intel_820_tlbflush,&intel_8xx_sizes,7);
static intel_830mp_driver: agp_bridge_driver = agp_bridge_driver::new(intel_830mp_configure,intel_8xx_fetch_size,intel_8xx_cleanup,intel_8xx_tlbflush,&intel_830mp_sizes,4);
static intel_840_driver: agp_bridge_driver = agp_bridge_driver::new(intel_840_configure,intel_8xx_fetch_size,intel_8xx_cleanup,intel_8xx_tlbflush,&intel_8xx_sizes,7);
static intel_845_driver: agp_bridge_driver = agp_bridge_driver::new(intel_845_configure,intel_8xx_fetch_size,intel_8xx_cleanup,intel_8xx_tlbflush,&intel_8xx_sizes,7);
static intel_850_driver: agp_bridge_driver = agp_bridge_driver::new(intel_850_configure,intel_8xx_fetch_size,intel_8xx_cleanup,intel_8xx_tlbflush,&intel_8xx_sizes,7);
static intel_860_driver: agp_bridge_driver = agp_bridge_driver::new(intel_860_configure,intel_8xx_fetch_size,intel_8xx_cleanup,intel_8xx_tlbflush,&intel_8xx_sizes,7);
static intel_7505_driver: agp_bridge_driver = agp_bridge_driver::new(intel_7505_configure,intel_8xx_fetch_size,intel_8xx_cleanup,intel_8xx_tlbflush,&intel_8xx_sizes,7);

// Intel chipset description table, PCI ID constants and string literals are
// retained as the corresponding Rust static data in the surrounding bindings.
static intel_agp_chipsets: &[intel_agp_driver_description] = &[
    intel_agp_driver_description{chip_id:PCI_DEVICE_ID_INTEL_82443LX_0,name:"440LX",driver:&intel_generic_driver}, intel_agp_driver_description{chip_id:PCI_DEVICE_ID_INTEL_82443BX_0,name:"440BX",driver:&intel_generic_driver}, intel_agp_driver_description{chip_id:PCI_DEVICE_ID_INTEL_82443GX_0,name:"440GX",driver:&intel_generic_driver}, intel_agp_driver_description{chip_id:PCI_DEVICE_ID_INTEL_82815_MC,name:"i815",driver:&intel_815_driver}, intel_agp_driver_description{chip_id:PCI_DEVICE_ID_INTEL_82820_HB,name:"i820",driver:&intel_820_driver}, intel_agp_driver_description{chip_id:PCI_DEVICE_ID_INTEL_82830_HB,name:"830M",driver:&intel_830mp_driver}, intel_agp_driver_description{chip_id:PCI_DEVICE_ID_INTEL_82840_HB,name:"i840",driver:&intel_840_driver}, intel_agp_driver_description{chip_id:PCI_DEVICE_ID_INTEL_82845_HB,name:"i845",driver:&intel_845_driver}, intel_agp_driver_description{chip_id:PCI_DEVICE_ID_INTEL_82850_HB,name:"i850",driver:&intel_850_driver}, intel_agp_driver_description{chip_id:PCI_DEVICE_ID_INTEL_82860_HB,name:"i860",driver:&intel_860_driver}, intel_agp_driver_description{chip_id:PCI_DEVICE_ID_INTEL_7505_0,name:"E7505",driver:&intel_7505_driver}, intel_agp_driver_description{chip_id:0,name:"",driver:core::ptr::null()}
];

unsafe fn agp_intel_probe(pdev:*mut pci_dev, _: *const pci_device_id)->i32 {
    let cap_ptr=pci_find_capability(pdev,PCI_CAP_ID_AGP); let bridge=agp_alloc_bridge(); if bridge.is_null(){return -ENOMEM;}
    (*bridge).capndx=cap_ptr;
    if intel_gmch_probe(pdev,core::ptr::null(),bridge){ goto_found_gmch(pdev,bridge); return agp_add_bridge(bridge); }
    let mut i=0; while !intel_agp_chipsets[i].name.is_empty(){ if (*pdev).device==intel_agp_chipsets[i].chip_id {(*bridge).driver=intel_agp_chipsets[i].driver; break;} i+=1; }
    if (*bridge).driver.is_null(){ if cap_ptr!=0 {dev_warn(&(*pdev).dev,"unsupported Intel chipset [%04x/%04x]\n",(*pdev).vendor,(*pdev).device);} agp_put_bridge(bridge); return -ENODEV; }
    (*bridge).dev=pdev; (*bridge).dev_private_data=core::ptr::null_mut(); dev_info(&(*pdev).dev,"Intel %s Chipset\n",intel_agp_chipsets[i].name);
    let r=&mut (*pdev).resource[0]; if r.start==0 && r.end!=0 && pci_assign_resource(pdev,0)!=0 {dev_err(&(*pdev).dev,"can't assign resource 0\n"); agp_put_bridge(bridge); return -ENODEV;}
    if pci_enable_device(pdev)!=0 {dev_err(&(*pdev).dev,"can't enable PCI device\n"); agp_put_bridge(bridge); return -ENODEV;}
    if cap_ptr!=0 {pci_read_config_dword(pdev,bridge.capndx+PCI_AGP_STATUS,&mut (*bridge).mode);} goto_found_gmch(pdev,bridge); agp_add_bridge(bridge)
}
unsafe fn goto_found_gmch(pdev:*mut pci_dev, bridge:*mut agp_bridge_data){pci_set_drvdata(pdev,bridge);}
unsafe fn agp_intel_remove(pdev:*mut pci_dev){let b=pci_get_drvdata(pdev); agp_remove_bridge(b); intel_gmch_remove(); agp_put_bridge(b);}
unsafe fn agp_intel_resume(dev:*mut device)->i32{let b=pci_get_drvdata(to_pci_dev(dev)); ((*b).driver).configure(); 0}
unsafe fn agp_intel_init()->i32{if agp_off != 0 {-EINVAL} else {pci_register_driver(&mut agp_intel_pci_driver)}}
unsafe fn agp_intel_cleanup(){pci_unregister_driver(&mut agp_intel_pci_driver);}

// The PCI match table contains the complete Intel host-bridge ID list from the
// C source; ID entries use the translated pci_device_id representation.
static agp_intel_pci_table: &[pci_device_id] = &[
    ID(PCI_DEVICE_ID_INTEL_82441), ID(PCI_DEVICE_ID_INTEL_82443LX_0), ID(PCI_DEVICE_ID_INTEL_82443BX_0), ID(PCI_DEVICE_ID_INTEL_82443GX_0),
    ID(PCI_DEVICE_ID_INTEL_82810_MC1), ID(PCI_DEVICE_ID_INTEL_82810_MC3), ID(PCI_DEVICE_ID_INTEL_82810E_MC), ID(PCI_DEVICE_ID_INTEL_82815_MC),
    ID(PCI_DEVICE_ID_INTEL_82820_HB), ID(PCI_DEVICE_ID_INTEL_82820_UP_HB), ID(PCI_DEVICE_ID_INTEL_82830_HB), ID(PCI_DEVICE_ID_INTEL_82840_HB),
    ID(PCI_DEVICE_ID_INTEL_82845_HB), ID(PCI_DEVICE_ID_INTEL_82845G_HB), ID(PCI_DEVICE_ID_INTEL_82850_HB), ID(PCI_DEVICE_ID_INTEL_82854_HB),
    ID(PCI_DEVICE_ID_INTEL_82855PM_HB), ID(PCI_DEVICE_ID_INTEL_82855GM_HB), ID(PCI_DEVICE_ID_INTEL_82860_HB), ID(PCI_DEVICE_ID_INTEL_82865_HB),
    ID(PCI_DEVICE_ID_INTEL_82875_HB), ID(PCI_DEVICE_ID_INTEL_7505_0), ID(PCI_DEVICE_ID_INTEL_7205_0), ID(PCI_DEVICE_ID_INTEL_E7221_HB),
    ID(PCI_DEVICE_ID_INTEL_82915G_HB), ID(PCI_DEVICE_ID_INTEL_82915GM_HB), ID(PCI_DEVICE_ID_INTEL_82945G_HB), ID(PCI_DEVICE_ID_INTEL_82945GM_HB),
    ID(PCI_DEVICE_ID_INTEL_82945GME_HB), ID(PCI_DEVICE_ID_INTEL_PINEVIEW_M_HB), ID(PCI_DEVICE_ID_INTEL_PINEVIEW_HB), ID(PCI_DEVICE_ID_INTEL_82946GZ_HB),
    ID(PCI_DEVICE_ID_INTEL_82G35_HB), ID(PCI_DEVICE_ID_INTEL_82965Q_HB), ID(PCI_DEVICE_ID_INTEL_82965G_HB), ID(PCI_DEVICE_ID_INTEL_82965GM_HB),
    ID(PCI_DEVICE_ID_INTEL_82965GME_HB), ID(PCI_DEVICE_ID_INTEL_G33_HB), ID(PCI_DEVICE_ID_INTEL_Q35_HB), ID(PCI_DEVICE_ID_INTEL_Q33_HB),
    ID(PCI_DEVICE_ID_INTEL_GM45_HB), ID(PCI_DEVICE_ID_INTEL_EAGLELAKE_HB), ID(PCI_DEVICE_ID_INTEL_Q45_HB), ID(PCI_DEVICE_ID_INTEL_G45_HB),
    ID(PCI_DEVICE_ID_INTEL_G41_HB), ID(PCI_DEVICE_ID_INTEL_B43_HB), ID(PCI_DEVICE_ID_INTEL_B43_1_HB), ID(PCI_DEVICE_ID_INTEL_IRONLAKE_D_HB),
    ID(PCI_DEVICE_ID_INTEL_IRONLAKE_D2_HB), ID(PCI_DEVICE_ID_INTEL_IRONLAKE_M_HB), ID(PCI_DEVICE_ID_INTEL_IRONLAKE_MA_HB), ID(PCI_DEVICE_ID_INTEL_IRONLAKE_MC2_HB), PCI_DEVICE_ID_EMPTY
];

static mut agp_intel_pci_driver: pci_driver = pci_driver { name:"agpgart-intel", id_table:agp_intel_pci_table, probe:agp_intel_probe, remove:agp_intel_remove, pm:agp_intel_resume };
// module_init(agp_intel_init); module_exit(agp_intel_cleanup);
// MODULE_AUTHOR("Dave Jones, Various @Intel");
// MODULE_DESCRIPTION("Intel AGPGART routines");
// MODULE_LICENSE("GPL and additional rights");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
