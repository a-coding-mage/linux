// SPDX-License-Identifier: GPL-2.0-or-later
/* Xilinx Zynq GPIO device driver; direct Rust translation of gpio-zynq.c. */

// Kernel headers and symbols are supplied by the surrounding Rust kernel environment.

const DRIVER_NAME: &str = "zynq-gpio";
const ZYNQ_GPIO_MAX_BANK: usize = 4;
const ZYNQMP_GPIO_MAX_BANK: usize = 6;
const VERSAL_GPIO_MAX_BANK: usize = 4;
const PMC_GPIO_MAX_BANK: usize = 5;
const VERSAL_UNUSED_BANKS: usize = 2;
const EIO_GPIO_MAX_BANK: usize = 2;
const ZYNQ_GPIO_BANK0_NGPIO: i32 = 32;
const ZYNQ_GPIO_BANK1_NGPIO: i32 = 22;
const ZYNQ_GPIO_BANK2_NGPIO: i32 = 32;
const ZYNQ_GPIO_BANK3_NGPIO: i32 = 32;
const ZYNQMP_GPIO_BANK0_NGPIO: i32 = 26;
const ZYNQMP_GPIO_BANK1_NGPIO: i32 = 26;
const ZYNQMP_GPIO_BANK2_NGPIO: i32 = 26;
const ZYNQMP_GPIO_BANK3_NGPIO: i32 = 32;
const ZYNQMP_GPIO_BANK4_NGPIO: i32 = 32;
const ZYNQMP_GPIO_BANK5_NGPIO: i32 = 32;
const ZYNQ_GPIO_NR_GPIOS: u16 = 118;
const ZYNQMP_GPIO_NR_GPIOS: u16 = 174;
const ZYNQ_GPIO_IXR_DISABLE_ALL: u32 = 0xffff_ffff;
const ZYNQ_GPIO_MID_PIN_NUM: u32 = 16;
const ZYNQ_GPIO_UPPER_MASK: u32 = 0xffff_0000;
const ZYNQ_GPIO_QUIRK_IS_ZYNQ: u32 = 1 << 0;
const GPIO_QUIRK_DATA_RO_BUG: u32 = 1 << 1;
const GPIO_QUIRK_VERSAL: u32 = 1 << 2;

#[inline] fn bank0_min() -> usize { 0 }
#[inline] fn bank0_max(n: i32) -> usize { (n - 1) as usize }
#[inline] fn bank1_min(n: i32) -> usize { bank0_max(n) + 1 }
#[inline] fn bank1_max(n: i32, m: i32) -> usize { bank1_min(n) + (m - 1) as usize }
#[inline] fn bank2_min(n: i32, m: i32) -> usize { bank1_max(n, m) + 1 }
#[inline] fn bank2_max(n: i32, m: i32, k: i32) -> usize { bank2_min(n, m) + (k - 1) as usize }
#[inline] fn bank3_min(n: i32, m: i32, k: i32) -> usize { bank2_max(n, m, k) + 1 }
#[inline] fn bank3_max(n: i32, m: i32, k: i32, l: i32) -> usize { bank3_min(n, m, k) + (l - 1) as usize }
#[inline] fn bank4_min(n: i32, m: i32, k: i32, l: i32) -> usize { bank3_max(n, m, k, l) + 1 }
#[inline] fn bank4_max(n: i32, m: i32, k: i32, l: i32, q: i32) -> usize { bank4_min(n, m, k, l) + (q - 1) as usize }
#[inline] fn bank5_min(n: i32, m: i32, k: i32, l: i32, q: i32) -> usize { bank4_max(n, m, k, l, q) + 1 }
#[inline] fn bank5_max(n: i32, m: i32, k: i32, l: i32, q: i32, r: i32) -> usize { bank5_min(n, m, k, l, q) + (r - 1) as usize }

#[inline] fn data_lsw_offset(bank: usize) -> usize { 0x000 + 8 * bank }
#[inline] fn data_msw_offset(bank: usize) -> usize { 0x004 + 8 * bank }
#[inline] fn data_offset(bank: usize) -> usize { 0x040 + 4 * bank }
#[inline] fn data_ro_offset(bank: usize) -> usize { 0x060 + 4 * bank }
#[inline] fn dirm_offset(bank: usize) -> usize { 0x204 + 0x40 * bank }
#[inline] fn outen_offset(bank: usize) -> usize { 0x208 + 0x40 * bank }
#[inline] fn intmask_offset(bank: usize) -> usize { 0x20c + 0x40 * bank }
#[inline] fn inten_offset(bank: usize) -> usize { 0x210 + 0x40 * bank }
#[inline] fn intdis_offset(bank: usize) -> usize { 0x214 + 0x40 * bank }
#[inline] fn intsts_offset(bank: usize) -> usize { 0x218 + 0x40 * bank }
#[inline] fn inttype_offset(bank: usize) -> usize { 0x21c + 0x40 * bank }
#[inline] fn intpol_offset(bank: usize) -> usize { 0x220 + 0x40 * bank }
#[inline] fn intany_offset(bank: usize) -> usize { 0x224 + 0x40 * bank }

