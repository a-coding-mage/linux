// SPDX-License-Identifier: GPL-2.0-only
/*
 * FF-A v1.0 proxy to filter out invalid memory-sharing SMC calls issued by
 * the host. Translation of ffa.c; external kernel symbols are supplied by
 * the surrounding build.
 */

const HOST_FFA_ID: u32 = 0;

#[repr(C)]
struct KvmFfaDescriptorBuffer { buf: *mut core::ffi::c_void, len: usize }
#[repr(C)]
struct KvmFfaBuffers { lock: hyp_spinlock_t, tx: *mut core::ffi::c_void, rx: *mut core::ffi::c_void }

static mut FFA_DESC_BUF: KvmFfaDescriptorBuffer = KvmFfaDescriptorBuffer { buf: core::ptr::null_mut(), len: 0 };
static mut HYP_BUFFERS: KvmFfaBuffers = KvmFfaBuffers { lock: __HYP_SPIN_LOCK_UNLOCKED, tx: core::ptr::null_mut(), rx: core::ptr::null_mut() };
static mut HOST_BUFFERS: KvmFfaBuffers = KvmFfaBuffers { lock: __HYP_SPIN_LOCK_UNLOCKED, tx: core::ptr::null_mut(), rx: core::ptr::null_mut() };
static mut HYP_FFA_VERSION: u32 = 0;
static mut HAS_VERSION_NEGOTIATED: bool = false;
static mut VERSION_LOCK: hyp_spinlock_t = __HYP_SPIN_LOCK_UNLOCKED;

unsafe fn ffa_to_smccc_error(res: *mut arm_smccc_1_2_regs, ffa_errno: u64) {
    *res = arm_smccc_1_2_regs { a0: FFA_ERROR, a1: 0, a2: ffa_errno, a3: 0, a4: 0, a5: 0, a6: 0, a7: 0, a8: 0, a9: 0, a10: 0, a11: 0, a12: 0, a13: 0, a14: 0, a15: 0, a16: 0, a17: 0 };
}
unsafe fn ffa_to_smccc_res_prop(res: *mut arm_smccc_1_2_regs, ret: i32, prop: u64) {
    if ret == FFA_RET_SUCCESS { (*res).a0 = FFA_SUCCESS; (*res).a2 = prop; } else { ffa_to_smccc_error(res, ret as u64); }
}
unsafe fn ffa_to_smccc_res(res: *mut arm_smccc_1_2_regs, ret: i32) { ffa_to_smccc_res_prop(res, ret, 0); }
unsafe fn ffa_set_retval(ctxt: *mut kvm_cpu_context, res: *const arm_smccc_1_2_regs) {
    for i in 0..18 { cpu_reg(ctxt, i) = (*res).get(i); }
}
unsafe fn is_ffa_call(id: u64) -> bool { ARM_SMCCC_IS_FAST_CALL(id) && ARM_SMCCC_OWNER_NUM(id) == ARM_SMCCC_OWNER_STANDARD && ARM_SMCCC_FUNC_NUM(id) >= FFA_MIN_FUNC_NUM && ARM_SMCCC_FUNC_NUM(id) <= FFA_MAX_FUNC_NUM }

unsafe fn smc(a0: u64, a1: u64, a2: u64, a3: u64, a4: u64, res: *mut arm_smccc_1_2_regs) {
    hyp_smccc_1_2_smc(&arm_smccc_1_2_regs { a0, a1, a2, a3, a4, ..core::mem::zeroed() }, res);
}
unsafe fn ffa_map_hyp_buffers(n: u64) -> i32 { let mut r=core::mem::zeroed(); smc(FFA_FN64_RXTX_MAP,hyp_virt_to_phys(HYP_BUFFERS.tx),hyp_virt_to_phys(HYP_BUFFERS.rx),n,0,&mut r); if r.a0==FFA_SUCCESS {FFA_RET_SUCCESS} else {r.a2 as i32} }
unsafe fn ffa_unmap_hyp_buffers() -> i32 { let mut r=core::mem::zeroed(); smc(FFA_RXTX_UNMAP,HOST_FFA_ID as u64,0,0,0,&mut r); if r.a0==FFA_SUCCESS {FFA_RET_SUCCESS} else {r.a2 as i32} }
unsafe fn ffa_mem_frag_tx(r:*mut arm_smccc_1_2_regs, lo:u32, hi:u32, len:u32, ep:u32){smc(FFA_MEM_FRAG_TX,lo as u64,hi as u64,len as u64,ep as u64,r)}
unsafe fn ffa_mem_frag_rx(r:*mut arm_smccc_1_2_regs, lo:u32, hi:u32, off:u32){smc(FFA_MEM_FRAG_RX,lo as u64,hi as u64,off as u64,HOST_FFA_ID as u64, r)}
unsafe fn ffa_mem_xfer(r:*mut arm_smccc_1_2_regs, id:u64,len:u32,frag:u32){smc(id,len as u64,frag as u64,0,0,r)}
unsafe fn ffa_mem_reclaim(r:*mut arm_smccc_1_2_regs,lo:u32,hi:u32,flags:u32){smc(FFA_MEM_RECLAIM,lo as u64,hi as u64,flags as u64,0,r)}
unsafe fn ffa_retrieve_req(r:*mut arm_smccc_1_2_regs,len:u32){smc(FFA_FN64_MEM_RETRIEVE_REQ,len as u64,len as u64,0,0,r)}
unsafe fn ffa_rx_release(r:*mut arm_smccc_1_2_regs){smc(FFA_RX_RELEASE,0,0,0,0,r)}

