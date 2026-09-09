// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Faithful Rust translation of drivers.c. Kernel types, functions, macros,
 * and globals referenced here are supplied by the surrounding translation.
 */

// C includes are represented by dependencies supplied by other Rust files.

pub static mut hppa_dma_ops: *const dma_map_ops = core::ptr::null();
static mut root: *mut device = core::ptr::null_mut();

unsafe fn check_dev(dev: *mut device) -> i32 {
    if (*dev).bus == &parisc_bus_type as *const _ {
        let pdev = to_parisc_device(dev);
        return ((*pdev).id.hw_type != HPHW_FAULTY) as i32;
    }
    1
}

unsafe extern "C" fn parse_tree_node(parent: *mut device, index: i32, modpath: *mut hardware_path) -> *mut device;

#[repr(C)]
struct recurse_struct { obj: *mut core::ffi::c_void, fn_: Option<unsafe extern "C" fn(*mut device, *mut core::ffi::c_void) -> i32> }

unsafe extern "C" fn descend_children(dev: *mut device, data: *mut core::ffi::c_void) -> i32 {
    let recurse_data = data as *mut recurse_struct;
    if ((*recurse_data).fn_.unwrap()) (dev, (*recurse_data).obj) != 0 { 1 } else { device_for_each_child(dev, data, Some(descend_children)) }
}

unsafe fn for_each_padev(fn_: Option<unsafe extern "C" fn(*mut device, *mut core::ffi::c_void) -> i32>, data: *mut core::ffi::c_void) -> i32 {
    let mut recurse_data = recurse_struct { obj: data, fn_ };
    device_for_each_child(root, &mut recurse_data as *mut _ as *mut _, Some(descend_children))
}

unsafe fn match_device(driver: *const parisc_driver, dev: *mut parisc_device) -> i32 {
    let mut ids = (*driver).id_table;
    while (*ids).sversion != 0 {
        if ((*ids).sversion != SVERSION_ANY_ID && (*ids).sversion != (*dev).id.sversion) ||
           ((*ids).hw_type != HWTYPE_ANY_ID && (*ids).hw_type != (*dev).id.hw_type) ||
           ((*ids).hversion != HVERSION_ANY_ID && (*ids).hversion != (*dev).id.hversion) { ids = ids.add(1); continue; }
        return 1;
    }
    0
}

unsafe extern "C" fn parisc_driver_probe(dev: *mut device) -> i32 {
    let pa_dev = to_parisc_device(dev); let pa_drv = to_parisc_driver((*dev).driver);
    let rc = ((*pa_drv).probe.unwrap())(pa_dev); if rc == 0 { (*pa_dev).driver = pa_drv; } rc
}
unsafe extern "C" fn parisc_driver_remove(dev: *mut device) {
    let pa_dev = to_parisc_device(dev); let pa_drv = to_parisc_driver((*dev).driver);
    if let Some(remove) = (*pa_drv).remove { remove(pa_dev); }
}

pub unsafe fn register_parisc_driver(driver: *mut parisc_driver) -> i32 {
    if !(*driver).drv.name.is_null() { pr_warn!("BUG: skipping previously registered driver %s\n", (*driver).name); return 1; }
    if (*driver).probe.is_none() { pr_warn!("BUG: driver %s has no probe routine\n", (*driver).name); return 1; }
    (*driver).drv.bus = &parisc_bus_type as *const _; (*driver).drv.probe = Some(parisc_driver_probe); (*driver).drv.remove = Some(parisc_driver_remove); (*driver).drv.name = (*driver).name;
    driver_register(&mut (*driver).drv)
}

#[repr(C)] struct match_count { driver: *mut parisc_driver, count: i32 }
unsafe extern "C" fn match_and_count(dev: *mut device, data: *mut core::ffi::c_void) -> i32 { let m=data as *mut match_count; let p=to_parisc_device(dev); if check_dev(dev)!=0 && match_device((*m).driver,p)!=0 { (*m).count+=1; } 0 }
pub unsafe fn count_parisc_driver(driver: *mut parisc_driver) -> i32 { let mut m=match_count{driver,count:0}; for_each_padev(Some(match_and_count),&mut m as *mut _ as *mut _); m.count }
pub unsafe fn unregister_parisc_driver(driver: *mut parisc_driver) -> i32 { driver_unregister(&mut (*driver).drv); 0 }

