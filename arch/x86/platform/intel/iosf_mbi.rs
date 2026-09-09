// SPDX-License-Identifier: GPL-2.0-only
/* IOSF-SB MailBox Interface Driver -- source-level Rust translation. */

// Kernel headers and build-time definitions are supplied by the surrounding tree.

use core::ffi::c_void;

const PCI_DEVICE_ID_INTEL_BAYTRAIL: u16 = 0x0F00;
const PCI_DEVICE_ID_INTEL_BRASWELL: u16 = 0x2280;
const PCI_DEVICE_ID_INTEL_QUARK_X1000: u16 = 0x0958;
const PCI_DEVICE_ID_INTEL_TANGIER: u16 = 0x1170;
const SEMAPHORE_TIMEOUT: u32 = 500;
const PUNIT_SEMAPHORE_BYT: u32 = 0x7;
const PUNIT_SEMAPHORE_CHT: u32 = 0x10e;
const PUNIT_SEMAPHORE_BIT: u32 = 1 << 0;
const PUNIT_SEMAPHORE_ACQUIRE: u32 = 1 << 1;

#[repr(C)] pub struct pci_dev { _private: [u8; 0] }
#[repr(C)] pub struct pci_device_id { pub vendor: u32, pub device: u32, pub subvendor: u32, pub subdevice: u32, pub class: u32, pub class_mask: u32, pub driver_data: usize }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct notifier_block { _private: [u8; 0] }
#[repr(C)] pub struct pm_qos_request { _private: [u8; 0] }
#[repr(C)] pub struct dentry { _private: [u8; 0] }
#[repr(C)] pub struct file_operations { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct wait_queue_head_t { _private: [u8; 0] }
#[repr(C)] pub struct pci_driver { pub name: *const u8, pub probe: Option<unsafe extern "C" fn(*mut pci_dev, *const pci_device_id) -> i32>, pub id_table: *const pci_device_id }

extern "C" {
    static mut mbi_pdev: *mut pci_dev;
    static mut iosf_mbi_lock: spinlock_t;
    static mut iosf_mbi_pmic_access_mutex: mutex;
    static mut iosf_mbi_pmic_bus_access_notifier: c_void;
    static mut iosf_mbi_pmic_access_waitq: wait_queue_head_t;
    static mut iosf_mbi_pmic_punit_access_count: u32;
    static mut iosf_mbi_pmic_i2c_access_count: u32;
    static mut iosf_mbi_sem_address: u32;
    static mut iosf_mbi_sem_acquired: usize;
    static mut iosf_mbi_pm_qos: pm_qos_request;
    fn pci_write_config_dword(dev: *mut pci_dev, where_: u32, val: u32) -> i32;
    fn pci_read_config_dword(dev: *mut pci_dev, where_: u32, val: *mut u32) -> i32;
    fn pcibios_err_to_errno(err: i32) -> i32;
    fn pci_enable_device(dev: *mut pci_dev) -> i32;
    fn pci_dev_get(dev: *mut pci_dev) -> *mut pci_dev;
    fn pci_dev_put(dev: *mut pci_dev);
    fn pci_register_driver(drv: *mut pci_driver) -> i32;
    fn pci_unregister_driver(drv: *mut pci_driver);
    fn mutex_lock(m: *mut mutex); fn mutex_unlock(m: *mut mutex);
    fn wait_event(q: *mut wait_queue_head_t, condition: bool);
    fn wake_up(q: *mut wait_queue_head_t);
    fn spin_lock_irqsave(l: *mut spinlock_t, flags: *mut usize);
    fn spin_unlock_irqrestore(l: *mut spinlock_t, flags: usize);
    fn jiffies() -> usize; fn msecs_to_jiffies(v: u32) -> usize; fn jiffies_to_msecs(v: usize) -> u32;
    fn usleep_range(min: u32, max: u32);
    fn cpu_latency_qos_update_request(r: *mut pm_qos_request, v: i32);
    fn cpu_latency_qos_add_request(r: *mut pm_qos_request, v: i32);
    fn cpu_latency_qos_remove_request(r: *mut pm_qos_request);
    fn blocking_notifier_call_chain(h: *mut c_void, val: u32, v: *mut c_void) -> i32;
    fn blocking_notifier_chain_register(h: *mut c_void, nb: *mut notifier_block) -> i32;
    fn blocking_notifier_chain_unregister(h: *mut c_void, nb: *mut notifier_block) -> i32;
    fn debugfs_remove_recursive(d: *mut dentry);
    fn debugfs_create_dir(name: *const u8, parent: *mut dentry) -> *mut dentry;
    fn debugfs_create_x32(name: *const u8, mode: u32, parent: *mut dentry, data: *mut u32) -> *mut dentry;
    fn debugfs_create_file(name: *const u8, mode: u32, parent: *mut dentry, data: *mut c_void, fops: *const file_operations) -> *mut dentry;
    fn capable(cap: u32) -> bool;
    fn pci_dev_name(dev: *mut pci_dev) -> *mut device;
    fn dev_err(dev: *mut device, fmt: *const u8, ...);
    fn dev_dbg(dev: *mut device, fmt: *const u8, ...);
}

