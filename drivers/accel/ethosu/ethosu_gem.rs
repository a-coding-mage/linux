// SPDX-License-Identifier: GPL-2.0-only or MIT
/* Copyright 2025 Arm, Ltd. */

// Kernel and driver declarations supplied by the surrounding repository.

unsafe fn ethosu_gem_free_object(obj: *mut drm_gem_object) {
    let bo = to_ethosu_bo(obj);
    kfree((*bo).info);
    drm_gem_free_mmap_offset(&mut (*bo).base.base);
    drm_gem_dma_free(&mut (*bo).base);
}

unsafe fn ethosu_gem_mmap(obj: *mut drm_gem_object, vma: *mut vm_area_struct) -> c_int {
    let bo = to_ethosu_bo(obj);
    if (*bo).flags & DRM_ETHOSU_BO_NO_MMAP != 0 { return -EINVAL; }
    drm_gem_dma_object_mmap(obj, vma)
}

static ethosu_gem_funcs: drm_gem_object_funcs = drm_gem_object_funcs {
    free: Some(ethosu_gem_free_object),
    print_info: Some(drm_gem_dma_object_print_info),
    get_sg_table: Some(drm_gem_dma_object_get_sg_table),
    vmap: Some(drm_gem_dma_object_vmap),
    mmap: Some(ethosu_gem_mmap),
    vm_ops: unsafe { &drm_gem_dma_vm_ops },
};

pub unsafe fn ethosu_gem_create_object(ddev: *mut drm_device, size: usize) -> *mut drm_gem_object {
    let obj: *mut ethosu_gem_object = kzalloc_obj();
    if obj.is_null() { return ERR_PTR(-ENOMEM); }
    (*obj).base.base.funcs = &ethosu_gem_funcs;
    &mut (*obj).base.base
}

pub unsafe fn ethosu_gem_create_with_handle(file: *mut drm_file, ddev: *mut drm_device,
                                             size: *mut u64, flags: u32,
                                             handle: *mut u32) -> c_int {
    let mem = drm_gem_dma_create(ddev, *size);
    if IS_ERR(mem) { return PTR_ERR(mem); }
    let bo = to_ethosu_bo(&mut (*mem).base);
    (*bo).flags = flags;
    let ret = drm_gem_handle_create(file, &mut (*mem).base, handle);
    if ret == 0 { *size = (*bo).base.base.size; }
    drm_gem_object_put(&mut (*mem).base);
    ret
}

#[repr(C)] struct dma { region: i8, len: u64, offset: u64, stride: [i64; 2] }
#[repr(C)] struct dma_state { size0: u16, size1: u16, mode: i8, src: dma, dst: dma }
#[repr(C)] struct buffer { base: u64, length: u32, region: i8 }
#[repr(C)] struct feat_matrix {
    base: [u64; 4], stride_x: i64, stride_y: i64, stride_c: i64, region: i8,
    broadcast: u8, stride_kernel: u16, precision: u16, depth: u16, width: u16,
    width0: u16, height: [u16; 3], pad_top: u8, pad_left: u8, pad_bottom: u8, pad_right: u8,
}
#[repr(C)] struct cmd_state {
    dma: dma_state, scale: [buffer; 2], weight: [buffer; 4], ofm: feat_matrix,
    ifm: feat_matrix, ifm2: feat_matrix,
}

unsafe fn cmd_state_init(st: *mut cmd_state) { memset(st as *mut c_void, 0xff, core::mem::size_of::<cmd_state>()); }
unsafe fn cmd_to_addr(cmd: *mut u32) -> u64 { (((*cmd as u64) & 0xff0000) << 16) | *cmd.add(1) as u64 }

unsafe fn dma_length(info: *mut ethosu_validated_cmdstream_info, ds: *mut dma_state, d: *mut dma) -> u64 {
    let mode = (*ds).mode; let mut len = (*d).len;
    if len == U64_MAX { return U64_MAX; }
    if mode >= 1 {
        if (*d).stride[0] < 0 && (-(*d).stride[0]) as u64 > len { return U64_MAX; }
        len = len.wrapping_add((*d).stride[0] as u64);
        if (*ds).size0 != 0 && len > u64::MAX / (*ds).size0 as u64 { return U64_MAX; }
        len = len.wrapping_mul((*ds).size0 as u64);
    }
    if mode == 2 {
        if (*d).stride[1] < 0 && (-(*d).stride[1]) as u64 > len { return U64_MAX; }
        len = len.wrapping_add((*d).stride[1] as u64);
        if (*ds).size1 != 0 && len > u64::MAX / (*ds).size1 as u64 { return U64_MAX; }
        len = len.wrapping_mul((*ds).size1 as u64);
    }
    if (*d).region >= 0 {
        let end = len.checked_add((*d).offset).unwrap_or(U64_MAX);
        let r = (*d).region as usize;
        (*info).region_size[r] = core::cmp::max((*info).region_size[r], end);
        if end == U64_MAX { return U64_MAX; }
    }
    len
}

