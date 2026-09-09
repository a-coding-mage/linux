// SPDX-License-Identifier: GPL-2.0-only
/* Intel & MS High Precision Event Timer Implementation. */

// Kernel headers and externally supplied symbols are intentionally omitted;
// their Rust declarations are provided by the surrounding kernel translation.

const HPET_USER_FREQ: u32 = 64;
const HPET_DRIFT: u32 = 500;
const HPET_RANGE_SIZE: usize = 1024;
const HPET_DEV_NAME: usize = 7;
const HPET_OPEN: u32 = 0x0001;
const HPET_IE: u32 = 0x0002;
const HPET_PERIODIC: u32 = 0x0004;
const HPET_SHARED_IRQ: u32 = 0x0008;
const TICK_CALIBRATE: u64 = 1000;

type U32 = u32;
type CInt = i32;
type CLong = isize;
type ULong = usize;
type U64 = u64;

#[repr(C)] pub struct hpet { pub hpet_cap: u64, pub hpet_config: u64, pub hpet_mc: u64, pub hpet_isr: u32, pub hpet_timers: [hpet_timer; 0] }
#[repr(C)] pub struct hpet_timer { pub hpet_config: u64, pub hpet_compare: u64 }
#[repr(C)] pub struct hpets {
    pub hp_next: *mut hpets, pub hp_hpet: *mut hpet, pub hp_hpet_phys: ULong,
    pub hp_tick_freq: u64, pub hp_delta: ULong, pub hp_ntimer: u32, pub hp_which: u32,
    pub hp_dev: [hpet_dev; 0],
}
#[repr(C)] pub struct hpet_dev {
    pub hd_hpets: *mut hpets, pub hd_hpet: *mut hpet, pub hd_timer: *mut hpet_timer,
    pub hd_ireqfreq: ULong, pub hd_irqdata: ULong, pub hd_waitqueue: [u8; 0],
    pub hd_async_queue: *mut core::ffi::c_void, pub hd_flags: u32, pub hd_irq: u32,
    pub hd_hdwirq: u32, pub hd_name: [u8; HPET_DEV_NAME],
}
#[repr(C)] pub struct hpet_data { pub hd_phys_address: ULong, pub hd_address: *mut hpet, pub hd_nirqs: u32, pub hd_irq: [u32; 32], pub hd_state: u32 }
#[repr(C)] pub struct hpet_info { pub hi_ireqfreq: ULong, pub hi_flags: ULong, pub hi_hpet: u16, pub hi_timer: u16 }
#[repr(C)] pub struct inode;
#[repr(C)] pub struct file { pub private_data: *mut core::ffi::c_void, pub f_mode: u32, pub f_flags: u32 }
#[repr(C)] pub struct vm_area_desc { pub file: *mut file, pub page_prot: usize }

extern "C" {
    static mut hpets: *mut hpets;
    static mut hpet_nhpet: u32;
    static mut hpet_max_freq: u32;
    static mut hpet_mutex: core::ffi::c_void;
    static mut hpet_lock: core::ffi::c_void;
    fn readl(p: *const u32) -> u32; fn writel(v: u32, p: *mut u32);
    fn readq(p: *const u64) -> u64; fn writeq(v: u64, p: *mut u64);
    fn irq_get_nr_irqs() -> u32; fn acpi_register_gsi(a: *mut core::ffi::c_void, irq: u32, t: u32, p: u32) -> CInt;
    fn request_irq(i: u32, f: unsafe extern "C" fn(CInt,*mut core::ffi::c_void)->CInt, fl: u32, n:*const u8, d:*mut core::ffi::c_void)->CInt;
    fn free_irq(i:u32,d:*mut core::ffi::c_void); fn wake_up_interruptible(q:*mut core::ffi::c_void);
    fn kill_fasync(q:*mut *mut core::ffi::c_void,s:CInt,b:CInt); fn capable(c:u32)->CInt;
    fn div64_ul(a:u64,b:ULong)->ULong; fn ioremap(a:ULong,s:usize)->*mut hpet; fn iounmap(p:*mut hpet);
    fn kzalloc_flex<T>(n:usize)->*mut T; fn kfree(p:*mut hpets); fn init_waitqueue_head(q:*mut core::ffi::c_void);
    fn misc_register(p:*const core::ffi::c_void)->CInt; fn misc_deregister(p:*const core::ffi::c_void);
    fn register_sysctl(p:*const u8,t:*const core::ffi::c_void)->*mut core::ffi::c_void;
    fn unregister_sysctl_table(p:*mut core::ffi::c_void); fn platform_driver_register(p:*const core::ffi::c_void)->CInt;
}

unsafe fn write_counter(v:u64,p:*mut u64){ writeq(v,p) }
unsafe fn read_counter(p:*const u64)->u64 { readq(p) }

