// SPDX-License-Identifier: GPL-2.0-only
/* Broadcom Kona GPIO Driver */

const BCM_GPIO_PASSWD: u32 = 0x00a5a501;
const GPIO_PER_BANK: usize = 32;
const GPIO_MAX_BANK_NUM: i32 = 8;
const LOCK_CODE: u32 = 0xffff_ffff;
const UNLOCK_CODE: u32 = 0;

const fn gpio_bank(gpio: usize) -> usize { gpio >> 5 }
const fn gpio_bit(gpio: usize) -> usize { gpio & (GPIO_PER_BANK - 1) }
const fn gpio_control(gpio: usize) -> usize { 0x100 + (gpio << 2) }
const fn gpio_out_status(bank: usize) -> usize { (bank << 2) }
const fn gpio_in_status(bank: usize) -> usize { 0x20 + (bank << 2) }
const fn gpio_out_set(bank: usize) -> usize { 0x40 + (bank << 2) }
const fn gpio_out_clear(bank: usize) -> usize { 0x60 + (bank << 2) }
const fn gpio_int_status(bank: usize) -> usize { 0x80 + (bank << 2) }
const fn gpio_int_mask(bank: usize) -> usize { 0xa0 + (bank << 2) }
const fn gpio_int_mskclr(bank: usize) -> usize { 0xc0 + (bank << 2) }
const fn gpio_pwd_status(bank: usize) -> usize { 0x500 + (bank << 2) }
const GPIO_GPPWR_OFFSET: usize = 0x520;
const GPIO_GPCTR0_DBR_SHIFT: u32 = 5;
const GPIO_GPCTR0_DBR_MASK: u32 = 0x1e0;
const GPIO_GPCTR0_ITR_SHIFT: u32 = 3;
const GPIO_GPCTR0_ITR_MASK: u32 = 0x18;
const GPIO_GPCTR0_ITR_CMD_RISING_EDGE: u32 = 1;
const GPIO_GPCTR0_ITR_CMD_FALLING_EDGE: u32 = 2;
const GPIO_GPCTR0_ITR_CMD_BOTH_EDGE: u32 = 3;
const GPIO_GPCTR0_IOTR_MASK: u32 = 1;
const GPIO_GPCTR0_IOTR_CMD_0UTPUT: u32 = 0;
const GPIO_GPCTR0_IOTR_CMD_INPUT: u32 = 1;
const GPIO_GPCTR0_DB_ENABLE_MASK: u32 = 0x100;

#[repr(C)] pub struct RawSpinLock { _private: [u8; 0] }
#[repr(C)] pub struct GpioChip { pub parent: *mut Device, pub ngpio: u32 }
#[repr(C)] pub struct IrqDomain { _private: [u8; 0] }
#[repr(C)] pub struct Device { _private: [u8; 0] }
#[repr(C)] pub struct PlatformDevice { pub dev: Device }
#[repr(C)] pub struct IrqData { pub hwirq: u32 }
#[repr(C)] pub struct IrqDesc { _private: [u8; 0] }
#[repr(C)] pub struct IrqChip { _private: [u8; 0] }
#[repr(C)] pub struct LockClassKey { _private: [u8; 0] }

#[repr(C)] pub struct BcmKonaGpioBank { pub id: i32, pub irq: i32, pub gpio_unlock_count: [u8; GPIO_PER_BANK], pub kona_gpio: *mut BcmKonaGpio }
#[repr(C)] pub struct BcmKonaGpio { pub reg_base: *mut u8, pub num_bank: i32, pub lock: RawSpinLock, pub gpio_chip: GpioChip, pub irq_domain: *mut IrqDomain, pub banks: [BcmKonaGpioBank; 0] }

