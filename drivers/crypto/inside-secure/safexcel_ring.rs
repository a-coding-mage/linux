// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2017 Marvell
 *
 * Antoine Tenart <antoine.tenart@free-electrons.com>
 */

// Linux DMA mapping, spinlock, and safexcel declarations are supplied by the
// surrounding translation unit.

pub unsafe fn safexcel_init_ring_descriptors(
    priv_: *mut safexcel_crypto_priv,
    cdr: *mut safexcel_desc_ring,
    rdr: *mut safexcel_desc_ring,
) -> i32 {
    let mut i: i32;
    let mut cdesc: *mut safexcel_command_desc;
    let mut atok: dma_addr_t;

    (*cdr).offset = (*priv_).config.cd_offset;
    (*cdr).base = dmam_alloc_coherent(
        (*priv_).dev,
        (*cdr).offset * EIP197_DEFAULT_RING_SIZE,
        &mut (*cdr).base_dma,
        GFP_KERNEL,
    );
    if (*cdr).base.is_null() { return -ENOMEM; }
    (*cdr).write = (*cdr).base;
    (*cdr).base_end = ((*cdr).base as *mut u8).add((*cdr).offset * (EIP197_DEFAULT_RING_SIZE - 1)) as *mut _;
    (*cdr).read = (*cdr).base;

    (*cdr).shoffset = (*priv_).config.cdsh_offset;
    (*cdr).shbase = dmam_alloc_coherent(
        (*priv_).dev,
        (*cdr).shoffset * EIP197_DEFAULT_RING_SIZE,
        &mut (*cdr).shbase_dma,
        GFP_KERNEL,
    );
    if (*cdr).shbase.is_null() { return -ENOMEM; }
    (*cdr).shwrite = (*cdr).shbase;
    (*cdr).shbase_end = ((*cdr).shbase as *mut u8).add((*cdr).shoffset * (EIP197_DEFAULT_RING_SIZE - 1)) as *mut _;

    cdesc = (*cdr).base as *mut safexcel_command_desc;
    atok = (*cdr).shbase_dma;
    i = 0;
    while i < EIP197_DEFAULT_RING_SIZE as i32 {
        (*cdesc).atok_lo = lower_32_bits(atok);
        (*cdesc).atok_hi = upper_32_bits(atok);
        cdesc = (cdesc as *mut u8).add((*cdr).offset) as *mut safexcel_command_desc;
        atok += (*cdr).shoffset as dma_addr_t;
        i += 1;
    }

    (*rdr).offset = (*priv_).config.rd_offset;
    (*rdr).shoffset = (*priv_).config.res_offset;
    (*rdr).base = dmam_alloc_coherent(
        (*priv_).dev,
        (*rdr).offset * EIP197_DEFAULT_RING_SIZE,
        &mut (*rdr).base_dma,
        GFP_KERNEL,
    );
    if (*rdr).base.is_null() { return -ENOMEM; }
    (*rdr).write = (*rdr).base;
    (*rdr).base_end = ((*rdr).base as *mut u8).add((*rdr).offset * (EIP197_DEFAULT_RING_SIZE - 1)) as *mut _;
    (*rdr).read = (*rdr).base;
    0
}

#[inline]
pub unsafe fn safexcel_select_ring(priv_: *mut safexcel_crypto_priv) -> i32 {
    (atomic_inc_return(&mut (*priv_).ring_used) % (*priv_).config.rings) as i32
}

unsafe fn safexcel_ring_next_cwptr(
    _priv_: *mut safexcel_crypto_priv,
    ring: *mut safexcel_desc_ring,
    first: bool,
    atoken: *mut *mut safexcel_token,
) -> *mut core::ffi::c_void {
    let ptr = (*ring).write;
    if first { *atoken = (*ring).shwrite as *mut safexcel_token; }
    if ((*ring).write == ((*ring).read as *mut u8).sub((*ring).offset) as *mut _) ||
       ((*ring).read == (*ring).base && (*ring).write == (*ring).base_end) { return ERR_PTR(-ENOMEM); }
    if (*ring).write == (*ring).base_end {
        (*ring).write = (*ring).base;
        (*ring).shwrite = (*ring).shbase;
    } else {
        (*ring).write = ((*ring).write as *mut u8).add((*ring).offset) as *mut _;
        (*ring).shwrite = ((*ring).shwrite as *mut u8).add((*ring).shoffset) as *mut _;
    }
    ptr
}

unsafe fn safexcel_ring_next_rwptr(
    _priv_: *mut safexcel_crypto_priv,
    ring: *mut safexcel_desc_ring,
    rtoken: *mut *mut result_data_desc,
) -> *mut core::ffi::c_void {
    let ptr = (*ring).write;
    *rtoken = ((*ring).write as *mut u8).add((*ring).shoffset) as *mut result_data_desc;
    if ((*ring).write == ((*ring).read as *mut u8).sub((*ring).offset) as *mut _) ||
       ((*ring).read == (*ring).base && (*ring).write == (*ring).base_end) { return ERR_PTR(-ENOMEM); }
    if (*ring).write == (*ring).base_end { (*ring).write = (*ring).base; }
    else { (*ring).write = ((*ring).write as *mut u8).add((*ring).offset) as *mut _; }
    ptr
}

