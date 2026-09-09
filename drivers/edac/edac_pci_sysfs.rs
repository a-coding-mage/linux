/*
 * (C) 2005, 2006 Linux Networx (http://lnxi.com)
 * This file may be distributed under the terms of the
 * GNU General Public License.
 *
 * Written Doug Thompson <norsk5@xmission.com>
 */

// Linux kernel dependencies and the local EDAC declarations are supplied by
// other translation units.

const EDAC_PCI_SYMLINK: &[u8] = b"device\0";

static mut CHECK_PCI_ERRORS: i32 = 0;
static mut EDAC_PCI_PANIC_ON_PE: i32 = 0;
static mut EDAC_PCI_LOG_PE: i32 = 1;
static mut EDAC_PCI_LOG_NPE: i32 = 1;
static mut EDAC_PCI_POLL_MSEC: i32 = 1000;

static mut PCI_PARITY_COUNT: atomic_t = atomic_t::init(0);
static mut PCI_NONPARITY_COUNT: atomic_t = atomic_t::init(0);
static mut EDAC_PCI_TOP_MAIN_KOBJ: *mut kobject = core::ptr::null_mut();
static mut EDAC_PCI_SYSFS_REFCOUNT: atomic_t = atomic_t::init(0);

pub unsafe fn edac_pci_get_check_errors() -> i32 { CHECK_PCI_ERRORS }
unsafe fn edac_pci_get_log_pe() -> i32 { EDAC_PCI_LOG_PE }
unsafe fn edac_pci_get_log_npe() -> i32 { EDAC_PCI_LOG_NPE }
unsafe fn edac_pci_get_panic_on_pe() -> i32 { EDAC_PCI_PANIC_ON_PE }
pub unsafe fn edac_pci_get_poll_msec() -> i32 { EDAC_PCI_POLL_MSEC }

unsafe fn instance_pe_count_show(pci: *mut edac_pci_ctl_info, data: *mut c_char) -> ssize_t {
    sprintf(data, b"%u\n\0".as_ptr() as *const c_char, atomic_read(&(*pci).counters.pe_count))
}
unsafe fn instance_npe_count_show(pci: *mut edac_pci_ctl_info, data: *mut c_char) -> ssize_t {
    sprintf(data, b"%u\n\0".as_ptr() as *const c_char, atomic_read(&(*pci).counters.npe_count))
}

#[repr(C)]
struct instance_attribute {
    attr: attribute,
    show: Option<unsafe fn(*mut edac_pci_ctl_info, *mut c_char) -> ssize_t>,
    store: Option<unsafe fn(*mut edac_pci_ctl_info, *const c_char, usize) -> ssize_t>,
}

unsafe fn edac_pci_instance_release(kobj: *mut kobject) {
    edac_dbg(0, b"\n\0".as_ptr() as *const c_char);
    let pci = container_of!(kobj, edac_pci_ctl_info, kobj);
    kobject_put(EDAC_PCI_TOP_MAIN_KOBJ);
    kfree(pci as *mut c_void);
}

unsafe fn edac_pci_instance_show(kobj: *mut kobject, attr: *mut attribute, buffer: *mut c_char) -> ssize_t {
    let pci = container_of!(kobj, edac_pci_ctl_info, kobj);
    let instance_attr = container_of!(attr, instance_attribute, attr);
    match (*instance_attr).show { Some(f) => f(pci, buffer), None => -EIO as ssize_t }
}
unsafe fn edac_pci_instance_store(kobj: *mut kobject, attr: *mut attribute, buffer: *const c_char, count: usize) -> ssize_t {
    let pci = container_of!(kobj, edac_pci_ctl_info, kobj);
    let instance_attr = container_of!(attr, instance_attribute, attr);
    match (*instance_attr).store { Some(f) => f(pci, buffer, count), None => -EIO as ssize_t }
}

static PCI_INSTANCE_OPS: sysfs_ops = sysfs_ops { show: Some(edac_pci_instance_show), store: Some(edac_pci_instance_store) };
static ATTR_INSTANCE_PE_COUNT: instance_attribute = instance_attribute { attr: attribute { name: b"pe_count\0".as_ptr() as *const c_char, mode: S_IRUGO }, show: Some(instance_pe_count_show), store: None };
static ATTR_INSTANCE_NPE_COUNT: instance_attribute = instance_attribute { attr: attribute { name: b"npe_count\0".as_ptr() as *const c_char, mode: S_IRUGO }, show: Some(instance_npe_count_show), store: None };
static PCI_INSTANCE_ATTRS: [*const attribute; 3] = [&ATTR_INSTANCE_PE_COUNT.attr, &ATTR_INSTANCE_NPE_COUNT.attr, core::ptr::null()];
static KTYPE_PCI_INSTANCE: kobj_type = kobj_type { release: Some(edac_pci_instance_release), sysfs_ops: &PCI_INSTANCE_OPS, default_groups: core::ptr::null() };

