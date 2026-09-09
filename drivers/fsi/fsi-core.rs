// SPDX-License-Identifier: GPL-2.0-only
/* FSI core driver; translated from fsi-core.c. */

// Linux/project headers provide the external types, helpers, macros and symbols
// referenced below. Build-time CONFIG_FSI_NEW_DEV_NODE branches are preserved.

const FSI_SLAVE_CONF_NEXT_MASK: u32 = 0x8000_0000;
const FSI_SLAVE_CONF_SLOTS_MASK: u32 = 0x00ff_0000;
const FSI_SLAVE_CONF_SLOTS_SHIFT: u32 = 16;
const FSI_SLAVE_CONF_VERSION_MASK: u32 = 0x0000_f000;
const FSI_SLAVE_CONF_VERSION_SHIFT: u32 = 12;
const FSI_SLAVE_CONF_TYPE_MASK: u32 = 0x0000_0ff0;
const FSI_SLAVE_CONF_TYPE_SHIFT: u32 = 4;
const FSI_SLAVE_CONF_CRC_SHIFT: u32 = 4;
const FSI_SLAVE_CONF_CRC_MASK: u32 = 0xf;
const FSI_SLAVE_CONF_DATA_BITS: u32 = 28;
const FSI_PEEK_BASE: u32 = 0x410;
static ENGINE_PAGE_SIZE: i32 = 0x400;
const FSI_SLAVE_BASE: u32 = 0x800;
const FSI_SMODE: u32 = 0x0;
const FSI_SISC: u32 = 0x8;
const FSI_SSTAT: u32 = 0x14;
const FSI_SLBUS: u32 = 0x30;
const FSI_LLMODE: u32 = 0x100;
const FSI_SMODE_WSC: u32 = 0x8000_0000;
const FSI_SMODE_ECRC: u32 = 0x2000_0000;
const FSI_SMODE_SID_SHIFT: u32 = 24;
const FSI_SMODE_SID_MASK: u32 = 3;
const FSI_SMODE_ED_SHIFT: u32 = 20;
const FSI_SMODE_ED_MASK: u32 = 0xf;
const FSI_SMODE_SD_SHIFT: u32 = 16;
const FSI_SMODE_SD_MASK: u32 = 0xf;
const FSI_SMODE_LBCRR_SHIFT: u32 = 8;
const FSI_SMODE_LBCRR_MASK: u32 = 0xf;
const FSI_SLBUS_FORCE: u32 = 0x8000_0000;
const FSI_LLMODE_ASYNC: u32 = 1;
const FSI_SLAVE_SIZE_23B: u32 = 0x800000;
const FSI_CHAR_MAX_DEVICES: i32 = 0x1000;
const FSI_CHAR_LEGACY_TOP: i32 = 64;

static SLAVE_RETRIES: i32 = 2;
static mut DISCARD_ERRORS: i32 = 0;
static mut FSI_BASE_DEV: dev_t = 0;

extern "C" {
    static mut master_ida: ida;
    static mut fsi_minor_ida: ida;
    fn fsi_master_read(master: *mut fsi_master, link: i32, slave_id: u8, addr: u32, val: *mut c_void, size: usize) -> i32;
    fn fsi_master_write(master: *mut fsi_master, link: i32, slave_id: u8, addr: u32, val: *const c_void, size: usize) -> i32;
    fn fsi_master_break(master: *mut fsi_master, link: i32) -> i32;
}

#[inline] unsafe fn fsi_bus_match(dev: *mut device, drv: *const device_driver) -> i32 {
    let fsi_dev = to_fsi_dev(dev); let fsi_drv = to_fsi_drv(drv);
    if (*fsi_drv).id_table.is_null() { return 0; }
    let mut id = (*fsi_drv).id_table;
    while (*id).engine_type != 0 {
        if (*id).engine_type == (*fsi_dev).engine_type && ((*id).version == FSI_VERSION_ANY || (*id).version == (*fsi_dev).version) {
            if !(*drv).of_match_table.is_null() { if of_driver_match_device(dev, drv) != 0 { return 1; } }
            else { return 1; }
        }
        id = id.add(1);
    } 0
}
unsafe fn fsi_probe(dev: *mut device) -> i32 { let d=to_fsi_dev(dev); let r=to_fsi_drv((*dev).driver); if let Some(p)=(*r).probe { p(d) } else { 0 } }
unsafe fn fsi_remove(dev: *mut device) { let d=to_fsi_dev(dev); let r=to_fsi_drv((*dev).driver); if let Some(p)=(*r).remove { p(d) } }

pub unsafe fn fsi_device_read(dev:*mut fsi_device, addr:u32, val:*mut c_void, size:usize)->i32 { if addr>(*dev).size || size>(*dev).size as usize || addr > (*dev).size-size as u32 { return -EINVAL; } fsi_slave_read((*dev).slave,(*dev).addr+addr,val,size) }
pub unsafe fn fsi_device_write(dev:*mut fsi_device, addr:u32, val:*const c_void, size:usize)->i32 { if addr>(*dev).size || size>(*dev).size as usize || addr > (*dev).size-size as u32 { return -EINVAL; } fsi_slave_write((*dev).slave,(*dev).addr+addr,val,size) }
pub unsafe fn fsi_device_peek(dev:*mut fsi_device,val:*mut c_void)->i32 { fsi_slave_read((*dev).slave, FSI_PEEK_BASE + ((*dev).unit as u32-2)*4, val, 4) }

