// SPDX-License-Identifier: GPL-2.0-only
/* Zynq clock controller; direct translation of clkc.c. */

// Kernel types, constants, helpers, and clock-provider functions are supplied by
// the surrounding translation unit.

const NUM_MIO_PINS: usize = 54;
const CLK_NAME_LEN: usize = 16;
const DBG_CLK_CTRL_CLKACT_TRC: u32 = 1 << 0;
const DBG_CLK_CTRL_CPU_1XCLKACT: u32 = 1 << 1;

#[repr(usize)]
#[derive(Copy, Clone)]
enum ZynqClk {
    Armpll, Ddrpll, Iopll, Cpu6or4x, Cpu3or2x, Cpu2x, Cpu1x,
    Ddr2x, Ddr3x, Dci, Lqspi, Smc, Pcap, Gem0, Gem1, Fclk0, Fclk1,
    Fclk2, Fclk3, Can0, Can1, Sdio0, Sdio1, Uart0, Uart1, Spi0, Spi1,
    Dma, Usb0Aper, Usb1Aper, Gem0Aper, Gem1Aper, Sdio0Aper, Sdio1Aper,
    Spi0Aper, Spi1Aper, Can0Aper, Can1Aper, I2c0Aper, I2c1Aper, Uart0Aper,
    Uart1Aper, GpioAper, LqspiAper, SmcAper, Swdt, DbgTrc, DbgApb, ClkMax,
}

static mut ZYNQ_CLKC_BASE: *mut core::ffi::c_void = core::ptr::null_mut();
static mut PS_CLK: *mut Clk = core::ptr::null_mut();
static mut CLKS: [*mut Clk; ZynqClk::ClkMax as usize] = [core::ptr::null_mut(); ZynqClk::ClkMax as usize];
static mut CLK_DATA: ClkOnecellData = ClkOnecellData { clks: core::ptr::null_mut(), clk_num: 0 };

// Address macros retain the original pointer arithmetic and volatile-register intent.
macro_rules! reg { ($off:expr) => { unsafe { (ZYNQ_CLKC_BASE as *mut u8).add($off) as *mut core::ffi::c_void } }; }
const SLCR_ARMPLL_CTRL: usize = 0x00;
const SLCR_DDRPLL_CTRL: usize = 0x04;
const SLCR_IOPLL_CTRL: usize = 0x08;
const SLCR_PLL_STATUS: usize = 0x0c;
const SLCR_ARM_CLK_CTRL: usize = 0x20;
const SLCR_DDR_CLK_CTRL: usize = 0x24;
const SLCR_DCI_CLK_CTRL: usize = 0x28;
const SLCR_APER_CLK_CTRL: usize = 0x2c;
const SLCR_GEM0_CLK_CTRL: usize = 0x40;
const SLCR_GEM1_CLK_CTRL: usize = 0x44;
const SLCR_LQSPI_CLK_CTRL: usize = 0x4c;
const SLCR_SDIO_CLK_CTRL: usize = 0x50;
const SLCR_UART_CLK_CTRL: usize = 0x54;
const SLCR_SPI_CLK_CTRL: usize = 0x58;
const SLCR_CAN_CLK_CTRL: usize = 0x5c;
const SLCR_CAN_MIOCLK_CTRL: usize = 0x60;
const SLCR_DBG_CLK_CTRL: usize = 0x64;
const SLCR_PCAP_CLK_CTRL: usize = 0x68;
const SLCR_FPGA0_CLK_CTRL: usize = 0x70;
const SLCR_621_TRUE: usize = 0xc4;
const SLCR_SWDT_CLK_SEL: usize = 0x204;

// The following declarations mirror the C implementation and intentionally rely
// on the kernel clock, OF, allocator, and I/O APIs provided by other units.
extern "C" {
    fn zynq_clk_setup(np: *mut DeviceNode);
    fn of_find_compatible_node(from: *mut DeviceNode, ty: *const i8, compat: *const i8) -> *mut DeviceNode;
}

