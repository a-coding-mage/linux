/*
 * JFFS2 -- Journalling Flash File System, Version 2.
 *
 * Copyright © 2001-2007 Red Hat, Inc.
 * Copyright © 2004-2010 David Woodhouse <dwmw2@infradead.org>
 * Copyright © 2004 Ferenc Havasi <havasi@inf.u-szeged.hu>,
 *                    University of Szeged, Hungary
 *
 * Created by Arjan van de Ven <arjan@infradead.org>
 *
 * For licensing information, see the file 'LICENCE' in this directory.
 */

/* Dependencies supplied by compr.h and the kernel environment are external. */

static mut JFFS2_COMPRESSOR_LIST_LOCK: DefineSpinlock = DEFINE_SPINLOCK();
static mut JFFS2_COMPRESSOR_LIST: ListHead = LIST_HEAD();
static mut jffs2_compression_mode: i32 = JFFS2_COMPR_MODE_PRIORITY;
static mut none_stat_compr_blocks: u32 = 0;
static mut none_stat_decompr_blocks: u32 = 0;
static mut none_stat_compr_size: u32 = 0;

/* Return 1 to use this compression. */
unsafe fn jffs2_is_best_compression(this: *mut jffs2_compressor, best: *mut jffs2_compressor, size: u32, bestsize: u32) -> i32 {
    match jffs2_compression_mode {
        JFFS2_COMPR_MODE_SIZE => if bestsize > size { 1 } else { 0 },
        JFFS2_COMPR_MODE_FAVOURLZO => {
            if ((*this).compr == JFFS2_COMPR_LZO) && (bestsize > size) { return 1; }
            if ((*best).compr != JFFS2_COMPR_LZO) && (bestsize > size) { return 1; }
            if ((*this).compr == JFFS2_COMPR_LZO) && (bestsize > size.wrapping_mul(FAVOUR_LZO_PERCENT).wrapping_div(100)) { return 1; }
            if (bestsize.wrapping_mul(FAVOUR_LZO_PERCENT).wrapping_div(100)) > size { return 1; }
            0
        }
        _ => 0,
    }
}

unsafe fn jffs2_selected_compress(compr: u8, data_in: *mut u8, cpage_out: *mut *mut u8, datalen: *mut u32, cdatalen: *mut u32) -> i32 {
    let mut ret = JFFS2_COMPR_NONE;
    let output_buf = kmalloc(*cdatalen as usize, GFP_KERNEL) as *mut i8;
    if output_buf.is_null() { pr_warn!("No memory for compressor allocation. Compression failed.\n"); return ret; }
    let orig_slen = *datalen;
    let orig_dlen = *cdatalen;
    spin_lock(&mut JFFS2_COMPRESSOR_LIST_LOCK);
    list_for_each_entry!(this, &mut JFFS2_COMPRESSOR_LIST, list, {
        if (*this).compress.is_none() || (*this).disabled { continue; }
        if compr != 0 && compr != (*this).compr { continue; }
        (*this).usecount += 1;
        spin_unlock(&mut JFFS2_COMPRESSOR_LIST_LOCK);
        *datalen = orig_slen; *cdatalen = orig_dlen;
        let err = ((*this).compress.unwrap())(data_in, output_buf as *mut u8, datalen, cdatalen);
        spin_lock(&mut JFFS2_COMPRESSOR_LIST_LOCK);
        (*this).usecount -= 1;
        if err == 0 { ret = (*this).compr as i32; (*this).stat_compr_blocks += 1; (*this).stat_compr_orig_size += *datalen; (*this).stat_compr_new_size += *cdatalen; break; }
    });
    spin_unlock(&mut JFFS2_COMPRESSOR_LIST_LOCK);
    if ret == JFFS2_COMPR_NONE { kfree(output_buf as *mut _); } else { *cpage_out = output_buf as *mut u8; }
    ret
}

pub unsafe fn jffs2_compress(c: *mut jffs2_sb_info, _f: *mut jffs2_inode_info, data_in: *mut u8, cpage_out: *mut *mut u8, datalen: *mut u32, cdatalen: *mut u32) -> u16 {
    let mut ret = JFFS2_COMPR_NONE;
    let mode = if (*c).mount_opts.override_compr { (*c).mount_opts.compr } else { jffs2_compression_mode };
    match mode {
        JFFS2_COMPR_MODE_NONE => {},
        JFFS2_COMPR_MODE_PRIORITY => { ret = jffs2_selected_compress(0, data_in, cpage_out, datalen, cdatalen); },
        JFFS2_COMPR_MODE_FORCELZO => { ret = jffs2_selected_compress(JFFS2_COMPR_LZO, data_in, cpage_out, datalen, cdatalen); },
        JFFS2_COMPR_MODE_FORCEZLIB => { ret = jffs2_selected_compress(JFFS2_COMPR_ZLIB, data_in, cpage_out, datalen, cdatalen); },
        JFFS2_COMPR_MODE_SIZE | JFFS2_COMPR_MODE_FAVOURLZO => {
            /* The shared-buffer selection loop is represented directly below. */
            let orig_slen = *datalen; let orig_dlen = *cdatalen;
            let mut best: *mut jffs2_compressor = core::ptr::null_mut(); let mut best_slen = 0; let mut best_dlen = 0;
            spin_lock(&mut JFFS2_COMPRESSOR_LIST_LOCK);
            list_for_each_entry!(this, &mut JFFS2_COMPRESSOR_LIST, list, {
                if (*this).compress.is_none() || (*this).disabled { continue; }
                if (*this).compr_buf.is_null() { spin_unlock(&mut JFFS2_COMPRESSOR_LIST_LOCK); (*this).compr_buf = kmalloc(orig_slen as usize, GFP_KERNEL) as *mut u8; spin_lock(&mut JFFS2_COMPRESSOR_LIST_LOCK); if (*this).compr_buf.is_null() { continue; } (*this).compr_buf_size = orig_slen; }
                (*this).usecount += 1; spin_unlock(&mut JFFS2_COMPRESSOR_LIST_LOCK);
                *datalen = orig_slen; *cdatalen = orig_dlen;
                let err = ((*this).compress.unwrap())(data_in, (*this).compr_buf, datalen, cdatalen);
                spin_lock(&mut JFFS2_COMPRESSOR_LIST_LOCK); (*this).usecount -= 1;
                if err == 0 && (best_dlen == 0 || jffs2_is_best_compression(this, best, *cdatalen, best_dlen) != 0) && *cdatalen < *datalen { best_dlen = *cdatalen; best_slen = *datalen; best = this; }
            });
            if best_dlen != 0 { *cdatalen = best_dlen; *datalen = best_slen; *cpage_out = (*best).compr_buf; (*best).compr_buf = core::ptr::null_mut(); (*best).compr_buf_size = 0; (*best).stat_compr_blocks += 1; (*best).stat_compr_orig_size += best_slen; (*best).stat_compr_new_size += best_dlen; ret = (*best).compr as i32; }
            spin_unlock(&mut JFFS2_COMPRESSOR_LIST_LOCK);
        }
        _ => pr_err!("unknown compression mode\n"),
    }
    if ret == JFFS2_COMPR_NONE { *cpage_out = data_in; *datalen = *cdatalen; none_stat_compr_blocks += 1; none_stat_compr_size += *datalen; }
    ret as u16
}

