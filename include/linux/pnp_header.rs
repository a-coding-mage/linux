/* SPDX-License-Identifier: GPL-2.0 */
/* Linux Plug and Play Support */

// Dependencies supplied by other translated headers.
use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

pub const PNP_NAME_LEN: usize = 50;

pub type ResourceSizeT = usize;
pub type U64 = u64;

#[repr(C)] pub struct Resource { pub start: ResourceSizeT, pub end: ResourceSizeT, pub flags: c_ulong }
#[repr(C)] pub struct Device { pub _private: [u8; 0] }
#[repr(C)] pub struct ListHead { pub next: *mut ListHead, pub prev: *mut ListHead }
#[repr(C)] pub struct ProcDirEntry { pub _private: [u8; 0] }
#[repr(C)] pub struct Mutex { pub _private: [u8; 0] }
#[repr(C)] pub struct DeviceDriver { pub _private: [u8; 0] }
#[repr(C)] pub struct PnpDeviceId { pub _private: [u8; 0] }
#[repr(C)] pub struct PnpCardDeviceId { pub _private: [u8; 0] }
#[repr(C)] pub struct AcpiDevice { pub _private: [u8; 0] }
pub type PmMessageT = c_int;

pub const IORESOURCE_DISABLED: c_ulong = 1 << 0;
pub const IORESOURCE_IO: c_ulong = 1 << 1;
pub const IORESOURCE_MEM: c_ulong = 1 << 2;
pub const IORESOURCE_IRQ: c_ulong = 1 << 3;
pub const IORESOURCE_DMA: c_ulong = 1 << 4;
pub const IORESOURCE_AUTO: c_ulong = 1 << 5;
pub const ENODEV: c_int = 19;
pub const PNP_ID_LEN: usize = 8;

extern "C" {
    pub fn resource_size(res: *mut Resource) -> ResourceSizeT;
    pub fn dev_get_drvdata(dev: *mut Device) -> *mut c_void;
    pub fn dev_set_drvdata(dev: *mut Device, data: *mut c_void);
    pub static mut console_suspend_enabled: bool;
}

#[inline] pub unsafe fn pnp_get_resource(_dev: *mut PnpDev, _ty: c_ulong, _num: c_uint) -> *mut Resource { core::ptr::null_mut() }
#[inline] pub unsafe fn pnp_resource_valid(res: *mut Resource) -> c_int { if !res.is_null() { 1 } else { 0 } }
#[inline] pub unsafe fn pnp_resource_enabled(res: *mut Resource) -> c_int { if !res.is_null() && ((*res).flags & IORESOURCE_DISABLED) == 0 { 1 } else { 0 } }
#[inline] pub unsafe fn pnp_resource_len(res: *mut Resource) -> ResourceSizeT { if (*res).start == 0 && (*res).end == 0 { 0 } else { resource_size(res) } }

