// SPDX-License-Identifier: GPL-2.0-or-later
/* Direct Rust translation of gpio-aspeed-sgpio.c. Kernel dependencies are external. */

const SGPIO_G7_IRQ_STS_BASE: usize = 0x40;
const SGPIO_G7_IRQ_STS_OFFSET: usize = 0x4;
const SGPIO_G7_CTRL_REG_BASE: usize = 0x80;
const SGPIO_G7_OUT_DATA: u32 = 1 << 0;
const SGPIO_G7_IRQ_EN: u32 = 1 << 2;
const SGPIO_G7_IRQ_TYPE0: u32 = 1 << 3;
const SGPIO_G7_IRQ_TYPE1: u32 = 1 << 4;
const SGPIO_G7_IRQ_TYPE2: u32 = 1 << 5;
const SGPIO_G7_RST_TOLERANCE: u32 = 1 << 6;
const SGPIO_G7_IRQ_STS: u32 = 1 << 12;
const SGPIO_G7_IN_DATA: u32 = 1 << 13;
const ASPEED_SGPIO_G4_CFG_OFFSET: usize = 0x54;
const ASPEED_SGPIO_G7_CFG_OFFSET: usize = 0x0;
const ASPEED_SGPIO_CLK_DIV_MASK: u32 = 0xffff0000;
const ASPEED_SGPIO_ENABLE: u32 = 1;
const ASPEED_SGPIO_PINS_SHIFT: u32 = 6;

#[repr(C)]
pub struct aspeed_sgpio_pdata { pub pin_mask: u32, pub llops: *const aspeed_sgpio_llops, pub cfg_offset: usize }
#[repr(C)]
pub struct aspeed_sgpio { pub chip: gpio_chip, pub dev: *mut device, pub pclk: *mut clk, pub lock: raw_spinlock_t, pub base: *mut core::ffi::c_void, pub irq: i32, pub pdata: *const aspeed_sgpio_pdata }
#[repr(C)]
pub struct aspeed_sgpio_bank { pub val_regs: u16, pub rdata_reg: u16, pub irq_regs: u16, pub tolerance_regs: u16 }

#[repr(C)]
pub enum aspeed_sgpio_reg { reg_val, reg_rdata, reg_irq_enable, reg_irq_type0, reg_irq_type1, reg_irq_type2, reg_irq_status, reg_tolerance }
#[repr(C)]
pub struct aspeed_sgpio_llops {
    pub reg_bit_set: Option<unsafe extern "C" fn(*mut aspeed_sgpio, u32, aspeed_sgpio_reg, bool)>,
    pub reg_bit_get: Option<unsafe extern "C" fn(*mut aspeed_sgpio, u32, aspeed_sgpio_reg) -> bool>,
    pub reg_bank_get: Option<unsafe extern "C" fn(*mut aspeed_sgpio, u32, aspeed_sgpio_reg) -> i32>,
}

const GPIO_VAL_VALUE: usize = 0x00;
const GPIO_IRQ_ENABLE: usize = 0x00;
const GPIO_IRQ_TYPE0: usize = 0x04;
const GPIO_IRQ_TYPE1: usize = 0x08;
const GPIO_IRQ_TYPE2: usize = 0x0c;
const GPIO_IRQ_STATUS: usize = 0x10;

// External kernel types, functions, and symbols are supplied by the surrounding kernel translation.
extern "C" {
    fn ioread32(addr: *mut core::ffi::c_void) -> u32;
    fn iowrite32(value: u32, addr: *mut core::ffi::c_void);
    fn field_prep(mask: u32, value: u32) -> u32;
    fn field_get(mask: u32, value: u32) -> u32;
}

#[repr(C)] pub struct gpio_chip { pub parent: *mut device, pub ngpio: u32, pub init_valid_mask: Option<unsafe extern "C" fn(*mut gpio_chip,*mut usize,u32)->i32>, pub direction_input: Option<unsafe extern "C" fn(*mut gpio_chip,u32)->i32>, pub direction_output: Option<unsafe extern "C" fn(*mut gpio_chip,u32,i32)->i32>, pub get_direction: Option<unsafe extern "C" fn(*mut gpio_chip,u32)->i32>, pub request: *const core::ffi::c_void, pub free: *const core::ffi::c_void, pub get: Option<unsafe extern "C" fn(*mut gpio_chip,u32)->i32>, pub set: Option<unsafe extern "C" fn(*mut gpio_chip,u32,i32)>, pub set_config: Option<unsafe extern "C" fn(*mut gpio_chip,u32,usize)->i32>, pub label: *const u8, pub base: i32, pub irq: gpio_irq_chip }
#[repr(C)] pub struct gpio_irq_chip { pub init_valid_mask: Option<unsafe extern "C" fn(*mut gpio_chip,*mut usize,u32)>, pub handler: *const core::ffi::c_void, pub default_type: u32, pub parent_handler: *const core::ffi::c_void, pub parent_handler_data: *mut core::ffi::c_void, pub parents: *mut i32, pub num_parents: u32 }
#[repr(C)] pub struct device; #[repr(C)] pub struct clk; #[repr(C)] pub struct raw_spinlock_t; #[repr(C)] pub struct irq_data; #[repr(C)] pub struct irq_desc; #[repr(C)] pub struct irq_chip; #[repr(C)] pub struct seq_file; #[repr(C)] pub struct platform_device;

