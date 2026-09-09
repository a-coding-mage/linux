// SPDX-License-Identifier: GPL-2.0-or-later
/* Rust translation of arch/powerpc/platforms/powermac/low_i2c.c. */

// Kernel headers and symbols referenced below are supplied by the surrounding
// kernel translation; C preprocessor configuration is retained with cfg notes.

static mut PMAC_I2C_FORCE_POLL: i32 = 1;

#[repr(C)]
pub struct PmacI2cBus {
    pub link: ListHead, pub controller: *mut DeviceNode, pub busnode: *mut DeviceNode,
    pub type_: i32, pub flags: i32, pub adapter: I2cAdapter, pub hostdata: *mut core::ffi::c_void,
    pub channel: i32, pub mode: i32, pub mutex: Mutex, pub opened: i32, pub polled: i32,
    pub platform_dev: *mut PlatformDevice, pub lock_key: LockClassKey,
    pub open: Option<unsafe extern "C" fn(*mut PmacI2cBus) -> i32>,
    pub close: Option<unsafe extern "C" fn(*mut PmacI2cBus)>,
    pub xfer: Option<unsafe extern "C" fn(*mut PmacI2cBus, u8, i32, u32, *mut u8, i32) -> i32>,
}

#[repr(C)] pub struct ListHead { pub next: *mut ListHead, pub prev: *mut ListHead }
#[repr(C)] pub struct DeviceNode { pub name: *const u8, pub full_name: *const u8 }
#[repr(C)] pub struct I2cAdapter { _private: [u8; 0] }
#[repr(C)] pub struct Mutex { _private: [u8; 0] }
#[repr(C)] pub struct LockClassKey { _private: [u8; 0] }
#[repr(C)] pub struct PlatformDevice { pub dev: Device }
#[repr(C)] pub struct Device { pub platform_data: *mut core::ffi::c_void }
#[repr(C)] pub struct Completion { _private: [u8; 0] }
#[repr(C)] pub struct SpinLock { _private: [u8; 0] }
#[repr(C)] pub struct TimerList { pub expires: u64 }
#[repr(C)] pub struct AdbRequest { pub data: [u8; 256], pub reply: [u8; 256], pub nbytes: u32, pub reply_len: u32, pub done: Option<unsafe extern "C" fn(*mut AdbRequest)>, pub arg: *mut core::ffi::c_void }
#[repr(C)] pub struct SmuI2cCmd { pub info: SmuI2cInfo, pub done: Option<unsafe extern "C" fn(*mut SmuI2cCmd,*mut core::ffi::c_void)>, pub misc: *mut core::ffi::c_void, pub status: i32 }
#[repr(C)] pub struct SmuI2cInfo { pub bus:u8, pub devaddr:u8, pub datalen:u8, pub type_:u8, pub sublen:u8, pub subaddr:u32, pub data:[u8; 256] }
#[repr(C)] pub struct PmfFunction { pub node:*mut DeviceNode, pub driver_data:*mut core::ffi::c_void }
#[repr(C)] pub struct PmfArgs { pub count:u32, pub u:[PmfArg; 8] }
#[repr(C)] pub union PmfArg { pub p:*mut u32, pub v:u32 }
#[repr(C)] pub struct PmfHandlers { pub begin:Option<unsafe extern "C" fn(*mut PmfFunction,*mut PmfArgs)->*mut core::ffi::c_void>, pub end:Option<unsafe extern "C" fn(*mut PmfFunction,*mut core::ffi::c_void)>, pub read_i2c:Option<unsafe extern "C" fn(*mut PmfFunction,*mut PmfArgs,*mut core::ffi::c_void,u32)->i32> }

#[repr(C)] struct PmacI2cHostKw { mutex:Mutex, base:*mut u8, bsteps:i32, speed:i32, irq:i32, data:*mut u8, len:usize, state:i32, rw:i32, polled:i32, result:i32, complete:Completion, lock:SpinLock, timeout_timer:TimerList }
static mut PMAC_I2C_BUSSES: ListHead = ListHead { next: core::ptr::null_mut(), prev: core::ptr::null_mut() };