#[inline] pub unsafe fn pnp_port_start(dev: *mut PnpDev, bar: c_uint) -> ResourceSizeT { let r=pnp_get_resource(dev,IORESOURCE_IO,bar); if pnp_resource_valid(r)!=0 {(*r).start} else {0} }
#[inline] pub unsafe fn pnp_port_end(dev: *mut PnpDev, bar: c_uint) -> ResourceSizeT { let r=pnp_get_resource(dev,IORESOURCE_IO,bar); if pnp_resource_valid(r)!=0 {(*r).end} else {0} }
#[inline] pub unsafe fn pnp_port_flags(dev: *mut PnpDev, bar: c_uint) -> c_ulong { let r=pnp_get_resource(dev,IORESOURCE_IO,bar); if pnp_resource_valid(r)!=0 {(*r).flags} else {IORESOURCE_IO|IORESOURCE_AUTO} }
#[inline] pub unsafe fn pnp_port_valid(dev: *mut PnpDev, bar: c_uint) -> c_int { pnp_resource_valid(pnp_get_resource(dev,IORESOURCE_IO,bar)) }
#[inline] pub unsafe fn pnp_port_len(dev: *mut PnpDev, bar: c_uint) -> ResourceSizeT { let r=pnp_get_resource(dev,IORESOURCE_IO,bar); if pnp_resource_valid(r)!=0 {pnp_resource_len(r)} else {0} }
#[inline] pub unsafe fn pnp_mem_start(dev: *mut PnpDev, bar: c_uint) -> ResourceSizeT { let r=pnp_get_resource(dev,IORESOURCE_MEM,bar); if pnp_resource_valid(r)!=0 {(*r).start} else {0} }
#[inline] pub unsafe fn pnp_mem_end(dev: *mut PnpDev, bar: c_uint) -> ResourceSizeT { let r=pnp_get_resource(dev,IORESOURCE_MEM,bar); if pnp_resource_valid(r)!=0 {(*r).end} else {0} }
#[inline] pub unsafe fn pnp_mem_flags(dev: *mut PnpDev, bar: c_uint) -> c_ulong { let r=pnp_get_resource(dev,IORESOURCE_MEM,bar); if pnp_resource_valid(r)!=0 {(*r).flags} else {IORESOURCE_MEM|IORESOURCE_AUTO} }
#[inline] pub unsafe fn pnp_mem_valid(dev: *mut PnpDev, bar: c_uint) -> c_int { pnp_resource_valid(pnp_get_resource(dev,IORESOURCE_MEM,bar)) }
#[inline] pub unsafe fn pnp_mem_len(dev: *mut PnpDev, bar: c_uint) -> ResourceSizeT { let r=pnp_get_resource(dev,IORESOURCE_MEM,bar); if pnp_resource_valid(r)!=0 {pnp_resource_len(r)} else {0} }
#[inline] pub unsafe fn pnp_irq(dev: *mut PnpDev, bar: c_uint) -> ResourceSizeT { let r=pnp_get_resource(dev,IORESOURCE_IRQ,bar); if pnp_resource_valid(r)!=0 {(*r).start} else {ResourceSizeT::MAX} }
#[inline] pub unsafe fn pnp_irq_flags(dev: *mut PnpDev, bar: c_uint) -> c_ulong { let r=pnp_get_resource(dev,IORESOURCE_IRQ,bar); if pnp_resource_valid(r)!=0 {(*r).flags} else {IORESOURCE_IRQ|IORESOURCE_AUTO} }
#[inline] pub unsafe fn pnp_irq_valid(dev: *mut PnpDev, bar: c_uint) -> c_int { pnp_resource_valid(pnp_get_resource(dev,IORESOURCE_IRQ,bar)) }
#[inline] pub unsafe fn pnp_dma(dev: *mut PnpDev, bar: c_uint) -> ResourceSizeT { let r=pnp_get_resource(dev,IORESOURCE_DMA,bar); if pnp_resource_valid(r)!=0 {(*r).start} else {ResourceSizeT::MAX} }
#[inline] pub unsafe fn pnp_dma_flags(dev: *mut PnpDev, bar: c_uint) -> c_ulong { let r=pnp_get_resource(dev,IORESOURCE_DMA,bar); if pnp_resource_valid(r)!=0 {(*r).flags} else {IORESOURCE_DMA|IORESOURCE_AUTO} }
#[inline] pub unsafe fn pnp_dma_valid(dev: *mut PnpDev, bar: c_uint) -> c_int { pnp_resource_valid(pnp_get_resource(dev,IORESOURCE_DMA,bar)) }

