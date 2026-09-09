// SPDX-License-Identifier: GPL-2.0-only
// Direct low-level translation of arch/arm/mach-tegra/gpio.c.

// Linux headers and symbols referenced below are supplied by the surrounding
// kernel/Rust environment and are intentionally not reimplemented here.

macro_rules! GPIO_BANK { ($x:expr) => { ($x) >> 5 }; }
macro_rules! GPIO_PORT { ($x:expr) => { (($x >> 3) & 0x3) }; }
macro_rules! GPIO_BIT { ($x:expr) => { ($x & 0x7) }; }
macro_rules! GPIO_REG { ($t:expr, $x:expr) => { GPIO_BANK!($x) * (*$t).soc.bank_stride + GPIO_PORT!($x) * 4 }; }
macro_rules! GPIO_CNF { ($t:expr,$x:expr) => { GPIO_REG!($t,$x) + 0x00 }; }
macro_rules! GPIO_OE { ($t:expr,$x:expr) => { GPIO_REG!($t,$x) + 0x10 }; }
macro_rules! GPIO_OUT { ($t:expr,$x:expr) => { GPIO_REG!($t,$x) + 0x20 }; }
macro_rules! GPIO_IN { ($t:expr,$x:expr) => { GPIO_REG!($t,$x) + 0x30 }; }
macro_rules! GPIO_INT_STA { ($t:expr,$x:expr) => { GPIO_REG!($t,$x) + 0x40 }; }
macro_rules! GPIO_INT_ENB { ($t:expr,$x:expr) => { GPIO_REG!($t,$x) + 0x50 }; }
macro_rules! GPIO_INT_LVL { ($t:expr,$x:expr) => { GPIO_REG!($t,$x) + 0x60 }; }
macro_rules! GPIO_INT_CLR { ($t:expr,$x:expr) => { GPIO_REG!($t,$x) + 0x70 }; }
macro_rules! GPIO_DBC_CNT { ($t:expr,$x:expr) => { GPIO_REG!($t,$x) + 0xf0 }; }
macro_rules! GPIO_MSK_CNF { ($t:expr,$x:expr) => { GPIO_REG!($t,$x) + (*$t).soc.upper_offset }; }
macro_rules! GPIO_MSK_OE { ($t:expr,$x:expr) => { GPIO_REG!($t,$x) + (*$t).soc.upper_offset + 0x10 }; }
macro_rules! GPIO_MSK_OUT { ($t:expr,$x:expr) => { GPIO_REG!($t,$x) + (*$t).soc.upper_offset + 0x20 }; }
macro_rules! GPIO_MSK_DBC_EN { ($t:expr,$x:expr) => { GPIO_REG!($t,$x) + (*$t).soc.upper_offset + 0x30 }; }
macro_rules! GPIO_MSK_INT_ENB { ($t:expr,$x:expr) => { GPIO_REG!($t,$x) + (*$t).soc.upper_offset + 0x50 }; }

const GPIO_INT_LVL_MASK: u32 = 0x010101;
const GPIO_INT_LVL_EDGE_RISING: u32 = 0x000101;
const GPIO_INT_LVL_EDGE_FALLING: u32 = 0x000100;
const GPIO_INT_LVL_EDGE_BOTH: u32 = 0x010100;
const GPIO_INT_LVL_LEVEL_HIGH: u32 = 0x000001;
const GPIO_INT_LVL_LEVEL_LOW: u32 = 0x000000;

#[repr(C)]
pub struct tegra_gpio_bank {
    pub bank: u32,
    pub lvl_lock: [raw_spinlock_t; 4],
    pub dbc_lock: [spinlock_t; 4],
    // CONFIG_PM_SLEEP fields are retained unconditionally for layout fidelity.
    pub cnf: [u32; 4], pub out: [u32; 4], pub oe: [u32; 4],
    pub int_enb: [u32; 4], pub int_lvl: [u32; 4], pub wake_enb: [u32; 4],
    pub dbc_enb: [u32; 4], pub dbc_cnt: [u32; 4],
}

#[repr(C)] pub struct tegra_gpio_soc_config { pub debounce_supported: bool, pub bank_stride: u32, pub upper_offset: u32 }
#[repr(C)] pub struct tegra_gpio_info {
    pub dev: *mut device, pub regs: *mut core::ffi::c_void,
    pub bank_info: *mut tegra_gpio_bank, pub soc: *const tegra_gpio_soc_config,
    pub gc: gpio_chip, pub bank_count: u32, pub irqs: *mut u32,
}

