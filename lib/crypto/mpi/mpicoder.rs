/* mpicoder.c - Coder for the external representation of MPIs */
/*
 * This file is part of GnuPG and is covered by the GNU General Public
 * License.  The original copyright and license notice is retained in the
 * corresponding C source.
 */

// Kernel headers and mpi-internal.h supply the types, constants, macros, and
// external functions referenced below.

const MAX_EXTERN_MPI_BITS: usize = 16384;

/** Read a raw byte stream as a positive integer. */
#[no_mangle]
pub unsafe extern "C" fn mpi_read_raw_data(xbuffer: *const core::ffi::c_void,
                                           mut nbytes: usize) -> MPI {
    let mut buffer = xbuffer as *const u8;
    let mut i: i32;
    let mut j: i32;
    let mut nbits: usize;
    let nlimbs: usize;
    let mut a: mpi_limb_t;
    let mut val: MPI = core::ptr::null_mut();

    while nbytes > 0 && *buffer == 0 {
        buffer = buffer.add(1);
        nbytes -= 1;
    }

    nbits = nbytes * 8;
    if nbits > MAX_EXTERN_MPI_BITS {
        pr_info(b"MPI: mpi too large (%u bits)\0".as_ptr(), nbits);
        return core::ptr::null_mut();
    }
    if nbytes > 0 {
        nbits -= count_leading_zeros(*buffer) as usize - (BITS_PER_LONG - 8);
    }

    nlimbs = (nbytes + BYTES_PER_MPI_LIMB - 1) / BYTES_PER_MPI_LIMB;
    val = mpi_alloc(nlimbs);
    if val.is_null() {
        return val;
    }
    (*val).nbits = nbits;
    (*val).sign = 0;
    (*val).nlimbs = nlimbs;

    if nbytes > 0 {
        i = (BYTES_PER_MPI_LIMB - nbytes % BYTES_PER_MPI_LIMB) as i32;
        i %= BYTES_PER_MPI_LIMB as i32;
        j = nlimbs as i32;
        while j > 0 {
            a = 0;
            while i < BYTES_PER_MPI_LIMB as i32 {
                a = (a << 8) | (*buffer as mpi_limb_t);
                buffer = buffer.add(1);
                i += 1;
            }
            i = 0;
            (*val).d[(j - 1) as usize] = a;
            j -= 1;
        }
    }
    val
}

#[no_mangle]
pub unsafe extern "C" fn mpi_read_from_buffer(xbuffer: *const core::ffi::c_void,
                                               ret_nread: *mut u32) -> MPI {
    let buffer = xbuffer as *const u8;
    let nbits: u32;
    let nbytes: u32;
    let val: MPI;

    if *ret_nread < 2 { return ERR_PTR(-EINVAL); }
    nbits = ((*buffer as u32) << 8) | *buffer.add(1) as u32;
    if nbits as usize > MAX_EXTERN_MPI_BITS { return ERR_PTR(-EINVAL); }
    nbytes = (nbits + 7) / 8;
    if nbytes + 2 > *ret_nread { return ERR_PTR(-EINVAL); }
    val = mpi_read_raw_data(buffer.add(2) as *const core::ffi::c_void, nbytes as usize);
    if val.is_null() { return ERR_PTR(-ENOMEM); }
    *ret_nread = nbytes + 2;
    val
}

unsafe fn count_lzeros(a: MPI) -> i32 {
    let mut lzeros = 0;
    let mut i = (*a).nlimbs as i32 - 1;
    while i >= 0 {
        let alimb = (*a).d[i as usize];
        if alimb == 0 { lzeros += core::mem::size_of::<mpi_limb_t>() as i32; }
        else { lzeros += (count_leading_zeros(alimb) / 8) as i32; break; }
        i -= 1;
    }
    lzeros
}

