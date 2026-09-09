// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * 6LoWPAN IPv6 UDP compression according to RFC6282
 *
 * Authors:
 * Alexander Aring <aar@pengutronix.de>
 *
 * Original written by:
 * Alexander Smirnov <alex.bluesman.smirnov@gmail.com>
 * Jon Smirl <jonsmirl@gmail.com>
 */

// Dependencies supplied by the surrounding kernel translation unit are intentionally
// left external, corresponding to the original inclusion of "nhc.h".

const LOWPAN_NHC_UDP_MASK: u8 = 0xF8;
const LOWPAN_NHC_UDP_ID: u8 = 0xF0;

const LOWPAN_NHC_UDP_4BIT_PORT: u16 = 0xF0B0;
const LOWPAN_NHC_UDP_4BIT_MASK: u16 = 0xFFF0;
const LOWPAN_NHC_UDP_8BIT_PORT: u16 = 0xF000;
const LOWPAN_NHC_UDP_8BIT_MASK: u16 = 0xFF00;

/* values for port compression, _with checksum_ ie bit 5 set to 0 */

/* all inline */
const LOWPAN_NHC_UDP_CS_P_00: u8 = 0xF0;
/* source 16bit inline, dest = 0xF0 + 8 bit inline */
const LOWPAN_NHC_UDP_CS_P_01: u8 = 0xF1;
/* source = 0xF0 + 8bit inline, dest = 16 bit inline */
const LOWPAN_NHC_UDP_CS_P_10: u8 = 0xF2;
/* source & dest = 0xF0B + 4bit inline */
const LOWPAN_NHC_UDP_CS_P_11: u8 = 0xF3;
/* checksum elided */
const LOWPAN_NHC_UDP_CS_C: u8 = 0x04;

unsafe fn udp_uncompress(skb: *mut sk_buff, needed: usize) -> i32 {
    let mut tmp: u8 = 0;
    let mut val: u8 = 0;
    let mut uh: udphdr = core::mem::zeroed();
    let mut fail: bool;
    let err: i32;

    fail = lowpan_fetch_skb(skb, &mut tmp as *mut u8 as *mut _, core::mem::size_of::<u8>());

    pr_debug!("UDP header uncompression\n");
    match tmp & LOWPAN_NHC_UDP_CS_P_11 {
        LOWPAN_NHC_UDP_CS_P_00 => {
            fail |= lowpan_fetch_skb(skb, &mut uh.source as *mut _ as *mut _, core::mem::size_of_val(&uh.source));
            fail |= lowpan_fetch_skb(skb, &mut uh.dest as *mut _ as *mut _, core::mem::size_of_val(&uh.dest));
        }
        LOWPAN_NHC_UDP_CS_P_01 => {
            fail |= lowpan_fetch_skb(skb, &mut uh.source as *mut _ as *mut _, core::mem::size_of_val(&uh.source));
            fail |= lowpan_fetch_skb(skb, &mut val as *mut u8 as *mut _, core::mem::size_of::<u8>());
            uh.dest = htons(val as u16 + LOWPAN_NHC_UDP_8BIT_PORT);
        }
        LOWPAN_NHC_UDP_CS_P_10 => {
            fail |= lowpan_fetch_skb(skb, &mut val as *mut u8 as *mut _, core::mem::size_of::<u8>());
            uh.source = htons(val as u16 + LOWPAN_NHC_UDP_8BIT_PORT);
            fail |= lowpan_fetch_skb(skb, &mut uh.dest as *mut _ as *mut _, core::mem::size_of_val(&uh.dest));
        }
        LOWPAN_NHC_UDP_CS_P_11 => {
            fail |= lowpan_fetch_skb(skb, &mut val as *mut u8 as *mut _, core::mem::size_of::<u8>());
            uh.source = htons(LOWPAN_NHC_UDP_4BIT_PORT + ((val >> 4) as u16));
            uh.dest = htons(LOWPAN_NHC_UDP_4BIT_PORT + ((val & 0x0f) as u16));
        }
        _ => BUG!(),
    }

    pr_debug!("uncompressed UDP ports: src = %d, dst = %d\n", ntohs(uh.source), ntohs(uh.dest));

    if (tmp & LOWPAN_NHC_UDP_CS_C) != 0 {
        pr_debug_ratelimited!("checksum elided currently not supported\n");
        fail = true;
    } else {
        fail |= lowpan_fetch_skb(skb, &mut uh.check as *mut _ as *mut _, core::mem::size_of_val(&uh.check));
    }

    if fail { return -22; }

    match (*lowpan_dev((*skb).dev)).lltype {
        LOWPAN_LLTYPE_IEEE802154 => {
            if (*lowpan_802154_cb(skb)).d_size != 0 {
                udp_set_len_short(&mut uh, (*lowpan_802154_cb(skb)).d_size - core::mem::size_of::<ipv6hdr>());
            } else {
                udp_set_len_short(&mut uh, (*skb).len + core::mem::size_of::<udphdr>());
            }
        }
        _ => udp_set_len_short(&mut uh, (*skb).len + core::mem::size_of::<udphdr>()),
    }
    pr_debug!("uncompressed UDP length: src = %d\n", udp_get_len_short(&uh));

    err = skb_cow(skb, needed);
    if unlikely(err) { return err; }
    skb_push(skb, core::mem::size_of::<udphdr>());
    skb_copy_to_linear_data(skb, &uh as *const _ as *const _, core::mem::size_of::<udphdr>());
    0
}

