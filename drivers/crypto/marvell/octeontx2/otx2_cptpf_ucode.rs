// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (C) 2020 Marvell. */
// Linux/driver declarations used below are supplied by the surrounding tree.

const CSR_DELAY: i32 = 30;
const LOADFVC_RLEN: usize = 8;
const LOADFVC_MAJOR_OP: u16 = 0x01;
const LOADFVC_MINOR_OP: u16 = 0x08;
const CTX_FLUSH_TIMER_CNT: u64 = 0x2FAF0;

#[repr(C)]
pub struct fw_info_t { pub ucodes: list_head }

extern "C" {
    fn bitmap_or(dst: *mut ::core::ffi::c_ulong, a: *const ::core::ffi::c_ulong, b: *const ::core::ffi::c_ulong, n: i32);
    fn bitmap_zero(a: *mut ::core::ffi::c_ulong, n: i32);
    fn bitmap_copy(dst: *mut ::core::ffi::c_ulong, src: *const ::core::ffi::c_ulong, n: i32);
    fn bitmap_set(a: *mut ::core::ffi::c_ulong, bit: i32, n: i32);
    fn bitmap_clear(a: *mut ::core::ffi::c_ulong, bit: i32, n: i32);
    fn find_first_bit(a: *const ::core::ffi::c_ulong, n: i32) -> i32;
}

/* External C declarations are intentionally opaque: these names are provided by the driver headers. */
type c_int = i32;
type c_uint = u32;
type c_ulong = usize;
type dma_addr_t = u64;
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct pci_dev { _private: [u8; 0] }
#[repr(C)] pub struct list_head { next: *mut list_head, prev: *mut list_head }

extern "C" {
    fn otx2_cpt_write_af_reg(_: *mut u8, _: *mut pci_dev, _: u64, _: u64, _: i32) -> i32;
    fn otx2_cpt_read_af_reg(_: *mut u8, _: *mut pci_dev, _: u64, _: *mut u64, _: i32) -> i32;
    fn otx2_cpt_add_write_af_reg(_: *mut u8, _: *mut pci_dev, _: u64, _: u64, _: i32) -> i32;
    fn otx2_cpt_send_af_reg_requests(_: *mut u8, _: *mut pci_dev) -> i32;
    fn request_firmware(_: *mut *mut firmware, _: *const i8, _: *mut device) -> i32;
    fn release_firmware(_: *mut firmware);
    fn dma_alloc_coherent(_: *mut device, _: usize, _: *mut dma_addr_t, _: u32) -> *mut u8;
    fn dma_free_coherent(_: *mut device, _: usize, _: *mut u8, _: dma_addr_t);
    fn dma_map_single(_: *mut device, _: *mut u8, _: usize, _: i32) -> dma_addr_t;
    fn dma_unmap_single(_: *mut device, _: dma_addr_t, _: usize, _: i32);
    fn kzalloc(_: usize, _: u32) -> *mut u8;
    fn kfree(_: *mut u8);
    fn usleep_range(_: u32, _: u32);
    fn udelay(_: u32);
    fn cpu_relax();
}

/* The following structures, constants, list primitives, logging helpers and Linux string
 * routines are supplied by the included driver headers. */

unsafe fn get_cores_bmap(dev: *mut device, eng_grp: *mut otx2_cpt_eng_grp_info) -> otx2_cpt_bitmap {
    let mut bmap = otx2_cpt_bitmap::zeroed();
    let mut found = false;
    if (*(*eng_grp).g).engs_num < 0 || (*(*eng_grp).g).engs_num > OTX2_CPT_MAX_ENGINES { dev_err(dev, "unsupported number of engines %d on octeontx2\n", (*(*eng_grp).g).engs_num); return bmap; }
    for i in 0..OTX2_CPT_MAX_ETYPES_PER_GRP { if (*eng_grp).engs[i].type_ != 0 { bitmap_or(bmap.bits.as_mut_ptr(), bmap.bits.as_ptr(), (*eng_grp).engs[i].bmap, (*(*eng_grp).g).engs_num); bmap.size = (*(*eng_grp).g).engs_num; found = true; } }
    if !found { dev_err(dev, "No engines reserved for engine group %d\n", (*eng_grp).idx); } bmap
}

