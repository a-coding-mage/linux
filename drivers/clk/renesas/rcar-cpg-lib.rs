// SPDX-License-Identifier: GPL-2.0
/*
 * R-Car Gen3 Clock Pulse Generator Library
 *
 * Copyright (C) 2015-2018 Glider bvba
 * Copyright (C) 2019 Renesas Electronics Corp.
 *
 * Based on clk-rcar-gen3.c
 *
 * Copyright (C) 2015 Renesas Electronics Corp.
 */

// Linux kernel dependencies supplied by other translation units.

pub static mut cpg_lock: spinlock_t = spinlock_t { raw: 0 };

extern "C" {
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut c_ulong);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: c_ulong);
    fn readl(addr: *const core::ffi::c_void) -> u32;
    fn writel(value: u32, addr: *mut core::ffi::c_void);
    fn raw_notifier_chain_register(head: *mut raw_notifier_head, nb: *mut notifier_block) -> c_int;
    fn clk_register_divider_table(
        dev: *mut device,
        name: *const c_char,
        parent_name: *const c_char,
        flags: ulong,
        reg: *mut core::ffi::c_void,
        shift: u8,
        width: u8,
        clk_divider_flags: u8,
        table: *const clk_div_table,
        lock: *mut spinlock_t,
    ) -> *mut clk;
    fn clk_register_composite(
        dev: *mut device,
        name: *const c_char,
        parent_names: *const *const c_char,
        num_parents: u8,
        mux_hw: *mut clk_hw,
        mux_ops: *const clk_ops,
        rate_hw: *mut clk_hw,
        rate_ops: *const clk_ops,
        gate_hw: *mut clk_hw,
        gate_ops: *const clk_ops,
        flags: ulong,
    ) -> *mut clk;
    fn kzalloc(size: usize, flags: gfp_t) -> *mut core::ffi::c_void;
    fn kfree(ptr: *mut core::ffi::c_void);
    fn is_err(ptr: *const core::ffi::c_void) -> bool;
    fn err_ptr(error: c_long) -> *mut core::ffi::c_void;
}

pub unsafe fn cpg_reg_modify(reg: *mut core::ffi::c_void, clear: u32, set: u32) {
    let mut flags: c_ulong = 0;
    spin_lock_irqsave(&mut cpg_lock, &mut flags);
    let mut val = readl(reg);
    val &= !clear;
    val |= set;
    writel(val, reg);
    spin_unlock_irqrestore(&mut cpg_lock, flags);
}

unsafe extern "C" fn cpg_simple_notifier_call(
    nb: *mut notifier_block,
    action: c_ulong,
    _data: *mut core::ffi::c_void,
) -> c_int {
    let csn = container_of!(nb, cpg_simple_notifier, nb);

    match action {
        PM_EVENT_SUSPEND => {
            (*csn).saved = readl((*csn).reg);
            NOTIFY_OK
        }
        PM_EVENT_RESUME => {
            writel((*csn).saved, (*csn).reg);
            NOTIFY_OK
        }
        _ => NOTIFY_DONE,
    }
}

pub unsafe fn cpg_simple_notifier_register(
    notifiers: *mut raw_notifier_head,
    csn: *mut cpg_simple_notifier,
) {
    (*csn).nb.notifier_call = Some(cpg_simple_notifier_call);
    raw_notifier_chain_register(notifiers, &mut (*csn).nb);
}

/*
 * SDn Clock
 */

pub const SDnSRCFC_SHIFT: u32 = 2;
pub const STPnHCK: u32 = 1 << (9 - SDnSRCFC_SHIFT);

static CPG_SDH_DIV_TABLE: [clk_div_table; 11] = [
    clk_div_table { val: 0, div: 1 },
    clk_div_table { val: 1, div: 2 },
    clk_div_table { val: STPnHCK | 2, div: 4 },
    clk_div_table { val: STPnHCK | 3, div: 8 },
    clk_div_table { val: STPnHCK | 4, div: 16 },
    clk_div_table { val: STPnHCK, div: 1 },
    clk_div_table { val: STPnHCK | 1, div: 2 },
    clk_div_table { val: 2, div: 4 },
    clk_div_table { val: 3, div: 8 },
    clk_div_table { val: 4, div: 16 },
    clk_div_table { val: 0, div: 0 },
];

pub unsafe fn cpg_sdh_clk_register(
    name: *const c_char,
    sdnckcr: *mut core::ffi::c_void,
    parent_name: *const c_char,
    notifiers: *mut raw_notifier_head,
) -> *mut clk {
    let csn = kzalloc(core::mem::size_of::<cpg_simple_notifier>(), GFP_KERNEL)
        as *mut cpg_simple_notifier;
    if csn.is_null() {
        return err_ptr(-ENOMEM as c_long) as *mut clk;
    }
    (*csn).reg = sdnckcr;
    let clk = clk_register_divider_table(
        core::ptr::null_mut(), name, parent_name, 0, sdnckcr,
        SDnSRCFC_SHIFT as u8, 8, 0, CPG_SDH_DIV_TABLE.as_ptr(), &mut cpg_lock,
    );
    if is_err(clk as *const core::ffi::c_void) {
        kfree(csn as *mut core::ffi::c_void);
        return clk;
    }
    cpg_simple_notifier_register(notifiers, csn);
    clk
}

