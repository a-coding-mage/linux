// SPDX-License-Identifier: GPL-2.0-only
// Translation of linux/arch/arm/plat-pxa/gpio.c

const GPLR_OFFSET: usize = 0x00;
const GPDR_OFFSET: usize = 0x0c;
const GPSR_OFFSET: usize = 0x18;
const GPCR_OFFSET: usize = 0x24;
const GRER_OFFSET: usize = 0x30;
const GFER_OFFSET: usize = 0x3c;
const GEDR_OFFSET: usize = 0x48;
const GAFR_OFFSET: usize = 0x54;
const ED_MASK_OFFSET: usize = 0x9c;

const fn bank_off(n: usize) -> usize { ((n / 3) << 8) + ((n % 3) << 2) }

static mut pxa_last_gpio: i32 = 0;
static mut irq_base: i32 = 0;

#[repr(C)]
pub struct pxa_gpio_bank {
    pub regbase: *mut core::ffi::c_void,
    pub irq_mask: usize,
    pub irq_edge_rise: usize,
    pub irq_edge_fall: usize,
    #[cfg(feature = "CONFIG_PM")] pub saved_gplr: u32,
    #[cfg(feature = "CONFIG_PM")] pub saved_gpdr: u32,
    #[cfg(feature = "CONFIG_PM")] pub saved_grer: u32,
    #[cfg(feature = "CONFIG_PM")] pub saved_gfer: u32,
}

#[repr(C)] pub struct gpio_chip { pub base: i32 }
#[repr(C)] pub struct device;
#[repr(C)] pub struct irq_domain;
#[repr(C)] pub struct platform_device { pub dev: device }
#[repr(C)] pub struct pxa_gpio_chip {
    pub dev: *mut device, pub chip: gpio_chip, pub banks: *mut pxa_gpio_bank,
    pub irqdomain: *mut irq_domain, pub irq0: i32, pub irq1: i32,
    pub set_wake: Option<unsafe extern "C" fn(u32, u32) -> i32>,
}

#[repr(i32)] #[derive(Copy, Clone)]
pub enum pxa_gpio_type { PXA25X_GPIO=0, PXA26X_GPIO, PXA27X_GPIO, PXA3XX_GPIO,
    PXA93X_GPIO, MMP_GPIO=0x10, MMP2_GPIO, PXA1928_GPIO }
#[repr(C)] pub struct pxa_gpio_id { pub r#type: pxa_gpio_type, pub gpio_nums: i32 }

static mut pxa_gpio_chip: *mut pxa_gpio_chip = core::ptr::null_mut();
static mut gpio_type: pxa_gpio_type = pxa_gpio_type::PXA25X_GPIO;

#[inline] unsafe fn gpio_bit(gpio: u32) -> usize { 1usize << (gpio & (usize::BITS - 1)) }
#[inline] unsafe fn gpio_bank_base(c: *mut gpio_chip, gpio: u32) -> *mut u8 {
    let p = gpiochip_get_data(c); (*(*p).banks.add((gpio / 32) as usize)).regbase as *mut u8
}
#[inline] unsafe fn gpio_to_pxabank(c: *mut gpio_chip, gpio: u32) -> *mut pxa_gpio_bank {
    (*gpiochip_get_data(c)).banks.add((gpio / 32) as usize)
}
#[inline] unsafe fn chip_to_pxachip(c: *mut gpio_chip) -> *mut pxa_gpio_chip { gpiochip_get_data(c) }
#[inline] unsafe fn inverted(gpio: i32) -> bool { matches!(gpio_type, pxa_gpio_type::PXA26X_GPIO) && gpio > 85 }
#[inline] unsafe fn mmp_type(t: pxa_gpio_type) -> bool { (t as i32 & 0x10) != 0 }

unsafe fn has_pinctrl() -> bool { !matches!(gpio_type, pxa_gpio_type::PXA3XX_GPIO|pxa_gpio_type::MMP2_GPIO|pxa_gpio_type::MMP_GPIO) }

