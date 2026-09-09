/* SPDX-License-Identifier: GPL-2.0-or-later */

// Dependencies supplied by the corresponding kernel headers:
// <net/dropreason-core.h>, <uapi/linux/mptcp.h>

macro_rules! DEFINE_RST_REASON {
    ($FN:ident, $FNe:ident) => {
        $FN!(NOT_SPECIFIED);
        $FN!(NO_SOCKET);
        $FN!(TCP_INVALID_ACK_SEQUENCE);
        $FN!(TCP_RFC7323_PAWS);
        $FN!(TCP_TOO_OLD_ACK);
        $FN!(TCP_ACK_UNSENT_DATA);
        $FN!(TCP_FLAGS);
        $FN!(TCP_OLD_ACK);
        $FN!(TCP_ABORT_ON_DATA);
        $FN!(TCP_TIMEWAIT_SOCKET);
        $FN!(INVALID_SYN);
        $FN!(TCP_ABORT_ON_CLOSE);
        $FN!(TCP_ABORT_ON_LINGER);
        $FN!(TCP_ABORT_ON_MEMORY);
        $FN!(TCP_STATE);
        $FN!(TCP_KEEPALIVE_TIMEOUT);
        $FN!(TCP_DISCONNECT_WITH_DATA);
        $FN!(MPTCP_RST_EUNSPEC);
        $FN!(MPTCP_RST_EMPTCP);
        $FN!(MPTCP_RST_ERESOURCE);
        $FN!(MPTCP_RST_EPROHIBIT);
        $FN!(MPTCP_RST_EWQ2BIG);
        $FN!(MPTCP_RST_EBADPERF);
        $FN!(MPTCP_RST_EMIDDLEBOX);
        $FN!(ERROR);
        $FNe!(MAX);
    };
}

/**
 * enum sk_rst_reason - the reasons of socket reset
 *
 * The reasons of sk reset, which are used in TCP/MPTCP protocols.
 *
 * There are three parts in order:
 * 1) skb drop reasons: relying on drop reasons for such as passive reset
 * 2) independent reset reasons: such as active reset reasons
 * 3) reset reasons in MPTCP: only for MPTCP use
 */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum sk_rst_reason {
    // Refer to include/net/dropreason-core.h. Rely on skb drop reasons.
    SK_RST_REASON_NOT_SPECIFIED,
    SK_RST_REASON_NO_SOCKET,
    SK_RST_REASON_TCP_INVALID_ACK_SEQUENCE,
    SK_RST_REASON_TCP_RFC7323_PAWS,
    SK_RST_REASON_TCP_TOO_OLD_ACK,
    SK_RST_REASON_TCP_ACK_UNSENT_DATA,
    SK_RST_REASON_TCP_FLAGS,
    SK_RST_REASON_TCP_OLD_ACK,
    SK_RST_REASON_TCP_ABORT_ON_DATA,

    // Here start with the independent reasons.
    SK_RST_REASON_TCP_TIMEWAIT_SOCKET,
    SK_RST_REASON_INVALID_SYN,
    SK_RST_REASON_TCP_ABORT_ON_CLOSE,
    SK_RST_REASON_TCP_ABORT_ON_LINGER,
    SK_RST_REASON_TCP_ABORT_ON_MEMORY,
    SK_RST_REASON_TCP_STATE,
    SK_RST_REASON_TCP_KEEPALIVE_TIMEOUT,
    SK_RST_REASON_TCP_DISCONNECT_WITH_DATA,

    // Copy from include/uapi/linux/mptcp.h; these adhere to RFC 8684.
    SK_RST_REASON_MPTCP_RST_EUNSPEC,
    SK_RST_REASON_MPTCP_RST_EMPTCP,
    SK_RST_REASON_MPTCP_RST_ERESOURCE,
    SK_RST_REASON_MPTCP_RST_EPROHIBIT,
    SK_RST_REASON_MPTCP_RST_EWQ2BIG,
    SK_RST_REASON_MPTCP_RST_EBADPERF,
    SK_RST_REASON_MPTCP_RST_EMIDDLEBOX,
    SK_RST_REASON_ERROR,
    SK_RST_REASON_MAX,
}

/* Convert skb drop reasons to enum sk_rst_reason type. */
#[inline]
pub unsafe fn sk_rst_convert_drop_reason(reason: skb_drop_reason) -> sk_rst_reason {
    match reason {
        SKB_DROP_REASON_NOT_SPECIFIED => sk_rst_reason::SK_RST_REASON_NOT_SPECIFIED,
        SKB_DROP_REASON_NO_SOCKET => sk_rst_reason::SK_RST_REASON_NO_SOCKET,
        SKB_DROP_REASON_TCP_INVALID_ACK_SEQUENCE => {
            sk_rst_reason::SK_RST_REASON_TCP_INVALID_ACK_SEQUENCE
        }
        SKB_DROP_REASON_TCP_RFC7323_PAWS => sk_rst_reason::SK_RST_REASON_TCP_RFC7323_PAWS,
        SKB_DROP_REASON_TCP_TOO_OLD_ACK => sk_rst_reason::SK_RST_REASON_TCP_TOO_OLD_ACK,
        SKB_DROP_REASON_TCP_ACK_UNSENT_DATA => sk_rst_reason::SK_RST_REASON_TCP_ACK_UNSENT_DATA,
        SKB_DROP_REASON_TCP_FLAGS => sk_rst_reason::SK_RST_REASON_TCP_FLAGS,
        SKB_DROP_REASON_TCP_OLD_ACK => sk_rst_reason::SK_RST_REASON_TCP_OLD_ACK,
        SKB_DROP_REASON_TCP_ABORT_ON_DATA => sk_rst_reason::SK_RST_REASON_TCP_ABORT_ON_DATA,
        _ => {
            // If we don't have our own corresponding reason.
            sk_rst_reason::SK_RST_REASON_NOT_SPECIFIED
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