const ENODEV: i32 = -19; const EPERM: i32 = -1; const ENXIO: i32 = -6; const ETIMEDOUT: i32 = -110;
const MBI_MCRX_OFFSET: u32 = 0; const MBI_MCR_OFFSET: u32 = 4; const MBI_MDR_OFFSET: u32 = 8;
const MBI_ENABLE: u32 = 1; const MBI_MASK_LO: u32 = 0x0000_ff00; const MBI_MASK_HI: u32 = 0xffff_0000;
const MBI_RD_MASK: u32 = 0; const MBI_WR_MASK: u32 = 1 << 24;
const BT_MBI_UNIT_GFX: u8 = 0; const BT_MBI_UNIT_PMC: u8 = 1; const MBI_REG_READ: u8 = 0; const MBI_REG_WRITE: u8 = 1;
const MBI_PMIC_BUS_ACCESS_BEGIN: u32 = 0; const MBI_PMIC_BUS_ACCESS_END: u32 = 1; const PM_QOS_DEFAULT_VALUE: i32 = 0;

#[inline] unsafe fn iosf_mbi_form_mcr(op: u8, port: u8, offset: u8) -> u32 { ((op as u32) << 24) | ((port as u32) << 16) | ((offset as u32) << 8) | MBI_ENABLE }
unsafe fn iosf_mbi_pci_read_mdr(mcrx: u32, mcr: u32, mdr: *mut u32) -> i32 { if mbi_pdev.is_null() { return ENODEV; } let mut r; if mcrx != 0 { r=pci_write_config_dword(mbi_pdev,MBI_MCRX_OFFSET,mcrx); if r<0{return r;} } r=pci_write_config_dword(mbi_pdev,MBI_MCR_OFFSET,mcr); if r<0{return r;} r=pci_read_config_dword(mbi_pdev,MBI_MDR_OFFSET,mdr); if r<0{return r;} 0 }
unsafe fn iosf_mbi_pci_write_mdr(mcrx: u32, mcr: u32, mdr: u32) -> i32 { if mbi_pdev.is_null(){return ENODEV;} let mut r=pci_write_config_dword(mbi_pdev,MBI_MDR_OFFSET,mdr); if r<0{return r;} if mcrx!=0 {r=pci_write_config_dword(mbi_pdev,MBI_MCRX_OFFSET,mcrx);if r<0{return r;}} r=pci_write_config_dword(mbi_pdev,MBI_MCR_OFFSET,mcr);if r<0{return r;} 0 }