unsafe fn edac_pci_create_instance_kobj(pci: *mut edac_pci_ctl_info, idx: i32) -> i32 {
    edac_dbg(0, b"\n\0".as_ptr() as *const c_char);
    if kobject_get(EDAC_PCI_TOP_MAIN_KOBJ).is_null() { return -ENODEV; }
    let err = kobject_init_and_add(&mut (*pci).kobj, &KTYPE_PCI_INSTANCE, EDAC_PCI_TOP_MAIN_KOBJ, b"pci%d\0".as_ptr() as *const c_char, idx);
    if err != 0 { edac_dbg(2, b"failed to register instance pci%d\n\0".as_ptr() as *const c_char, idx); kobject_put(EDAC_PCI_TOP_MAIN_KOBJ); return err; }
    kobject_uevent(&mut (*pci).kobj, KOBJ_ADD); 0
}
unsafe fn edac_pci_unregister_sysfs_instance_kobj(pci: *mut edac_pci_ctl_info) { edac_dbg(0, b"\n\0".as_ptr() as *const c_char); kobject_put(&mut (*pci).kobj); }

unsafe fn edac_pci_int_show(ptr: *mut c_void, buffer: *mut c_char) -> ssize_t { sprintf(buffer, b"%d\n\0".as_ptr() as *const c_char, *(ptr as *mut i32)) }
unsafe fn edac_pci_int_store(ptr: *mut c_void, buffer: *const c_char, count: usize) -> ssize_t { if isdigit(*buffer as i32) != 0 { *(ptr as *mut i32) = simple_strtoul(buffer, core::ptr::null_mut(), 0) as i32; } count as ssize_t }

#[repr(C)] struct edac_pci_dev_attribute { attr: attribute, value: *mut c_void, show: Option<unsafe fn(*mut c_void,*mut c_char)->ssize_t>, store: Option<unsafe fn(*mut c_void,*const c_char,usize)->ssize_t> }
unsafe fn edac_pci_dev_show(_: *mut kobject, attr: *mut attribute, buffer: *mut c_char) -> ssize_t { let a=attr as *mut edac_pci_dev_attribute; match (*a).show { Some(f)=>f((*a).value,buffer),None=>-EIO as ssize_t } }
unsafe fn edac_pci_dev_store(_: *mut kobject, attr: *mut attribute, buffer: *const c_char, count: usize) -> ssize_t { let a=attr as *mut edac_pci_dev_attribute; match (*a).store { Some(f)=>f((*a).value,buffer,count),None=>-EIO as ssize_t } }
static EDAC_PCI_SYSFS_OPS: sysfs_ops = sysfs_ops { show: Some(edac_pci_dev_show), store: Some(edac_pci_dev_store) };

unsafe fn edac_pci_release_main_kobj(kobj: *mut kobject) { edac_dbg(0,b"here to module_put(THIS_MODULE)\n\0".as_ptr() as *const c_char); kfree(kobj as *mut c_void); module_put(THIS_MODULE); }
static KTYPE_EDAC_PCI_MAIN_KOBJ: kobj_type = kobj_type { release: Some(edac_pci_release_main_kobj), sysfs_ops: &EDAC_PCI_SYSFS_OPS, default_groups: core::ptr::null() };

unsafe fn edac_pci_main_kobj_setup() -> i32 {
    if atomic_inc_return(&mut EDAC_PCI_SYSFS_REFCOUNT) != 1 { return 0; }
    let edac_subsys = edac_get_sysfs_subsys();
    if !try_module_get(THIS_MODULE) { atomic_dec(&mut EDAC_PCI_SYSFS_REFCOUNT); return -ENODEV; }
    EDAC_PCI_TOP_MAIN_KOBJ = kzalloc_obj::<kobject>();
    if EDAC_PCI_TOP_MAIN_KOBJ.is_null() { module_put(THIS_MODULE); atomic_dec(&mut EDAC_PCI_SYSFS_REFCOUNT); return -ENOMEM; }
    let dev_root = bus_get_dev_root(edac_subsys);
    let mut err = -ENODEV;
    if !dev_root.is_null() { err=kobject_init_and_add(EDAC_PCI_TOP_MAIN_KOBJ,&KTYPE_EDAC_PCI_MAIN_KOBJ,&mut (*dev_root).kobj,b"pci\0".as_ptr() as *const c_char); put_device(dev_root); }
    if err != 0 { kobject_put(EDAC_PCI_TOP_MAIN_KOBJ); module_put(THIS_MODULE); atomic_dec(&mut EDAC_PCI_SYSFS_REFCOUNT); }
    else { kobject_uevent(EDAC_PCI_TOP_MAIN_KOBJ,KOBJ_ADD); }
    err
}
unsafe fn edac_pci_main_kobj_teardown() { if atomic_dec_return(&mut EDAC_PCI_SYSFS_REFCOUNT)==0 { kobject_put(EDAC_PCI_TOP_MAIN_KOBJ); } }

