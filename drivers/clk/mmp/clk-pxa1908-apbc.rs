// SPDX-License-Identifier: GPL-2.0-only
// Translated from clk-pxa1908-apbc.c. Kernel headers and symbols are external dependencies.

const APBC_UART0: usize = 0x0;
const APBC_UART1: usize = 0x4;
const APBC_GPIO: usize = 0x8;
const APBC_PWM0: usize = 0xc;
const APBC_PWM1: usize = 0x10;
const APBC_PWM2: usize = 0x14;
const APBC_PWM3: usize = 0x18;
const APBC_SSP0: usize = 0x1c;
const APBC_SSP1: usize = 0x20;
const APBC_IPC_RST: usize = 0x24;
const APBC_RTC: usize = 0x28;
const APBC_TWSI0: usize = 0x2c;
const APBC_KPC: usize = 0x30;
const APBC_SWJTAG: usize = 0x40;
const APBC_SSP2: usize = 0x4c;
const APBC_TWSI1: usize = 0x60;
const APBC_THERMAL: usize = 0x6c;
const APBC_TWSI3: usize = 0x70;
const APBC_NR_CLKS: usize = 19;

#[repr(C)]
struct pxa1908_clk_unit {
    unit: mmp_clk_unit,
    base: *mut core::ffi::c_void,
}

static mut pwm0_lock: spinlock_t = DEFINE_SPINLOCK!();
static mut pwm2_lock: spinlock_t = DEFINE_SPINLOCK!();
static mut uart0_lock: spinlock_t = DEFINE_SPINLOCK!();
static mut uart1_lock: spinlock_t = DEFINE_SPINLOCK!();

