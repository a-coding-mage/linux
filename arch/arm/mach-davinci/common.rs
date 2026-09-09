// SPDX-License-Identifier: GPL-2.0-only
/*
 * Code commons to all DaVinci SoCs.
 *
 * Author: Mark A. Greer <mgreer@mvista.com>
 *
 * 2009 (c) MontaVista Software, Inc.
 */

// External kernel, architecture, and local declarations are supplied by the
// surrounding translation unit.

pub static mut davinci_soc_info: davinci_soc_info = davinci_soc_info {
    ..unsafe { core::mem::zeroed() }
};

unsafe fn davinci_init_id(soc_info: *mut davinci_soc_info) -> i32 {
    let mut i: i32;
    let mut dip: *mut davinci_id;
    let variant: u8;
    let part_no: u16;
    let base: *mut core::ffi::c_void;

    base = ioremap((*soc_info).jtag_id_reg, SZ_4K);
    if base.is_null() {
        pr_err!("Unable to map JTAG ID register\n");
        return -ENOMEM;
    }

    (*soc_info).jtag_id = __raw_readl(base);
    iounmap(base);

    variant = (((*soc_info).jtag_id & 0xf0000000) >> 28) as u8;
    part_no = (((*soc_info).jtag_id & 0x0ffff000) >> 12) as u16;

    i = 0;
    dip = (*soc_info).ids;
    while i < (*soc_info).ids_num {
        // Don't care about the manufacturer right now
        if (*dip).part_no == part_no && (*dip).variant == variant {
            (*soc_info).cpu_id = (*dip).cpu_id;
            pr_info!(
                "DaVinci {} variant 0x{:x}\n",
                (*dip).name,
                (*dip).variant
            );
            return 0;
        }
        i += 1;
        dip = dip.add(1);
    }

    pr_err!("Unknown DaVinci JTAG ID 0x{:x}\n", (*soc_info).jtag_id);
    -EINVAL
}

pub unsafe fn davinci_common_init(soc_info: *const davinci_soc_info) {
    let ret: i32;

    if soc_info.is_null() {
        ret = -EINVAL;
        goto_err(ret);
        return;
    }

    core::ptr::copy_nonoverlapping(
        soc_info,
        core::ptr::addr_of_mut!(davinci_soc_info),
        1,
    );

    if !davinci_soc_info.io_desc.is_null() && davinci_soc_info.io_desc_num > 0 {
        iotable_init(
            davinci_soc_info.io_desc,
            davinci_soc_info.io_desc_num,
        );
    }

    /*
     * Normally devicemaps_init() would flush caches and tlb after
     * mdesc->map_io(), but we must also do it here because of the CPU
     * revision check below.
     */
    local_flush_tlb_all();
    flush_cache_all();

    /*
     * We want to check CPU revision early for cpu_is_xxxx() macros.
     * IO space mapping must be initialized before we can do that.
     */
    ret = davinci_init_id(core::ptr::addr_of_mut!(davinci_soc_info));
    if ret < 0 {
        goto_err(ret);
        return;
    }

    return;
}

unsafe fn goto_err(_ret: i32) {
    panic!("davinci_common_init: SoC Initialization failed\n");
}

pub unsafe fn davinci_init_late() {
    davinci_cpufreq_init();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