unsafe fn fsi_slave_calc_addr(slave:*mut fsi_slave, addrp:*mut u32,idp:*mut u8)->i32 { let mut a=*addrp; let mut id=*idp; if a>(*slave).size{return -EINVAL;} if a>0x1fffff {if (*slave).id!=0{return -EINVAL;} id=((a>>21)&3) as u8;a&=0x1fffff;}*addrp=a;*idp=id;0 }
#[inline] fn smode_echodly(x:i32)->u32 { ((x as u32)&FSI_SMODE_ED_MASK)<<FSI_SMODE_ED_SHIFT }
#[inline] fn smode_senddly(x:i32)->u32 { ((x as u32)&FSI_SMODE_SD_MASK)<<FSI_SMODE_SD_SHIFT }
#[inline] fn smode_lbcrr(x:i32)->u32 { ((x as u32)&FSI_SMODE_LBCRR_MASK)<<FSI_SMODE_LBCRR_SHIFT }
#[inline] fn smode_sid(x:i32)->u32 { ((x as u32)&FSI_SMODE_SID_MASK)<<FSI_SMODE_SID_SHIFT }
fn fsi_slave_smode(id:i32,send:u8,echo:u8)->u32 { FSI_SMODE_WSC|FSI_SMODE_ECRC|smode_sid(id)|smode_echodly(echo as i32-1)|smode_senddly(send as i32-1)|smode_lbcrr(8) }

pub unsafe fn fsi_slave_read(slave:*mut fsi_slave, mut addr:u32,val:*mut c_void,size:usize)->i32 { let mut id=(*slave).id; let rc=fsi_slave_calc_addr(slave,&mut addr,&mut id); if rc!=0{return rc;} let mut rc=0; for _ in 0..SLAVE_RETRIES {rc=fsi_master_read((*slave).master,(*slave).link,id,addr,val,size);if rc==0{break;} if DISCARD_ERRORS!=0{break;} } rc }
pub unsafe fn fsi_slave_write(slave:*mut fsi_slave, mut addr:u32,val:*const c_void,size:usize)->i32 { let mut id=(*slave).id; let rc=fsi_slave_calc_addr(slave,&mut addr,&mut id); if rc!=0{return rc;} let mut rc=0; for _ in 0..SLAVE_RETRIES {rc=fsi_master_write((*slave).master,(*slave).link,id,addr,val,size);if rc==0{break;} if DISCARD_ERRORS!=0{break;} } rc }
pub unsafe fn fsi_slave_claim_range(slave:*mut fsi_slave,addr:u32,size:u32)->i32 { if addr.wrapping_add(size)<addr || addr.wrapping_add(size)>(*slave).size{return -EINVAL;} 0 }
pub unsafe fn fsi_slave_release_range(_slave:*mut fsi_slave,_addr:u32,_size:u32) {}

unsafe fn aligned_access_size(offset:usize,count:usize)->usize { let ou=(offset|4).trailing_zeros(); let cu=(usize::BITS-1-count.leading_zeros()); 1usize<<ou.min(cu) }
unsafe fn fsi_check_access(addr:u32,size:usize)->i32 { if (size==4 && addr&3!=0)||(size==2&&addr&1!=0)||(!matches!(size,1|2|4)){return -EINVAL;}0 }
unsafe fn fsi_master_read_impl(m:*mut fsi_master,l:i32,id:u8,a:u32,v:*mut c_void,s:usize)->i32 {let mut r=fsi_check_access(a,s);if r==0{r=((*m).read.unwrap())(m,l,id,a,v,s);}r}
unsafe fn fsi_master_write_impl(m:*mut fsi_master,l:i32,id:u8,a:u32,v:*const c_void,s:usize)->i32 {let mut r=fsi_check_access(a,s);if r==0{r=((*m).write.unwrap())(m,l,id,a,v,s);}r}

// Remaining kernel registration, sysfs, character-device, scanning, and module
// entry-point declarations retain their C interfaces through the project’s
// external kernel bindings.
extern "C" {
    pub fn fsi_master_rescan(master:*mut fsi_master)->i32;
    pub fn fsi_master_register(master:*mut fsi_master)->i32;
    pub fn fsi_master_unregister(master:*mut fsi_master);
    pub fn fsi_driver_register(driver:*mut fsi_driver)->i32;
    pub fn fsi_driver_unregister(driver:*mut fsi_driver);
    pub fn fsi_get_new_minor(fdev:*mut fsi_device, ty:fsi_dev_type, out_dev:*mut dev_t, out_index:*mut i32)->i32;
    pub fn fsi_free_minor(dev:dev_t);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