extern "C" {
    fn readl(addr: *mut u8) -> u32; fn writel(value: u32, addr: *mut u8);
    fn gpiochip_get_data(chip: *mut GpioChip) -> *mut BcmKonaGpio;
    fn irq_data_get_irq_chip_data(d: *mut IrqData) -> *mut BcmKonaGpio;
    fn irq_create_mapping(d: *mut IrqDomain, gpio: u32) -> i32;
    fn pinconf_to_config_param(config: u64) -> u32; fn pinconf_to_config_argument(config: u64) -> u32;
    fn generic_handle_domain_irq(d: *mut IrqDomain, hwirq: i32);
    fn gpiochip_disable_irq(chip: *mut GpioChip, gpio: u32); fn gpiochip_enable_irq(chip: *mut GpioChip, gpio: u32);
    fn gpiochip_reqres_irq(chip: *mut GpioChip, gpio: u32) -> i32; fn gpiochip_relres_irq(chip: *mut GpioChip, gpio: u32);
    fn chained_irq_enter(chip: *mut IrqChip, desc: *mut IrqDesc); fn chained_irq_exit(chip: *mut IrqChip, desc: *mut IrqDesc);
    fn irq_desc_get_handler_data(desc: *mut IrqDesc) -> *mut BcmKonaGpioBank; fn irq_desc_get_chip(desc: *mut IrqDesc) -> *mut IrqChip;
}

unsafe fn bcm_kona_gpio_write_lock_regs(reg_base: *mut u8, bank_id: usize, lockcode: u32) {
    writel(BCM_GPIO_PASSWD, reg_base.add(GPIO_GPPWR_OFFSET));
    writel(lockcode, reg_base.add(gpio_pwd_status(bank_id)));
}

unsafe fn bcm_kona_gpio_lock_gpio(kona_gpio: *mut BcmKonaGpio, gpio: usize) {
    let bank_id = gpio_bank(gpio); let bit = gpio_bit(gpio); let bank = &mut *(*kona_gpio).banks.as_mut_ptr().add(bank_id);
    if bank.gpio_unlock_count[bit] == 0 { return; }
    bank.gpio_unlock_count[bit] -= 1;
    if bank.gpio_unlock_count[bit] == 0 { let val = readl((*kona_gpio).reg_base.add(gpio_pwd_status(bank_id))) | (1u32 << bit); bcm_kona_gpio_write_lock_regs((*kona_gpio).reg_base, bank_id, val); }
}
unsafe fn bcm_kona_gpio_unlock_gpio(kona_gpio: *mut BcmKonaGpio, gpio: usize) {
    let bank_id = gpio_bank(gpio); let bit = gpio_bit(gpio); let bank = &mut *(*kona_gpio).banks.as_mut_ptr().add(bank_id);
    if bank.gpio_unlock_count[bit] == 0 { let val = readl((*kona_gpio).reg_base.add(gpio_pwd_status(bank_id))) & !(1u32 << bit); bcm_kona_gpio_write_lock_regs((*kona_gpio).reg_base, bank_id, val); }
    bank.gpio_unlock_count[bit] += 1;
}

unsafe fn bcm_kona_gpio_get_dir(chip: *mut GpioChip, gpio: usize) -> i32 { let k = gpiochip_get_data(chip); if readl((*k).reg_base.add(gpio_control(gpio))) & GPIO_GPCTR0_IOTR_MASK != 0 { 1 } else { 0 } }
unsafe fn bcm_kona_gpio_set(chip: *mut GpioChip, gpio: usize, value: i32) -> i32 { let k=gpiochip_get_data(chip); if bcm_kona_gpio_get_dir(chip,gpio)==1{return 0;} let b=gpio_bank(gpio); let o=if value!=0{gpio_out_set(b)}else{gpio_out_clear(b)}; let v=readl((*k).reg_base.add(o))|(1<<gpio_bit(gpio)); writel(v,(*k).reg_base.add(o)); 0 }
unsafe fn bcm_kona_gpio_get(chip: *mut GpioChip, gpio: usize) -> i32 { let k=gpiochip_get_data(chip); let b=gpio_bank(gpio); let o=if bcm_kona_gpio_get_dir(chip,gpio)==1{gpio_in_status(b)}else{gpio_out_status(b)}; ((readl((*k).reg_base.add(o))>>(gpio_bit(gpio)))&1) as i32 }
unsafe fn bcm_kona_gpio_request(chip:*mut GpioChip,gpio:usize)->i32{bcm_kona_gpio_unlock_gpio(gpiochip_get_data(chip),gpio);0}
unsafe fn bcm_kona_gpio_free(chip:*mut GpioChip,gpio:usize){bcm_kona_gpio_lock_gpio(gpiochip_get_data(chip),gpio)}
unsafe fn bcm_kona_gpio_direction_input(chip:*mut GpioChip,gpio:usize)->i32{let k=gpiochip_get_data(chip);let p=(*k).reg_base.add(gpio_control(gpio));let mut v=readl(p);v=(v&!GPIO_GPCTR0_IOTR_MASK)|GPIO_GPCTR0_IOTR_CMD_INPUT;writel(v,p);0}
unsafe fn bcm_kona_gpio_direction_output(chip:*mut GpioChip,gpio:usize,value:i32)->i32{let k=gpiochip_get_data(chip);let p=(*k).reg_base.add(gpio_control(gpio));let mut v=readl(p)&!GPIO_GPCTR0_IOTR_MASK;writel(v,p);bcm_kona_gpio_set(chip,gpio,value)}