unsafe fn feat_matrix_chained(edev: *mut ethosu_device, fm: *mut feat_matrix) -> bool {
    !ethosu_is_u65(edev) && ((*fm).precision >> 14) == 2
}

unsafe fn feat_matrix_length(edev: *mut ethosu_device, info: *mut ethosu_validated_cmdstream_info,
                             fm: *mut feat_matrix, mut x: u32, mut y: u32, c: u32, ofm: bool) -> u64 {
    let storage = (*fm).precision >> 14; let mut tile = 0usize;
    if (*fm).region < 0 { return U64_MAX; }
    if feat_matrix_chained(edev, fm) { return 0; }
    match storage {
        0 => { if x >= (*fm).width0 as u32 + 1 { x -= (*fm).width0 as u32 + 1; tile += 1; }
               if y >= (*fm).height[tile] as u32 + 1 { y -= (*fm).height[tile] as u32 + 1; tile += 2; } }
        1 => { if y >= (*fm).height[1] as u32 + 1 { y -= (*fm).height[1] as u32 + 1; tile = 2; }
               else if y >= (*fm).height[0] as u32 + 1 { y -= (*fm).height[0] as u32 + 1; tile = 1; } }
        _ => return U64_MAX,
    }
    if (*fm).base[tile] == U64_MAX { return U64_MAX; }
    let mut addr = (*fm).base[tile].wrapping_add((y as i64).wrapping_mul((*fm).stride_y) as u64);
    let es = 1u32 << (((*fm).precision >> if ofm { 1 } else { 2 }) & 3);
    match ((*fm).precision >> 6) & 3 {
        0 => { addr = addr.wrapping_add((x as i64).wrapping_mul((*fm).stride_x) as u64).wrapping_add(c as u64 * es as u64); }
        1 => { addr = addr.wrapping_add((c / 16) as i64 .wrapping_mul((*fm).stride_c) as u64)
                    .wrapping_add((16 * x + (c & 0xf)) as u64 * es as u64); }
        _ => {}
    }
    let r = (*fm).region as usize;
    (*info).region_size[r] = core::cmp::max((*info).region_size[r], addr.wrapping_add(1));
    addr
}

unsafe fn calc_sizes(ddev: *mut drm_device, info: *mut ethosu_validated_cmdstream_info, op: u16,
                     st: *mut cmd_state, ifm: bool, ifm2: bool, weight: bool, scale: bool) -> c_int {
    let edev = to_ethosu_device(ddev); let mut len;
    if ifm {
        if (*st).ifm.stride_kernel == U16_MAX { return -EINVAL; }
        let sy = (((*st).ifm.stride_kernel >> 8) & 2) + (((*st).ifm.stride_kernel >> 1) & 1) + 1;
        let sx = (((*st).ifm.stride_kernel >> 5) & 2) + ((*st).ifm.stride_kernel & 1) + 1;
        let h = (*st).ofm.height[2] as i32 * sy as i32 + (*st).ifm.height[2] as i32 -
                ((*st).ifm.pad_top + (*st).ifm.pad_bottom) as i32;
        let w = (*st).ofm.width as i32 * sx as i32 + (*st).ifm.width as i32 -
                ((*st).ifm.pad_left + (*st).ifm.pad_right) as i32;
        if h < 0 || w < 0 { return -EINVAL; }
        len = feat_matrix_length(edev, info, &mut (*st).ifm, w as u32, h as u32, (*st).ifm.depth as u32, false);
        if len == U64_MAX { return -EINVAL; }
    }
    if ifm2 { len = feat_matrix_length(edev, info, &mut (*st).ifm2, (*st).ifm.depth as u32, 0, (*st).ofm.depth as u32, false); if len == U64_MAX { return -EINVAL; } }
    if weight {
        if (*st).weight[0].region < 0 || (*st).weight[0].base == U64_MAX || (*st).weight[0].length == U32_MAX { return -EINVAL; }
        let r = (*st).weight[0].region as usize; (*info).region_size[r] = core::cmp::max((*info).region_size[r], (*st).weight[0].base.wrapping_add((*st).weight[0].length as u64));
    }
    if scale {
        if (*st).scale[0].region < 0 || (*st).scale[0].base == U64_MAX || (*st).scale[0].length == U32_MAX { return -EINVAL; }
        let r = (*st).scale[0].region as usize; (*info).region_size[r] = core::cmp::max((*info).region_size[r], (*st).scale[0].base.wrapping_add((*st).scale[0].length as u64));
    }
    len = feat_matrix_length(edev, info, &mut (*st).ofm, (*st).ofm.width as u32, (*st).ofm.height[2] as u32, (*st).ofm.depth as u32, true);
    if len == U64_MAX { return -EINVAL; }
    if !feat_matrix_chained(edev, &mut (*st).ofm) { (*info).output_region[(*st).ofm.region as usize] = true; }
    0
}

