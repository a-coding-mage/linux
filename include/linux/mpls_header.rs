/* SPDX-License-Identifier: GPL-2.0 */

// Dependency corresponding to: #include <uapi/linux/mpls.h>
// The MPLS_LS_* constants are supplied by that external header.

pub const MPLS_TTL_MASK: _ = MPLS_LS_TTL_MASK >> MPLS_LS_TTL_SHIFT;
pub const MPLS_BOS_MASK: _ = MPLS_LS_S_MASK >> MPLS_LS_S_SHIFT;
pub const MPLS_TC_MASK: _ = MPLS_LS_TC_MASK >> MPLS_LS_TC_SHIFT;
pub const MPLS_LABEL_MASK: _ = MPLS_LS_LABEL_MASK >> MPLS_LS_LABEL_SHIFT;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