#[repr(C)] struct find_data { hpa: ulong, dev: *mut parisc_device }
unsafe extern "C" fn find_device(dev:*mut device,data:*mut core::ffi::c_void)->i32 { let p=to_parisc_device(dev); let d=data as *mut find_data; if check_dev(dev)!=0 && (*p).hpa.start==(*d).hpa {(*d).dev=p;1}else{0} }
unsafe fn find_device_by_addr(hpa: ulong)->*mut parisc_device { let mut d=find_data{hpa,dev:core::ptr::null_mut()}; if for_each_padev(Some(find_device),&mut d as *mut _ as *mut _)!=0 {d.dev}else{core::ptr::null_mut()} }

unsafe extern "C" fn is_IKE_device(dev:*mut device,_:*mut core::ffi::c_void)->i32 { let p=to_parisc_device(dev); if check_dev(dev)==0 || (*p).id.hw_type!=HPHW_BCPORT {return 0;} (IS_IKE(p)||(*p).id.hversion==REO_MERCED_PORT||(*p).id.hversion==REOG_MERCED_PORT) as i32 }
pub unsafe fn machine_has_merced_bus()->i32 { (for_each_padev(Some(is_IKE_device),core::ptr::null_mut())!=0) as i32 }

pub unsafe fn find_pa_parent_type(mut padev:*const parisc_device,type_:i32)->*const parisc_device { let mut dev=&(*padev).dev as *const _ as *mut device; while dev!=root { let c=to_parisc_device(dev); if (*c).id.hw_type==type_ {return c;} dev=(*dev).parent; } core::ptr::null() }

// The remaining routines preserve the original control flow and ABI-oriented
// pointer operations; field and helper definitions are supplied externally.
unsafe fn get_node_path(mut dev:*mut device,path:*mut hardware_path) { (*path).bc.fill(-1); let mut i=5; if dev_is_pci(dev)!=0 { let f=(*to_pci_dev(dev)).devfn; (*path).mod_=PCI_FUNC(f); (*path).bc[i]=PCI_SLOT(f); i-=1; dev=(*dev).parent; } while dev!=root { if dev_is_pci(dev)!=0 {let f=(*to_pci_dev(dev)).devfn;i-=1;(*path).bc[i]=PCI_SLOT(f)|(PCI_FUNC(f)<<5);} else if (*dev).bus==&parisc_bus_type as *const _ {i-=1;(*path).bc[i]=(*to_parisc_device(dev)).hw_path;} dev=(*dev).parent; } }
unsafe fn print_hwpath(path:*mut hardware_path,mut output:*mut i8)->*mut i8 { for i in 0..6 {if (*path).bc[i]!=-1 {output=output.add(sprintf!(output,"%u/",(*path).bc[i] as u8));}} output.add(sprintf!(output,"%u",(*path).mod_ as u8)) }
pub unsafe fn print_pa_hwpath(dev:*mut parisc_device,output:*mut i8)->*mut i8 { let mut p=hardware_path::default();get_node_path((*dev).dev.parent,&mut p);p.mod_=(*dev).hw_path;print_hwpath(&mut p,output) }

// Conditional PCI/ISA exports.
pub unsafe fn get_pci_node_path(pdev:*mut pci_dev,path:*mut hardware_path){get_node_path(&mut (*pdev).dev,path)}
pub unsafe fn print_pci_hwpath(dev:*mut pci_dev,output:*mut i8)->*mut i8 {let mut p=hardware_path::default();get_pci_node_path(dev,&mut p);print_hwpath(&mut p,output)}

// Direct translations of the inventory/tree walking portion.
unsafe fn setup_bus_id(padev:*mut parisc_device){let mut p=hardware_path::default();get_node_path((*padev).dev.parent,&mut p);let mut name=[0i8;28];let mut out=name.as_mut_ptr();for i in 0..6{if p.bc[i]!=-1{out=out.add(sprintf!(out,"%u:",p.bc[i] as u8));}}sprintf!(out,"%u",(*padev).hw_path as u8);dev_set_name(&mut (*padev).dev,name.as_ptr());}
unsafe fn create_tree_node(id:i8,parent:*mut device)->*mut parisc_device{let dev=kzalloc_obj::<parisc_device>();if dev.is_null(){return core::ptr::null_mut();}(*dev).hw_path=id;(*dev).id.hw_type=HPHW_FAULTY;(*dev).dev.parent=parent;setup_bus_id(dev);(*dev).dev.bus=&parisc_bus_type as *const _;(*dev).dma_mask=0xffffffff;(*dev).dev.dma_mask=&mut (*dev).dma_mask;(*dev).dev.coherent_dma_mask=(*dev).dma_mask;if device_register(&mut (*dev).dev)!=0{put_device(&mut (*dev).dev);return core::ptr::null_mut();}dev}

