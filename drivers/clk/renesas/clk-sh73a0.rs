// SPDX-License-Identifier: GPL-2.0
/*
 * sh73a0 Core CPG Clocks
 *
 * Copyright (C) 2014  Ulrich Hecht
 */

// Dependencies supplied by the surrounding kernel/Rust translation.

#[repr(C)]
pub struct Sh73a0Cpg {
    pub data: ClkOnecellData,
    pub lock: Spinlock,
}

pub const CPG_FRQCRA: u32 = 0x00;
pub const CPG_FRQCRB: u32 = 0x04;
pub const CPG_SD0CKCR: u32 = 0x74;
pub const CPG_SD1CKCR: u32 = 0x78;
pub const CPG_SD2CKCR: u32 = 0x7c;
pub const CPG_PLLECR: u32 = 0xd0;
pub const CPG_PLL0CR: u32 = 0xd8;
pub const CPG_PLL1CR: u32 = 0x28;
pub const CPG_PLL2CR: u32 = 0x2c;
pub const CPG_PLL3CR: u32 = 0xdc;
pub const CPG_CKSCR: u32 = 0xc0;
pub const CPG_DSI0PHYCR: u32 = 0x6c;
pub const CPG_DSI1PHYCR: u32 = 0x70;

#[repr(C)]
pub struct Div4Clk {
    pub name: *const core::ffi::c_char,
    pub parent: *const core::ffi::c_char,
    pub reg: u32,
    pub shift: u32,
}

static DIV4_CLKS: &[Div4Clk] = &[
    Div4Clk { name: c"zg".as_ptr(), parent: c"pll0".as_ptr(), reg: CPG_FRQCRA, shift: 16 },
    Div4Clk { name: c"m3".as_ptr(), parent: c"pll1".as_ptr(), reg: CPG_FRQCRA, shift: 12 },
    Div4Clk { name: c"b".as_ptr(), parent: c"pll1".as_ptr(), reg: CPG_FRQCRA, shift: 8 },
    Div4Clk { name: c"m1".as_ptr(), parent: c"pll1".as_ptr(), reg: CPG_FRQCRA, shift: 4 },
    Div4Clk { name: c"m2".as_ptr(), parent: c"pll1".as_ptr(), reg: CPG_FRQCRA, shift: 0 },
    Div4Clk { name: c"zx".as_ptr(), parent: c"pll1".as_ptr(), reg: CPG_FRQCRB, shift: 12 },
    Div4Clk { name: c"hp".as_ptr(), parent: c"pll1".as_ptr(), reg: CPG_FRQCRB, shift: 4 },
];

static DIV4_DIV_TABLE: &[ClkDivTable] = &[
    ClkDivTable { val: 0, div: 2 }, ClkDivTable { val: 1, div: 3 },
    ClkDivTable { val: 2, div: 4 }, ClkDivTable { val: 3, div: 6 },
    ClkDivTable { val: 4, div: 8 }, ClkDivTable { val: 5, div: 12 },
    ClkDivTable { val: 6, div: 16 }, ClkDivTable { val: 7, div: 18 },
    ClkDivTable { val: 8, div: 24 }, ClkDivTable { val: 10, div: 36 },
    ClkDivTable { val: 11, div: 48 }, ClkDivTable { val: 12, div: 7 },
    ClkDivTable { val: 0, div: 0 },
];

static Z_DIV_TABLE: &[ClkDivTable] = &[
    // ZSEL == 0
    ClkDivTable { val: 0, div: 1 }, ClkDivTable { val: 1, div: 1 },
    ClkDivTable { val: 2, div: 1 }, ClkDivTable { val: 3, div: 1 },
    ClkDivTable { val: 4, div: 1 }, ClkDivTable { val: 5, div: 1 },
    ClkDivTable { val: 6, div: 1 }, ClkDivTable { val: 7, div: 1 },
    ClkDivTable { val: 8, div: 1 }, ClkDivTable { val: 9, div: 1 },
    ClkDivTable { val: 10, div: 1 }, ClkDivTable { val: 11, div: 1 },
    ClkDivTable { val: 12, div: 1 }, ClkDivTable { val: 13, div: 1 },
    ClkDivTable { val: 14, div: 1 }, ClkDivTable { val: 15, div: 1 },
    // ZSEL == 1
    ClkDivTable { val: 16, div: 2 }, ClkDivTable { val: 17, div: 3 },
    ClkDivTable { val: 18, div: 4 }, ClkDivTable { val: 19, div: 6 },
    ClkDivTable { val: 20, div: 8 }, ClkDivTable { val: 21, div: 12 },
    ClkDivTable { val: 22, div: 16 }, ClkDivTable { val: 24, div: 24 },
    ClkDivTable { val: 27, div: 48 }, ClkDivTable { val: 0, div: 0 },
];

