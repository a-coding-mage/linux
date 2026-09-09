// SPDX-License-Identifier: GPL-2.0
/*
 * r8a73a4 Core CPG Clocks
 *
 * Copyright (C) 2014  Ulrich Hecht
 */

// Linux kernel dependencies are supplied by the surrounding translation.

#[repr(C)]
struct R8a73a4Cpg {
    data: ClkOnecellData,
    lock: SpinlockT,
}

const CPG_CKSCR: usize = 0xc0;
const CPG_FRQCRA: usize = 0x00;
const CPG_FRQCRB: usize = 0x04;
const CPG_FRQCRC: usize = 0xe0;
const CPG_PLL0CR: usize = 0xd8;
const CPG_PLL1CR: usize = 0x28;
const CPG_PLL2CR: usize = 0x2c;
const CPG_PLL2HCR: usize = 0xe4;
const CPG_PLL2SCR: usize = 0xf4;

#[repr(C)]
struct Div4Clk {
    name: *const c_char,
    reg: u32,
    shift: u32,
}

static mut DIV4_CLKS: [Div4Clk; 11] = [
    Div4Clk { name: c"i".as_ptr(), reg: CPG_FRQCRA as u32, shift: 20 },
    Div4Clk { name: c"m3".as_ptr(), reg: CPG_FRQCRA as u32, shift: 12 },
    Div4Clk { name: c"b".as_ptr(), reg: CPG_FRQCRA as u32, shift: 8 },
    Div4Clk { name: c"m1".as_ptr(), reg: CPG_FRQCRA as u32, shift: 4 },
    Div4Clk { name: c"m2".as_ptr(), reg: CPG_FRQCRA as u32, shift: 0 },
    Div4Clk { name: c"ztr".as_ptr(), reg: CPG_FRQCRB as u32, shift: 20 },
    Div4Clk { name: c"zt".as_ptr(), reg: CPG_FRQCRB as u32, shift: 16 },
    Div4Clk { name: c"zx".as_ptr(), reg: CPG_FRQCRB as u32, shift: 12 },
    Div4Clk { name: c"zs".as_ptr(), reg: CPG_FRQCRB as u32, shift: 8 },
    Div4Clk { name: c"hp".as_ptr(), reg: CPG_FRQCRB as u32, shift: 4 },
    Div4Clk { name: core::ptr::null(), reg: 0, shift: 0 },
];

static DIV4_DIV_TABLE: [ClkDivTable; 13] = [
    ClkDivTable { val: 0, div: 2 }, ClkDivTable { val: 1, div: 3 },
    ClkDivTable { val: 2, div: 4 }, ClkDivTable { val: 3, div: 6 },
    ClkDivTable { val: 4, div: 8 }, ClkDivTable { val: 5, div: 12 },
    ClkDivTable { val: 6, div: 16 }, ClkDivTable { val: 7, div: 18 },
    ClkDivTable { val: 8, div: 24 }, ClkDivTable { val: 10, div: 36 },
    ClkDivTable { val: 11, div: 48 }, ClkDivTable { val: 12, div: 10 },
    ClkDivTable { val: 0, div: 0 },
];

