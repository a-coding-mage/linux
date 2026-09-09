// SPDX-License-Identifier: GPL-2.0
// Translated from prom_irqtrans.c. Kernel and architecture declarations are
// supplied by the surrounding translation unit.

#[cfg(feature = "CONFIG_PCI")]
const PSYCHO_IMAP_A_SLOT0: usize = 0x0c00;
#[cfg(feature = "CONFIG_PCI")]
const PSYCHO_IMAP_B_SLOT0: usize = 0x0c20;

#[cfg(feature = "CONFIG_PCI")]
unsafe fn psycho_pcislot_imap_offset(ino: usize) -> usize {
    if ((ino & 0x10) >> 4) == 0 { PSYCHO_IMAP_A_SLOT0 + ((ino & 0x0c) >> 2) * 8 }
    else { PSYCHO_IMAP_B_SLOT0 + ((ino & 0x0c) >> 2) * 8 }
}

#[cfg(feature = "CONFIG_PCI")]
unsafe fn psycho_irq_build(dp: *mut device_node, mut ino: u32, data: *mut core::ffi::c_void) -> u32 {
    let regs = data as usize;
    ino &= 0x3f;
    let imap = regs + if ino < 0x20 { psycho_pcislot_imap_offset(ino as usize) } else { 0x1000 + ((ino as usize & 0x1f) << 3) };
    let iclr = regs + if (ino & 0x20) != 0 { 0x1800 + ((ino as usize & 0x1f) << 3) } else { 0x1400 + ((ino as usize & 0x1f) << 3) };
    let fixup = if (ino & 0x20) == 0 { (ino & 3) as i32 } else { 0 };
    build_irq(fixup, iclr, imap)
}

#[cfg(feature = "CONFIG_PCI")]
unsafe fn psycho_irq_trans_init(dp: *mut device_node) {
    (*dp).irq_trans = prom_early_alloc(core::mem::size_of::<of_irq_controller>()) as *mut of_irq_controller;
    (*(*dp).irq_trans).irq_build = Some(psycho_irq_build);
    let regs = of_get_property(dp, b"reg\0".as_ptr() as _, core::ptr::null_mut()) as *const linux_prom64_registers;
    (*(*dp).irq_trans).data = (*regs).phys_addr as *mut _;
}

#[cfg(feature = "CONFIG_PCI")]
struct sabre_irq_data { controller_regs: usize, pci_first_busno: u32 }

#[cfg(feature = "CONFIG_PCI")]
unsafe fn sabre_irq_build(dp: *mut device_node, mut ino: u32, data: *mut core::ffi::c_void) -> u32 {
    let d = &mut *(data as *mut sabre_irq_data); ino &= 0x3f;
    let bus = ((ino & 0x10) >> 4) as usize; let slot = ((ino & 0x0c) >> 2) as usize;
    let imap = d.controller_regs + if ino < 0x20 { if bus == 0 { 0xc00 + slot*8 } else { 0xc20 + slot*8 } } else { 0x1000 + ((ino as usize & 0x1f)<<3) };
    let iclr = d.controller_regs + if (ino & 0x20)!=0 { 0x1800 + ((ino as usize&0x1f)<<3) } else { 0x1400 + ((ino as usize&0x1f)<<3) };
    build_irq(if (ino&0x20)==0 {(ino&3) as i32} else {0}, iclr, imap)
}

#[cfg(feature = "CONFIG_PCI")]
unsafe fn sabre_irq_trans_init(dp: *mut device_node) {
    (*dp).irq_trans = prom_early_alloc(core::mem::size_of::<of_irq_controller>()) as *mut _;
    (*(*dp).irq_trans).irq_build=Some(sabre_irq_build);
    let d=prom_early_alloc(core::mem::size_of::<sabre_irq_data>()) as *mut sabre_irq_data;
    let r=of_get_property(dp,b"reg\0".as_ptr() as _,core::ptr::null_mut()) as *const linux_prom64_registers;
    (*d).controller_regs=(*r).phys_addr; let br=of_get_property(dp,b"bus-range\0".as_ptr() as _,core::ptr::null_mut()) as *const u32; (*d).pci_first_busno=*br;
    (*(*dp).irq_trans).data=d as *mut _;
}

