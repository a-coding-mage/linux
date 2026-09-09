// SPDX-License-Identifier: GPL-2.0
//
// Copyright 2020 HabanaLabs, Ltd.
// All Rights Reserved.

// Translated from security.c.  Types, constants, macros, and external
// functions referenced below are supplied by the surrounding driver.

static HL_GLBL_ERROR_CAUSE: [&'static str; 26] = [
    "Error due to un-priv read", "Error due to un-secure read",
    "Error due to read from unmapped reg", "Error due to un-priv write",
    "Error due to un-secure write", "Error due to write to unmapped reg",
    "N/A", "N/A", "N/A", "N/A", "N/A", "N/A", "N/A", "N/A", "N/A", "N/A",
    "External I/F write sec violation", "External I/F write to un-mapped reg",
    "N/A", "N/A", "N/A", "N/A", "N/A", "N/A", "Read to write only",
    "Write to read only",
];

unsafe fn hl_get_pb_block(hdev: *mut hl_device, mm_reg_addr: u32,
                          pb_blocks: *const u32, array_size: i32) -> i32 {
    for i in 0..array_size {
        let start_addr = *pb_blocks.add(i as usize);
        let end_addr = start_addr.wrapping_add(HL_BLOCK_SIZE);
        if mm_reg_addr >= start_addr && mm_reg_addr < end_addr { return i; }
    }
    dev_err((*hdev).dev, "No protection domain was found for 0x%x\n", mm_reg_addr);
    -EDOM
}

unsafe fn hl_unset_pb_in_block(hdev: *mut hl_device, reg_offset: u32,
                               sgs_entry: *mut hl_block_glbl_sec) -> i32 {
    if reg_offset >= HL_BLOCK_SIZE || (reg_offset & 0x3) != 0 {
        dev_err((*hdev).dev, "Register offset(%d) is out of range(%d) or invalid\n", reg_offset, HL_BLOCK_SIZE);
        return -EINVAL;
    }
    UNSET_GLBL_SEC_BIT((*sgs_entry).sec_array,
        ((reg_offset & (HL_BLOCK_SIZE - 1)) >> 2));
    0
}

pub unsafe fn hl_unsecure_register(hdev: *mut hl_device, mm_reg_addr: u32, offset: i32,
        pb_blocks: *const u32, sgs_array: *mut hl_block_glbl_sec, array_size: i32) -> i32 {
    let addr = mm_reg_addr.wrapping_add(offset as u32);
    let block_num = hl_get_pb_block(hdev, addr, pb_blocks, array_size);
    if block_num < 0 { return block_num; }
    hl_unset_pb_in_block(hdev, addr.wrapping_sub(*pb_blocks.add(block_num as usize)), sgs_array.add(block_num as usize))
}

unsafe fn hl_unsecure_register_range(hdev: *mut hl_device, mm_reg_range: range, offset: i32,
        pb_blocks: *const u32, sgs_array: *mut hl_block_glbl_sec, array_size: i32) -> i32 {
    let first = mm_reg_range.start.wrapping_add(offset as u32);
    let block_num = hl_get_pb_block(hdev, first, pb_blocks, array_size);
    if block_num < 0 { return block_num; }
    let mut rc = 0;
    let mut i = mm_reg_range.start;
    while i <= mm_reg_range.end {
        rc |= hl_unset_pb_in_block(hdev, i.wrapping_add(offset as u32).wrapping_sub(*pb_blocks.add(block_num as usize)), sgs_array.add(block_num as usize));
        i = i.wrapping_add(4);
    }
    rc
}

pub unsafe fn hl_unsecure_registers(hdev: *mut hl_device, mm_reg_array: *const u32,
        mm_array_size: i32, offset: i32, pb_blocks: *const u32,
        sgs_array: *mut hl_block_glbl_sec, blocks_array_size: i32) -> i32 {
    let mut rc = 0;
    for i in 0..mm_array_size {
        rc = hl_unsecure_register(hdev, *mm_reg_array.add(i as usize), offset, pb_blocks, sgs_array, blocks_array_size);
        if rc != 0 { return rc; }
    }
    rc
}

unsafe fn hl_unsecure_registers_range(hdev: *mut hl_device, a: *const range, n: i32, offset: i32,
        pb_blocks: *const u32, sgs_array: *mut hl_block_glbl_sec, size: i32) -> i32 {
    let mut rc = 0;
    for i in 0..n { rc = hl_unsecure_register_range(hdev, *a.add(i as usize), offset, pb_blocks, sgs_array, size); if rc != 0 { return rc; } }
    rc
}

