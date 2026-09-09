/* Direct low-level Rust translation of edac_mc_sysfs.c.  Kernel types,
 * constants, macros, and functions are supplied by the surrounding crate. */

use core::{ffi::{c_char, c_int, c_uint, c_ulong, c_void}, mem, ptr};

extern "C" {
    static mut edac_mem_types: [*const c_char; 16];
    static mut edac_layer_name: [*const c_char; 16];
    fn edac_mc_reset_delay_period(i: c_uint);
    fn edac_dimm_info_location(dimm: *mut dimm_info, data: *mut c_char, size: usize) -> isize;
    fn kstrtouint(s: *const c_char, base: c_uint, out: *mut c_uint) -> c_int;
    fn kstrtoul(s: *const c_char, base: c_uint, out: *mut c_ulong) -> c_int;
    fn scnprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> isize;
    fn sysfs_emit(buf: *mut c_char, fmt: *const c_char, ...) -> isize;
    fn device_initialize(dev: *mut device); fn device_add(dev: *mut device) -> c_int;
    fn device_del(dev: *mut device); fn device_unregister(dev: *mut device);
    fn device_is_registered(dev: *mut device) -> bool; fn put_device(dev: *mut device);
    fn dev_set_name(dev: *mut device, fmt: *const c_char, ...);
    fn dev_name(dev: *mut device) -> *const c_char; fn dev_set_drvdata(dev: *mut device, p: *mut c_void);
    fn pm_runtime_forbid(dev: *mut device); fn edac_get_sysfs_subsys() -> *mut c_void;
    fn edac_create_debugfs_nodes(mci: *mut mem_ctl_info); fn edac_remove_sysfs_mci_device(mci: *mut mem_ctl_info);
    fn edac_dbg(level: c_int, fmt: *const c_char, ...); fn edac_printk(level: c_int, area: c_int, fmt: *const c_char, ...);
    fn kfree(p: *mut c_void);
}

static mut edac_mc_log_ue: c_int = 1;
static mut edac_mc_log_ce: c_int = 1;
static mut edac_mc_panic_on_ue: c_int = 0;
static mut edac_mc_poll_msec: c_uint = 1000;

pub unsafe extern "C" fn edac_mc_get_log_ue() -> c_int { edac_mc_log_ue }
pub unsafe extern "C" fn edac_mc_get_log_ce() -> c_int { edac_mc_log_ce }
pub unsafe extern "C" fn edac_mc_get_panic_on_ue() -> c_int { edac_mc_panic_on_ue }
pub unsafe extern "C" fn edac_mc_get_poll_msec() -> c_uint { edac_mc_poll_msec }

unsafe fn edac_set_poll_msec(val: *const c_char, arg: *mut c_uint) -> c_int {
    if val.is_null() { return -22; }
    let mut i = 0; let ret = kstrtouint(val, 0, &mut i);
    if ret != 0 { return ret; } if i < 1000 { return -22; }
    *arg = i; edac_mc_reset_delay_period(i); 0
}

static mut mci_pdev: *mut device = ptr::null_mut();
static dev_types: [*const c_char; 8] = [b"Unknown\0".as_ptr() as _, b"x1\0".as_ptr() as _, b"x2\0".as_ptr() as _, b"x4\0".as_ptr() as _, b"x8\0".as_ptr() as _, b"x16\0".as_ptr() as _, b"x32\0".as_ptr() as _, b"x64\0".as_ptr() as _];
static edac_caps: [*const c_char; 10] = [b"Unknown\0".as_ptr() as _, b"None\0".as_ptr() as _, b"Reserved\0".as_ptr() as _, b"PARITY\0".as_ptr() as _, b"EC\0".as_ptr() as _, b"SECDED\0".as_ptr() as _, b"S2ECD2ED\0".as_ptr() as _, b"S4ECD4ED\0".as_ptr() as _, b"S8ECD8ED\0".as_ptr() as _, b"S16ECD16ED\0".as_ptr() as _];

/* The following structures and callback bodies preserve the C object model;
 * their concrete kernel definitions are external dependencies. */
