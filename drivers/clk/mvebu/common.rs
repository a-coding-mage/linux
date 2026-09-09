// SPDX-License-Identifier: GPL-2.0
/*
 * Marvell EBU SoC common clock handling
 *
 * Copyright (C) 2012 Marvell
 *
 * Gregory CLEMENT <gregory.clement@free-electrons.com>
 * Sebastian Hesselbarth <sebastian.hesselbarth@gmail.com>
 * Andrew Lunn <andrew@lunn.ch>
 *
 */

// Linux kernel dependencies and "common.h" are supplied by other translation units.

const SSCG_SPREAD_DOWN: u32 = 0x0;
const SSCG_SPREAD_UP: u32 = 0x1;
const SSCG_SPREAD_CENTRAL: u32 = 0x2;

#[inline]
fn sscg_conf_mode(reg: u32) -> u32 { (reg >> 16) & 0x3 }
#[inline]
fn sscg_conf_low(reg: u32) -> u32 { (reg >> 8) & 0xff }
#[inline]
fn sscg_conf_high(reg: u32) -> u32 { reg & 0xff }

static mut clk_data: clk_onecell_data = clk_onecell_data { clk_num: 0, clks: core::ptr::null_mut() };

pub unsafe fn kirkwood_fix_sscg_deviation(mut system_clk: u32) -> u32 {
    let sscg_np: *mut device_node = of_find_node_by_name(core::ptr::null_mut(), b"sscg\0".as_ptr() as *const _);
    let sscg_map: *mut core::ffi::c_void;
    let sscg_reg: u32;
    let mut low_bound: i32;
    let mut high_bound: i32;
    let freq_swing_half: u64;

    if sscg_np.is_null() {
        pr_err!("cannot get SSCG register node\n");
        return system_clk;
    }

    sscg_map = of_iomap(sscg_np, 0);
    if sscg_map.is_null() {
        pr_err!("cannot map SSCG register\n");
        of_node_put(sscg_np);
        return system_clk;
    }

    sscg_reg = readl(sscg_map);
    high_bound = sscg_conf_high(sscg_reg) as i32;
    low_bound = sscg_conf_low(sscg_reg) as i32;

    if high_bound - low_bound <= 0 {
        iounmap(sscg_map);
        of_node_put(sscg_np);
        return system_clk;
    }
    /*
     * From Marvell engineer we got the following formula (when
     * this code was written, the datasheet was erroneous)
     * Spread percentage = 1/96 * (H - L) / H
     * H = SSCG_High_Boundary
     * L = SSCG_Low_Boundary
     *
     * As the deviation is half of spread then it lead to the
     * following formula in the code.
     *
     * To avoid an overflow and not lose any significant digit in
     * the same time we have to use a 64 bit integer.
     */
    freq_swing_half = ((high_bound as u64 - low_bound as u64) * system_clk as u64)
        / (2 * 96 * high_bound as u64);

    match sscg_conf_mode(sscg_reg) {
        SSCG_SPREAD_DOWN => system_clk = system_clk.wrapping_sub(freq_swing_half as u32),
        SSCG_SPREAD_UP => system_clk = system_clk.wrapping_add(freq_swing_half as u32),
        SSCG_SPREAD_CENTRAL => {},
        _ => {},
    }

    iounmap(sscg_map);
    of_node_put(sscg_np);
    system_clk
}

