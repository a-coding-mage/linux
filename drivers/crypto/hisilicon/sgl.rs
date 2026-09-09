// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2019 HiSilicon Limited. */

const HISI_ACC_SGL_SGE_NR_MIN: u32 = 1;
const HISI_ACC_SGL_NR_MAX: u32 = 256;
const HISI_ACC_SGL_ALIGN_SIZE: usize = 64;
const HISI_ACC_MEM_BLOCK_NR: usize = 5;

#[repr(C)]
pub struct acc_hw_sge {
    pub buf: dma_addr_t,
    pub page_ctrl: *mut core::ffi::c_void,
    pub len: __le32,
    pub pad: __le32,
    pub pad0: __le32,
    pub pad1: __le32,
}

/* use default sgl head size 64B */
#[repr(C, align(1))]
pub struct hisi_acc_hw_sgl {
    pub next_dma: dma_addr_t,
    pub entry_sum_in_chain: __le16,
    pub entry_sum_in_sgl: __le16,
    pub entry_length_in_sgl: __le16,
    pub pad0: __le16,
    pub pad1: [__le64; 5],
    pub next: *mut hisi_acc_hw_sgl,
    pub sge_entries: [acc_hw_sge; 0],
}

#[repr(C)]
pub struct hisi_acc_sgl_pool {
    pub mem_block: [mem_block; HISI_ACC_MEM_BLOCK_NR],
    pub sgl_num_per_block: u32,
    pub block_num: u32,
    pub count: u32,
    pub sge_nr: u32,
    pub sgl_size: usize,
}

#[repr(C)]
pub struct mem_block {
    pub sgl: *mut hisi_acc_hw_sgl,
    pub sgl_dma: dma_addr_t,
    pub size: usize,
}

/**
 * hisi_acc_create_sgl_pool() - Create a hw sgl pool.
 * @dev: The device which hw sgl pool belongs to.
 * @count: Count of hisi_acc_hw_sgl in pool.
 * @sge_nr: The count of sge in hw_sgl
 *
 * This function creates a hw sgl pool, after this user can get hw sgl memory
 * from it.
 */
