/*
 * GPIO functions for Au1000, Au1500, Au1100, Au1550, Au1200
 *
 * Copyright (c) 2009 Manuel Lauss.
 *
 * Licensed under the terms outlined in the file COPYING.
 */

// Dependency: <asm/mach-au1x00/au1000.h>

pub const ALCHEMY_GPIO1_BASE: i32 = 0;
pub const ALCHEMY_GPIO2_BASE: i32 = 200;
pub const ALCHEMY_GPIO1_NUM: i32 = 32;
pub const ALCHEMY_GPIO2_NUM: i32 = 16;
pub const ALCHEMY_GPIO1_MAX: i32 = ALCHEMY_GPIO1_BASE + ALCHEMY_GPIO1_NUM - 1;
pub const ALCHEMY_GPIO2_MAX: i32 = ALCHEMY_GPIO2_BASE + ALCHEMY_GPIO2_NUM - 1;

pub const AU1000_SYS_TRIOUTRD: usize = 0x100;
pub const AU1000_SYS_TRIOUTCLR: usize = 0x100;
pub const AU1000_SYS_OUTPUTRD: usize = 0x108;
pub const AU1000_SYS_OUTPUTSET: usize = 0x108;
pub const AU1000_SYS_OUTPUTCLR: usize = 0x10C;
pub const AU1000_SYS_PINSTATERD: usize = 0x110;
pub const AU1000_SYS_PININPUTEN: usize = 0x110;

pub const AU1000_GPIO2_DIR: usize = 0x00;
pub const AU1000_GPIO2_OUTPUT: usize = 0x08;
pub const AU1000_GPIO2_PINSTATE: usize = 0x0C;
pub const AU1000_GPIO2_INTENABLE: usize = 0x10;
pub const AU1000_GPIO2_ENABLE: usize = 0x14;

#[repr(C)]
pub struct software_node {
    _private: [u8; 0],
}
extern "C" {
    pub static alchemy_gpio1_node: software_node;
    pub static alchemy_gpio2_node: software_node;
}

#[inline]
pub fn MAKE_IRQ(intc: i32, off: i32) -> i32 {
    if intc == 0 { AU1000_INTC0_INT_BASE + off } else { AU1000_INTC1_INT_BASE + off }
}

#[inline] pub fn au1000_gpio1_to_irq(gpio: i32) -> i32 { MAKE_IRQ(1, gpio - ALCHEMY_GPIO1_BASE) }
#[inline] pub fn au1000_gpio2_to_irq(_gpio: i32) -> i32 { -ENXIO }
#[inline]
pub fn au1000_irq_to_gpio(irq: i32) -> i32 {
    if (AU1000_GPIO0_INT..=AU1000_GPIO31_INT).contains(&irq) { ALCHEMY_GPIO1_BASE + (irq - AU1000_GPIO0_INT) } else { -ENXIO }
}

#[inline]
pub fn au1500_gpio1_to_irq(mut gpio: i32) -> i32 { gpio -= ALCHEMY_GPIO1_BASE; match gpio { 0..=15 | 20 | 23..=28 => MAKE_IRQ(1, gpio), _ => -ENXIO } }
#[inline]
pub fn au1500_gpio2_to_irq(mut gpio: i32) -> i32 { gpio -= ALCHEMY_GPIO2_BASE; match gpio { 0..=3 => MAKE_IRQ(1, 16 + gpio), 4..=5 => MAKE_IRQ(1, 21 + gpio - 4), 6..=7 => MAKE_IRQ(1, 29 + gpio - 6), _ => -ENXIO } }
#[inline]
pub fn au1500_irq_to_gpio(irq: i32) -> i32 { match irq {
    AU1500_GPIO0_INT..=AU1500_GPIO15_INT | AU1500_GPIO20_INT | AU1500_GPIO23_INT..=AU1500_GPIO28_INT => ALCHEMY_GPIO1_BASE + (irq - AU1500_GPIO0_INT),
    AU1500_GPIO200_INT..=AU1500_GPIO203_INT => ALCHEMY_GPIO2_BASE + (irq - AU1500_GPIO200_INT),
    AU1500_GPIO204_INT..=AU1500_GPIO205_INT => ALCHEMY_GPIO2_BASE + (irq - AU1500_GPIO204_INT) + 4,
    AU1500_GPIO206_INT..=AU1500_GPIO207_INT => ALCHEMY_GPIO2_BASE + (irq - AU1500_GPIO206_INT) + 6,
    AU1500_GPIO208_215_INT => ALCHEMY_GPIO2_BASE + 8, _ => -ENXIO } }