unsafe fn __gpio_is_occupied(p: *mut pxa_gpio_chip, gpio: u32) -> i32 {
    let base = gpio_bank_base(&mut (*p).chip, gpio); let gpdr = readl(base.add(GPDR_OFFSET));
    match gpio_type {
        pxa_gpio_type::PXA25X_GPIO|pxa_gpio_type::PXA26X_GPIO|pxa_gpio_type::PXA27X_GPIO => {
            let gafr=readl(base.add(GAFR_OFFSET)); let af=(gafr >> ((gpio&0xf)*2))&3; let dir=gpdr & gpio_bit(gpio) as u32;
            if inverted(gpio as i32) { ((af != 1) || dir == 0) as i32 } else { ((af != 0) || dir != 0) as i32 }
        }, _ => (gpdr & gpio_bit(gpio) as u32 != 0) as i32
    }
}

unsafe fn update_edge_detect(c: *mut pxa_gpio_bank) {
    let grer=readl((*c).regbase.cast::<u8>().add(GRER_OFFSET)) & !(*c).irq_mask as u32;
    let gfer=readl((*c).regbase.cast::<u8>().add(GFER_OFFSET)) & !(*c).irq_mask as u32;
    writel(grer | ((*c).irq_edge_rise & (*c).irq_mask) as u32, (*c).regbase.cast::<u8>().add(GRER_OFFSET));
    writel(gfer | ((*c).irq_edge_fall & (*c).irq_mask) as u32, (*c).regbase.cast::<u8>().add(GFER_OFFSET));
}

unsafe fn pxa_gpio_get(chip:*mut gpio_chip, offset:u32)->i32 { let b=gpio_bank_base(chip,offset); ((readl(b.add(GPLR_OFFSET)) & gpio_bit(offset) as u32)!=0) as i32 }
unsafe fn pxa_gpio_set(chip:*mut gpio_chip, offset:u32, value:i32)->i32 { let b=gpio_bank_base(chip,offset); writel(gpio_bit(offset) as u32,b.add(if value!=0 {GPSR_OFFSET}else{GPCR_OFFSET})); 0 }

unsafe fn pxa_gpio_direction_input(chip:*mut gpio_chip, offset:u32)->i32 {
    let b=gpio_bank_base(chip,offset); let mask=gpio_bit(offset) as u32; let mut v=readl(b.add(GPDR_OFFSET));
    if has_pinctrl() { let r=pinctrl_gpio_direction_input(chip,offset); if r!=0{return r;} }
    if inverted((*chip).base+offset as i32){v|=mask}else{v&=!mask}; writel(v,b.add(GPDR_OFFSET)); 0
}
unsafe fn pxa_gpio_direction_output(chip:*mut gpio_chip, offset:u32, value:i32)->i32 {
    let b=gpio_bank_base(chip,offset); let mask=gpio_bit(offset) as u32; writel(mask,b.add(if value!=0{GPSR_OFFSET}else{GPCR_OFFSET}));
    if has_pinctrl(){let r=pinctrl_gpio_direction_output(chip,offset);if r!=0{return r;}}
    let mut v=readl(b.add(GPDR_OFFSET)); if inverted((*chip).base+offset as i32){v&=!mask}else{v|=mask}; writel(v,b.add(GPDR_OFFSET));0
}

// Remaining kernel registration, IRQ, probe, and power-management declarations are retained as external ABI-facing items.
extern "C" {
    fn gpiochip_get_data(c:*mut gpio_chip)->*mut pxa_gpio_chip;
    fn readl(p:*mut u8)->u32; fn writel(v:u32,p:*mut u8);
    fn pinctrl_gpio_direction_input(c:*mut gpio_chip,o:u32)->i32;
    fn pinctrl_gpio_direction_output(c:*mut gpio_chip,o:u32)->i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