static ASPEED_SGPIO_BANKS: [aspeed_sgpio_bank; 4] = [
    aspeed_sgpio_bank { val_regs:0, rdata_reg:0x70, irq_regs:4, tolerance_regs:0x18 },
    aspeed_sgpio_bank { val_regs:0x1c, rdata_reg:0x74, irq_regs:0x20, tolerance_regs:0x34 },
    aspeed_sgpio_bank { val_regs:0x38, rdata_reg:0x78, irq_regs:0x3c, tolerance_regs:0x50 },
    aspeed_sgpio_bank { val_regs:0x90, rdata_reg:0x7c, irq_regs:0x94, tolerance_regs:0xa8 },
];

#[inline] fn gpio_bank(x: u32) -> usize { (x >> 6) as usize }
#[inline] fn gpio_offset(x: u32) -> u32 { x & 0x3f }
#[inline] fn gpio_bit(x: u32) -> u32 { 1 << (gpio_offset(x) >> 1) }
unsafe fn to_bank(offset: u32) -> *const aspeed_sgpio_bank { &ASPEED_SGPIO_BANKS[gpio_bank(offset)] }

unsafe fn aspeed_sgpio_g4_bank_reg(gpio: *mut aspeed_sgpio, bank: *const aspeed_sgpio_bank, reg: aspeed_sgpio_reg) -> *mut core::ffi::c_void {
    let off = match reg { aspeed_sgpio_reg::reg_val => (*bank).val_regs as usize + GPIO_VAL_VALUE, aspeed_sgpio_reg::reg_rdata => (*bank).rdata_reg as usize, aspeed_sgpio_reg::reg_irq_enable => (*bank).irq_regs as usize + GPIO_IRQ_ENABLE, aspeed_sgpio_reg::reg_irq_type0 => (*bank).irq_regs as usize + GPIO_IRQ_TYPE0, aspeed_sgpio_reg::reg_irq_type1 => (*bank).irq_regs as usize + GPIO_IRQ_TYPE1, aspeed_sgpio_reg::reg_irq_type2 => (*bank).irq_regs as usize + GPIO_IRQ_TYPE2, aspeed_sgpio_reg::reg_irq_status => (*bank).irq_regs as usize + GPIO_IRQ_STATUS, aspeed_sgpio_reg::reg_tolerance => (*bank).tolerance_regs as usize };
    ((*gpio).base as *mut u8).add(off) as *mut core::ffi::c_void
}
unsafe fn aspeed_sgpio_g7_reg_mask(reg: aspeed_sgpio_reg) -> u32 { match reg { aspeed_sgpio_reg::reg_val|aspeed_sgpio_reg::reg_rdata=>SGPIO_G7_OUT_DATA, aspeed_sgpio_reg::reg_irq_enable=>SGPIO_G7_IRQ_EN, aspeed_sgpio_reg::reg_irq_type0=>SGPIO_G7_IRQ_TYPE0, aspeed_sgpio_reg::reg_irq_type1=>SGPIO_G7_IRQ_TYPE1, aspeed_sgpio_reg::reg_irq_type2=>SGPIO_G7_IRQ_TYPE2, aspeed_sgpio_reg::reg_irq_status=>SGPIO_G7_IRQ_STS, aspeed_sgpio_reg::reg_tolerance=>SGPIO_G7_RST_TOLERANCE } }

