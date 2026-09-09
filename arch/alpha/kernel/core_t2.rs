// SPDX-License-Identifier: GPL-2.0
/* Literal low-level translation of linux/arch/alpha/kernel/core_t2.c. */

// C headers and build-time configuration are supplied by the surrounding kernel.
const DEBUG_PRINT_INITIAL_SETTINGS: usize = 0;
const DEBUG_PRINT_FINAL_SETTINGS: usize = 0;
const T2_DIRECTMAP_START: usize = 0x8000_0000;
const T2_DIRECTMAP_LENGTH: usize = 0x8000_0000;
const T2_ISA_SG_START: usize = 0x0080_0000;
const T2_ISA_SG_LENGTH: usize = 0x0080_0000;
const DEBUG_CONFIG: usize = 0;
const SIC_SEIC: u64 = 1u64 << 33;

#[repr(C)]
struct T2Window { wbase: usize, wmask: usize, tbase: usize }
#[repr(C)]
struct T2SavedConfig {
    window: [T2Window; 2], hae_1: usize, hae_2: usize, hae_3: usize,
    hae_4: usize, hbase: usize,
}

static mut T2_MCHECK_ANY_EXPECTED: u32 = 0;
static mut T2_MCHECK_LAST_TAKEN: u32 = 0;
static mut T2_SAVED_CONFIG: T2SavedConfig = T2SavedConfig {
    window: [T2Window { wbase: 0, wmask: 0, tbase: 0 }; 2],
    hae_1: 0, hae_2: 0, hae_3: 0, hae_4: 0, hbase: 0,
};

extern "C" {
    static mut __direct_map_base: usize;
    static mut __direct_map_size: usize;
    static mut pci_isa_hose: *mut pci_controller;
    static mut ioport_resource: resource;
    static mut iomem_resource: resource;
    static mut pci_hae0_name: *const i8;
    static mut srm_hae: usize;
    static T2_HAE_1: usize; static T2_HAE_2: usize; static T2_HAE_3: usize;
    static T2_HAE_4: usize; static T2_HBASE: usize; static T2_WBASE1: usize;
    static T2_WMASK1: usize; static T2_TBASE1: usize; static T2_WBASE2: usize;
    static T2_WMASK2: usize; static T2_TBASE2: usize; static T2_IOCSR: usize;
    static T2_CONF: usize; static T2_CERR1: usize; static T2_PERR1: usize;
    fn smp_processor_id() -> usize; fn mb(); fn draina(); fn udelay(n: usize);
    fn wrmces(v: usize); fn process_mcheck_info(v: usize, p: usize, s: *const i8, e: usize);
    fn mcheck_expected(cpu: usize) -> *mut usize; fn mcheck_taken(cpu: usize) -> *mut usize;
    fn alloc_pci_controller() -> *mut pci_controller; fn alloc_resource() -> *mut resource;
    fn request_resource(a: *mut resource, b: *mut resource) -> i32;
    fn iommu_arena_new(h: *mut pci_controller, b: usize, l: usize, c: usize) -> *mut arena;
    fn virt_to_phys(p: *mut pte) -> usize;
    fn printk(fmt: *const i8, ...);
}

#[repr(C)] struct pci_bus { number: u8 }
#[repr(C)] struct pte;
#[repr(C)] struct arena { ptes: *mut pte }
#[repr(C)] struct resource { start: usize, end: usize, name: *const i8 }
#[repr(C)] struct pci_controller { io_space: *mut resource, mem_space: *mut resource, index: i32, sparse_mem_base: usize, dense_mem_base: usize, sparse_io_base: usize, dense_io_base: usize, sg_isa: *mut arena, sg_pci: *mut arena }
#[repr(C)] struct sable_cpu_csr { sic: u64, bcce: u64, cbe: u64, bcue: u64, dter: u64 }

unsafe fn reg(p: usize) -> *mut usize { p as *mut usize }

unsafe fn mk_conf_addr(pbus: *mut pci_bus, device_fn: u32, where_: i32, pci_addr: *mut usize, type1: *mut u8) -> i32 {
    let bus = (*pbus).number;
    if bus == 0 {
        let device = device_fn >> 3;
        if device > 8 { return -1; }
        *type1 = 0; *pci_addr = (0x0800usize << device) | ((device_fn & 7) << 8) | where_ as usize;
    } else { *type1 = 1; *pci_addr = ((bus as usize) << 16) | ((device_fn as usize) << 8) | where_ as usize; }
    0
}