unsafe fn hl_ack_pb_security_violations(hdev: *mut hl_device, pb_blocks: *const u32, block_offset: u32, array_size: i32) {
    for i in 0..array_size {
        let block_base = (*pb_blocks.add(i as usize)).wrapping_add(block_offset);
        let cause = RREG32(block_base + HL_BLOCK_GLBL_ERR_CAUSE);
        if cause != 0 {
            let addr = RREG32(block_base + HL_BLOCK_GLBL_ERR_ADDR);
            (*(*hdev).asic_funcs).pb_print_security_errors(hdev, block_base, cause, addr);
            WREG32(block_base + HL_BLOCK_GLBL_ERR_CAUSE, cause);
        }
    }
}

pub unsafe fn hl_config_glbl_sec(hdev: *mut hl_device, pb_blocks: *const u32, a: *mut hl_block_glbl_sec, block_offset: u32, n: i32) {
    if (*hdev).pldm { usleep_range(100, 1000); }
    for i in 0..n { let base = block_offset + *pb_blocks.add(i as usize) + HL_BLOCK_GLBL_SEC_OFFS; for j in 0..HL_BLOCK_GLBL_SEC_LEN { WREG32(base + j * core::mem::size_of::<u32>() as u32, (*a.add(i as usize)).sec_array[j as usize]); } }
}

pub unsafe fn hl_secure_block(_hdev: *mut hl_device, a: *mut hl_block_glbl_sec, n: i32) {
    for i in 0..n { core::ptr::write_bytes((*a.add(i as usize)).sec_array.as_mut_ptr() as *mut u8, 0, HL_BLOCK_GLBL_SEC_SIZE as usize); }
}

pub unsafe fn hl_init_pb_with_mask(hdev: *mut hl_device, nd: u32, doff: u32, ni: u32, ioff: u32, pb: *const u32, bs: u32, regs: *const u32, rn: u32, mask: u64) -> i32 {
    let glbl = kzalloc_objs::<hl_block_glbl_sec>(bs); if glbl.is_null() { return -ENOMEM; }
    hl_secure_block(hdev, glbl, bs as i32); hl_unsecure_registers(hdev, regs, rn as i32, 0, pb, glbl, bs as i32);
    for i in 0..nd { for j in 0..ni { let seq = i * ni + j; if (mask & BIT_ULL(seq)) == 0 { continue; } hl_config_glbl_sec(hdev, pb, glbl, i * doff + j * ioff, bs as i32); } }
    kfree(glbl); 0
}

pub unsafe fn hl_init_pb(hdev: *mut hl_device, nd: u32, doff: u32, ni: u32, ioff: u32, pb: *const u32, bs: u32, regs: *const u32, rn: u32) -> i32 { hl_init_pb_with_mask(hdev, nd, doff, ni, ioff, pb, bs, regs, rn, U64_MAX) }

// The remaining range, single-dcore, acknowledgement, and special-block
// entry points retain the C control flow and call the corresponding helpers.
pub unsafe fn hl_init_pb_ranges_with_mask(hdev: *mut hl_device, nd: u32, doff: u32, ni: u32, ioff: u32, pb: *const u32, bs: u32, rr: *const range, rn: u32, mask: u64) -> i32 {
    let glbl = kzalloc_objs::<hl_block_glbl_sec>(bs); if glbl.is_null() { return -ENOMEM; }
    hl_secure_block(hdev, glbl, bs as i32); let rc = hl_unsecure_registers_range(hdev, rr, rn as i32, 0, pb, glbl, bs as i32);
    if rc == 0 { for i in 0..nd { for j in 0..ni { let seq=i*ni+j; if (mask & BIT_ULL(seq)) != 0 { hl_config_glbl_sec(hdev,pb,glbl,i*doff+j*ioff,bs as i32); } } } }
    kfree(glbl); rc
}

pub unsafe fn hl_init_pb_ranges(hdev:*mut hl_device,nd:u32,doff:u32,ni:u32,ioff:u32,pb:*const u32,bs:u32,rr:*const range,rn:u32)->i32 { hl_init_pb_ranges_with_mask(hdev,nd,doff,ni,ioff,pb,bs,rr,rn,U64_MAX) }

pub unsafe fn hl_ack_pb_with_mask(hdev:*mut hl_device,nd:u32,doff:u32,ni:u32,ioff:u32,pb:*const u32,bs:u32,mask:u64) { for i in 0..nd { for j in 0..ni { let seq=i*ni+j; if (mask&BIT_ULL(seq))!=0 { hl_ack_pb_security_violations(hdev,pb,i*doff+j*ioff,bs as i32); } } } }
pub unsafe fn hl_ack_pb(hdev:*mut hl_device,nd:u32,doff:u32,ni:u32,ioff:u32,pb:*const u32,bs:u32) { hl_ack_pb_with_mask(hdev,nd,doff,ni,ioff,pb,bs,U64_MAX); }
pub unsafe fn hl_ack_pb_single_dcore(hdev:*mut hl_device,off:u32,ni:u32,ioff:u32,pb:*const u32,bs:u32) { for i in 0..ni { hl_ack_pb_security_violations(hdev,pb,off+i*ioff,bs as i32); } }

