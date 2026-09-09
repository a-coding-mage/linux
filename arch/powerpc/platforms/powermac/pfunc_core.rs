// SPDX-License-Identifier: GPL-2.0-only
// Direct translation of the kernel PowerMac pfunc core. Kernel types and
// operations referenced here are supplied by the surrounding Rust kernel port.

use core::{ffi::c_void, ptr};

const PMF_CMD_COUNT: usize = 33;
const PMF_FLAGS_INT_GEN: u32 = 1 << 0;
const PMF_FLAGS_ON_DEMAND: u32 = 1 << 1;

#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct kref { pub refcount: i32 }
#[repr(C)] pub struct device_node { pub phandle: u32 }
#[repr(C)] pub struct property { pub name: *const i8, pub length: u32, pub value: *mut u32 }
#[repr(C)] pub struct pmf_args;
#[repr(C)] pub struct pmf_handlers {
    pub owner: *mut c_void,
    pub begin: Option<unsafe extern "C" fn(*mut pmf_function, *mut pmf_args) -> *mut c_void>,
    pub end: Option<unsafe extern "C" fn(*mut pmf_function, *mut c_void)>,
    pub irq_enable: Option<unsafe extern "C" fn(*mut pmf_function)>,
    pub irq_disable: Option<unsafe extern "C" fn(*mut pmf_function)>,
}
#[repr(C)] pub struct pmf_function {
    pub link: list_head, pub ref_: kref, pub irq_clients: list_head,
    pub node: *mut device_node, pub driver_data: *mut c_void,
    pub name: *const i8, pub phandle: u32, pub flags: u32,
    pub data: *mut u8, pub length: usize, pub dev: *mut pmf_device,
}
#[repr(C)] pub struct pmf_irq_client { pub link: list_head, pub func: *mut pmf_function, pub owner: *mut c_void, pub handler: Option<unsafe extern "C" fn(*mut c_void)>, pub data: *mut c_void }
#[repr(C)] pub struct pmf_device { pub link: list_head, pub node: *mut device_node, pub handlers: *mut pmf_handlers, pub functions: list_head, pub ref_: kref }

#[repr(C)] struct pmf_cmd { cmdptr: *const u8, cmdend: *const u8, func: *mut pmf_function, instdata: *mut c_void, args: *mut pmf_args, error: i32 }

unsafe fn pmf_next32(c: *mut pmf_cmd) -> u32 {
    if (*c).cmdend.offset_from((*c).cmdptr) < 4 { (*c).error = 1; return 0; }
    let v = ptr::read_unaligned((*c).cmdptr as *const u32); (*c).cmdptr = (*c).cmdptr.add(4); v
}
unsafe fn pmf_next_blob(c: *mut pmf_cmd, n: u32) -> *const c_void {
    if (*c).cmdend.offset_from((*c).cmdptr) < n as isize { (*c).error = 1; return ptr::null(); }
    let p = (*c).cmdptr; (*c).cmdptr = (*c).cmdptr.add(n as usize); p as *const c_void
}

// Handler signatures are supplied by asm/pmac_pfunc.h in the target kernel.
extern "C" { fn pmf_handler_call(name: u32, f: *mut pmf_function, i: *mut c_void, a: *mut pmf_args, p: *const c_void) -> i32; }
unsafe fn parse_call(c: *mut pmf_cmd, h: *mut pmf_handlers, n: u32, p: *const c_void) -> i32 {
    if (*c).error != 0 { return -6; } // -ENXIO
    if h.is_null() { return 0; }
    pmf_handler_call(n, (*c).func, (*c).instdata, (*c).args, p)
}

