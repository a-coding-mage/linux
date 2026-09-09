// SPDX-License-Identifier: GPL-2.0
/*
 * R-Car MSTP clocks
 *
 * Copyright (C) 2013 Ideas On Board SPRL
 * Copyright (C) 2015 Glider bvba
 *
 * Contact: Laurent Pinchart <laurent.pinchart@ideasonboard.com>
 */

// Kernel dependencies supplied by other translation units.

/*
 * MSTP clocks. We can't use standard gate clocks as we need to poll on the
 * status register when enabling the clock.
 */

const MSTP_MAX_CLOCKS: usize = 32;

/**
 * struct mstp_clock_group - MSTP gating clocks group
 *
 * @data: clock specifier translation for clocks in this group
 * @smstpcr: module stop control register
 * @mstpsr: module stop status register (optional)
 * @lock: protects writes to SMSTPCR
 * @width_8bit: registers are 8-bit, not 32-bit
 * @clks: clocks in this group
 */
#[repr(C)]
struct mstp_clock_group {
    data: clk_onecell_data,
    smstpcr: *mut core::ffi::c_void,
    mstpsr: *mut core::ffi::c_void,
    lock: spinlock_t,
    width_8bit: bool,
    clks: [*mut clk; 0],
}

/**
 * struct mstp_clock - MSTP gating clock
 * @hw: handle between common and hardware-specific interfaces
 * @bit_index: control bit index
 * @group: MSTP clocks group
 */
#[repr(C)]
struct mstp_clock {
    hw: clk_hw,
    bit_index: u32,
    group: *mut mstp_clock_group,
}

#[inline]
unsafe fn cpg_mstp_read(group: *mut mstp_clock_group, reg: *mut u32) -> u32 {
    if (*group).width_8bit { readb(reg as *const core::ffi::c_void) as u32 } else { readl(reg as *const core::ffi::c_void) }
}

#[inline]
unsafe fn cpg_mstp_write(group: *mut mstp_clock_group, val: u32, reg: *mut u32) {
    if (*group).width_8bit { writeb(val as u8, reg as *mut core::ffi::c_void); } else { writel(val, reg as *mut core::ffi::c_void); }
}

unsafe fn cpg_mstp_clock_endisable(hw: *mut clk_hw, enable: bool) -> i32 {
    let clock = container_of_mstp_clock(hw);
    let group = (*clock).group;
    let bitmask = 1u32 << (*clock).bit_index;
    let mut flags: ulong = 0;
    let value: u32;
    let ret: i32;

    spin_lock_irqsave(&mut (*group).lock, &mut flags);

    value = cpg_mstp_read(group, (*group).smstpcr);
    let value = if enable { value & !bitmask } else { value | bitmask };
    cpg_mstp_write(group, value, (*group).smstpcr);

    if (*group).mstpsr.is_null() {
        /* dummy read to ensure write has completed */
        cpg_mstp_read(group, (*group).smstpcr);
        barrier_data((*group).smstpcr);
    }

    spin_unlock_irqrestore(&mut (*group).lock, flags);

    if !enable || (*group).mstpsr.is_null() { return 0; }

    /* group->width_8bit is always false if group->mstpsr is present */
    ret = readl_poll_timeout_atomic((*group).mstpsr, &mut value, (value & bitmask) == 0, 0, 10);
    if ret != 0 {
        pr_err!("{}: failed to enable {:p}[{}]\n", cstr!("cpg_mstp_clock_endisable"), (*group).smstpcr, (*clock).bit_index);
    }
    ret
}

unsafe fn cpg_mstp_clock_enable(hw: *mut clk_hw) -> i32 { cpg_mstp_clock_endisable(hw, true) }
unsafe fn cpg_mstp_clock_disable(hw: *mut clk_hw) { cpg_mstp_clock_endisable(hw, false); }

unsafe fn cpg_mstp_clock_is_enabled(hw: *mut clk_hw) -> i32 {
    let clock = container_of_mstp_clock(hw);
    let group = (*clock).group;
    let value = if !(*group).mstpsr.is_null() { cpg_mstp_read(group, (*group).mstpsr) } else { cpg_mstp_read(group, (*group).smstpcr) };
    if (value & (1u32 << (*clock).bit_index)) == 0 { 1 } else { 0 }
}

static cpg_mstp_clock_ops: clk_ops = clk_ops {
    enable: Some(cpg_mstp_clock_enable),
    disable: Some(cpg_mstp_clock_disable),
    is_enabled: Some(cpg_mstp_clock_is_enabled),
};

unsafe fn cpg_mstp_clock_register(name: *const c_char, parent_name: *const c_char, index: c_uint, group: *mut mstp_clock_group) -> *mut clk {
    let mut init: clk_init_data = core::mem::zeroed();
    let clock = kzalloc_mstp_clock();
    if clock.is_null() { return err_ptr::<clk>(-12); }

    init.name = name;
    init.ops = &cpg_mstp_clock_ops;
    init.flags = CLK_SET_RATE_PARENT;
    /* INTC-SYS is the module clock of the GIC, and must not be disabled */
    if strcmp(name, cstr!("intc-sys")) == 0 {
        pr_debug!("MSTP {} setting CLK_IS_CRITICAL\n", name);
        init.flags |= CLK_IS_CRITICAL;
    }
    init.parent_names = &parent_name;
    init.num_parents = 1;

    (*clock).bit_index = index;
    (*clock).group = group;
    (*clock).hw.init = &init;

    let clk = clk_register(core::ptr::null_mut(), &mut (*clock).hw);
    if is_err(clk) { kfree(clock as *mut core::ffi::c_void); }
    clk
}

