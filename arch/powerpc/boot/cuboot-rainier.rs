// SPDX-License-Identifier: GPL-2.0-only
/*
 * Old U-boot compatibility for Rainier
 *
 * Valentine Barshak <vbarshak@ru.mvista.com>
 * Copyright 2007 MontaVista Software, Inc
 *
 * Based on Ebony code by David Gibson <david@gibson.dropbear.id.au>
 * Copyright IBM Corporation, 2007
 *
 * Based on Bamboo code by Josh Boyer <jwboyer@linux.vnet.ibm.com>
 * Copyright IBM Corporation, 2007
 */

// Dependencies supplied by the surrounding PowerPC boot sources.

const TARGET_4XX: bool = true;
const TARGET_44X: bool = true;

static mut bd: bd_t = unsafe { core::mem::zeroed() };

unsafe fn rainier_fixups() {
    let sysclk: c_ulong = 33333333;

    ibm440ep_fixup_clocks(sysclk, 11059200, 50000000);
    ibm4xx_fixup_ebc_ranges(c"/plb/opb/ebc".as_ptr());
    ibm4xx_denali_fixup_memsize();
    dt_fixup_mac_address_by_alias(c"ethernet0".as_ptr(), bd.bi_enetaddr.as_ptr());
    dt_fixup_mac_address_by_alias(c"ethernet1".as_ptr(), bd.bi_enet1addr.as_ptr());
}

pub unsafe fn platform_init(
    r3: c_ulong,
    r4: c_ulong,
    r5: c_ulong,
    r6: c_ulong,
    r7: c_ulong,
) {
    CUBOOT_INIT!();
    platform_ops.fixups = Some(rainier_fixups);
    platform_ops.exit = Some(ibm44x_dbcr_reset);
    fdt_init(_dtb_start);
    serial_console_init();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
