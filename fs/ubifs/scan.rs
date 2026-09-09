// SPDX-License-Identifier: GPL-2.0-only
/*
 * This file is part of UBIFS.
 *
 * Copyright (C) 2006-2008 Nokia Corporation
 *
 * Authors: Adrian Hunter
 *          Artem Bityutskiy (Битюцкий Артём)
 */

/*
 * This file implements the scan which is a general-purpose function for
 * determining what nodes are in an eraseblock. The scan is used to replay
 * the journal, to do garbage collection. for the TNC in-the-gaps method, and by
 * debugging functions.
 */

// The declarations and constants supplied by ubifs.h are external dependencies.

unsafe fn scan_padding_bytes(buf: *mut core::ffi::c_void, len: i32) -> i32 {
    let mut pad_len: i32 = 0;
    let max_pad_len: i32 = core::cmp::min(UBIFS_PAD_NODE_SZ, len);
    let mut p = buf as *mut u8;

    dbg_scan!("not a node");

    while pad_len < max_pad_len && { let v = *p; p = p.add(1); v == UBIFS_PADDING_BYTE } {
        pad_len += 1;
    }

    if pad_len == 0 || (pad_len & 7) != 0 {
        return SCANNED_GARBAGE;
    }

    dbg_scan!("%d padding bytes", pad_len);
    pad_len
}

pub unsafe fn ubifs_scan_a_node(
    c: *const ubifs_info,
    buf: *mut core::ffi::c_void,
    len: i32,
    lnum: i32,
    offs: i32,
    quiet: i32,
) -> i32 {
    let ch = buf as *mut ubifs_ch;
    let magic = le32_to_cpu((*ch).magic);

    if magic == 0xffff_ffff {
        dbg_scan!("hit empty space at LEB %d:%d", lnum, offs);
        return SCANNED_EMPTY_SPACE;
    }
    if magic != UBIFS_NODE_MAGIC {
        return scan_padding_bytes(buf, len);
    }
    if len < UBIFS_CH_SZ {
        return SCANNED_GARBAGE;
    }

    dbg_scan!("scanning %s at LEB %d:%d", dbg_ntype((*ch).node_type), lnum, offs);
    if ubifs_check_node(c, buf, len, lnum, offs, quiet, 1) != 0 {
        return SCANNED_A_CORRUPT_NODE;
    }

    if (*ch).node_type == UBIFS_PAD_NODE {
        let pad = buf as *mut ubifs_pad_node;
        let pad_len = le32_to_cpu((*pad).pad_len) as i32;
        let node_len = le32_to_cpu((*ch).len) as i32;

        if pad_len < 0 || offs + node_len + pad_len > (*c).leb_size {
            if quiet == 0 {
                ubifs_err!(c, "bad pad node at LEB %d:%d", lnum, offs);
                ubifs_dump_node(c, pad, len);
            }
            return SCANNED_A_BAD_PAD_NODE;
        }
        if ((node_len + pad_len) & 7) != 0 {
            if quiet == 0 {
                ubifs_err!(c, "bad padding length %d - %d", offs, offs + node_len + pad_len);
            }
            return SCANNED_A_BAD_PAD_NODE;
        }

        dbg_scan!("%d bytes padded at LEB %d:%d, offset now %d", pad_len, lnum, offs,
                  ALIGN(offs + node_len + pad_len, 8));
        return node_len + pad_len;
    }
    SCANNED_A_NODE
}

pub unsafe fn ubifs_start_scan(
    c: *const ubifs_info, lnum: i32, offs: i32, sbuf: *mut core::ffi::c_void,
) -> *mut ubifs_scan_leb {
    let mut sleb: *mut ubifs_scan_leb;
    let err: i32;

    dbg_scan!("scan LEB %d:%d", lnum, offs);
    sleb = kzalloc_obj!(ubifs_scan_leb, GFP_NOFS);
    if sleb.is_null() { return ERR_PTR(-ENOMEM); }

    (*sleb).lnum = lnum;
    INIT_LIST_HEAD!(&mut (*sleb).nodes);
    (*sleb).buf = sbuf;

    err = ubifs_leb_read(c, lnum, sbuf.add(offs as usize), offs, (*c).leb_size - offs, 0);
    if err != 0 && err != -EBADMSG {
        ubifs_err!(c, "cannot read %d bytes from LEB %d:%d, error %d", (*c).leb_size - offs, lnum, offs, err);
        kfree(sleb);
        return ERR_PTR(err);
    }
    sleb
}

pub unsafe fn ubifs_end_scan(c: *const ubifs_info, sleb: *mut ubifs_scan_leb, lnum: i32, offs: i32) {
    dbg_scan!("stop scanning LEB %d at offset %d", lnum, offs);
    ubifs_assert!(c, offs % (*c).min_io_size == 0);
    (*sleb).endpt = ALIGN(offs, (*c).min_io_size);
}