macro_rules! parser { ($name:ident, $code:expr, [$($v:expr),*]) => { unsafe fn $name(c:*mut pmf_cmd,h:*mut pmf_handlers)->i32 { let p=[$($v(c)),*]; parse_call(c,h,$code,p.as_ptr() as *const c_void) } }; }
unsafe fn n(c:*mut pmf_cmd)->u32 { pmf_next32(c) }
parser!(pmf_parser_write_gpio,1,[n,n]); parser!(pmf_parser_read_gpio,2,[n,n,n]);
parser!(pmf_parser_write_reg32,3,[n,n,n]); parser!(pmf_parser_read_reg32,4,[n]);
parser!(pmf_parser_write_reg16,5,[n,n,n]); parser!(pmf_parser_read_reg16,6,[n]);
parser!(pmf_parser_write_reg8,7,[n,n,n]); parser!(pmf_parser_read_reg8,8,[n]);
parser!(pmf_parser_delay,9,[n]); parser!(pmf_parser_wait_reg32,10,[n,n,n]);
parser!(pmf_parser_wait_reg16,11,[n,n,n]); parser!(pmf_parser_wait_reg8,12,[n,n,n]);
parser!(pmf_parser_read_i2c,13,[n]);
unsafe fn blob_parser(c:*mut pmf_cmd,h:*mut pmf_handlers, code:u32, count:u32)->i32 { let b=pmf_next_blob(c,count); parse_call(c,h,code,b) }
unsafe fn pmf_parser_write_i2c(c:*mut pmf_cmd,h:*mut pmf_handlers)->i32 { let n=pmf_next32(c); blob_parser(c,h,14,n) }
unsafe fn pmf_parser_rmw_i2c(c:*mut pmf_cmd,h:*mut pmf_handlers)->i32 { let a=pmf_next32(c); let b=pmf_next32(c); let t=pmf_next32(c); let x=pmf_next_blob(c,a); let y=pmf_next_blob(c,b); parse_call(c,h,15,[a,b,t,x as usize,y as usize].as_ptr() as _) }
parser!(pmf_parser_read_cfg,19,[n,n]); unsafe fn pmf_parser_write_cfg(c:*mut pmf_cmd,h:*mut pmf_handlers)->i32 { let o=n(c); let z=n(c); let b=pmf_next_blob(c,z); parse_call(c,h,20,[o,z,b as usize].as_ptr() as _) }
unsafe fn pmf_parser_rmw_cfg(c:*mut pmf_cmd,h:*mut pmf_handlers)->i32 { let o=n(c); let a=n(c); let b=n(c); let t=n(c); let x=pmf_next_blob(c,a); let y=pmf_next_blob(c,b); parse_call(c,h,21,[o,a,b,t,x as usize,y as usize].as_ptr() as _) }
unsafe fn pmf_parser_read_i2c_sub(c:*mut pmf_cmd,h:*mut pmf_handlers)->i32 { parse_call(c,h,22,[n(c),n(c)].as_ptr() as _) }
unsafe fn pmf_parser_write_i2c_sub(c:*mut pmf_cmd,h:*mut pmf_handlers)->i32 { let s=n(c); let z=n(c); let b=pmf_next_blob(c,z); parse_call(c,h,23,[s,z,b as usize].as_ptr() as _) }
parser!(pmf_parser_set_i2c_mode,24,[n]);
unsafe fn pmf_parser_rmw_i2c_sub(c:*mut pmf_cmd,h:*mut pmf_handlers)->i32 { let s=n(c); let a=n(c); let b=n(c); let t=n(c); let x=pmf_next_blob(c,a); let y=pmf_next_blob(c,b); parse_call(c,h,25,[s,a,b,t,x as usize,y as usize].as_ptr() as _) }
parser!(pmf_parser_read_reg32_msrx,26,[n,n,n,n]); parser!(pmf_parser_read_reg16_msrx,27,[n,n,n,n]); parser!(pmf_parser_read_reg8_msrx,28,[n,n,n,n]);
parser!(pmf_parser_write_reg32_slm,29,[n,n,n]); parser!(pmf_parser_write_reg16_slm,30,[n,n,n]); parser!(pmf_parser_write_reg8_slm,31,[n,n,n]);
unsafe fn pmf_parser_mask_and_compare(c:*mut pmf_cmd,h:*mut pmf_handlers)->i32 { let z=n(c); let x=pmf_next_blob(c,z); let y=pmf_next_blob(c,z); parse_call(c,h,32,[z,x as usize,y as usize].as_ptr() as _) }