unsafe fn udp_compress(skb: *mut sk_buff, hc_ptr: *mut *mut u8) -> i32 {
    let uh = udp_hdr(skb);
    let mut tmp: u8;

    if (ntohs((*uh).source) & LOWPAN_NHC_UDP_4BIT_MASK) == LOWPAN_NHC_UDP_4BIT_PORT &&
       (ntohs((*uh).dest) & LOWPAN_NHC_UDP_4BIT_MASK) == LOWPAN_NHC_UDP_4BIT_PORT {
        pr_debug!("UDP header: both ports compression to 4 bits\n");
        tmp = LOWPAN_NHC_UDP_CS_P_11;
        lowpan_push_hc_data(hc_ptr, &tmp, core::mem::size_of::<u8>());
        tmp = (ntohs((*uh).dest) - LOWPAN_NHC_UDP_4BIT_PORT) as u8 +
              (((ntohs((*uh).source) - LOWPAN_NHC_UDP_4BIT_PORT) << 4) as u8);
        lowpan_push_hc_data(hc_ptr, &tmp, core::mem::size_of::<u8>());
    } else if (ntohs((*uh).dest) & LOWPAN_NHC_UDP_8BIT_MASK) == LOWPAN_NHC_UDP_8BIT_PORT {
        pr_debug!("UDP header: remove 8 bits of dest\n");
        tmp = LOWPAN_NHC_UDP_CS_P_01;
        lowpan_push_hc_data(hc_ptr, &tmp, 1);
        lowpan_push_hc_data(hc_ptr, &(*uh).source, core::mem::size_of_val(&(*uh).source));
        tmp = (ntohs((*uh).dest) - LOWPAN_NHC_UDP_8BIT_PORT) as u8;
        lowpan_push_hc_data(hc_ptr, &tmp, 1);
    } else if (ntohs((*uh).source) & LOWPAN_NHC_UDP_8BIT_MASK) == LOWPAN_NHC_UDP_8BIT_PORT {
        pr_debug!("UDP header: remove 8 bits of source\n");
        tmp = LOWPAN_NHC_UDP_CS_P_10;
        lowpan_push_hc_data(hc_ptr, &tmp, 1);
        tmp = (ntohs((*uh).source) - LOWPAN_NHC_UDP_8BIT_PORT) as u8;
        lowpan_push_hc_data(hc_ptr, &tmp, 1);
        lowpan_push_hc_data(hc_ptr, &(*uh).dest, core::mem::size_of_val(&(*uh).dest));
    } else {
        pr_debug!("UDP header: can't compress\n");
        tmp = LOWPAN_NHC_UDP_CS_P_00;
        lowpan_push_hc_data(hc_ptr, &tmp, 1);
        lowpan_push_hc_data(hc_ptr, &(*uh).source, core::mem::size_of_val(&(*uh).source));
        lowpan_push_hc_data(hc_ptr, &(*uh).dest, core::mem::size_of_val(&(*uh).dest));
    }
    lowpan_push_hc_data(hc_ptr, &(*uh).check, core::mem::size_of_val(&(*uh).check));
    0
}

// LOWPAN_NHC(nhc_udp, "RFC6282 UDP", NEXTHDR_UDP, sizeof(struct udphdr),
//            LOWPAN_NHC_UDP_ID, LOWPAN_NHC_UDP_MASK, udp_uncompress, udp_compress);
// module_lowpan_nhc(nhc_udp);
// MODULE_DESCRIPTION("6LoWPAN next header RFC6282 UDP compression");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
