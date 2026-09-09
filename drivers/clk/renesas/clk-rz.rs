// SPDX-License-Identifier: GPL-2.0
/*
 * RZ/A1 Core CPG Clocks
 *
 * Copyright (C) 2013 Ideas On Board SPRL
 * Copyright (C) 2014 Wolfram Sang, Sang Engineering <wsa@sang-engineering.com>
 */

// Dependencies supplied by the surrounding kernel translation unit.

const CPG_FRQCR: usize = 0x10;
const CPG_FRQCR2: usize = 0x14;

const PPR0: usize = 0xFCFE3200;
const PIBC0: usize = 0xFCFE7000;

#[inline]
const fn md_clk(x: u16) -> u16 {
    (x >> 2) & 1 // P0_2
}

/* -----------------------------------------------------------------------------
 * Initialization
 */

unsafe fn rz_cpg_read_mode_pins() -> u16 {
    let ppr0: *mut core::ffi::c_void = ioremap(PPR0, 2);
    let pibc0: *mut core::ffi::c_void = ioremap(PIBC0, 2);
    bug_on(ppr0.is_null() || pibc0.is_null());
    iowrite16(4, pibc0); // enable input buffer
    let modes = ioread16(ppr0);
    iounmap(ppr0);
    iounmap(pibc0);

    modes
}

unsafe fn rz_cpg_register_clock(
    np: *mut device_node,
    base: *mut core::ffi::c_void,
    name: *const core::ffi::c_char,
) -> *mut clk {
    let mut val: u32;
    static FRQCR_TAB: [u32; 4] = [3, 2, 0, 1];

    if strcmp(name, c"pll".as_ptr()) == 0 {
        let cpg_mode: u32 = md_clk(rz_cpg_read_mode_pins()) as u32;
        let parent_name = of_clk_get_parent_name(np, cpg_mode);
        let mult: u32 = if cpg_mode != 0 { 32 / 4 } else { 30 };

        return clk_register_fixed_factor(core::ptr::null_mut(), name, parent_name, 0, mult, 1);
    }

    /* If mapping regs failed, skip non-pll clocks. System will boot anyhow */
    if base.is_null() {
        return err_ptr(-ENXIO);
    }

    /* FIXME:"i" and "g" are variable clocks with non-integer dividers (e.g. 2/3)
     * and the constraint that always g <= i. To get the rz platform started,
     * let them run at fixed current speed and implement the details later.
     */
    if strcmp(name, c"i".as_ptr()) == 0 {
        val = (readl(base.add(CPG_FRQCR / core::mem::size_of::<u32>())) >> 8) & 3;
    } else if strcmp(name, c"g".as_ptr()) == 0 {
        val = readl(base.add(CPG_FRQCR2 / core::mem::size_of::<u32>())) & 3;
    } else {
        return err_ptr(-EINVAL);
    }

    let mult = FRQCR_TAB[val as usize];
    clk_register_fixed_factor(core::ptr::null_mut(), name, c"pll".as_ptr(), 0, mult, 3)
}

unsafe fn rz_cpg_clocks_init(np: *mut device_node) {
    let mut data: *mut clk_onecell_data;
    let mut clks: *mut *mut clk;
    let base: *mut core::ffi::c_void;
    let num_clks: i32;

    num_clks = of_property_count_strings(np, c"clock-output-names".as_ptr());
    if warn(num_clks <= 0, c"can't count CPG clocks\n".as_ptr()) {
        return;
    }

    data = kzalloc_obj::<clk_onecell_data>();
    clks = kzalloc_objs::<*mut clk>(num_clks as usize);
    bug_on(data.is_null() || clks.is_null());

    (*data).clks = clks;
    (*data).clk_num = num_clks as u32;

    base = of_iomap(np, 0);

    for i in 0..num_clks as usize {
        let mut name: *const core::ffi::c_char = core::ptr::null();
        let clk: *mut clk;

        of_property_read_string_index(np, c"clock-output-names".as_ptr(), i, &mut name);

        clk = rz_cpg_register_clock(np, base, name);
        if is_err(clk) {
            pr_err(c"%s: failed to register %pOFn %s clock (%ld)\n".as_ptr(),
                   c"rz_cpg_clocks_init".as_ptr(), np, name, ptr_err(clk));
        } else {
            *clks.add(i) = clk;
        }
    }

    of_clk_add_provider(np, of_clk_src_onecell_get, data);

    cpg_mstp_add_clk_domain(np);
}

// CLK_OF_DECLARE(rz_cpg_clks, "renesas,rz-cpg-clocks", rz_cpg_clocks_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