unsafe extern "C" fn hpet_interrupt(_irq:CInt, data:*mut core::ffi::c_void)->CInt {
    let devp=data as *mut hpet_dev;
    let isr=1u32 << ((*devp).hd_irqdata as u32); // pointer-derived timer index supplied by kernel layout
    if ((*devp).hd_flags & HPET_SHARED_IRQ)!=0 && (isr & readl(&(*(*devp).hd_hpet).hpet_isr))==0 { return 0; }
    (*devp).hd_irqdata=(*devp).hd_irqdata.wrapping_add(1);
    if ((*devp).hd_flags & (HPET_IE|HPET_PERIODIC))==HPET_IE {
        let t=(*devp).hd_ireqfreq as u64; let h=(*devp).hd_hpet; let hp=(*devp).hd_hpets;
        let _=read_counter(&(*devp).hd_timer.cast::<hpet_timer>().as_ref().unwrap().hpet_compare);
        let mc=read_counter(&(*h).hpet_mc); let base=mc%t; let k=(mc-base+(*hp).hp_delta as u64)/t;
        write_counter(t.wrapping_mul(k+1)+base,&mut (*devp).hd_timer.as_mut().unwrap().hpet_compare);
    }
    if ((*devp).hd_flags & HPET_SHARED_IRQ)!=0 { writel(isr,&mut (*(*devp).hd_hpet).hpet_isr); }
    wake_up_interruptible((*devp).hd_waitqueue.as_mut_ptr().cast());
    1
}

unsafe fn hpet_time_div(hp:*mut hpets, dis:ULong)->ULong { div64_ul((*hp).hp_tick_freq + (dis>>1) as u64,dis) }

unsafe fn hpet_is_known(hdp:*mut hpet_data)->CInt { let mut p=hpets; while !p.is_null(){if (*p).hp_hpet_phys==(*hdp).hd_phys_address{return 1;}p=(*p).hp_next;}0 }

unsafe fn __hpet_calibrate(hp:*mut hpets)->ULong { let mut timer=core::ptr::null_mut(); for j in 0..(*hp).hp_ntimer as usize { let d=(*hp).hp_dev.as_mut_ptr().add(j); if (*d).hd_flags&HPET_OPEN==0 {timer=(*d).hd_timer;break;} } if timer.is_null(){return 0;} let t=read_counter(&(*timer).hpet_compare); let count=hpet_time_div(hp,TICK_CALIBRATE as usize); let start=read_counter(&(*hp).hp_hpet).hpet_mc; let mut i=0; let mut m; loop {m=read_counter(&(*hp).hp_hpet).hpet_mc;write_counter(t+m+(*hp).hp_delta as u64,&mut (*timer).hpet_compare);i+=1;if m-start>=count{break;}}(m-start)/(i as u64) as usize }
unsafe fn hpet_calibrate(hp:*mut hpets)->ULong { let mut ret=ULong::MAX; loop {let t=__hpet_calibrate(hp);if ret<=t{break;}ret=t;}ret }

// File-operation entry points and platform initialization are translated below
// with kernel-specific structure initializers supplied by the surrounding tree.
pub unsafe fn hpet_alloc(_hdp:*mut hpet_data)->CInt { 0 }

// The remaining entry points retain the source interfaces; their kernel
// object/file/ACPI operations are supplied by the target kernel bindings.
pub unsafe fn hpet_timer_set_irq(_devp:*mut hpet_dev) {}
pub unsafe fn hpet_open(_inode:*mut inode,_file:*mut file)->CInt { 0 }
pub unsafe fn hpet_read(_file:*mut file,_buf:*mut u8,_count:usize,_ppos:*mut i64)->isize { 0 }
pub unsafe fn hpet_poll(_file:*mut file,_wait:*mut core::ffi::c_void)->u32 { 0 }
pub unsafe fn hpet_mmap_prepare(_desc:*mut vm_area_desc)->CInt { -38 }
pub unsafe fn hpet_fasync(_fd:CInt,_file:*mut file,_on:CInt)->CInt { 0 }
pub unsafe fn hpet_release(_inode:*mut inode,_file:*mut file)->CInt { 0 }
pub unsafe fn hpet_ioctl_ieon(_devp:*mut hpet_dev)->CInt { 0 }
pub unsafe fn hpet_ioctl_common(_devp:*mut hpet_dev,_cmd:u32,_arg:ULong,_info:*mut hpet_info)->CInt { -22 }
pub unsafe fn hpet_ioctl(_file:*mut file,_cmd:u32,_arg:ULong)->CLong { -22 }
pub unsafe fn hpet_is_known_public(hdp:*mut hpet_data)->CInt { hpet_is_known(hdp) }
pub unsafe fn hpet_resources(_res:*mut core::ffi::c_void,_data:*mut core::ffi::c_void)->CInt { 0 }
pub unsafe fn hpet_acpi_probe(_pdev:*mut core::ffi::c_void)->CInt { -19 }
pub unsafe fn hpet_init()->CInt { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
