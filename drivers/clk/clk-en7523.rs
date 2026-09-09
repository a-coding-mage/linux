// SPDX-License-Identifier: GPL-2.0-only
// Kernel headers and device-tree bindings are supplied by the surrounding tree.

const RST_NR_PER_BANK: usize = 32;
const REG_PCI_CONTROL: usize = 0x88;
const REG_PCI_CONTROL_PERSTOUT: u32 = 1 << 29;
const REG_PCI_CONTROL_PERSTOUT1: u32 = 1 << 26;
const REG_PCI_CONTROL_REFCLK_EN0: u32 = 1 << 23;
const REG_PCI_CONTROL_REFCLK_EN1: u32 = 1 << 22;
const REG_PCI_CONTROL_PERSTOUT2: u32 = 1 << 16;
const REG_GSW_CLK_DIV_SEL: usize = 0x1b4;
const REG_EMI_CLK_DIV_SEL: usize = 0x1b8;
const REG_BUS_CLK_DIV_SEL: usize = 0x1bc;
const REG_SPI_CLK_DIV_SEL: usize = 0x1c4;
const REG_SPI_CLK_FREQ_SEL: usize = 0x1c8;
const REG_NPU_CLK_DIV_SEL: usize = 0x1fc;
const REG_CRYPTO_CLKSRC: usize = 0x200;
const REG_RESET_CONTROL2: usize = 0x830;
const REG_RESET_CONTROL1: usize = 0x834;
const REG_RESET_CONTROL_PCIEHB: u32 = 1 << 29;
const REG_RESET_CONTROL_PCIE1: u32 = 1 << 27;
const REG_RESET_CONTROL_PCIE2: u32 = 1 << 26;
const REG_HIR: usize = 0x064;
const REG_HIR_MASK: u32 = 0xffff0000;
const REG_NP_SCU_PCIC: usize = 0x88;
const REG_NP_SCU_SSTR: usize = 0x9c;
const REG_PCIE_XSI0_SEL_MASK: u32 = 0x6000;
const REG_PCIE_XSI1_SEL_MASK: u32 = 0x1800;
const REG_CRYPTO_CLKSRC2: usize = 0x20c;
const EN751221_REG_SPI_DIV: usize = 0x0cc;
const EN751221_REG_SPI_DIV_MASK: u32 = 0xffffff00;
const EN751221_SPI_BASE: u32 = 500_000_000;
const EN751221_SPI_BASE_EN7526C: u32 = 400_000_000;
const EN751221_SPI_DIV_DEFAULT: u32 = 40;
const EN751221_REG_BUS: usize = 0x284;
const EN751221_REG_BUS_MASK: u32 = 0x003ff000;
const EN751221_REG_SSR3: usize = 0x094;
const EN751221_REG_SSR3_GSW_MASK: u32 = 0x300;
const REG_RST_CTRL2: usize = 0x830;
const REG_RST_CTRL1: usize = 0x834;
const REG_PCIE_HB_RST: u32 = 1 << 29;
const EN751221_REG_RST_DMT: usize = 0x84;
const EN751221_REG_RST_USB: usize = 0xec;
const EN751221_MAX_CLKS: usize = 5;

#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum EnHir { Unknown = -1, Tc3169 = 0, Tc3182, Rt65168, Rt63165, Rt63365,
    Mt751020, Mt7505, En751221, En7526C, En751627, En7580, En7528, En7523, En7581, Max }

#[repr(C)]
union BaseValue { base_values: *const u32, base_value: u32 }
#[repr(C)]
struct EnClkDesc { id: i32, name: *const u8, base_reg: usize, base_bits: u8,
    base_shift: u8, base: BaseValue, n_base_values: usize, div_reg: u16,
    div_bits: u8, div_shift: u8, div_val0: u16, div_step: u8, div_offset: u8 }
#[repr(C)] struct EnClkGate { base: *mut u8, hw: ClkHw }
#[repr(C)] struct EnRstData { bank_ofs: *const u16, idx_map: *const u16, base: *mut u8, rcdev: ResetControllerDev }
#[repr(C)] struct EnClkSocData { num_clocks: u32, pcie_ops: ClkOps, hw_init: Option<unsafe extern "C" fn(*mut PlatformDevice,*mut ClkHwOnecellData)->i32> }

// External kernel types and functions are intentionally declarations only.
#[repr(C)] struct ClkHw { init: *const ClkInitData }
#[repr(C)] struct ClkInitData { name: *const u8, ops: *const ClkOps }
#[repr(C)] struct ClkOps { is_enabled: Option<unsafe extern "C" fn(*mut ClkHw)->i32>, prepare: Option<unsafe extern "C" fn(*mut ClkHw)->i32>, unprepare: Option<unsafe extern "C" fn(*mut ClkHw)>, enable: Option<unsafe extern "C" fn(*mut ClkHw)->i32>, disable: Option<unsafe extern "C" fn(*mut ClkHw)> }
#[repr(C)] struct ClkHwOnecellData { num: u32, hws: *mut *mut ClkHw }
#[repr(C)] struct ResetControllerDev { nr_resets: u32, ops: *const ResetControlOps, of_node: *mut u8, of_reset_n_cells: u32, dev: *mut Device }
#[repr(C)] struct ResetControlOps { assert: Option<unsafe extern "C" fn(*mut ResetControllerDev,u64)->i32>, deassert: Option<unsafe extern "C" fn(*mut ResetControllerDev,u64)->i32>, status: Option<unsafe extern "C" fn(*mut ResetControllerDev,u64)->i32> }
#[repr(C)] struct Device { of_node: *mut u8 }
#[repr(C)] struct PlatformDevice { dev: Device }
#[repr(C)] struct Regmap;

