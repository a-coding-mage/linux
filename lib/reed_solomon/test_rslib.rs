// SPDX-License-Identifier: GPL-2.0
/*
 * Tests for Generic Reed Solomon encoder / decoder library
 *
 * Written by Ferdinand Blomqvist
 * Based on previous work by Phil Karn, KA9Q
 */

use core::ffi::c_void;

#[repr(C)]
pub struct rs_codec {
    pub nroots: i32,
    pub nn: i32,
    pub alpha_to: *mut u16,
    pub index_of: *mut u16,
    pub prim: i32,
    pub fcr: i32,
}

#[repr(C)]
pub struct rs_control {
    pub codec: *mut rs_codec,
}

extern "C" {
    fn init_rs(symsize: i32, gfpoly: i32, fcr: i32, prim: i32, nroots: i32) -> *mut rs_control;
    fn free_rs(rs: *mut rs_control);
    fn encode_rs16(rs: *mut rs_control, data: *mut u16, len: i32, parity: *mut u16, pad: i32);
    fn decode_rs16(rs: *mut rs_control, data: *mut u16, par: *mut u16, len: i32,
                   synd: *mut u16, no_eras: i32, eras_pos: *mut i32, pad: i32,
                   corr: *mut u16) -> i32;
    fn rs_modnn(rs: *mut rs_codec, x: i32) -> i32;
    fn get_random_u32() -> u32;
    fn get_random_u32_below(n: u32) -> u32;
    fn pr_info(fmt: *const i8, ...);
    fn pr_warn(fmt: *const i8, ...);
}

extern "C" {
    fn kmalloc_array(n: usize, size: usize, flags: u32) -> *mut c_void;
    fn kzalloc(size: usize, flags: u32) -> *mut c_void;
    fn kfree(p: *mut c_void);
    fn memset(s: *mut c_void, c: i32, n: usize) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn memcmp(a: *const c_void, b: *const c_void, n: usize) -> i32;
}

const GFP_KERNEL: u32 = 0;
const ENOMEM: i32 = 12;
const EAGAIN: i32 = 11;

#[repr(i32)]
enum verbosity { V_SILENT, V_PROGRESS, V_CSUMMARY }
#[repr(i32)]
enum method { CORR_BUFFER, CALLER_SYNDROME, IN_PLACE }

static mut V: i32 = V_PROGRESS as i32;
static mut EWSC: i32 = 1;
static mut BC: i32 = 1;

#[repr(C)]
struct etab { symsize: i32, genpoly: i32, fcs: i32, prim: i32, nroots: i32, ntrials: i32 }

static mut TAB: [etab; 12] = [
    etab { symsize: 2, genpoly: 0x7, fcs: 1, prim: 1, nroots: 1, ntrials: 100000 },
    etab { symsize: 3, genpoly: 0xb, fcs: 1, prim: 1, nroots: 2, ntrials: 100000 },
    etab { symsize: 3, genpoly: 0xb, fcs: 1, prim: 1, nroots: 3, ntrials: 100000 },
    etab { symsize: 3, genpoly: 0xb, fcs: 2, prim: 1, nroots: 4, ntrials: 100000 },
    etab { symsize: 4, genpoly: 0x13, fcs: 1, prim: 1, nroots: 4, ntrials: 10000 },
    etab { symsize: 5, genpoly: 0x25, fcs: 1, prim: 1, nroots: 6, ntrials: 1000 },
    etab { symsize: 6, genpoly: 0x43, fcs: 3, prim: 1, nroots: 8, ntrials: 1000 },
    etab { symsize: 7, genpoly: 0x89, fcs: 1, prim: 1, nroots: 14, ntrials: 500 },
    etab { symsize: 8, genpoly: 0x11d, fcs: 1, prim: 1, nroots: 30, ntrials: 100 },
    etab { symsize: 8, genpoly: 0x187, fcs: 112, prim: 11, nroots: 32, ntrials: 100 },
    etab { symsize: 9, genpoly: 0x211, fcs: 1, prim: 1, nroots: 33, ntrials: 80 },
    etab { symsize: 0, genpoly: 0, fcs: 0, prim: 0, nroots: 0, ntrials: 0 },
];

#[repr(C)] struct estat { dwrong: i32, irv: i32, wepos: i32, nwords: i32 }
#[repr(C)] struct bcstat { rfail: i32, rsuccess: i32, noncw: i32, nwords: i32 }
#[repr(C)] struct wspace { c: *mut u16, r: *mut u16, s: *mut u16, corr: *mut u16, errlocs: *mut i32, derrlocs: *mut i32 }
#[repr(C)] struct pad { mult: i32, shift: i32 }
static PAD_COEF: [pad; 5] = [pad {mult:0,shift:0}, pad {mult:1,shift:2}, pad {mult:1,shift:1}, pad {mult:3,shift:2}, pad {mult:1,shift:0}];

