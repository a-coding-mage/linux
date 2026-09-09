// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (C) 2021 Marvell. */

// Dependencies supplied by the corresponding kernel headers and translation units.

unsafe extern "C" {
    fn otx2_cpt_send_cmd(cptinst: *mut otx2_cpt_inst_s, insts_num: u32,
                         lf: *mut otx2_cptlf_info);
    fn otx2_cpt_get_compcode() -> u32;
    fn otx2_cpt_get_uc_compcode() -> u32;
    fn otx2_sg_info_create() -> u32;
    fn cn10k_cpt_get_compcode() -> u32;
    fn cn10k_cpt_get_uc_compcode() -> u32;
    fn cn10k_lmt_flush(val: u64, tar_addr: u64);
    fn dma_wmb();
    fn dma_free_attrs(dev: *mut device, size: usize, cpu_addr: *mut core::ffi::c_void,
                      dma_addr: u64, attrs: u64);
    fn dma_alloc_attrs(dev: *mut device, size: usize, dma_handle: *mut u64,
                       flags: u32, attrs: u64) -> *mut core::ffi::c_void;
    fn otx2_cpt_lmtst_tbl_setup_msg(lfs: *mut otx2_cptlfs_info) -> i32;
    fn dev_err(dev: *mut device, fmt: *const core::ffi::c_char, ...);
    fn is_dev_cn10ka_ax(pdev: *mut pci_dev) -> bool;
    fn cn10k_cpt_ctx_flush(pdev: *mut pci_dev, cptr: u64, inval: bool);
    fn dma_unmap_single(dev: *mut device, dma_addr: u64, size: usize, direction: u32);
    fn kfree(ptr: *mut core::ffi::c_void);
    fn kmalloc(size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn dma_map_single(dev: *mut device, ptr: *mut core::ffi::c_void,
                      size: usize, direction: u32) -> u64;
    fn dma_mapping_error(dev: *mut device, dma_addr: u64) -> bool;
    fn otx2_cpt_write64(reg_base: *mut core::ffi::c_void, blkaddr: u32, slot: u32,
                        offset: u32, value: u64);
    fn otx2_cpt_read64(reg_base: *mut core::ffi::c_void, blkaddr: u32, slot: u32,
                       offset: u32) -> u64;
    fn wmb();
    fn pci_get_drvdata(pdev: *mut pci_dev) -> *mut otx2_cptvf_dev;
    fn test_bit(bit: u32, addr: *const u64) -> bool;
}

#[repr(C)]
pub struct cpt_hw_ops {
    pub send_cmd: unsafe extern "C" fn(*mut otx2_cpt_inst_s, u32, *mut otx2_cptlf_info),
    pub cpt_get_compcode: unsafe extern "C" fn() -> u32,
    pub cpt_get_uc_compcode: unsafe extern "C" fn() -> u32,
    pub cpt_sg_info_create: unsafe extern "C" fn() -> u32,
}

static mut otx2_hw_ops: cpt_hw_ops = cpt_hw_ops {
    send_cmd: otx2_cpt_send_cmd,
    cpt_get_compcode: otx2_cpt_get_compcode,
    cpt_get_uc_compcode: otx2_cpt_get_uc_compcode,
    cpt_sg_info_create: otx2_sg_info_create,
};

static mut cn10k_hw_ops: cpt_hw_ops = cpt_hw_ops {
    send_cmd: cn10k_cpt_send_cmd,
    cpt_get_compcode: cn10k_cpt_get_compcode,
    cpt_get_uc_compcode: cn10k_cpt_get_uc_compcode,
    cpt_sg_info_create: otx2_sg_info_create,
};

unsafe extern "C" fn cn10k_cpt_send_cmd(cptinst: *mut otx2_cpt_inst_s, insts_num: u32,
                                         lf: *mut otx2_cptlf_info) {
    let lmtline = (*(*lf).lfs).lmt_info.base.add(((*lf).slot as usize) * LMTLINE_SIZE);
    let val = ((*lf).slot as u64) & 0x7ff;
    let mut tar_addr: u64 = 0;

    // tar_addr<6:4> = Size of first LMTST - 1 in units of 128b.
    tar_addr |= (*lf).ioreg as u64 |
        ((((OTX2_CPT_INST_SIZE / 16) - 1) & 0x7) as u64) << 4;
    // Make sure memory areas pointed in CPT_INST_S are flushed before the instruction is sent to CPT.
    dma_wmb();
    core::ptr::copy_nonoverlapping(cptinst as *const u8, lmtline as *mut u8,
                                   (insts_num as usize) * OTX2_CPT_INST_SIZE);
    cn10k_lmt_flush(val, tar_addr);
}

pub unsafe extern "C" fn cn10k_cpt_lmtst_free(pdev: *mut pci_dev, lfs: *mut otx2_cptlfs_info) {
    let lmt_info = &mut (*lfs).lmt_info;
    if lmt_info.base.is_null() { return; }
    dma_free_attrs(&mut (*pdev).dev, lmt_info.size as usize,
                   lmt_info.base.sub(lmt_info.align as usize) as *mut _,
                   lmt_info.iova - lmt_info.align, DMA_ATTR_FORCE_CONTIGUOUS);
}

unsafe fn cn10k_cpt_lmtst_alloc(pdev: *mut pci_dev, lfs: *mut otx2_cptlfs_info, size: u32) -> i32 {
    let lmt_info = &mut (*lfs).lmt_info;
    let mut iova = 0u64;
    lmt_info.base = dma_alloc_attrs(&mut (*pdev).dev, size as usize, &mut iova,
                                    GFP_KERNEL, DMA_ATTR_FORCE_CONTIGUOUS) as *mut u8;
    if lmt_info.base.is_null() { return -ENOMEM; }
    let align_iova = (iova + (LMTLINE_ALIGN as u64 - 1)) & !(LMTLINE_ALIGN as u64 - 1);
    lmt_info.iova = align_iova;
    lmt_info.align = align_iova - iova;
    lmt_info.size = size;
    lmt_info.base = lmt_info.base.add(lmt_info.align as usize);
    0
}

pub unsafe extern "C" fn cn10k_cptpf_lmtst_init(cptpf: *mut otx2_cptpf_dev) -> i32 {
    let pdev = (*cptpf).pdev;
    if !test_bit(CN10K_LMTST, &(*cptpf).cap_flag) {
        (*cptpf).lfs.ops = &mut otx2_hw_ops;
        return 0;
    }
    (*cptpf).lfs.ops = &mut cn10k_hw_ops;
    let size = (OTX2_CPT_MAX_VFS_NUM * LMTLINE_SIZE + LMTLINE_ALIGN) as u32;
    let ret = cn10k_cpt_lmtst_alloc(pdev, &mut (*cptpf).lfs, size);
    if ret != 0 { return ret; }
    let ret = otx2_cpt_lmtst_tbl_setup_msg(&mut (*cptpf).lfs);
    if ret != 0 { cn10k_cpt_lmtst_free(pdev, &mut (*cptpf).lfs); }
    0
}

pub unsafe extern "C" fn cn10k_cptvf_lmtst_init(cptvf: *mut otx2_cptvf_dev) -> i32 {
    let pdev = (*cptvf).pdev;
    if !test_bit(CN10K_LMTST, &(*cptvf).cap_flag) { return 0; }
    let size = ((*cptvf).lfs.lfs_num * LMTLINE_SIZE + LMTLINE_ALIGN) as u32;
    let ret = cn10k_cpt_lmtst_alloc(pdev, &mut (*cptvf).lfs, size);
    if ret != 0 { return ret; }
    let ret = otx2_cpt_lmtst_tbl_setup_msg(&mut (*cptvf).lfs);
    if ret != 0 { cn10k_cpt_lmtst_free(pdev, &mut (*cptvf).lfs); }
    0
}

pub unsafe extern "C" fn cn10k_cpt_hw_ctx_clear(pdev: *mut pci_dev, er_ctx: *mut cn10k_cpt_errata_ctx) {
    if !is_dev_cn10ka_ax(pdev) { return; }
    let cptr_dma = (*er_ctx).cptr_dma & !BIT_ULL(60);
    cn10k_cpt_ctx_flush(pdev, cptr_dma, true);
    dma_unmap_single(&mut (*pdev).dev, cptr_dma, CN10K_CPT_HW_CTX_SIZE, DMA_BIDIRECTIONAL);
    kfree((*er_ctx).hw_ctx as *mut _);
}

pub unsafe extern "C" fn cn10k_cpt_hw_ctx_set(hctx: *mut cn10k_cpt_hw_ctx, ctx_sz: u16) {
    (*hctx).w0.aop_valid = 1;
    (*hctx).w0.ctx_hdr_sz = 0;
    (*hctx).w0.ctx_sz = ctx_sz;
    (*hctx).w0.ctx_push_sz = 1;
}

pub unsafe extern "C" fn cn10k_cpt_hw_ctx_init(pdev: *mut pci_dev, er_ctx: *mut cn10k_cpt_errata_ctx) -> i32 {
    (*er_ctx).cptr_dma = 0;
    (*er_ctx).hw_ctx = core::ptr::null_mut();
    if !is_dev_cn10ka_ax(pdev) { return 0; }
    let hctx = kmalloc(CN10K_CPT_HW_CTX_SIZE, GFP_KERNEL) as *mut cn10k_cpt_hw_ctx;
    if hctx.is_null() { return -ENOMEM; }
    let cptr_dma = dma_map_single(&mut (*pdev).dev, hctx as *mut _, CN10K_CPT_HW_CTX_SIZE, DMA_BIDIRECTIONAL);
    if dma_mapping_error(&mut (*pdev).dev, cptr_dma) { kfree(hctx as *mut _); return -ENOMEM; }
    cn10k_cpt_hw_ctx_set(hctx, 1);
    (*er_ctx).hw_ctx = hctx;
    (*er_ctx).cptr_dma = cptr_dma | BIT_ULL(60);
    0
}

pub unsafe extern "C" fn cn10k_cpt_ctx_flush(pdev: *mut pci_dev, cptr: u64, inval: bool) {
    let cptvf = pci_get_drvdata(pdev);
    let lfs = &mut (*cptvf).lfs;
    let mut reg = cptr as usize as u64 >> 7;
    if inval { reg |= BIT_ULL(46); }
    otx2_cpt_write64(lfs.reg_base, lfs.blkaddr, lfs.lf[0].slot, OTX2_CPT_LF_CTX_FLUSH, reg);
    wmb();
    otx2_cpt_read64(lfs.reg_base, lfs.blkaddr, lfs.lf[0].slot, OTX2_CPT_LF_CTX_ERR);
}

pub unsafe extern "C" fn cptvf_hw_ops_get(cptvf: *mut otx2_cptvf_dev) {
    if test_bit(CN10K_LMTST, &(*cptvf).cap_flag) {
        (*cptvf).lfs.ops = &mut cn10k_hw_ops;
    } else {
        (*cptvf).lfs.ops = &mut otx2_hw_ops;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
