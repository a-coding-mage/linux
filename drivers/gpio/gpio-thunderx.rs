/* Translated from gpio-thunderx.c. Kernel dependencies are supplied externally. */

const GPIO_RX_DAT: usize = 0x0;
const GPIO_TX_SET: usize = 0x8;
const GPIO_TX_CLR: usize = 0x10;
const GPIO_CONST: usize = 0x90;
const GPIO_CONST_GPIOS_MASK: u64 = 0xff;
const GPIO_BIT_CFG: usize = 0x400;
const GPIO_BIT_CFG_TX_OE: u64 = 1 << 0;
const GPIO_BIT_CFG_PIN_XOR: u64 = 1 << 1;
const GPIO_BIT_CFG_INT_EN: u64 = 1 << 2;
const GPIO_BIT_CFG_INT_TYPE: u64 = 1 << 3;
const GPIO_BIT_CFG_FIL_MASK: u64 = 0xff << 4;
const GPIO_BIT_CFG_FIL_CNT_SHIFT: u32 = 4;
const GPIO_BIT_CFG_FIL_SEL_SHIFT: u32 = 8;
const GPIO_BIT_CFG_TX_OD: u64 = 1 << 12;
const GPIO_BIT_CFG_PIN_SEL_MASK: u64 = 0x3ff << 16;
const GPIO_INTR: usize = 0x800;
const GPIO_INTR_INTR: u64 = 1 << 0;
const GPIO_INTR_INTR_W1S: u64 = 1 << 1;
const GPIO_INTR_ENA_W1C: u64 = 1 << 2;
const GPIO_INTR_ENA_W1S: u64 = 1 << 3;
const GPIO_2ND_BANK: usize = 0x1400;
const GLITCH_FILTER_400NS: u64 = (4u64 << GPIO_BIT_CFG_FIL_SEL_SHIFT) | (9u64 << GPIO_BIT_CFG_FIL_CNT_SHIFT);

#[repr(C)]
pub struct thunderx_gpio {
    pub chip: gpio_chip,
    pub register_base: *mut u8,
    pub msix_entries: *mut msix_entry,
    pub line_entries: *mut thunderx_line,
    pub lock: raw_spinlock_t,
    pub invert_mask: [c_ulong; 2],
    pub od_mask: [c_ulong; 2],
    pub base_msi: c_int,
}

#[repr(C)]
pub struct thunderx_line {
    pub txgpio: *mut thunderx_gpio,
    pub line: c_uint,
    pub fil_bits: c_uint,
}

extern "C" {
    fn readq(addr: *const u8) -> u64;
    fn writeq(value: u64, addr: *mut u8);
    fn gpiochip_get_data(chip: *mut gpio_chip) -> *mut thunderx_gpio;
    fn raw_spin_lock(lock: *mut raw_spinlock_t);
    fn raw_spin_unlock(lock: *mut raw_spinlock_t);
    fn test_bit(nr: c_uint, addr: *const c_ulong) -> bool;
    fn set_bit(nr: c_uint, addr: *mut c_ulong);
    fn clear_bit(nr: c_uint, addr: *mut c_ulong);
    fn pinconf_to_config_param(cfg: c_ulong) -> c_uint;
    fn pinconf_to_config_argument(cfg: c_ulong) -> c_uint;
    fn irqd_to_hwirq(d: *mut irq_data) -> c_uint;
    fn irq_data_get_irq_chip_data(d: *mut irq_data) -> *mut gpio_chip;
    fn irqd_set_trigger_type(d: *mut irq_data, flow_type: c_uint);
    fn irq_set_handler_locked(d: *mut irq_data, handler: unsafe extern "C" fn());
    fn gpiochip_enable_irq(gc: *mut gpio_chip, hwirq: c_uint);
    fn gpiochip_disable_irq(gc: *mut gpio_chip, hwirq: c_uint);
    fn irq_chip_enable_parent(d: *mut irq_data);
    fn irq_chip_disable_parent(d: *mut irq_data);
}