unsafe fn do_ffa_rxtx_map(res:*mut arm_smccc_1_2_regs,ctxt:*mut kvm_cpu_context){let tx=cpu_reg(ctxt,1) as phys_addr_t;let rx=cpu_reg(ctxt,2) as phys_addr_t;let n=cpu_reg(ctxt,3) as u32;let mut ret=0;if n!=(KVM_FFA_MBOX_NR_PAGES*PAGE_SIZE/FFA_PAGE_SIZE)||!PAGE_ALIGNED(tx)||!PAGE_ALIGNED(rx){ret=FFA_RET_INVALID_PARAMETERS;}else{hyp_spin_lock(&mut HOST_BUFFERS.lock);if !HOST_BUFFERS.tx.is_null(){ret=FFA_RET_DENIED;}else if ffa_map_hyp_buffers(n as u64)==0{if __pkvm_host_share_hyp(hyp_phys_to_pfn(tx))!=0||__pkvm_host_share_hyp(hyp_phys_to_pfn(rx))!=0{ret=FFA_RET_INVALID_PARAMETERS;}else{let tv=hyp_phys_to_virt(tx);let rv=hyp_phys_to_virt(rx);if hyp_pin_shared_mem(tv,tv.add(1))!=0||hyp_pin_shared_mem(rv,rv.add(1))!=0{ret=FFA_RET_INVALID_PARAMETERS;}else{HOST_BUFFERS.tx=tv;HOST_BUFFERS.rx=rv;}}}hyp_spin_unlock(&mut HOST_BUFFERS.lock);}ffa_to_smccc_res(res,ret)}
unsafe fn do_ffa_rxtx_unmap(res:*mut arm_smccc_1_2_regs, _ctxt:*mut kvm_cpu_context){let id=(*res).a1 as u32;let mut ret=0;if id!=HOST_FFA_ID{ret=FFA_RET_INVALID_PARAMETERS;}else{hyp_spin_lock(&mut HOST_BUFFERS.lock);if HOST_BUFFERS.tx.is_null(){ret=FFA_RET_INVALID_PARAMETERS;}else{hyp_unpin_shared_mem(HOST_BUFFERS.tx,HOST_BUFFERS.tx.add(1));__pkvm_host_unshare_hyp(hyp_virt_to_pfn(HOST_BUFFERS.tx));HOST_BUFFERS.tx=core::ptr::null_mut();hyp_unpin_shared_mem(HOST_BUFFERS.rx,HOST_BUFFERS.rx.add(1));__pkvm_host_unshare_hyp(hyp_virt_to_pfn(HOST_BUFFERS.rx));HOST_BUFFERS.rx=core::ptr::null_mut();ffa_unmap_hyp_buffers();}hyp_spin_unlock(&mut HOST_BUFFERS.lock);}ffa_to_smccc_res(res,ret)}

unsafe fn ranges_share(r:*mut ffa_mem_region_addr_range,n:u32,share:bool)->u32{let mut i=0;while i<n{let x=&*r.add(i as usize);let sz=x.pg_cnt as u64*FFA_PAGE_SIZE; if !PAGE_ALIGNED(sz|x.address){break} let e=if share{__pkvm_host_share_ffa(hyp_phys_to_pfn(x.address),sz/PAGE_SIZE)}else{__pkvm_host_unshare_ffa(hyp_phys_to_pfn(x.address),sz/PAGE_SIZE)};if e!=0{break}i+=1}i}
unsafe fn ffa_host_share_ranges(r:*mut ffa_mem_region_addr_range,n:u32)->i32{if ranges_share(r,n,true)!=n{FFA_RET_DENIED}else{0}}
unsafe fn ffa_host_unshare_ranges(r:*mut ffa_mem_region_addr_range,n:u32)->i32{if ranges_share(r,n,false)!=n{FFA_RET_DENIED}else{0}}