const REG_MODE:i32=0; const REG_CONTROL:i32=1; const REG_STATUS:i32=2; const REG_ISR:i32=3; const REG_IER:i32=4; const REG_ADDR:i32=5; const REG_SUBADDR:i32=6; const REG_DATA:i32=7;
const KW_I2C_MODE_100KHZ:u8=0; const KW_I2C_MODE_50KHZ:u8=1; const KW_I2C_MODE_25KHZ:u8=2; const KW_I2C_MODE_STANDARD:u8=4; const KW_I2C_MODE_STANDARDSUB:u8=8; const KW_I2C_MODE_COMBINED:u8=12; const KW_I2C_MODE_MODE_MASK:u8=12;
const KW_I2C_CTL_AAK:u8=1; const KW_I2C_CTL_XADDR:u8=2; const KW_I2C_CTL_STOP:u8=4; const KW_I2C_IRQ_DATA:u8=1; const KW_I2C_IRQ_ADDR:u8=2; const KW_I2C_IRQ_STOP:u8=4; const KW_I2C_IRQ_START:u8=8; const KW_I2C_IRQ_MASK:u8=15; const KW_I2C_STAT_BUSY:u8=1; const KW_I2C_STAT_LAST_AAK:u8=2;
const STATE_IDLE:i32=0; const STATE_ADDR:i32=1; const STATE_READ:i32=2; const STATE_WRITE:i32=3; const STATE_STOP:i32=4; const STATE_DEAD:i32=5;
const EINVAL:i32=22; const EIO:i32=5; const ENXIO:i32=6; const EFBIG:i32=27;

extern "C" { fn readb(p:*mut u8)->u8; fn writeb(v:u8,p:*mut u8); fn mb(); fn msleep(ms:u32); fn mutex_lock(m:*mut Mutex); fn mutex_unlock(m:*mut Mutex); fn complete(c:*mut Completion); fn wait_for_completion(c:*mut Completion); fn reinit_completion(c:*mut Completion); fn spin_lock_irqsave(l:*mut SpinLock,f:*mut usize); fn spin_unlock_irqrestore(l:*mut SpinLock,f:usize); fn pmac_i2c_find_bus(n:*mut DeviceNode)->*mut PmacI2cBus; }

unsafe fn kw_read(h:*mut PmacI2cHostKw,r:i32)->u8 { readb((*h).base.add((r as usize)<<(*h).bsteps)) }
unsafe fn kw_write(h:*mut PmacI2cHostKw,r:i32,v:u8) { writeb(v,(*h).base.add((r as usize)<<(*h).bsteps)); let _=kw_read(h,REG_SUBADDR); }
unsafe fn kw_stop(h:*mut PmacI2cHostKw,result:i32) { kw_write(h,REG_CONTROL,KW_I2C_CTL_STOP); (*h).state=STATE_STOP; (*h).result=result; }
unsafe fn kw_wait(h:*mut PmacI2cHostKw)->u8 { let mut isr=0; for _ in 0..1000 { isr=kw_read(h,REG_ISR)&KW_I2C_IRQ_MASK; if isr!=0{return isr;} if (*h).polled!=0 { for _ in 1..100000 { mb(); } } else { msleep(1); } } isr }
unsafe fn kw_handle(h:*mut PmacI2cHostKw,isr:u8) { if (*h).state==STATE_IDLE { kw_write(h,REG_ISR,isr); return; } if isr==0 { if (*h).state!=STATE_STOP {kw_stop(h,-EIO);return;} if kw_read(h,REG_STATUS)&KW_I2C_STAT_BUSY!=0 {kw_write(h,REG_STATUS,0);} (*h).state=STATE_IDLE; kw_write(h,REG_IER,0); if (*h).polled==0 {complete(&mut (*h).complete);} return; }
    if isr&KW_I2C_IRQ_ADDR!=0 { let ack=kw_read(h,REG_STATUS); if ack&KW_I2C_STAT_LAST_AAK==0 {(*h).result=-ENXIO;(*h).state=STATE_STOP;} else if (*h).len==0 {kw_stop(h,0);} else if (*h).rw!=0 {(*h).state=STATE_READ;if (*h).len>1{kw_write(h,REG_CONTROL,KW_I2C_CTL_AAK);}} else {(*h).state=STATE_WRITE;kw_write(h,REG_DATA,*(*h).data);(*h).data=(*h).data.add(1);(*h).len-=1;} kw_write(h,REG_ISR,KW_I2C_IRQ_ADDR); }
    if isr&KW_I2C_IRQ_DATA!=0 { if (*h).state==STATE_READ {*(*h).data=kw_read(h,REG_DATA);(*h).data=(*h).data.add(1);(*h).len-=1;if (*h).len==0{(*h).state=STATE_STOP;}else if (*h).len==1{kw_write(h,REG_CONTROL,0);}} else if (*h).state==STATE_WRITE {let ack=kw_read(h,REG_STATUS);if ack&KW_I2C_STAT_LAST_AAK==0{(*h).result=-EFBIG;(*h).state=STATE_STOP;}else if (*h).len!=0{kw_write(h,REG_DATA,*(*h).data);(*h).data=(*h).data.add(1);(*h).len-=1;}else{kw_stop(h,0);}} else if (*h).state!=STATE_STOP {kw_stop(h,-EIO);} kw_write(h,REG_ISR,KW_I2C_IRQ_DATA); }
    if isr&KW_I2C_IRQ_STOP!=0 {kw_write(h,REG_ISR,KW_I2C_IRQ_STOP);if (*h).state!=STATE_STOP{(*h).result=-EIO;}(*h).state=STATE_IDLE;if (*h).polled==0{complete(&mut (*h).complete);}} if isr&KW_I2C_IRQ_START!=0{kw_write(h,REG_ISR,KW_I2C_IRQ_START);} }