unsafe fn conf_read(addr: usize, type1: u8) -> u32 {
    let cpu = smp_processor_id(); let mut cfg = 0usize; let mut value;
    if type1 != 0 { cfg = core::ptr::read_volatile(reg(T2_HAE_3)) & !0xc000_0000; core::ptr::write_volatile(reg(T2_HAE_3), 0x4000_0000 | cfg); mb(); }
    mb(); draina(); *mcheck_expected(cpu)=1; *mcheck_taken(cpu)=0; T2_MCHECK_ANY_EXPECTED |= 1 << cpu; mb();
    value = core::ptr::read_volatile(addr as *const u32); mb(); mb(); udelay(100);
    if *mcheck_taken(cpu) != 0 { *mcheck_taken(cpu)=0; T2_MCHECK_LAST_TAKEN |= 1 << cpu; value=0xffff_ffff; mb(); }
    *mcheck_expected(cpu)=0; T2_MCHECK_ANY_EXPECTED=0; mb();
    if type1 != 0 { core::ptr::write_volatile(reg(T2_HAE_3), cfg); mb(); } value
}

unsafe fn conf_write(addr: usize, value: u32, type1: u8) {
    let cpu=smp_processor_id(); let mut cfg=0usize;
    if type1 != 0 { cfg=core::ptr::read_volatile(reg(T2_HAE_3)) & !0xc000_0000; core::ptr::write_volatile(reg(T2_HAE_3), cfg|0x4000_0000); mb(); }
    mb(); draina(); *mcheck_expected(cpu)=1; *mcheck_taken(cpu)=0; T2_MCHECK_ANY_EXPECTED |= 1<<cpu; mb();
    core::ptr::write_volatile(addr as *mut u32, value); mb(); mb(); udelay(100);
    if *mcheck_taken(cpu)!=0 { *mcheck_taken(cpu)=0; T2_MCHECK_LAST_TAKEN |= 1<<cpu; mb(); }
    *mcheck_expected(cpu)=0; T2_MCHECK_ANY_EXPECTED=0; mb(); if type1!=0 { core::ptr::write_volatile(reg(T2_HAE_3),cfg); mb(); }
}

unsafe fn t2_read_config(bus:*mut pci_bus, devfn:u32, where_:i32, size:i32, value:*mut u32)->i32 { let mut pa=0; let mut t=0; if mk_conf_addr(bus,devfn,where_,&mut pa,&mut t)!=0{return -1;} let mask=(size-1)*8; let addr=(pa<<5)+mask as usize+T2_CONF; *value=conf_read(addr,t)>>((where_&3)*8); 0 }
unsafe fn t2_write_config(bus:*mut pci_bus,devfn:u32,where_:i32,size:i32,value:u32)->i32 { let mut pa=0; let mut t=0; if mk_conf_addr(bus,devfn,where_,&mut pa,&mut t)!=0{return -1;} let mask=(size-1)*8; conf_write((pa<<5)+mask as usize+T2_CONF,value<<((where_&3)*8),t); 0 }

#[repr(C)] pub struct pci_ops { pub read: unsafe fn(*mut pci_bus,u32,i32,i32,*mut u32)->i32, pub write: unsafe fn(*mut pci_bus,u32,i32,i32,u32)->i32 }
#[no_mangle] pub static t2_pci_ops: pci_ops = pci_ops { read:t2_read_config, write:t2_write_config };

unsafe fn t2_direct_map_window1(base:usize,length:usize) { __direct_map_base=base; __direct_map_size=length; let temp=(base&0xfff0_0000)|((base+length-1)>>20); core::ptr::write_volatile(reg(T2_WBASE1),temp|0x80000); core::ptr::write_volatile(reg(T2_WMASK1),(length-1)&0xfff0_0000); core::ptr::write_volatile(reg(T2_TBASE1),0); }
unsafe fn t2_sg_map_window2(hose:*mut pci_controller,base:usize,length:usize) { (*hose).sg_isa=iommu_arena_new(hose,base,length,64); (*hose).sg_pci=core::ptr::null_mut(); let temp=(base&0xfff0_0000)|((base+length-1)>>20); core::ptr::write_volatile(reg(T2_WBASE2),temp|0xc0000); core::ptr::write_volatile(reg(T2_WMASK2),(length-1)&0xfff0_0000); core::ptr::write_volatile(reg(T2_TBASE2),virt_to_phys((*(*hose).sg_isa).ptes)>>1); mb(); t2_pci_tbi(hose,0,usize::MAX); }

unsafe fn t2_save_configuration() { T2_SAVED_CONFIG.window[0]=T2Window{wbase:core::ptr::read_volatile(reg(T2_WBASE1)),wmask:core::ptr::read_volatile(reg(T2_WMASK1)),tbase:core::ptr::read_volatile(reg(T2_TBASE1))}; T2_SAVED_CONFIG.window[1]=T2Window{wbase:core::ptr::read_volatile(reg(T2_WBASE2)),wmask:core::ptr::read_volatile(reg(T2_WMASK2)),tbase:core::ptr::read_volatile(reg(T2_TBASE2))}; T2_SAVED_CONFIG.hae_1=srm_hae; T2_SAVED_CONFIG.hae_2=core::ptr::read_volatile(reg(T2_HAE_2)); T2_SAVED_CONFIG.hae_3=core::ptr::read_volatile(reg(T2_HAE_3)); T2_SAVED_CONFIG.hae_4=core::ptr::read_volatile(reg(T2_HAE_4)); T2_SAVED_CONFIG.hbase=core::ptr::read_volatile(reg(T2_HBASE)); }

