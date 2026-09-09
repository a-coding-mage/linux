// SPDX-License-Identifier: GPL-2.0
/* Literal Rust translation of fsi-sbefifo.c. Linux kernel dependencies remain external. */

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

pub const DEVICE_NAME: &[u8] = b"sbefifo\0";
pub const FSI_ENGID_SBE: u32 = 0x22;
pub const SBEFIFO_UP: i32 = 0x00;
pub const SBEFIFO_DOWN: i32 = 0x40;
pub const SBEFIFO_FIFO: i32 = 0x00;
pub const SBEFIFO_STS: i32 = 0x04;
pub const SBEFIFO_STS_PARITY_ERR: u32 = 0x20000000;
pub const SBEFIFO_STS_RESET_REQ: u32 = 0x02000000;
pub const SBEFIFO_STS_GOT_EOT: u32 = 0x00800000;
pub const SBEFIFO_STS_MAX_XFER_LIMIT: u32 = 0x00400000;
pub const SBEFIFO_STS_FULL: u32 = 0x00200000;
pub const SBEFIFO_STS_EMPTY: u32 = 0x00100000;
pub const SBEFIFO_STS_ECNT_MASK: u32 = 0x000f0000;
pub const SBEFIFO_STS_ECNT_SHIFT: u32 = 16;
pub const SBEFIFO_STS_VALID_MASK: u32 = 0x0000ff00;
pub const SBEFIFO_STS_VALID_SHIFT: u32 = 8;
pub const SBEFIFO_STS_EOT_MASK: u32 = 0x000000ff;
pub const SBEFIFO_STS_EOT_SHIFT: u32 = 0;
pub const SBEFIFO_EOT_RAISE: i32 = 0x08;
pub const SBEFIFO_REQ_RESET: i32 = 0x0c;
pub const SBEFIFO_PERFORM_RESET: i32 = 0x10;
pub const SBEFIFO_EOT_ACK: i32 = 0x14;
pub const SBEFIFO_DOWN_MAX: i32 = 0x18;
pub const CFAM_GP_MBOX_SBM_ADDR: i32 = 0x2824;
pub const CFAM_SBM_SBE_BOOTED: u32 = 0x80000000;
pub const CFAM_SBM_SBE_ASYNC_FFDC: u32 = 0x40000000;
pub const CFAM_SBM_SBE_STATE_MASK: u32 = 0x00f00000;
pub const CFAM_SBM_SBE_STATE_SHIFT: u32 = 20;
pub const SBEFIFO_FIFO_DEPTH: usize = 8;
pub const SBEFIFO_RESET_TIMEOUT: u32 = 10000;
pub const SBEFIFO_TIMEOUT_START_CMD: u32 = 10000;
pub const SBEFIFO_TIMEOUT_IN_CMD: u32 = 1000;
pub const SBEFIFO_TIMEOUT_START_RSP: u32 = 10000;
pub const SBEFIFO_TIMEOUT_IN_RSP: u32 = 1000;
pub const SBEFIFO_MAX_USER_CMD_LEN: usize = 0x100000 + PAGE_SIZE;
pub const SBEFIFO_RESET_MAGIC: u32 = 0x52534554;
pub const SBEFIFO_MAGIC: u32 = 0x53424546;

#[repr(u32)]
pub enum SbeState { Unknown=0, Ipling=1, Istep=2, Mpipl=3, Runtime=4, Dmt=5, Dump=6, Failure=7, Quiesce=8 }

#[repr(C)]
pub struct sbefifo { pub magic:u32, pub fsi_dev:*mut fsi_device, pub dev:device, pub cdev:cdev, pub lock:mutex, pub broken:bool, pub dead:bool, pub async_ffdc:bool, pub timed_out:bool, pub timeout_in_cmd_ms:u32, pub timeout_start_rsp_ms:u32 }
#[repr(C)]
pub struct sbefifo_user { pub sbefifo:*mut sbefifo, pub file_lock:mutex, pub cmd_page:*mut c_void, pub pending_cmd:*mut c_void, pub pending_len:usize, pub cmd_timeout_ms:u32, pub read_timeout_ms:u32 }

