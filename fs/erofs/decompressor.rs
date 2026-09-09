// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (C) 2019 HUAWEI, Inc. */
/* Copyright (C) 2024 Alibaba Cloud */

// External kernel/compression declarations are supplied by the surrounding crate.
const LZ4_MAX_DISTANCE_PAGES: usize = DIV_ROUND_UP(LZ4_DISTANCE_MAX, PAGE_SIZE) + 1;

unsafe fn z_erofs_load_lz4_config(sb: *mut super_block, dsb: *mut erofs_super_block,
                                  data: *mut core::ffi::c_void, size: i32) -> i32 {
    let sbi = EROFS_SB(sb);
    let lz4 = data as *mut z_erofs_lz4_cfgs;
    let distance: u16;
    if !lz4.is_null() {
        if size < core::mem::size_of::<z_erofs_lz4_cfgs>() as i32 { erofs_err(sb, "invalid lz4 cfgs, size=%u", size); return -EINVAL; }
        distance = le16_to_cpu((*lz4).max_distance);
        (*sbi).lz4.max_pclusterblks = le16_to_cpu((*lz4).max_pclusterblks);
        if (*sbi).lz4.max_pclusterblks == 0 { (*sbi).lz4.max_pclusterblks = 1; }
        else if (*sbi).lz4.max_pclusterblks > erofs_blknr(sb, Z_EROFS_PCLUSTER_MAX_SIZE) { erofs_err(sb, "too large lz4 pclusterblks %u", (*sbi).lz4.max_pclusterblks); return -EINVAL; }
    } else {
        distance = le16_to_cpu((*dsb).u1.lz4_max_distance);
        if distance == 0 && !erofs_sb_has_lz4_0padding(sbi) { return 0; }
        (*sbi).lz4.max_pclusterblks = 1;
        (*sbi).available_compr_algs = 1 << Z_EROFS_COMPRESSION_LZ4;
    }
    (*sbi).lz4.max_distance_pages = if distance != 0 { DIV_ROUND_UP(distance, PAGE_SIZE) + 1 } else { LZ4_MAX_DISTANCE_PAGES };
    z_erofs_gbuf_growsize((*sbi).lz4.max_pclusterblks)
}

unsafe fn z_erofs_lz4_prepare_dstpages(rq: *mut z_erofs_decompress_req, pagepool: *mut *mut page) -> i32 {
    let mut availables: [*mut page; LZ4_MAX_DISTANCE_PAGES] = [core::ptr::null_mut(); LZ4_MAX_DISTANCE_PAGES];
    let mut bounced: [usize; DIV_ROUND_UP(LZ4_MAX_DISTANCE_PAGES, BITS_PER_LONG)] = [0; DIV_ROUND_UP(LZ4_MAX_DISTANCE_PAGES, BITS_PER_LONG)];
    let max = (*EROFS_SB((*rq).sb)).lz4.max_distance_pages;
    let mut kaddr: *mut core::ffi::c_void = core::ptr::null_mut();
    let mut top = 0usize;
    let mut j = 0usize;
    for i in 0..(*rq).outpages {
        if j >= max { j = 0; }
        if !(*rq).fillgaps && test_bit(j, bounced.as_mut_ptr()) { DBG_BUGON(i < max); DBG_BUGON(top >= max); availables[top] = *(*rq).out.add(i - max); top += 1; }
        let p = *(*rq).out.add(i);
        if !p.is_null() {
            __clear_bit(j, bounced.as_mut_ptr());
            if !PageHighMem(p) {
                if i == 0 { kaddr = page_address(p); j += 1; continue; }
                if !kaddr.is_null() && (kaddr as usize) + PAGE_SIZE == page_address(p) as usize { kaddr = (kaddr as usize + PAGE_SIZE) as *mut _; j += 1; continue; }
            }
            kaddr = core::ptr::null_mut(); j += 1; continue;
        }
        kaddr = core::ptr::null_mut(); __set_bit(j, bounced.as_mut_ptr());
        let victim = if top != 0 { top -= 1; availables[top] } else { let v = __erofs_allocpage(pagepool, (*rq).gfp, true); if v.is_null() { return -ENOMEM; } set_page_private(v, Z_EROFS_SHORTLIVED_PAGE); v };
        *(*rq).out.add(i) = victim; j += 1;
    }
    if !kaddr.is_null() { 1 } else { 0 }
}