unsafe fn free_ws(ws: *mut wspace) {
    if ws.is_null() { return; }
    kfree((*ws).errlocs as *mut c_void); kfree((*ws).c as *mut c_void); kfree(ws as *mut c_void);
}

unsafe fn alloc_ws(rs: *mut rs_codec) -> *mut wspace {
    let nroots = (*rs).nroots as usize; let nn = (*rs).nn as usize;
    let ws = kzalloc(core::mem::size_of::<wspace>(), GFP_KERNEL) as *mut wspace;
    if ws.is_null() { return core::ptr::null_mut(); }
    (*ws).c = kmalloc_array(2 * (nn + nroots), core::mem::size_of::<u16>(), GFP_KERNEL) as *mut u16;
    if (*ws).c.is_null() { free_ws(ws); return core::ptr::null_mut(); }
    (*ws).r = (*ws).c.add(nn); (*ws).s = (*ws).r.add(nn); (*ws).corr = (*ws).s.add(nroots);
    (*ws).errlocs = kmalloc_array(nn + nroots, core::mem::size_of::<i32>(), GFP_KERNEL) as *mut i32;
    if (*ws).errlocs.is_null() { free_ws(ws); return core::ptr::null_mut(); }
    (*ws).derrlocs = (*ws).errlocs.add(nn); ws
}

unsafe fn get_rcw_we(rs: *mut rs_control, ws: *mut wspace, len: i32, mut errs: i32, eras: i32) -> i32 {
    let codec = (*rs).codec; let nroots = (*codec).nroots; let dlen = len - nroots; let nn = (*codec).nn;
    let c = (*ws).c; let r = (*ws).r; let errlocs = (*ws).errlocs; let derrlocs = (*ws).derrlocs;
    for i in 0..dlen { *c.add(i as usize) = (get_random_u32() & nn as u32) as u16; }
    memset(c.add(dlen as usize) as *mut c_void, 0, (nroots as usize) * 2); encode_rs16(rs, c, dlen, c.add(dlen as usize), 0);
    memcpy(r as *mut c_void, c as *const c_void, len as usize * 2); memset(errlocs as *mut c_void, 0, len as usize * 4); memset(derrlocs as *mut c_void, 0, nroots as usize * 4);
    for i in 0..errs { let errval = loop { let x = get_random_u32() & nn as u32; if x != 0 { break x as u16; } }; let loc = loop { let x = get_random_u32_below(len as u32) as usize; if *errlocs.add(x) == 0 { break x; } }; *errlocs.add(loc)=1; *r.add(loc) ^= errval; let _ = i; }
    for i in 0..eras { let loc = loop { let x = get_random_u32_below(len as u32) as usize; if *errlocs.add(x) == 0 { break x; } }; *derrlocs.add(i as usize)=loc as i32; if EWSC != 0 && get_random_u32_below(2) != 0 { *errlocs.add(loc)=2; } else { let errval = loop { let x=get_random_u32() & nn as u32; if x != 0 {break x as u16;} }; *errlocs.add(loc)=1; *r.add(loc)^=errval; errs+=1; } }
    errs
}

unsafe fn fix_err(data: *mut u16, nerrs: i32, corr: *mut u16, errlocs: *mut i32) { for i in 0..nerrs { let loc=*errlocs.add(i as usize) as usize; *data.add(loc)^=*corr.add(i as usize); } }

unsafe fn compute_syndrome(rsc: *mut rs_control, data: *mut u16, len: i32, syn: *mut u16) {
    let rs=(*rsc).codec; for i in 0..(*rs).nroots { *syn.add(i as usize)=*data; for j in 1..len { if *syn.add(i as usize)==0 { *syn.add(i as usize)=*data.add(j as usize); } else { let x=rs_modnn(rs, (*rs).index_of.add(*syn.add(i as usize) as usize).read() as i32 + ((*rs).fcr+i)*(*rs).prim); *syn.add(i as usize)=*data.add(j as usize)^(*rs).alpha_to.add(x as usize).read(); } } } for i in 0..(*rs).nroots { *syn.add(i as usize)=(*rs).index_of.add(*syn.add(i as usize) as usize).read(); }
}

