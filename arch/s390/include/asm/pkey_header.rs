/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Kernelspace interface to the pkey device driver
 *
 * Copyright IBM Corp. 2016, 2023
 *
 * Author: Harald Freudenberger <freude@de.ibm.com>
 *
 */

// Dependencies corresponding to: <linux/ioctl.h>, <linux/types.h>,
// <linux/delay.h>, and <uapi/asm/pkey.h> are supplied externally.

/*
 * In-kernel API: Transform an key blob (of any type) into a protected key.
 * @param key pointer to a buffer containing the key blob
 * @param keylen size of the key blob in bytes
 * @param protkey pointer to buffer receiving the protected key
 * @param xflags additional execution flags (see PKEY_XFLAG_* definitions below)
 *	  As of now the only supported flags are PKEY_XFLAG_NOMEMALLOC
 *	  and PKEY_XFLAG_NOCLEARKEY.
 * @return 0 on success, negative errno value on failure
 */
extern "C" {
    pub fn pkey_key2protkey(
        key: *const u8,
        keylen: u32,
        protkey: *mut u8,
        protkeylen: *mut u32,
        protkeytype: *mut u32,
        xflags: u32,
    ) -> i32;
}

/*
 * If this flag is given in the xflags parameter, the pkey implementation
 * is not allowed to allocate memory but instead should fall back to use
 * preallocated memory or simple fail with -ENOMEM.
 * This flag is for protected key derive within a cipher or similar
 * which must not allocate memory which would cause io operations - see
 * also the CRYPTO_ALG_ALLOCATES_MEMORY flag in crypto.h.
 */
pub const PKEY_XFLAG_NOMEMALLOC: u32 = 0x0001;

/*
 * Do not accept a clear key token as source for a protected key.
 */
pub const PKEY_XFLAG_NOCLEARKEY: u32 = 0x0002;

extern "C" {
    fn msleep(msecs: u32);
}

pub const ENOSPC: i32 = 28;

pub unsafe fn pkey_handle_expired() -> i32 {
    /*
     * Protected key expired due to relocation to another host. The long
     * running re-wrap has no asynchronous completion notification, so
     * polling is required. Trigger a re-schedule of this request by
     * returning -ENOSPC ("hardware queue full") to the crypto engine.
     * To avoid immediately re-invocation of this callback,
     * tell the scheduler to voluntarily give up the CPU here.
     */
    msleep(1);
    // Equivalent kernel debug trace: pr_debug("rescheduling request\n");
    -ENOSPC
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
