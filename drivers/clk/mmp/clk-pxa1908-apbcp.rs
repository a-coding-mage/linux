// SPDX-License-Identifier: GPL-2.0-only
// External Linux/kernel and local clock/reset dependencies are supplied by the build environment.

const APBCP_UART2: usize = 0x1c;
const APBCP_TWSI2: usize = 0x28;
const APBCP_AICER: usize = 0x38;

const APBCP_NR_CLKS: usize = 4;

#[repr(C)]
struct pxa1908_clk_unit {
    unit: mmp_clk_unit,
    base: *mut core::ffi::c_void,
}

static mut uart2_lock: spinlock_t = spinlock_t::new();

static uart_parent_names: [&'static str; 2] = ["pll1_117", "uart_pll"];

static mut apbcp_gate_clks: [mmp_param_gate_clk; 3] = [
    mmp_param_gate_clk {
        id: PXA1908_CLK_UART2,
        name: "uart2_clk",
        parent_name: "uart2_mux",
        flags: CLK_SET_RATE_PARENT,
        offset: APBCP_UART2,
        mask: 0x3,
        val: 0x3,
        invert: 0x0,
        shift: 0,
        lock: &raw mut uart2_lock,
    },
    mmp_param_gate_clk {
        id: PXA1908_CLK_TWSI2,
        name: "twsi2_clk",
        parent_name: "pll1_32",
        flags: CLK_SET_RATE_PARENT,
        offset: APBCP_TWSI2,
        mask: 0x3,
        val: 0x3,
        invert: 0x0,
        shift: 0,
        lock: core::ptr::null_mut(),
    },
    mmp_param_gate_clk {
        id: PXA1908_CLK_AICER,
        name: "ripc_clk",
        parent_name: core::ptr::null(),
        flags: 0,
        offset: APBCP_AICER,
        mask: 0x3,
        val: 0x2,
        invert: 0x0,
        shift: 0,
        lock: core::ptr::null_mut(),
    },
];

static mut apbcp_mux_clks: [mmp_param_mux_clk; 1] = [mmp_param_mux_clk {
    id: 0,
    name: "uart2_mux",
    parent_names: uart_parent_names.as_ptr(),
    num_parents: uart_parent_names.len(),
    flags: CLK_SET_RATE_PARENT,
    offset: APBCP_UART2,
    shift: 4,
    width: 3,
    reserved: 0,
    lock: &raw mut uart2_lock,
}];

unsafe fn pxa1908_apb_p_periph_clk_init(pxa_unit: *mut pxa1908_clk_unit) {
    let unit: *mut mmp_clk_unit = &mut (*pxa_unit).unit;

    mmp_register_mux_clks(
        unit,
        apbcp_mux_clks.as_mut_ptr(),
        (*pxa_unit).base,
        apbcp_mux_clks.len(),
    );
    mmp_register_gate_clks(
        unit,
        apbcp_gate_clks.as_mut_ptr(),
        (*pxa_unit).base,
        apbcp_gate_clks.len(),
    );
}

/* Taken from clk-of-pxa1928.c */
unsafe fn pxa1908_clk_reset_init(
    np: *mut device_node,
    pxa_unit: *mut pxa1908_clk_unit,
) {
    let nr_cells: usize = apbcp_gate_clks.len();
    let cells: *mut mmp_clk_reset_cell = kzalloc_objs::<mmp_clk_reset_cell>(nr_cells);
    if cells.is_null() {
        return;
    }

    for i in 0..nr_cells {
        (*cells.add(i)).clk_id = (*apbcp_gate_clks.as_ptr().add(i)).id;
        (*cells.add(i)).reg = (*pxa_unit).base.add((*apbcp_gate_clks.as_ptr().add(i)).offset);
        (*cells.add(i)).bits = BIT(2);
        (*cells.add(i)).flags = 0;
        (*cells.add(i)).lock = (*apbcp_gate_clks.as_ptr().add(i)).lock;
    }

    mmp_clk_reset_register(np, cells, nr_cells);
}

unsafe fn pxa1908_apbcp_probe(pdev: *mut platform_device) -> i32 {
    let pxa_unit: *mut pxa1908_clk_unit =
        devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<pxa1908_clk_unit>(), GFP_KERNEL);
    if pxa_unit.is_null() {
        return -ENOMEM;
    }

    (*pxa_unit).base = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR((*pxa_unit).base) {
        return PTR_ERR((*pxa_unit).base);
    }

    mmp_clk_init((*pdev).dev.of_node, &mut (*pxa_unit).unit, APBCP_NR_CLKS);
    pxa1908_apb_p_periph_clk_init(pxa_unit);
    pxa1908_clk_reset_init((*pdev).dev.of_node, pxa_unit);

    0
}

static pxa1908_apbcp_match_table: [of_device_id; 2] = [
    of_device_id { compatible: "marvell,pxa1908-apbcp" },
    of_device_id { compatible: core::ptr::null() },
];

static mut pxa1908_apbcp_driver: platform_driver = platform_driver {
    probe: Some(pxa1908_apbcp_probe),
    driver: driver {
        name: "pxa1908-apbcp",
        of_match_table: pxa1908_apbcp_match_table.as_ptr(),
    },
};

// MODULE_DEVICE_TABLE(of, pxa1908_apbcp_match_table);
// module_platform_driver(pxa1908_apbcp_driver);
// MODULE_AUTHOR("Duje Mihanović <duje.mihanovic@skole.hr>");
// MODULE_DESCRIPTION("Marvell PXA1908 APBCP Clock Driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
