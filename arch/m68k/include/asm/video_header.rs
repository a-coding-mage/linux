/* SPDX-License-Identifier: GPL-2.0 */

/* Dependencies supplied by the surrounding architecture headers are intentionally
 * left external, matching the original header's include relationships. */

#[inline]
pub unsafe fn pgprot_framebuffer(
    mut prot: pgprot_t,
    _vm_start: core::ffi::c_ulong,
    _vm_end: core::ffi::c_ulong,
    _offset: core::ffi::c_ulong,
) -> pgprot_t {
    #[cfg(CONFIG_MMU)]
    {
    #[cfg(CONFIG_SUN3)]
    {
        pgprot_val(&mut prot) |= SUN3_PAGE_NOCACHE;
    }

    #[cfg(not(CONFIG_SUN3))]
    {
        if CPU_IS_020_OR_030 {
            pgprot_val(&mut prot) |= _PAGE_NOCACHE030;
        }
        if CPU_IS_040_OR_060 {
            pgprot_val(&mut prot) &= _CACHEMASK040;
            /* Use no-cache mode, serialized */
            pgprot_val(&mut prot) |= _PAGE_NOCACHE_S;
        }
    }
    }

    prot
}

/* #define pgprot_framebuffer pgprot_framebuffer */

/* The generic video header is included by the original source and supplies
 * additional declarations outside this file's local translation. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