pub unsafe fn jffs2_decompress(_c: *mut jffs2_sb_info, _f: *mut jffs2_inode_info, mut comprtype: u16, cdata_in: *mut u8, data_out: *mut u8, cdatalen: u32, datalen: u32) -> i32 {
    if (comprtype & 0xff) <= JFFS2_COMPR_ZLIB as u16 { comprtype &= 0xff; }
    match comprtype & 0xff {
        JFFS2_COMPR_NONE => { core::ptr::copy(cdata_in, data_out, datalen as usize); none_stat_decompr_blocks += 1; 0 },
        JFFS2_COMPR_ZERO => { core::ptr::write_bytes(data_out, 0, datalen as usize); 0 },
        _ => { spin_lock(&mut JFFS2_COMPRESSOR_LIST_LOCK); let mut result = -EIO; list_for_each_entry!(this, &mut JFFS2_COMPRESSOR_LIST, list, { if comprtype as u8 == (*this).compr { (*this).usecount += 1; spin_unlock(&mut JFFS2_COMPRESSOR_LIST_LOCK); result = ((*this).decompress.unwrap())(cdata_in, data_out, cdatalen, datalen); spin_lock(&mut JFFS2_COMPRESSOR_LIST_LOCK); (*this).usecount -= 1; break; } }); spin_unlock(&mut JFFS2_COMPRESSOR_LIST_LOCK); result }
    }
}

pub unsafe fn jffs2_free_comprbuf(comprbuf: *mut u8, orig: *mut u8) { if orig != comprbuf { kfree(comprbuf as *mut _); } }

/* Registration and compressor lifecycle declarations are supplied by the surrounding kernel translation. */
pub unsafe fn jffs2_register_compressor(comp: *mut jffs2_compressor) -> i32 { if (*comp).name.is_null() { return -1; } (*comp).compr_buf_size=0; (*comp).compr_buf=core::ptr::null_mut(); (*comp).usecount=0; (*comp).stat_compr_orig_size=0; (*comp).stat_compr_new_size=0; (*comp).stat_compr_blocks=0; (*comp).stat_decompr_blocks=0; spin_lock(&mut JFFS2_COMPRESSOR_LIST_LOCK); list_add_tail!(&mut (*comp).list, &mut JFFS2_COMPRESSOR_LIST); spin_unlock(&mut JFFS2_COMPRESSOR_LIST_LOCK); 0 }
pub unsafe fn jffs2_unregister_compressor(comp: *mut jffs2_compressor) -> i32 { spin_lock(&mut JFFS2_COMPRESSOR_LIST_LOCK); if (*comp).usecount != 0 { spin_unlock(&mut JFFS2_COMPRESSOR_LIST_LOCK); return -1; } list_del!(&mut (*comp).list); spin_unlock(&mut JFFS2_COMPRESSOR_LIST_LOCK); 0 }

pub unsafe fn jffs2_compressors_init() -> i32 { let mut ret = jffs2_zlib_init(); if ret != 0 { return ret; } ret=jffs2_rtime_init(); if ret != 0 { jffs2_zlib_exit(); return ret; } ret=jffs2_rubinmips_init(); if ret != 0 { jffs2_rtime_exit(); jffs2_zlib_exit(); return ret; } ret=jffs2_dynrubin_init(); if ret != 0 { jffs2_rubinmips_exit(); jffs2_rtime_exit(); jffs2_zlib_exit(); return ret; } ret=jffs2_lzo_init(); if ret != 0 { jffs2_dynrubin_exit(); jffs2_rubinmips_exit(); jffs2_rtime_exit(); jffs2_zlib_exit(); return ret; } 0 }
pub unsafe fn jffs2_compressors_exit() -> i32 { jffs2_lzo_exit(); jffs2_dynrubin_exit(); jffs2_rubinmips_exit(); jffs2_rtime_exit(); jffs2_zlib_exit(); 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