unsafe fn test_uc(rs:*mut rs_control,len:i32,errs:i32,eras:i32,trials:i32,stat:*mut estat,ws:*mut wspace,method:i32) { let dlen=len-(*(*rs).codec).nroots; for _ in 0..trials { let nerrs=get_rcw_we(rs,ws,len,errs,eras); let derrs=match method { 0=>{let x=decode_rs16(rs,(*ws).r,(*ws).r.add(dlen as usize),dlen,core::ptr::null_mut(),eras,(*ws).derrlocs,0,(*ws).corr);fix_err((*ws).r,x,(*ws).corr,(*ws).derrlocs);x},1=>{compute_syndrome(rs,(*ws).r,len,(*ws).s);let x=decode_rs16(rs,core::ptr::null_mut(),core::ptr::null_mut(),dlen,(*ws).s,eras,(*ws).derrlocs,0,(*ws).corr);fix_err((*ws).r,x,(*ws).corr,(*ws).derrlocs);x},2=>decode_rs16(rs,(*ws).r,(*ws).r.add(dlen as usize),dlen,core::ptr::null_mut(),eras,(*ws).derrlocs,0,core::ptr::null_mut()), _=>continue}; if derrs!=nerrs {(*stat).irv+=1;} if method!=2 {for i in 0..derrs {if *(*ws).errlocs.add(*(*ws).derrlocs.add(i as usize) as usize)!=1 {(*stat).wepos+=1;}}} if memcmp((*ws).r as *const c_void,(*ws).c as *const c_void,len as usize*2)!=0 {(*stat).dwrong+=1;} } (*stat).nwords+=trials; }

unsafe fn ex_rs_helper(rs:*mut rs_control,ws:*mut wspace,len:i32,trials:i32,method:i32)->i32 { let mut stat=estat{dwrong:0,irv:0,wepos:0,nwords:0}; let nroots=(*(*rs).codec).nroots; for errs in 0..=nroots/2 { for eras in 0..=nroots-2*errs { test_uc(rs,len,errs,eras,trials,&mut stat,ws,method); } } stat.dwrong+stat.wepos+stat.irv }
unsafe fn exercise_rs(rs:*mut rs_control,ws:*mut wspace,len:i32,trials:i32)->i32 { let mut retval=0; for method in 0..=2 { retval|=ex_rs_helper(rs,ws,len,trials,method); } retval }
unsafe fn test_bc(rs:*mut rs_control,len:i32,errs:i32,eras:i32,trials:i32,stat:*mut bcstat,ws:*mut wspace) { let dlen=len-(*(*rs).codec).nroots; for _ in 0..trials { get_rcw_we(rs,ws,len,errs,eras); let derrs=decode_rs16(rs,(*ws).r,(*ws).r.add(dlen as usize),dlen,core::ptr::null_mut(),eras,(*ws).derrlocs,0,(*ws).corr); fix_err((*ws).r,derrs,(*ws).corr,(*ws).derrlocs); if derrs>=0 {(*stat).rsuccess+=1; memset((*ws).corr as *mut c_void,0,(*(*rs).codec).nroots as usize*2); encode_rs16(rs,(*ws).r,dlen,(*ws).corr,0); if memcmp((*ws).r.add(dlen as usize) as *const c_void,(*ws).corr as *const c_void,(*(*rs).codec).nroots as usize*2)!=0 {(*stat).noncw+=1;} } else {(*stat).rfail+=1;} } (*stat).nwords+=trials; }
unsafe fn exercise_rs_bc(rs:*mut rs_control,ws:*mut wspace,len:i32,trials:i32)->i32 { let mut stat=bcstat{rfail:0,rsuccess:0,noncw:0,nwords:0}; let nroots=(*(*rs).codec).nroots; for errs in 1..=nroots { let mut eras=nroots-2*errs+1; if eras<0 {eras=0;} let cutoff=if nroots<=len-errs {nroots} else {len-errs}; while eras<=cutoff {test_bc(rs,len,errs,eras,trials,&mut stat,ws);eras+=1;} } stat.noncw }
unsafe fn run_exercise(e:*mut etab)->i32 { let nn=(1i32<<(*e).symsize)-1; let kk=nn-(*e).nroots; let rsc=init_rs((*e).symsize,(*e).genpoly,(*e).fcs,(*e).prim,(*e).nroots); if rsc.is_null(){return -ENOMEM;} let ws=alloc_ws((*rsc).codec); if ws.is_null(){free_rs(rsc);return -ENOMEM;} let max_pad=kk-1; let mut prev_pad=-1; let mut retval=0; for p in PAD_COEF { let pad=(p.mult*max_pad)>>p.shift; if pad==prev_pad {continue;} prev_pad=pad; let len=nn-pad; retval|=exercise_rs(rsc,ws,len,(*e).ntrials); if BC!=0 {retval|=exercise_rs_bc(rsc,ws,len,(*e).ntrials);} } free_ws(ws); free_rs(rsc); retval }
#[allow(dead_code)]
unsafe fn test_rslib_init() -> i32 { let mut fail=0; let mut i=0; while TAB[i].symsize!=0 { let retval=run_exercise(&mut TAB[i]); if retval<0{return -ENOMEM;} fail|=retval;i+=1;} fail; -EAGAIN }
#[allow(dead_code)]
unsafe fn test_rslib_exit() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
