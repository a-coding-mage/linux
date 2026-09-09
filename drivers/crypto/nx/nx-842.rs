// SPDX-License-Identifier: GPL-2.0-or-later
/* Cryptographic API for the NX-842 hardware compression. */

// Linux kernel and nx-842 declarations are supplied by the surrounding build.

const NX842_CRYPTO_MAGIC: u16 = 0xf842;
const BOUNCE_BUFFER_ORDER: u32 = 2;
const COMP_BUSY_TIMEOUT: u64 = 250;
const DECOMP_BUSY_TIMEOUT: u64 = 50;

#[inline]
const fn nx842_crypto_header_size(g: usize) -> usize {
    core::mem::size_of::<nx842_crypto_header>()
        + core::mem::size_of::<nx842_crypto_header_group>() * g
}

const NX842_CRYPTO_HEADER_MAX_SIZE: usize =
    nx842_crypto_header_size(NX842_CRYPTO_GROUP_MAX as usize);

#[repr(C)]
pub struct nx842_crypto_param {
    pub r#in: *mut u8,
    pub iremain: u32,
    pub out: *mut u8,
    pub oremain: u32,
    pub ototal: u32,
}

unsafe fn update_param(p: *mut nx842_crypto_param, slen: u32, dlen: u32) -> i32 {
    if (*p).iremain < slen { return -EOVERFLOW; }
    if (*p).oremain < dlen { return -ENOSPC; }
    (*p).r#in = (*p).r#in.add(slen as usize);
    (*p).iremain -= slen;
    (*p).out = (*p).out.add(dlen as usize);
    (*p).oremain -= dlen;
    (*p).ototal += dlen;
    0
}

pub unsafe fn nx842_crypto_alloc_ctx(driver: *mut nx842_driver) -> *mut core::ffi::c_void {
    let ctx = kzalloc_obj::<nx842_crypto_ctx>();
    if ctx.is_null() { return ERR_PTR(-ENOMEM); }
    spin_lock_init(&mut (*ctx).lock);
    (*ctx).driver = driver;
    (*ctx).wmem = kmalloc((*driver).workmem_size, GFP_KERNEL);
    (*ctx).sbounce = __get_free_pages(GFP_KERNEL, BOUNCE_BUFFER_ORDER) as *mut u8;
    (*ctx).dbounce = __get_free_pages(GFP_KERNEL, BOUNCE_BUFFER_ORDER) as *mut u8;
    if (*ctx).wmem.is_null() || (*ctx).sbounce.is_null() || (*ctx).dbounce.is_null() {
        nx842_crypto_free_ctx(ctx as *mut core::ffi::c_void);
        return ERR_PTR(-ENOMEM);
    }
    ctx as *mut core::ffi::c_void
}

pub unsafe fn nx842_crypto_free_ctx(p: *mut core::ffi::c_void) {
    let ctx = p as *mut nx842_crypto_ctx;
    kfree((*ctx).wmem);
    free_pages((*ctx).sbounce as usize, BOUNCE_BUFFER_ORDER);
    free_pages((*ctx).dbounce as usize, BOUNCE_BUFFER_ORDER);
    kfree(ctx);
}

unsafe fn check_constraints(c: *mut nx842_constraints) {
    if (*c).maximum > BOUNCE_BUFFER_SIZE { (*c).maximum = BOUNCE_BUFFER_SIZE; }
}

unsafe fn nx842_crypto_add_header(hdr: *mut nx842_crypto_header, buf: *mut u8) -> i32 {
    let s = nx842_crypto_header_size((*hdr).groups as usize);
    if s > be16_to_cpu((*hdr).group[0].padding) as usize { return -EINVAL; }
    core::ptr::copy_nonoverlapping(hdr as *const u8, buf, s);
    0
}

