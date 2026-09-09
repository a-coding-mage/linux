// SPDX-License-Identifier: GPL-2.0
/*
 * r8a7740 Core CPG Clocks
 *
 * Copyright (C) 2014  Ulrich Hecht
 */

// Linux clock-provider, Renesas, init, I/O, kernel, slab, OF, and spinlock
// declarations are supplied by the surrounding kernel translation.

#[repr(C)]
struct R8a7740Cpg {
    data: ClkOnecellData,
    lock: SpinlockT,
}

const CPG_FRQCRA: u32 = 0x00;
const CPG_FRQCRB: u32 = 0x04;
const CPG_PLLC2CR: u32 = 0x2c;
const CPG_USBCKCR: u32 = 0x8c;
const CPG_FRQCRC: u32 = 0xe0;

#[repr(C)]
struct Div4Clk {
    name: *const core::ffi::c_char,
    reg: u32,
    shift: u32,
}

static mut DIV4_CLKS: [Div4Clk; 14] = [
    Div4Clk { name: c"i".as_ptr(), reg: CPG_FRQCRA, shift: 20 },
    Div4Clk { name: c"zg".as_ptr(), reg: CPG_FRQCRA, shift: 16 },
    Div4Clk { name: c"b".as_ptr(), reg: CPG_FRQCRA, shift: 8 },
    Div4Clk { name: c"m1".as_ptr(), reg: CPG_FRQCRA, shift: 4 },
    Div4Clk { name: c"ztr".as_ptr(), reg: CPG_FRQCRB, shift: 20 },
    Div4Clk { name: c"zt".as_ptr(), reg: CPG_FRQCRB, shift: 16 },
    Div4Clk { name: c"hp".as_ptr(), reg: CPG_FRQCRB, shift: 4 },
    Div4Clk { name: c"hpp".as_ptr(), reg: CPG_FRQCRC, shift: 20 },
    Div4Clk { name: c"usbp".as_ptr(), reg: CPG_FRQCRC, shift: 16 },
    Div4Clk { name: c"s".as_ptr(), reg: CPG_FRQCRC, shift: 12 },
    Div4Clk { name: c"zb".as_ptr(), reg: CPG_FRQCRC, shift: 8 },
    Div4Clk { name: c"m3".as_ptr(), reg: CPG_FRQCRC, shift: 4 },
    Div4Clk { name: c"cp".as_ptr(), reg: CPG_FRQCRC, shift: 0 },
    Div4Clk { name: core::ptr::null(), reg: 0, shift: 0 },
];

static DIV4_DIV_TABLE: [ClkDivTable; 15] = [
    ClkDivTable { val: 0, div: 2 }, ClkDivTable { val: 1, div: 3 },
    ClkDivTable { val: 2, div: 4 }, ClkDivTable { val: 3, div: 6 },
    ClkDivTable { val: 4, div: 8 }, ClkDivTable { val: 5, div: 12 },
    ClkDivTable { val: 6, div: 16 }, ClkDivTable { val: 7, div: 18 },
    ClkDivTable { val: 8, div: 24 }, ClkDivTable { val: 9, div: 32 },
    ClkDivTable { val: 10, div: 36 }, ClkDivTable { val: 11, div: 48 },
    ClkDivTable { val: 13, div: 72 }, ClkDivTable { val: 14, div: 96 },
    ClkDivTable { val: 0, div: 0 },
];

static mut CPG_MODE: u32 = 0;