unsafe fn calc_sizes_elemwise(ddev: *mut drm_device, info: *mut ethosu_validated_cmdstream_info, _op: u16,
                              st: *mut cmd_state, ifm: bool, ifm2: bool) -> c_int {
    let edev = to_ethosu_device(ddev); let (ow, oh, od) = ((*st).ofm.width as u32, (*st).ofm.height[2] as u32, (*st).ofm.depth as u32);
    if ifm { let l = feat_matrix_length(edev, info, &mut (*st).ifm, if (*st).ifm.broadcast & 2 != 0 {0} else {ow}, if (*st).ifm.broadcast & 1 != 0 {0} else {oh}, if (*st).ifm.broadcast & 4 != 0 {0} else {od}, false); if l == U64_MAX { return -EINVAL; } }
    if ifm2 { let l = feat_matrix_length(edev, info, &mut (*st).ifm2, if (*st).ifm2.broadcast & 2 != 0 {0} else {ow}, if (*st).ifm2.broadcast & 1 != 0 {0} else {oh}, if (*st).ifm2.broadcast & 4 != 0 {0} else {od}, false); if l == U64_MAX { return -EINVAL; } }
    let l = feat_matrix_length(edev, info, &mut (*st).ofm, ow, oh, od, true); if l == U64_MAX { return -EINVAL; }
    if !feat_matrix_chained(edev, &mut (*st).ofm) { (*info).output_region[(*st).ofm.region as usize] = true; } 0
}