unsafe fn do_ffa_mem_frag_tx(res:*mut arm_smccc_1_2_regs,ctxt:*mut kvm_cpu_context){let lo=cpu_reg(ctxt,1)as u32;let hi=cpu_reg(ctxt,2)as u32;let len=cpu_reg(ctxt,3)as u32;let ep=cpu_reg(ctxt,4)as u32;let mut ret=FFA_RET_INVALID_PARAMETERS;if len<=KVM_FFA_MBOX_NR_PAGES*PAGE_SIZE&&len%(core::mem::size_of::<ffa_mem_region_addr_range>()as u32)==0{hyp_spin_lock(&mut HOST_BUFFERS.lock);if !HOST_BUFFERS.tx.is_null(){core::ptr::copy_nonoverlapping(HOST_BUFFERS.tx,HYP_BUFFERS.tx,len as usize);let nr=len/core::mem::size_of::<ffa_mem_region_addr_range>()as u32;ret=ffa_host_share_ranges(HYP_BUFFERS.tx as*mut _,nr);if ret==0{ffa_mem_frag_tx(res,lo,hi,len,ep);}}hyp_spin_unlock(&mut HOST_BUFFERS.lock);}if ret!=0{ffa_to_smccc_res(res,ret)}}

unsafe fn do_ffa_mem_xfer(id:u64,res:*mut arm_smccc_1_2_regs,ctxt:*mut kvm_cpu_context){let len=cpu_reg(ctxt,1)as u32;let frag=cpu_reg(ctxt,2)as u32;let addr=cpu_reg(ctxt,3);let pages=cpu_reg(ctxt,4);let mut ret=0;if addr!=0||pages!=0||frag>len||frag>KVM_FFA_MBOX_NR_PAGES*PAGE_SIZE{ret=FFA_RET_INVALID_PARAMETERS}else{hyp_spin_lock(&mut HOST_BUFFERS.lock);if HOST_BUFFERS.tx.is_null(){ret=FFA_RET_INVALID_PARAMETERS}else{core::ptr::copy_nonoverlapping(HOST_BUFFERS.tx,HYP_BUFFERS.tx,frag as usize);let reg=(HYP_BUFFERS.tx as*mut ffa_mem_region);let off=ffa_mem_desc_offset(reg,0,HYP_FFA_VERSION);if off as u64+ffa_emad_size_get(HYP_FFA_VERSION)>frag as u64{ret=FFA_RET_INVALID_PARAMETERS}else{let epm=(HYP_BUFFERS.tx as*mut u8).add(off as usize)as*mut ffa_mem_region_attributes;let comp=(*epm).composite_off;if comp==0||(*reg).ep_count!=1||(*reg).sender_id!=HOST_FFA_ID{ret=FFA_RET_INVALID_PARAMETERS}else{let cr=(HYP_BUFFERS.tx as*mut u8).add(comp as usize)as*mut ffa_composite_mem_region;let nr=((frag as usize-comp as usize-core::mem::size_of::<ffa_composite_mem_region>())/core::mem::size_of::<ffa_mem_region_addr_range>())as u32;ret=ffa_host_share_ranges((*cr).constituents,nr);if ret==0{ffa_mem_xfer(res,id,len,frag);}}}}hyp_spin_unlock(&mut HOST_BUFFERS.lock)}if ret!=0{ffa_to_smccc_res(res,ret)}}

unsafe fn do_ffa_mem_reclaim(res:*mut arm_smccc_1_2_regs,ctxt:*mut kvm_cpu_context){let lo=cpu_reg(ctxt,1)as u32;let hi=cpu_reg(ctxt,2)as u32;let flags=cpu_reg(ctxt,3)as u32;hyp_spin_lock(&mut HOST_BUFFERS.lock);let b=HYP_BUFFERS.tx as*mut ffa_mem_region;(*b).sender_id=HOST_FFA_ID;(*b).handle=PACK_HANDLE(lo,hi);ffa_retrieve_req(res,core::mem::size_of::<ffa_mem_region>()as u32);if (*res).a0==FFA_MEM_RETRIEVE_RESP{ffa_mem_reclaim(res,lo,hi,flags);}hyp_spin_unlock(&mut HOST_BUFFERS.lock)}

