// SPDX-License-Identifier: GPL-2.0
/* s390 specific pci instructions */

const ZPCI_INSN_BUSY_DELAY: u64 = 1;

#[repr(C, packed)]
pub struct ZpciErrInsnData {
    pub insn: u8,
    pub cc: u8,
    pub status: u8,
    pub req_or_addr: u64,
    pub offset_or_len: u64,
}

extern "C" {
    fn zpci_err_hex_level(level: i32, data: *const ZpciErrInsnData, len: usize);
    fn msleep(usecs: u64);
    fn udelay(usecs: u64);
    fn test_facility(nr: i32) -> bool;
    fn static_branch_unlikely(key: *const core::ffi::c_void) -> bool;
    fn static_branch_likely(key: *const core::ffi::c_void) -> bool;
    static have_mio: core::ffi::c_void;
    static mut zpci_iomap_start: ZpciIomapEntry;
}

#[repr(C)]
pub struct ZpciFib { _private: [u8; 0] }
#[repr(C)]
pub union ZpciSicIib { pub raw: [u8; 0] }
#[repr(C)]
pub struct ZpciIomapEntry { pub fh: u64, pub bar: u32 }

#[inline]
unsafe fn zpci_err_insn_req(level: i32, insn: u8, cc: u8, status: u8, req: u64, offset: u64) {
    let data = ZpciErrInsnData { insn, cc, status, req_or_addr: req, offset_or_len: offset };
    zpci_err_hex_level(level, &data, core::mem::size_of::<ZpciErrInsnData>());
}

#[inline]
unsafe fn zpci_err_insn_addr(level: i32, insn: u8, cc: u8, status: u8, addr: u64, len: u64) {
    let data = ZpciErrInsnData { insn, cc, status, req_or_addr: addr, offset_or_len: len };
    zpci_err_hex_level(level, &data, core::mem::size_of::<ZpciErrInsnData>());
}

#[inline]
unsafe fn __mpcifc(mut req: u64, fib: *mut ZpciFib, status: *mut u8) -> u8 {
    let mut cc: i32;
    core::arch::asm!(".insn rxy,0xe300000000d0,{req},{fib}", req = inout(reg) req, fib = inout(reg) *fib, lateout("cc") cc);
    *status = ((req >> 24) & 0xff) as u8;
    cc as u8
}

#[no_mangle]
pub unsafe extern "C" fn zpci_mod_fc(req: u64, fib: *mut ZpciFib, status: *mut u8) -> u8 {
    let mut retried = false;
    let mut cc;
    loop { cc = __mpcifc(req, fib, status); if cc != 2 { break; } msleep(ZPCI_INSN_BUSY_DELAY); if !retried { zpci_err_insn_req(1, b'M', cc, *status, req, 0); retried = true; } }
    if cc != 0 { zpci_err_insn_req(0, b'M', cc, *status, req, 0); } else if retried { zpci_err_insn_req(1, b'M', cc, *status, req, 0); }
    cc
}

#[inline]
unsafe fn __rpcit(mut fn_: u64, addr: u64, range: u64, status: *mut u8) -> u8 {
    let mut cc: i32;
    core::arch::asm!(".insn rre,0xb9d30000,{fn_},{addr}", fn_ = inout(reg) fn_, addr = in(reg) addr, lateout("cc") cc);
    *status = ((fn_ >> 24) & 0xff) as u8; cc as u8
}

#[no_mangle]
pub unsafe extern "C" fn zpci_refresh_trans(fn_: u64, addr: u64, range: u64) -> i32 {
    let mut retried = false; let mut status = 0; let cc;
    loop { cc = __rpcit(fn_, addr, range, &mut status); if cc != 2 { break; } udelay(ZPCI_INSN_BUSY_DELAY); if !retried { zpci_err_insn_addr(1, b'R', cc, status, addr, range); retried = true; } }
    if cc != 0 { zpci_err_insn_addr(0, b'R', cc, status, addr, range); } else if retried { zpci_err_insn_addr(1, b'R', cc, status, addr, range); }
    if cc == 1 && (status == 4 || status == 16) { return -12; } if cc != 0 { -5 } else { 0 }
}

#[no_mangle]
pub unsafe extern "C" fn zpci_set_irq_ctrl(ctl: u16, isc: u8, iib: *mut ZpciSicIib) -> i32 {
    if !test_facility(72) { return -5; }
    core::arch::asm!(".insn rsy,0xeb00000000d1,{ctl},{isc},{iib}", ctl = in(reg) ctl, isc = in(reg) (isc as u64) << 27, iib = in(reg) *iib);
    0
}

#[inline]
unsafe fn __pcilg(data: *mut u64, req: u64, offset: u64, status: *mut u8) -> i32 {
    let mut value: u64 = 0; let mut cc: i32 = 0; let mut pair = [req, offset];
    core::arch::asm!(".insn rre,0xb9d20000,{data},{pair}", data = lateout(reg) value, pair = inout(reg) pair, lateout("cc") cc);
    *status = ((pair[0] >> 24) & 0xff) as u8; *data = value; cc
}

#[no_mangle]
pub unsafe extern "C" fn __zpci_load(data: *mut u64, req: u64, offset: u64) -> i32 {
    let mut retried = false; let mut status = 0; let cc;
    loop { cc = __pcilg(data, req, offset, &mut status); if cc != 2 { break; } udelay(1); if !retried { zpci_err_insn_req(1, b'l', cc as u8, status, req, offset); retried = true; } }
    if cc != 0 { zpci_err_insn_req(0, b'l', cc as u8, status, req, offset); } else if retried { zpci_err_insn_req(1, b'l', 0, status, req, offset); } if cc > 0 { -5 } else { cc }
}

#[no_mangle]
pub unsafe extern "C" fn zpci_load(data: *mut u64, addr: *const core::ffi::c_void, len: usize) -> i32 { let _ = len; __zpci_load(data, addr as u64, 0) }

#[inline]
unsafe fn __pcistg(data: u64, req: u64, offset: u64, status: *mut u8) -> i32 { let mut cc: i32 = 0; let mut pair=[req,offset]; core::arch::asm!(".insn rre,0xb9d00000,{data},{pair}", data=in(reg) data, pair=inout(reg) pair, lateout("cc") cc); *status=((pair[0]>>24)&255) as u8; cc }
#[no_mangle]
pub unsafe extern "C" fn __zpci_store(data:u64, req:u64, offset:u64)->i32 { let mut s=0; let mut r=false; let c; loop{c=__pcistg(data,req,offset,&mut s);if c!=2{break}udelay(1);if !r{zpci_err_insn_req(1,b's',c as u8,s,req,offset);r=true}} if c!=0{zpci_err_insn_req(0,b's',c as u8,s,req,offset)}else if r{zpci_err_insn_req(1,b's',0,s,req,offset)}if c>0{-5}else{c} }
#[no_mangle] pub unsafe extern "C" fn zpci_store(addr:*const core::ffi::c_void,data:u64,len:usize)->i32{__zpci_store(data,addr as u64,len as u64)}
#[no_mangle] pub unsafe extern "C" fn __zpci_store_block(data:*const u64,req:u64,offset:u64)->i32{let _=(data,req,offset);-6}
#[no_mangle] pub unsafe extern "C" fn zpci_write_block(dst:*mut core::ffi::c_void,src:*const core::ffi::c_void,len:usize)->i32{__zpci_store_block(src as *const u64,dst as u64,len as u64)}
#[no_mangle] pub unsafe extern "C" fn zpci_barrier(){if static_branch_likely(&have_mio){core::arch::asm!(".insn rre,0xb9d50000,0,0");}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
