/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * linux/include/linux/nfsd/stats.h
 *
 * Statistics for NFS server.
 *
 * Copyright (C) 1995, 1996 Olaf Kirch <okir@monad.swb.de>
 */

// Dependency intent from: #include <linux/nfs4.h>

/* thread usage wraps very million seconds (approx one fortnight) */
pub const NFSD_USAGE_WRAP: usize = HZ * 1_000_000;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