#[repr(C)] pub struct device { pub type_: *const c_void, pub parent: *mut device, pub groups: *const c_void, pub bus: *mut c_void, pub release: Option<unsafe extern "C" fn(*mut device)>, pub init_name: *const c_char }
#[repr(C)] pub struct dimm_info { pub dev: device, pub mci: *mut mem_ctl_info, pub idx: c_int, pub label: [c_char; 64], pub nr_pages: c_int, pub mtype: usize, pub dtype: usize, pub edac_mode: usize, pub ce_count: c_uint, pub ue_count: c_uint }
#[repr(C)] pub struct mem_ctl_info { pub dev: device, pub csbased: bool, pub mc_idx: c_int, pub nr_csrows: c_int, pub n_layers: c_int, pub csrows: *mut *mut csrow_info, pub ue_mc: c_uint, pub ce_mc: c_uint, pub ue_noinfo_count: c_uint, pub ce_noinfo_count: c_uint, pub start_time: c_ulong, pub ctl_name: *const c_char, pub set_sdram_scrub_rate: Option<unsafe extern "C" fn(*mut mem_ctl_info,c_ulong)->c_int>, pub get_sdram_scrub_rate: Option<unsafe extern "C" fn(*mut mem_ctl_info)->c_int> }
#[repr(C)] pub struct csrow_info { pub nr_channels: c_int, pub channels: *mut *mut channel_info, pub ue_count: c_uint, pub ce_count: c_uint }
#[repr(C)] pub struct channel_info { pub dimm: *mut dimm_info, pub ce_count: c_uint }

unsafe fn dimm_from_dev(dev: *mut device) -> *mut dimm_info { dev as *mut dimm_info }
unsafe fn mci_from_dev(dev: *mut device) -> *mut mem_ctl_info { dev as *mut mem_ctl_info }
unsafe fn dimmdev_location_show(dev:*mut device, _: *mut c_void, data:*mut c_char)->isize { let d=dimm_from_dev(dev); let n=edac_dimm_info_location(d,data,4096); n+scnprintf(data.add(n as usize),4096-n as usize,b"\n\0".as_ptr() as _) }
unsafe fn dimmdev_label_show(dev:*mut device, _: *mut c_void, data:*mut c_char)->isize { let d=dimm_from_dev(dev); if (*d).label[0]==0{return 0}; sysfs_emit(data,b"%s\n\0".as_ptr() as _,(*d).label.as_ptr()) }
unsafe fn dimmdev_label_store(dev:*mut device, _: *mut c_void, data:*const c_char,count:usize)->isize { let d=dimm_from_dev(dev); if count==0{return -22}; let mut n=count; let last=*data.add(count-1); if last==0||last==b'\n' as _ {n-=1}; if n==0||n>=64{return -22}; ptr::copy_nonoverlapping(data,*(&mut (*d).label as *mut _ as *mut c_char),n); (*d).label[n]=0; count as isize }

unsafe fn mci_reset_counters_store(dev:*mut device,_:*mut c_void,_:*const c_char,count:usize)->isize { let m=mci_from_dev(dev);(*m).ue_mc=0;(*m).ce_mc=0;(*m).ue_noinfo_count=0;(*m).ce_noinfo_count=0;(*m).start_time=0;count as isize }
unsafe fn mci_sdram_scrub_rate_store(dev:*mut device,_:*mut c_void,data:*const c_char,count:usize)->isize { let m=mci_from_dev(dev);let mut bw=0;if kstrtoul(data,10,&mut bw)<0{return -22};if let Some(f)=(*m).set_sdram_scrub_rate {if f(m,bw)<0{return -22}} count as isize }
unsafe fn mci_sdram_scrub_rate_show(dev:*mut device,_:*mut c_void,data:*mut c_char)->isize { let m=mci_from_dev(dev);if let Some(f)=(*m).get_sdram_scrub_rate {let b=f(m);if b<0{return b as isize};return sysfs_emit(data,b"%d\n\0".as_ptr() as _,b)} -22 }

pub unsafe extern "C" fn edac_create_sysfs_mci_device(mci:*mut mem_ctl_info,groups:*const c_void)->c_int { (*mci).dev.parent=mci_pdev;(*mci).dev.groups=groups;pm_runtime_forbid(&mut (*mci).dev);let e=device_add(&mut (*mci).dev);if e<0{return e};edac_create_debugfs_nodes(mci);0 }
pub unsafe extern "C" fn edac_remove_sysfs_mci_device_impl(mci:*mut mem_ctl_info){if device_is_registered(&mut (*mci).dev){device_del(&mut (*mci).dev)}}
pub unsafe extern "C" fn edac_mc_sysfs_init()->c_int { mci_pdev=libc::calloc(1,mem::size_of::<device>()) as *mut device;if mci_pdev.is_null(){return -12};(*mci_pdev).bus=edac_get_sysfs_subsys();device_register(mci_pdev) }
pub unsafe extern "C" fn edac_mc_sysfs_exit(){device_unregister(mci_pdev)}

extern "C" { fn device_register(dev:*mut device)->c_int; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