#[no_mangle] pub unsafe extern "C" fn iosf_mbi_read(port:u8, opcode:u8, offset:u32, mdr:*mut u32)->i32 { if port==BT_MBI_UNIT_GFX{return EPERM;} let mcr=iosf_mbi_form_mcr(opcode,port,(offset&MBI_MASK_LO) as u8); let mcrx=offset&MBI_MASK_HI; let mut f=0; spin_lock_irqsave(&mut iosf_mbi_lock,&mut f); let r=iosf_mbi_pci_read_mdr(mcrx,mcr,mdr);spin_unlock_irqrestore(&mut iosf_mbi_lock,f);r }
#[no_mangle] pub unsafe extern "C" fn iosf_mbi_write(port:u8,opcode:u8,offset:u32,mdr:u32)->i32 { if port==BT_MBI_UNIT_GFX{return EPERM;} let mcr=iosf_mbi_form_mcr(opcode,port,(offset&MBI_MASK_LO) as u8);let mcrx=offset&MBI_MASK_HI;let mut f=0;spin_lock_irqsave(&mut iosf_mbi_lock,&mut f);let r=iosf_mbi_pci_write_mdr(mcrx,mcr,mdr);spin_unlock_irqrestore(&mut iosf_mbi_lock,f);r }
#[no_mangle] pub unsafe extern "C" fn iosf_mbi_modify(port:u8,opcode:u8,offset:u32,mdr:u32,mask:u32)->i32 { if port==BT_MBI_UNIT_GFX{return EPERM;} let mcr=iosf_mbi_form_mcr(opcode,port,(offset&MBI_MASK_LO) as u8);let mcrx=offset&MBI_MASK_HI;let mut f=0;spin_lock_irqsave(&mut iosf_mbi_lock,&mut f);let mut v=0;let r=iosf_mbi_pci_read_mdr(mcrx,mcr&MBI_RD_MASK,&mut v);if r<0{spin_unlock_irqrestore(&mut iosf_mbi_lock,f);return r;}v=(v&!mask)|(mdr&mask);let r=iosf_mbi_pci_write_mdr(mcrx,mcr|MBI_WR_MASK,v);spin_unlock_irqrestore(&mut iosf_mbi_lock,f);r }
#[no_mangle] pub unsafe extern "C" fn iosf_mbi_available()->bool { !mbi_pdev.is_null() }

#[no_mangle] pub unsafe extern "C" fn iosf_mbi_punit_acquire(){mutex_lock(&mut iosf_mbi_pmic_access_mutex);while iosf_mbi_pmic_i2c_access_count!=0{mutex_unlock(&mut iosf_mbi_pmic_access_mutex);wait_event(&mut iosf_mbi_pmic_access_waitq,iosf_mbi_pmic_i2c_access_count==0);mutex_lock(&mut iosf_mbi_pmic_access_mutex);}iosf_mbi_pmic_punit_access_count+=1;mutex_unlock(&mut iosf_mbi_pmic_access_mutex)}
#[no_mangle] pub unsafe extern "C" fn iosf_mbi_punit_release(){mutex_lock(&mut iosf_mbi_pmic_access_mutex);iosf_mbi_pmic_punit_access_count-=1;let w=iosf_mbi_pmic_punit_access_count==0;mutex_unlock(&mut iosf_mbi_pmic_access_mutex);if w{wake_up(&mut iosf_mbi_pmic_access_waitq)}}

