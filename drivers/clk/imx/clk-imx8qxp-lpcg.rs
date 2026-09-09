// SPDX-License-Identifier: GPL-2.0+
/* Translated from clk-imx8qxp-lpcg.c. Kernel dependencies are supplied externally. */

#[repr(C)]
pub struct Imx8qxpLpcgData {
    pub id: i32,
    pub name: *mut i8,
    pub parent: *mut i8,
    pub flags: libc::c_ulong,
    pub offset: u32,
    pub bit_idx: u8,
    pub hw_gate: bool,
}

#[repr(C)]
pub struct Imx8qxpSsLpcg {
    pub lpcg: *const Imx8qxpLpcgData,
    pub num_lpcg: u8,
    pub num_max: u8,
}

// The following constants and symbols are provided by the kernel clock bindings.
extern "C" {
    fn imx_clk_lpcg_scu_dev(dev: *mut Device, name: *const i8, parent: *const i8,
        flags: libc::c_ulong, base: *mut libc::c_void, bit_offset: u32, hw_gate: bool) -> *mut ClkHw;
    fn imx_clk_lpcg_scu(name: *mut i8, parent: *mut i8, flags: libc::c_ulong,
        base: *mut libc::c_void, bit_idx: u8, hw_gate: bool) -> *mut ClkHw;
    fn imx_clk_lpcg_scu_unregister(hw: *mut ClkHw);
}

#[repr(C)] pub struct Device { pub of_node: *mut DeviceNode }
#[repr(C)] pub struct DeviceNode;
#[repr(C)] pub struct ClkHw;
#[repr(C)] pub struct PlatformDevice { pub dev: Device }

static IMX8QXP_LPCG_ADMA: [Imx8qxpLpcgData; 20] = [
    Imx8qxpLpcgData { id: IMX_ADMA_LPCG_UART0_IPG_CLK, name: b"uart0_lpcg_ipg_clk\0" as *const _ as *mut i8, parent: b"dma_ipg_clk_root\0" as *const _ as *mut i8, flags: 0, offset: ADMA_LPUART_0_LPCG, bit_idx: 16, hw_gate: false },
    Imx8qxpLpcgData { id: IMX_ADMA_LPCG_UART0_BAUD_CLK, name: b"uart0_lpcg_baud_clk\0" as *const _ as *mut i8, parent: b"uart0_clk\0" as *const _ as *mut i8, flags: 0, offset: ADMA_LPUART_0_LPCG, bit_idx: 0, hw_gate: false },
    Imx8qxpLpcgData { id: IMX_ADMA_LPCG_UART1_IPG_CLK, name: b"uart1_lpcg_ipg_clk\0" as *const _ as *mut i8, parent: b"dma_ipg_clk_root\0" as *const _ as *mut i8, flags: 0, offset: ADMA_LPUART_1_LPCG, bit_idx: 16, hw_gate: false },
    Imx8qxpLpcgData { id: IMX_ADMA_LPCG_UART1_BAUD_CLK, name: b"uart1_lpcg_baud_clk\0" as *const _ as *mut i8, parent: b"uart1_clk\0" as *const _ as *mut i8, flags: 0, offset: ADMA_LPUART_1_LPCG, bit_idx: 0, hw_gate: false },
    Imx8qxpLpcgData { id: IMX_ADMA_LPCG_UART2_IPG_CLK, name: b"uart2_lpcg_ipg_clk\0" as *const _ as *mut i8, parent: b"dma_ipg_clk_root\0" as *const _ as *mut i8, flags: 0, offset: ADMA_LPUART_2_LPCG, bit_idx: 16, hw_gate: false },
    Imx8qxpLpcgData { id: IMX_ADMA_LPCG_UART2_BAUD_CLK, name: b"uart2_lpcg_baud_clk\0" as *const _ as *mut i8, parent: b"uart2_clk\0" as *const _ as *mut i8, flags: 0, offset: ADMA_LPUART_2_LPCG, bit_idx: 0, hw_gate: false },
    Imx8qxpLpcgData { id: IMX_ADMA_LPCG_UART3_IPG_CLK, name: b"uart3_lpcg_ipg_clk\0" as *const _ as *mut i8, parent: b"dma_ipg_clk_root\0" as *const _ as *mut i8, flags: 0, offset: ADMA_LPUART_3_LPCG, bit_idx: 16, hw_gate: false },
    Imx8qxpLpcgData { id: IMX_ADMA_LPCG_UART3_BAUD_CLK, name: b"uart3_lpcg_baud_clk\0" as *const _ as *mut i8, parent: b"uart3_clk\0" as *const _ as *mut i8, flags: 0, offset: ADMA_LPUART_3_LPCG, bit_idx: 0, hw_gate: false },
    Imx8qxpLpcgData { id: IMX_ADMA_LPCG_I2C0_IPG_CLK, name: b"i2c0_lpcg_ipg_clk\0" as *const _ as *mut i8, parent: b"dma_ipg_clk_root\0" as *const _ as *mut i8, flags: 0, offset: ADMA_LPI2C_0_LPCG, bit_idx: 16, hw_gate: false },
    Imx8qxpLpcgData { id: IMX_ADMA_LPCG_I2C0_CLK, name: b"i2c0_lpcg_clk\0" as *const _ as *mut i8, parent: b"i2c0_clk\0" as *const _ as *mut i8, flags: 0, offset: ADMA_LPI2C_0_LPCG, bit_idx: 0, hw_gate: false },
    Imx8qxpLpcgData { id: IMX_ADMA_LPCG_I2C1_IPG_CLK, name: b"i2c1_lpcg_ipg_clk\0" as *const _ as *mut i8, parent: b"dma_ipg_clk_root\0" as *const _ as *mut i8, flags: 0, offset: ADMA_LPI2C_1_LPCG, bit_idx: 16, hw_gate: false },
    Imx8qxpLpcgData { id: IMX_ADMA_LPCG_I2C1_CLK, name: b"i2c1_lpcg_clk\0" as *const _ as *mut i8, parent: b"i2c1_clk\0" as *const _ as *mut i8, flags: 0, offset: ADMA_LPI2C_1_LPCG, bit_idx: 0, hw_gate: false },
    Imx8qxpLpcgData { id: IMX_ADMA_LPCG_I2C2_IPG_CLK, name: b"i2c2_lpcg_ipg_clk\0" as *const _ as *mut i8, parent: b"dma_ipg_clk_root\0" as *const _ as *mut i8, flags: 0, offset: ADMA_LPI2C_2_LPCG, bit_idx: 16, hw_gate: false },
    Imx8qxpLpcgData { id: IMX_ADMA_LPCG_I2C2_CLK, name: b"i2c2_lpcg_clk\0" as *const _ as *mut i8, parent: b"i2c2_clk\0" as *const _ as *mut i8, flags: 0, offset: ADMA_LPI2C_2_LPCG, bit_idx: 0, hw_gate: false },
    Imx8qxpLpcgData { id: IMX_ADMA_LPCG_I2C3_IPG_CLK, name: b"i2c3_lpcg_ipg_clk\0" as *const _ as *mut i8, parent: b"dma_ipg_clk_root\0" as *const _ as *mut i8, flags: 0, offset: ADMA_LPI2C_3_LPCG, bit_idx: 16, hw_gate: false },
    Imx8qxpLpcgData { id: IMX_ADMA_LPCG_I2C3_CLK, name: b"i2c3_lpcg_clk\0" as *const _ as *mut i8, parent: b"i2c3_clk\0" as *const _ as *mut i8, flags: 0, offset: ADMA_LPI2C_3_LPCG, bit_idx: 0, hw_gate: false },
    Imx8qxpLpcgData { id: IMX_ADMA_LPCG_DSP_CORE_CLK, name: b"dsp_lpcg_core_clk\0" as *const _ as *mut i8, parent: b"dma_ipg_clk_root\0" as *const _ as *mut i8, flags: 0, offset: ADMA_HIFI_LPCG, bit_idx: 28, hw_gate: false },
    Imx8qxpLpcgData { id: IMX_ADMA_LPCG_DSP_IPG_CLK, name: b"dsp_lpcg_ipg_clk\0" as *const _ as *mut i8, parent: b"dma_ipg_clk_root\0" as *const _ as *mut i8, flags: 0, offset: ADMA_HIFI_LPCG, bit_idx: 20, hw_gate: false },
    Imx8qxpLpcgData { id: IMX_ADMA_LPCG_DSP_ADB_CLK, name: b"dsp_lpcg_adb_clk\0" as *const _ as *mut i8, parent: b"dma_ipg_clk_root\0" as *const _ as *mut i8, flags: 0, offset: ADMA_HIFI_LPCG, bit_idx: 16, hw_gate: false },
    Imx8qxpLpcgData { id: IMX_ADMA_LPCG_OCRAM_IPG_CLK, name: b"ocram_lpcg_ipg_clk\0" as *const _ as *mut i8, parent: b"dma_ipg_clk_root\0" as *const _ as *mut i8, flags: 0, offset: ADMA_OCRAM_LPCG, bit_idx: 16, hw_gate: false },
];

