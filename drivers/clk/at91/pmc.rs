// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Copyright (C) 2013 Boris BREZILLON <b.brezillon@overkiz.com>
 */

// Declarations supplied by the Linux clock, device-tree, regmap, syscore,
// processor, and local PMC dependencies are intentionally left external.

const PMC_MAX_IDS: usize = 128;
const PMC_MAX_PCKS: usize = 8;

pub unsafe fn of_at91_get_clk_range(
    np: *mut device_node,
    propname: *const c_char,
    range: *mut clk_range,
) -> c_int {
    let mut min: u32 = 0;
    let mut max: u32 = 0;
    let mut ret: c_int;

    ret = of_property_read_u32_index(np, propname, 0, &mut min);
    if ret != 0 {
        return ret;
    }

    ret = of_property_read_u32_index(np, propname, 1, &mut max);
    if ret != 0 {
        return ret;
    }

    if !range.is_null() {
        (*range).min = min;
        (*range).max = max;
    }

    0
}

pub unsafe fn of_clk_hw_pmc_get(
    clkspec: *mut of_phandle_args,
    data: *mut c_void,
) -> *mut clk_hw {
    let type_ = (*clkspec).args[0];
    let idx = (*clkspec).args[1];
    let pmc_data = data as *mut pmc_data;

    match type_ {
        PMC_TYPE_CORE => {
            if idx < (*pmc_data).ncore {
                return *(*pmc_data).chws.add(idx as usize);
            }
        }
        PMC_TYPE_SYSTEM => {
            if idx < (*pmc_data).nsystem {
                return *(*pmc_data).shws.add(idx as usize);
            }
        }
        PMC_TYPE_PERIPHERAL => {
            if idx < (*pmc_data).nperiph {
                return *(*pmc_data).phws.add(idx as usize);
            }
        }
        PMC_TYPE_GCK => {
            if idx < (*pmc_data).ngck {
                return *(*pmc_data).ghws.add(idx as usize);
            }
        }
        PMC_TYPE_PROGRAMMABLE => {
            if idx < (*pmc_data).npck {
                return *(*pmc_data).pchws.add(idx as usize);
            }
        }
        _ => {}
    }

    pr_err!("{}: invalid type ({}) or index ({})\n", "of_clk_hw_pmc_get", type_, idx);
    ERR_PTR(-EINVAL)
}

pub unsafe fn pmc_data_allocate(
    ncore: c_uint,
    nsystem: c_uint,
    nperiph: c_uint,
    ngck: c_uint,
    npck: c_uint,
) -> *mut pmc_data {
    let num_clks = ncore + nsystem + nperiph + ngck + npck;
    let pmc_data = kzalloc_flex!(pmc_data, hwtable, num_clks);
    if pmc_data.is_null() {
        return core::ptr::null_mut();
    }

    (*pmc_data).ncore = ncore;
    (*pmc_data).chws = (*pmc_data).hwtable;
    (*pmc_data).nsystem = nsystem;
    (*pmc_data).shws = (*pmc_data).chws.add(ncore as usize);
    (*pmc_data).nperiph = nperiph;
    (*pmc_data).phws = (*pmc_data).shws.add(nsystem as usize);
    (*pmc_data).ngck = ngck;
    (*pmc_data).ghws = (*pmc_data).phws.add(nperiph as usize);
    (*pmc_data).npck = npck;
    (*pmc_data).pchws = (*pmc_data).ghws.add(ngck as usize);

    pmc_data
}

#[cfg(CONFIG_PM)]
static mut at91_pmc_backup_suspend: *mut c_void = core::ptr::null_mut();

#[cfg(CONFIG_PM)]
unsafe fn at91_pmc_suspend(_data: *mut c_void) -> c_int {
    if at91_pmc_backup_suspend.is_null() {
        return 0;
    }
    let backup = readl_relaxed(at91_pmc_backup_suspend);
    if backup == 0 { return 0; }
    clk_save_context()
}

#[cfg(CONFIG_PM)]
unsafe fn at91_pmc_resume(_data: *mut c_void) {
    if at91_pmc_backup_suspend.is_null() { return; }
    let backup = readl_relaxed(at91_pmc_backup_suspend);
    if backup == 0 { return; }
    clk_restore_context();
}

#[cfg(CONFIG_PM)]
static pmc_syscore_ops: syscore_ops = syscore_ops {
    suspend: Some(at91_pmc_suspend),
    resume: Some(at91_pmc_resume),
};

#[cfg(CONFIG_PM)]
static mut pmc_syscore: syscore = syscore { ops: &pmc_syscore_ops };

#[cfg(CONFIG_PM)]
static pmc_dt_ids: [of_device_id; 4] = [
    of_device_id { compatible: c"atmel,sama5d2-pmc" },
    of_device_id { compatible: c"microchip,sama7g5-pmc" },
    of_device_id { compatible: c"microchip,sama7d65-pmc" },
    of_device_id { compatible: core::ptr::null() },
];

#[cfg(CONFIG_PM)]
unsafe fn pmc_register_ops() -> c_int {
    let mut np = of_find_matching_node(core::ptr::null_mut(), pmc_dt_ids.as_ptr());
    if np.is_null() { return -ENODEV; }
    if !of_device_is_available(np) {
        of_node_put(np);
        return -ENODEV;
    }
    of_node_put(np);

    np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(), c"atmel,sama5d2-securam");
    if np.is_null() { return -ENODEV; }
    if !of_device_is_available(np) {
        of_node_put(np);
        return -ENODEV;
    }
    at91_pmc_backup_suspend = of_iomap(np, 0);
    of_node_put(np);
    if at91_pmc_backup_suspend.is_null() {
        pr_warn!("pmc_register_ops(): unable to map securam\n");
        return -ENOMEM;
    }
    register_syscore(&mut pmc_syscore);
    0
}

// This has to happen before arch_initcall because of the tcb_clksrc driver.
#[cfg(CONFIG_PM)]
postcore_initcall!(pmc_register_ops);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