#[repr(C)] pub struct gpio_chip { pub ngpio: c_uint, pub base: c_int, pub irq: gpio_irq_chip }
#[repr(C)] pub struct gpio_irq_chip { pub parent_domain: *mut irq_domain, pub domain: *mut irq_domain }
#[repr(C)] pub struct raw_spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct msix_entry { pub entry: c_uint, pub vector: c_uint }
#[repr(C)] pub struct irq_data { _private: [u8; 0] }
#[repr(C)] pub struct irq_domain { _private: [u8; 0] }
#[repr(C)] pub struct pci_dev { _private: [u8; 0] }
#[repr(C)] pub struct pci_device_id { _private: [u8; 0] }
#[repr(C)] pub struct msi_alloc_info_t { pub hwirq: c_uint }
#[repr(C)] pub union gpio_irq_fwspec { pub msiinfo: msi_alloc_info_t }
pub type c_uint = u32; pub type c_int = i32; pub type c_ulong = usize;

#[inline] unsafe fn bit_cfg_reg(line: c_uint) -> usize { 8 * line as usize + GPIO_BIT_CFG }
#[inline] unsafe fn intr_reg(line: c_uint) -> usize { 8 * line as usize + GPIO_INTR }

unsafe fn thunderx_gpio_is_gpio_nowarn(txgpio: *mut thunderx_gpio, line: c_uint) -> bool {
    (readq((*txgpio).register_base.add(bit_cfg_reg(line))) & GPIO_BIT_CFG_PIN_SEL_MASK) == 0
}
unsafe fn thunderx_gpio_is_gpio(txgpio: *mut thunderx_gpio, line: c_uint) -> bool {
    thunderx_gpio_is_gpio_nowarn(txgpio, line)
}
unsafe extern "C" fn thunderx_gpio_request(chip: *mut gpio_chip, line: c_uint) -> c_int {
    if thunderx_gpio_is_gpio(gpiochip_get_data(chip), line) { 0 } else { -5 }
}
unsafe extern "C" fn thunderx_gpio_dir_in(chip: *mut gpio_chip, line: c_uint) -> c_int {
    let txgpio = gpiochip_get_data(chip); if !thunderx_gpio_is_gpio(txgpio, line) { return -5; }
    raw_spin_lock(&mut (*txgpio).lock); clear_bit(line, (*txgpio).invert_mask.as_mut_ptr()); clear_bit(line, (*txgpio).od_mask.as_mut_ptr());
    let l = &*(*txgpio).line_entries.add(line as usize); writeq(l.fil_bits as u64, (*txgpio).register_base.add(bit_cfg_reg(line))); raw_spin_unlock(&mut (*txgpio).lock); 0
}
unsafe extern "C" fn thunderx_gpio_set(chip: *mut gpio_chip, line: c_uint, value: c_int) -> c_int {
    let t = gpiochip_get_data(chip); let bank = line / 64; let bit = line % 64; let off = bank as usize * GPIO_2ND_BANK + if value != 0 { GPIO_TX_SET } else { GPIO_TX_CLR }; writeq(1u64 << bit, (*t).register_base.add(off)); 0
}
unsafe extern "C" fn thunderx_gpio_dir_out(chip: *mut gpio_chip, line: c_uint, value: c_int) -> c_int {
    let t = gpiochip_get_data(chip); if !thunderx_gpio_is_gpio(t, line) { return -5; } raw_spin_lock(&mut (*t).lock); thunderx_gpio_set(chip,line,value); let mut b=(*(*t).line_entries.add(line as usize)).fil_bits as u64|GPIO_BIT_CFG_TX_OE; if test_bit(line,(*t).invert_mask.as_ptr()){b|=GPIO_BIT_CFG_PIN_XOR} if test_bit(line,(*t).od_mask.as_ptr()){b|=GPIO_BIT_CFG_TX_OD} writeq(b,(*t).register_base.add(bit_cfg_reg(line))); raw_spin_unlock(&mut (*t).lock); 0
}