#[no_mangle]
pub unsafe extern "C" fn mpi_read_buffer(a: MPI, buf: *mut u8, buf_len: u32,
                                          nbytes: *mut u32, sign: *mut i32) -> i32 {
    if buf.is_null() || nbytes.is_null() { return -EINVAL; }
    if !sign.is_null() { *sign = (*a).sign; }
    let n = mpi_get_size(a) as i32;
    let mut lzeros = count_lzeros(a);
    if buf_len < (n - lzeros) as u32 { *nbytes = (n - lzeros) as u32; return -EOVERFLOW; }
    *nbytes = (n - lzeros) as u32;
    let mut p = buf;
    let mut i = (*a).nlimbs as i32 - 1 - lzeros / BYTES_PER_MPI_LIMB as i32;
    lzeros %= BYTES_PER_MPI_LIMB as i32;
    while i >= 0 {
        let limb = (*a).d[i as usize].to_be_bytes();
        let count = BYTES_PER_MPI_LIMB as i32 - lzeros;
        core::ptr::copy_nonoverlapping(limb.as_ptr().add(lzeros as usize), p, count as usize);
        p = p.add(count as usize);
        lzeros = 0;
        i -= 1;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn mpi_get_buffer(a: MPI, nbytes: *mut u32, sign: *mut i32) -> *mut core::ffi::c_void {
    if nbytes.is_null() { return core::ptr::null_mut(); }
    let mut n = mpi_get_size(a);
    if n == 0 { n = 1; }
    let buf = kmalloc(n, GFP_KERNEL) as *mut u8;
    if buf.is_null() { return core::ptr::null_mut(); }
    if mpi_read_buffer(a, buf, n as u32, nbytes, sign) != 0 { kfree(buf as *mut core::ffi::c_void); return core::ptr::null_mut(); }
    buf as *mut core::ffi::c_void
}

#[no_mangle]
pub unsafe extern "C" fn mpi_write_to_sgl(a: MPI, sgl: *mut scatterlist,
                                            mut nbytes: u32, sign: *mut i32) -> i32 {
    if !sign.is_null() { *sign = (*a).sign; }
    let n = mpi_get_size(a) as u32;
    if nbytes < n { return -EOVERFLOW; }
    let nents = sg_nents_for_len(sgl, nbytes);
    if nents < 0 { return -EINVAL; }
    let mut miter = core::mem::zeroed::<sg_mapping_iter>();
    sg_miter_start(&mut miter, sgl, nents as u32, SG_MITER_ATOMIC | SG_MITER_TO_SG);
    sg_miter_next(&mut miter);
    let mut buf_len = miter.length;
    let mut p2 = miter.addr;
    while nbytes > n {
        let i = core::cmp::min(nbytes - n, buf_len);
        core::ptr::write_bytes(p2, 0, i as usize);
        p2 = p2.add(i as usize); nbytes -= i; buf_len -= i;
        if buf_len == 0 { sg_miter_next(&mut miter); buf_len = miter.length; p2 = miter.addr; }
    }
    let mut i = (*a).nlimbs as i32 - 1;
    while i >= 0 {
        let bytes = (*a).d[i as usize].to_be_bytes();
        for byte in bytes {
            *p2 = byte; p2 = p2.add(1); buf_len -= 1;
            if buf_len == 0 { sg_miter_next(&mut miter); buf_len = miter.length; p2 = miter.addr; }
        }
        i -= 1;
    }
    sg_miter_stop(&mut miter); 0
}

#[no_mangle]
pub unsafe extern "C" fn mpi_read_raw_from_sgl(sgl: *mut scatterlist, mut nbytes: u32) -> MPI {
    let ents = sg_nents_for_len(sgl, nbytes);
    if ents < 0 { return core::ptr::null_mut(); }
    let mut miter = core::mem::zeroed::<sg_mapping_iter>();
    sg_miter_start(&mut miter, sgl, ents as u32, SG_MITER_ATOMIC | SG_MITER_FROM_SG);
    let mut lzeros = 0u32; let mut len = 0u32; let mut buff: *const u8 = core::ptr::null();
    while nbytes > 0 {
        while len > 0 && *buff == 0 && lzeros < nbytes { lzeros += 1; len -= 1; buff = buff.add(1); }
        if len > 0 && *buff != 0 { break; }
        sg_miter_next(&mut miter); buff = miter.addr; len = miter.length; nbytes -= lzeros; lzeros = 0;
    }
    miter.consumed = lzeros;
    nbytes -= lzeros;
    let mut nbits = nbytes * 8;
    if nbits as usize > MAX_EXTERN_MPI_BITS { sg_miter_stop(&mut miter); return core::ptr::null_mut(); }
    if nbytes > 0 { nbits -= count_leading_zeros(*buff) as u32 - (BITS_PER_LONG - 8) as u32; }
    sg_miter_stop(&mut miter);
    let nlimbs = (nbytes as usize + BYTES_PER_MPI_LIMB - 1) / BYTES_PER_MPI_LIMB;
    let val = mpi_alloc(nlimbs); if val.is_null() { return val; }
    (*val).nbits = nbits as usize; (*val).sign = 0; (*val).nlimbs = nlimbs;
    if nbytes == 0 { return val; }
    let mut j = nlimbs as i32 - 1; let mut a: mpi_limb_t = 0;
    let mut z = (BYTES_PER_MPI_LIMB as u32 - nbytes % BYTES_PER_MPI_LIMB as u32) % BYTES_PER_MPI_LIMB as u32;
    sg_miter_start(&mut miter, sgl, ents as u32, SG_MITER_ATOMIC | SG_MITER_FROM_SG);
    while sg_miter_next(&mut miter) {
        buff = miter.addr; let take = core::cmp::min(miter.length, nbytes); nbytes -= take;
        for x in 0..take { a = (a << 8) | *buff.add(x as usize) as mpi_limb_t; if (z + x + 1) % BYTES_PER_MPI_LIMB as u32 == 0 { (*val).d[j as usize] = a; j -= 1; a = 0; } }
        z += take;
    }
    val
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
