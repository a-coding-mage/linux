// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2025 MIPS.
 */

// C dependencies supplied by the surrounding kernel translation.

#[inline]
unsafe fn errata_probe_pause() -> bool {
    // Build-time condition: CONFIG_ERRATA_MIPS_P8700_PAUSE_OPCODE.
    if !IS_ENABLED_CONFIG_ERRATA_MIPS_P8700_PAUSE_OPCODE {
        return false;
    }

    if !riscv_isa_vendor_extension_available(MIPS_VENDOR_ID, XMIPSEXECTL) {
        return false;
    }

    true
}

unsafe fn mips_errata_probe() -> u32 {
    let mut cpu_req_errata: u32 = 0;

    if errata_probe_pause() {
        cpu_req_errata |= 1u32 << ERRATA_MIPS_P8700_PAUSE_OPCODE;
    }

    cpu_req_errata
}

pub unsafe fn mips_errata_patch_func(
    mut begin: *mut alt_entry,
    end: *mut alt_entry,
    _archid: usize,
    _impid: usize,
    stage: u32,
) {
    let mut alt: *mut alt_entry;
    let cpu_req_errata: u32 = mips_errata_probe();
    let mut tmp: u32;

    // BUILD_BUG_ON(ERRATA_MIPS_NUMBER >= RISCV_VENDOR_EXT_ALTERNATIVES_BASE);

    if stage == RISCV_ALTERNATIVES_EARLY_BOOT {
        return;
    }

    alt = begin;
    while alt < end {
        if (*alt).vendor_id != MIPS_VENDOR_ID {
            alt = alt.add(1);
            continue;
        }

        if (*alt).patch_id >= ERRATA_MIPS_NUMBER {
            WARN(
                1,
                "MIPS errata id:{} not in kernel errata list\n",
                (*alt).patch_id,
            );
            alt = alt.add(1);
            continue;
        }

        tmp = 1u32 << (*alt).patch_id;
        if cpu_req_errata & tmp != 0 {
            mutex_lock(&text_mutex);
            patch_text_nosync(ALT_OLD_PTR(alt), ALT_ALT_PTR(alt), (*alt).alt_len);
            mutex_unlock(&text_mutex);
        }

        alt = alt.add(1);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