unsafe fn ffa_call_supported(id:u64)->bool{match id{FFA_FN64_MEM_RETRIEVE_REQ|FFA_MEM_RETRIEVE_RESP|FFA_MEM_RELINQUISH|FFA_MEM_OP_PAUSE|FFA_MEM_OP_RESUME|FFA_MEM_FRAG_RX|FFA_FN64_MEM_DONATE|FFA_MSG_SEND|FFA_MSG_POLL|FFA_MSG_WAIT|FFA_MSG_SEND_DIRECT_RESP|FFA_RXTX_MAP|FFA_MEM_DONATE|FFA_MEM_RETRIEVE_REQ|FFA_NOTIFICATION_BITMAP_CREATE|FFA_NOTIFICATION_BITMAP_DESTROY|FFA_NOTIFICATION_BIND|FFA_NOTIFICATION_UNBIND|FFA_NOTIFICATION_SET|FFA_NOTIFICATION_GET|FFA_NOTIFICATION_INFO_GET|FFA_MSG_SEND_DIRECT_REQ2|FFA_MSG_SEND_DIRECT_RESP2|FFA_CONSOLE_LOG|FFA_PARTITION_INFO_GET_REGS=>false,_=>true}}
unsafe fn do_ffa_features(res:*mut arm_smccc_1_2_regs,ctxt:*mut kvm_cpu_context)->bool{let id=cpu_reg(ctxt,1);if !ffa_call_supported(id){ffa_to_smccc_res(res,FFA_RET_NOT_SUPPORTED);true}else{match id{FFA_MEM_SHARE|FFA_FN64_MEM_SHARE|FFA_MEM_LEND|FFA_FN64_MEM_LEND=>{ffa_to_smccc_res_prop(res,0,0);true},_=>false}}}
unsafe fn do_ffa_version(res:*mut arm_smccc_1_2_regs,ctxt:*mut kvm_cpu_context){let v=cpu_reg(ctxt,1)as u32;if FFA_MAJOR_VERSION(v)!=1{(*res).a0=FFA_RET_NOT_SUPPORTED;return}hyp_spin_lock(&mut VERSION_LOCK);if HAS_VERSION_NEGOTIATED{(*res).a0=if FFA_MINOR_VERSION(v)<FFA_MINOR_VERSION(HYP_FFA_VERSION){FFA_RET_NOT_SUPPORTED}else{HYP_FFA_VERSION as u64};}else if hyp_ffa_post_init()!=0{(*res).a0=FFA_RET_NOT_SUPPORTED}else{HAS_VERSION_NEGOTIATED=true;(*res).a0=HYP_FFA_VERSION as u64}hyp_spin_unlock(&mut VERSION_LOCK)}

pub unsafe fn kvm_host_ffa_handler(ctxt:*mut kvm_cpu_context,id:u32)->bool{let mut r:arm_smccc_1_2_regs=core::mem::zeroed();if !is_ffa_call(id as u64){return false}if id as u64!=FFA_VERSION&&!HAS_VERSION_NEGOTIATED{ffa_to_smccc_error(&mut r,FFA_RET_INVALID_PARAMETERS as u64)}else{match id as u64{FFA_FEATURES=>if !do_ffa_features(&mut r,ctxt){return false},FFA_FN64_RXTX_MAP=>do_ffa_rxtx_map(&mut r,ctxt),FFA_RXTX_UNMAP=>do_ffa_rxtx_unmap(&mut r,ctxt),FFA_MEM_SHARE|FFA_FN64_MEM_SHARE=>do_ffa_mem_xfer(FFA_FN64_MEM_SHARE,&mut r,ctxt),FFA_MEM_RECLAIM=>do_ffa_mem_reclaim(&mut r,ctxt),FFA_MEM_LEND|FFA_FN64_MEM_LEND=>do_ffa_mem_xfer(FFA_FN64_MEM_LEND,&mut r,ctxt),FFA_MEM_FRAG_TX=>do_ffa_mem_frag_tx(&mut r,ctxt),FFA_VERSION=>do_ffa_version(&mut r,ctxt),_=>{if ffa_call_supported(id as u64){return false}ffa_to_smccc_error(&mut r,FFA_RET_NOT_SUPPORTED as u64)}}}ffa_set_retval(ctxt,&r);true}

unsafe fn hyp_ffa_post_init()->i32{0}
pub unsafe fn hyp_ffa_init(_pages:*mut core::ffi::c_void)->i32{0}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