unsafe fn z_erofs_lz4_handle_overlap(rq: *const z_erofs_decompress_req, mut inpage: *mut core::ffi::c_void, out: *mut core::ffi::c_void, inputmargin: *mut u32, maptype: *mut i32, may_inplace: bool) -> *mut u8 {
    if !(*rq).inplace_io { if (*rq).inpages <= 1 { *maptype = 0; return inpage as *mut u8; } kunmap_local(inpage); let src = erofs_vm_map_ram((*rq).in, (*rq).inpages); if src.is_null() { return ERR_PTR(-ENOMEM); } *maptype = 1; return src as *mut u8; }
    let oend = (*rq).pageofs_out + (*rq).outputsize; let omargin = PAGE_ALIGN(oend) - oend;
    if !(*rq).partial_decoding && may_inplace && omargin >= LZ4_DECOMPRESS_INPLACE_MARGIN((*rq).inputsize) {
        let mut i = 0; while i < (*rq).inpages && *(*rq).out.add((*rq).outpages - (*rq).inpages + i) == *(*rq).in.add(i) { i += 1; }
        if i >= (*rq).inpages { kunmap_local(inpage); *maptype = 3; return (out as usize + ((*rq).outpages - (*rq).inpages) * (1 << PAGE_SHIFT)) as *mut u8; }
    }
    let src = z_erofs_get_gbuf((*rq).inpages); if src.is_null() { DBG_BUGON(true); kunmap_local(inpage); return ERR_PTR(-EFAULT); }
    let mut copied = 0u32; let mut n = 0usize;
    while copied < (*rq).inputsize { let cnt = core::cmp::min((*rq).inputsize - copied, PAGE_SIZE as u32 - *inputmargin); if inpage.is_null() { inpage = kmap_local_page(*(*rq).in.add(n)); } core::ptr::copy_nonoverlapping((inpage as *mut u8).add(*inputmargin as usize), (src as *mut u8).add(copied as usize), cnt as usize); kunmap_local(inpage); inpage = core::ptr::null_mut(); *inputmargin = 0; copied += cnt; n += 1; }
    *maptype = 2; src as *mut u8
}

unsafe fn z_erofs_fixup_insize(rq: *mut z_erofs_decompress_req, padbuf: *const u8, padbufsize: u32) -> *const i8 {
    let padend = memchr_inv(padbuf, 0, padbufsize); if padend.is_null() { return c"compressed data start not found".as_ptr(); }
    let n = padend.offset_from(padbuf) as u32; (*rq).inputsize -= n; (*rq).pageofs_in += n; core::ptr::null()
}

unsafe fn __z_erofs_lz4_decompress(rq: *mut z_erofs_decompress_req, dst: *mut u8) -> *const i8 {
    let head = kmap_local_page(*(*rq).in); let mut margin = (*rq).pageofs_in;
    let reason = z_erofs_fixup_insize(rq, head.add((*rq).pageofs_in as usize), core::cmp::min((*rq).inputsize, (*(*rq).sb).s_blocksize - (*rq).pageofs_in));
    if !reason.is_null() { kunmap_local(head); return reason; }
    let inplace = ((*rq).pageofs_in + (*rq).inputsize) & ((*(*rq).sb).s_blocksize - 1) == 0;
    let mut mt = 0; let src = z_erofs_lz4_handle_overlap(rq, head as *mut _, dst as *mut _, &mut margin, &mut mt, inplace); if IS_ERR(src as *mut _) { return src as *const i8; }
    let out = dst.add((*rq).pageofs_out as usize); let ret = if (*rq).partial_decoding { LZ4_decompress_safe_partial(src.add(margin as usize), out, (*rq).inputsize, (*rq).outputsize, (*rq).outputsize) } else { LZ4_decompress_safe(src.add(margin as usize), out, (*rq).inputsize, (*rq).outputsize) };
    if mt == 0 { kunmap_local(head); } else if mt == 1 { vm_unmap_ram(src as *mut _, (*rq).inpages); } else if mt == 2 { z_erofs_put_gbuf(src as *mut _); } else if mt != 3 { DBG_BUGON(true); return ERR_PTR(-EFAULT) as *const i8; }
    if ret == (*rq).outputsize { core::ptr::null() } else if ret < 0 { c"corrupted compressed data".as_ptr() } else { c"unexpected end of stream".as_ptr() }
}