#[no_mangle] pub unsafe extern "C" fn iosf_mbi_block_punit_i2c_access()->i32 { if mbi_pdev.is_null()||iosf_mbi_sem_address==0{return ENXIO;} mutex_lock(&mut iosf_mbi_pmic_access_mutex);while iosf_mbi_pmic_punit_access_count!=0{mutex_unlock(&mut iosf_mbi_pmic_access_mutex);wait_event(&mut iosf_mbi_pmic_access_waitq,iosf_mbi_pmic_punit_access_count==0);mutex_lock(&mut iosf_mbi_pmic_access_mutex);}iosf_mbi_pmic_i2c_access_count+=1;mutex_unlock(&mut iosf_mbi_pmic_access_mutex);0 }
#[no_mangle] pub unsafe extern "C" fn iosf_mbi_unblock_punit_i2c_access(){mutex_lock(&mut iosf_mbi_pmic_access_mutex);iosf_mbi_pmic_i2c_access_count-=1;let w=iosf_mbi_pmic_i2c_access_count==0;mutex_unlock(&mut iosf_mbi_pmic_access_mutex);if w{wake_up(&mut iosf_mbi_pmic_access_waitq)}}
#[no_mangle] pub unsafe extern "C" fn iosf_mbi_register_pmic_bus_access_notifier(nb:*mut notifier_block)->i32{iosf_mbi_punit_acquire();let r=blocking_notifier_chain_register(&mut iosf_mbi_pmic_bus_access_notifier,nb);iosf_mbi_punit_release();r}
#[no_mangle] pub unsafe extern "C" fn iosf_mbi_unregister_pmic_bus_access_notifier_unlocked(nb:*mut notifier_block)->i32{iosf_mbi_assert_punit_acquired();blocking_notifier_chain_unregister(&mut iosf_mbi_pmic_bus_access_notifier,nb)}
#[no_mangle] pub unsafe extern "C" fn iosf_mbi_assert_punit_acquired(){ }

#[repr(C)] struct pci_driver_real { name:*const u8, probe:Option<unsafe extern "C" fn(*mut pci_dev,*const pci_device_id)->i32>, id_table:*const pci_device_id }
static mut iosf_mbi_pci_ids:[pci_device_id;5]=[
 pci_device_id{vendor:0,device:PCI_DEVICE_ID_INTEL_BAYTRAIL as u32,subvendor:0,subdevice:0,class:0,class_mask:0,driver_data:PUNIT_SEMAPHORE_BYT as usize},
 pci_device_id{vendor:0,device:PCI_DEVICE_ID_INTEL_BRASWELL as u32,subvendor:0,subdevice:0,class:0,class_mask:0,driver_data:PUNIT_SEMAPHORE_CHT as usize},
 pci_device_id{vendor:0,device:PCI_DEVICE_ID_INTEL_QUARK_X1000 as u32,subvendor:0,subdevice:0,class:0,class_mask:0,driver_data:0},
 pci_device_id{vendor:0,device:PCI_DEVICE_ID_INTEL_TANGIER as u32,subvendor:0,subdevice:0,class:0,class_mask:0,driver_data:0},
 pci_device_id{vendor:0,device:0,subvendor:0,subdevice:0,class:0,class_mask:0,driver_data:0},
];
unsafe extern "C" fn iosf_mbi_probe(pdev:*mut pci_dev,dev_id:*const pci_device_id)->i32 { let r=pci_enable_device(pdev);if r<0{return r;}mbi_pdev=pci_dev_get(pdev);iosf_mbi_sem_address=(*dev_id).driver_data as u32;0 }
static mut iosf_mbi_pci_driver:pci_driver_real=pci_driver_real{name:b"iosf_mbi_pci\0".as_ptr(),probe:Some(iosf_mbi_probe),id_table:unsafe{iosf_mbi_pci_ids.as_ptr()}};
#[no_mangle] pub unsafe extern "C" fn iosf_mbi_init()->i32 {cpu_latency_qos_add_request(&mut iosf_mbi_pm_qos,PM_QOS_DEFAULT_VALUE);pci_register_driver(&mut iosf_mbi_pci_driver as *mut _ as *mut pci_driver)}
#[no_mangle] pub unsafe extern "C" fn iosf_mbi_exit(){pci_unregister_driver(&mut iosf_mbi_pci_driver as *mut _ as *mut pci_driver);if !mbi_pdev.is_null(){pci_dev_put(mbi_pdev);}mbi_pdev=core::ptr::null_mut();cpu_latency_qos_remove_request(&mut iosf_mbi_pm_qos)}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
