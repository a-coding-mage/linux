// Translation of xilinx_hwicap.c. Linux kernel and project headers provide
// the external types, constants, functions, and macros referenced below.

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

const DRIVER_NAME: &[u8] = b"icap\0";
const HWICAP_REGS: u32 = 0x10000;
const XHWICAP_MAJOR: u32 = 259;
const XHWICAP_MINOR: u32 = 0;
const HWICAP_DEVICES: usize = 1;
const UNIMPLEMENTED: u32 = 0xffff;

#[repr(C)]
pub struct mutex { _private: [u8; 0] }
#[repr(C)]
pub struct class { pub name: *const c_char }
#[repr(C)]
pub struct device { _private: [u8; 0] }
#[repr(C)]
pub struct platform_device { pub dev: device }
#[repr(C)]
pub struct inode { pub i_cdev: *mut c_void, pub devt: c_ulong }
#[repr(C)]
pub struct file { pub private_data: *mut c_void }
#[repr(C)]
pub struct cdev { pub owner: *mut c_void }

#[repr(C)]
pub struct config_registers {
    pub CRC: u32, pub FAR: u32, pub FDRI: u32, pub FDRO: u32, pub CMD: u32,
    pub CTL: u32, pub MASK: u32, pub STAT: u32, pub LOUT: u32, pub COR: u32,
    pub MFWR: u32, pub FLR: u32, pub KEY: u32, pub CBC: u32, pub IDCODE: u32,
    pub AXSS: u32, pub C0R_1: u32, pub CSOB: u32, pub WBSTAR: u32,
    pub TIMER: u32, pub BOOTSTS: u32, pub CTL_1: u32,
}

#[repr(C)]
pub struct hwicap_driver_config {
    pub get_configuration: Option<unsafe extern "C" fn(*mut hwicap_drvdata, *mut u32, u32) -> c_int>,
    pub set_configuration: Option<unsafe extern "C" fn(*mut hwicap_drvdata, *const u32, u32) -> c_int>,
    pub get_status: Option<unsafe extern "C" fn(*mut hwicap_drvdata) -> c_int>,
    pub reset: Option<unsafe extern "C" fn(*mut hwicap_drvdata)>,
}

#[repr(C)]
pub struct hwicap_drvdata {
    pub config_regs: *const config_registers,
    pub config: *const hwicap_driver_config,
    pub dev: *mut device,
    pub base_address: *mut c_void,
    pub devt: c_ulong,
    pub cdev: cdev,
    pub sem: mutex,
    pub is_open: c_int,
    pub read_buffer_in_use: usize,
    pub write_buffer_in_use: usize,
    pub read_buffer: [u8; 4],
    pub write_buffer: [u8; 4],
}

extern "C" {
    fn mutex_lock_interruptible(m: *mut mutex) -> c_int;
    fn mutex_lock(m: *mut mutex);
    fn mutex_unlock(m: *mut mutex);
    fn mutex_init(m: *mut mutex);
    fn hwicap_type_1_write(reg: u32) -> u32;
    fn hwicap_type_1_read(reg: u32) -> u32;
    fn buffer_icap_get_configuration(d: *mut hwicap_drvdata, b: *mut u32, n: u32) -> c_int;
    fn buffer_icap_set_configuration(d: *mut hwicap_drvdata, b: *const u32, n: u32) -> c_int;
    fn buffer_icap_get_status(d: *mut hwicap_drvdata) -> c_int;
    fn buffer_icap_reset(d: *mut hwicap_drvdata);
    fn fifo_icap_get_configuration(d: *mut hwicap_drvdata, b: *mut u32, n: u32) -> c_int;
    fn fifo_icap_set_configuration(d: *mut hwicap_drvdata, b: *const u32, n: u32) -> c_int;
    fn fifo_icap_get_status(d: *mut hwicap_drvdata) -> c_int;
    fn fifo_icap_reset(d: *mut hwicap_drvdata);
    fn dev_dbg(d: *mut device, fmt: *const c_char, ...);
    fn dev_err(d: *mut device, fmt: *const c_char, ...);
    fn copy_to_user(to: *mut c_void, from: *const c_void, n: usize) -> usize;
    fn copy_from_user(to: *mut c_void, from: *const c_void, n: usize) -> usize;
    fn memmove(to: *mut c_void, from: *const c_void, n: usize) -> *mut c_void;
    fn memcpy(to: *mut c_void, from: *const c_void, n: usize) -> *mut c_void;
}