pub unsafe fn hisi_acc_create_sgl_pool(
    dev: *mut device,
    count: u32,
    sge_nr: u32,
) -> *mut hisi_acc_sgl_pool {
    let mut sgl_size: u32;
    let mut block_size: u32;
    let mut sgl_num_per_block: u32;
    let mut block_num: u32;
    let mut remain_sgl: u32;
    let pool: *mut hisi_acc_sgl_pool;
    let block: *mut mem_block;
    let mut i: u32;
    let mut j: u32;

    if dev.is_null() || count == 0 || sge_nr == 0 || sge_nr > HISI_ACC_SGL_NR_MAX {
        return ERR_PTR(-EINVAL);
    }

    sgl_size = align(
        (core::mem::size_of::<acc_hw_sge>() as u32) * sge_nr
            + core::mem::size_of::<hisi_acc_hw_sgl>() as u32,
        HISI_ACC_SGL_ALIGN_SIZE as u32,
    );

    /*
     * the pool may allocate a block of memory of size PAGE_SIZE * 2^MAX_PAGE_ORDER,
     * block size may exceed 2^31 on ia64, so the max of block size is 2^31
     */
    block_size = 1u32 << if PAGE_SHIFT + MAX_PAGE_ORDER < 32 {
        PAGE_SHIFT + MAX_PAGE_ORDER
    } else {
        31
    };
    sgl_num_per_block = block_size / sgl_size;
    block_num = count / sgl_num_per_block;
    remain_sgl = count % sgl_num_per_block;

    if (remain_sgl == 0 && block_num > HISI_ACC_MEM_BLOCK_NR as u32)
        || (remain_sgl > 0 && block_num > (HISI_ACC_MEM_BLOCK_NR - 1) as u32)
    {
        return ERR_PTR(-EINVAL);
    }

    pool = kzalloc_obj::<hisi_acc_sgl_pool>();
    if pool.is_null() {
        return ERR_PTR(-ENOMEM);
    }
    block = (*pool).mem_block.as_mut_ptr();

    i = 0;
    while i < block_num {
        (*block.add(i as usize)).sgl = dma_alloc_coherent(
            dev,
            block_size as usize,
            &mut (*block.add(i as usize)).sgl_dma,
            GFP_KERNEL,
        ) as *mut hisi_acc_hw_sgl;
        if (*block.add(i as usize)).sgl.is_null() {
            dev_err(dev, "Fail to allocate hw SG buffer!\n");
            let mut j = 0;
            while j < i {
                dma_free_coherent(
                    dev,
                    block_size as usize,
                    (*block.add(j as usize)).sgl as *mut core::ffi::c_void,
                    (*block.add(j as usize)).sgl_dma,
                );
                j += 1;
            }
            kfree_sensitive(pool as *mut core::ffi::c_void);
            return ERR_PTR(-ENOMEM);
        }

        (*block.add(i as usize)).size = block_size as usize;
        i += 1;
    }

    if remain_sgl > 0 {
        (*block.add(i as usize)).sgl = dma_alloc_coherent(
            dev,
            (remain_sgl * sgl_size) as usize,
            &mut (*block.add(i as usize)).sgl_dma,
            GFP_KERNEL,
        ) as *mut hisi_acc_hw_sgl;
        if (*block.add(i as usize)).sgl.is_null() {
            dev_err(dev, "Fail to allocate remained hw SG buffer!\n");
            let mut j = 0;
            while j < i {
                dma_free_coherent(
                    dev,
                    block_size as usize,
                    (*block.add(j as usize)).sgl as *mut core::ffi::c_void,
                    (*block.add(j as usize)).sgl_dma,
                );
                j += 1;
            }
            kfree_sensitive(pool as *mut core::ffi::c_void);
            return ERR_PTR(-ENOMEM);
        }

        (*block.add(i as usize)).size = (remain_sgl * sgl_size) as usize;
    }

    (*pool).sgl_num_per_block = sgl_num_per_block;
    (*pool).block_num = if remain_sgl != 0 { block_num + 1 } else { block_num };
    (*pool).count = count;
    (*pool).sgl_size = sgl_size as usize;
    (*pool).sge_nr = sge_nr;

    pool
}

pub unsafe fn hisi_acc_free_sgl_pool(dev: *mut device, pool: *mut hisi_acc_sgl_pool) {
    if dev.is_null() || pool.is_null() {
        return;
    }

    let block = (*pool).mem_block.as_mut_ptr();
    let mut i = 0;
    while i < (*pool).block_num {
        dma_free_coherent(
            dev,
            (*block.add(i as usize)).size,
            (*block.add(i as usize)).sgl as *mut core::ffi::c_void,
            (*block.add(i as usize)).sgl_dma,
        );
        i += 1;
    }

    kfree(pool as *mut core::ffi::c_void);
}

unsafe fn acc_get_sgl(
    pool: *mut hisi_acc_sgl_pool,
    index: u32,
    hw_sgl_dma: *mut dma_addr_t,
) -> *mut hisi_acc_hw_sgl {
    let block = (*pool).mem_block.as_mut_ptr();
    let block_index = index / (*pool).sgl_num_per_block;
    let offset = index % (*pool).sgl_num_per_block;

    *hw_sgl_dma = (*block.add(block_index as usize)).sgl_dma
        + ((*pool).sgl_size * offset as usize) as dma_addr_t;
    ((*block.add(block_index as usize)).sgl as *mut u8)
        .add((*pool).sgl_size * offset as usize) as *mut hisi_acc_hw_sgl
}

unsafe fn sg_map_to_hw_sg(sgl: *mut scatterlist, hw_sge: *mut acc_hw_sge) {
    (*hw_sge).buf = sg_dma_address(sgl);
    (*hw_sge).len = cpu_to_le32(sg_dma_len(sgl));
    (*hw_sge).page_ctrl = sg_virt(sgl);
}