pub unsafe fn sh73a0_cpg_register_clock(
    np: *mut DeviceNode, cpg: *mut Sh73a0Cpg, base: *mut u8, name: *const core::ffi::c_char,
) -> *mut Clk {
    let mut table: *const ClkDivTable = core::ptr::null();
    let mut shift: u32 = 0;
    let mut reg: u32 = 0;
    let mut width: u32 = 0;
    let mut parent_name: *const core::ffi::c_char = core::ptr::null();
    let mut mult: u32 = 1;
    let mut div: u32 = 1;

    if c_str_eq(name, c"main".as_ptr()) {
        let parent_idx = (readl(base.add(CPG_CKSCR as usize)) >> 28) & 3;
        parent_name = of_clk_get_parent_name(np, parent_idx >> 1);
        div = (parent_idx & 1) + 1;
    } else if c_str_n_eq(name, c"pll".as_ptr(), 3) {
        let mut enable_reg = base;
        let enable_bit = (*name.add(3) as u8 - b'0') as u32;
        parent_name = c"main".as_ptr();
        enable_reg = match enable_bit {
            0 => enable_reg.add(CPG_PLL0CR as usize),
            1 => enable_reg.add(CPG_PLL1CR as usize),
            2 => enable_reg.add(CPG_PLL2CR as usize),
            3 => enable_reg.add(CPG_PLL3CR as usize),
            _ => return err_ptr(-22),
        };
        if readl(base.add(CPG_PLLECR as usize)) & (1u32 << enable_bit) != 0 {
            mult = ((readl(enable_reg) >> 24) & 0x3f) + 1;
            // handle CFG bit for PLL1 and PLL2
            if (enable_bit == 1 || enable_bit == 2) && readl(enable_reg) & (1 << 20) != 0 {
                mult *= 2;
            }
        }
    } else if c_str_eq(name, c"dsi0phy".as_ptr()) || c_str_eq(name, c"dsi1phy".as_ptr()) {
        let phy_no = (*name.add(3) as u8 - b'0') as u32;
        let dsi_reg = base.add(if phy_no != 0 { CPG_DSI1PHYCR } else { CPG_DSI0PHYCR } as usize);
        parent_name = if phy_no != 0 { c"dsi1pck".as_ptr() } else { c"dsi0pck".as_ptr() };
        mult = readl(dsi_reg);
        mult = if mult & 0x8000 == 0 { 1 } else { (mult & 0x3f) + 1 };
    } else if c_str_eq(name, c"z".as_ptr()) {
        parent_name = c"pll0".as_ptr(); table = Z_DIV_TABLE.as_ptr(); reg = CPG_FRQCRB; shift = 24; width = 5;
    } else {
        let mut found = false;
        for c in DIV4_CLKS {
            if c_str_eq(name, c.name) {
                parent_name = c.parent; table = DIV4_DIV_TABLE.as_ptr(); reg = c.reg; shift = c.shift; width = 4; found = true; break;
            }
        }
        if !found { return err_ptr(-22); }
    }

    if table.is_null() {
        clk_register_fixed_factor(core::ptr::null_mut(), name, parent_name, 0, mult, div)
    } else {
        clk_register_divider_table(core::ptr::null_mut(), name, parent_name, 0, base.add(reg as usize), shift, width, 0, table, &mut (*cpg).lock)
    }
}

pub unsafe fn sh73a0_cpg_clocks_init(np: *mut DeviceNode) {
    let num_clks = of_property_count_strings(np, c"clock-output-names".as_ptr());
    if num_clks < 0 { pr_err(c"%s: failed to count clocks\n".as_ptr(), c"sh73a0_cpg_clocks_init".as_ptr()); return; }
    let cpg = kzalloc_cpg();
    let clks = kzalloc_clks(num_clks as usize);
    if cpg.is_null() || clks.is_null() { return; }
    spin_lock_init(&mut (*cpg).lock);
    (*cpg).data.clks = clks;
    (*cpg).data.clk_num = num_clks;
    let base = of_iomap(np, 0);
    if base.is_null() { return; }
    writel(0x108, base.add(CPG_SD0CKCR as usize)); writel(0x108, base.add(CPG_SD1CKCR as usize)); writel(0x108, base.add(CPG_SD2CKCR as usize));
    for i in 0..num_clks as usize {
        let name = of_property_read_string_index(np, c"clock-output-names".as_ptr(), i);
        let clk = sh73a0_cpg_register_clock(np, cpg, base, name);
        if is_err(clk) { pr_err(c"%s: failed to register %s clock\n".as_ptr(), c"sh73a0_cpg_clocks_init".as_ptr(), name); } else { *clks.add(i) = clk; }
    }
    of_clk_add_provider(np, of_clk_src_onecell_get, &mut (*cpg).data);
}

// CLK_OF_DECLARE(sh73a0_cpg_clks, "renesas,sh73a0-cpg-clocks", sh73a0_cpg_clocks_init)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