unsafe fn bcm_kona_gpio_irq_ack(d:*mut IrqData) { let k=irq_data_get_irq_chip_data(d); let g=(*d).hwirq as usize; let b=gpio_bank(g); let p=(*k).reg_base.add(gpio_int_status(b)); writel(readl(p)|(1<<gpio_bit(g)),p); }
unsafe fn bcm_kona_gpio_irq_mask(d:*mut IrqData) { let k=irq_data_get_irq_chip_data(d); let g=(*d).hwirq as usize; let b=gpio_bank(g); let p=(*k).reg_base.add(gpio_int_mask(b)); writel(readl(p)|(1<<gpio_bit(g)),p); gpiochip_disable_irq(&mut (*k).gpio_chip,g as u32); }
unsafe fn bcm_kona_gpio_irq_unmask(d:*mut IrqData) { let k=irq_data_get_irq_chip_data(d); let g=(*d).hwirq as usize; let b=gpio_bank(g); let p=(*k).reg_base.add(gpio_int_mskclr(b)); writel(readl(p)|(1<<gpio_bit(g)),p); gpiochip_enable_irq(&mut (*k).gpio_chip,g as u32); }
unsafe fn bcm_kona_gpio_irq_set_type(d:*mut IrqData,ty:u32)->i32 { let k=irq_data_get_irq_chip_data(d); let g=(*d).hwirq as usize; let p=(*k).reg_base.add(gpio_control(g)); let t=match ty&0xf {1=>GPIO_GPCTR0_ITR_CMD_RISING_EDGE,2=>GPIO_GPCTR0_ITR_CMD_FALLING_EDGE,3=>GPIO_GPCTR0_ITR_CMD_BOTH_EDGE,_=>return -22}; let v=(readl(p)&!GPIO_GPCTR0_ITR_MASK)|(t<<GPIO_GPCTR0_ITR_SHIFT); writel(v,p); 0 }
unsafe fn bcm_kona_gpio_irq_handler(desc:*mut IrqDesc) { let bank=irq_desc_get_handler_data(desc); let k=(*bank).kona_gpio; let b=(*bank).id as usize; let p=(*k).reg_base.add(gpio_int_status(b)); let m=(*k).reg_base.add(gpio_int_mask(b)); let mut sta=readl(p)&!readl(m); while sta!=0 { let bit=sta.trailing_zeros() as usize; writel(readl(p)|(1<<bit),p); generic_handle_domain_irq((*k).irq_domain,(GPIO_PER_BANK*b+bit) as i32); sta&=sta-1; } }
unsafe fn bcm_kona_gpio_reset(k:*mut BcmKonaGpio) { for i in 0..(*k).num_bank as usize { bcm_kona_gpio_write_lock_regs((*k).reg_base,i,UNLOCK_CODE); writel(0xffff_ffff,(*k).reg_base.add(gpio_int_mask(i))); writel(0xffff_ffff,(*k).reg_base.add(gpio_int_status(i))); bcm_kona_gpio_write_lock_regs((*k).reg_base,i,LOCK_CODE); } }
unsafe fn bcm_kona_gpio_probe(_pdev:*mut PlatformDevice)->i32 { -38 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