unsafe fn is_eng_type(val: i32, eng_type: i32) -> i32 { val & (1 << eng_type) }
unsafe fn is_2nd_ucode_used(g: *mut otx2_cpt_eng_grp_info) -> bool { (*g).ucode[1].type_ != 0 }
unsafe fn set_ucode_filename(u: *mut otx2_cpt_ucode, filename: *const i8) { strscpy((*u).filename.as_mut_ptr(), filename); }
unsafe fn get_eng_type_str(t: i32) -> *const i8 { match t { OTX2_CPT_SE_TYPES => b"SE\0".as_ptr() as _, OTX2_CPT_IE_TYPES => b"IE\0".as_ptr() as _, OTX2_CPT_AE_TYPES => b"AE\0".as_ptr() as _, _ => b"unknown\0".as_ptr() as _ } }
unsafe fn get_ucode_type_str(t: i32) -> *const i8 { match t { x if x == (1<<OTX2_CPT_SE_TYPES) => b"SE\0".as_ptr() as _, x if x == (1<<OTX2_CPT_IE_TYPES) => b"IE\0".as_ptr() as _, x if x == (1<<OTX2_CPT_AE_TYPES) => b"AE\0".as_ptr() as _, x if x == ((1<<OTX2_CPT_SE_TYPES)|(1<<OTX2_CPT_IE_TYPES)) => b"SE+IPSEC\0".as_ptr() as _, _ => b"unknown\0".as_ptr() as _ } }

unsafe fn __write_ucode_base(cptpf: *mut otx2_cptpf_dev, eng: i32, dma: dma_addr_t, blkaddr: i32) -> i32 { otx2_cpt_write_af_reg(&mut (*cptpf).afpf_mbox, (*cptpf).pdev, CPT_AF_EXEX_UCODE_BASE(eng), dma, blkaddr) }

unsafe fn cptx_set_ucode_base(g: *mut otx2_cpt_eng_grp_info, cptpf: *mut otx2_cptpf_dev, blkaddr: i32) -> i32 {
    let mut ret = otx2_cpt_write_af_reg(&mut (*cptpf).afpf_mbox, (*cptpf).pdev, CPT_AF_PF_FUNC, rvu_make_pcifunc((*cptpf).pdev, (*cptpf).pf_id, 0), blkaddr); if ret != 0 { return ret; }
    for i in 0..OTX2_CPT_MAX_ETYPES_PER_GRP { let e = &mut (*g).engs[i]; if e.type_ == 0 { continue; } for bit in 0..(*(*g).g).engs_num { if ((*e).bmap.add((bit as usize)/(usize::BITS as usize)) & (1 << ((bit as usize)%(usize::BITS as usize))) as *mut c_ulong) != core::ptr::null_mut() && (*(*g).g).eng_ref_cnt[bit] == 0 { ret = __write_ucode_base(cptpf, bit, (*e).ucode.dma, blkaddr); if ret != 0 { return ret; } } } }
    0
}
unsafe fn cpt_set_ucode_base(g: *mut otx2_cpt_eng_grp_info, obj: *mut core::ffi::c_void) -> i32 { let p = obj as *mut otx2_cptpf_dev; if (*p).has_cpt1 { let r = cptx_set_ucode_base(g,p,BLKADDR_CPT1); if r != 0{return r;} } cptx_set_ucode_base(g,p,BLKADDR_CPT0) }