#[inline] pub fn au1100_gpio1_to_irq(gpio: i32) -> i32 { MAKE_IRQ(1, gpio - ALCHEMY_GPIO1_BASE) }
#[inline] pub fn au1100_gpio2_to_irq(mut gpio: i32) -> i32 { gpio -= ALCHEMY_GPIO2_BASE; if (8..=15).contains(&gpio) { MAKE_IRQ(0, 29) } else { -ENXIO } }
#[inline] pub fn au1100_irq_to_gpio(irq: i32) -> i32 { match irq { AU1100_GPIO0_INT..=AU1100_GPIO31_INT => ALCHEMY_GPIO1_BASE + irq - AU1100_GPIO0_INT, AU1100_GPIO208_215_INT => ALCHEMY_GPIO2_BASE + 8, _ => -ENXIO } }

#[inline]
pub fn au1550_gpio1_to_irq(mut gpio: i32) -> i32 { gpio -= ALCHEMY_GPIO1_BASE; match gpio { 0..=15 | 20..=28 => MAKE_IRQ(1, gpio), 16..=17 => MAKE_IRQ(1, 18 + gpio - 16), _ => -ENXIO } }
#[inline]
pub fn au1550_gpio2_to_irq(mut gpio: i32) -> i32 { gpio -= ALCHEMY_GPIO2_BASE; match gpio { 0 => MAKE_IRQ(1,16), 1..=5 => MAKE_IRQ(1,17), 6..=7 => MAKE_IRQ(1,29 + gpio - 6), 8..=15 => MAKE_IRQ(1,31), _ => -ENXIO } }
#[inline]
pub fn au1550_irq_to_gpio(irq: i32) -> i32 { match irq {
    AU1550_GPIO0_INT..=AU1550_GPIO15_INT => ALCHEMY_GPIO1_BASE + irq - AU1550_GPIO0_INT,
    AU1550_GPIO200_INT | AU1550_GPIO201_205_INT => ALCHEMY_GPIO2_BASE + irq - AU1550_GPIO200_INT,
    AU1550_GPIO16_INT..=AU1550_GPIO28_INT => ALCHEMY_GPIO1_BASE + irq - AU1550_GPIO16_INT + 16,
    AU1550_GPIO206_INT..=AU1550_GPIO208_215_INT => ALCHEMY_GPIO2_BASE + irq - AU1550_GPIO206_INT + 6, _ => -ENXIO } }

#[inline] pub fn au1200_gpio1_to_irq(gpio: i32) -> i32 { MAKE_IRQ(1, gpio - ALCHEMY_GPIO1_BASE) }
#[inline] pub fn au1200_gpio2_to_irq(mut gpio: i32) -> i32 { gpio -= ALCHEMY_GPIO2_BASE; match gpio { 0..=2 => MAKE_IRQ(0,5+gpio), 3 => MAKE_IRQ(0,22), 4..=7 => MAKE_IRQ(0,24+gpio-4), 8..=15 => MAKE_IRQ(0,28), _ => -ENXIO } }
#[inline] pub fn au1200_irq_to_gpio(irq: i32) -> i32 { match irq { AU1200_GPIO0_INT..=AU1200_GPIO31_INT => ALCHEMY_GPIO1_BASE + irq - AU1200_GPIO0_INT, AU1200_GPIO200_INT..=AU1200_GPIO202_INT => ALCHEMY_GPIO2_BASE + irq - AU1200_GPIO200_INT, AU1200_GPIO203_INT => ALCHEMY_GPIO2_BASE + 3, AU1200_GPIO204_INT..=AU1200_GPIO208_215_INT => ALCHEMY_GPIO2_BASE + irq - AU1200_GPIO204_INT + 4, _ => -ENXIO } }

extern "C" {
    fn alchemy_wrsys(value: usize, reg: usize);
    fn alchemy_rdsys(reg: usize) -> usize;
    fn alchemy_get_cputype() -> i32;
    fn KSEG1ADDR(addr: usize) -> usize;
    fn __raw_writel(value: u32, addr: *mut u8);
    fn __raw_readl(addr: *const u8) -> u32;
    fn wmb();
    fn local_irq_save(flags: *mut usize);
    fn local_irq_restore(flags: usize);
}