unsafe fn r8a7740_cpg_register_clock(
    np: *mut DeviceNode,
    cpg: *mut R8a7740Cpg,
    base: *mut core::ffi::c_void,
    name: *const core::ffi::c_char,
) -> *mut Clk {
    let mut table: *const ClkDivTable = core::ptr::null();
    let mut parent_name: *const core::ffi::c_char;
    let mut shift: u32 = 0;
    let mut reg: u32 = 0;
    let mut mult: u32 = 1;
    let mut div: u32 = 1;

    if strcmp(name, c"r".as_ptr()) == 0 {
        match CPG_MODE & (BIT(2) | BIT(1)) {
            x if x == (BIT(1) | BIT(2)) => { parent_name = of_clk_get_parent_name(np, 0); div = 2048; }
            x if x == BIT(2) => { parent_name = of_clk_get_parent_name(np, 0); div = 1024; }
            _ => { parent_name = of_clk_get_parent_name(np, 2); }
        }
    } else if strcmp(name, c"system".as_ptr()) == 0 {
        parent_name = of_clk_get_parent_name(np, 0);
        if CPG_MODE & BIT(1) != 0 { div = 2; }
    } else if strcmp(name, c"pllc0".as_ptr()) == 0 {
        let value = readl((base as *mut u8).add(CPG_FRQCRC as usize) as *const u32);
        parent_name = c"system".as_ptr(); mult = ((value >> 24) & 0x7f) + 1;
    } else if strcmp(name, c"pllc1".as_ptr()) == 0 {
        let value = readl((base as *mut u8).add(CPG_FRQCRA as usize) as *const u32);
        parent_name = c"system".as_ptr(); mult = ((value >> 24) & 0x7f) + 1; div = 2;
    } else if strcmp(name, c"pllc2".as_ptr()) == 0 {
        let value = readl((base as *mut u8).add(CPG_PLLC2CR as usize) as *const u32);
        parent_name = c"system".as_ptr(); mult = ((value >> 24) & 0x3f) + 1;
    } else if strcmp(name, c"usb24s".as_ptr()) == 0 {
        let value = readl((base as *mut u8).add(CPG_USBCKCR as usize) as *const u32);
        parent_name = if value & BIT(7) != 0 { of_clk_get_parent_name(np, 1) } else { c"system".as_ptr() };
        if value & BIT(6) == 0 { div = 2; }
    } else {
        let mut c = DIV4_CLKS.as_mut_ptr();
        while !(*c).name.is_null() {
            if strcmp(name, (*c).name) == 0 {
                parent_name = c"pllc1".as_ptr(); table = DIV4_DIV_TABLE.as_ptr(); reg = (*c).reg; shift = (*c).shift; break;
            }
            c = c.add(1);
        }
        if (*c).name.is_null() { return ERR_PTR(-EINVAL); }
    }

    if table.is_null() {
        clk_register_fixed_factor(core::ptr::null_mut(), name, parent_name, 0, mult, div)
    } else {
        clk_register_divider_table(core::ptr::null_mut(), name, parent_name, 0,
            (base as *mut u8).add(reg as usize) as *mut core::ffi::c_void,
            shift, 4, 0, table, &mut (*cpg).lock)
    }
}

unsafe fn r8a7740_cpg_clocks_init(np: *mut DeviceNode) {
    let mut cpg: *mut R8a7740Cpg;
    let mut base: *mut core::ffi::c_void;
    let clks: *mut *mut Clk;
    let mut i: u32;
    let num_clks: i32;

    if of_property_read_u32(np, c"renesas,mode".as_ptr(), &mut CPG_MODE) != 0 { pr_warn(c"%s: missing renesas,mode property\n".as_ptr(), c"r8a7740_cpg_clocks_init".as_ptr()); }
    num_clks = of_property_count_strings(np, c"clock-output-names".as_ptr());
    if num_clks < 0 { pr_err(c"%s: failed to count clocks\n".as_ptr(), c"r8a7740_cpg_clocks_init".as_ptr()); return; }
    cpg = kzalloc_obj::<R8a7740Cpg>();
    clks = kzalloc_objs::<*mut Clk>(num_clks as usize);
    if cpg.is_null() || clks.is_null() { return; }
    spin_lock_init(&mut (*cpg).lock);
    (*cpg).data.clks = clks; (*cpg).data.clk_num = num_clks as u32;
    base = of_iomap(np, 0);
    if WARN_ON(base.is_null()) { return; }
    i = 0;
    while i < num_clks as u32 {
        let mut name: *const core::ffi::c_char = core::ptr::null();
        of_property_read_string_index(np, c"clock-output-names".as_ptr(), i, &mut name);
        let clk = r8a7740_cpg_register_clock(np, cpg, base, name);
        if IS_ERR(clk) { pr_err(c"%s: failed to register %pOFn %s clock (%ld)\n".as_ptr(), c"r8a7740_cpg_clocks_init".as_ptr(), np, name, PTR_ERR(clk)); }
        else { *clks.add(i as usize) = clk; }
        i += 1;
    }
    of_clk_add_provider(np, of_clk_src_onecell_get, &mut (*cpg).data);
}

// CLK_OF_DECLARE(r8a7740_cpg_clks, "renesas,r8a7740-cpg-clocks",
//                r8a7740_cpg_clocks_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
