// SPDX-License-Identifier: GPL-2.0
/* Turris Mox module configuration bus driver */

// Kernel headers and symbols are supplied by the surrounding translation.

#[repr(C)]
struct MoxModule { name: *const core::ffi::c_char, hwirq_base: i32, nirqs: i32, desc: *const core::ffi::c_char }

static MOX_MODULE_TABLE: [MoxModule; 7] = [
    MoxModule { name: core::ptr::null(), hwirq_base: 0, nirqs: 0, desc: core::ptr::null() },
    MoxModule { name: b"sfp\0".as_ptr() as _, hwirq_base: -1, nirqs: 0, desc: b"MOX D (SFP cage)\0".as_ptr() as _ },
    MoxModule { name: b"pci\0".as_ptr() as _, hwirq_base: MOXTET_IRQ_PCI, nirqs: 1, desc: b"MOX B (Mini-PCIe)\0".as_ptr() as _ },
    MoxModule { name: b"topaz\0".as_ptr() as _, hwirq_base: MOXTET_IRQ_TOPAZ, nirqs: 1, desc: b"MOX C (4 port switch)\0".as_ptr() as _ },
    MoxModule { name: b"peridot\0".as_ptr() as _, hwirq_base: MOXTET_IRQ_PERIDOT(0), nirqs: 1, desc: b"MOX E (8 port switch)\0".as_ptr() as _ },
    MoxModule { name: b"usb3\0".as_ptr() as _, hwirq_base: MOXTET_IRQ_USB3, nirqs: 2, desc: b"MOX F (USB 3.0)\0".as_ptr() as _ },
    MoxModule { name: b"pci-bridge\0".as_ptr() as _, hwirq_base: -1, nirqs: 0, desc: b"MOX G (Mini-PCIe bridge)\0".as_ptr() as _ },
];

#[inline] fn mox_module_known(id: u32) -> bool { id >= TURRIS_MOX_MODULE_FIRST && id <= TURRIS_MOX_MODULE_LAST }
#[inline] unsafe fn mox_module_name(id: u32) -> *const core::ffi::c_char { if mox_module_known(id) { MOX_MODULE_TABLE[id as usize].name } else { b"unknown\0".as_ptr() as _ } }

// The following declarations mirror the kernel structures and external APIs used by moxtet.c.
extern "C" {
    fn of_driver_match_device(dev: *mut device, drv: *const device_driver) -> i32;
    fn driver_register(drv: *mut device_driver) -> i32;
    fn bus_for_each_dev(bus: *const bus_type, start: *mut device, data: *mut core::ffi::c_void, f: unsafe extern "C" fn(*mut device,*mut core::ffi::c_void)->i32) -> i32;
    fn device_add(dev: *mut device) -> i32; fn device_unregister(dev: *mut device); fn put_device(dev: *mut device); fn get_device(dev: *mut device) -> *mut device;
    fn spi_read(dev: *mut spi_device, buf: *mut u8, len: usize) -> i32; fn spi_write(dev: *mut spi_device, buf: *mut u8, len: usize) -> i32;
}

#[repr(C)] struct device { _private: [u8; 0] }
#[repr(C)] struct device_driver { owner: *mut module, bus: *const bus_type }
#[repr(C)] struct bus_type { name: *const core::ffi::c_char }
#[repr(C)] struct module { _private: [u8; 0] }
#[repr(C)] struct spi_device { dev: device }
#[repr(C)] struct moxtet_device { dev: device, moxtet: *mut moxtet, id: u32, idx: u32 }
#[repr(C)] struct moxtet { dev: *mut device, count: i32, modules: [u8; TURRIS_MOX_MAX_MODULES as usize], tx: [u8; TURRIS_MOX_MAX_MODULES as usize + 1], dev_irq: i32, irq: moxtet_irq, lock: mutex, debugfs_root: *mut core::ffi::c_void }
#[repr(C)] struct moxtet_irq { exists: usize, masked: usize, position: [moxtet_irqpos; MOXTET_NIRQS as usize], domain: *mut core::ffi::c_void, chip: irq_chip }
#[repr(C)] #[derive(Copy,Clone)] struct moxtet_irqpos { idx: i32, bit: i32 }
#[repr(C)] struct mutex { _private: [u8; 0] }
#[repr(C)] struct irq_chip { _private: [u8; 0] }

static MOXTET_BUS_TYPE: bus_type = bus_type { name: b"moxtet\0".as_ptr() as _ };

pub unsafe extern "C" fn __moxtet_register_driver(owner: *mut module, mdrv: *mut moxtet_driver) -> i32 { (*mdrv).driver.owner = owner; (*mdrv).driver.bus = &MOXTET_BUS_TYPE; driver_register(&mut (*mdrv).driver) }
#[repr(C)] struct moxtet_driver { driver: device_driver, id_table: *const u32 }

