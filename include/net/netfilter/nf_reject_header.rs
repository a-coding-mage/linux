/* SPDX-License-Identifier: GPL-2.0 */

// Translated from nf_reject.h.
// The C header dependencies and symbols are supplied by the surrounding crate.

#[inline]
pub unsafe fn nf_reject_verify_csum(
    skb: *mut sk_buff,
    dataoff: i32,
    proto: __u8,
) -> bool {
    /* Skip protocols that don't use 16-bit one's complement checksum
     * of the entire payload.
     */
    match proto {
        /* Protocols with optional checksums. */
        IPPROTO_UDP => {
            let mut _udp_hdr: udphdr = core::mem::zeroed();

            let udp_hdr: *const udphdr = skb_header_pointer(
                skb,
                dataoff,
                core::mem::size_of::<udphdr>(),
                &mut _udp_hdr as *mut udphdr as *mut core::ffi::c_void,
            );
            if udp_hdr.is_null() || (*udp_hdr).check != 0 {
                return true;
            }

            false
        }
        /* Protocols with other integrity checks. */
        IPPROTO_GRE | IPPROTO_AH | IPPROTO_ESP | IPPROTO_SCTP => false,

        /* Protocols with partial checksums. */
        IPPROTO_UDPLITE => false,

        _ => true,
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
