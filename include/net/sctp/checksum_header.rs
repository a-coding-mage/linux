/* SPDX-License-Identifier: GPL-2.0-or-later */
/* SCTP kernel reference Implementation
 * Copyright (c) 1999-2001 Motorola, Inc.
 * Copyright (c) 2001-2003 International Business Machines, Corp.
 *
 * This file is part of the SCTP kernel reference Implementation
 *
 * SCTP Checksum functions
 *
 * Please send any bug reports or fixes you make to the
 * email address(es):
 *    lksctp developers <linux-sctp@vger.kernel.org>
 *
 * Written or modified by:
 *    Dinakaran Joseph
 *    Jon Grimm <jgrimm@us.ibm.com>
 *    Sridhar Samudrala <sri@us.ibm.com>
 *    Vlad Yasevich <vladislav.yasevich@hp.com>
 */

/* Dependencies supplied by the surrounding translation unit:
 * linux/types.h and linux/sctp.h
 */

pub unsafe fn sctp_compute_cksum(skb: *const sk_buff, offset: usize) -> __le32 {
    let sh = ((*skb).data.add(offset)) as *mut sctphdr;
    let old: __le32 = (*sh).checksum;
    let new: u32;

    (*sh).checksum = 0;
    new = !skb_crc32c(skb, offset, (*skb).len - offset, !0);
    (*sh).checksum = old;
    cpu_to_le32(new)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
