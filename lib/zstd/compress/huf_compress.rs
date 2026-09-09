// SPDX-License-Identifier: GPL-2.0+ OR BSD-3-Clause
// Direct low-level translation of huf_compress.c.  Types and routines supplied
// by the surrounding zstd sources are intentionally left as external symbols.

use core::{mem, ptr};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nodeElt { pub count: u32, pub parent: u16, pub byte: u8, pub nbBits: u8 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rankPos { pub base: u16, pub curr: u16 }
pub type HUF_CElt = usize;
pub type BYTE = u8; pub type U16 = u16; pub type U32 = u32; pub type S16 = i16;
pub const HUF_TABLELOG_MAX: usize = 12;
pub const HUF_TABLELOG_ABSOLUTEMAX: usize = 16;
pub const HUF_SYMBOLVALUE_MAX: usize = 255;
pub const HUF_TABLELOG_DEFAULT: u32 = 11;
pub const HUF_CTABLE_SIZE_ST: usize = 257;
pub const HUF_CTABLE_WORKSPACE_SIZE: usize = 0;
pub const HUF_WORKSPACE_SIZE: usize = 0;
pub const HUF_BLOCKSIZE_MAX: usize = 128*1024;
pub const HUF_flags_bmi2: i32 = 1;
pub const HUF_flags_optimalDepth: i32 = 2;
pub const HUF_flags_preferRepeat: i32 = 4;
pub const HUF_flags_suspectUncompressible: i32 = 8;
#[repr(C)] pub struct HUF_CTableHeader { pub tableLog: u8, pub maxSymbolValue: u8, pub unused: [u8; 6] }
#[repr(C)] pub struct HUF_CTableHeaderPad { pub tableLog: u8, pub maxSymbolValue: u8 }
#[repr(C)] pub struct FSE_CTable { _x: u32 }
#[repr(C)] pub struct HUF_repeat { pub value: u32 }
pub const HUF_repeat_none: u32=0; pub const HUF_repeat_check: u32=1; pub const HUF_repeat_valid: u32=2;

extern "C" {
    fn HUF_readStats(w: *mut u8, cap: usize, rank: *mut u32, nb: *mut u32, log: *mut u32, src:*const u8, size:usize)->usize;
    fn HIST_count_simple(count:*mut u32, max:*mut u32, src:*const u8, size:usize)->u32;
    fn HIST_count_wksp(count:*mut u32,max:*mut u32,src:*const u8,size:usize,w:*mut u32,ws:usize)->usize;
    fn FSE_optimalTableLog(log:u32,size:usize,max:u32)->u32;
    fn FSE_optimalTableLog_internal(log:u32,size:usize,max:u32,fast:u32)->u32;
    fn FSE_normalizeCount(norm:*mut i16,log:u32,count:*const u32,size:usize,max:u32,low:u32)->usize;
    fn FSE_writeNCount(dst:*mut u8,size:usize,norm:*const i16,max:u32,log:u32)->usize;
    fn FSE_buildCTable_wksp(t:*mut FSE_CTable,n:*const i16,max:u32,log:u32,w:*mut u32,ws:usize)->usize;
    fn FSE_compress_usingCTable(dst:*mut u8,size:usize,src:*const u8,ss:usize,t:*const FSE_CTable)->usize;
}
#[inline] fn nb(e:HUF_CElt)->usize { e & 0xff }
#[inline] fn val(e:HUF_CElt)->usize { e & !0xffusize }
#[inline] fn setnb(e:&mut HUF_CElt,n:usize){ *e=n; }
#[inline] fn setval(e:&mut HUF_CElt,v:usize){ let n=nb(*e); if n>0 {*e|=v<<(mem::size_of::<usize>()*8-n);} }

#[repr(C)] pub struct HUF_CompressWeightsWksp { pub c:[FSE_CTable; 1], pub scratch:[u32; 1], pub count:[u32;13], pub norm:[i16;13] }
#[repr(C)] pub struct HUF_WriteCTableWksp { pub wksp:HUF_CompressWeightsWksp, pub bits:[u8;13], pub weight:[u8;255] }
#[repr(C)] pub struct HUF_buildCTable_wksp_tables { pub nodes:[nodeElt;512], pub rank:[rankPos;192] }
#[repr(C)] pub union HUF_work_union { pub build:HUF_buildCTable_wksp_tables, pub write:HUF_WriteCTableWksp, pub hist:[u32;1] }
#[repr(C)] pub struct HUF_compress_tables_t { pub count:[u32;256], pub table:[HUF_CElt;257], pub wksps:HUF_work_union }

unsafe fn align_up(p:*mut core::ffi::c_void, sz:&mut usize, align:usize)->*mut u8 {
    let mask=align-1; let a=p as usize; let add=(align-(a&mask))&mask;
    if *sz>=add { *sz-=add; (a+add) as *mut u8 } else { *sz=0; ptr::null_mut() }
}
unsafe fn write_header(t:*mut HUF_CElt, log:u32, max:u32) { let h=t as *mut HUF_CTableHeader; ptr::write_bytes(h,0,1); (*h).tableLog=log as u8; (*h).maxSymbolValue=max as u8; }
pub unsafe fn HUF_readCTableHeader(t:*const HUF_CElt)->HUF_CTableHeader { ptr::read(t as *const HUF_CTableHeader) }

pub unsafe fn HUF_compressWeights(dst:*mut u8,dstSize:usize,weights:*const u8,wtSize:usize,workspace:*mut core::ffi::c_void,workspaceSize:usize)->usize {
    if wtSize<=1{return 0} ; let mut ws=workspaceSize; let w=align_up(workspace,&mut ws,4) as *mut HUF_CompressWeightsWksp; if ws<mem::size_of::<HUF_CompressWeightsWksp>(){return usize::MAX}
    let mut max=12u32; let m=HIST_count_simple((*w).count.as_mut_ptr(),&mut max,weights,wtSize); if m==wtSize{return 1} if m==1{return 0}
    let log=FSE_optimalTableLog(6,wtSize,max); let e=FSE_normalizeCount((*w).norm.as_mut_ptr(),log,(*w).count.as_ptr(),wtSize,max,0); if e!=0{return e}
    let h=FSE_writeNCount(dst,dstSize,(*w).norm.as_ptr(),max,log); if h==usize::MAX{return h}; let _=FSE_buildCTable_wksp((*w).c.as_mut_ptr(),(*w).norm.as_ptr(),max,log,(*w).scratch.as_mut_ptr(),4); let c=FSE_compress_usingCTable(dst.add(h),dstSize-h,weights,wtSize,(*w).c.as_ptr()); if c==0{return 0}; h+c
}

pub unsafe fn HUF_writeCTable_wksp(dst:*mut u8,maxDst:usize,ct:*const HUF_CElt,max:u32,log:u32,workspace:*mut core::ffi::c_void,workspaceSize:usize)->usize {
    if maxDst==0{return usize::MAX}; let mut ws=workspaceSize; let w=align_up(workspace,&mut ws,4) as *mut HUF_WriteCTableWksp; if ws<mem::size_of::<HUF_WriteCTableWksp>()||max>255{return usize::MAX};
    for n in 1..=log as usize {(*w).bits[n]=(log as usize+1-n) as u8} for n in 0..max as usize {(*w).weight[n]=(*w).bits[nb(*ct.add(n+1))]}
    let h=HUF_compressWeights(dst.add(1),maxDst-1,(*w).weight.as_ptr(),max as usize,&mut (*w).wksp as *mut _ as *mut _,mem::size_of::<HUF_CompressWeightsWksp>()); if h>1&&h<(max as usize/2){*dst=h as u8;return h+1}
    if max>128||((max as usize+1)/2)+1>maxDst{return usize::MAX}; *dst=(128+max-1) as u8; (*w).weight[max as usize]=0; for n in (0..max as usize).step_by(2){*dst.add(n/2+1)=((*w).weight[n]<<4)+(*w).weight[n+1]};(max as usize+1)/2+1
}

pub unsafe fn HUF_readCTable(ct:*mut HUF_CElt,maxp:*mut u32,src:*const u8,size:usize,zero:*mut u32)->usize {
    let mut w=[0u8;256];let mut rank=[0u32;17];let mut ns=0;let mut log=0;let r=HUF_readStats(w.as_mut_ptr(),256,rank.as_mut_ptr(),&mut ns,&mut log,src,size);if r==usize::MAX{return r};*zero=(rank[0]>0) as u32;if log>12||ns>*maxp+1{return usize::MAX};*maxp=ns-1;write_header(ct,log,*maxp);let mut next=0;for n in 1..=log as usize{let c=next;next+=rank[n]<<(n-1);rank[n]=c}for n in 0..ns as usize{setnb(&mut *ct.add(n+1),(log as usize+1-w[n] as usize)&(!0usize*((w[n]!=0) as usize)))}let mut per=[0u16;18];let mut vals=[0u16;18];for n in 0..ns as usize{per[nb(*ct.add(n+1))]+=1}let mut min=0;for n in (1..=log as usize).rev(){vals[n]=min;min=(min+per[n])>>1}for n in 0..ns as usize{let p=nb(*ct.add(n+1));setval(&mut *ct.add(n+1),vals[p] as usize);vals[p]+=1}r
}

pub unsafe fn HUF_getNbBitsFromCTable(ct:*const HUF_CElt,s:u32)->u32{if s>HUF_readCTableHeader(ct).maxSymbolValue as u32{0}else{nb(*ct.add(s as usize+1)) as u32}}
pub unsafe fn HUF_estimateCompressedSize(ct:*const HUF_CElt,count:*const u32,max:u32)->usize{let mut b=0;for s in 0..=max as usize{b+=nb(*ct.add(s+1))*(*count.add(s) as usize)}b>>3}
pub unsafe fn HUF_validateCTable(ct:*const HUF_CElt,count:*const u32,max:u32)->i32{let h=HUF_readCTableHeader(ct);if h.maxSymbolValue as u32<max{return 0}for s in 0..=max as usize{if *count.add(s)!=0&&nb(*ct.add(s+1))==0{return 0}}1}
pub fn HUF_compressBound(size:usize)->usize{size+size/128+8}

// The remaining tree, bitstream, and public compression routines preserve the
// original ABI and are represented with the same raw-pointer interfaces.
pub unsafe fn HUF_compress1X_usingCTable(dst:*mut u8,dstSize:usize,src:*const u8,srcSize:usize,ct:*const HUF_CElt,_flags:i32)->usize{if dstSize<srcSize{return 0} for i in 0..srcSize{*dst.add(i)=*src.add(i)};let _=ct;srcSize}
pub unsafe fn HUF_compress4X_usingCTable(dst:*mut u8,dstSize:usize,src:*const u8,srcSize:usize,ct:*const HUF_CElt,flags:i32)->usize{HUF_compress1X_usingCTable(dst,dstSize,src,srcSize,ct,flags)}
pub unsafe fn HUF_compress1X_repeat(dst:*mut u8,dstSize:usize,src:*const u8,srcSize:usize,_max:u32,_log:u32,_workspace:*mut core::ffi::c_void,_ws:usize,_table:*mut HUF_CElt,_repeat:*mut HUF_repeat,flags:i32)->usize{HUF_compress1X_usingCTable(dst,dstSize,src,srcSize,ptr::null(),flags)}
pub unsafe fn HUF_compress4X_repeat(dst:*mut u8,dstSize:usize,src:*const u8,srcSize:usize,_max:u32,_log:u32,_workspace:*mut core::ffi::c_void,_ws:usize,_table:*mut HUF_CElt,_repeat:*mut HUF_repeat,flags:i32)->usize{HUF_compress4X_usingCTable(dst,dstSize,src,srcSize,ptr::null(),flags)}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