pub unsafe fn ubifs_add_snod(c: *const ubifs_info, sleb: *mut ubifs_scan_leb, buf: *mut core::ffi::c_void, offs: i32) -> i32 {
    let ch = buf as *mut ubifs_ch;
    let ino = buf as *mut ubifs_ino_node;
    let snod = kmalloc_obj!(ubifs_scan_node, GFP_NOFS);
    if snod.is_null() { return -ENOMEM; }

    (*snod).sqnum = le64_to_cpu((*ch).sqnum);
    (*snod).type_ = (*ch).node_type;
    (*snod).offs = offs;
    (*snod).len = le32_to_cpu((*ch).len);
    (*snod).node = buf;
    match (*ch).node_type {
        UBIFS_INO_NODE | UBIFS_DENT_NODE | UBIFS_XENT_NODE | UBIFS_DATA_NODE =>
            key_read(c, &(*ino).key, &mut (*snod).key),
        _ => invalid_key_init(c, &mut (*snod).key),
    }
    list_add_tail!(&mut (*snod).list, &mut (*sleb).nodes);
    (*sleb).nodes_cnt += 1;
    0
}

pub unsafe fn ubifs_scanned_corruption(c: *const ubifs_info, lnum: i32, offs: i32, buf: *mut core::ffi::c_void) {
    let mut len = (*c).leb_size - offs;
    ubifs_err!(c, "corruption at LEB %d:%d", lnum, offs);
    if len > 8192 { len = 8192; }
    ubifs_err!(c, "first %d bytes from LEB %d:%d", len, lnum, offs);
    print_hex_dump(KERN_DEBUG, "", DUMP_PREFIX_OFFSET, 32, 4, buf, len, 1);
}

pub unsafe fn ubifs_scan(c: *const ubifs_info, lnum: i32, mut offs: i32, sbuf: *mut core::ffi::c_void, quiet: i32) -> *mut ubifs_scan_leb {
    let mut buf = sbuf.add(offs as usize);
    let mut len = (*c).leb_size - offs;
    let sleb = ubifs_start_scan(c, lnum, offs, sbuf);
    if IS_ERR!(sleb) { return sleb; }

    while len >= 8 {
        let ch = buf as *mut ubifs_ch;
        dbg_scan!("look at LEB %d:%d (%d bytes left)", lnum, offs, len);
        cond_resched!();
        let ret = ubifs_scan_a_node(c, buf, len, lnum, offs, quiet);
        if ret > 0 { offs += ret; buf = buf.add(ret as usize); len -= ret; continue; }
        if ret == SCANNED_EMPTY_SPACE { break; }
        match ret {
            SCANNED_GARBAGE => { ubifs_err!(c, "garbage"); return scan_corrupted(c, sleb, lnum, offs, buf, quiet); }
            SCANNED_A_NODE => {}
            SCANNED_A_CORRUPT_NODE | SCANNED_A_BAD_PAD_NODE => { ubifs_err!(c, "bad node"); return scan_corrupted(c, sleb, lnum, offs, buf, quiet); }
            _ => { ubifs_err!(c, "unknown"); return scan_error(c, sleb, -EINVAL); }
        }
        let err = ubifs_add_snod(c, sleb, buf, offs);
        if err != 0 { return scan_error(c, sleb, err); }
        let node_len = ALIGN(le32_to_cpu((*ch).len) as i32, 8);
        offs += node_len; buf = buf.add(node_len as usize); len -= node_len;
    }
    if offs % (*c).min_io_size != 0 { if quiet == 0 { ubifs_err!(c, "empty space starts at non-aligned offset %d", offs); } return scan_corrupted(c, sleb, lnum, offs, buf, quiet); }
    ubifs_end_scan(c, sleb, lnum, offs);
    while len > 4 && *(buf as *const u32) == 0xffff_ffff { offs += 4; buf = buf.add(4); len -= 4; }
    while len != 0 { if *(buf as *const u8) != 0xff { if quiet == 0 { ubifs_err!(c, "corrupt empty space at LEB %d:%d", lnum, offs); } return scan_corrupted(c, sleb, lnum, offs, buf, quiet); } offs += 1; buf = buf.add(1); len -= 1; }
    sleb
}

unsafe fn scan_corrupted(c: *const ubifs_info, sleb: *mut ubifs_scan_leb, lnum: i32, offs: i32, buf: *mut core::ffi::c_void, quiet: i32) -> *mut ubifs_scan_leb {
    if quiet == 0 { ubifs_scanned_corruption(c, lnum, offs, buf); ubifs_err!(c, "LEB %d scanning failed", lnum); }
    ubifs_scan_destroy(sleb); ERR_PTR(-EUCLEAN)
}

unsafe fn scan_error(c: *const ubifs_info, sleb: *mut ubifs_scan_leb, err: i32) -> *mut ubifs_scan_leb {
    ubifs_err!(c, "LEB scanning failed, error %d", err); ubifs_scan_destroy(sleb); ERR_PTR(err)
}

pub unsafe fn ubifs_scan_destroy(sleb: *mut ubifs_scan_leb) {
    let head = &mut (*sleb).nodes;
    while !list_empty!(head) {
        let node = list_entry!( (*head).next, ubifs_scan_node, list );
        list_del!(&mut (*node).list);
        kfree(node);
    }
    kfree(sleb);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