#[inline] pub fn alchemy_gpio1_set_value(gpio: i32, v: i32) { let mask = 1usize << (gpio - ALCHEMY_GPIO1_BASE); alchemy_wrsys(mask, if v != 0 { AU1000_SYS_OUTPUTSET } else { AU1000_SYS_OUTPUTCLR }); }
#[inline] pub fn alchemy_gpio1_get_value(gpio: i32) -> i32 { (alchemy_rdsys(AU1000_SYS_PINSTATERD) & (1usize << (gpio - ALCHEMY_GPIO1_BASE))) as i32 }
#[inline] pub fn alchemy_gpio1_direction_input(gpio: i32) -> i32 { alchemy_wrsys(1usize << (gpio - ALCHEMY_GPIO1_BASE), AU1000_SYS_TRIOUTCLR); 0 }
#[inline] pub fn alchemy_gpio1_direction_output(gpio: i32, v: i32) -> i32 { alchemy_gpio1_set_value(gpio,v); 0 }
#[inline] pub fn alchemy_gpio1_is_valid(gpio: i32) -> i32 { (gpio >= ALCHEMY_GPIO1_BASE && gpio <= ALCHEMY_GPIO1_MAX) as i32 }

#[inline] pub fn alchemy_gpio1_to_irq(gpio: i32) -> i32 { match unsafe { alchemy_get_cputype() } { ALCHEMY_CPU_AU1000 => au1000_gpio1_to_irq(gpio), ALCHEMY_CPU_AU1100 => au1100_gpio1_to_irq(gpio), ALCHEMY_CPU_AU1500 => au1500_gpio1_to_irq(gpio), ALCHEMY_CPU_AU1550 => au1550_gpio1_to_irq(gpio), ALCHEMY_CPU_AU1200 => au1200_gpio1_to_irq(gpio), _ => -ENXIO } }

#[inline] pub fn alchemy_gpio1_input_enable() { let base = KSEG1ADDR(AU1000_SYS_PHYS_ADDR) as *mut u8; __raw_writel(0, unsafe { base.add(0x110) }); wmb(); }

#[inline] pub fn __alchemy_gpio2_mod_dir(gpio: i32, to_out: i32) { let base=KSEG1ADDR(AU1500_GPIO2_PHYS_ADDR) as *mut u8; let mask=1u32 << (gpio-ALCHEMY_GPIO2_BASE); let mut d=__raw_readl(unsafe{base.add(AU1000_GPIO2_DIR)}); if to_out!=0 {d|=mask} else {d&=!mask}; __raw_writel(d,unsafe{base.add(AU1000_GPIO2_DIR)}); wmb(); }
#[inline] pub fn alchemy_gpio2_set_value(gpio:i32,v:i32) { let base=KSEG1ADDR(AU1500_GPIO2_PHYS_ADDR) as *mut u8; let mask=(if v!=0 {0x00010001} else {0x00010000}) << (gpio-ALCHEMY_GPIO2_BASE); __raw_writel(mask,unsafe{base.add(AU1000_GPIO2_OUTPUT)}); wmb(); }
#[inline] pub fn alchemy_gpio2_get_value(gpio:i32)->i32 { let base=KSEG1ADDR(AU1500_GPIO2_PHYS_ADDR) as *mut u8; (__raw_readl(unsafe{base.add(AU1000_GPIO2_PINSTATE)}) & (1u32 << (gpio-ALCHEMY_GPIO2_BASE))) as i32 }
#[inline] pub fn alchemy_gpio2_direction_input(gpio:i32)->i32 { let mut flags=0usize; local_irq_save(&mut flags); __alchemy_gpio2_mod_dir(gpio,0); local_irq_restore(flags); 0 }
#[inline] pub fn alchemy_gpio2_direction_output(gpio:i32,v:i32)->i32 { let mut flags=0usize; alchemy_gpio2_set_value(gpio,v); local_irq_save(&mut flags); __alchemy_gpio2_mod_dir(gpio,1); local_irq_restore(flags); 0 }
#[inline] pub fn alchemy_gpio2_is_valid(gpio:i32)->i32 {(gpio>=ALCHEMY_GPIO2_BASE&&gpio<=ALCHEMY_GPIO2_MAX) as i32}
#[inline] pub fn alchemy_gpio2_to_irq(gpio:i32)->i32 { match unsafe{alchemy_get_cputype()} { ALCHEMY_CPU_AU1000=>au1000_gpio2_to_irq(gpio), ALCHEMY_CPU_AU1100=>au1100_gpio2_to_irq(gpio), ALCHEMY_CPU_AU1500=>au1500_gpio2_to_irq(gpio), ALCHEMY_CPU_AU1550=>au1550_gpio2_to_irq(gpio), ALCHEMY_CPU_AU1200=>au1200_gpio2_to_irq(gpio), _=>-ENXIO } }