unsafe fn z_erofs_lz4_decompress(rq: *mut z_erofs_decompress_req, pool: *mut *mut page) -> *const i8 {
    let mut dst; let mut typ;
    if (*rq).inpages == 1 && (*rq).outpages == 1 && !(*rq).inplace_io { DBG_BUGON((*(*rq).out).is_null()); dst = kmap_local_page(*(*rq).out); typ = 0; }
    else { let r = z_erofs_lz4_prepare_dstpages(rq, pool); if r < 0 { return ERR_PTR(r) as *const i8; } if r > 0 { dst = page_address(*(*rq).out); typ = 1; } else { dst = erofs_vm_map_ram((*rq).out, (*rq).outpages); if dst.is_null() { return ERR_PTR(-ENOMEM) as *const i8; } typ = 2; } }
    let reason = __z_erofs_lz4_decompress(rq, dst as *mut u8); if typ == 0 { kunmap_local(dst); } else if typ == 2 { vm_unmap_ram(dst, (*rq).outpages); } reason
}

unsafe fn z_erofs_transform_plain(rq: *mut z_erofs_decompress_req, _pool: *mut *mut page) -> *const i8 {
    if (*rq).outputsize > (*rq).inputsize { return ERR_PTR(-EOPNOTSUPP) as *const i8; }
    let mut cur = 0u32; let mut ni = 0usize;
    if (*rq).alg == Z_EROFS_COMPRESSION_INTERLACED { cur = (*(*rq).sb).s_blocksize - ((*rq).pageofs_out & ((*(*rq).sb).s_blocksize - 1)); let c = core::cmp::min(cur, (*rq).outputsize); if c != 0 && !(*(*rq).out).is_null() { let k = kmap_local_page(*(*rq).in.add((*rq).inpages - 1)); if *(*rq).out == *(*rq).in.add((*rq).inpages - 1) { core::ptr::copy(k.add((*rq).pageofs_in as usize), k.add((*rq).pageofs_out as usize), c as usize); } kunmap_local(k); } (*rq).outputsize -= c; }
    while (*rq).outputsize != 0 { (*rq).pageofs_in = 0; let insz = core::cmp::min(PAGE_SIZE as u32 - (*rq).pageofs_in, (*rq).outputsize); (*rq).outputsize -= insz; if !(*(*rq).in.add(ni)).is_null() { let k = kmap_local_page(*(*rq).in.add(ni)); core::ptr::copy(k, k, insz as usize); kunmap_local(k); } cur += insz; ni += 1; }
    DBG_BUGON(ni > (*rq).inpages); core::ptr::null()
}

unsafe fn z_erofs_stream_switch_bufs(_dctx: *mut z_erofs_stream_dctx, _dst: *mut *mut core::ffi::c_void, _src: *mut *mut core::ffi::c_void, _pgpl: *mut *mut page) -> *const i8 { core::ptr::null() }

// The decompressor table and configuration/init entry points retain the C ABI layout.
extern "C" { static mut z_erofs_decomp: *mut *const z_erofs_decompressor; }

pub unsafe fn z_erofs_parse_cfgs(sb: *mut super_block, dsb: *mut erofs_super_block) -> i32 { z_erofs_load_lz4_config(sb, dsb, core::ptr::null_mut(), 0) }
pub unsafe fn z_erofs_init_decompressor() -> i32 { 0 }
pub unsafe fn z_erofs_exit_decompressor() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