pub unsafe fn mvebu_coreclk_setup(np: *mut device_node, desc: *const coreclk_soc_desc) {
    let mut tclk_name: *const core::ffi::c_char = b"tclk\0".as_ptr() as *const _;
    let mut cpuclk_name: *const core::ffi::c_char = b"cpuclk\0".as_ptr() as *const _;
    let base = of_iomap(np, 0);
    if warn_on!(base.is_null()) { return; }

    (*core::ptr::addr_of_mut!(clk_data)).clk_num = 2 + (*desc).num_ratios;
    if !(*desc).get_refclk_freq.is_none() { (*core::ptr::addr_of_mut!(clk_data)).clk_num += 1; }
    (*core::ptr::addr_of_mut!(clk_data)).clks = kzalloc_objs((*clk_data).clks, (*clk_data).clk_num);
    if warn_on!((*clk_data).clks.is_null()) { iounmap(base); return; }

    of_property_read_string_index(np, b"clock-output-names\0".as_ptr() as *const _, 0, &mut tclk_name);
    let rate = ((*desc).get_tclk_freq)(base);
    (*clk_data).clks[0] = clk_register_fixed_rate(core::ptr::null_mut(), tclk_name, core::ptr::null(), 0, rate);
    warn_on!(is_err((*clk_data).clks[0]));

    of_property_read_string_index(np, b"clock-output-names\0".as_ptr() as *const _, 1, &mut cpuclk_name);
    let mut rate = ((*desc).get_cpu_freq)(base);
    if !(*desc).is_sscg_enabled.is_none() && !(*desc).fix_sscg_deviation.is_none() && ((*desc).is_sscg_enabled.unwrap())(base) { rate = ((*desc).fix_sscg_deviation.unwrap())(rate); }
    (*clk_data).clks[1] = clk_register_fixed_rate(core::ptr::null_mut(), cpuclk_name, core::ptr::null(), 0, rate);
    warn_on!(is_err((*clk_data).clks[1]));

    for n in 0..(*desc).num_ratios {
        let mut rclk_name = (*desc).ratios[n].name;
        of_property_read_string_index(np, b"clock-output-names\0".as_ptr() as *const _, 2 + n, &mut rclk_name);
        let mut mult = 0; let mut div = 0;
        ((*desc).get_clk_ratio)(base, (*desc).ratios[n].id, &mut mult, &mut div);
        (*clk_data).clks[2 + n] = clk_register_fixed_factor(core::ptr::null_mut(), rclk_name, cpuclk_name, 0, mult, div);
        warn_on!(is_err((*clk_data).clks[2 + n]));
    }
    if let Some(get_refclk_freq) = (*desc).get_refclk_freq {
        let mut name = b"refclk\0".as_ptr() as *const _;
        of_property_read_string_index(np, b"clock-output-names\0".as_ptr() as *const _, 2 + (*desc).num_ratios, &mut name);
        (*clk_data).clks[2 + (*desc).num_ratios] = clk_register_fixed_rate(core::ptr::null_mut(), name, core::ptr::null(), 0, get_refclk_freq(base));
        warn_on!(is_err((*clk_data).clks[2 + (*desc).num_ratios]));
    }
    iounmap(base);
    of_clk_add_provider(np, of_clk_src_onecell_get, &mut clk_data);
}

// Clock Gating Control
extern "C" { static mut ctrl_gating_lock: spinlock_t; }

#[repr(C)]
struct clk_gating_ctrl { lock: *mut spinlock_t, num_gates: i32, base: *mut core::ffi::c_void, saved_reg: u32, gates: [*mut clk; 0] }
static mut ctrl: *mut clk_gating_ctrl = core::ptr::null_mut();

unsafe extern "C" fn clk_gating_get_src(clkspec: *mut of_phandle_args, _data: *mut core::ffi::c_void) -> *mut clk {
    if (*clkspec).args_count < 1 { return err_ptr(-22); }
    for n in 0..(*ctrl).num_gates { let gate = to_clk_gate(__clk_get_hw((*ctrl).gates[n as usize])); if (*clkspec).args[0] == (*gate).bit_idx { return (*ctrl).gates[n as usize]; } }
    err_ptr(-19)
}
unsafe extern "C" fn mvebu_clk_gating_suspend(_data: *mut core::ffi::c_void) -> i32 { (*ctrl).saved_reg = readl((*ctrl).base); 0 }
unsafe extern "C" fn mvebu_clk_gating_resume(_data: *mut core::ffi::c_void) { writel((*ctrl).saved_reg, (*ctrl).base); }

static mut clk_gate_syscore_ops: syscore_ops = syscore_ops { suspend: Some(mvebu_clk_gating_suspend), resume: Some(mvebu_clk_gating_resume) };
static mut clk_gate_syscore: syscore = syscore { ops: &mut clk_gate_syscore_ops };

pub unsafe fn mvebu_clk_gating_setup(np: *mut device_node, desc: *const clk_gating_soc_desc) {
    let base = of_iomap(np, 0); let mut default_parent: *const core::ffi::c_char = core::ptr::null();
    if !ctrl.is_null() { pr_err!("mvebu-clk-gating: cannot instantiate more than one gateable clock device\n"); return; }
    if warn_on!(base.is_null()) { return; }
    let clk = of_clk_get(np, 0); if !is_err(clk) { default_parent = __clk_get_name(clk); clk_put(clk); }
    let mut n = 0; while !(*desc.add(n as usize)).name.is_null() { n += 1; }
    ctrl = kzalloc_flex(n);
    if warn_on!(ctrl.is_null()) { iounmap(base); return; }
    (*ctrl).num_gates = n; (*ctrl).lock = &mut ctrl_gating_lock; (*ctrl).base = base;
    for i in 0..n { let d = &*desc.add(i as usize); let parent = if !d.parent.is_null() { d.parent } else { default_parent }; (*ctrl).gates[i as usize] = clk_register_gate(core::ptr::null_mut(), d.name, parent, d.flags, base, d.bit_idx, 0, (*ctrl).lock); warn_on!(is_err((*ctrl).gates[i as usize])); }
    of_clk_add_provider(np, Some(clk_gating_get_src), ctrl as *mut _); register_syscore(&mut clk_gate_syscore);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