#[inline] pub fn __alchemy_gpio2_mod_int(gpio2:i32,en:i32) { let base=KSEG1ADDR(AU1500_GPIO2_PHYS_ADDR) as *mut u8; let mut r=__raw_readl(unsafe{base.add(AU1000_GPIO2_INTENABLE)}); if en!=0 {r|=1u32<<gpio2} else {r&=!(1u32<<gpio2)}; __raw_writel(r,unsafe{base.add(AU1000_GPIO2_INTENABLE)}); wmb(); }
#[inline] pub fn alchemy_gpio2_enable_int(mut gpio2:i32) { let mut flags=0usize; gpio2-=ALCHEMY_GPIO2_BASE; match unsafe{alchemy_get_cputype()} { ALCHEMY_CPU_AU1100|ALCHEMY_CPU_AU1500=>gpio2-=8, _=>{} } local_irq_save(&mut flags); __alchemy_gpio2_mod_int(gpio2,1); local_irq_restore(flags); }
#[inline] pub fn alchemy_gpio2_disable_int(mut gpio2:i32) { let mut flags=0usize; gpio2-=ALCHEMY_GPIO2_BASE; match unsafe{alchemy_get_cputype()} { ALCHEMY_CPU_AU1100|ALCHEMY_CPU_AU1500=>gpio2-=8, _=>{} } local_irq_save(&mut flags); __alchemy_gpio2_mod_int(gpio2,0); local_irq_restore(flags); }
#[inline] pub fn alchemy_gpio2_enable() { let base=KSEG1ADDR(AU1500_GPIO2_PHYS_ADDR) as *mut u8; __raw_writel(3,unsafe{base.add(AU1000_GPIO2_ENABLE)}); wmb(); __raw_writel(1,unsafe{base.add(AU1000_GPIO2_ENABLE)}); wmb(); }
#[inline] pub fn alchemy_gpio2_disable() { let base=KSEG1ADDR(AU1500_GPIO2_PHYS_ADDR) as *mut u8; __raw_writel(2,unsafe{base.add(AU1000_GPIO2_ENABLE)}); wmb(); }

#[inline] pub fn alchemy_gpio_direction_input(gpio:i32)->i32 { if gpio>=ALCHEMY_GPIO2_BASE {alchemy_gpio2_direction_input(gpio)} else {alchemy_gpio1_direction_input(gpio)} }
#[inline] pub fn alchemy_gpio_direction_output(gpio:i32,v:i32)->i32 { if gpio>=ALCHEMY_GPIO2_BASE {alchemy_gpio2_direction_output(gpio,v)} else {alchemy_gpio1_direction_output(gpio,v)} }
#[inline] pub fn alchemy_gpio_get_value(gpio:i32)->i32 { if gpio>=ALCHEMY_GPIO2_BASE {alchemy_gpio2_get_value(gpio)} else {alchemy_gpio1_get_value(gpio)} }
#[inline] pub fn alchemy_gpio_set_value(gpio:i32,v:i32) { if gpio>=ALCHEMY_GPIO2_BASE {alchemy_gpio2_set_value(gpio,v)} else {alchemy_gpio1_set_value(gpio,v)} }
#[inline] pub fn alchemy_gpio_is_valid(gpio:i32)->i32 { if gpio>=ALCHEMY_GPIO2_BASE {alchemy_gpio2_is_valid(gpio)} else {alchemy_gpio1_is_valid(gpio)} }
#[inline] pub fn alchemy_gpio_to_irq(gpio:i32)->i32 { if gpio>=ALCHEMY_GPIO2_BASE {alchemy_gpio2_to_irq(gpio)} else {alchemy_gpio1_to_irq(gpio)} }
#[inline] pub fn alchemy_irq_to_gpio(irq:i32)->i32 { match unsafe{alchemy_get_cputype()} { ALCHEMY_CPU_AU1000=>au1000_irq_to_gpio(irq), ALCHEMY_CPU_AU1100=>au1100_irq_to_gpio(irq), ALCHEMY_CPU_AU1500=>au1500_irq_to_gpio(irq), ALCHEMY_CPU_AU1550=>au1550_irq_to_gpio(irq), ALCHEMY_CPU_AU1200=>au1200_irq_to_gpio(irq), _=>-ENXIO } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