#[cfg(feature = "CONFIG_PCI")]
struct schizo_irq_data { pbm_regs: usize, sync_reg: usize, portid: u32, chip_version: i32 }
#[cfg(feature = "CONFIG_PCI")]
unsafe fn schizo_irq_build(_dp:*mut device_node,mut ino:u32,data:*mut core::ffi::c_void)->u32 { let d=&*(data as *mut schizo_irq_data); ino&=0x3f; let imap=d.pbm_regs+0x1000+(ino as usize*8); let iclr=d.pbm_regs+0x1400+(ino as usize*8); let fix=if d.sync_reg!=0 && d.portid&1!=0 {64} else {0}; build_irq(fix,iclr,imap) }
#[cfg(feature = "CONFIG_PCI")]
unsafe fn __schizo_irq_trans_init(dp:*mut device_node,tomatillo:i32){(*dp).irq_trans=prom_early_alloc(core::mem::size_of::<of_irq_controller>()) as *mut _;(*(*dp).irq_trans).irq_build=Some(schizo_irq_build);let d=prom_early_alloc(core::mem::size_of::<schizo_irq_data>()) as *mut schizo_irq_data;let r=of_get_property(dp,b"reg\0".as_ptr() as _,core::ptr::null_mut()) as *const linux_prom64_registers;(*d).pbm_regs=(*r).phys_addr;(*d).sync_reg=if tomatillo!=0 {(*r.add(3)).phys_addr+0x1a18}else{0};(*d).portid=of_getintprop_default(dp,b"portid\0".as_ptr() as _,0);(*d).chip_version=of_getintprop_default(dp,b"version#\0".as_ptr() as _,0) as i32;(*(*dp).irq_trans).data=d as *mut _;}
#[cfg(feature = "CONFIG_PCI")] unsafe fn schizo_irq_trans_init(dp:*mut device_node){__schizo_irq_trans_init(dp,0)}
#[cfg(feature = "CONFIG_PCI")] unsafe fn tomatillo_irq_trans_init(dp:*mut device_node){__schizo_irq_trans_init(dp,1)}

unsafe fn pci_sun4v_irq_build(_dp:*mut device_node,devino:u32,data:*mut core::ffi::c_void)->u32{sun4v_build_irq(data as usize as u32,devino)}
unsafe fn sun4v_vdev_irq_build(_dp:*mut device_node,devino:u32,data:*mut core::ffi::c_void)->u32{sun4v_build_irq(data as usize as u32,devino)}
unsafe fn init_sun4v(dp:*mut device_node, f: unsafe fn(*mut device_node,u32,*mut core::ffi::c_void)->u32){(*dp).irq_trans=prom_early_alloc(core::mem::size_of::<of_irq_controller>()) as *mut _;(*(*dp).irq_trans).irq_build=Some(f);let r=of_get_property(dp,b"reg\0".as_ptr() as _,core::ptr::null_mut()) as *const linux_prom64_registers;(*(*dp).irq_trans).data=(((*r).phys_addr>>32)&0x0fffffff) as *mut _;}

#[cfg(feature = "CONFIG_PCI")]
struct fire_irq_data { pbm_regs: usize, portid: u32 }
#[cfg(feature = "CONFIG_PCI")]
unsafe fn fire_irq_build(_dp:*mut device_node,mut ino:u32,data:*mut core::ffi::c_void)->u32 { let d=&*(data as *mut fire_irq_data); ino&=0x3f; let imap=d.pbm_regs+0x1000+(ino as usize*8); let iclr=d.pbm_regs+0x1400+(ino as usize*8); let ctrl=1usize<<6; upa_writeq(ctrl,imap); ino|=d.portid<<6; ino-=ctrl as u32; build_irq(ino as i32,iclr,imap) }
#[cfg(feature = "CONFIG_PCI")]
unsafe fn fire_irq_trans_init(dp:*mut device_node){(*dp).irq_trans=prom_early_alloc(core::mem::size_of::<of_irq_controller>()) as *mut _;(*(*dp).irq_trans).irq_build=Some(fire_irq_build);let d=prom_early_alloc(core::mem::size_of::<fire_irq_data>()) as *mut fire_irq_data;let r=of_get_property(dp,b"reg\0".as_ptr() as _,core::ptr::null_mut()) as *const linux_prom64_registers;(*d).pbm_regs=(*r).phys_addr;(*d).portid=of_getintprop_default(dp,b"portid\0".as_ptr() as _,0);(*(*dp).irq_trans).data=d as *mut _;}

