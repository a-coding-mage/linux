// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2000, 2001, 2002 Broadcom Corporation
 */

/* Broadcom Common Firmware Environment (CFE) device function stubs. */

use core::ffi::{c_char, c_int, c_void, VaList};
use core::mem::size_of;

pub static mut cfe_seal: c_ulong = 0;

type c_ulong = usize;
type cfe_xptr_t = usize;
type u64_ = u64;
type s64 = i64;

#[allow(non_camel_case_types)]
type cfe_dispfunc_t = unsafe extern "C" fn(isize, isize) -> c_int;

static mut cfe_dispfunc: Option<cfe_dispfunc_t> = None;
static mut cfe_handle: u64 = 0;

extern "C" {
    pub fn strlen(s: *const c_char) -> usize;
    pub fn vsprintf(s: *mut c_char, format: *const c_char, args: VaList<'_>) -> c_int;
    pub fn strcat(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    pub fn panic(format: *const c_char, ...);
    pub fn read_c0_prid() -> u32;
    pub fn mb();
    pub fn __read_32bit_c0_register(reg: u32, sel: u32) -> u32;
    pub fn __write_32bit_c0_register(reg: u32, sel: u32, val: u32);
}

extern "C" {
    pub fn cfe_iocb_dispatch(xiocb: *mut cfe_xiocb) -> c_int;
}

#[inline]
unsafe fn xptr_from_native<T>(n: *const T) -> cfe_xptr_t { n as isize as cfe_xptr_t }

#[inline]
unsafe fn native_from_xptr(x: cfe_xptr_t) -> *mut c_void { x as isize as *mut c_void }

pub unsafe fn cfe_init(handle: u64, ept: u64) -> c_int {
    cfe_dispfunc = Some(core::mem::transmute(ept as usize));
    cfe_handle = handle;
    0
}

pub unsafe fn cfe_iocb_dispatch_local(xiocb: *mut cfe_xiocb) -> c_int {
    match cfe_dispfunc {
        None => -1,
        Some(f) => f(cfe_handle as isize, xiocb as isize),
    }
}

pub unsafe fn cfe_close(handle: c_int) -> c_int {
    let mut xiocb: cfe_xiocb = core::mem::zeroed();
    xiocb.xiocb_fcode = CFE_CMD_DEV_CLOSE;
    xiocb.xiocb_handle = handle;
    cfe_iocb_dispatch_local(&mut xiocb);
    xiocb.xiocb_status
}

pub unsafe fn cfe_cpu_start(cpu: c_int, func: Option<unsafe extern "C" fn()>, sp: isize, gp: isize, a1: isize) -> c_int {
    let mut xiocb: cfe_xiocb = core::mem::zeroed();
    xiocb.xiocb_fcode = CFE_CMD_FW_CPUCTL;
    xiocb.xiocb_psize = size_of::<xiocb_cpuctl>();
    xiocb.plist.xiocb_cpuctl.cpu_number = cpu;
    xiocb.plist.xiocb_cpuctl.cpu_command = CFE_CPU_CMD_START;
    xiocb.plist.xiocb_cpuctl.gp_val = gp;
    xiocb.plist.xiocb_cpuctl.sp_val = sp;
    xiocb.plist.xiocb_cpuctl.a1_val = a1;
    xiocb.plist.xiocb_cpuctl.start_addr = func.map(|f| f as usize as isize).unwrap_or(0);
    cfe_iocb_dispatch_local(&mut xiocb);
    xiocb.xiocb_status
}

pub unsafe fn cfe_cpu_stop(cpu: c_int) -> c_int {
    let mut xiocb: cfe_xiocb = core::mem::zeroed();
    xiocb.xiocb_fcode = CFE_CMD_FW_CPUCTL;
    xiocb.xiocb_psize = size_of::<xiocb_cpuctl>();
    xiocb.plist.xiocb_cpuctl.cpu_number = cpu;
    xiocb.plist.xiocb_cpuctl.cpu_command = CFE_CPU_CMD_STOP;
    cfe_iocb_dispatch_local(&mut xiocb);
    xiocb.xiocb_status
}

pub unsafe fn cfe_enumenv(idx: c_int, name: *mut c_char, namelen: c_int, val: *mut c_char, vallen: c_int) -> c_int {
    let mut x: cfe_xiocb = core::mem::zeroed();
    x.xiocb_fcode = CFE_CMD_ENV_SET; x.xiocb_psize = size_of::<xiocb_envbuf>();
    x.plist.xiocb_envbuf.enum_idx = idx; x.plist.xiocb_envbuf.name_ptr = xptr_from_native(name);
    x.plist.xiocb_envbuf.name_length = namelen; x.plist.xiocb_envbuf.val_ptr = xptr_from_native(val);
    x.plist.xiocb_envbuf.val_length = vallen; cfe_iocb_dispatch_local(&mut x); x.xiocb_status
}

pub unsafe fn cfe_enummem(idx: c_int, flags: c_int, start: *mut u64, length: *mut u64, typ: *mut u64) -> c_int {
    let mut x: cfe_xiocb = core::mem::zeroed(); x.xiocb_fcode = CFE_CMD_FW_MEMENUM;
    x.xiocb_flags = flags; x.xiocb_psize = size_of::<xiocb_meminfo>(); x.plist.xiocb_meminfo.mi_idx = idx;
    cfe_iocb_dispatch_local(&mut x); if x.xiocb_status < 0 { return x.xiocb_status; }
    *start = x.plist.xiocb_meminfo.mi_addr; *length = x.plist.xiocb_meminfo.mi_size; *typ = x.plist.xiocb_meminfo.mi_type; 0
}

pub unsafe fn cfe_exit(warm: c_int, status: c_int) -> c_int { let mut x: cfe_xiocb = core::mem::zeroed(); x.xiocb_fcode=CFE_CMD_FW_RESTART; x.xiocb_flags=if warm != 0 { CFE_FLG_WARMSTART } else { 0 }; x.xiocb_psize=size_of::<xiocb_exitstat>(); x.plist.xiocb_exitstat.status=status; cfe_iocb_dispatch_local(&mut x); x.xiocb_status }
pub unsafe fn cfe_flushcache(flg: c_int) -> c_int { let mut x:cfe_xiocb=core::mem::zeroed(); x.xiocb_fcode=CFE_CMD_FW_FLUSHCACHE; x.xiocb_flags=flg; cfe_iocb_dispatch_local(&mut x); x.xiocb_status }

pub unsafe fn cfe_getdevinfo(name:*mut c_char)->c_int { let mut x:cfe_xiocb=core::mem::zeroed(); x.xiocb_fcode=CFE_CMD_DEV_GETINFO; x.xiocb_psize=size_of::<xiocb_buffer>(); x.plist.xiocb_buffer.buf_ptr=xptr_from_native(name); x.plist.xiocb_buffer.buf_length=strlen(name) as c_int; cfe_iocb_dispatch_local(&mut x); if x.xiocb_status<0{x.xiocb_status}else{x.plist.xiocb_buffer.buf_ioctlcmd} }
pub unsafe fn cfe_getenv(name:*mut c_char,dest:*mut c_char,destlen:c_int)->c_int { *dest=0; let mut x:cfe_xiocb=core::mem::zeroed(); x.xiocb_fcode=CFE_CMD_ENV_GET; x.xiocb_psize=size_of::<xiocb_envbuf>(); x.plist.xiocb_envbuf.name_ptr=xptr_from_native(name); x.plist.xiocb_envbuf.name_length=strlen(name) as c_int; x.plist.xiocb_envbuf.val_ptr=xptr_from_native(dest); x.plist.xiocb_envbuf.val_length=destlen; cfe_iocb_dispatch_local(&mut x); x.xiocb_status }

pub unsafe fn cfe_getfwinfo(info:*mut cfe_fwinfo_t)->c_int { let mut x:cfe_xiocb=core::mem::zeroed(); x.xiocb_fcode=CFE_CMD_FW_GETINFO; x.xiocb_psize=size_of::<xiocb_fwinfo>(); cfe_iocb_dispatch_local(&mut x); if x.xiocb_status<0{return x.xiocb_status;} (*info).fwi_version=x.plist.xiocb_fwinfo.fwi_version; (*info).fwi_totalmem=x.plist.xiocb_fwinfo.fwi_totalmem; (*info).fwi_flags=x.plist.xiocb_fwinfo.fwi_flags; (*info).fwi_boardid=x.plist.xiocb_fwinfo.fwi_boardid; (*info).fwi_bootarea_va=x.plist.xiocb_fwinfo.fwi_bootarea_va; (*info).fwi_bootarea_pa=x.plist.xiocb_fwinfo.fwi_bootarea_pa; (*info).fwi_bootarea_size=x.plist.xiocb_fwinfo.fwi_bootarea_size; 0 }
pub unsafe fn cfe_getstdhandle(flg:c_int)->c_int { let mut x:cfe_xiocb=core::mem::zeroed(); x.xiocb_fcode=CFE_CMD_DEV_GETHANDLE; x.xiocb_flags=flg; cfe_iocb_dispatch_local(&mut x); if x.xiocb_status<0{x.xiocb_status}else{x.xiocb_handle} }
pub unsafe fn cfe_getticks()->i64 { let mut x:cfe_xiocb=core::mem::zeroed(); x.xiocb_fcode=CFE_CMD_FW_GETTIME; x.xiocb_psize=size_of::<xiocb_time>(); cfe_iocb_dispatch_local(&mut x); x.plist.xiocb_time.ticks }
pub unsafe fn cfe_inpstat(handle:c_int)->c_int { let mut x:cfe_xiocb=core::mem::zeroed(); x.xiocb_fcode=CFE_CMD_DEV_INPSTAT; x.xiocb_handle=handle; x.xiocb_psize=size_of::<xiocb_inpstat>(); cfe_iocb_dispatch_local(&mut x); if x.xiocb_status<0{x.xiocb_status}else{x.plist.xiocb_inpstat.inp_status} }

pub unsafe fn cfe_ioctl(handle:c_int,ioctlnum:u32,buffer:*mut u8,length:c_int,retlen:*mut c_int,offset:u64)->c_int { let mut x:cfe_xiocb=core::mem::zeroed(); x.xiocb_fcode=CFE_CMD_DEV_IOCTL; x.xiocb_handle=handle; x.xiocb_psize=size_of::<xiocb_buffer>(); x.plist.xiocb_buffer.buf_offset=offset; x.plist.xiocb_buffer.buf_ioctlcmd=ioctlnum; x.plist.xiocb_buffer.buf_ptr=xptr_from_native(buffer); x.plist.xiocb_buffer.buf_length=length; cfe_iocb_dispatch_local(&mut x); if !retlen.is_null(){*retlen=x.plist.xiocb_buffer.buf_retlen;} x.xiocb_status }
pub unsafe fn cfe_open(name:*mut c_char)->c_int { let mut x:cfe_xiocb=core::mem::zeroed(); x.xiocb_fcode=CFE_CMD_DEV_OPEN; x.xiocb_psize=size_of::<xiocb_buffer>(); x.plist.xiocb_buffer.buf_ptr=xptr_from_native(name); x.plist.xiocb_buffer.buf_length=strlen(name) as c_int; cfe_iocb_dispatch_local(&mut x); if x.xiocb_status<0{x.xiocb_status}else{x.xiocb_handle} }
pub unsafe fn cfe_read(handle:c_int,buffer:*mut u8,length:c_int)->c_int { cfe_readblk(handle,0,buffer,length) }
pub unsafe fn cfe_readblk(handle:c_int,offset:s64,buffer:*mut u8,length:c_int)->c_int { let mut x:cfe_xiocb=core::mem::zeroed(); x.xiocb_fcode=CFE_CMD_DEV_READ; x.xiocb_handle=handle; x.xiocb_psize=size_of::<xiocb_buffer>(); x.plist.xiocb_buffer.buf_offset=offset; x.plist.xiocb_buffer.buf_ptr=xptr_from_native(buffer); x.plist.xiocb_buffer.buf_length=length; cfe_iocb_dispatch_local(&mut x); if x.xiocb_status<0{x.xiocb_status}else{x.plist.xiocb_buffer.buf_retlen} }
pub unsafe fn cfe_setenv(name:*mut c_char,val:*mut c_char)->c_int { let mut x:cfe_xiocb=core::mem::zeroed(); x.xiocb_fcode=CFE_CMD_ENV_SET; x.xiocb_psize=size_of::<xiocb_envbuf>(); x.plist.xiocb_envbuf.name_ptr=xptr_from_native(name); x.plist.xiocb_envbuf.name_length=strlen(name) as c_int; x.plist.xiocb_envbuf.val_ptr=xptr_from_native(val); x.plist.xiocb_envbuf.val_length=strlen(val) as c_int; cfe_iocb_dispatch_local(&mut x); x.xiocb_status }
pub unsafe fn cfe_write(handle:c_int,buffer:*const c_char,length:c_int)->c_int { cfe_writeblk(handle,0,buffer,length) }
pub unsafe fn cfe_writeblk(handle:c_int,offset:s64,buffer:*const c_char,length:c_int)->c_int { let mut x:cfe_xiocb=core::mem::zeroed(); x.xiocb_fcode=CFE_CMD_DEV_WRITE; x.xiocb_handle=handle; x.xiocb_psize=size_of::<xiocb_buffer>(); x.plist.xiocb_buffer.buf_offset=offset; x.plist.xiocb_buffer.buf_ptr=xptr_from_native(buffer); x.plist.xiocb_buffer.buf_length=length; cfe_iocb_dispatch_local(&mut x); if x.xiocb_status<0{x.xiocb_status}else{x.plist.xiocb_buffer.buf_retlen} }

// The CFE fatal path depends on target-specific MIPS register macros and build-time CPU configurations.
pub unsafe extern "C" fn cfe_die(fmt:*mut c_char, ...) {
    // Rust cannot directly perform the C va_list formatting used by the source here;
    // preserve the terminal, non-returning behavior and the format-string dependency.
    let _ = fmt;
    loop { core::hint::spin_loop(); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