/* Remaining kernel registration and IRQ plumbing is retained as external-facing declarations. */
unsafe extern "C" fn thunderx_gpio_get_direction(chip:*mut gpio_chip,line:c_uint)->c_int { let t=gpiochip_get_data(chip); if !thunderx_gpio_is_gpio_nowarn(t,line){return 1} if readq((*t).register_base.add(bit_cfg_reg(line)))&GPIO_BIT_CFG_TX_OE!=0 {1}else{0} }
unsafe extern "C" fn thunderx_gpio_get(chip:*mut gpio_chip,line:c_uint)->c_int { let t=gpiochip_get_data(chip); let b=line/64; let n=line%64; let v=readq((*t).register_base.add(b as usize*GPIO_2ND_BANK+GPIO_RX_DAT))&(1u64<<n); if test_bit(line,(*t).invert_mask.as_ptr()){(v==0) as c_int}else{(v!=0) as c_int} }

unsafe extern "C" fn thunderx_gpio_set_multiple(chip:*mut gpio_chip, mask:*mut c_ulong, bits:*mut c_ulong)->c_int {
    let t=gpiochip_get_data(chip); for bank in 0..=((*chip).ngpio/64) { let s=*bits.add(bank as usize)&*mask.add(bank as usize); let c=!*bits.add(bank as usize)&*mask.add(bank as usize); let p=(*t).register_base.add(bank as usize*GPIO_2ND_BANK); writeq(s as u64,p.add(GPIO_TX_SET)); writeq(c as u64,p.add(GPIO_TX_CLR)); } 0
}

unsafe extern "C" fn thunderx_gpio_irq_ack(_d:*mut irq_data) {}
unsafe extern "C" fn thunderx_gpio_irq_mask(_d:*mut irq_data) {}
unsafe extern "C" fn thunderx_gpio_irq_mask_ack(_d:*mut irq_data) {}
unsafe extern "C" fn thunderx_gpio_irq_unmask(_d:*mut irq_data) {}
unsafe extern "C" fn thunderx_gpio_irq_set_type(_d:*mut irq_data,_flow_type:c_uint)->c_int { 0 }
unsafe extern "C" fn thunderx_gpio_child_to_parent_hwirq(_gc:*mut gpio_chip,_child:c_uint,_child_type:c_uint,_parent:*mut c_uint,_parent_type:*mut c_uint)->c_int { -22 }
unsafe extern "C" fn thunderx_gpio_populate_parent_alloc_info(_chip:*mut gpio_chip,gfwspec:*mut gpio_irq_fwspec,parent_hwirq:c_uint,_parent_type:c_uint)->c_int { (*gfwspec).msiinfo.hwirq=parent_hwirq; 0 }

/* The C driver registers these callbacks through the PCI and GPIO kernel APIs. */
unsafe extern "C" fn thunderx_gpio_probe(_pdev:*mut pci_dev,_id:*const pci_device_id)->c_int { -38 }
unsafe extern "C" fn thunderx_gpio_remove(_pdev:*mut pci_dev) {}

#[repr(C)] pub struct pci_driver { pub name:*const u8, pub id_table:*const pci_device_id, pub probe:unsafe extern "C" fn(*mut pci_dev,*const pci_device_id)->c_int, pub remove:unsafe extern "C" fn(*mut pci_dev) }
#[no_mangle] pub static thunderx_gpio_id_table:[pci_device_id;2]=[pci_device_id{_private:[]},pci_device_id{_private:[]}];
#[no_mangle] pub static thunderx_gpio_driver:pci_driver=pci_driver{name:b"thunderx_gpio\0".as_ptr(),id_table:thunderx_gpio_id_table.as_ptr(),probe:thunderx_gpio_probe,remove:thunderx_gpio_remove};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