pub unsafe fn t2_init_arch() { for i in 0..64 { *mcheck_expected(i)=0; *mcheck_taken(i)=0; } T2_MCHECK_ANY_EXPECTED=0; T2_MCHECK_LAST_TAKEN=0; let temp=core::ptr::read_volatile(reg(T2_IOCSR)); if temp&(1<<26)==0 { core::ptr::write_volatile(reg(T2_IOCSR),temp|(1<<26)); mb(); core::ptr::read_volatile(reg(T2_IOCSR)); } t2_save_configuration(); let hose=alloc_pci_controller(); pci_isa_hose=hose; (*hose).io_space=&mut ioport_resource; let mem=alloc_resource(); (*mem).start=0; (*mem).end=0xffff_ffff; (*mem).name=pci_hae0_name; request_resource(&mut iomem_resource,mem); (*hose).mem_space=mem; (*hose).index=0; t2_direct_map_window1(T2_DIRECTMAP_START,T2_DIRECTMAP_LENGTH); t2_sg_map_window2(hose,T2_ISA_SG_START,T2_ISA_SG_LENGTH); core::ptr::write_volatile(reg(T2_HBASE),0); for r in [T2_HAE_1,T2_HAE_2,T2_HAE_3,T2_HAE_4] { core::ptr::write_volatile(reg(r),0); mb(); } }

pub unsafe fn t2_kill_arch(_mode:i32) { let a=&T2_SAVED_CONFIG; core::ptr::write_volatile(reg(T2_WBASE1),a.window[0].wbase); core::ptr::write_volatile(reg(T2_WMASK1),a.window[0].wmask); core::ptr::write_volatile(reg(T2_TBASE1),a.window[0].tbase); core::ptr::write_volatile(reg(T2_WBASE2),a.window[1].wbase); core::ptr::write_volatile(reg(T2_WMASK2),a.window[1].wmask); core::ptr::write_volatile(reg(T2_TBASE2),a.window[1].tbase); mb(); core::ptr::write_volatile(reg(T2_HAE_1),srm_hae); core::ptr::write_volatile(reg(T2_HAE_2),a.hae_2); core::ptr::write_volatile(reg(T2_HAE_3),a.hae_3); core::ptr::write_volatile(reg(T2_HAE_4),a.hae_4); core::ptr::write_volatile(reg(T2_HBASE),a.hbase); mb(); core::ptr::read_volatile(reg(T2_HBASE)); }
pub unsafe fn t2_pci_tbi(_hose:*mut pci_controller,_start:usize,_end:usize) { let x=core::ptr::read_volatile(reg(T2_IOCSR)); core::ptr::write_volatile(reg(T2_IOCSR),x|(1<<28)); mb(); core::ptr::read_volatile(reg(T2_IOCSR)); core::ptr::write_volatile(reg(T2_IOCSR),x&!(1<<28)); mb(); core::ptr::read_volatile(reg(T2_IOCSR)); }

unsafe fn t2_clear_errors(cpu:i32) { let r=(0usize as *mut sable_cpu_csr).wrapping_add(cpu as usize); (*r).sic&=!SIC_SEIC; (*r).bcce|=(*r).bcce; (*r).cbe|=(*r).cbe; (*r).bcue|=(*r).bcue; (*r).dter|=(*r).dter; let a=core::ptr::read_volatile(reg(T2_CERR1)); core::ptr::write_volatile(reg(T2_CERR1),a|a); let b=core::ptr::read_volatile(reg(T2_PERR1)); core::ptr::write_volatile(reg(T2_PERR1),b|b); mb(); mb(); }
pub unsafe fn t2_machine_check(vector:usize,la_ptr:usize) { let cpu=smp_processor_id(); mb(); mb(); draina(); t2_clear_errors(cpu as i32); wrmces(7); mb(); if *mcheck_expected(cpu)==0 && T2_MCHECK_ANY_EXPECTED!=0{return;} if *mcheck_expected(cpu)==0 && T2_MCHECK_ANY_EXPECTED==0 { if T2_MCHECK_LAST_TAKEN&(1<<cpu)!=0 { T2_MCHECK_LAST_TAKEN=0; mb(); return; } else { T2_MCHECK_LAST_TAKEN=0; mb(); } } process_mcheck_info(vector,la_ptr,b"T2\0".as_ptr() as *const i8,*mcheck_expected(cpu)); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
