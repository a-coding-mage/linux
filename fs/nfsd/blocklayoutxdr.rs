// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2014-2016 Christoph Hellwig.
 */

// Dependencies are supplied by the surrounding kernel translation.

const NFSDDBG_FACILITY: u32 = NFSDDBG_PNFS;

/**
 * nfsd4_block_encode_layoutget - encode block/scsi layout extent array
 * @xdr: stream for data encoding
 * @lgp: layoutget content, actually an array of extents to encode
 *
 * Encode the opaque loc_body field in the layoutget response. Since the
 * pnfs_block_layout4 and pnfs_scsi_layout4 structures on the wire are
 * the same, this function is used by both layout drivers.
 */
pub unsafe extern "C" fn nfsd4_block_encode_layoutget(
    xdr: *mut xdr_stream,
    lgp: *const nfsd4_layoutget,
) -> __be32 {
    let bl = (*lgp).lg_content as *const pnfs_block_layout;
    let len: u32 = (core::mem::size_of::<__be32>() as u32)
        .wrapping_add((*bl).nr_extents.wrapping_mul(PNFS_BLOCK_EXTENT_SIZE));
    let mut p = xdr_reserve_space(
        xdr,
        (core::mem::size_of::<__be32>() as u32).wrapping_add(len) as usize,
    );
    if p.is_null() {
        return nfserr_toosmall;
    }

    *p = cpu_to_be32(len);
    p = p.add(1);
    *p = cpu_to_be32((*bl).nr_extents);
    p = p.add(1);

    let mut i = 0;
    while i < (*bl).nr_extents {
        let bex = (*bl).extents.add(i as usize);
        p = svcxdr_encode_deviceid4(p, &(*bex).vol_id);
        p = xdr_encode_hyper(p, (*bex).foff);
        p = xdr_encode_hyper(p, (*bex).len);
        p = xdr_encode_hyper(p, (*bex).soff);
        *p = cpu_to_be32((*bex).es);
        p = p.add(1);
        i = i.wrapping_add(1);
    }

    nfs_ok
}

unsafe fn nfsd4_block_encode_volume(
    xdr: *mut xdr_stream,
    b: *mut pnfs_block_volume,
) -> i32 {
    let mut p: *mut __be32;
    let len: i32;

    match (*b).type_ {
        PNFS_BLOCK_VOLUME_SIMPLE => {
            len = 4 + 4 + 8 + 4 + (XDR_QUADLEN((*b).simple.sig_len) << 2) as i32;
            p = xdr_reserve_space(xdr, len as usize);
            if p.is_null() { return -ETOOSMALL; }
            *p = cpu_to_be32((*b).type_); p = p.add(1);
            *p = cpu_to_be32(1); p = p.add(1); // single signature
            p = xdr_encode_hyper(p, (*b).simple.offset);
            p = xdr_encode_opaque(p, (*b).simple.sig, (*b).simple.sig_len);
        }
        PNFS_BLOCK_VOLUME_SCSI => {
            len = 4 + 4 + 4 + 4
                + (XDR_QUADLEN((*b).scsi.designator_len) << 2) as i32 + 8;
            p = xdr_reserve_space(xdr, len as usize);
            if p.is_null() { return -ETOOSMALL; }
            *p = cpu_to_be32((*b).type_); p = p.add(1);
            *p = cpu_to_be32((*b).scsi.code_set); p = p.add(1);
            *p = cpu_to_be32((*b).scsi.designator_type); p = p.add(1);
            p = xdr_encode_opaque(p, (*b).scsi.designator, (*b).scsi.designator_len);
            p = xdr_encode_hyper(p, (*b).scsi.pr_key);
        }
        _ => return -ENOTSUPP,
    }
    len
}