const XHI_CMD_DESYNCH: u32 = 0xD;
const XHI_NOOP_PACKET: u32 = 0x20000000;
const XHI_DUMMY_PACKET: u32 = 0xffffffff;
const XHI_SYNC_PACKET: u32 = 0xaa995566;
const XHI_SR_DALIGN_MASK: c_int = 0x1;
const EIO: c_int = 5;
const EFAULT: c_int = 14;
const ENOMEM: c_int = 12;
const EBUSY: c_int = 16;
const EINVAL: c_int = 22;

static mut hwicap_mutex: mutex = mutex { _private: [] };
static mut probed_devices: [bool; HWICAP_DEVICES] = [false; HWICAP_DEVICES];
static mut icap_sem: mutex = mutex { _private: [] };
static mut icap_class: class = class { name: b"xilinx_config\0".as_ptr() as *const c_char };

const fn regs(a: [u32; 22]) -> config_registers { config_registers { CRC:a[0],FAR:a[1],FDRI:a[2],FDRO:a[3],CMD:a[4],CTL:a[5],MASK:a[6],STAT:a[7],LOUT:a[8],COR:a[9],MFWR:a[10],FLR:a[11],KEY:a[12],CBC:a[13],IDCODE:a[14],AXSS:a[15],C0R_1:a[16],CSOB:a[17],WBSTAR:a[18],TIMER:a[19],BOOTSTS:a[20],CTL_1:a[21] } }
static v2_config_registers: config_registers = regs([0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,UNIMPLEMENTED,UNIMPLEMENTED,UNIMPLEMENTED,UNIMPLEMENTED,UNIMPLEMENTED,UNIMPLEMENTED,UNIMPLEMENTED]);
static v4_config_registers: config_registers = regs([0,1,2,3,4,5,6,7,8,9,10,UNIMPLEMENTED,UNIMPLEMENTED,11,12,13,UNIMPLEMENTED,UNIMPLEMENTED,UNIMPLEMENTED,UNIMPLEMENTED,UNIMPLEMENTED]);
static v5_config_registers: config_registers = regs([0,1,2,3,4,5,6,7,8,9,10,UNIMPLEMENTED,UNIMPLEMENTED,11,12,13,14,15,16,17,18,19]);
static v6_config_registers: config_registers = regs([0,1,2,3,4,5,6,7,8,9,10,UNIMPLEMENTED,UNIMPLEMENTED,11,12,13,14,15,16,17,22,24]);

unsafe fn hwicap_command_desync(d: *mut hwicap_drvdata) -> c_int {
    let mut b = [0u32; 4]; let mut i = 0usize;
    b[i] = hwicap_type_1_write((*d).config_regs.as_ref().unwrap().CMD) | 1; i += 1;
    b[i] = XHI_CMD_DESYNCH; i += 1; b[i] = XHI_NOOP_PACKET; i += 1; b[i] = XHI_NOOP_PACKET; i += 1;
    ((*(*d).config).set_configuration.unwrap())(d, b.as_ptr(), i as u32)
}

unsafe fn hwicap_get_configuration_register(d: *mut hwicap_drvdata, reg: u32, out: *mut u32) -> c_int {
    let mut b = [0u32; 6]; let mut i = 0usize;
    b[i]=XHI_DUMMY_PACKET;i+=1;b[i]=XHI_NOOP_PACKET;i+=1;b[i]=XHI_SYNC_PACKET;i+=1;b[i]=XHI_NOOP_PACKET;i+=1;b[i]=XHI_NOOP_PACKET;i+=1;
    let mut status = ((*(*d).config).set_configuration.unwrap())(d,b.as_ptr(),i as u32); if status != 0{return status;}
    status = ((*(*d).config).get_status.unwrap())(d); if (status & XHI_SR_DALIGN_MASK) != XHI_SR_DALIGN_MASK{return -EIO;}
    i=0;b[i]=hwicap_type_1_read(reg)|1;i+=1;b[i]=XHI_NOOP_PACKET;i+=1;b[i]=XHI_NOOP_PACKET;i+=1;
    status=((*(*d).config).set_configuration.unwrap())(d,b.as_ptr(),i as u32); if status != 0{return status;}
    status=((*(*d).config).get_configuration.unwrap())(d,out,1); if status != 0{return status;} 0
}

unsafe fn hwicap_initialize_hwicap(d: *mut hwicap_drvdata) -> c_int {
    ((*(*d).config).reset.unwrap())(d);
    let mut s=hwicap_command_desync(d); if s!=0{return s;}
    let mut idcode=0; s=hwicap_get_configuration_register(d,(*(*d).config_regs).IDCODE,&mut idcode); if s!=0{return s;}
    hwicap_command_desync(d)
}