pub unsafe fn edac_pci_create_sysfs(pci: *mut edac_pci_ctl_info) -> i32 { let mut err=edac_pci_main_kobj_setup(); if err!=0{return err;} err=edac_pci_create_instance_kobj(pci,(*pci).pci_idx); if err!=0 { edac_pci_main_kobj_teardown(); return err; } err=sysfs_create_link(&mut (*pci).kobj,&mut (*(*pci).dev).kobj,EDAC_PCI_SYMLINK.as_ptr() as *const c_char); if err!=0 { edac_pci_unregister_sysfs_instance_kobj(pci); edac_pci_main_kobj_teardown(); } err }
pub unsafe fn edac_pci_remove_sysfs(pci: *mut edac_pci_ctl_info) { sysfs_remove_link(&mut (*pci).kobj,EDAC_PCI_SYMLINK.as_ptr() as *const c_char); edac_pci_unregister_sysfs_instance_kobj(pci); edac_pci_main_kobj_teardown(); }

unsafe fn get_pci_parity_status(dev: *mut pci_dev, secondary: i32) -> u16 { let where_ = if secondary!=0 {PCI_SEC_STATUS} else {PCI_STATUS}; let mut status=0u16; pci_read_config_word(dev,where_,&mut status); if status==0xffff { let mut sanity=0u32; pci_read_config_dword(dev,0,&mut sanity); if sanity==0xffffffff{return 0;} } status &= PCI_STATUS_DETECTED_PARITY|PCI_STATUS_SIG_SYSTEM_ERROR|PCI_STATUS_PARITY; if status!=0 {pci_write_config_word(dev,where_,status);} status }
unsafe fn edac_pci_dev_parity_clear(dev: *mut pci_dev) { get_pci_parity_status(dev,0); let mut header=0u8; pci_read_config_byte(dev,PCI_HEADER_TYPE,&mut header); if header&PCI_HEADER_TYPE_MASK==PCI_HEADER_TYPE_BRIDGE {get_pci_parity_status(dev,1);} }
unsafe fn edac_pci_dev_parity_test(dev: *mut pci_dev) { let mut flags=0; local_irq_save(&mut flags); let mut status=get_pci_parity_status(dev,0); let mut header=0u8; pci_read_config_byte(dev,PCI_HEADER_TYPE,&mut header); local_irq_restore(flags); if status!=0 && (*dev).broken_parity_status==0 { if status&PCI_STATUS_SIG_SYSTEM_ERROR!=0 {atomic_inc(&mut PCI_NONPARITY_COUNT);} if status&(PCI_STATUS_PARITY|PCI_STATUS_DETECTED_PARITY)!=0 {atomic_inc(&mut PCI_PARITY_COUNT);} } if header&PCI_HEADER_TYPE_MASK==PCI_HEADER_TYPE_BRIDGE {status=get_pci_parity_status(dev,1); if status!=0 && (*dev).broken_parity_status==0 {if status&PCI_STATUS_SIG_SYSTEM_ERROR!=0 {atomic_inc(&mut PCI_NONPARITY_COUNT);} if status&(PCI_STATUS_PARITY|PCI_STATUS_DETECTED_PARITY)!=0 {atomic_inc(&mut PCI_PARITY_COUNT);}}} }
unsafe fn edac_pci_dev_parity_iterator(fn_: unsafe fn(*mut pci_dev)) { let mut dev=core::ptr::null_mut(); for_each_pci_dev!(dev, fn_); }
pub unsafe fn edac_pci_do_parity_check() { if CHECK_PCI_ERRORS==0{return;} let before=atomic_read(&PCI_PARITY_COUNT); edac_pci_dev_parity_iterator(edac_pci_dev_parity_test); if edac_pci_get_panic_on_pe()!=0 && before!=atomic_read(&PCI_PARITY_COUNT) {panic(b"EDAC: PCI Parity Error\0".as_ptr() as *const c_char);} }
pub unsafe fn edac_pci_clear_parity_errors() { edac_pci_dev_parity_iterator(edac_pci_dev_parity_clear); }
pub unsafe fn edac_pci_handle_pe(pci:*mut edac_pci_ctl_info,msg:*const c_char) {atomic_inc(&mut (*pci).counters.pe_count); if edac_pci_get_log_pe()!=0 {edac_pci_printk(pci,KERN_WARNING,b"Parity Error ctl: %s %d: %s\n\0".as_ptr() as *const c_char,(*pci).ctl_name,(*pci).pci_idx,msg);} edac_pci_do_parity_check();}
pub unsafe fn edac_pci_handle_npe(pci:*mut edac_pci_ctl_info,msg:*const c_char) {atomic_inc(&mut (*pci).counters.npe_count); if edac_pci_get_log_npe()!=0 {edac_pci_printk(pci,KERN_WARNING,b"Non-Parity Error ctl: %s %d: %s\n\0".as_ptr() as *const c_char,(*pci).ctl_name,(*pci).pci_idx,msg);} edac_pci_do_parity_check();}

// module_param(check_pci_errors, int, 0644)
// MODULE_PARM_DESC(check_pci_errors, "Check for PCI bus parity errors: 0=off 1=on")
// module_param(edac_pci_panic_on_pe, int, 0644)
// MODULE_PARM_DESC(edac_pci_panic_on_pe, "Panic on PCI Bus Parity error: 0=off 1=on")

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
