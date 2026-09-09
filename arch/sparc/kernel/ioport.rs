// SPDX-License-Identifier: GPL-2.0
/*
 * ioport.c: Simple io mapping allocator.
 */

// External kernel types, constants, and functions are supplied by other translation units.

#[repr(C)]
pub struct Resource {
    pub name: *mut i8,
    pub start: usize,
    pub end: usize,
    pub flags: u64,
    pub child: *mut Resource,
    pub sibling: *mut Resource,
}

#[repr(C)]
pub struct XResource {
    pub xres: Resource,
    pub xflag: i32,
    pub xname: [i8; XNMLN + 1],
}

#[repr(C)]
pub struct DeviceNode { pub full_name: *mut i8 }
#[repr(C)]
pub struct Device { pub of_node: *mut DeviceNode }
#[repr(C)]
pub struct SeqFile { pub private: *mut core::ffi::c_void }

pub type PhysAddr = usize;
pub type DmaDataDirection = i32;

pub const XNMLN: usize = 15;
pub const XNRES: usize = 10;
pub const PAGE_SIZE: usize = 4096;
pub const PAGE_MASK: usize = !(PAGE_SIZE - 1);
pub const DMA_TO_DEVICE: DmaDataDirection = 1;

extern "C" {
    static mut DVMA_VADDR: usize;
    static mut DVMA_END: usize;
    static mut IOBASE_VADDR: usize;
    static mut IOBASE_END: usize;
    static mut sparc_cpu_model: i32;
    static mut sparc_leon: i32;
    static mut sparc_leon3_snooping_enabled: unsafe extern "C" fn() -> bool;
    static mut sparc_iomap: Resource;
    fn lookup_resource(root: *mut Resource, addr: usize) -> *mut Resource;
    fn allocate_resource(root: *mut Resource, res: *mut Resource, size: usize,
                         start: usize, end: usize, align: usize,
                         a: *mut core::ffi::c_void, b: *mut core::ffi::c_void) -> i32;
    fn release_resource(res: *mut Resource);
    fn srmmu_mapiorange(bus: u32, pa: usize, start: usize, size: usize);
    fn srmmu_unmapiorange(start: usize, size: usize);
    fn prom_printf(fmt: *const i8, ...);
    fn prom_halt() -> !;
    fn printk(fmt: *const i8, ...);
    fn kfree(ptr: *mut core::ffi::c_void);
    fn kmalloc(size: usize, flags: u32) -> *mut i8;
    fn leon_flush_dcache_all();
    fn proc_create_single_data(name: *const i8, mode: u32, parent: *mut core::ffi::c_void,
                                show: unsafe extern "C" fn(*mut SeqFile, *mut core::ffi::c_void) -> i32,
                                data: *mut core::ffi::c_void);
    fn seq_printf(m: *mut SeqFile, fmt: *const i8, ...);
}

// This points to the next to use virtual memory for DVMA mappings.
static mut SPARC_DVMA: Resource = Resource { name: b"sparc_dvma\0".as_ptr() as *mut i8, start: 0, end: 0, flags: 0, child: core::ptr::null_mut(), sibling: core::ptr::null_mut() };

// This points to the start of I/O mappings, usable from outside.
#[no_mangle]
pub static mut sparc_iomap: Resource = Resource { name: b"sparc_iomap\0".as_ptr() as *mut i8, start: 0, end: 0, flags: 0, child: core::ptr::null_mut(), sibling: core::ptr::null_mut() };

static mut XRESV: [XResource; XNRES] = unsafe { core::mem::zeroed() };

unsafe fn xres_alloc() -> *mut XResource {
    let mut xrp = XRESV.as_mut_ptr();
    for _ in 0..XNRES {
        if (*xrp).xflag == 0 { (*xrp).xflag = 1; return xrp; }
        xrp = xrp.add(1);
    }
    core::ptr::null_mut()
}

unsafe fn xres_free(xrp: *mut XResource) { (*xrp).xflag = 0; }

#[no_mangle]
pub unsafe extern "C" fn ioremap(offset: PhysAddr, size: usize) -> *mut core::ffi::c_void {
    let mut name = [0i8; 14];
    // sprintf(name, "phys_%08x", (u32)offset);
    let _ = offset;
    _sparc_alloc_io(0, offset, size, name.as_mut_ptr())
}

#[no_mangle]
pub unsafe extern "C" fn iounmap(virtual_: *mut core::ffi::c_void) {
    let vaddr = virtual_ as usize & PAGE_MASK;
    let res = lookup_resource(core::ptr::addr_of_mut!(sparc_iomap), vaddr);
    if res.is_null() { printk(b"free_io/iounmap: cannot free %lx\n\0".as_ptr() as *const i8, vaddr); return; }
    _sparc_free_io(res);
    let first = XRESV.as_ptr() as usize;
    let last = XRESV.as_ptr().add(XNRES) as usize;
    if (res as usize) >= first && (res as usize) < last { xres_free(res as *mut XResource); } else { kfree(res as *mut _); }
}

#[no_mangle]
pub unsafe extern "C" fn of_ioremap(res: *mut Resource, offset: usize, size: usize, name: *mut i8) -> *mut core::ffi::c_void {
    _sparc_alloc_io(((*res).flags & 0xF) as u32, (*res).start + offset, size, name)
}

#[no_mangle]
pub unsafe extern "C" fn of_iounmap(_res: *mut Resource, base: *mut core::ffi::c_void, _size: usize) { iounmap(base); }

