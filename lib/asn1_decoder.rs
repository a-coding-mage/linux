// SPDX-License-Identifier: GPL-2.0-or-later
/* Decoder for ASN.1 BER/DER/CER encoded bytestream
 *
 * Copyright (C) 2012 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// Linux kernel headers and their supplied symbols are external dependencies.

static ASN1_OP_LENGTHS: [u8; ASN1_OP__NR as usize] = [
    2, 2, 3, 3, 3, 3, 1, 1, 2, 2, 2, 3, 3, 1, 1, 2, 2, 1, 1, 1,
    1, 2, 1, 2, 2, 3, 2, 3,
];

/*
 * Find the length of an indefinite length object
 * @data: The data buffer
 * @datalen: The end of the innermost containing element in the buffer
 * @_dp: The data parse cursor (updated before returning)
 * @_len: Where to return the size of the element.
 * @_errmsg: Where to return a pointer to an error message on error
 */
unsafe fn asn1_find_indefinite_length(
    data: *const u8,
    datalen: usize,
    dp_out: *mut usize,
    len_out: *mut usize,
    errmsg: *mut *const u8,
) -> i32 {
    let mut dp = *dp_out;
    let mut indef_level: i32 = 1;
    'next_tag: loop {
        if datalen - dp < 2 {
            if datalen == dp { *errmsg = b"Missing EOC in indefinite len cons\0".as_ptr(); }
            else { *errmsg = b"Data overrun error\0".as_ptr(); }
            *dp_out = dp;
            return -1;
        }
        let tag = *data.add(dp); dp += 1;
        if tag == ASN1_EOC {
            if *data.add(dp) != 0 { *errmsg = b"Invalid length EOC\0".as_ptr(); *dp_out = dp; return -1; }
            dp += 1; indef_level -= 1;
            if indef_level <= 0 { *len_out = dp - *dp_out; *dp_out = dp; return 0; }
            continue 'next_tag;
        }
        if (tag & 0x1f) == ASN1_LONG_TAG {
            loop {
                if datalen - dp < 2 { *errmsg = b"Data overrun error\0".as_ptr(); *dp_out = dp; return -1; }
                let tmp = *data.add(dp); dp += 1;
                if tmp & 0x80 == 0 { break; }
            }
        }
        let mut len = *data.add(dp); dp += 1;
        if len > 0x7f {
            if len == ASN1_INDEFINITE_LENGTH {
                if (tag & ASN1_CONS_BIT) == ASN1_PRIM << 5 { *errmsg = b"Indefinite len primitive not permitted\0".as_ptr(); *dp_out = dp; return -1; }
                indef_level += 1; continue 'next_tag;
            }
            let mut n = (len - 0x80) as usize;
            if n > core::mem::size_of::<usize>() - 1 { *errmsg = b"Unsupported length\0".as_ptr(); *dp_out = dp; return -1; }
            if n > datalen - dp { *errmsg = b"Data overrun error\0".as_ptr(); *dp_out = dp; return -1; }
            len = 0;
            while n > 0 { len = (len << 8) | *data.add(dp) as u8; dp += 1; n -= 1; }
        }
        if len as usize > datalen - dp { *errmsg = b"Data overrun error\0".as_ptr(); *dp_out = dp; return -1; }
        dp += len as usize;
    }
}