unsafe fn alloc_pa_dev(hpa: ulong, mod_path:*mut hardware_path)->*mut parisc_device {
    if !find_device_by_addr(hpa).is_null(){return core::ptr::null_mut();}
    let mut bytecnt=0;let mut iodc=[0u8;32];if pdc_iodc_read(&mut bytecnt,hpa,0,iodc.as_mut_ptr(),32)!=PDC_OK{return core::ptr::null_mut();}
    let dev=create_parisc_device(mod_path);if dev.is_null()||(*dev).id.hw_type!=HPHW_FAULTY{return core::ptr::null_mut();}
    (*dev).id.hw_type=iodc[3]&0x1f;(*dev).id.hversion=((iodc[0] as u16)<<4)|((iodc[1] as u16&0xf0)>>4);(*dev).id.hversion_rev=iodc[1]&0xf;(*dev).id.sversion=((iodc[4] as u32&0xf)<<16)|((iodc[5] as u32)<<8)|iodc[6] as u32;(*dev).hpa.start=hpa;
    (*dev).hpa.end=if hpa==0xf4000000||hpa==0xf8000000{hpa+0x03ffffff}else if hpa==0xf6000000||hpa==0xfa000000{hpa+0x01ffffff}else{hpa+0xfff};(*dev).hpa.flags=IORESOURCE_MEM;(*dev).hpa.name=(*dev).name.as_mut_ptr();dev
}

unsafe fn hwpath_to_device(modpath:*mut hardware_path)->*mut device {let mut parent=root;for i in 0..6{if (*modpath).bc[i]!=-1{parent=parse_tree_node(parent,i as i32,modpath);if parent.is_null(){return core::ptr::null_mut();}}}if dev_is_pci(parent)!=0{parent}else{parse_tree_node(parent,6,modpath)}}
unsafe fn device_to_hwpath(dev:*mut device,path:*mut hardware_path){if (*dev).bus==&parisc_bus_type as *const _{let p=to_parisc_device(dev);get_node_path((*dev).parent,path);(*path).mod_=(*p).hw_path}else if dev_is_pci(dev)!=0{get_node_path(dev,path);}}

unsafe fn walk_native_bus(mut low:ulong,high:ulong,parent:*mut device){let mut path=hardware_path::default();get_node_path(parent,&mut path);let mut found=0;loop{for i in 0..64{let hpa=low+i*0x1000;let mut dev=find_device_by_addr(hpa);if dev.is_null(){path.mod_=i as i8;dev=alloc_pa_dev(hpa,&mut path);if dev.is_null(){continue;}register_parisc_device(dev);found+=1;}walk_lower_bus(dev);}if found!=0||low+64*0x1000>=high{break;}low+=64*0x1000;}}
unsafe fn walk_lower_bus(dev:*mut parisc_device){if ((*dev).id.hw_type!=HPHW_IOA&&(*dev).id.hw_type!=HPHW_BCPORT)||IS_LOWER_PORT(dev){return;}let low=READ_IO_IO_LOW(dev);let high=if (*dev).id.hw_type==HPHW_IOA{(low as i32 as ulong)<<16+64*0x1000}else{READ_IO_IO_HIGH(dev)};walk_native_bus(low,high,&mut (*dev).dev);}
pub unsafe fn walk_central_bus(){walk_native_bus(CENTRAL_BUS_ADDR,CENTRAL_BUS_ADDR+64*0x1000,root)}
pub unsafe fn init_parisc_bus(){if bus_register(&parisc_bus_type)!=0{panic!("Could not register PA-RISC bus type\n")}root=root_device_register("parisc");if IS_ERR(root){panic!("Could not register PA-RISC root device\n")}}
pub unsafe fn print_parisc_devices(){for_each_padev(Some(print_one_device),core::ptr::null_mut());}
unsafe extern "C" fn print_one_device(dev:*mut device,_:*mut core::ffi::c_void)->i32{if check_dev(dev)!=0{print_parisc_device(to_parisc_device(dev));}0}
unsafe fn print_parisc_device(dev:*mut parisc_device){pr_info!("%s at %pap",(*dev).name, &(*dev).hpa.start);}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