static uart_parent_names: [&'static str; 2] = ["pll1_117", "uart_pll"];
static ssp_parent_names: [&'static str; 4] = ["pll1_d16", "pll1_d48", "pll1_d24", "pll1_d12"];

static mut apbc_gate_clks: [mmp_param_gate_clk; 14] = [
    mmp_param_gate_clk { id: PXA1908_CLK_TWSI0, name: "twsi0_clk", parent_name: "pll1_32", flags: CLK_SET_RATE_PARENT, offset: APBC_TWSI0, mask: 0x3, bits: 3, lock: core::ptr::null_mut() },
    mmp_param_gate_clk { id: PXA1908_CLK_TWSI1, name: "twsi1_clk", parent_name: "pll1_32", flags: CLK_SET_RATE_PARENT, offset: APBC_TWSI1, mask: 0x3, bits: 3, lock: core::ptr::null_mut() },
    mmp_param_gate_clk { id: PXA1908_CLK_TWSI3, name: "twsi3_clk", parent_name: "pll1_32", flags: CLK_SET_RATE_PARENT, offset: APBC_TWSI3, mask: 0x3, bits: 3, lock: core::ptr::null_mut() },
    mmp_param_gate_clk { id: PXA1908_CLK_GPIO, name: "gpio_clk", parent_name: "vctcxo", flags: CLK_SET_RATE_PARENT, offset: APBC_GPIO, mask: 0x3, bits: 3, lock: core::ptr::null_mut() },
    mmp_param_gate_clk { id: PXA1908_CLK_KPC, name: "kpc_clk", parent_name: "clk32", flags: CLK_SET_RATE_PARENT, offset: APBC_KPC, mask: 0x3, bits: 3, flags2: MMP_CLK_GATE_NEED_DELAY, lock: core::ptr::null_mut() },
    mmp_param_gate_clk { id: PXA1908_CLK_RTC, name: "rtc_clk", parent_name: "clk32", flags: CLK_SET_RATE_PARENT, offset: APBC_RTC, mask: 0x83, bits: 0x83, flags2: MMP_CLK_GATE_NEED_DELAY, lock: core::ptr::null_mut() },
    mmp_param_gate_clk { id: PXA1908_CLK_PWM1, name: "pwm1_clk", parent_name: "pwm01_apb_share", flags: CLK_SET_RATE_PARENT, offset: APBC_PWM1, mask: 0x2, bits: 2, lock: core::ptr::null_mut() },
    mmp_param_gate_clk { id: PXA1908_CLK_PWM3, name: "pwm3_clk", parent_name: "pwm23_apb_share", flags: CLK_SET_RATE_PARENT, offset: APBC_PWM3, mask: 0x2, bits: 2, lock: core::ptr::null_mut() },
    mmp_param_gate_clk { id: PXA1908_CLK_UART0, name: "uart0_clk", parent_name: "uart0_mux", flags: CLK_SET_RATE_PARENT, offset: APBC_UART0, mask: 0x3, bits: 3, lock: unsafe { &raw mut uart0_lock } },
    mmp_param_gate_clk { id: PXA1908_CLK_UART1, name: "uart1_clk", parent_name: "uart1_mux", flags: CLK_SET_RATE_PARENT, offset: APBC_UART1, mask: 0x3, bits: 3, lock: unsafe { &raw mut uart1_lock } },
    mmp_param_gate_clk { id: PXA1908_CLK_THERMAL, name: "thermal_clk", parent_name: core::ptr::null(), flags: 0, offset: APBC_THERMAL, mask: 0x3, bits: 3, lock: core::ptr::null_mut() },
    mmp_param_gate_clk { id: PXA1908_CLK_IPC_RST, name: "ipc_clk", parent_name: core::ptr::null(), flags: 0, offset: APBC_IPC_RST, mask: 0x3, bits: 3, lock: core::ptr::null_mut() },
    mmp_param_gate_clk { id: PXA1908_CLK_SSP0, name: "ssp0_clk", parent_name: "ssp0_mux", flags: 0, offset: APBC_SSP0, mask: 0x3, bits: 3, lock: core::ptr::null_mut() },
    mmp_param_gate_clk { id: PXA1908_CLK_SSP2, name: "ssp2_clk", parent_name: "ssp2_mux", flags: 0, offset: APBC_SSP2, mask: 0x3, bits: 3, lock: core::ptr::null_mut() },
];

static mut apbc_gate_no_reset_clks: [mmp_param_gate_clk; 2] = [
    mmp_param_gate_clk { id: PXA1908_CLK_PWM0, name: "pwm0_clk", parent_name: "pwm01_apb_share", flags: CLK_SET_RATE_PARENT, offset: APBC_PWM0, mask: 0x2, bits: 2, lock: unsafe { &raw mut pwm0_lock } },
    mmp_param_gate_clk { id: PXA1908_CLK_PWM2, name: "pwm2_clk", parent_name: "pwm23_apb_share", flags: CLK_SET_RATE_PARENT, offset: APBC_PWM2, mask: 0x2, bits: 2, lock: core::ptr::null_mut() },
];

static mut apbc_mux_clks: [mmp_param_mux_clk; 4] = [
    mmp_param_mux_clk { id: 0, name: "uart0_mux", parent_names: uart_parent_names.as_ptr(), num_parents: 2, flags: CLK_SET_RATE_PARENT, offset: APBC_UART0, shift: 4, width: 3, lock: unsafe { &raw mut uart0_lock } },
    mmp_param_mux_clk { id: 0, name: "uart1_mux", parent_names: uart_parent_names.as_ptr(), num_parents: 2, flags: CLK_SET_RATE_PARENT, offset: APBC_UART1, shift: 4, width: 3, lock: unsafe { &raw mut uart1_lock } },
    mmp_param_mux_clk { id: 0, name: "ssp0_mux", parent_names: ssp_parent_names.as_ptr(), num_parents: 4, flags: 0, offset: APBC_SSP0, shift: 4, width: 3, lock: core::ptr::null_mut() },
    mmp_param_mux_clk { id: 0, name: "ssp2_mux", parent_names: ssp_parent_names.as_ptr(), num_parents: 4, flags: 0, offset: APBC_SSP2, shift: 4, width: 3, lock: core::ptr::null_mut() },
];

unsafe fn pxa1908_apb_periph_clk_init(pxa_unit: *mut pxa1908_clk_unit) {
    let unit = &mut (*pxa_unit).unit;
    mmp_clk_register_gate(core::ptr::null_mut(), "pwm01_apb_share", "pll1_d48", CLK_SET_RATE_PARENT, (*pxa_unit).base.add(APBC_PWM0), 0x5, 1, 0, 0, &raw mut pwm0_lock);
    mmp_clk_register_gate(core::ptr::null_mut(), "pwm23_apb_share", "pll1_d48", CLK_SET_RATE_PARENT, (*pxa_unit).base.add(APBC_PWM2), 0x5, 1, 0, 0, &raw mut pwm2_lock);
    let clk = mmp_clk_register_apbc("swjtag", core::ptr::null(), (*pxa_unit).base.add(APBC_SWJTAG), 10, 0, core::ptr::null_mut());
    mmp_clk_add(unit, PXA1908_CLK_SWJTAG, clk);
    mmp_register_mux_clks(unit, apbc_mux_clks.as_mut_ptr(), (*pxa_unit).base, 4);
    mmp_register_gate_clks(unit, apbc_gate_clks.as_mut_ptr(), (*pxa_unit).base, 14);
    mmp_register_gate_clks(unit, apbc_gate_no_reset_clks.as_mut_ptr(), (*pxa_unit).base, 2);
}

// Taken from clk-of-pxa1928.c
unsafe fn pxa1908_clk_reset_init(np: *mut device_node, pxa_unit: *mut pxa1908_clk_unit) {
    let nr_cells = 14;
    let cells = kzalloc_objs::<mmp_clk_reset_cell>(nr_cells);
    if cells.is_null() { return; }
    for i in 0..nr_cells {
        (*cells.add(i)).clk_id = apbc_gate_clks[i].id;
        (*cells.add(i)).reg = (*pxa_unit).base.add(apbc_gate_clks[i].offset);
        (*cells.add(i)).bits = BIT(2);
        (*cells.add(i)).flags = 0;
        (*cells.add(i)).lock = apbc_gate_clks[i].lock;
    }
    mmp_clk_reset_register(np, cells, nr_cells);
}

unsafe fn pxa1908_apbc_probe(pdev: *mut platform_device) -> i32 {
    let pxa_unit = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<pxa1908_clk_unit>(), GFP_KERNEL) as *mut pxa1908_clk_unit;
    if pxa_unit.is_null() { return -ENOMEM; }
    (*pxa_unit).base = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR((*pxa_unit).base) { return PTR_ERR((*pxa_unit).base); }
    mmp_clk_init((*pdev).dev.of_node, &mut (*pxa_unit).unit, APBC_NR_CLKS);
    pxa1908_apb_periph_clk_init(pxa_unit);
    pxa1908_clk_reset_init((*pdev).dev.of_node, pxa_unit);
    0
}

static pxa1908_apbc_match_table: [of_device_id; 2] = [
    of_device_id { compatible: "marvell,pxa1908-apbc" },
    of_device_id { compatible: core::ptr::null() },
];

static mut pxa1908_apbc_driver: platform_driver = platform_driver {
    probe: Some(pxa1908_apbc_probe),
    driver: driver { name: "pxa1908-apbc", of_match_table: pxa1908_apbc_match_table.as_ptr() },
};

// MODULE_DEVICE_TABLE(of, pxa1908_apbc_match_table);
// module_platform_driver(pxa1908_apbc_driver);
// MODULE_AUTHOR("Duje Mihanović <duje.mihanovic@skole.hr>");
// MODULE_DESCRIPTION("Marvell PXA1908 APBC Clock Driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