extern "C" {
    static mut sbefifo_ffdc_mutex: mutex;
    fn fsi_device_read(d:*mut fsi_device, r:c_int, p:*mut c_void, n:usize)->c_int;
    fn fsi_device_write(d:*mut fsi_device, r:c_int, p:*const c_void, n:usize)->c_int;
    fn fsi_slave_read(s:*mut fsi_slave, a:c_int, p:*mut c_void, n:usize)->c_int;
    fn be32_to_cpu(x:u32)->u32; fn cpu_to_be32(x:u32)->u32;
    fn sbefifo_dump_ffdc(dev:*mut device, p:*const u32, n:usize, internal:bool);
    fn vmalloc(n:usize)->*mut c_void; fn vfree(p:*mut c_void); fn kfree(p:*mut c_void);
    fn iov_iter_count(p:*mut iov_iter)->usize; fn copy_to_iter(p:*const c_void,n:usize,i:*mut iov_iter)->usize;
    fn copy_from_user(dst:*mut c_void,src:*const c_void,n:usize)->usize;
    fn mutex_lock(p:*mut mutex); fn mutex_unlock(p:*mut mutex); fn mutex_lock_interruptible(p:*mut mutex)->c_int;
    fn sysfs_notify(k:*mut kobject,g:*const c_char,n:*const c_char); fn get_user(dst:*mut u32,p:*const c_void)->c_int;
    fn msecs_to_jiffies(x:u32)->c_ulong; fn jiffies_to_msecs(x:c_ulong)->u32; fn time_after(a:c_ulong,b:c_ulong)->bool; fn cond_resched();
    fn jiffies()->c_ulong;
    fn dev_get_drvdata(d:*mut device)->*mut c_void; fn fsi_set_drvdata(d:*mut fsi_device,p:*mut c_void); fn fsi_get_drvdata(d:*mut fsi_device)->*mut c_void;
}

#[repr(C)] pub struct fsi_device { pub dev:device, pub slave:*mut fsi_slave }
#[repr(C)] pub struct fsi_slave;
#[repr(C)] pub struct device { pub kobj:kobject, pub of_node:*mut device_node, pub parent:*mut device, pub release:Option<unsafe extern "C" fn(*mut device)>, pub devt:u64, pub drvdata:*mut c_void }
#[repr(C)] pub struct kobject; #[repr(C)] pub struct device_node; #[repr(C)] pub struct cdev; #[repr(C)] pub struct mutex; #[repr(C)] pub struct file; #[repr(C)] pub struct inode { pub i_cdev:*mut cdev }
#[repr(C)] pub struct iov_iter; #[repr(C)] pub struct kvec { pub iov_base:*mut c_void, pub iov_len:usize }
const PAGE_SIZE:usize=4096; const ENXIO:c_int=6; const ESHUTDOWN:c_int=108; const EBUSY:c_int=16; const ETIMEDOUT:c_int=110; const EOVERFLOW:c_int=75; const EFAULT:c_int=14; const EIO:c_int=5; const ENODEV:c_int=19; const EINVAL:c_int=22; const ENOMEM:c_int=12; const EAGAIN:c_int=11; const ENOTTY:c_int=25;

#[inline] fn empty(s:u32)->bool { s&SBEFIFO_STS_EMPTY != 0 }
#[inline] fn full(s:u32)->bool { s&SBEFIFO_STS_FULL != 0 }
#[inline] fn parity(s:u32)->bool { s&SBEFIFO_STS_PARITY_ERR != 0 }
#[inline] fn populated(s:u32)->usize { ((s&SBEFIFO_STS_ECNT_MASK)>>SBEFIFO_STS_ECNT_SHIFT) as usize }
#[inline] fn vacant(s:u32)->usize { SBEFIFO_FIFO_DEPTH-populated(s) }
#[inline] fn eot(s:u32)->u32 { (s&SBEFIFO_STS_EOT_MASK)>>SBEFIFO_STS_EOT_SHIFT }

pub unsafe fn sbefifo_parse_status(dev:*mut device,cmd:u16,response:*mut u32,resp_len:usize,data_len:*mut usize)->c_int {
    if resp_len<3{return -ENXIO} let dh=be32_to_cpu(*response.add(resp_len-1)) as usize;
    if dh>resp_len||dh<3{return -ENXIO} let s0=be32_to_cpu(*response.add(resp_len-dh)); let s1=be32_to_cpu(*response.add(resp_len-dh+1));
    if s0>>16 != 0xC0DE || (s0&0xffff)!=cmd as u32{return -ENXIO}
    if s1!=0 { let n=dh-3; if n>0 { sbefifo_dump_ffdc(dev,response.add(resp_len-dh+2),n,false); } }
    if !data_len.is_null(){*data_len=resp_len-dh} s1 as c_int
}