pub unsafe fn safexcel_ring_next_rptr(_priv_: *mut safexcel_crypto_priv, ring: *mut safexcel_desc_ring) -> *mut core::ffi::c_void {
    let ptr = (*ring).read;
    if (*ring).write == (*ring).read { return ERR_PTR(-ENOENT); }
    if (*ring).read == (*ring).base_end { (*ring).read = (*ring).base; }
    else { (*ring).read = ((*ring).read as *mut u8).add((*ring).offset) as *mut _; }
    ptr
}

#[inline]
pub unsafe fn safexcel_ring_curr_rptr(priv_: *mut safexcel_crypto_priv, ring: i32) -> *mut core::ffi::c_void {
    (*priv_).ring[ring as usize].rdr.read
}

#[inline]
pub unsafe fn safexcel_ring_first_rdr_index(priv_: *mut safexcel_crypto_priv, ring: i32) -> i32 {
    let rdr = &(*priv_).ring[ring as usize].rdr;
    ((rdr.read as usize - rdr.base as usize) / rdr.offset) as i32
}

#[inline]
pub unsafe fn safexcel_ring_rdr_rdesc_index(priv_: *mut safexcel_crypto_priv, ring: i32, rdesc: *mut safexcel_result_desc) -> i32 {
    let rdr = &(*priv_).ring[ring as usize].rdr;
    ((rdesc as usize - rdr.base as usize) / rdr.offset) as i32
}

pub unsafe fn safexcel_ring_rollback_wptr(_priv_: *mut safexcel_crypto_priv, ring: *mut safexcel_desc_ring) {
    if (*ring).write == (*ring).read { return; }
    if (*ring).write == (*ring).base {
        (*ring).write = (*ring).base_end;
        (*ring).shwrite = (*ring).shbase_end;
    } else {
        (*ring).write = ((*ring).write as *mut u8).sub((*ring).offset) as *mut _;
        (*ring).shwrite = ((*ring).shwrite as *mut u8).sub((*ring).shoffset) as *mut _;
    }
}

pub unsafe fn safexcel_add_cdesc(priv_: *mut safexcel_crypto_priv, ring_id: i32, first: bool, last: bool, data: dma_addr_t, data_len: u32, full_data_len: u32, context: dma_addr_t, atoken: *mut *mut safexcel_token) -> *mut safexcel_command_desc {
    let cdesc = safexcel_ring_next_cwptr(priv_, &mut (*priv_).ring[ring_id as usize].cdr, first, atoken) as *mut safexcel_command_desc;
    if IS_ERR(cdesc) { return cdesc; }
    (*cdesc).particle_size = data_len; (*cdesc).rsvd0 = 0; (*cdesc).last_seg = last; (*cdesc).first_seg = first; (*cdesc).additional_cdata_size = 0; (*cdesc).rsvd1 = 0;
    (*cdesc).data_lo = lower_32_bits(data); (*cdesc).data_hi = upper_32_bits(data);
    if first {
        (*cdesc).control_data.packet_length = if full_data_len != 0 { full_data_len } else { 1 };
        (*cdesc).control_data.options = EIP197_OPTION_MAGIC_VALUE | EIP197_OPTION_64BIT_CTX | EIP197_OPTION_CTX_CTRL_IN_CMD | EIP197_OPTION_RC_AUTO;
        (*cdesc).control_data.type_ = EIP197_TYPE_BCLA;
        (*cdesc).control_data.context_lo = lower_32_bits(context) | EIP197_CONTEXT_SMALL;
        (*cdesc).control_data.context_hi = upper_32_bits(context);
    }
    cdesc
}

pub unsafe fn safexcel_add_rdesc(priv_: *mut safexcel_crypto_priv, ring_id: i32, first: bool, last: bool, data: dma_addr_t, len: u32) -> *mut safexcel_result_desc {
    let mut rtoken: *mut result_data_desc = core::ptr::null_mut();
    let rdesc = safexcel_ring_next_rwptr(priv_, &mut (*priv_).ring[ring_id as usize].rdr, &mut rtoken) as *mut safexcel_result_desc;
    if IS_ERR(rdesc) { return rdesc; }
    (*rdesc).particle_size = len; (*rdesc).rsvd0 = 0; (*rdesc).descriptor_overflow = 1; (*rdesc).buffer_overflow = 1;
    (*rdesc).last_seg = last; (*rdesc).first_seg = first; (*rdesc).result_size = EIP197_RD64_RESULT_SIZE; (*rdesc).rsvd1 = 0;
    (*rdesc).data_lo = lower_32_bits(data); (*rdesc).data_hi = upper_32_bits(data);
    (*rtoken).packet_length = 0; (*rtoken).error_code = 0x7fff;
    rdesc
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