/** Decode BER/DER/CER ASN.1 according to pattern. */
pub unsafe fn asn1_ber_decoder(
    decoder: *const asn1_decoder,
    context: *mut core::ffi::c_void,
    data: *const u8,
    datalen: usize,
) -> i32 {
    let machine = (*decoder).machine;
    let actions = (*decoder).actions;
    let machlen = (*decoder).machlen;
    let mut tag: u8 = 0; let mut csp: u8 = 0; let mut jsp: u8 = 0;
    let mut optag: u8 = 0; let mut hdr: u8 = 0;
    let mut pc = 0usize; let mut dp = 0usize; let mut tdp = 0usize; let mut len = 0usize;
    let mut flags: u8 = 0;
    const FLAG_INDEFINITE_LENGTH: u8 = 0x01;
    const FLAG_MATCHED: u8 = 0x02;
    const FLAG_LAST_MATCHED: u8 = 0x04;
    const FLAG_CONS: u8 = 0x20;
    let mut cons_dp_stack = [0u16; 10]; let mut cons_datalen_stack = [0u16; 10];
    let mut cons_hdrlen_stack = [0u8; 10]; let mut jump_stack = [0u8; 10];
    if datalen > 65535 { return -EMSGSIZE; }

    'next_op: loop {
        if pc >= machlen { return -EBADMSG; }
        let op = *machine.add(pc);
        if pc + ASN1_OP_LENGTHS[op as usize] as usize > machlen { return -EBADMSG; }
        if op <= ASN1_OP__MATCHES_TAG {
            if ((op & ASN1_OP_MATCH__COND) != 0 && flags & FLAG_MATCHED != 0) ||
               ((op & ASN1_OP_MATCH__SKIP) != 0 && dp == datalen) {
                flags &= !FLAG_LAST_MATCHED; pc += ASN1_OP_LENGTHS[op as usize] as usize; continue 'next_op;
            }
            flags = 0; hdr = 2;
            if datalen - dp < 2 { return -EBADMSG; }
            tag = *data.add(dp); dp += 1;
            if (tag & 0x1f) == ASN1_LONG_TAG { return -EBADMSG; }
            if op & ASN1_OP_MATCH__ANY == 0 {
                optag = *machine.add(pc + 1); flags |= optag & FLAG_CONS;
                let mut tmp = (optag ^ tag) & !(optag & ASN1_CONS_BIT);
                if tmp != 0 {
                    if op & ASN1_OP_MATCH__SKIP != 0 { pc += ASN1_OP_LENGTHS[op as usize] as usize; dp -= 1; continue 'next_op; }
                    return -EBADMSG;
                }
            }
            flags |= FLAG_MATCHED;
            len = *data.add(dp) as usize; dp += 1;
            if len > 0x7f {
                if len == ASN1_INDEFINITE_LENGTH as usize { if tag & ASN1_CONS_BIT == 0 { return -EBADMSG; } flags |= FLAG_INDEFINITE_LENGTH; if 2 > datalen - dp { return -EBADMSG; } }
                else { let mut n = len - 0x80; if n > 2 || n > datalen - dp { return -EBADMSG; } hdr += n as u8; len = 0; while n > 0 { len = (len << 8) | *data.add(dp) as usize; dp += 1; n -= 1; } if len > datalen - dp { return -EBADMSG; } }
            } else if len > datalen - dp { return -EBADMSG; }
            if flags & FLAG_CONS != 0 { if csp >= 10 { return -EBADMSG; } cons_dp_stack[csp as usize] = dp as u16; cons_hdrlen_stack[csp as usize] = hdr; if flags & FLAG_INDEFINITE_LENGTH == 0 { cons_datalen_stack[csp as usize] = datalen as u16; datalen = dp + len; } else { cons_datalen_stack[csp as usize] = 0; } csp += 1; }
            tdp = dp;
        }
        match op {
            ASN1_OP_MATCH | ASN1_OP_MATCH_OR_SKIP | ASN1_OP_MATCH_ACT | ASN1_OP_MATCH_ACT_OR_SKIP |
            ASN1_OP_MATCH_ANY | ASN1_OP_MATCH_ANY_OR_SKIP | ASN1_OP_MATCH_ANY_ACT | ASN1_OP_MATCH_ANY_ACT_OR_SKIP |
            ASN1_OP_COND_MATCH_OR_SKIP | ASN1_OP_COND_MATCH_ACT_OR_SKIP | ASN1_OP_COND_MATCH_ANY |
            ASN1_OP_COND_MATCH_ANY_OR_SKIP | ASN1_OP_COND_MATCH_ANY_ACT | ASN1_OP_COND_MATCH_ANY_ACT_OR_SKIP => {
                if flags & FLAG_CONS == 0 { if flags & FLAG_INDEFINITE_LENGTH != 0 { let mut tmp = dp; if asn1_find_indefinite_length(data, datalen, &mut tmp, &mut len, &mut (core::ptr::null_mut())) < 0 { return -EBADMSG; } } }
                if op & ASN1_OP_MATCH__ACT != 0 { let act = if op & ASN1_OP_MATCH__ANY != 0 { *machine.add(pc + 1) } else { *machine.add(pc + 2) }; let ret = ((*actions.add(act as usize))(context, hdr, tag, data.add(dp), len)); if ret < 0 { return ret; } }
                if flags & FLAG_CONS == 0 { dp += len; } pc += ASN1_OP_LENGTHS[op as usize] as usize; continue 'next_op;
            }
            ASN1_OP_MATCH_JUMP | ASN1_OP_MATCH_JUMP_OR_SKIP | ASN1_OP_COND_MATCH_JUMP_OR_SKIP => { if jsp >= 10 { return -EBADMSG; } jump_stack[jsp as usize] = (pc + ASN1_OP_LENGTHS[op as usize] as usize) as u8; jsp += 1; pc = *machine.add(pc + 2) as usize; continue 'next_op; }
            ASN1_OP_COND_FAIL => { if flags & FLAG_MATCHED == 0 { return -EBADMSG; } pc += ASN1_OP_LENGTHS[op as usize] as usize; continue 'next_op; }
            ASN1_OP_COMPLETE => { if jsp != 0 || csp != 0 { return -EBADMSG; } return 0; }
            ASN1_OP_END_SET | ASN1_OP_END_SET_ACT | ASN1_OP_END_SEQ | ASN1_OP_END_SET_OF |
            ASN1_OP_END_SEQ_OF | ASN1_OP_END_SEQ_ACT | ASN1_OP_END_SET_OF_ACT | ASN1_OP_END_SEQ_OF_ACT => {
                if (op == ASN1_OP_END_SET || op == ASN1_OP_END_SET_ACT) && flags & FLAG_MATCHED == 0 { return -EBADMSG; }
                if csp == 0 { return -EBADMSG; }
                csp -= 1; tdp = cons_dp_stack[csp as usize] as usize; hdr = cons_hdrlen_stack[csp as usize];
                let old_len = datalen; datalen = cons_datalen_stack[csp as usize] as usize; len = old_len;
                if datalen == 0 {
                    datalen = len; if datalen - dp < 2 { return -EBADMSG; }
                    if *data.add(dp) != 0 { if op & ASN1_OP_END__OF != 0 { csp += 1; pc = *machine.add(pc + 1) as usize; continue 'next_op; } return -EBADMSG; }
                    dp += 1; if *data.add(dp) != 0 { return -EBADMSG; } dp += 1; len = dp - tdp - 2;
                } else {
                    if dp < len && op & ASN1_OP_END__OF != 0 { csp += 1; datalen = len; pc = *machine.add(pc + 1) as usize; continue 'next_op; }
                    if dp != len { return -EBADMSG; } len -= tdp;
                }
                if op & ASN1_OP_END__ACT != 0 { let act = if op & ASN1_OP_END__OF != 0 { *machine.add(pc + 2) } else { *machine.add(pc + 1) }; let ret = (*actions.add(act as usize))(context, hdr, 0, data.add(tdp), len); if ret < 0 { return ret; } }
                pc += ASN1_OP_LENGTHS[op as usize] as usize; continue 'next_op;
            }
            ASN1_OP_MAYBE_ACT => { if flags & FLAG_LAST_MATCHED == 0 { pc += ASN1_OP_LENGTHS[op as usize] as usize; continue 'next_op; } }
            ASN1_OP_ACT => { let ret = (*actions.add(*machine.add(pc + 1) as usize))(context, hdr, tag, data.add(tdp), len); if ret < 0 { return ret; } pc += ASN1_OP_LENGTHS[op as usize] as usize; continue 'next_op; }
            ASN1_OP_RETURN => { if jsp == 0 { return -EBADMSG; } jsp -= 1; pc = jump_stack[jsp as usize] as usize; flags |= FLAG_MATCHED | FLAG_LAST_MATCHED; continue 'next_op; }
            _ => return -EBADMSG,
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