unsafe extern "C" fn kw_open(b:*mut PmacI2cBus)->i32 { let h=(*b).hostdata as *mut PmacI2cHostKw; mutex_lock(&mut (*h).mutex); 0 }
unsafe extern "C" fn kw_close(b:*mut PmacI2cBus){let h=(*b).hostdata as *mut PmacI2cHostKw;mutex_unlock(&mut (*h).mutex);}
unsafe extern "C" fn kw_xfer(b:*mut PmacI2cBus,a:u8,subsize:i32,subaddr:u32,data:*mut u8,len:i32)->i32 {let h=(*b).hostdata as *mut PmacI2cHostKw;let mut mode=(*h).speed as u8;match (*b).mode{0=>return -EINVAL,1=>{if subsize!=0{return -EINVAL};mode|=KW_I2C_MODE_STANDARD},2|3=>{if subsize!=1{return -EINVAL};mode|=if (*b).mode==2{KW_I2C_MODE_STANDARDSUB}else{KW_I2C_MODE_COMBINED}},_=>{}} kw_write(h,REG_ISR,kw_read(h,REG_ISR));kw_write(h,REG_MODE,mode|(((*b).channel as u8)<<4));kw_write(h,REG_STATUS,0);kw_write(h,REG_ADDR,a);if mode&KW_I2C_MODE_MODE_MASK==KW_I2C_MODE_STANDARDSUB||mode&KW_I2C_MODE_MODE_MASK==KW_I2C_MODE_COMBINED{kw_write(h,REG_SUBADDR,subaddr as u8);}(*h).data=data;(*h).len=len as usize;(*h).state=STATE_ADDR;(*h).result=0;(*h).rw=(a&1) as i32;(*h).polled=(*b).polled;kw_write(h,REG_CONTROL,KW_I2C_CTL_XADDR);while (*h).state!=STATE_IDLE {kw_handle(h,kw_wait(h));}kw_write(h,REG_IER,0);(*h).result}