unsafe fn inc_hw_sgl_sge(hw_sgl: *mut hisi_acc_hw_sgl) {
    let mut var = le16_to_cpu((*hw_sgl).entry_sum_in_sgl);
    var += 1;
    (*hw_sgl).entry_sum_in_sgl = cpu_to_le16(var);
}

unsafe fn update_hw_sgl_sum_sge(hw_sgl: *mut hisi_acc_hw_sgl, sum: u16) {
    (*hw_sgl).entry_sum_in_chain = cpu_to_le16(sum);
}

unsafe fn clear_hw_sgl_sge(hw_sgl: *mut hisi_acc_hw_sgl) {
    let hw_sge = (*hw_sgl).sge_entries.as_mut_ptr();
    let entry_sum = le16_to_cpu((*hw_sgl).entry_sum_in_sgl);
    let mut i = 0;
    while i < entry_sum as usize {
        (*hw_sge.add(i)).page_ctrl = core::ptr::null_mut();
        (*hw_sge.add(i)).buf = 0;
        (*hw_sge.add(i)).len = 0;
        i += 1;
    }
}

/** Map a scatterlist to a hw sgl. */
pub unsafe fn hisi_acc_sg_buf_map_to_hw_sgl(
    dev: *mut device,
    sgl: *mut scatterlist,
    pool: *mut hisi_acc_sgl_pool,
    index: u32,
    hw_sgl_dma: *mut dma_addr_t,
    dir: dma_data_direction,
) -> *mut hisi_acc_hw_sgl {
    if dev.is_null() || sgl.is_null() || pool.is_null() || hw_sgl_dma.is_null()
        || index >= (*pool).count
    {
        return ERR_PTR(-EINVAL);
    }

    let sg_n = sg_nents(sgl);
    let sg_n_mapped = dma_map_sg(dev, sgl, sg_n, dir);
    if sg_n_mapped == 0 {
        dev_err(dev, "DMA mapping for SG error!\n");
        return ERR_PTR(-EINVAL);
    }

    if sg_n_mapped > (*pool).sge_nr {
        dev_err(dev, "the number of entries in input scatterlist is bigger than SGL pool setting.\n");
        dma_unmap_sg(dev, sgl, sg_n, dir);
        return ERR_PTR(-EINVAL);
    }

    let mut curr_sgl_dma = 0;
    let curr_hw_sgl = acc_get_sgl(pool, index, &mut curr_sgl_dma);
    (*curr_hw_sgl).entry_length_in_sgl = cpu_to_le16((*pool).sge_nr as u16);
    let mut curr_hw_sge = (*curr_hw_sgl).sge_entries.as_mut_ptr();

    let mut i = 0;
    let mut sg = sgl;
    while i < sg_n_mapped {
        sg_map_to_hw_sg(sg, curr_hw_sge);
        inc_hw_sgl_sge(curr_hw_sgl);
        curr_hw_sge = curr_hw_sge.add(1);
        sg = sg_next(sg);
        i += 1;
    }

    update_hw_sgl_sum_sge(curr_hw_sgl, (*pool).sge_nr as u16);
    *hw_sgl_dma = curr_sgl_dma;
    curr_hw_sgl
}

/** Unmap allocated hw sgl. */
pub unsafe fn hisi_acc_sg_buf_unmap(
    dev: *mut device,
    sgl: *mut scatterlist,
    hw_sgl: *mut hisi_acc_hw_sgl,
    dir: dma_data_direction,
) {
    if dev.is_null() || sgl.is_null() || hw_sgl.is_null() {
        return;
    }

    dma_unmap_sg(dev, sgl, sg_nents(sgl), dir);
    clear_hw_sgl_sge(hw_sgl);
    (*hw_sgl).entry_sum_in_chain = 0;
    (*hw_sgl).entry_sum_in_sgl = 0;
    (*hw_sgl).entry_length_in_sgl = 0;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
