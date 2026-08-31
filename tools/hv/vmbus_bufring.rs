// SPDX-License-Identifier: BSD-3-Clause
/*
 * Copyright (c) 2009-2012,2016,2023 Microsoft Corp.
 * Copyright (c) 2012 NetApp Inc.
 * Copyright (c) 2012 Citrix Inc.
 * All rights reserved.
 */

use core::arch::asm;
use core::arch::x86_64::_mm_pause;
use core::ffi::{c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

const VMBUS_RQST_ERROR: u64 = 0xFFFFFFFFFFFFFFFF;

const EAGAIN: c_int = 11;
const EINVAL: c_int = 22;
const EIO: c_int = 5;
const ENOBUFS: c_int = 105;

const PROT_READ: c_int = 0x1;
const PROT_WRITE: c_int = 0x2;
const MAP_SHARED: c_int = 0x01;
const MAP_FAILED: *mut c_void = !0usize as *mut c_void;

// External definitions supplied by the translated header and other repository files.
extern "C" {
    static VMBUS_CHANPKT_SIZE_SHIFT: c_uint;
    static VMBUS_CHANPKT_HLEN_MIN: u16;

    fn mmap(
        addr: *mut c_void,
        length: usize,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: isize,
    ) -> *mut c_void;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn vmbus_br_availwrite(br: *const vmbus_br, windex: u32) -> u32;
    fn vmbus_br_availread(br: *const vmbus_br) -> u32;
}

#[repr(C)]
pub struct iovec {
    pub iov_base: *mut c_void,
    pub iov_len: usize,
}

#[repr(C)]
pub struct vmbus_bufring {
    pub windex: u32,
    pub rindex: u32,
    pub data: [u8; 0],
}

#[repr(C)]
pub struct vmbus_br {
    pub vbr: *mut vmbus_bufring,
    pub windex: u32,
    pub dsize: u32,
}

#[repr(C)]
pub struct vmbus_chanpkt_hdr {
    pub r#type: u16,
    pub hlen: u16,
    pub tlen: u16,
    pub flags: u32,
    pub xactid: u64,
}

#[repr(C)]
pub struct vmbus_chanpkt {
    pub hdr: vmbus_chanpkt_hdr,
}

/**
 * Compiler barrier.
 *
 * Guarantees that operation reordering does not occur at compile time
 * for operations directly before and after the barrier.
 */
#[inline(always)]
unsafe fn rte_compiler_barrier() {
    asm!("", options(nostack, preserves_flags));
}

#[inline]
fn align_u32(val: u32, align: usize) -> u32 {
    val & !((align as u32).wrapping_sub(1))
}

#[no_mangle]
pub unsafe extern "C" fn vmbus_uio_map(fd: *mut c_int, size: c_int) -> *mut c_void {
    let map: *mut c_void;

    map = mmap(
        ptr::null_mut(),
        (2 * size) as usize,
        PROT_READ | PROT_WRITE,
        MAP_SHARED,
        *fd,
        0,
    );
    if map == MAP_FAILED {
        return ptr::null_mut();
    }

    map
}

/* Increase bufring index by inc with wraparound */
#[inline]
unsafe fn vmbus_br_idxinc(mut idx: u32, inc: u32, sz: u32) -> u32 {
    idx = idx.wrapping_add(inc);
    if idx >= sz {
        idx = idx.wrapping_sub(sz);
    }

    idx
}

#[no_mangle]
pub unsafe extern "C" fn vmbus_br_setup(br: *mut vmbus_br, buf: *mut c_void, blen: c_uint) {
    (*br).vbr = buf as *mut vmbus_bufring;
    (*br).windex = (*(*br).vbr).windex;
    (*br).dsize = (blen as usize).wrapping_sub(size_of::<vmbus_bufring>()) as u32;
}

#[inline(always)]
unsafe fn rte_smp_mb() {
    asm!("lock addl $0, -128(%rsp); ", options(nostack, preserves_flags));
}

#[inline]
unsafe fn rte_atomic32_cmpset(dst: *mut u32, exp: u32, src: u32) -> c_int {
    let res: u8;

    asm!(
        "lock ; cmpxchgl {src:e}, [{dst}]; sete {res}",
        src = in(reg) src,
        dst = in(reg) dst,
        inout("eax") exp => _,
        res = lateout(reg_byte) res,
        options(nostack)
    );
    res as c_int
}

#[inline]
unsafe fn vmbus_txbr_copyto(
    tbr: *const vmbus_br,
    windex: u32,
    src0: *const c_void,
    cplen: u32,
) -> u32 {
    let br_data: *mut u8 = (*(*tbr).vbr).data.as_mut_ptr();
    let br_dsize: u32 = (*tbr).dsize;
    let src: *const u8 = src0 as *const u8;

    /* XXX use double mapping like Linux kernel? */
    if cplen > br_dsize.wrapping_sub(windex) {
        let fraglen: u32 = br_dsize.wrapping_sub(windex);

        /* Wrap-around detected */
        memcpy(
            br_data.add(windex as usize) as *mut c_void,
            src as *const c_void,
            fraglen as usize,
        );
        memcpy(
            br_data as *mut c_void,
            src.add(fraglen as usize) as *const c_void,
            cplen.wrapping_sub(fraglen) as usize,
        );
    } else {
        memcpy(
            br_data.add(windex as usize) as *mut c_void,
            src as *const c_void,
            cplen as usize,
        );
    }

    vmbus_br_idxinc(windex, cplen, br_dsize)
}

/*
 * Write scattered channel packet to TX bufring.
 *
 * The offset of this channel packet is written as a 64bits value
 * immediately after this channel packet.
 *
 * The write goes through three stages:
 *  1. Reserve space in ring buffer for the new data.
 *     Writer atomically moves priv_write_index.
 *  2. Copy the new data into the ring.
 *  3. Update the tail of the ring (visible to host) that indicates
 *     next read location. Writer updates write_index
 */
unsafe fn vmbus_txbr_write(tbr: *mut vmbus_br, iov: *const iovec, iovlen: c_int) -> c_int {
    let vbr: *mut vmbus_bufring = (*tbr).vbr;
    let ring_size: u32 = (*tbr).dsize;
    let mut old_windex: u32;
    let next_windex: u32;
    let mut windex: u32;
    let mut total: u32;
    let mut save_windex: u64;
    let mut i: c_int;

    total = 0;
    i = 0;
    while i < iovlen {
        total = total.wrapping_add((*iov.add(i as usize)).iov_len as u32);
        i += 1;
    }
    total = total.wrapping_add(size_of::<u64>() as u32);

    /* Reserve space in ring */
    loop {
        let avail: u32;

        /* Get current free location */
        old_windex = (*tbr).windex;

        /* Prevent compiler reordering this with calculation */
        rte_compiler_barrier();

        avail = vmbus_br_availwrite(tbr, old_windex);

        /* If not enough space in ring, then tell caller. */
        if avail <= total {
            return -EAGAIN;
        }

        let candidate_next_windex = vmbus_br_idxinc(old_windex, total, ring_size);

        /* Atomic update of next write_index for other threads */
        if rte_atomic32_cmpset(&mut (*tbr).windex, old_windex, candidate_next_windex) != 0 {
            next_windex = candidate_next_windex;
            break;
        }
    }

    /* Space from old..new is now reserved */
    windex = old_windex;
    i = 0;
    while i < iovlen {
        windex = vmbus_txbr_copyto(
            tbr,
            windex,
            (*iov.add(i as usize)).iov_base,
            (*iov.add(i as usize)).iov_len as u32,
        );
        i += 1;
    }

    /* Set the offset of the current channel packet. */
    save_windex = (old_windex as u64) << 32;
    windex = vmbus_txbr_copyto(
        tbr,
        windex,
        &save_windex as *const u64 as *const c_void,
        size_of::<u64>() as u32,
    );

    /* The region reserved should match region used */
    if windex != next_windex {
        return -EINVAL;
    }

    /* Ensure that data is available before updating host index */
    rte_compiler_barrier();

    /* Checkin for our reservation. wait for our turn to update host */
    while rte_atomic32_cmpset(&mut (*vbr).windex, old_windex, next_windex) == 0 {
        _mm_pause();
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn rte_vmbus_chan_send(
    txbr: *mut vmbus_br,
    r#type: u16,
    data: *mut c_void,
    dlen: u32,
    flags: u32,
) -> c_int {
    let mut pkt: vmbus_chanpkt = core::mem::zeroed();
    let pktlen: c_uint;
    let pad_pktlen: c_uint;
    let hlen: u32 = size_of::<vmbus_chanpkt>() as u32;
    let mut pad: u64 = 0;
    let mut iov: [iovec; 3] = core::mem::zeroed();
    let error: c_int;

    pktlen = hlen.wrapping_add(dlen);
    pad_pktlen = align_u32(pktlen, size_of::<u64>());

    pkt.hdr.r#type = r#type;
    pkt.hdr.flags = flags;
    pkt.hdr.hlen = hlen >> VMBUS_CHANPKT_SIZE_SHIFT;
    pkt.hdr.tlen = pad_pktlen >> VMBUS_CHANPKT_SIZE_SHIFT;
    pkt.hdr.xactid = VMBUS_RQST_ERROR;

    iov[0].iov_base = &mut pkt as *mut vmbus_chanpkt as *mut c_void;
    iov[0].iov_len = hlen as usize;
    iov[1].iov_base = data;
    iov[1].iov_len = dlen as usize;
    iov[2].iov_base = &mut pad as *mut u64 as *mut c_void;
    iov[2].iov_len = pad_pktlen.wrapping_sub(pktlen) as usize;

    error = vmbus_txbr_write(txbr, iov.as_ptr(), 3);

    error
}

#[inline]
unsafe fn vmbus_rxbr_copyfrom(
    rbr: *const vmbus_br,
    rindex: u32,
    dst0: *mut c_void,
    cplen: usize,
) -> u32 {
    let br_data: *const u8 = (*(*rbr).vbr).data.as_ptr();
    let br_dsize: u32 = (*rbr).dsize;
    let dst: *mut u8 = dst0 as *mut u8;

    if cplen > br_dsize.wrapping_sub(rindex) as usize {
        let fraglen: u32 = br_dsize.wrapping_sub(rindex);

        /* Wrap-around detected. */
        memcpy(
            dst as *mut c_void,
            br_data.add(rindex as usize) as *const c_void,
            fraglen as usize,
        );
        memcpy(
            dst.add(fraglen as usize) as *mut c_void,
            br_data as *const c_void,
            cplen.wrapping_sub(fraglen as usize),
        );
    } else {
        memcpy(
            dst as *mut c_void,
            br_data.add(rindex as usize) as *const c_void,
            cplen,
        );
    }

    vmbus_br_idxinc(rindex, cplen as u32, br_dsize)
}

/* Copy data from receive ring but don't change index */
unsafe fn vmbus_rxbr_peek(rbr: *const vmbus_br, data: *mut c_void, dlen: usize) -> c_int {
    let avail: u32;

    /*
     * The requested data and the 64bits channel packet
     * offset should be there at least.
     */
    avail = vmbus_br_availread(rbr);
    if avail < dlen.wrapping_add(size_of::<u64>()) as u32 {
        return -EAGAIN;
    }

    vmbus_rxbr_copyfrom(rbr, (*(*rbr).vbr).rindex, data, dlen);
    0
}

/*
 * Copy data from receive ring and change index
 * NOTE:
 * We assume (dlen + skip) == sizeof(channel packet).
 */
unsafe fn vmbus_rxbr_read(
    rbr: *mut vmbus_br,
    data: *mut c_void,
    dlen: usize,
    skip: usize,
) -> c_int {
    let vbr: *mut vmbus_bufring = (*rbr).vbr;
    let br_dsize: u32 = (*rbr).dsize;
    let mut rindex: u32;

    if vmbus_br_availread(rbr) < dlen.wrapping_add(skip).wrapping_add(size_of::<u64>()) as u32 {
        return -EAGAIN;
    }

    /* Record where host was when we started read (for debug) */
    (*rbr).windex = (*(*rbr).vbr).windex;

    /*
     * Copy channel packet from RX bufring.
     */
    rindex = vmbus_br_idxinc((*(*rbr).vbr).rindex, skip as u32, br_dsize);
    rindex = vmbus_rxbr_copyfrom(rbr, rindex, data, dlen);

    /*
     * Discard this channel packet's 64bits offset, which is useless to us.
     */
    rindex = vmbus_br_idxinc(rindex, size_of::<u64>() as u32, br_dsize);

    /* Update the read index _after_ the channel packet is fetched.	 */
    rte_compiler_barrier();

    (*vbr).rindex = rindex;

    0
}

#[no_mangle]
pub unsafe extern "C" fn rte_vmbus_chan_recv_raw(
    rxbr: *mut vmbus_br,
    data: *mut c_void,
    len: *mut u32,
) -> c_int {
    let mut pkt: vmbus_chanpkt_hdr = core::mem::zeroed();
    let dlen: u32;
    let bufferlen: u32 = *len;
    let mut error: c_int;

    error = vmbus_rxbr_peek(
        rxbr,
        &mut pkt as *mut vmbus_chanpkt_hdr as *mut c_void,
        size_of::<vmbus_chanpkt_hdr>(),
    );
    if error != 0 {
        return error;
    }

    if pkt.hlen < VMBUS_CHANPKT_HLEN_MIN {
        /* XXX this channel is dead actually. */
        return -EIO;
    }

    if pkt.hlen > pkt.tlen {
        return -EIO;
    }

    /* Length are in quad words */
    dlen = (pkt.tlen as u32) << VMBUS_CHANPKT_SIZE_SHIFT;
    *len = dlen;

    /* If caller buffer is not large enough */
    if dlen > bufferlen {
        return -ENOBUFS;
    }

    /* Read data and skip packet header */
    error = vmbus_rxbr_read(rxbr, data, dlen as usize, 0);
    if error != 0 {
        return error;
    }

    /* Return the number of bytes read */
    dlen.wrapping_add(size_of::<u64>() as u32) as c_int
}