unsafe extern "C" fn cpg_mstp_clocks_init(np: *mut device_node) {
    let group = kzalloc_mstp_group(MSTP_MAX_CLOCKS);
    if group.is_null() { return; }

    let clks = (*group).clks.as_mut_ptr();
    spin_lock_init(&mut (*group).lock);
    (*group).data.clks = clks;
    (*group).smstpcr = of_iomap(np, 0);
    (*group).mstpsr = of_iomap(np, 1);

    if (*group).smstpcr.is_null() {
        pr_err!("{}: failed to remap SMSTPCR\n", cstr!("cpg_mstp_clocks_init"));
        kfree(group as *mut core::ffi::c_void);
        return;
    }
    if of_device_is_compatible(np, cstr!("renesas,r7s72100-mstp-clocks")) { (*group).width_8bit = true; }
    for i in 0..MSTP_MAX_CLOCKS { *clks.add(i) = err_ptr(-2); }

    let idxname = if of_property_present(np, cstr!("clock-indices")) { cstr!("clock-indices") } else { cstr!("renesas,clock-indices") };
    for i in 0..MSTP_MAX_CLOCKS {
        let mut parent_name: *const c_char = core::ptr::null();
        let mut name: *const c_char = core::ptr::null();
        let mut clkidx: u32 = 0;
        /* Skip clocks with no name. */
        let ret = of_property_read_string_index(np, cstr!("clock-output-names"), i as c_int, &mut name);
        if ret < 0 || strlen(name) == 0 { continue; }
        parent_name = of_clk_get_parent_name(np, i as c_int);
        let ret = of_property_read_u32_index(np, idxname, i as c_int, &mut clkidx);
        if parent_name.is_null() || ret < 0 { break; }
        if clkidx >= MSTP_MAX_CLOCKS as u32 { pr_err!("invalid clock index {}\n", clkidx); continue; }
        *clks.add(clkidx as usize) = cpg_mstp_clock_register(name, parent_name, clkidx, group);
        if !is_err(*clks.add(clkidx as usize)) { (*group).data.clk_num = core::cmp::max((*group).data.clk_num, clkidx + 1); }
        else { pr_err!("failed to register clock\n"); }
    }
    of_clk_add_provider(np, of_clk_src_onecell_get, &mut (*group).data);
}

unsafe fn cpg_mstp_attach_dev(_unused: *mut generic_pm_domain, dev: *mut device) -> i32 {
    let np = (*dev).of_node;
    let mut clkspec: of_phandle_args = core::mem::zeroed();
    let mut i = 0;
    loop {
        if of_parse_phandle_with_args(np, cstr!("clocks"), cstr!("#clock-cells"), i, &mut clkspec) != 0 { return 0; }
        if of_device_is_compatible(clkspec.np, cstr!("renesas,cpg-mstp-clocks")) || of_node_name_eq(clkspec.np, cstr!("zb_clk")) { break; }
        of_node_put(clkspec.np); i += 1;
    }
    let clk = of_clk_get_from_provider(&clkspec);
    of_node_put(clkspec.np);
    if is_err(clk) { return ptr_err(clk); }
    let mut error = pm_clk_create(dev);
    if error != 0 { clk_put(clk); return error; }
    error = pm_clk_add_clk(dev, clk);
    if error != 0 { pm_clk_destroy(dev); clk_put(clk); }
    error
}

unsafe fn cpg_mstp_detach_dev(_unused: *mut generic_pm_domain, dev: *mut device) { if !pm_clk_no_clocks(dev) { pm_clk_destroy(dev); } }

static mut cpg_mstp_pd_np: *mut device_node = core::ptr::null_mut();
static mut cpg_mstp_pd_genpd: *mut generic_pm_domain = core::ptr::null_mut();

unsafe fn cpg_mstp_add_clk_domain(np: *mut device_node) {
    let mut ncells = 0u32;
    if of_property_read_u32(np, cstr!("#power-domain-cells"), &mut ncells) != 0 { pr_warn!("power domain lacks cells\n"); return; }
    let pd = kzalloc_generic_pm_domain();
    if pd.is_null() { return; }
    (*pd).name = (*np).name;
    (*pd).flags = GENPD_FLAG_PM_CLK | GENPD_FLAG_ALWAYS_ON | GENPD_FLAG_ACTIVE_WAKEUP;
    (*pd).attach_dev = Some(cpg_mstp_attach_dev);
    (*pd).detach_dev = Some(cpg_mstp_detach_dev);
    pm_genpd_init(pd, &pm_domain_always_on_gov, false);
    cpg_mstp_pd_np = of_node_get(np);
    cpg_mstp_pd_genpd = pd;
}

unsafe fn cpg_mstp_pd_init_provider() -> i32 {
    if cpg_mstp_pd_np.is_null() { return -19; }
    let error = of_genpd_add_provider_simple(cpg_mstp_pd_np, cpg_mstp_pd_genpd);
    of_node_put(cpg_mstp_pd_np);
    error
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