unsafe fn _sparc_alloc_io(busno: u32, phys: usize, size: usize, name: *mut i8) -> *mut core::ffi::c_void {
    static mut PRINTED_FULL: bool = false;
    let mut xres = xres_alloc();
    let (res, tack);
    if !xres.is_null() { tack = (*xres).xname.as_mut_ptr(); res = &mut (*xres).xres; }
    else {
        if !PRINTED_FULL { printk(b"ioremap: done with statics, switching to malloc\n\0".as_ptr() as *const i8); PRINTED_FULL = true; }
        let _tlen = 0usize; // strlen(name), retained as an external dependency in the original.
        let p = kmalloc(core::mem::size_of::<Resource>() + _tlen + 1, 0);
        if p.is_null() { return core::ptr::null_mut(); }
        core::ptr::write_bytes(p, 0, core::mem::size_of::<Resource>());
        res = &mut *(p as *mut Resource); tack = p.add(core::mem::size_of::<Resource>());
    }
    let _ = name; // strscpy(tack, name, XNMLN + 1)
    res.name = tack;
    _sparc_ioremap(res, busno, phys as u32, size as i32)
}

unsafe fn _sparc_ioremap(res: *mut Resource, bus: u32, pa: u32, sz: i32) -> *mut core::ffi::c_void {
    let offset = pa as usize & !PAGE_MASK;
    let size = (offset + sz as usize + PAGE_SIZE - 1) & PAGE_MASK;
    if allocate_resource(core::ptr::addr_of_mut!(sparc_iomap), res, size, sparc_iomap.start, sparc_iomap.end, PAGE_SIZE, core::ptr::null_mut(), core::ptr::null_mut()) != 0 {
        prom_printf(b"alloc_io_res(%s): cannot occupy\n\0".as_ptr() as *const i8, if !(*res).name.is_null() { (*res).name } else { b"???\0".as_ptr() as *mut i8 });
        prom_halt();
    }
    srmmu_mapiorange(bus, pa as usize & PAGE_MASK, (*res).start, (*res).end - (*res).start + 1);
    ((*res).start + offset) as *mut core::ffi::c_void
}

unsafe fn _sparc_free_io(res: *mut Resource) {
    let plen = (*res).end - (*res).start + 1;
    srmmu_unmapiorange((*res).start, plen);
    release_resource(res);
}

#[no_mangle]
pub unsafe extern "C" fn sparc_dma_alloc_resource(dev: *mut Device, len: usize) -> usize {
    let res = kmalloc(core::mem::size_of::<Resource>(), 0) as *mut Resource;
    if res.is_null() { return 0; }
    (*res).name = (*(*dev).of_node).full_name;
    if allocate_resource(core::ptr::addr_of_mut!(SPARC_DVMA), res, len, SPARC_DVMA.start, SPARC_DVMA.end, PAGE_SIZE, core::ptr::null_mut(), core::ptr::null_mut()) != 0 { kfree(res as *mut _); return 0; }
    (*res).start
}

#[no_mangle]
pub unsafe extern "C" fn sparc_dma_free_resource(cpu_addr: *mut core::ffi::c_void, size: usize) -> bool {
    let addr = cpu_addr as usize;
    let res = lookup_resource(core::ptr::addr_of_mut!(SPARC_DVMA), addr);
    if res.is_null() || addr & (PAGE_SIZE - 1) != 0 { return false; }
    let size = (size + PAGE_SIZE - 1) & PAGE_MASK;
    if (*res).end - (*res).start + 1 != size { return false; }
    release_resource(res); kfree(res as *mut _); true
}

#[no_mangle]
pub unsafe extern "C" fn arch_sync_dma_for_cpu(_paddr: PhysAddr, _size: usize, dir: DmaDataDirection) {
    if dir != DMA_TO_DEVICE && sparc_cpu_model == sparc_leon && !(sparc_leon3_snooping_enabled)() { leon_flush_dcache_all(); }
}

// CONFIG_SBUS conditional section.
#[cfg(feature = "CONFIG_SBUS")]
#[no_mangle]
pub unsafe extern "C" fn sbus_set_sbus64(_dev: *mut Device, _x: i32) {
    printk(b"sbus_set_sbus64: unsupported\0".as_ptr() as *const i8);
}

#[cfg(feature = "CONFIG_SBUS")]
unsafe extern "C" fn sparc_register_ioport() -> i32 {
    register_proc_sparc_ioport();
    0
}

// arch_initcall(sparc_register_ioport) is a build-system registration hook.

// CONFIG_PROC_FS conditional section.
#[cfg(feature = "CONFIG_PROC_FS")]
unsafe extern "C" fn sparc_io_proc_show(m: *mut SeqFile, _v: *mut core::ffi::c_void) -> i32 {
    let root = (*m).private as *mut Resource;
    let mut r = (*root).child;
    while !r.is_null() {
        let nm = if (*r).name.is_null() { b"???\0".as_ptr() as *const i8 } else { (*r).name as *const i8 };
        seq_printf(m, b"%016llx-%016llx: %s\n\0".as_ptr() as *const i8,
                   (*r).start as u64, (*r).end as u64, nm);
        r = (*r).sibling;
    }
    0
}

unsafe fn register_proc_sparc_ioport() {
    #[cfg(feature = "CONFIG_PROC_FS")]
    {
        proc_create_single_data(b"io_map\0".as_ptr() as *const i8, 0, core::ptr::null_mut(), sparc_io_proc_show, core::ptr::addr_of_mut!(sparc_iomap) as *mut _);
        proc_create_single_data(b"dvma_map\0".as_ptr() as *const i8, 0, core::ptr::null_mut(), sparc_io_proc_show, core::ptr::addr_of_mut!(SPARC_DVMA) as *mut _);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