pub unsafe fn hl_init_pb_single_dcore(hdev:*mut hl_device,off:u32,ni:u32,ioff:u32,pb:*const u32,bs:u32,regs:*const u32,rn:u32)->i32 {
    let glbl=kzalloc_objs::<hl_block_glbl_sec>(bs); if glbl.is_null(){return -ENOMEM;} hl_secure_block(hdev,glbl,bs as i32); let rc=hl_unsecure_registers(hdev,regs,rn as i32,0,pb,glbl,bs as i32); if rc==0 { for i in 0..ni { hl_config_glbl_sec(hdev,pb,glbl,off+i*ioff,bs as i32); } } kfree(glbl); rc
}
pub unsafe fn hl_init_pb_ranges_single_dcore(hdev:*mut hl_device,off:u32,ni:u32,ioff:u32,pb:*const u32,bs:u32,rr:*const range,rn:u32)->i32 {
    let glbl=kzalloc_objs::<hl_block_glbl_sec>(bs); if glbl.is_null(){return -ENOMEM;} hl_secure_block(hdev,glbl,bs as i32); hl_unsecure_registers_range(hdev,rr,rn as i32,0,pb,glbl,bs as i32); for i in 0..ni { hl_config_glbl_sec(hdev,pb,glbl,off+i*ioff,bs as i32); } kfree(glbl); 0
}

unsafe fn hl_automated_get_block_base_addr(hdev:*mut hl_device, bi:*mut hl_special_block_info, major:u32, minor:u32, sub:u32)->u32 { (*bi).base_addr + major*(*bi).major_offset + minor*(*bi).minor_offset + sub*(*bi).sub_minor_offset - lower_32_bits((*hdev).asic_prop.cfg_base_address) }
unsafe fn hl_check_block_type_exclusion(cfg:*mut hl_skip_blocks_cfg, typ:i32)->bool { for i in 0..(*cfg).block_types_len { if typ==*(*cfg).block_types.add(i as usize){return true;} } false }
unsafe fn hl_check_block_range_exclusion(hdev:*mut hl_device,cfg:*mut hl_skip_blocks_cfg,bi:*mut hl_special_block_info,major:u32,minor:u32,sub:u32)->bool { let addr=hl_automated_get_block_base_addr(hdev,bi,major,minor,sub); for i in 0..(*cfg).block_ranges_len { let r=*(*cfg).block_ranges.add(i as usize); let n=(r.end-r.start)/HL_BLOCK_SIZE+1; for j in 0..n { if addr==r.start+j*HL_BLOCK_SIZE{return true;} } } false }

pub unsafe fn hl_iterate_special_blocks(hdev:*mut hl_device,ctx:*mut iterate_special_ctx)->i32 {
    let cfg=(*ctx).data as *mut hl_special_blocks_cfg; let skip=(*cfg).skip_blocks_cfg; let mut arr=(*hdev).asic_prop.special_blocks; if arr.is_null(){return -EINVAL;} let n=(*hdev).asic_prop.num_of_special_blocks;
    for b in 0..n { let bi=arr.add(b as usize); if hl_check_block_type_exclusion(skip,(*bi).block_type){continue;} for major in 0..(*bi).major { let mut minor=0; while minor<(*bi).minor { let mut sub=0; while sub<(*bi).sub_minor { if !hl_check_block_range_exclusion(hdev,skip,bi,major,minor,sub) { let rc=((*ctx).fn)(hdev,b,major,minor,sub,(*ctx).data); if rc!=0{return rc;} } sub+=1; } minor+=1; } } } 0
}

pub unsafe fn hl_check_for_glbl_errors(hdev:*mut hl_device) { let mut cfg=hl_special_blocks_cfg{skip_blocks_cfg:&mut (*hdev).asic_prop.skip_special_blocks_cfg}; let mut ctx=iterate_special_ctx{fn:hl_read_glbl_errors,data:&mut cfg as *mut _ as *mut core::ffi::c_void}; let rc=hl_iterate_special_blocks(hdev,&mut ctx); if rc!=0 { dev_err_ratelimited((*hdev).dev,"Could not iterate special blocks, glbl error check failed\n"); } }
unsafe fn hl_read_glbl_errors(_hdev:*mut hl_device,_b:u32,_a:u32,_m:u32,_s:u32,_d:*mut core::ffi::c_void)->i32 { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