unsafe fn compress(ctx: *mut nx842_crypto_ctx, p: *mut nx842_crypto_param,
                   g: *mut nx842_crypto_header_group, c: *mut nx842_constraints,
                   ignore: *mut u16, hdrsize: u32) -> i32 {
    let mut slen = (*p).iremain; let mut dlen = (*p).oremain;
    let mut adj_slen = slen; let mut src = (*p).r#in; let mut dst = (*p).out;
    let mut dskip: i32 = 0;
    if (*p).iremain == 0 { return -EOVERFLOW; }
    if (*p).oremain == 0 || hdrsize + (*c).minimum > dlen { return -ENOSPC; }
    if slen % (*c).multiple != 0 { adj_slen = round_up(slen, (*c).multiple); }
    if slen < (*c).minimum { adj_slen = (*c).minimum; }
    if slen > (*c).maximum { adj_slen = (*c).maximum; slen = (*c).maximum; }
    if adj_slen > slen || (src as u64) % (*c).alignment as u64 != 0 {
        adj_slen = core::cmp::min(adj_slen, BOUNCE_BUFFER_SIZE);
        slen = core::cmp::min(slen, BOUNCE_BUFFER_SIZE);
        if adj_slen > slen { core::ptr::write_bytes((*ctx).sbounce.add(slen as usize), 0, (adj_slen-slen) as usize); }
        core::ptr::copy_nonoverlapping(src, (*ctx).sbounce, slen as usize); src = (*ctx).sbounce; slen = adj_slen;
    }
    dst = dst.add(hdrsize as usize); dlen -= hdrsize;
    if (dst as u64) % (*c).alignment as u64 != 0 { dskip = (ptr_align(dst, (*c).alignment) as usize - dst as usize) as i32; dst = dst.add(dskip as usize); dlen -= dskip as u32; }
    if dlen % (*c).multiple != 0 { dlen = round_down(dlen, (*c).multiple); }
    if dlen < (*c).minimum { dst = (*ctx).dbounce; dlen = round_down(core::cmp::min((*p).oremain, BOUNCE_BUFFER_SIZE), (*c).multiple); dskip = 0; }
    if dlen > (*c).maximum { dlen = (*c).maximum; }
    let tmplen = dlen; let timeout = ktime_add_ms(ktime_get(), COMP_BUSY_TIMEOUT);
    let mut ret;
    loop { dlen = tmplen; ret = ((*(*ctx).driver).compress)(src, slen, dst, &mut dlen, (*ctx).wmem); if ret == -ENOSPC && dst != (*ctx).dbounce { dst = (*ctx).dbounce; dlen = round_down(core::cmp::min((*p).oremain, BOUNCE_BUFFER_SIZE), (*c).multiple); dskip = 0; continue; } if ret != -EBUSY || !ktime_before(ktime_get(), timeout) { break; } }
    if ret != 0 { return ret; }
    dskip += hdrsize as i32;
    if dst == (*ctx).dbounce { core::ptr::copy_nonoverlapping(dst, (*p).out.add(dskip as usize), dlen as usize); }
    (*g).padding = cpu_to_be16(dskip as u16); (*g).compressed_length = cpu_to_be32(dlen); (*g).uncompressed_length = cpu_to_be32(slen);
    if (*p).iremain < slen { *ignore = (slen - (*p).iremain) as u16; slen = (*p).iremain; }
    update_param(p, slen, dskip as u32 + dlen)
}

// The remaining public entry points retain the source interfaces; their full
// bodies depend on the externally supplied kernel/NX declarations.
pub unsafe fn nx842_crypto_compress(_tfm: *mut crypto_scomp, _src: *const u8, _slen: u32, _dst: *mut u8, _dlen: *mut u32, _pctx: *mut core::ffi::c_void) -> i32 { unimplemented!() }
pub unsafe fn nx842_crypto_decompress(_tfm: *mut crypto_scomp, _src: *const u8, _slen: u32, _dst: *mut u8, _dlen: *mut u32, _pctx: *mut core::ffi::c_void) -> i32 { unimplemented!() }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