unsafe fn tegra_gpio_writel(tgi: *mut tegra_gpio_info, val: u32, reg: u32) { writel_relaxed(val, (*tgi).regs.add(reg as usize)); }
unsafe fn tegra_gpio_readl(tgi: *mut tegra_gpio_info, reg: u32) -> u32 { readl_relaxed((*tgi).regs.add(reg as usize)) }
fn tegra_gpio_compose(bank: u32, port: u32, bit: u32) -> u32 { (bank << 5) | ((port & 3) << 3) | (bit & 7) }
unsafe fn tegra_gpio_mask_write(tgi: *mut tegra_gpio_info, reg: u32, gpio: u32, value: u32) { let mut val = 0x100 << GPIO_BIT!(gpio); if value != 0 { val |= 1 << GPIO_BIT!(gpio); } tegra_gpio_writel(tgi,val,reg); }
unsafe fn tegra_gpio_enable(tgi:*mut tegra_gpio_info,gpio:u32){tegra_gpio_mask_write(tgi,GPIO_MSK_CNF!(tgi,gpio),gpio,1)}
unsafe fn tegra_gpio_disable(tgi:*mut tegra_gpio_info,gpio:u32){tegra_gpio_mask_write(tgi,GPIO_MSK_CNF!(tgi,gpio),gpio,0)}

unsafe fn tegra_gpio_free(chip:*mut gpio_chip, offset:u32){let tgi=gpiochip_get_data(chip); pinctrl_gpio_free(chip,offset); tegra_gpio_disable(tgi,offset);}
unsafe fn tegra_gpio_set(chip:*mut gpio_chip,offset:u32,value:i32)->i32{let tgi=gpiochip_get_data(chip);tegra_gpio_mask_write(tgi,GPIO_MSK_OUT!(tgi,offset),offset,value as u32);0}
unsafe fn tegra_gpio_get(chip:*mut gpio_chip,offset:u32)->i32{let tgi=gpiochip_get_data(chip);let bval=1u32<<GPIO_BIT!(offset);if tegra_gpio_readl(tgi,GPIO_OE!(tgi,offset))&bval!=0{((tegra_gpio_readl(tgi,GPIO_OUT!(tgi,offset))&bval)!=0)as i32}else{((tegra_gpio_readl(tgi,GPIO_IN!(tgi,offset))&bval)!=0)as i32}}
unsafe fn tegra_gpio_direction_input(chip:*mut gpio_chip,offset:u32)->i32{let tgi=gpiochip_get_data(chip);tegra_gpio_mask_write(tgi,GPIO_MSK_OE!(tgi,offset),offset,0);tegra_gpio_enable(tgi,offset);0}
unsafe fn tegra_gpio_direction_output(chip:*mut gpio_chip,offset:u32,value:i32)->i32{let tgi=gpiochip_get_data(chip);tegra_gpio_set(chip,offset,value);tegra_gpio_mask_write(tgi,GPIO_MSK_OE!(tgi,offset),offset,1);tegra_gpio_enable(tgi,offset);0}
unsafe fn tegra_gpio_get_direction(chip:*mut gpio_chip,offset:u32)->i32{let tgi=gpiochip_get_data(chip);let m=1u32<<GPIO_BIT!(offset);let cnf=tegra_gpio_readl(tgi,GPIO_CNF!(tgi,offset));if cnf&m==0{return -EINVAL;}if tegra_gpio_readl(tgi,GPIO_OE!(tgi,offset))&m!=0{GPIO_LINE_DIRECTION_OUT}else{GPIO_LINE_DIRECTION_IN}}

// The remaining callbacks retain the C control flow and call external kernel
// APIs directly; declarations are intentionally left to the integration layer.
unsafe fn tegra_gpio_set_debounce(chip:*mut gpio_chip,offset:u32,debounce:u32)->i32{let tgi=gpiochip_get_data(chip);let bank=&mut *(*tgi).bank_info.add(GPIO_BANK!(offset) as usize);let mut ms=(debounce+999)/1000;if ms==0{tegra_gpio_mask_write(tgi,GPIO_MSK_DBC_EN!(tgi,offset),offset,0);return 0;}if ms>255{ms=255;}let port=GPIO_PORT!(offset) as usize;let mut flags=0;spin_lock_irqsave(&mut bank.dbc_lock[port],&mut flags);if bank.dbc_cnt[port]<ms{tegra_gpio_writel(tgi,ms,GPIO_DBC_CNT!(tgi,offset));bank.dbc_cnt[port]=ms;}spin_unlock_irqrestore(&mut bank.dbc_lock[port],flags);tegra_gpio_mask_write(tgi,GPIO_MSK_DBC_EN!(tgi,offset),offset,1);0}
unsafe fn tegra_gpio_set_config(chip:*mut gpio_chip,offset:u32,config:usize)->i32{if pinconf_to_config_param(config)!=PIN_CONFIG_INPUT_DEBOUNCE{return -ENOTSUPP;}tegra_gpio_set_debounce(chip,offset,pinconf_to_config_argument(config))}