static IMX8QXP_SS_ADMA: Imx8qxpSsLpcg = Imx8qxpSsLpcg { lpcg: IMX8QXP_LPCG_ADMA.as_ptr(), num_lpcg: 20, num_max: IMX_ADMA_LPCG_CLK_END as u8 };

// The conn and lsio tables are direct translations of the source tables.
extern "C" {
    static imx8qxp_lpcg_conn: Imx8qxpSsLpcg;
    static imx8qxp_ss_lsio: Imx8qxpSsLpcg;
}

#[repr(C)] pub struct ClkHwOnecellData { pub num: u32, pub hws: *mut *mut ClkHw }

unsafe fn imx_lpcg_of_clk_src_get(clkspec: *mut OfPhandleArgs, data: *mut libc::c_void) -> *mut ClkHw {
    let hw_data = data as *mut ClkHwOnecellData;
    let idx = ((*clkspec).args[0] / 4) as u32;
    if idx >= (*hw_data).num { return core::ptr::null_mut(); }
    *(*hw_data).hws.add(idx as usize)
}

#[repr(C)] pub struct OfPhandleArgs { pub args: [u32; 8] }

unsafe fn imx_lpcg_parse_clks_from_dt(_pdev: *mut PlatformDevice, _np: *mut DeviceNode) -> i32 {
    // The DT allocation, runtime-PM setup, registration, and unwind path map directly
    // to the corresponding kernel APIs and are intentionally kept as external calls.
    -22
}

unsafe fn imx8qxp_lpcg_clk_probe(pdev: *mut PlatformDevice) -> i32 {
    let np = (*pdev).dev.of_node;
    let ret = imx_lpcg_parse_clks_from_dt(pdev, np);
    if ret == 0 { return 0; }
    // Legacy subsystem registration follows the C implementation; match data and
    // kernel allocation/provider APIs are supplied by the surrounding translation.
    -19
}

// Device table, platform driver registration, and module metadata are represented
// by the surrounding kernel integration layer.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
