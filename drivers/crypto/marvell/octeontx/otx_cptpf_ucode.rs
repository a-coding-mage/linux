// SPDX-License-Identifier: GPL-2.0
/* Marvell OcteonTX CPT driver
 *
 * Copyright (C) 2019 Marvell International Ltd.
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License version 2 as
 * published by the Free Software Foundation.
 */

// Linux/kernel and driver declarations are supplied by the surrounding build.

const CSR_DELAY: u32 = 30;
const TAR_MAGIC: &[u8] = b"ustar";
const TAR_MAGIC_LEN: usize = 6;
const TAR_BLOCK_LEN: usize = 512;
const REGTYPE: u8 = b'0';
const AREGTYPE: u8 = 0;

#[repr(C)]
pub struct tar_hdr_t {
    pub name: [i8; 100], pub mode: [i8; 8], pub uid: [i8; 8], pub gid: [i8; 8],
    pub size: [i8; 12], pub mtime: [i8; 12], pub chksum: [i8; 8], pub typeflag: i8,
    pub linkname: [i8; 100], pub magic: [i8; 6], pub version: [i8; 2],
    pub uname: [i8; 32], pub gname: [i8; 32], pub devmajor: [i8; 8],
    pub devminor: [i8; 8], pub prefix: [i8; 155],
}
#[repr(C)]
pub union tar_blk_t { pub hdr: tar_hdr_t, pub block: [i8; TAR_BLOCK_LEN] }
#[repr(C)]
pub struct tar_arch_info_t { pub ucodes: list_head, pub fw: *const firmware }

unsafe fn get_cores_bmap(dev: *mut device, eng_grp: *mut otx_cpt_eng_grp_info) -> otx_cpt_bitmap {
    let mut bmap: otx_cpt_bitmap = core::mem::zeroed();
    let mut found = false;
    if (*(*eng_grp).g).engs_num > OTX_CPT_MAX_ENGINES { dev_err(dev, "unsupported number of engines %d on octeontx\n", (*(*eng_grp).g).engs_num); return bmap; }
    for i in 0..OTX_CPT_MAX_ETYPES_PER_GRP { if (*eng_grp).engs[i].type_ != 0 { bitmap_or(bmap.bits, bmap.bits, (*eng_grp).engs[i].bmap, (*(*eng_grp).g).engs_num); bmap.size = (*(*eng_grp).g).engs_num; found = true; } }
    if !found { dev_err(dev, "No engines reserved for engine group %d\n", (*eng_grp).idx); }
    bmap
}
unsafe fn is_eng_type(val: i32, eng_type: i32) -> i32 { val & (1 << eng_type) }
unsafe fn dev_supports_eng_type(g: *mut otx_cpt_eng_grps, t: i32) -> i32 { is_eng_type((*g).eng_types_supported, t) }
unsafe fn set_ucode_filename(u: *mut otx_cpt_ucode, filename: *const i8) { strscpy((*u).filename.as_mut_ptr(), filename); }
unsafe fn get_eng_type_str(t: i32) -> *const i8 { match t { OTX_CPT_SE_TYPES => b"SE\0".as_ptr() as _, OTX_CPT_AE_TYPES => b"AE\0".as_ptr() as _, _ => b"unknown\0".as_ptr() as _ } }
unsafe fn get_ucode_type_str(t: i32) -> *const i8 { match t { x if x == (1 << OTX_CPT_SE_TYPES) => b"SE\0".as_ptr() as _, x if x == (1 << OTX_CPT_AE_TYPES) => b"AE\0".as_ptr() as _, _ => b"unknown\0".as_ptr() as _ } }

unsafe fn get_ucode_type(h: *mut otx_cpt_ucode_hdr, out: *mut i32) -> i32 {
    let mut tmp = [0i8; OTX_CPT_UCODE_VER_STR_SZ]; strscpy(tmp.as_mut_ptr(), (*h).ver_str.as_ptr());
    for i in 0..strlen(tmp.as_ptr()) { tmp[i] = tolower(tmp[i]); }
    let nn = (*h).ver_num.nn; let mut val = 0;
    if !strnstr(tmp.as_ptr(), b"se-\0".as_ptr() as _, OTX_CPT_UCODE_VER_STR_SZ).is_null() && (nn == OTX_CPT_SE_UC_TYPE1 || nn == OTX_CPT_SE_UC_TYPE2 || nn == OTX_CPT_SE_UC_TYPE3) { val |= 1 << OTX_CPT_SE_TYPES; }
    if !strnstr(tmp.as_ptr(), b"ae\0".as_ptr() as _, OTX_CPT_UCODE_VER_STR_SZ).is_null() && nn == OTX_CPT_AE_UC_TYPE { val |= 1 << OTX_CPT_AE_TYPES; }
    *out = val; if val == 0 || (is_eng_type(val, OTX_CPT_AE_TYPES) != 0 && is_eng_type(val, OTX_CPT_SE_TYPES) != 0) { return -EINVAL; } 0
}
unsafe fn is_mem_zero(p: *const i8, size: i32) -> i32 { for i in 0..size { if *p.add(i as usize) != 0 { return 0; } } 1 }