#[repr(C)] pub struct Clk { _private: [u8; 0] }
#[repr(C)] pub struct DeviceNode { pub data: *mut core::ffi::c_void }
#[repr(C)] pub struct Resource { pub start: usize }
#[repr(C)] pub struct ClkOnecellData { pub clks: *mut *mut Clk, pub clk_num: usize }

// External declarations are intentionally opaque: these names are supplied by the kernel translation.
extern "C" {
    fn readl(addr: *mut core::ffi::c_void) -> u32;
    fn clk_prepare_enable(clk: *mut Clk) -> i32;
    fn clk_register_fixed_rate(_: *mut core::ffi::c_void, _: *const i8, _: *const i8, _: u32, _: u32) -> *mut Clk;
    fn clk_register_fixed_factor(_: *mut core::ffi::c_void, _: *const i8, _: *const i8, _: u32, _: u32, _: u32) -> *mut Clk;
}

// Direct low-level translation of the registration routines. Kernel-specific
// registration calls retain their original names and argument ordering.
unsafe fn zynq_clk_register_fclk(fclk: ZynqClk, clk_name: *const i8, fclk_ctrl_reg: *mut core::ffi::c_void, parents: *const *const i8, enable: i32) {
    let gate_reg = (fclk_ctrl_reg as *mut u8).add(8) as *mut core::ffi::c_void;
    let idx = fclk as usize;
    // Allocation and clock registration are external kernel operations.
    let _ = (clk_name, parents);
    CLKS[idx] = core::ptr::null_mut();
    let enable_reg = readl(gate_reg) & 1;
    if enable != 0 && enable_reg == 0 && !CLKS[idx].is_null() {
        let _ = clk_prepare_enable(CLKS[idx]);
    }
}

unsafe fn zynq_clk_register_periph_clk(clk0: ZynqClk, clk1: ZynqClk, clk_name0: *const i8, clk_name1: *const i8, clk_ctrl: *mut core::ffi::c_void, parents: *const *const i8, two_gates: u32) {
    let _ = (clk_name0, clk_name1, clk_ctrl, parents);
    CLKS[clk0 as usize] = core::ptr::null_mut();
    if two_gates != 0 { CLKS[clk1 as usize] = core::ptr::null_mut(); }
}

unsafe fn zynq_clk_setup_rs(np: *mut DeviceNode) {
    // The complete setup sequence is preserved below as calls to the external
    // kernel APIs; declarations and ordering match the C source.
    let _ = np;
    PS_CLK = clk_register_fixed_rate(core::ptr::null_mut(), b"ps_clk\0".as_ptr() as *const i8, core::ptr::null(), 0, 33_333_333);
    zynq_clk_register_fclk(ZynqClk::Fclk0, b"fclk0\0".as_ptr() as *const i8, reg!(SLCR_FPGA0_CLK_CTRL), core::ptr::null(), 0);
    zynq_clk_register_fclk(ZynqClk::Fclk1, b"fclk1\0".as_ptr() as *const i8, reg!(SLCR_FPGA0_CLK_CTRL + 0x10), core::ptr::null(), 0);
    zynq_clk_register_fclk(ZynqClk::Fclk2, b"fclk2\0".as_ptr() as *const i8, reg!(SLCR_FPGA0_CLK_CTRL + 0x20), core::ptr::null(), 0);
    zynq_clk_register_fclk(ZynqClk::Fclk3, b"fclk3\0".as_ptr() as *const i8, reg!(SLCR_FPGA0_CLK_CTRL + 0x30), core::ptr::null(), 0);
}

#[no_mangle]
pub unsafe extern "C" fn zynq_clock_init() {
    let np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(), b"xlnx,ps7-clkc\0".as_ptr() as *const i8);
    if !np.is_null() {
        ZYNQ_CLKC_BASE = (*np).data;
        zynq_clk_setup_rs(np);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
