// SPDX-License-Identifier: GPL-2.0-only
// Direct low-level Rust translation of ecard.c; kernel dependencies are external.

#[repr(C)]
pub struct ecard_request {
    pub fn_: Option<unsafe extern "C" fn(*mut ecard_request)>,
    pub ec: *mut ecard_t, pub address: u32, pub length: u32, pub use_loader: u32,
    pub buffer: *mut core::ffi::c_void, pub complete: *mut completion,
}
#[repr(C)]
pub struct expcard_quirklist {
    pub manufacturer:u16, pub product:u16, pub type_:*const i8,
    pub init:Option<unsafe extern "C" fn(*mut ecard_t)>,
}
extern "C" {
    fn ecard_loader_reset(base:usize, loader:*mut core::ffi::c_void)->i32;
    fn ecard_loader_read(off:i32, base:usize, loader:*mut core::ffi::c_void)->i32;
    fn atomwide_3p_quirk(*mut ecard_t);
}
static mut cards:*mut ecard_t=core::ptr::null_mut();
static mut slot_to_expcard:[*mut ecard_t;MAX_ECARDS]=[core::ptr::null_mut();MAX_ECARDS];
static mut ectcr:u32=0;
static mut quirklist:[expcard_quirklist;2]=[
 expcard_quirklist{manufacturer:MANU_ACORN,product:PROD_ACORN_ETHER1,type_:b"Acorn Ether1\\0".as_ptr() as _,init:None},
 expcard_quirklist{manufacturer:MANU_ATOMWIDE,product:PROD_ATOMWIDE_3PSERIAL,type_:core::ptr::null(),init:Some(atomwide_3p_quirk)}];

#[inline] unsafe fn ecard_getu16(v:*const u8)->u16{*v as u16|((*v.add(1) as u16)<<8)}
#[inline] unsafe fn ecard_gets24(v:*const u8)->i32{(*v as i32)|((*v.add(1) as i32)<<8)|((*v.add(2) as i32)<<16)|if *v.add(2)&0x80!=0{-0x1000000}else{0}}
#[inline] unsafe fn slot_to_ecard(slot:u32)->*mut ecard_t{if slot<MAX_ECARDS as u32{slot_to_expcard[slot as usize]}else{core::ptr::null_mut()}}

unsafe fn ecard_task_reset(req:*mut ecard_request){
 let ec=(*req).ec;
 let r=if (*ec).slot_no==8{&(*ec).resource[ECARD_RES_MEMC]}else if (*ec).easi{&(*ec).resource[ECARD_RES_EASI]}else{&(*ec).resource[ECARD_RES_IOCSYNC]};
 ecard_loader_reset(r.start,(*ec).loader);
}
unsafe fn ecard_call(req:*mut ecard_request){
 let mut c=DECLARE_COMPLETION_ONSTACK!();(*req).complete=&mut c;
 mutex_lock(&mut ecard_mutex);ecard_req=req;wake_up(&mut ecard_wait);
 wait_for_completion(&mut c);mutex_unlock(&mut ecard_mutex);
}
unsafe fn ecard_readbytes(addr:*mut core::ffi::c_void,ec:*mut ecard_t,off:i32,len:i32,loader:i32){
 let mut r=ecard_request{fn_:Some(ecard_task_readbytes),ec,address:off as u32,length:len as u32,use_loader:loader as u32,buffer:addr,complete:core::ptr::null_mut()};ecard_call(&mut r);
}
pub unsafe fn ecard_readchunk(cd:*mut in_chunk_dir,ec:*mut ecard_t,id:i32,mut num:i32)->i32{
 if (*ec).cid.cd==0{return 0} let mut index=16;let mut use_loader=0;
 loop{let mut x=core::mem::zeroed::<ex_chunk_dir>();ecard_readbytes(&mut x as *mut _ as _,ec,index,8,use_loader);index+=8;
 let xid=c_id(&x);if xid==0{if use_loader==0&&!(*ec).loader.is_null(){use_loader=1;index=0;continue}return 0}
 if xid==0xf0{index=c_start(&x);continue} if xid==0x80{if (*ec).loader.is_null(){(*ec).loader=kmalloc(c_len(&x),GFP_KERNEL);if (*ec).loader.is_null(){return 0}ecard_readbytes((*ec).loader,ec,c_start(&x) as i32,c_len(&x),use_loader)}continue}
 if xid==id{if num==0{if xid&0x80!=0&&xid&0x70==0x70{ecard_readbytes(x.d.string.as_mut_ptr() as _,ec,c_start(&x) as i32,c_len(&x),use_loader)}(*cd).start_offset=c_start(&x);memcpy((*cd).d.string.as_mut_ptr() as _,x.d.string.as_ptr() as _,256);return 1}num-=1}}
}
unsafe fn ecard_def_irq_enable(_: *mut ecard_t,_:i32){}
unsafe fn ecard_def_irq_disable(_: *mut ecard_t,_:i32){}
unsafe fn ecard_def_irq_pending(ec:*mut ecard_t)->i32{if (*ec).irqmask==0||readb((*ec).irqaddr)&(*ec).irqmask!=0{1}else{0}}
unsafe fn ecard_def_fiq_enable(_: *mut ecard_t,_:i32){panic!("ecard_def_fiq_enable called - impossible")}
unsafe fn ecard_def_fiq_disable(_: *mut ecard_t,_:i32){panic!("ecard_def_fiq_disable called - impossible")}
unsafe fn ecard_def_fiq_pending(ec:*mut ecard_t)->i32{if (*ec).fiqmask==0||readb((*ec).fiqaddr)&(*ec).fiqmask!=0{1}else{0}}
static mut ecard_default_ops:expansioncard_ops_t=expansioncard_ops_t{irqenable:ecard_def_irq_enable,irqdisable:ecard_def_irq_disable,irqpending:ecard_def_irq_pending,fiqenable:ecard_def_fiq_enable,fiqdisable:ecard_def_fiq_disable,fiqpending:ecard_def_fiq_pending};

pub unsafe fn ecard_enablefiq(n:u32){let e=slot_to_ecard(n);if !e.is_null(){if (*e).ops.is_null(){(*e).ops=&mut ecard_default_ops}if (*e).claimed!=0{((*(*e).ops).fiqenable)(e,n as i32)}}}
pub unsafe fn ecard_disablefiq(n:u32){let e=slot_to_ecard(n);if !e.is_null()&&!(*e).ops.is_null(){((*(*e).ops).fiqdisable)(e,n as i32)}}
pub unsafe fn ecard_setirq(ec:*mut ecard_t,ops:*const expansioncard_ops_t,data:*mut core::ffi::c_void){(*ec).irq_data=data;barrier();(*ec).ops=ops as *mut _;}
pub unsafe fn ecard_request_resources(_: *mut ecard_t)->i32{unimplemented!("external kernel resource ABI")}
pub unsafe fn ecard_release_resources(_: *mut ecard_t){}
pub unsafe fn ecardm_iomap(_: *mut ecard_t,_:u32,_:usize,_:usize)->*mut core::ffi::c_void{unimplemented!("external kernel mapping ABI")}
// CONFIG_ARCH_RPC and IOMD_ECTCR conditional bodies remain build-time dependent.


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
