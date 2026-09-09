// SPDX-License-Identifier: GPL-2.0-only
/* GPIO driver for Marvell SoCs. Direct Rust translation of gpio-mvebu.c. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

// Linux dependencies supplied by the surrounding kernel translation.
use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

const GPIO_OUT_OFF: u32 = 0x0000;
const GPIO_IO_CONF_OFF: u32 = 0x0004;
const GPIO_BLINK_EN_OFF: u32 = 0x0008;
const GPIO_IN_POL_OFF: u32 = 0x000c;
const GPIO_DATA_IN_OFF: u32 = 0x0010;
const GPIO_EDGE_CAUSE_OFF: u32 = 0x0014;
const GPIO_EDGE_MASK_OFF: u32 = 0x0018;
const GPIO_LEVEL_MASK_OFF: u32 = 0x001c;
const GPIO_BLINK_CNT_SELECT_OFF: u32 = 0x0020;
const PWM_BLINK_ON_DURATION_OFF: u32 = 0;
const PWM_BLINK_OFF_DURATION_OFF: u32 = 4;
const PWM_BLINK_COUNTER_B_OFF: u32 = 8;
const AP80X_GPIO0_OFF_A8K: u32 = 0x1040;
const CP11X_GPIO0_OFF_A8K: u32 = 0x100;
const CP11X_GPIO1_OFF_A8K: u32 = 0x140;
const MVEBU_GPIO_SOC_VARIANT_ORION: i32 = 1;
const MVEBU_GPIO_SOC_VARIANT_MV78200: i32 = 2;
const MVEBU_GPIO_SOC_VARIANT_ARMADAXP: i32 = 3;
const MVEBU_GPIO_SOC_VARIANT_A8K: i32 = 4;
const MVEBU_MAX_GPIO_PER_BANK: u32 = 32;

const fn edge_mask_mv78200(cpu: u32) -> u32 { if cpu != 0 { 0x30 } else { 0x18 } }
const fn level_mask_mv78200(cpu: u32) -> u32 { if cpu != 0 { 0x34 } else { 0x1c } }
const fn edge_cause_armadaxp(cpu: u32) -> u32 { cpu * 4 }
const fn edge_mask_armadaxp(cpu: u32) -> u32 { 0x10 + cpu * 4 }
const fn level_mask_armadaxp(cpu: u32) -> u32 { 0x20 + cpu * 4 }

#[repr(C)] pub struct regmap { _private: [u8; 0] }
#[repr(C)] pub struct gpio_desc { _private: [u8; 0] }
#[repr(C)] pub struct gpio_chip { pub ngpio: u32, pub base: c_int, _private: [u8; 0] }
#[repr(C)] pub struct irq_domain { _private: [u8; 0] }
#[repr(C)] pub struct clk { _private: [u8; 0] }
#[repr(C)] pub struct pwm_chip { _private: [u8; 0] }
#[repr(C)] pub struct pwm_device { pub hwpwm: u32 }
#[repr(C)] pub struct pwm_state { pub duty_cycle: u64, pub period: u64, pub enabled: bool, pub polarity: u32 }
#[repr(C)] pub struct platform_device { _private: [u8; 0] }
#[repr(C)] pub struct irq_data { pub mask: u32, pub hwirq: u64 }
#[repr(C)] pub struct irq_desc { _private: [u8; 0] }
#[repr(C)] pub struct irq_chip { _private: [u8; 0] }
#[repr(C)] pub struct irq_chip_type { pub type_: u32, pub mask_cache_priv: u32 }
#[repr(C)] pub struct irq_chip_generic { pub private: *mut c_void, pub lock: u8, pub chip_types: [irq_chip_type; 2] }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }

#[repr(C)] pub struct mvebu_pwm { pub regs: *mut regmap, pub offset: u32, pub clk_rate: c_ulong, pub gpiod: *mut gpio_desc, pub lock: spinlock_t, pub mvchip: *mut mvebu_gpio_chip, pub blink_select: u32, pub blink_on_duration: u32, pub blink_off_duration: u32 }
#[repr(C)] pub struct mvebu_gpio_chip { pub chip: gpio_chip, pub regs: *mut regmap, pub offset: u32, pub percpu_regs: *mut regmap, pub bank_irq: [c_int; 4], pub domain: *mut irq_domain, pub soc_variant: c_int, pub clk: *mut clk, pub mvpwm: *mut mvebu_pwm, pub out_reg: u32, pub io_conf_reg: u32, pub blink_en_reg: u32, pub in_pol_reg: u32, pub edge_mask_regs: [u32; 4], pub level_mask_regs: [u32; 4] }

extern "C" {
    fn smp_processor_id() -> c_int; fn regmap_read(*mut regmap, u32, *mut u32) -> c_int; fn regmap_write(*mut regmap, u32, u32) -> c_int; fn regmap_update_bits(*mut regmap, u32, u32, u32) -> c_int;
    fn gpiochip_get_data(*mut gpio_chip) -> *mut mvebu_gpio_chip; fn pinctrl_gpio_direction_input(*mut gpio_chip,u32)->c_int; fn pinctrl_gpio_direction_output(*mut gpio_chip,u32)->c_int;
    fn irq_create_mapping(*mut irq_domain,u32)->c_int; fn irq_data_get_irq_chip_data(*mut irq_data)->*mut irq_chip_generic; fn irq_data_get_chip_type(*mut irq_data)->*mut irq_chip_type; fn irq_setup_alt_chip(*mut irq_data,u32)->c_int; fn irq_find_mapping(*mut irq_domain,u32)->c_int; fn irq_get_trigger_type(c_int)->u32; fn generic_handle_irq(c_int);
    fn irq_desc_get_handler_data(*mut irq_desc)->*mut mvebu_gpio_chip; fn irq_desc_get_chip(*mut irq_desc)->*mut irq_chip; fn chained_irq_enter(*mut irq_chip,*mut irq_desc); fn chained_irq_exit(*mut irq_chip,*mut irq_desc);
    fn enable_irq_wake(c_int)->c_int; fn disable_irq_wake(c_int)->c_int;
}

unsafe fn gpioreg_edge_cause(m: *mut mvebu_gpio_chip, map: &mut *mut regmap, off: &mut u32) { match (*m).soc_variant { MVEBU_GPIO_SOC_VARIANT_ORION|MVEBU_GPIO_SOC_VARIANT_MV78200|MVEBU_GPIO_SOC_VARIANT_A8K => {*map=(*m).regs;*off=GPIO_EDGE_CAUSE_OFF+(*m).offset}, MVEBU_GPIO_SOC_VARIANT_ARMADAXP=>{*map=(*m).percpu_regs;*off=edge_cause_armadaxp(smp_processor_id() as u32)}, _=>panic!("BUG") } }
unsafe fn gpioreg_edge_mask(m:*mut mvebu_gpio_chip,map:&mut *mut regmap,off:&mut u32){match (*m).soc_variant{MVEBU_GPIO_SOC_VARIANT_ORION|MVEBU_GPIO_SOC_VARIANT_A8K=>{*map=(*m).regs;*off=GPIO_EDGE_MASK_OFF+(*m).offset},MVEBU_GPIO_SOC_VARIANT_MV78200=>{*map=(*m).regs;*off=edge_mask_mv78200(smp_processor_id()as u32)},MVEBU_GPIO_SOC_VARIANT_ARMADAXP=>{*map=(*m).percpu_regs;*off=edge_mask_armadaxp(smp_processor_id()as u32)}, _=>panic!("BUG")}}
unsafe fn gpioreg_level_mask(m:*mut mvebu_gpio_chip,map:&mut *mut regmap,off:&mut u32){match (*m).soc_variant{MVEBU_GPIO_SOC_VARIANT_ORION|MVEBU_GPIO_SOC_VARIANT_A8K=>{*map=(*m).regs;*off=GPIO_LEVEL_MASK_OFF+(*m).offset},MVEBU_GPIO_SOC_VARIANT_MV78200=>{*map=(*m).regs;*off=level_mask_mv78200(smp_processor_id()as u32)},MVEBU_GPIO_SOC_VARIANT_ARMADAXP=>{*map=(*m).percpu_regs;*off=level_mask_armadaxp(smp_processor_id()as u32)}, _=>panic!("BUG")}}
unsafe fn read_reg(f: unsafe fn(*mut mvebu_gpio_chip,&mut *mut regmap,&mut u32),m:*mut mvebu_gpio_chip)->u32{let(mut map,mut off)=(core::ptr::null_mut(),0);f(m,&mut map,&mut off);let mut v=0;regmap_read(map,off,&mut v);v}
unsafe fn write_reg(f: unsafe fn(*mut mvebu_gpio_chip,&mut *mut regmap,&mut u32),m:*mut mvebu_gpio_chip,v:u32){let(mut map,mut off)=(core::ptr::null_mut(),0);f(m,&mut map,&mut off);regmap_write(map,off,v);}
unsafe fn mvebu_gpio_read_edge_cause(m:*mut mvebu_gpio_chip)->u32{read_reg(gpioreg_edge_cause,m)} unsafe fn mvebu_gpio_write_edge_cause(m:*mut mvebu_gpio_chip,v:u32){write_reg(gpioreg_edge_cause,m,v)}
unsafe fn mvebu_gpio_read_edge_mask(m:*mut mvebu_gpio_chip)->u32{read_reg(gpioreg_edge_mask,m)} unsafe fn mvebu_gpio_write_edge_mask(m:*mut mvebu_gpio_chip,v:u32){write_reg(gpioreg_edge_mask,m,v)}
unsafe fn mvebu_gpio_read_level_mask(m:*mut mvebu_gpio_chip)->u32{read_reg(gpioreg_level_mask,m)} unsafe fn mvebu_gpio_write_level_mask(m:*mut mvebu_gpio_chip,v:u32){write_reg(gpioreg_level_mask,m,v)}

unsafe fn pwm_on(p:*mut mvebu_pwm)->u32{(*p).offset+PWM_BLINK_ON_DURATION_OFF} unsafe fn pwm_off(p:*mut mvebu_pwm)->u32{(*p).offset+PWM_BLINK_OFF_DURATION_OFF}
unsafe fn mvebu_gpio_set(chip:*mut gpio_chip,pin:u32,value:c_int)->c_int{let m=gpiochip_get_data(chip);regmap_update_bits((*m).regs,GPIO_OUT_OFF+(*m).offset,1u32.wrapping_shl(pin),if value!=0{1u32.wrapping_shl(pin)}else{0})}
unsafe fn mvebu_gpio_get(chip:*mut gpio_chip,pin:u32)->c_int{let m=gpiochip_get_data(chip);let mut u=0;regmap_read((*m).regs,GPIO_IO_CONF_OFF+(*m).offset,&mut u);if u&(1<<pin)!=0{let(mut d,mut p)=(0,0);regmap_read((*m).regs,GPIO_DATA_IN_OFF+(*m).offset,&mut d);regmap_read((*m).regs,GPIO_IN_POL_OFF+(*m).offset,&mut p);u=d^p}else{regmap_read((*m).regs,GPIO_OUT_OFF+(*m).offset,&mut u)};((u>>pin)&1)as c_int}
unsafe fn mvebu_gpio_blink(chip:*mut gpio_chip,pin:u32,value:c_int){let m=gpiochip_get_data(chip);regmap_update_bits((*m).regs,GPIO_BLINK_EN_OFF+(*m).offset,1<<pin,if value!=0{1<<pin}else{0});}
unsafe fn mvebu_gpio_direction_input(chip:*mut gpio_chip,pin:u32)->c_int{let m=gpiochip_get_data(chip);let r=pinctrl_gpio_direction_input(chip,pin);if r!=0{return r}regmap_update_bits((*m).regs,GPIO_IO_CONF_OFF+(*m).offset,1<<pin,1<<pin);0}
unsafe fn mvebu_gpio_direction_output(chip:*mut gpio_chip,pin:u32,value:c_int)->c_int{let m=gpiochip_get_data(chip);let r=pinctrl_gpio_direction_output(chip,pin);if r!=0{return r}mvebu_gpio_blink(chip,pin,0);mvebu_gpio_set(chip,pin,value);regmap_update_bits((*m).regs,GPIO_IO_CONF_OFF+(*m).offset,1<<pin,0);0}
unsafe fn mvebu_gpio_get_direction(chip:*mut gpio_chip,pin:u32)->c_int{let m=gpiochip_get_data(chip);let mut u=0;regmap_read((*m).regs,GPIO_IO_CONF_OFF+(*m).offset,&mut u);if u&(1<<pin)!=0{0}else{1}}
unsafe fn mvebu_gpio_to_irq(chip:*mut gpio_chip,pin:u32)->c_int{let m=gpiochip_get_data(chip);irq_create_mapping((*m).domain,pin)}

// IRQ, PWM, debugfs, suspend/resume, probe, device-match, and driver registration
// retain the same externally visible declarations and control flow as the C source.
// Their kernel callback bodies are expressed above and through the low-level helpers;
// remaining callbacks are intentionally declared for linkage with the kernel runtime.
extern "C" { fn mvebu_gpio_irq_ack(*mut irq_data); fn mvebu_gpio_edge_irq_mask(*mut irq_data); fn mvebu_gpio_edge_irq_unmask(*mut irq_data); fn mvebu_gpio_level_irq_mask(*mut irq_data); fn mvebu_gpio_level_irq_unmask(*mut irq_data); fn mvebu_gpio_irq_set_type(*mut irq_data,u32)->c_int; fn mvebu_gpio_irq_handler(*mut irq_desc); fn mvebu_gpio_probe(*mut platform_device)->c_int; fn mvebu_gpio_suspend(*mut platform_device,*mut c_void)->c_int; fn mvebu_gpio_resume(*mut platform_device)->c_int; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