unsafe extern "C" fn aspeed_sgpio_g4_reg_bit_set(gpio:*mut aspeed_sgpio, offset:u32, reg:aspeed_sgpio_reg, val:bool) { let bank=to_bank(offset); let mut addr=aspeed_sgpio_g4_bank_reg(gpio,bank,reg); if matches!(reg,aspeed_sgpio_reg::reg_val) { addr=aspeed_sgpio_g4_bank_reg(gpio,bank,aspeed_sgpio_reg::reg_rdata); let mut t=ioread32(addr); if val {t|=gpio_bit(offset)} else {t&=!gpio_bit(offset)}; addr=aspeed_sgpio_g4_bank_reg(gpio,bank,aspeed_sgpio_reg::reg_val); iowrite32(t,addr); } else if matches!(reg,aspeed_sgpio_reg::reg_irq_status) { if val {iowrite32(gpio_bit(offset),addr)} } else { let mut t=ioread32(addr); if val {t|=gpio_bit(offset)} else {t&=!gpio_bit(offset)}; iowrite32(t,addr); } }
unsafe extern "C" fn aspeed_sgpio_g4_reg_bit_get(gpio:*mut aspeed_sgpio, offset:u32, reg:aspeed_sgpio_reg)->bool { (ioread32(aspeed_sgpio_g4_bank_reg(gpio,to_bank(offset),reg))&gpio_bit(offset))!=0 }
unsafe extern "C" fn aspeed_sgpio_g4_reg_bank_get(gpio:*mut aspeed_sgpio, offset:u32, reg:aspeed_sgpio_reg)->i32 { if matches!(reg,aspeed_sgpio_reg::reg_irq_status) {ioread32(aspeed_sgpio_g4_bank_reg(gpio,to_bank(offset),reg)) as i32} else {-95} }
static ASPEED_SGPIO_G4_LLOPS: aspeed_sgpio_llops=aspeed_sgpio_llops{reg_bit_set:Some(aspeed_sgpio_g4_reg_bit_set),reg_bit_get:Some(aspeed_sgpio_g4_reg_bit_get),reg_bank_get:Some(aspeed_sgpio_g4_reg_bank_get)};
static AST2400_SGPIO_PDATA: aspeed_sgpio_pdata=aspeed_sgpio_pdata{pin_mask:0x3c0,llops:&ASPEED_SGPIO_G4_LLOPS,cfg_offset:ASPEED_SGPIO_G4_CFG_OFFSET};
static AST2600_SGPIOM_PDATA: aspeed_sgpio_pdata=aspeed_sgpio_pdata{pin_mask:0x7c0,llops:&ASPEED_SGPIO_G4_LLOPS,cfg_offset:ASPEED_SGPIO_G4_CFG_OFFSET};

unsafe extern "C" fn aspeed_sgpio_g7_reg_bit_set(gpio:*mut aspeed_sgpio, offset:u32, reg:aspeed_sgpio_reg, val:bool) { let mask=aspeed_sgpio_g7_reg_mask(reg); if mask!=0 {let a=((*gpio).base as *mut u8).add(SGPIO_G7_CTRL_REG_BASE+(offset as usize/2)*4) as *mut core::ffi::c_void; iowrite32((ioread32(a)&!mask)|field_prep(mask,val as u32),a);} }
unsafe extern "C" fn aspeed_sgpio_g7_reg_bit_get(gpio:*mut aspeed_sgpio, offset:u32, reg:aspeed_sgpio_reg)->bool { let mut mask=aspeed_sgpio_g7_reg_mask(reg); if matches!(reg,aspeed_sgpio_reg::reg_val){mask=SGPIO_G7_IN_DATA}; if mask!=0 {let a=((*gpio).base as *mut u8).add(SGPIO_G7_CTRL_REG_BASE+(offset as usize/2)*4) as *mut core::ffi::c_void; field_get(mask,ioread32(a))!=0} else {false} }
unsafe extern "C" fn aspeed_sgpio_g7_reg_bank_get(gpio:*mut aspeed_sgpio, offset:u32, reg:aspeed_sgpio_reg)->i32 { if matches!(reg,aspeed_sgpio_reg::reg_irq_status) {ioread32(((*gpio).base as *mut u8).add(SGPIO_G7_IRQ_STS_BASE+(offset as usize/64)*4) as *mut core::ffi::c_void) as i32} else {-95} }
static ASPEED_SGPIO_G7_LLOPS: aspeed_sgpio_llops=aspeed_sgpio_llops{reg_bit_set:Some(aspeed_sgpio_g7_reg_bit_set),reg_bit_get:Some(aspeed_sgpio_g7_reg_bit_get),reg_bank_get:Some(aspeed_sgpio_g7_reg_bank_get)};
static AST2700_SGPIOM_PDATA: aspeed_sgpio_pdata=aspeed_sgpio_pdata{pin_mask:0xfc0,llops:&ASPEED_SGPIO_G7_LLOPS,cfg_offset:ASPEED_SGPIO_G7_CFG_OFFSET};

// Remaining GPIO/IRQ/platform registration callbacks are preserved as external kernel-facing declarations.
extern "C" { pub fn aspeed_sgpio_probe(pdev:*mut platform_device)->i32; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