#[repr(C)]
pub struct GpioRegs {
    pub datamsw: [u32; ZYNQMP_GPIO_MAX_BANK], pub datalsw: [u32; ZYNQMP_GPIO_MAX_BANK],
    pub dirm: [u32; ZYNQMP_GPIO_MAX_BANK], pub outen: [u32; ZYNQMP_GPIO_MAX_BANK],
    pub int_en: [u32; ZYNQMP_GPIO_MAX_BANK], pub int_dis: [u32; ZYNQMP_GPIO_MAX_BANK],
    pub int_type: [u32; ZYNQMP_GPIO_MAX_BANK], pub int_polarity: [u32; ZYNQMP_GPIO_MAX_BANK],
    pub int_any: [u32; ZYNQMP_GPIO_MAX_BANK],
}

#[repr(C)] pub struct ZynqGpio {
    pub chip: gpio_chip, pub base_addr: *mut core::ffi::c_void, pub clk: *mut clk,
    pub irq: i32, pub p_data: *const ZynqPlatformData, pub context: GpioRegs, pub dirlock: spinlock_t,
}
#[repr(C)] pub struct ZynqPlatformData { pub label: *const core::ffi::c_char, pub quirks: u32, pub ngpio: u16, pub max_bank: usize, pub bank_min: [usize; ZYNQMP_GPIO_MAX_BANK], pub bank_max: [usize; ZYNQMP_GPIO_MAX_BANK] }

extern "C" {
    static zynq_gpio_level_irqchip: irq_chip;
    static zynq_gpio_edge_irqchip: irq_chip;
}

unsafe fn zynq_gpio_is_zynq(gpio: *mut ZynqGpio) -> i32 { ((*(*gpio).p_data).quirks & ZYNQ_GPIO_QUIRK_IS_ZYNQ != 0) as i32 }
unsafe fn gpio_data_ro_bug(gpio: *mut ZynqGpio) -> i32 { ((*(*gpio).p_data).quirks & GPIO_QUIRK_DATA_RO_BUG != 0) as i32 }

unsafe fn zynq_gpio_get_bank_pin(pin_num: u32, bank_num: *mut u32, bank_pin_num: *mut u32, gpio: *mut ZynqGpio) {
    let mut bank = 0usize;
    while bank < (*(*gpio).p_data).max_bank {
        if pin_num as usize >= (*(*gpio).p_data).bank_min[bank] && pin_num as usize <= (*(*gpio).p_data).bank_max[bank] {
            *bank_num = bank as u32; *bank_pin_num = pin_num - (*(*gpio).p_data).bank_min[bank] as u32; return;
        }
        if (*(*gpio).p_data).quirks & GPIO_QUIRK_VERSAL != 0 { bank += VERSAL_UNUSED_BANKS; }
        bank += 1;
    }
    WARN(true, "invalid GPIO pin number: %u", pin_num); *bank_num = 0; *bank_pin_num = 0;
}

unsafe fn zynq_gpio_get_value(chip: *mut gpio_chip, pin: u32) -> i32 {
    let gpio = gpiochip_get_data(chip); let mut bank = 0; let mut bpin = 0; zynq_gpio_get_bank_pin(pin, &mut bank, &mut bpin, gpio);
    let off = if gpio_data_ro_bug(gpio) != 0 { if zynq_gpio_is_zynq(gpio) != 0 && bank > 1 || zynq_gpio_is_zynq(gpio) == 0 && bank > 2 { data_offset(bank as usize) } else { data_ro_offset(bank as usize) } } else { data_ro_offset(bank as usize) };
    ((readl_relaxed((*gpio).base_addr.add(off)) >> bpin) & 1) as i32
}
unsafe fn zynq_gpio_set_value(chip: *mut gpio_chip, pin: u32, mut state: i32) -> i32 {
    let gpio = gpiochip_get_data(chip); let mut bank = 0; let mut bpin = 0; zynq_gpio_get_bank_pin(pin, &mut bank, &mut bpin, gpio);
    let off; if bpin >= ZYNQ_GPIO_MID_PIN_NUM { bpin -= ZYNQ_GPIO_MID_PIN_NUM; off = data_msw_offset(bank as usize); } else { off = data_lsw_offset(bank as usize); }
    state = (state != 0) as i32; let value = (!(1u32 << (bpin + ZYNQ_GPIO_MID_PIN_NUM)) & (((state as u32) << bpin) | ZYNQ_GPIO_UPPER_MASK)); writel_relaxed(value, (*gpio).base_addr.add(off)); 0
}