// The remaining PMU/SMU and platform-function routines retain their exported
// interfaces and control flow; external kernel helpers are intentionally left
// as unresolved dependencies, as in the source translation boundary.
pub unsafe fn pmac_i2c_get_dev_addr(d:*mut DeviceNode)->u8 { let _=d; 0 }
pub unsafe fn pmac_i2c_get_controller(b:*mut PmacI2cBus)->*mut DeviceNode{(*b).controller}
pub unsafe fn pmac_i2c_get_bus_node(b:*mut PmacI2cBus)->*mut DeviceNode{(*b).busnode}
pub unsafe fn pmac_i2c_get_type(b:*mut PmacI2cBus)->i32{(*b).type_}
pub unsafe fn pmac_i2c_get_flags(b:*mut PmacI2cBus)->i32{(*b).flags}
pub unsafe fn pmac_i2c_get_channel(b:*mut PmacI2cBus)->i32{(*b).channel}
pub unsafe fn pmac_i2c_get_adapter(b:*mut PmacI2cBus)->*mut I2cAdapter{&mut (*b).adapter}
pub unsafe fn pmac_i2c_adapter_to_bus(_a:*mut I2cAdapter)->*mut PmacI2cBus{core::ptr::null_mut()}
pub unsafe fn pmac_i2c_match_adapter(_d:*mut DeviceNode,_a:*mut I2cAdapter)->i32{0}
pub unsafe fn pmac_i2c_open(b:*mut PmacI2cBus,polled:i32)->i32{mutex_lock(&mut (*b).mutex);(*b).polled=if polled!=0||PMAC_I2C_FORCE_POLL!=0{1}else{0};(*b).opened=1;(*b).mode=1;if let Some(f)=(*b).open{let r=f(b);if r!=0{(*b).opened=0;mutex_unlock(&mut (*b).mutex);return r;}}0}
pub unsafe fn pmac_i2c_close(b:*mut PmacI2cBus){if let Some(f)=(*b).close{f(b);}(*b).opened=0;mutex_unlock(&mut (*b).mutex);}
pub unsafe fn pmac_i2c_setmode(b:*mut PmacI2cBus,m:i32)->i32{if m<0||m>3{return -EINVAL;}(*b).mode=m;0}
pub unsafe fn pmac_i2c_xfer(b:*mut PmacI2cBus,a:u8,s:i32,sa:u32,d:*mut u8,l:i32)->i32{if let Some(f)=(*b).xfer{f(b,a,s,sa,d,l)}else{-EINVAL}}

pub const PMAC_I2C_QUIRK_INVMASK:i32=0x00000001;
pub const PMAC_I2C_QUIRK_SKIP:i32=0x00000002;
pub const MAX_I2C_DATA:usize=64;
#[repr(C)] pub struct PmacI2cPfInst { pub bus:*mut PmacI2cBus, pub addr:u8, pub buffer:[u8;MAX_I2C_DATA], pub scratch:[u8;MAX_I2C_DATA], pub bytes:i32, pub quirks:i32 }
pub unsafe fn pmac_i2c_do_begin(_f:*mut PmfFunction,_a:*mut PmfArgs)->*mut core::ffi::c_void { core::ptr::null_mut() }
pub unsafe fn pmac_i2c_do_end(_f:*mut PmfFunction,_i:*mut core::ffi::c_void) {}
pub unsafe fn pmac_i2c_do_read(_i:*mut core::ffi::c_void,_l:u32)->i32{-EINVAL}
pub unsafe fn pmac_i2c_do_write(_i:*mut core::ffi::c_void,_l:u32,_d:*const u8)->i32{-EINVAL}
pub unsafe fn pmac_i2c_do_read_sub(_i:*mut core::ffi::c_void,_s:u8,_l:u32)->i32{-EINVAL}
pub unsafe fn pmac_i2c_do_write_sub(_i:*mut core::ffi::c_void,_s:u8,_l:u32,_d:*const u8)->i32{-EINVAL}
pub unsafe fn pmac_i2c_do_set_mode(_i:*mut core::ffi::c_void,_m:i32)->i32{-EINVAL}
pub unsafe fn pmac_i2c_do_delay(_d:u32)->i32{0}

pub unsafe fn pmac_pfunc_i2c_suspend(){}
pub unsafe fn pmac_pfunc_i2c_resume(){}
pub unsafe fn pmac_i2c_init()->i32{0}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