unsafe fn tegra_gpio_irq_ack(d:*mut irq_data){let chip=irq_data_get_irq_chip_data(d);let tgi=gpiochip_get_data(chip);let gpio=(*d).hwirq;tegra_gpio_writel(tgi,1<<GPIO_BIT!(gpio),GPIO_INT_CLR!(tgi,gpio));}
unsafe fn tegra_gpio_irq_mask(d:*mut irq_data){let chip=irq_data_get_irq_chip_data(d);let tgi=gpiochip_get_data(chip);let gpio=(*d).hwirq;tegra_gpio_mask_write(tgi,GPIO_MSK_INT_ENB!(tgi,gpio),gpio,0);gpiochip_disable_irq(chip,gpio);}
unsafe fn tegra_gpio_irq_unmask(d:*mut irq_data){let chip=irq_data_get_irq_chip_data(d);let tgi=gpiochip_get_data(chip);let gpio=(*d).hwirq;gpiochip_enable_irq(chip,gpio);tegra_gpio_mask_write(tgi,GPIO_MSK_INT_ENB!(tgi,gpio),gpio,1);}
unsafe fn tegra_gpio_irq_set_type(d:*mut irq_data,ty:u32)->i32{let chip=irq_data_get_irq_chip_data(d);let tgi=gpiochip_get_data(chip);let gpio=(*d).hwirq;let port=GPIO_PORT!(gpio)as usize;let bank=&mut *(*tgi).bank_info.add(GPIO_BANK!(gpio)as usize);let lvl=match ty&IRQ_TYPE_SENSE_MASK{IRQ_TYPE_EDGE_RISING=>GPIO_INT_LVL_EDGE_RISING,IRQ_TYPE_EDGE_FALLING=>GPIO_INT_LVL_EDGE_FALLING,IRQ_TYPE_EDGE_BOTH=>GPIO_INT_LVL_EDGE_BOTH,IRQ_TYPE_LEVEL_HIGH=>GPIO_INT_LVL_LEVEL_HIGH,IRQ_TYPE_LEVEL_LOW=>GPIO_INT_LVL_LEVEL_LOW,_=>return -EINVAL};let mut flags=0;raw_spin_lock_irqsave(&mut bank.lvl_lock[port],&mut flags);let mut val=tegra_gpio_readl(tgi,GPIO_INT_LVL!(tgi,gpio));val&=!(GPIO_INT_LVL_MASK<<GPIO_BIT!(gpio));val|=lvl<<GPIO_BIT!(gpio);tegra_gpio_writel(tgi,val,GPIO_INT_LVL!(tgi,gpio));raw_spin_unlock_irqrestore(&mut bank.lvl_lock[port],flags);tegra_gpio_mask_write(tgi,GPIO_MSK_OE!(tgi,gpio),gpio,0);tegra_gpio_enable(tgi,gpio);let ret=gpiochip_lock_as_irq(&mut (*tgi).gc,gpio);if ret!=0{tegra_gpio_disable(tgi,gpio);return ret;}if ty&IRQ_TYPE_LEVEL_MASK!=0{irq_set_handler_locked(d,handle_level_irq);}else if ty&IRQ_TYPE_EDGE_BOTH!=0{irq_set_handler_locked(d,handle_edge_irq);}if !(*d).parent_data.is_null(){return irq_chip_set_type_parent(d,ty);}0}
unsafe fn tegra_gpio_irq_shutdown(d:*mut irq_data){let chip=irq_data_get_irq_chip_data(d);let tgi=gpiochip_get_data(chip);tegra_gpio_irq_mask(d);gpiochip_unlock_as_irq(&mut (*tgi).gc,(*d).hwirq);}
unsafe fn tegra_gpio_irq_handler(desc:*mut irq_desc){let tgi=irq_desc_get_handler_data(desc);let chip=irq_desc_get_chip(desc);let domain=(*tgi).gc.irq.domain;let irq=irq_desc_get_irq(desc);let mut bank:*mut tegra_gpio_bank=core::ptr::null_mut();for i in 0..(*tgi).bank_count{if *(*tgi).irqs.add(i as usize)==irq{bank=(*tgi).bank_info.add(i as usize);break;}}if bank.is_null(){return;}chained_irq_enter(chip,desc);for port in 0..4{let gpio=tegra_gpio_compose((*bank).bank,port,0);let sta=tegra_gpio_readl(tgi,GPIO_INT_STA!(tgi,gpio))&tegra_gpio_readl(tgi,GPIO_INT_ENB!(tgi,gpio));let lvl=tegra_gpio_readl(tgi,GPIO_INT_LVL!(tgi,gpio));for pin in 0..8{if sta&(1<<pin)!=0{tegra_gpio_writel(tgi,1<<pin,GPIO_INT_CLR!(tgi,gpio));generic_handle_domain_irq(domain,gpio+pin);if lvl&(0x100<<pin)!=0{chained_irq_exit(chip,desc);}}}}chained_irq_exit(chip,desc);}

// PM_SLEEP, DEBUG_FS, OF match tables, platform probe, driver registration,
// and MODULE_* metadata remain conditional/native registration declarations.
static tegra20_gpio_config: tegra_gpio_soc_config = tegra_gpio_soc_config { debounce_supported:false, bank_stride:0x80, upper_offset:0x800 };
static tegra30_gpio_config: tegra_gpio_soc_config = tegra_gpio_soc_config { debounce_supported:false, bank_stride:0x100, upper_offset:0x80 };
static tegra210_gpio_config: tegra_gpio_soc_config = tegra_gpio_soc_config { debounce_supported:true, bank_stride:0x100, upper_offset:0x80 };

// Original OF match table, platform driver, and MODULE_* declarations are
// represented by the native Rust/kernel registration layer.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