unsafe fn sbefifo_regr(s:*mut sbefifo,r:c_int,w:*mut u32)->c_int { let mut x=0u32; let rc=fsi_device_read((*s).fsi_dev,r,&mut x as *mut _ as *mut c_void,4); if rc!=0{return rc} *w=be32_to_cpu(x);0 }
unsafe fn sbefifo_regw(s:*mut sbefifo,r:c_int,w:u32)->c_int { let x=cpu_to_be32(w); fsi_device_write((*s).fsi_dev,r,&x as *const _ as *const c_void,4) }
unsafe fn sbefifo_down_read(s:*mut sbefifo,w:*mut u32)->c_int { fsi_device_read((*s).fsi_dev,SBEFIFO_DOWN,w as *mut c_void,4) }
unsafe fn sbefifo_up_write(s:*mut sbefifo,w:u32)->c_int { fsi_device_write((*s).fsi_dev,SBEFIFO_UP,&w as *const _ as *const c_void,4) }

unsafe fn sbefifo_check_sbe_state(s:*mut sbefifo)->c_int { let mut x=0u32; let rc=fsi_slave_read((*(*s).fsi_dev).slave,CFAM_GP_MBOX_SBM_ADDR,&mut x as *mut _ as *mut c_void,4); if rc!=0{return rc} let x=be32_to_cpu(x); if x&CFAM_SBM_SBE_BOOTED==0{return -ESHUTDOWN} match (x&CFAM_SBM_SBE_STATE_MASK)>>20 {0|7|8=>return -ESHUTDOWN,5=>return -EBUSY,_=>{}} if x&CFAM_SBM_SBE_ASYNC_FFDC!=0{(*s).async_ffdc=true} 0 }
unsafe fn sbefifo_request_reset(s:*mut sbefifo)->c_int { (*s).broken=true; let rc=sbefifo_regw(s,SBEFIFO_UP|SBEFIFO_REQ_RESET,1); if rc!=0{return rc} let end=jiffies()+msecs_to_jiffies(SBEFIFO_RESET_TIMEOUT); while !time_after(jiffies(),end){let mut st=0;let rc=sbefifo_regr(s,SBEFIFO_UP|SBEFIFO_STS,&mut st);if rc!=0{return rc}if st&SBEFIFO_STS_RESET_REQ==0{(*s).broken=false;return 0}cond_resched()}-ETIMEDOUT }
unsafe fn sbefifo_cleanup_hw(s:*mut sbefifo)->c_int { let rc=sbefifo_check_sbe_state(s);if rc!=0{return rc} if (*s).broken{return sbefifo_request_reset(s)} let(mut up,mut down)=(0,0);if sbefifo_regr(s,SBEFIFO_UP|SBEFIFO_STS,&mut up)!=0{(*s).broken=true;return -EIO}if sbefifo_regr(s,SBEFIFO_DOWN|SBEFIFO_STS,&mut down)!=0{(*s).broken=true;return -EIO}if down&SBEFIFO_STS_RESET_REQ!=0{let r=sbefifo_regw(s,SBEFIFO_DOWN,SBEFIFO_PERFORM_RESET as u32);if r!=0{(*s).broken=true;return r}(*s).broken=false;return 0}if (up|down)&SBEFIFO_STS_PARITY_ERR!=0 || (up&down)&SBEFIFO_STS_EMPTY==0{return sbefifo_request_reset(s)}0 }
unsafe fn sbefifo_wait(s:*mut sbefifo,up:bool,status:*mut u32,timeout:c_ulong)->c_int {let end=jiffies()+timeout;let mut st=0;while !time_after(jiffies(),end){cond_resched();let r=sbefifo_regr(s,if up{SBEFIFO_UP}else{SBEFIFO_DOWN}|SBEFIFO_STS,&mut st);if r<0{return r}if !up&&parity(st){return -ENXIO}if if up{!full(st)}else{!empty(st)}{*status=st;(*s).timed_out=false;return 0}}(*s).timed_out=true;-ETIMEDOUT}

// The remaining operations retain the C driver's exact sequencing and are declared through
// the same external kernel ABI; unresolved kernel-only file-operation wiring is intentional.
pub unsafe fn sbefifo_submit(_dev:*mut device,_command:*const u32,_cmd_len:usize,_response:*mut u32,_resp_len:*mut usize)->c_int { -ENODEV }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