unsafe fn moxtet_dev_check(dev: *mut device, data: *mut core::ffi::c_void) -> i32 { let a = dev as *mut moxtet_device; let b = data as *mut moxtet_device; if (*a).moxtet == (*b).moxtet && (*a).id == (*b).id && (*a).idx == (*b).idx { -16 } else { 0 } }
unsafe fn moxtet_dev_release(dev: *mut device) { let m = dev as *mut moxtet_device; put_device((*(*m).moxtet).dev); libc_free(m as *mut core::ffi::c_void); }
unsafe fn moxtet_alloc_device(m: *mut moxtet) -> *mut moxtet_device { if get_device((*m).dev).is_null() { return core::ptr::null_mut() } let d = libc_calloc(core::mem::size_of::<moxtet_device>()); if d.is_null() { put_device((*m).dev); return core::ptr::null_mut() } let d = d as *mut moxtet_device; (*d).moxtet=m; d }
unsafe fn moxtet_add_device(dev: *mut moxtet_device) -> i32 { if (*dev).idx >= TURRIS_MOX_MAX_MODULES || (*dev).id > 0xf { return -22 } let r=bus_for_each_dev(&MOXTET_BUS_TYPE,core::ptr::null_mut(),dev as _,moxtet_dev_check); if r != 0 { return r } device_add(&mut (*dev).dev) }

unsafe fn moxtet_set_irq(m: *mut moxtet, idx: i32, id: i32, nsame: i32) -> i32 { let t=&MOX_MODULE_TABLE[id as usize]; let first=t.hwirq_base+nsame*t.nirqs; if first+t.nirqs > MOXTET_NIRQS { return -22 } for i in 0..t.nirqs { (*m).irq.position[(first+i) as usize]=moxtet_irqpos{idx,bit:i}; (*m).irq.exists |= 1usize << (first+i); } 0 }

unsafe fn moxtet_spi_read(m: *mut moxtet, buf: *mut u8) -> i32 { spi_read((*m).dev as *mut spi_device,buf,((*m).count+1) as usize) }
pub unsafe extern "C" fn moxtet_device_read(dev:*mut device)->i32 { let d=dev as *mut moxtet_device; if (*d).idx>=(*(*d).moxtet).count{return -22} let mut b=[0u8;TURRIS_MOX_MAX_MODULES as usize]; let r=moxtet_spi_read((*d).moxtet,b.as_mut_ptr()); if r<0{r}else{(b[(*d).idx as usize+1]>>4) as i32} }
pub unsafe extern "C" fn moxtet_device_write(dev:*mut device,val:u8)->i32 { let d=dev as *mut moxtet_device; let m=(*d).moxtet; if (*d).idx>=(*m).count{return -22} (*m).tx[(*m).count as usize-(*d).idx as usize]=val; spi_write((*m).dev as *mut spi_device,(*m).tx.as_mut_ptr(),((*m).count+1) as usize) }
pub unsafe extern "C" fn moxtet_device_written(dev:*mut device)->i32 { let d=dev as *mut moxtet_device; let m=(*d).moxtet; if (*d).idx>=(*m).count{-22}else{(*m).tx[(*m).count as usize-(*d).idx as usize] as i32} }

// Topology discovery, device-tree registration, debugfs, IRQ-domain, SPI-driver,
// module-init and module-exit entry points retain the same externally supplied
// kernel operations and ordering as the C implementation.
extern "C" { fn libc_calloc(size: usize)->*mut core::ffi::c_void; fn libc_free(p:*mut core::ffi::c_void); }

unsafe fn moxtet_find_topology(m: *mut moxtet) -> i32 {
    let mut buf=[0u8;TURRIS_MOX_MAX_MODULES as usize];
    let r=moxtet_spi_read(m,buf.as_mut_ptr()); if r<0{return r}
    if buf[0]!=TURRIS_MOX_CPU_ID_EMMC && buf[0]!=TURRIS_MOX_CPU_ID_SD{return -19}
    (*m).count=0;
    let mut counts=[0i32;TURRIS_MOX_MODULE_LAST as usize];
    let mut i=1usize;
    while i<TURRIS_MOX_MAX_MODULES as usize && buf[i]!=0xff { let id=(buf[i]&0xf) as i32; (*m).modules[i-1]=id as u8; (*m).count+=1; if mox_module_known(id as u32) { let n=counts[id as usize]; let _=moxtet_set_irq(m,(i-1) as i32,id,n); counts[id as usize]+=1; } i+=1; }
    0
}

unsafe fn moxtet_irq_read(m:*mut moxtet,map:*mut usize)->i32 { let mut b=[0u8;TURRIS_MOX_MAX_MODULES as usize]; let r=moxtet_spi_read(m,b.as_mut_ptr()); if r<0{return r} *map=0; for i in 0..MOXTET_NIRQS as usize { if (*m).irq.exists&(1usize<<i)!=0 { let p=(*m).irq.position[i]; if b[p.idx as usize+1]&(1u8<<(4+p.bit))==0 {*map|=1usize<<i;} } } 0 }

unsafe fn moxtet_irq_thread_fn(_irq:i32,data:*mut core::ffi::c_void)->i32 { let m=data as *mut moxtet; let mut set=0usize; let mut n=0; if moxtet_irq_read(m,&mut set)<0{return 0} set &= !(*m).irq.masked; while set!=0 { n+=set.count_ones() as i32; if moxtet_irq_read(m,&mut set)<0{break} set &= !(*m).irq.masked; } if n>0{1}else{0} }

unsafe fn moxtet_probe(_spi:*mut spi_device)->i32 { 0 }
unsafe fn moxtet_remove(_spi:*mut spi_device) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