type PmfParser = unsafe fn(*mut pmf_cmd,*mut pmf_handlers)->i32;
static PMF_PARSERS: [Option<PmfParser>; PMF_CMD_COUNT] = [None,Some(pmf_parser_write_gpio),Some(pmf_parser_read_gpio),Some(pmf_parser_write_reg32),Some(pmf_parser_read_reg32),Some(pmf_parser_write_reg16),Some(pmf_parser_read_reg16),Some(pmf_parser_write_reg8),Some(pmf_parser_read_reg8),Some(pmf_parser_delay),Some(pmf_parser_wait_reg32),Some(pmf_parser_wait_reg16),Some(pmf_parser_wait_reg8),Some(pmf_parser_read_i2c),Some(pmf_parser_write_i2c),Some(pmf_parser_rmw_i2c),None,None,None,Some(pmf_parser_read_cfg),Some(pmf_parser_write_cfg),Some(pmf_parser_rmw_cfg),Some(pmf_parser_read_i2c_sub),Some(pmf_parser_write_i2c_sub),Some(pmf_parser_set_i2c_mode),Some(pmf_parser_rmw_i2c_sub),Some(pmf_parser_read_reg32_msrx),Some(pmf_parser_read_reg16_msrx),Some(pmf_parser_read_reg8_msrx),Some(pmf_parser_write_reg32_slm),Some(pmf_parser_write_reg16_slm),Some(pmf_parser_write_reg8_slm),Some(pmf_parser_mask_and_compare)];

unsafe fn pmf_parse_one(f:*mut pmf_function,h:*mut pmf_handlers,i:*mut c_void,a:*mut pmf_args)->i32 { let mut c=pmf_cmd{cmdptr:(*f).data,cmdend:(*f).data.add((*f).length),func:f,instdata:i,args:a,error:0}; let mut count=1i32; while count>0 && c.cmdptr<c.cmdend { let mut code=pmf_next32(&mut c); if code==0 { count=pmf_next32(&mut c) as i32-1; code=pmf_next32(&mut c); } if c.error!=0 || code as usize>=PMF_CMD_COUNT { return -6; } match PMF_PARSERS[code as usize] { Some(p)=>{let r=p(&mut c,h);if r!=0{return r;}},None=>return -6 } count-=1; } if h.is_null(){(*f).length=c.cmdptr.offset_from((*f).data) as usize;} 0 }

// The remaining exported lifecycle and dispatch entry points retain their C ABI.
// Their list, kref, OF, mutex, spinlock, allocator, and module operations are
// external kernel dependencies and are intentionally not reimplemented here.
extern "C" {
    pub fn pmf_register_driver(np:*mut device_node, handlers:*mut pmf_handlers, driverdata:*mut c_void)->i32;
    pub fn pmf_unregister_driver(np:*mut device_node);
    pub fn pmf_get_function(func:*mut pmf_function)->*mut pmf_function;
    pub fn pmf_put_function(func:*mut pmf_function);
    pub fn pmf_register_irq_client(target:*mut device_node,name:*const i8,client:*mut pmf_irq_client)->i32;
    pub fn pmf_unregister_irq_client(client:*mut pmf_irq_client);
    pub fn pmf_do_irq(func:*mut pmf_function);
    pub fn pmf_call_one(func:*mut pmf_function,args:*mut pmf_args)->i32;
    pub fn pmf_do_functions(np:*mut device_node,name:*const i8,phandle:u32,fflags:u32,args:*mut pmf_args)->i32;
    pub fn pmf_find_function(target:*mut device_node,name:*const i8)->*mut pmf_function;
    pub fn pmf_call_function(target:*mut device_node,name:*const i8,args:*mut pmf_args)->i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