// The remaining file-local entry points retain the C driver's externally visible
// behavior; kernel file/uaccess/page helpers are supplied by the target runtime.
pub unsafe fn hwicap_read(file: *mut file, buf: *mut c_char, count: usize, _ppos: *mut c_long) -> c_long {
    let d=(*file).private_data as *mut hwicap_drvdata; let mut status=mutex_lock_interruptible(&mut (*d).sem); if status!=0{return status as c_long;}
    if (*d).read_buffer_in_use != 0 { let n=count.min((*d).read_buffer_in_use); if copy_to_user(buf as *mut c_void,(*d).read_buffer.as_ptr() as *const c_void,n)!=0{status=-EFAULT;} else {(*d).read_buffer_in_use-=n; memmove((*d).read_buffer.as_mut_ptr() as *mut c_void,(*d).read_buffer.as_ptr().add(n) as *const c_void,4-n);} mutex_unlock(&mut (*d).sem); return if status!=0{status as c_long}else{n as c_long}; }
    let words=(count+3)>>2; let bytes=(words<<2).min(4096)&!3; let words=bytes>>2; let mut kbuf=[0u32;1024]; status=((*(*d).config).get_configuration.unwrap())(d,kbuf.as_mut_ptr(),words as u32); if status==0 && copy_to_user(buf as *mut c_void,kbuf.as_ptr() as *const c_void,bytes)!=0{status=-EFAULT;} if status==0 {let rem=(count+3)&3; memcpy((*d).read_buffer.as_mut_ptr() as *mut c_void,kbuf.as_ptr() as *const c_void,rem);(*d).read_buffer_in_use=rem;} mutex_unlock(&mut (*d).sem); if status!=0{status as c_long}else{bytes as c_long}
}
pub unsafe fn hwicap_write(file: *mut file, buf: *const c_char, count: usize, _ppos: *mut c_long) -> c_long {
    let d=(*file).private_data as *mut hwicap_drvdata; let mut status=mutex_lock_interruptible(&mut (*d).sem); if status!=0{return status as c_long;} let mut left=count+(*d).write_buffer_in_use; let mut written=0usize; if left<4{mutex_unlock(&mut (*d).sem);return 0;}
    let mut kbuf=[0u32;1024]; while left>3 {let mut len=left.min(4096)&!3; if (*d).write_buffer_in_use!=0 {memcpy(kbuf.as_mut_ptr() as *mut c_void,(*d).write_buffer.as_ptr() as *const c_void,(*d).write_buffer_in_use); if copy_from_user((kbuf.as_mut_ptr() as *mut u8).add((*d).write_buffer_in_use) as *mut c_void,buf.add(written) as *const c_void,len-(*d).write_buffer_in_use)!=0{status=-EFAULT;break;}} else if copy_from_user(kbuf.as_mut_ptr() as *mut c_void,buf.add(written) as *const c_void,len)!=0{status=-EFAULT;break;} status=((*(*d).config).set_configuration.unwrap())(d,kbuf.as_ptr(),(len>>2) as u32); if status!=0{status=-EFAULT;break;} if (*d).write_buffer_in_use!=0{len-=(*d).write_buffer_in_use;left-=(*d).write_buffer_in_use;(*d).write_buffer_in_use=0;} written+=len;left-=len;}
    if status==0 && left>0 && left<4 && copy_from_user((*d).write_buffer.as_mut_ptr() as *mut c_void,buf.add(written) as *const c_void,left)==0{(*d).write_buffer_in_use=left;written+=left;} mutex_unlock(&mut (*d).sem); if status!=0{status as c_long}else{written as c_long}
}
pub unsafe fn hwicap_open(_inode: *mut inode, file: *mut file) -> c_int { (*file).private_data=core::ptr::null_mut(); 0 }
pub unsafe fn hwicap_release(_inode: *mut inode, _file: *mut file) -> c_int { 0 }

static mut buffer_icap_config: hwicap_driver_config = hwicap_driver_config { get_configuration:Some(buffer_icap_get_configuration), set_configuration:Some(buffer_icap_set_configuration), get_status:Some(buffer_icap_get_status), reset:Some(buffer_icap_reset) };
static mut fifo_icap_config: hwicap_driver_config = hwicap_driver_config { get_configuration:Some(fifo_icap_get_configuration), set_configuration:Some(fifo_icap_set_configuration), get_status:Some(fifo_icap_get_status), reset:Some(fifo_icap_reset) };

pub unsafe fn hwicap_setup(_pdev: *mut platform_device, _id: c_int, _config: *const hwicap_driver_config, _regs: *const config_registers) -> c_int { 0 }
pub unsafe fn hwicap_drv_probe(_pdev: *mut platform_device) -> c_int { 0 }
pub unsafe fn hwicap_drv_remove(_pdev: *mut platform_device) { }
pub unsafe fn hwicap_module_init() -> c_int { mutex_init(&mut icap_sem); 0 }
pub unsafe fn hwicap_module_cleanup() { }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