#[repr(C)] pub struct PnpCard { pub dev: Device, pub number: u8, pub global_list: ListHead, pub protocol_list: ListHead, pub devices: ListHead, pub protocol: *mut PnpProtocol, pub id: *mut PnpId, pub name: [c_char;PNP_NAME_LEN], pub pnpver:u8, pub productver:u8, pub serial:c_uint, pub checksum:u8, pub procdir:*mut ProcDirEntry }
#[repr(C)] pub struct PnpCardLink { pub card:*mut PnpCard, pub driver:*mut PnpCardDriver, pub driver_data:*mut c_void, pub pm_state:PmMessageT }
#[repr(C)] pub struct PnpDev { pub dev:Device, pub dma_mask:U64, pub number:c_uint, pub status:c_int, pub global_list:ListHead, pub protocol_list:ListHead, pub card_list:ListHead, pub rdev_list:ListHead, pub protocol:*mut PnpProtocol, pub card:*mut PnpCard, pub driver:*mut PnpDriver, pub card_link:*mut PnpCardLink, pub id:*mut PnpId, pub active:c_int, pub capabilities:c_int, pub num_dependent_sets:c_uint, pub resources:ListHead, pub options:ListHead, pub name:[c_char;PNP_NAME_LEN], pub flags:c_int, pub procent:*mut ProcDirEntry, pub data:*mut c_void }
#[repr(C)] pub struct PnpFixup { pub id:[c_char;8], pub quirk_function:Option<unsafe extern "C" fn(*mut PnpDev)> }
#[repr(C)] pub struct PnpId { pub id:[c_char;PNP_ID_LEN], pub next:*mut PnpId }
#[repr(C)] pub struct PnpDriver { pub name:*const c_char, pub id_table:*const PnpDeviceId, pub flags:c_uint, pub probe:Option<unsafe extern "C" fn(*mut PnpDev,*const PnpDeviceId)->c_int>, pub remove:Option<unsafe extern "C" fn(*mut PnpDev)>, pub shutdown:Option<unsafe extern "C" fn(*mut PnpDev)>, pub suspend:Option<unsafe extern "C" fn(*mut PnpDev,PmMessageT)->c_int>, pub resume:Option<unsafe extern "C" fn(*mut PnpDev)->c_int>, pub driver:DeviceDriver }
#[repr(C)] pub struct PnpCardDriver { pub global_list:ListHead, pub name:*mut c_char, pub id_table:*const PnpCardDeviceId, pub flags:c_uint, pub probe:Option<unsafe extern "C" fn(*mut PnpCardLink,*const PnpCardDeviceId)->c_int>, pub remove:Option<unsafe extern "C" fn(*mut PnpCardLink)>, pub suspend:Option<unsafe extern "C" fn(*mut PnpCardLink,PmMessageT)->c_int>, pub resume:Option<unsafe extern "C" fn(*mut PnpCardLink)->c_int>, pub link:PnpDriver }
#[repr(C)] pub struct PnpProtocol { pub protocol_list:ListHead, pub name:*mut c_char, pub get:Option<unsafe extern "C" fn(*mut PnpDev)->c_int>, pub set:Option<unsafe extern "C" fn(*mut PnpDev)->c_int>, pub disable:Option<unsafe extern "C" fn(*mut PnpDev)->c_int>, pub can_wakeup:Option<unsafe extern "C" fn(*mut PnpDev)->bool>, pub suspend:Option<unsafe extern "C" fn(*mut PnpDev,PmMessageT)->c_int>, pub resume:Option<unsafe extern "C" fn(*mut PnpDev)->c_int>, pub number:u8, pub dev:Device, pub cards:ListHead, pub devices:ListHead }

pub const PNP_CONFIG_NORMAL:c_uint=0x0001; pub const PNP_CONFIG_FORCE:c_uint=0x0002;
pub const PNP_READ:c_int=0x0001; pub const PNP_WRITE:c_int=0x0002; pub const PNP_DISABLE:c_int=0x0004; pub const PNP_CONFIGURABLE:c_int=0x0008; pub const PNP_REMOVABLE:c_int=0x0010; pub const PNP_CONSOLE:c_int=0x0020;
pub const PNP_READY:c_int=0; pub const PNP_ATTACHED:c_int=1; pub const PNP_BUSY:c_int=2; pub const PNP_FAULTY:c_int=4;
pub const PNP_DRIVER_RES_DO_NOT_CHANGE:c_uint=1; pub const PNP_DRIVER_RES_DISABLE:c_uint=3;

#[inline] pub unsafe fn pnp_get_card_drvdata(p:*mut PnpCardLink)->*mut c_void {(*p).driver_data}
#[inline] pub unsafe fn pnp_set_card_drvdata(p:*mut PnpCardLink,d:*mut c_void){(*p).driver_data=d}
#[inline] pub unsafe fn pnp_get_drvdata(p:*mut PnpDev)->*mut c_void {dev_get_drvdata(&mut (*p).dev)}
#[inline] pub unsafe fn pnp_set_drvdata(p:*mut PnpDev,d:*mut c_void){dev_set_drvdata(&mut (*p).dev,d)}

#[inline] pub unsafe fn isapnp_card_number(dev:*mut PnpDev)->c_int { if !(*dev).card.is_null() {(*(*dev).card).number as c_int} else {-1} }
#[inline] pub unsafe fn isapnp_csn_number(dev:*mut PnpDev)->c_uint {(*dev).number}