pub unsafe extern "C" fn nfsd4_block_encode_getdeviceinfo(
    xdr: *mut xdr_stream,
    gdp: *const nfsd4_getdeviceinfo,
) -> __be32 {
    let dev = (*gdp).gd_device as *mut pnfs_block_deviceaddr;
    let mut len: i32 = core::mem::size_of::<__be32>() as i32;
    if (*gdp).gd_maxcount == 0 {
        if xdr_stream_encode_u32(xdr, 0) != XDR_UNIT { return nfserr_resource; }
        return nfs_ok;
    }
    let mut p = xdr_reserve_space(xdr, (len + core::mem::size_of::<__be32>() as i32) as usize);
    if p.is_null() { return nfserr_resource; }
    let mut i = 0;
    while i < (*dev).nr_volumes {
        let ret = nfsd4_block_encode_volume(xdr, (*dev).volumes.add(i as usize));
        if ret < 0 { return nfserrno(ret); }
        len += ret;
        i += 1;
    }
    *p = cpu_to_be32(len as u32); p = p.add(1);
    *p = cpu_to_be32((*dev).nr_volumes);
    0
}

pub unsafe extern "C" fn nfsd4_block_decode_layoutupdate(
    xdr: *mut xdr_stream, iomapp: *mut *mut iomap,
    nr_iomapsp: *mut i32, block_size: u32,
) -> __be32 {
    decode_block_layoutupdate(xdr, iomapp, nr_iomapsp, block_size)
}

unsafe fn decode_block_layoutupdate(
    xdr: *mut xdr_stream, iomapp: *mut *mut iomap,
    nr_iomapsp: *mut i32, block_size: u32,
) -> __be32 {
    let mut nr = 0u32;
    if xdr_stream_decode_u32(xdr, &mut nr) != 0 { return nfserr_bad_xdr; }
    let len = (core::mem::size_of::<__be32>() as u32).wrapping_add(xdr_stream_remaining(xdr));
    let expected = (core::mem::size_of::<__be32>() as u32).wrapping_add(nr.wrapping_mul(PNFS_BLOCK_EXTENT_SIZE));
    if len != expected { return nfserr_bad_xdr; }
    let iomaps = kzalloc_iomaps(nr);
    if iomaps.is_null() { return nfserr_delay; }
    let mut i = 0;
    while i < nr {
        let mut bex = core::mem::zeroed::<pnfs_block_extent>();
        if nfsd4_decode_deviceid4(xdr, &mut bex.vol_id) != 0 || xdr_stream_decode_u64(xdr, &mut bex.foff) != 0 || bex.foff & (block_size - 1) != 0 || xdr_stream_decode_u64(xdr, &mut bex.len) != 0 || bex.len & (block_size - 1) != 0 || xdr_stream_decode_u64(xdr, &mut bex.soff) != 0 || bex.soff & (block_size - 1) != 0 || xdr_stream_decode_u32(xdr, &mut bex.es) != 0 || bex.es != PNFS_BLOCK_READWRITE_DATA { kfree(iomaps); return nfserr_bad_xdr; }
        (*iomaps.add(i as usize)).offset = bex.foff; (*iomaps.add(i as usize)).length = bex.len; i += 1;
    }
    *iomapp = iomaps; *nr_iomapsp = nr as i32; nfs_ok
}

pub unsafe extern "C" fn nfsd4_scsi_decode_layoutupdate(
    xdr: *mut xdr_stream, iomapp: *mut *mut iomap,
    nr_iomapsp: *mut i32, block_size: u32,
) -> __be32 {
    let mut nr = 0u32;
    if xdr_stream_decode_u32(xdr, &mut nr) != 0 { return nfserr_bad_xdr; }
    let len = (core::mem::size_of::<__be32>() as u32).wrapping_add(xdr_stream_remaining(xdr));
    if len != (core::mem::size_of::<__be32>() as u32).wrapping_add(nr.wrapping_mul(PNFS_SCSI_RANGE_SIZE)) { return nfserr_bad_xdr; }
    let iomaps = kzalloc_iomaps(nr); if iomaps.is_null() { return nfserr_delay; }
    let mut i = 0;
    while i < nr {
        let mut val = 0u64;
        if xdr_stream_decode_u64(xdr, &mut val) != 0 || val & (block_size - 1) != 0 { kfree(iomaps); return nfserr_bad_xdr; }
        (*iomaps.add(i as usize)).offset = val;
        if xdr_stream_decode_u64(xdr, &mut val) != 0 || val & (block_size - 1) != 0 { kfree(iomaps); return nfserr_bad_xdr; }
        (*iomaps.add(i as usize)).length = val; i += 1;
    }
    *iomapp = iomaps; *nr_iomapsp = nr as i32; nfs_ok
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