static CPG_SD_DIV_TABLE: [clk_div_table; 3] = [
    clk_div_table { val: 0, div: 2 },
    clk_div_table { val: 1, div: 4 },
    clk_div_table { val: 0, div: 0 },
];

pub unsafe fn cpg_sd_clk_register(
    name: *const c_char,
    sdnckcr: *mut core::ffi::c_void,
    parent_name: *const c_char,
) -> *mut clk {
    clk_register_divider_table(
        core::ptr::null_mut(), name, parent_name, 0, sdnckcr,
        0, 2, 0, CPG_SD_DIV_TABLE.as_ptr(), &mut cpg_lock,
    )
}

#[repr(C)]
pub struct rpc_clock {
    pub div: clk_divider,
    pub gate: clk_gate,
    /* One notifier covers both RPC and RPCD2 clocks as they are both
     * controlled by the same RPCCKCR register... */
    pub csn: cpg_simple_notifier,
}

static CPG_RPC_DIV_TABLE: [clk_div_table; 5] = [
    clk_div_table { val: 1, div: 2 },
    clk_div_table { val: 3, div: 4 },
    clk_div_table { val: 5, div: 6 },
    clk_div_table { val: 7, div: 8 },
    clk_div_table { val: 0, div: 0 },
];

pub unsafe fn cpg_rpc_clk_register(
    name: *const c_char, rpcckcr: *mut core::ffi::c_void,
    parent_name: *const c_char, notifiers: *mut raw_notifier_head,
) -> *mut clk {
    let rpc = kzalloc(core::mem::size_of::<rpc_clock>(), GFP_KERNEL) as *mut rpc_clock;
    if rpc.is_null() { return err_ptr(-ENOMEM as c_long) as *mut clk; }
    (*rpc).div.reg = rpcckcr;
    (*rpc).div.width = 3;
    (*rpc).div.table = CPG_RPC_DIV_TABLE.as_ptr();
    (*rpc).div.lock = &mut cpg_lock;
    (*rpc).gate.reg = rpcckcr;
    (*rpc).gate.bit_idx = 8;
    (*rpc).gate.flags = CLK_GATE_SET_TO_DISABLE;
    (*rpc).gate.lock = &mut cpg_lock;
    (*rpc).csn.reg = rpcckcr;
    let parents = &parent_name as *const *const c_char;
    let clk = clk_register_composite(
        core::ptr::null_mut(), name, parents, 1, core::ptr::null_mut(), core::ptr::null(),
        &mut (*rpc).div.hw, &clk_divider_ops, &mut (*rpc).gate.hw, &clk_gate_ops,
        CLK_SET_RATE_PARENT,
    );
    if is_err(clk as *const core::ffi::c_void) {
        kfree(rpc as *mut core::ffi::c_void);
        return clk;
    }
    cpg_simple_notifier_register(notifiers, &mut (*rpc).csn);
    clk
}

#[repr(C)]
pub struct rpcd2_clock {
    pub fixed: clk_fixed_factor,
    pub gate: clk_gate,
}

pub unsafe fn cpg_rpcd2_clk_register(
    name: *const c_char, rpcckcr: *mut core::ffi::c_void, parent_name: *const c_char,
) -> *mut clk {
    let rpcd2 = kzalloc(core::mem::size_of::<rpcd2_clock>(), GFP_KERNEL) as *mut rpcd2_clock;
    if rpcd2.is_null() { return err_ptr(-ENOMEM as c_long) as *mut clk; }
    (*rpcd2).fixed.mult = 1;
    (*rpcd2).fixed.div = 2;
    (*rpcd2).gate.reg = rpcckcr;
    (*rpcd2).gate.bit_idx = 9;
    (*rpcd2).gate.flags = CLK_GATE_SET_TO_DISABLE;
    (*rpcd2).gate.lock = &mut cpg_lock;
    let parents = &parent_name as *const *const c_char;
    let clk = clk_register_composite(
        core::ptr::null_mut(), name, parents, 1, core::ptr::null_mut(), core::ptr::null(),
        &mut (*rpcd2).fixed.hw, &clk_fixed_factor_ops, &mut (*rpcd2).gate.hw, &clk_gate_ops,
        CLK_SET_RATE_PARENT,
    );
    if is_err(clk as *const core::ffi::c_void) { kfree(rpcd2 as *mut core::ffi::c_void); }
    clk
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