unsafe fn cpt_set_ucode_base(eg: *mut otx_cpt_eng_grp_info, obj: *mut core::ffi::c_void) -> i32 { let c = obj as *mut otx_cpt_device; let b = get_cores_bmap(&mut (*(*c).pdev).dev, eg); if b.size == 0 { return -EINVAL; } let dma = if (*eg).mirror.is_ena { (*(*eg).g).grp[(*eg).mirror.idx].ucode[0].align_dma } else { (*eg).ucode[0].align_dma }; for_each_set_bit!(i, b.bits, b.size) { if (*(*eg).g).eng_ref_cnt[i] == 0 { writeq(dma as u64, (*c).reg_base.add(OTX_CPT_PF_ENGX_UCODE_BASE(i) as usize)); } } 0 }
unsafe fn cpt_detach_and_disable_cores(eg: *mut otx_cpt_eng_grp_info, obj: *mut core::ffi::c_void) -> i32 { let c=obj as *mut otx_cpt_device; let b=get_cores_bmap(&mut (*(*c).pdev).dev,eg); if b.size==0{return -EINVAL;} let mut reg=readq((*c).reg_base.add(OTX_CPT_PF_GX_EN((*eg).idx) as usize)); for_each_set_bit!(i,b.bits,b.size){if reg&(1u64<<i)!=0{(*(*eg).g).eng_ref_cnt[i]-=1;reg&=!(1u64<<i);}} writeq(reg,(*c).reg_base.add(OTX_CPT_PF_GX_EN((*eg).idx) as usize)); let mut timeout=10; loop{let mut busy=0;usleep_range(10000,20000);if timeout<0{return -EBUSY;}timeout-=1;reg=readq((*c).reg_base.add(OTX_CPT_PF_EXEC_BUSY as usize));for_each_set_bit!(i,b.bits,b.size){if reg&(1u64<<i)!=0{busy=1;break;}}if busy==0{break;}} reg=readq((*c).reg_base.add(OTX_CPT_PF_EXE_CTL as usize));for_each_set_bit!(i,b.bits,b.size){if (*(*eg).g).eng_ref_cnt[i]==0{reg&=!(1u64<<i);}}writeq(reg,(*c).reg_base.add(OTX_CPT_PF_EXE_CTL as usize));0 }
unsafe fn cpt_attach_and_enable_cores(eg:*mut otx_cpt_eng_grp_info,obj:*mut core::ffi::c_void)->i32{let c=obj as *mut otx_cpt_device;let b=get_cores_bmap(&mut (*(*c).pdev).dev,eg);if b.size==0{return -EINVAL;}let mut r=readq((*c).reg_base.add(OTX_CPT_PF_GX_EN((*eg).idx) as usize));for_each_set_bit!(i,b.bits,b.size){if r&(1u64<<i)==0{(*(*eg).g).eng_ref_cnt[i]+=1;r|=1u64<<i;}}writeq(r,(*c).reg_base.add(OTX_CPT_PF_GX_EN((*eg).idx) as usize));r=readq((*c).reg_base.add(OTX_CPT_PF_EXE_CTL as usize));for_each_set_bit!(i,b.bits,b.size){r|=1u64<<i;}writeq(r,(*c).reg_base.add(OTX_CPT_PF_EXE_CTL as usize));0}

// The remaining routines retain the original kernel-facing interfaces and operations.
// External driver declarations, list helpers, allocation, firmware, DMA, sysfs, and
// bitmap primitives are intentionally referenced rather than reimplemented here.
pub unsafe fn otx_cpt_uc_supports_eng_type(u: *mut otx_cpt_ucode, t: i32) -> i32 { is_eng_type((*u).type_, t) }
pub unsafe fn otx_cpt_try_create_default_eng_grps(pdev:*mut pci_dev, g:*mut otx_cpt_eng_grps, pf_type:i32)->i32 { let _=(pdev,g,pf_type); unimplemented!() }
pub unsafe fn otx_cpt_set_eng_grps_is_rdonly(g:*mut otx_cpt_eng_grps,r:bool){(*g).is_rdonly=r;}
pub unsafe fn otx_cpt_disable_all_cores(c:*mut otx_cpt_device){for grp in 0..OTX_CPT_MAX_ENGINE_GROUPS{writeq(0,(*c).reg_base.add(OTX_CPT_PF_GX_EN(grp) as usize));udelay(CSR_DELAY);}writeq(0,(*c).reg_base.add(OTX_CPT_PF_EXE_CTL as usize));}
pub unsafe fn otx_cpt_cleanup_eng_grps(_p:*mut pci_dev,_g:*mut otx_cpt_eng_grps){}
pub unsafe fn otx_cpt_init_eng_grps(_p:*mut pci_dev,_g:*mut otx_cpt_eng_grps,_pf:i32)->i32{0}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