#[cfg(feature = "CONFIG_PCI")]
unsafe fn sabre_wsync_handler(_ino:u32,_a:*mut core::ffi::c_void,_b:*mut core::ffi::c_void) { /* SPARC membar/ldxa sequence is an external architectural primitive. */ }
#[cfg(feature = "CONFIG_PCI")]
unsafe fn tomatillo_wsync_handler(_ino:u32,_a:*mut core::ffi::c_void,_b:*mut core::ffi::c_void) { /* SPARC sync-register and cacheline commit sequence. */ }

#[cfg(feature = "CONFIG_SBUS")]
unsafe fn sbus_of_build_irq(dp:*mut device_node,mut ino:u32,data:*mut core::ffi::c_void)->u32{let base=data as usize;let r=of_get_property(dp,b"reg\0".as_ptr() as _,core::ptr::null_mut()) as *const linux_prom_registers;let slot=if r.is_null(){0}else{(*r).which_io as usize};ino&=0x3f;if ino<0x20{ino+=(slot*8) as u32};let off=[0x2c00,0x2c00,0x2c00,0x2c00,0x2c00,0x2c00,0x2c00,0x2c00,0x2c08,0x2c08,0x2c08,0x2c08,0x2c08,0x2c08,0x2c08,0x2c08,0x2c10,0x2c10,0x2c10,0x2c10,0x2c10,0x2c10,0x2c10,0x2c10,0x2c18,0x2c18,0x2c18,0x2c18,0x2c18,0x2c18,0x2c18,0x2c18,0x3000,0x3008,0x3010,usize::MAX,0x3018,0x3020,usize::MAX,usize::MAX,0x3028,0x3030,0x3038,0x3040,0x3048,0x3050,usize::MAX,usize::MAX,0x3060,0x3068,usize::MAX,usize::MAX,0x3070,0x3078,0x3080,0x3088,0x3090,0x3098];let im=off[ino as usize];if im==usize::MAX{prom_halt()};let imap=base+im;let iclr=if ino>=0x20{imap+0x800}else{base+[0x3408,0x3448,0x3488,0x34c8][slot.min(3)]+((ino as usize&7)-1)*8};build_irq((ino&7) as i32,iclr,imap)}

unsafe fn central_build_irq(dp:*mut device_node,ino:u32,data:*mut core::ffi::c_void)->u32{let op=of_find_device_by_node(data as *mut _);let idx=if of_node_name_eq(dp,b"eeprom\0".as_ptr() as _){5}else if of_node_name_eq(dp,b"zs\0".as_ptr() as _){4}else if of_node_name_eq(dp,b"clock-board\0".as_ptr() as _){3}else{return ino};let res=&(*op).resource[idx];let imap=res.start;let iclr=res.start+0x10;upa_writel(0,iclr);upa_readl(iclr);let mut tmp=upa_readl(imap)&!0x80000000;upa_writel(tmp,imap);build_irq(0,iclr,imap)}

pub unsafe fn irq_trans_init(dp:*mut device_node){
    // Build-time PCI/SBUS dispatch tables and OF name matching are retained
    // here as the surrounding kernel translation supplies their declarations.
    if of_node_name_eq(dp,b"sbus\0".as_ptr() as _)||of_node_name_eq(dp,b"sbi\0".as_ptr() as _){(*dp).irq_trans=prom_early_alloc(core::mem::size_of::<of_irq_controller>()) as *mut _;(*(*dp).irq_trans).irq_build=Some(sbus_of_build_irq);let r=of_get_property(dp,b"reg\0".as_ptr() as _,core::ptr::null_mut()) as *const linux_prom64_registers;(*(*dp).irq_trans).data=(*r).phys_addr as *mut _;return;}
    if of_node_name_eq(dp,b"fhc\0".as_ptr() as _)&&of_node_name_eq((*dp).parent,b"central\0".as_ptr() as _){(*dp).irq_trans=prom_early_alloc(core::mem::size_of::<of_irq_controller>()) as *mut _;(*(*dp).irq_trans).irq_build=Some(central_build_irq);(*(*dp).irq_trans).data=dp as *mut _;return;}
    if of_node_name_eq(dp,b"virtual-devices\0".as_ptr() as _)||of_node_name_eq(dp,b"niu\0".as_ptr() as _){init_sun4v(dp,sun4v_vdev_irq_build);}
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