unsafe fn r8a73a4_cpg_register_clock(
    np: *mut DeviceNode, cpg: *mut R8a73a4Cpg, base: *mut u8, name: *const c_char,
) -> *mut Clk {
    let mut table: *const ClkDivTable = core::ptr::null();
    let mut parent_name: *const c_char;
    let mut shift: u32 = 0;
    let mut reg: usize = 0;
    let mut mult: u32 = 1;
    let mut div: u32 = 1;

    if strcmp(name, c"main".as_ptr()) == 0 {
        let ckscr = readl(base.add(CPG_CKSCR));
        match (ckscr >> 28) & 3 {
            0 => { parent_name = of_clk_get_parent_name(np, 0); }
            1 => { parent_name = of_clk_get_parent_name(np, 0); div = 2; }
            2 => { parent_name = of_clk_get_parent_name(np, 1); }
            _ => { parent_name = of_clk_get_parent_name(np, 1); div = 2; }
        }
    } else if strcmp(name, c"pll0".as_ptr()) == 0 {
        let value = readl(base.add(CPG_PLL0CR));
        parent_name = c"main".as_ptr();
        mult = ((value >> 24) & 0x7f) + 1;
        if value & (1 << 20) != 0 { div = 2; }
    } else if strcmp(name, c"pll1".as_ptr()) == 0 {
        let value = readl(base.add(CPG_PLL1CR));
        parent_name = c"main".as_ptr();
        // XXX: enable bit?
        mult = ((value >> 24) & 0x7f) + 1;
        if value & (1 << 7) != 0 { div = 2; }
    } else if strncmp(name, c"pll2".as_ptr(), 4) == 0 {
        let cr = match *name.add(4) as u8 {
            0 => CPG_PLL2CR,
            b's' => CPG_PLL2SCR,
            b'h' => CPG_PLL2HCR,
            _ => return err_ptr(-22),
        };
        let value = readl(base.add(cr));
        match (value >> 5) & 7 {
            0 => { parent_name = c"main".as_ptr(); div = 2; }
            1 => { parent_name = c"extal2".as_ptr(); div = 2; }
            3 => { parent_name = c"extal2".as_ptr(); div = 4; }
            4 => { parent_name = c"main".as_ptr(); }
            5 => { parent_name = c"extal2".as_ptr(); }
            _ => { pr_warn(c"%s: unexpected parent of %s\n".as_ptr(), c"r8a73a4_cpg_register_clock".as_ptr(), name); return err_ptr(-22); }
        }
        // XXX: enable bit?
        mult = ((value >> 24) & 0x7f) + 1;
    } else if strcmp(name, c"z".as_ptr()) == 0 || strcmp(name, c"z2".as_ptr()) == 0 {
        let mut zshift = 8;
        parent_name = c"pll0".as_ptr();
        if *name.add(1) as u8 == b'2' { div = 2; zshift = 0; }
        div *= 32;
        mult = 0x20 - ((readl(base.add(CPG_FRQCRC)) >> zshift) & 0x1f);
    } else {
        let mut c = DIV4_CLKS.as_mut_ptr();
        while !(*c).name.is_null() && strcmp(name, (*c).name) != 0 { c = c.add(1); }
        if (*c).name.is_null() { return err_ptr(-22); }
        parent_name = c"pll1".as_ptr();
        table = DIV4_DIV_TABLE.as_ptr();
        reg = (*c).reg as usize;
        shift = (*c).shift;
    }

    if table.is_null() {
        clk_register_fixed_factor(core::ptr::null_mut(), name, parent_name, 0, mult, div)
    } else {
        clk_register_divider_table(core::ptr::null_mut(), name, parent_name, 0,
            base.add(reg), shift, 4, 0, table, &mut (*cpg).lock)
    }
}

unsafe fn r8a73a4_cpg_clocks_init(np: *mut DeviceNode) {
    let num_clks = of_property_count_strings(np, c"clock-output-names".as_ptr());
    if num_clks < 0 { pr_err(c"%s: failed to count clocks\n".as_ptr(), c"r8a73a4_cpg_clocks_init".as_ptr()); return; }
    let cpg = kzalloc::<R8a73a4Cpg>();
    let clks = kzalloc_array::<*mut Clk>(num_clks as usize);
    if cpg.is_null() || clks.is_null() { return; }
    spin_lock_init(&mut (*cpg).lock);
    (*cpg).data.clks = clks;
    (*cpg).data.clk_num = num_clks as u32;
    let base = of_iomap(np, 0);
    if warn_on(base.is_null()) { return; }
    for i in 0..num_clks as usize {
        let mut name: *const c_char = core::ptr::null();
        of_property_read_string_index(np, c"clock-output-names".as_ptr(), i, &mut name);
        let clk = r8a73a4_cpg_register_clock(np, cpg, base, name);
        if is_err(clk) { pr_err(c"%s: failed to register %pOFn %s clock (%ld)\n".as_ptr(), c"r8a73a4_cpg_clocks_init".as_ptr(), np, name, ptr_err(clk)); }
        else { *(*cpg).data.clks.add(i) = clk; }
    }
    of_clk_add_provider(np, of_clk_src_onecell_get, &mut (*cpg).data);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
