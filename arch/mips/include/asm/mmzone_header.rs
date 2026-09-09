/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Written by Kanoj Sarcar (kanoj@sgi.com) Aug 99
 * Rewritten for Linux 2.6 by Christoph Hellwig (hch@lst.de) Jan 2004
 */

// Dependency supplied by the surrounding translation unit: <asm/page.h>
// When CONFIG_NUMA is enabled, the original header also includes <mmzone.h>.

// The original definitions are conditional on these names not already being
// provided by another header; Rust macro availability is supplied by the
// surrounding translation unit.
macro_rules! pa_to_nid {
    ($addr:expr) => { 0 };
}

macro_rules! nid_to_addrbase {
    ($nid:expr) => { 0 };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