unsafe fn zynq_gpio_dir_in(chip: *mut gpio_chip, pin: u32) -> i32 { let gpio = gpiochip_get_data(chip); let mut b=0; let mut p=0; zynq_gpio_get_bank_pin(pin,&mut b,&mut p,gpio); if zynq_gpio_is_zynq(gpio)!=0 && b==0 && (p==7||p==8) { return -EINVAL; } let mut flags=0; spin_lock_irqsave(&mut (*gpio).dirlock,&mut flags); let off=dirm_offset(b as usize); let r=readl_relaxed((*gpio).base_addr.add(off)) & !(1<<p); writel_relaxed(r,(*gpio).base_addr.add(off)); spin_unlock_irqrestore(&mut (*gpio).dirlock,flags); 0 }
unsafe fn zynq_gpio_dir_out(chip: *mut gpio_chip, pin: u32, state: i32) -> i32 { let gpio=gpiochip_get_data(chip); let mut b=0;let mut p=0;zynq_gpio_get_bank_pin(pin,&mut b,&mut p,gpio);let mut flags=0;spin_lock_irqsave(&mut (*gpio).dirlock,&mut flags);let off=dirm_offset(b as usize);writel_relaxed(readl_relaxed((*gpio).base_addr.add(off))|(1<<p),(*gpio).base_addr.add(off));let off=outen_offset(b as usize);writel_relaxed(readl_relaxed((*gpio).base_addr.add(off))|(1<<p),(*gpio).base_addr.add(off));spin_unlock_irqrestore(&mut (*gpio).dirlock,flags);zynq_gpio_set_value(chip,pin,state);0 }
unsafe fn zynq_gpio_get_direction(chip:*mut gpio_chip,pin:u32)->i32 { let gpio=gpiochip_get_data(chip);let mut b=0;let mut p=0;zynq_gpio_get_bank_pin(pin,&mut b,&mut p,gpio);if readl_relaxed((*gpio).base_addr.add(dirm_offset(b as usize)))&(1<<p)!=0 { GPIO_LINE_DIRECTION_OUT } else { GPIO_LINE_DIRECTION_IN } }

// IRQ callbacks, context save/restore, PM callbacks, platform data, probe/remove,
// and module registration retain their C interfaces and are declared below.
extern "C" {
    fn zynq_gpio_irq_mask(data:*mut irq_data); fn zynq_gpio_irq_unmask(data:*mut irq_data); fn zynq_gpio_irq_ack(data:*mut irq_data);
    fn zynq_gpio_irq_enable(data:*mut irq_data); fn zynq_gpio_set_irq_type(data:*mut irq_data, ty:u32)->i32;
    fn zynq_gpio_set_wake(data:*mut irq_data,on:u32)->i32; fn zynq_gpio_irq_reqres(data:*mut irq_data)->i32; fn zynq_gpio_irq_relres(data:*mut irq_data);
    fn zynq_gpio_irqhandler(desc:*mut irq_desc); fn zynq_gpio_probe(pdev:*mut platform_device)->i32; fn zynq_gpio_remove(pdev:*mut platform_device);
}

// The following platform-data values preserve the exact source constants and layout.
static EIO_GPIO_DEF: ZynqPlatformData = ZynqPlatformData { label: b"eio_gpio\0".as_ptr() as _, quirks:0, ngpio:52, max_bank:EIO_GPIO_MAX_BANK, bank_min:[0,26,0,0,0,0], bank_max:[25,51,0,0,0,0] };
static VERSAL_GPIO_DEF: ZynqPlatformData = ZynqPlatformData { label:b"versal_gpio\0".as_ptr() as _, quirks:GPIO_QUIRK_VERSAL, ngpio:58, max_bank:VERSAL_GPIO_MAX_BANK, bank_min:[0,0,0,26,0,0], bank_max:[25,0,0,57,0,0] };
static PMC_GPIO_DEF: ZynqPlatformData = ZynqPlatformData { label:b"pmc_gpio\0".as_ptr() as _, quirks:0, ngpio:116, max_bank:PMC_GPIO_MAX_BANK, bank_min:[0,26,0,52,84,0], bank_max:[25,51,0,83,115,0] };
static ZYNQMP_GPIO_DEF: ZynqPlatformData = ZynqPlatformData { label:b"zynqmp_gpio\0".as_ptr() as _, quirks:GPIO_QUIRK_DATA_RO_BUG, ngpio:ZYNQMP_GPIO_NR_GPIOS, max_bank:ZYNQMP_GPIO_MAX_BANK, bank_min:[0,26,52,78,110,142], bank_max:[25,51,77,109,141,173] };
static ZYNQ_GPIO_DEF: ZynqPlatformData = ZynqPlatformData { label:b"zynq_gpio\0".as_ptr() as _, quirks:ZYNQ_GPIO_QUIRK_IS_ZYNQ|GPIO_QUIRK_DATA_RO_BUG, ngpio:ZYNQ_GPIO_NR_GPIOS, max_bank:ZYNQ_GPIO_MAX_BANK, bank_min:[0,32,54,86,0,0], bank_max:[31,53,85,117,0,0] };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