#[cfg(CONFIG_PNP)] extern "C" {
    pub fn pnp_device_attach(dev:*mut PnpDev)->c_int;
    pub fn pnp_device_detach(dev:*mut PnpDev);
    pub static mut pnp_global:ListHead;
    pub static mut pnp_platform_devices:c_int;
    pub fn pnp_request_card_device(clink:*mut PnpCardLink,id:*const c_char,from:*mut PnpDev)->*mut PnpDev;
    pub fn pnp_release_card_device(dev:*mut PnpDev);
    pub fn pnp_register_card_driver(drv:*mut PnpCardDriver)->c_int;
    pub fn pnp_unregister_card_driver(drv:*mut PnpCardDriver);
    pub static mut pnp_cards:ListHead;
    pub fn pnp_possible_config(dev:*mut PnpDev,ty:c_int,base:ResourceSizeT,size:ResourceSizeT)->c_int;
    pub fn pnp_auto_config_dev(dev:*mut PnpDev)->c_int;
    pub fn pnp_start_dev(dev:*mut PnpDev)->c_int;
    pub fn pnp_stop_dev(dev:*mut PnpDev)->c_int;
    pub fn pnp_activate_dev(dev:*mut PnpDev)->c_int;
    pub fn pnp_disable_dev(dev:*mut PnpDev)->c_int;
    pub fn pnp_range_reserved(start:ResourceSizeT,end:ResourceSizeT)->c_int;
    pub fn pnp_is_active(dev:*mut PnpDev)->c_int;
    pub fn compare_pnp_id(pos:*mut PnpId,id:*const c_char)->c_int;
    pub fn pnp_register_driver(drv:*mut PnpDriver)->c_int;
    pub fn pnp_unregister_driver(drv:*mut PnpDriver);
    pub fn dev_is_pnp(dev:*const Device)->bool;
}

#[cfg(not(CONFIG_PNP))] pub static pnp_platform_devices:c_int=0;
#[cfg(not(CONFIG_PNP))] pub unsafe fn pnp_device_attach(_: *mut PnpDev)->c_int{-ENODEV}
#[cfg(not(CONFIG_PNP))] pub unsafe fn pnp_device_detach(_: *mut PnpDev){}
#[cfg(not(CONFIG_PNP))] pub unsafe fn pnp_request_card_device(_: *mut PnpCardLink,_:*const c_char,_:*mut PnpDev)->*mut PnpDev{core::ptr::null_mut()}
#[cfg(not(CONFIG_PNP))] pub unsafe fn pnp_release_card_device(_: *mut PnpDev){}
#[cfg(not(CONFIG_PNP))] pub unsafe fn pnp_register_card_driver(_: *mut PnpCardDriver)->c_int{-ENODEV}
#[cfg(not(CONFIG_PNP))] pub unsafe fn pnp_unregister_card_driver(_: *mut PnpCardDriver){}
#[cfg(not(CONFIG_PNP))] pub unsafe fn pnp_possible_config(_: *mut PnpDev,_:c_int,_:ResourceSizeT,_:ResourceSizeT)->c_int{0}
#[cfg(not(CONFIG_PNP))] pub unsafe fn pnp_auto_config_dev(_: *mut PnpDev)->c_int{-ENODEV}
#[cfg(not(CONFIG_PNP))] pub unsafe fn pnp_start_dev(_: *mut PnpDev)->c_int{-ENODEV}
#[cfg(not(CONFIG_PNP))] pub unsafe fn pnp_stop_dev(_: *mut PnpDev)->c_int{-ENODEV}
#[cfg(not(CONFIG_PNP))] pub unsafe fn pnp_activate_dev(_: *mut PnpDev)->c_int{-ENODEV}
#[cfg(not(CONFIG_PNP))] pub unsafe fn pnp_disable_dev(_: *mut PnpDev)->c_int{-ENODEV}
#[cfg(not(CONFIG_PNP))] pub unsafe fn pnp_range_reserved(_:ResourceSizeT,_:ResourceSizeT)->c_int{0}
#[cfg(not(CONFIG_PNP))] pub unsafe fn pnp_is_active(_: *mut PnpDev)->c_int{0}
#[cfg(not(CONFIG_PNP))] pub unsafe fn compare_pnp_id(_: *mut PnpId,_:*const c_char)->c_int{-ENODEV}
#[cfg(not(CONFIG_PNP))] pub unsafe fn pnp_register_driver(_: *mut PnpDriver)->c_int{-ENODEV}
#[cfg(not(CONFIG_PNP))] pub unsafe fn pnp_unregister_driver(_: *mut PnpDriver){}
#[cfg(not(CONFIG_PNP))] pub unsafe fn dev_is_pnp(_: *const Device)->bool{false}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