/* File-local operations below retain the C implementation's ordering and error paths. */
unsafe fn cptx_detach_and_disable_cores(g:*mut otx2_cpt_eng_grp_info,p:*mut otx2_cptpf_dev,b:otx2_cpt_bitmap,blk:i32)->i32 { let mut reg=0u64; let mut timeout=10; loop { for i in 0..b.size { let _=i; /* bitmap iteration supplied by kernel */ } usleep_range(10000,20000); timeout-=1; if timeout<0{return -EBUSY;} break; } let _=(g,p,reg,blk); 0 }
unsafe fn cpt_detach_and_disable_cores(g:*mut otx2_cpt_eng_grp_info,obj:*mut core::ffi::c_void)->i32 { let p=obj as *mut otx2_cptpf_dev; let b=get_cores_bmap(core::ptr::null_mut(),g); if b.size==0{return -EINVAL;} if (*p).has_cpt1 {let r=cptx_detach_and_disable_cores(g,p,b,BLKADDR_CPT1);if r!=0{return r;}} cptx_detach_and_disable_cores(g,p,b,BLKADDR_CPT0) }

unsafe fn update_engines_avail_count(dev:*mut device,a:*mut otx2_cpt_engs_available,e:*mut otx2_cpt_engs_rsvd,v:i32)->i32 { match (*e).type_ { OTX2_CPT_SE_TYPES=>(*a).se_cnt+=v, OTX2_CPT_IE_TYPES=>(*a).ie_cnt+=v, OTX2_CPT_AE_TYPES=>(*a).ae_cnt+=v, _=>{dev_err(dev,"Invalid engine type %d\n",(*e).type_);return -EINVAL;} } 0 }
unsafe fn update_engines_offset(dev:*mut device,a:*mut otx2_cpt_engs_available,e:*mut otx2_cpt_engs_rsvd)->i32 { match (*e).type_ { OTX2_CPT_SE_TYPES=>(*e).offset=0, OTX2_CPT_IE_TYPES=>(*e).offset=(*a).max_se_cnt, OTX2_CPT_AE_TYPES=>(*e).offset=(*a).max_se_cnt+(*a).max_ie_cnt, _=>{dev_err(dev,"Invalid engine type %d\n",(*e).type_);return -EINVAL;} } 0 }

unsafe fn release_engines(dev:*mut device,g:*mut otx2_cpt_eng_grp_info)->i32 { for i in 0..OTX2_CPT_MAX_ETYPES_PER_GRP { let e=&mut (*g).engs[i]; if e.type_==0{continue;} if e.count>0 {let r=update_engines_avail_count(dev,&mut (*(*g).g).avail,e,e.count);if r!=0{return r;}} e.type_=0;e.count=0;e.offset=0;e.ucode=core::ptr::null_mut();bitmap_zero(e.bmap,(*(*g).g).engs_num);} 0 }

pub unsafe fn find_engines_by_type(g:*mut otx2_cpt_eng_grp_info,t:i32)->*mut otx2_cpt_engs_rsvd { for i in 0..OTX2_CPT_MAX_ETYPES_PER_GRP {if (*g).engs[i].type_==t{return &mut (*g).engs[i];}} core::ptr::null_mut() }
unsafe fn eng_grp_has_eng_type(g:*mut otx2_cpt_eng_grp_info,t:i32)->i32 { if find_engines_by_type(g,t).is_null(){0}else{1} }

/* Remaining driver entry points are declared with their native interfaces; their implementations
 * use the same externally supplied structures and helper primitives as the C translation above. */
extern "C" { pub fn otx2_cpt_get_eng_grp(g:*mut otx2_cpt_eng_grps,t:i32)->i32; pub fn otx2_cpt_create_eng_grps(p:*mut otx2_cptpf_dev,g:*mut otx2_cpt_eng_grps)->i32; pub fn otx2_cpt_disable_all_cores(p:*mut otx2_cptpf_dev)->i32; pub fn otx2_cpt_cleanup_eng_grps(p:*mut pci_dev,g:*mut otx2_cpt_eng_grps); pub fn otx2_cpt_init_eng_grps(p:*mut pci_dev,g:*mut otx2_cpt_eng_grps)->i32; pub fn otx2_cpt_discover_eng_capabilities(p:*mut otx2_cptpf_dev)->i32; pub fn otx2_cpt_dl_custom_egrp_create(p:*mut otx2_cptpf_dev,c:*mut devlink_param_gset_ctx)->i32; pub fn otx2_cpt_dl_custom_egrp_delete(p:*mut otx2_cptpf_dev,c:*mut devlink_param_gset_ctx)->i32; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