static GSW_BASE: [u32;2] = [400_000_000,500_000_000];
static EMI_BASE: [u32;2] = [333_000_000,400_000_000];
static BUS_BASE: [u32;2] = [500_000_000,540_000_000];
static SLIC_BASE: [u32;2] = [100_000_000,3_125_000];
static NPU_BASE: [u32;3] = [333_000_000,400_000_000,500_000_000];
static EMI7581_BASE: [u32;4] = [540_000_000,480_000_000,400_000_000,300_000_000];
static BUS7581_BASE: [u32;2] = [600_000_000,540_000_000];
static NPU7581_BASE: [u32;4] = [800_000_000,750_000_000,720_000_000,600_000_000];
static CRYPTO_BASE: [u32;2] = [540_000_000,480_000_000];
static EMMC7581_BASE: [u32;2] = [200_000_000,150_000_000];
static GSW751221_BASE: [u32;4] = [500_000_000,250_000_000,400_000_000,200_000_000];

#[inline] unsafe fn get_base_rate(d: &EnClkDesc, mut val: u32) -> u32 {
    if d.base_bits == 0 { return d.base.base_value; }
    val = (val >> d.base_shift) & ((1 << d.base_bits) - 1);
    if val as usize >= d.n_base_values { 0 } else { *d.base.base_values.add(val as usize) }
}
#[inline] fn get_div(d: &EnClkDesc, mut val: u32) -> u32 {
    if d.div_bits == 0 { return 1; }
    val = (val >> d.div_shift) & ((1 << d.div_bits) - 1);
    if val == 0 && d.div_val0 != 0 { d.div_val0 as u32 } else { (val + d.div_offset as u32) * d.div_step as u32 }
}

unsafe fn reset_update(r: *mut EnRstData, id: usize, assert: bool) -> i32 {
    let offset = *(*r).bank_ofs.add(id / RST_NR_PER_BANK) as usize;
    let addr = (*r).base.add(offset);
    let inverted = offset == REG_NP_SCU_PCIC;
    let val = core::ptr::read_volatile(addr as *const u32);
    let bit = 1u32 << (id % RST_NR_PER_BANK);
    let val = if assert ^ inverted { val | bit } else { val & !bit };
    core::ptr::write_volatile(addr as *mut u32, val);
    0
}
unsafe extern "C" fn reset_assert(r: *mut ResetControllerDev, id: u64) -> i32 {
    reset_update(r as *mut EnRstData, id as usize, true)
}
unsafe extern "C" fn reset_deassert(r: *mut ResetControllerDev, id: u64) -> i32 {
    reset_update(r as *mut EnRstData, id as usize, false)
}
unsafe extern "C" fn reset_status(r: *mut ResetControllerDev, id: u64) -> i32 {
    let d = r as *mut EnRstData;
    let offset = *(*d).bank_ofs.add(id as usize / RST_NR_PER_BANK) as usize;
    let val = core::ptr::read_volatile((*d).base.add(offset) as *const u32) & (1 << (id as usize % RST_NR_PER_BANK));
    if offset == REG_NP_SCU_PCIC { (!val) as i32 } else { (val != 0) as i32 }
}
unsafe fn get_hw_id(np_base: *mut u8) -> EnHir {
    let val = (core::ptr::read_volatile(np_base.add(REG_HIR) as *const u32) & REG_HIR_MASK) >> 16;
    if val < EnHir::Max as u32 { core::mem::transmute(val as i32) } else { EnHir::Unknown }
}
unsafe fn pci_is_enabled(hw: *mut ClkHw) -> i32 {
    let cg = hw as *mut EnClkGate;
    ((core::ptr::read_volatile((*cg).base.add(REG_PCI_CONTROL) as *const u32) & REG_PCI_CONTROL_REFCLK_EN1) != 0) as i32
}
unsafe fn pci_unprepare(hw: *mut ClkHw) {
    let cg = hw as *mut EnClkGate;
    let p = (*cg).base.add(REG_PCI_CONTROL) as *mut u32;
    let v = core::ptr::read_volatile(p);
    core::ptr::write_volatile(p, v & !REG_PCI_CONTROL_REFCLK_EN1);
}

// External kernel entry points and platform data remain declarations, as in the C source.
extern "C" {
    fn en7523_clk_probe(pdev: *mut PlatformDevice) -> i32;
    fn clk_en7523_init() -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