// Command validation retains the original command numbers and state-machine ordering.
pub unsafe fn ethosu_gem_cmdstream_copy_and_validate(ddev: *mut drm_device, ucmds: *mut u32,
    bo: *mut ethosu_gem_object, size: u32) -> c_int {
    let info = kzalloc_obj::<ethosu_validated_cmdstream_info>(); if info.is_null() { return -ENOMEM; }
    (*info).cmd_size = size; let edev = to_ethosu_device(ddev); let bocmds = (*bo).base.vaddr as *mut u32;
    let mut st: cmd_state = core::mem::zeroed(); cmd_state_init(&mut st); let mut i = 0u32;
    while i < size / 4 {
        let mut cmds = [0u32; 2]; if get_user(&mut cmds[0], ucmds.add(i as usize)) != 0 { return -EFAULT; }
        *bocmds.add(i as usize) = cmds[0]; let cmd = cmds[0] as u16; let param = (cmds[0] >> 16) as u16; let mut addr = 0;
        if cmd & 0x4000 != 0 { if get_user(&mut cmds[1], ucmds.add(i as usize + 1)) != 0 { return -EFAULT; } i += 1; if i >= size / 4 { return -EINVAL; } *bocmds.add(i as usize) = cmds[1]; addr = cmd_to_addr(cmds.as_mut_ptr()); }
        let ret = match cmd {
            NPU_OP_DMA_START => { let a=dma_length(info,&mut st.dma,&mut st.dma.src); let b=dma_length(info,&mut st.dma,&mut st.dma.dst); if a==U64_MAX||b==U64_MAX {-EINVAL} else { if st.dma.dst.region>=0 {(*info).output_region[st.dma.dst.region as usize]=true;} 0 } }
            NPU_OP_CONV | NPU_OP_DEPTHWISE => calc_sizes(ddev,info,cmd,&mut st,true,param&1!=0,!((param&1)!=0),(*st.ofm.precision & 0x100)==0),
            NPU_OP_POOL => calc_sizes(ddev,info,cmd,&mut st,param != 4,false,false,(*st.ofm.precision & 0x100)==0),
            NPU_OP_ELEMENTWISE => { let us=if ethosu_is_u65(edev){st.ifm2.broadcast&0x80!=0}else{st.ifm2.broadcast==8}; calc_sizes_elemwise(ddev,info,cmd,&mut st,st.ifm.broadcast!=8,!(us||param==5||param==6||param==7||param==0x24)) }
            NPU_OP_RESIZE => -EINVAL,
            NPU_SET_KERNEL_WIDTH_M1 => {st.ifm.width=param;0}, NPU_SET_KERNEL_HEIGHT_M1=>{st.ifm.height[2]=param;0}, NPU_SET_KERNEL_STRIDE=>{st.ifm.stride_kernel=param;0},
            NPU_SET_IFM_PAD_TOP=>{st.ifm.pad_top=(param&0x7f) as u8;0}, NPU_SET_IFM_PAD_LEFT=>{st.ifm.pad_left=(param&0x7f) as u8;0}, NPU_SET_IFM_PAD_RIGHT=>{st.ifm.pad_right=(param&0xff) as u8;0}, NPU_SET_IFM_PAD_BOTTOM=>{st.ifm.pad_bottom=(param&0xff) as u8;0}, NPU_SET_IFM_DEPTH_M1=>{st.ifm.depth=param;0}, NPU_SET_IFM_PRECISION=>{st.ifm.precision=param;0}, NPU_SET_IFM_BROADCAST=>{st.ifm.broadcast=param as u8;0}, NPU_SET_IFM_REGION=>{st.ifm.region=(param&7) as i8;0}, NPU_SET_IFM_WIDTH0_M1=>{st.ifm.width0=param;0}, NPU_SET_IFM_HEIGHT0_M1=>{st.ifm.height[0]=param;0}, NPU_SET_IFM_HEIGHT1_M1=>{st.ifm.height[1]=param;0},
            NPU_SET_IFM_BASE0|NPU_SET_IFM_BASE1|NPU_SET_IFM_BASE2|NPU_SET_IFM_BASE3 => {st.ifm.base[(cmd&3) as usize]=addr;0}, NPU_SET_IFM_STRIDE_X=>{st.ifm.stride_x=addr as i64;0}, NPU_SET_IFM_STRIDE_Y=>{st.ifm.stride_y=addr as i64;0}, NPU_SET_IFM_STRIDE_C=>{st.ifm.stride_c=addr as i64;0},
            NPU_SET_OFM_WIDTH_M1=>{st.ofm.width=param;0}, NPU_SET_OFM_HEIGHT_M1=>{st.ofm.height[2]=param;0}, NPU_SET_OFM_DEPTH_M1=>{st.ofm.depth=param;0}, NPU_SET_OFM_PRECISION=>{st.ofm.precision=param;0}, NPU_SET_OFM_REGION=>{st.ofm.region=(param&7) as i8;0}, NPU_SET_OFM_WIDTH0_M1=>{st.ofm.width0=param;0}, NPU_SET_OFM_HEIGHT0_M1=>{st.ofm.height[0]=param;0}, NPU_SET_OFM_HEIGHT1_M1=>{st.ofm.height[1]=param;0}, NPU_SET_OFM_BASE0|NPU_SET_OFM_BASE1|NPU_SET_OFM_BASE2|NPU_SET_OFM_BASE3=>{st.ofm.base[(cmd&3) as usize]=addr;0}, NPU_SET_OFM_STRIDE_X=>{st.ofm.stride_x=addr as i64;0}, NPU_SET_OFM_STRIDE_Y=>{st.ofm.stride_y=addr as i64;0}, NPU_SET_OFM_STRIDE_C=>{st.ofm.stride_c=addr as i64;0},
            NPU_SET_IFM2_BROADCAST=>{st.ifm2.broadcast=param as u8;0}, NPU_SET_IFM2_PRECISION=>{st.ifm2.precision=param;0}, NPU_SET_IFM2_REGION=>{st.ifm2.region=(param&7) as i8;0}, NPU_SET_IFM2_WIDTH0_M1=>{st.ifm2.width0=param;0}, NPU_SET_IFM2_HEIGHT0_M1=>{st.ifm2.height[0]=param;0}, NPU_SET_IFM2_HEIGHT1_M1=>{st.ifm2.height[1]=param;0}, NPU_SET_IFM2_BASE0|NPU_SET_IFM2_BASE1|NPU_SET_IFM2_BASE2|NPU_SET_IFM2_BASE3=>{st.ifm2.base[(cmd&3) as usize]=addr;0}, NPU_SET_IFM2_STRIDE_X=>{st.ifm2.stride_x=addr as i64;0}, NPU_SET_IFM2_STRIDE_Y=>{st.ifm2.stride_y=addr as i64;0}, NPU_SET_IFM2_STRIDE_C=>{st.ifm2.stride_c=addr as i64;0},
            NPU_SET_WEIGHT_REGION=>{st.weight[0].region=(param&7) as i8;0}, NPU_SET_SCALE_REGION=>{st.scale[0].region=(param&7) as i8;0}, NPU_SET_WEIGHT_BASE=>{st.weight[0].base=addr;0}, NPU_SET_WEIGHT_LENGTH=>{st.weight[0].length=cmds[1];0}, NPU_SET_SCALE_BASE=>{st.scale[0].base=addr;0}, NPU_SET_SCALE_LENGTH=>{st.scale[0].length=cmds[1];0}, NPU_SET_WEIGHT1_BASE=>{st.weight[1].base=addr;0}, NPU_SET_WEIGHT1_LENGTH=>{st.weight[1].length=cmds[1];0}, NPU_SET_SCALE1_BASE=>{if ethosu_is_u65(edev){st.scale[1].base=addr}else{st.weight[2].base=addr};0}, NPU_SET_SCALE1_LENGTH=>{if ethosu_is_u65(edev){st.scale[1].length=cmds[1]}else{st.weight[2].length=cmds[1]};0}, NPU_SET_WEIGHT3_BASE=>{st.weight[3].base=addr;0}, NPU_SET_WEIGHT3_LENGTH=>{st.weight[3].length=cmds[1];0},
            NPU_SET_DMA0_SRC_REGION=>{st.dma.src.region=if param&0x100!=0{-1}else{(param&7) as i8};st.dma.mode=((param>>9)&3) as i8;0}, NPU_SET_DMA0_DST_REGION=>{st.dma.dst.region=if param&0x100!=0{-1}else{(param&7) as i8};0}, NPU_SET_DMA0_SIZE0=>{st.dma.size0=param;0}, NPU_SET_DMA0_SIZE1=>{st.dma.size1=param;0}, NPU_SET_DMA0_SRC_STRIDE0=>{st.dma.src.stride[0]=(addr as i8) as i64;0}, NPU_SET_DMA0_SRC_STRIDE1=>{st.dma.src.stride[1]=(addr as i8) as i64;0}, NPU_SET_DMA0_DST_STRIDE0=>{st.dma.dst.stride[0]=(addr as i8) as i64;0}, NPU_SET_DMA0_DST_STRIDE1=>{st.dma.dst.stride[1]=(addr as i8) as i64;0}, NPU_SET_DMA0_SRC=>{st.dma.src.offset=addr;0}, NPU_SET_DMA0_DST=>{st.dma.dst.offset=addr;0}, NPU_SET_DMA0_LEN=>{st.dma.src.len=addr;st.dma.dst.len=addr;0},
            _ => 0,
        }; if ret != 0 { return ret; } i += 1;
    }
    (*bo).info = info; 0
}

pub unsafe fn ethosu_gem_cmdstream_create(file:*mut drm_file,ddev:*mut drm_device,size:u32,data:u64,flags:u32,handle:*mut u32)->c_int {
    let mem=drm_gem_dma_create(ddev,size); if IS_ERR(mem){return PTR_ERR(mem);} let bo=to_ethosu_bo(&mut (*mem).base); (*bo).flags=flags;
    let ret=ethosu_gem_cmdstream_copy_and_validate(ddev,data as *mut u32,bo,size); if ret==0 { let r=drm_gem_handle_create(file,&mut (*mem).base,handle); drm_gem_object_put(&mut (*mem).base); return r; } drm_gem_object_put(&mut (*mem).base); ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
