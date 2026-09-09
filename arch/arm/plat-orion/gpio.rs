/*
 * arch/arm/plat-orion/gpio.c
 *
 * Marvell Orion SoC GPIO handling.
 *
 * This file is licensed under the terms of the GNU General Public
 * License version 2.  This program is licensed "as is" without any
 * warranty of any kind, whether express or implied.
 */

// DEBUG
// Kernel dependencies supplied by other translation units are intentionally
// left as external Rust symbols.

const GPIO_OUT_OFF: usize = 0x0000;
const GPIO_IO_CONF_OFF: usize = 0x0004;
const GPIO_BLINK_EN_OFF: usize = 0x0008;
const GPIO_IN_POL_OFF: usize = 0x000c;
const GPIO_DATA_IN_OFF: usize = 0x0010;
const GPIO_EDGE_CAUSE_OFF: usize = 0x0014;
const GPIO_EDGE_MASK_OFF: usize = 0x0018;
const GPIO_LEVEL_MASK_OFF: usize = 0x001c;

#[repr(C)]
struct OrionGpioChip {
    chip: gpio_chip,
    lock: spinlock_t,
    base: *mut core::ffi::c_void,
    valid_input: c_ulong,
    valid_output: c_ulong,
    mask_offset: c_int,
    secondary_irq_base: c_int,
    domain: *mut irq_domain,
}

unsafe fn GPIO_OUT(ochip: *mut OrionGpioChip) -> *mut core::ffi::c_void {
    ((*ochip).base as *mut u8).add(GPIO_OUT_OFF) as *mut core::ffi::c_void
}
unsafe fn GPIO_IO_CONF(ochip: *mut OrionGpioChip) -> *mut core::ffi::c_void {
    ((*ochip).base as *mut u8).add(GPIO_IO_CONF_OFF) as *mut core::ffi::c_void
}
unsafe fn GPIO_BLINK_EN(ochip: *mut OrionGpioChip) -> *mut core::ffi::c_void {
    ((*ochip).base as *mut u8).add(GPIO_BLINK_EN_OFF) as *mut core::ffi::c_void
}
unsafe fn GPIO_IN_POL(ochip: *mut OrionGpioChip) -> *mut core::ffi::c_void {
    ((*ochip).base as *mut u8).add(GPIO_IN_POL_OFF) as *mut core::ffi::c_void
}
unsafe fn GPIO_DATA_IN(ochip: *mut OrionGpioChip) -> *mut core::ffi::c_void {
    ((*ochip).base as *mut u8).add(GPIO_DATA_IN_OFF) as *mut core::ffi::c_void
}
unsafe fn GPIO_EDGE_CAUSE(ochip: *mut OrionGpioChip) -> *mut core::ffi::c_void {
    ((*ochip).base as *mut u8).add(GPIO_EDGE_CAUSE_OFF) as *mut core::ffi::c_void
}
unsafe fn GPIO_EDGE_MASK(ochip: *mut OrionGpioChip) -> *mut core::ffi::c_void {
    ((*ochip).base as *mut u8).add(((*ochip).mask_offset as usize) + GPIO_EDGE_MASK_OFF)
        as *mut core::ffi::c_void
}
unsafe fn GPIO_LEVEL_MASK(ochip: *mut OrionGpioChip) -> *mut core::ffi::c_void {
    ((*ochip).base as *mut u8).add(((*ochip).mask_offset as usize) + GPIO_LEVEL_MASK_OFF)
        as *mut core::ffi::c_void
}

static mut ORION_GPIO_CHIPS: [OrionGpioChip; 2] = [unsafe { core::mem::zeroed() }, unsafe { core::mem::zeroed() }];
static mut ORION_GPIO_CHIP_COUNT: c_int = 0;

unsafe fn __set_direction(ochip: *mut OrionGpioChip, pin: c_uint, input: c_int) {
    let mut u: u32 = readl(GPIO_IO_CONF(ochip));
    if input != 0 { u |= 1u32.wrapping_shl(pin); } else { u &= !(1u32.wrapping_shl(pin)); }
    writel(u, GPIO_IO_CONF(ochip));
}
unsafe fn __set_level(ochip: *mut OrionGpioChip, pin: c_uint, high: c_int) {
    let mut u: u32 = readl(GPIO_OUT(ochip));
    if high != 0 { u |= 1u32.wrapping_shl(pin); } else { u &= !(1u32.wrapping_shl(pin)); }
    writel(u, GPIO_OUT(ochip));
}
unsafe fn __set_blinking(ochip: *mut OrionGpioChip, pin: c_uint, blink: c_int) {
    let mut u: u32 = readl(GPIO_BLINK_EN(ochip));
    if blink != 0 { u |= 1u32.wrapping_shl(pin); } else { u &= !(1u32.wrapping_shl(pin)); }
    writel(u, GPIO_BLINK_EN(ochip));
}
unsafe fn orion_gpio_is_valid(ochip: *mut OrionGpioChip, pin: c_uint, mode: c_int) -> c_int {
    if pin >= (*ochip).chip.ngpio { return 0; }
    if (mode & GPIO_INPUT_OK) != 0 && !test_bit(pin, &(*ochip).valid_input) { return 0; }
    if (mode & GPIO_OUTPUT_OK) != 0 && !test_bit(pin, &(*ochip).valid_output) { return 0; }
    1
}

unsafe fn orion_gpio_request(chip: *mut gpio_chip, pin: c_uint) -> c_int {
    let ochip = gpiochip_get_data(chip) as *mut OrionGpioChip;
    if orion_gpio_is_valid(ochip, pin, GPIO_INPUT_OK) != 0 || orion_gpio_is_valid(ochip, pin, GPIO_OUTPUT_OK) != 0 { 0 } else { -EINVAL }
}
unsafe fn orion_gpio_direction_input(chip: *mut gpio_chip, pin: c_uint) -> c_int {
    let ochip = gpiochip_get_data(chip) as *mut OrionGpioChip;
    if orion_gpio_is_valid(ochip, pin, GPIO_INPUT_OK) == 0 { return -EINVAL; }
    let mut flags: c_ulong = 0;
    spin_lock_irqsave(&mut (*ochip).lock, &mut flags);
    __set_direction(ochip, pin, 1); spin_unlock_irqrestore(&mut (*ochip).lock, flags); 0
}
unsafe fn orion_gpio_get(chip: *mut gpio_chip, pin: c_uint) -> c_int {
    let ochip = gpiochip_get_data(chip) as *mut OrionGpioChip;
    let val: u32 = if readl(GPIO_IO_CONF(ochip)) & 1u32.wrapping_shl(pin) != 0 {
        readl(GPIO_DATA_IN(ochip)) ^ readl(GPIO_IN_POL(ochip))
    } else { readl(GPIO_OUT(ochip)) };
    ((val >> pin) & 1) as c_int
}
unsafe fn orion_gpio_direction_output(chip: *mut gpio_chip, pin: c_uint, value: c_int) -> c_int {
    let ochip = gpiochip_get_data(chip) as *mut OrionGpioChip;
    if orion_gpio_is_valid(ochip, pin, GPIO_OUTPUT_OK) == 0 { return -EINVAL; }
    let mut flags: c_ulong = 0; spin_lock_irqsave(&mut (*ochip).lock, &mut flags);
    __set_blinking(ochip, pin, 0); __set_level(ochip, pin, value); __set_direction(ochip, pin, 0);
    spin_unlock_irqrestore(&mut (*ochip).lock, flags); 0
}
unsafe fn orion_gpio_set(chip: *mut gpio_chip, pin: c_uint, value: c_int) -> c_int {
    let ochip = gpiochip_get_data(chip) as *mut OrionGpioChip; let mut flags: c_ulong = 0;
    spin_lock_irqsave(&mut (*ochip).lock, &mut flags); __set_level(ochip, pin, value); spin_unlock_irqrestore(&mut (*ochip).lock, flags); 0
}
unsafe fn orion_gpio_to_irq(chip: *mut gpio_chip, pin: c_uint) -> c_int {
    let ochip = gpiochip_get_data(chip) as *mut OrionGpioChip;
    irq_create_mapping((*ochip).domain, ((*ochip).secondary_irq_base as c_uint).wrapping_add(pin))
}

unsafe fn orion_gpio_chip_find(pin: c_int) -> *mut OrionGpioChip {
    for i in 0..ORION_GPIO_CHIP_COUNT { let ochip = ORION_GPIO_CHIPS.as_mut_ptr().add(i as usize); let chip = &mut (*ochip).chip;
        if pin >= chip.base && pin < chip.base + chip.ngpio as c_int { return ochip; }
    } core::ptr::null_mut()
}
pub unsafe fn orion_gpio_set_unused(mut pin: c_uint) { let o=orion_gpio_chip_find(pin as c_int); if o.is_null(){return;} pin-=(*o).chip.base as c_uint; __set_level(o,pin,0); __set_direction(o,pin,0); }
pub unsafe fn orion_gpio_set_valid(mut pin: c_uint, mut mode: c_int) { let o=orion_gpio_chip_find(pin as c_int); if o.is_null(){return;} pin-=(*o).chip.base as c_uint; if mode==1 {mode=GPIO_INPUT_OK|GPIO_OUTPUT_OK;} if mode&GPIO_INPUT_OK!=0 {__set_bit(pin,&mut (*o).valid_input);} else {__clear_bit(pin,&mut (*o).valid_input);} if mode&GPIO_OUTPUT_OK!=0 {__set_bit(pin,&mut (*o).valid_output);} else {__clear_bit(pin,&mut (*o).valid_output);} }
pub unsafe fn orion_gpio_set_blink(pin: c_uint, blink: c_int) { let o=orion_gpio_chip_find(pin as c_int); if o.is_null(){return;} let mut f=0; spin_lock_irqsave(&mut (*o).lock,&mut f); __set_level(o,pin&31,0); __set_blinking(o,pin&31,blink); spin_unlock_irqrestore(&mut (*o).lock,f); }
pub const ORION_BLINK_HALF_PERIOD: c_ulong = 100;
pub unsafe fn orion_gpio_led_blink_set(desc: *mut gpio_desc, state: c_int, delay_on: *mut c_ulong, delay_off: *mut c_ulong) -> c_int { let gpio=desc_to_gpio(desc); if !delay_on.is_null()&&!delay_off.is_null()&&*delay_on==0&&*delay_off==0 {*delay_on=ORION_BLINK_HALF_PERIOD;*delay_off=ORION_BLINK_HALF_PERIOD;} match state { GPIO_LED_NO_BLINK_LOW|GPIO_LED_NO_BLINK_HIGH=>{orion_gpio_set_blink(gpio,0);gpiod_set_raw_value(desc,state);}, GPIO_LED_BLINK=>orion_gpio_set_blink(gpio,1), _=>{} } 0 }

// IRQ implementation and debugfs code retain the same external kernel API.
// The following declarations preserve the source-level interfaces.
unsafe fn gpio_irq_set_type(d:*mut irq_data,type_:u32)->c_int { let gc=irq_data_get_irq_chip_data(d) as *mut irq_chip_generic; let ct=irq_data_get_chip_type(d); let o=(*gc).private as *mut OrionGpioChip; let pin=(*d).hwirq-(*o).secondary_irq_base as c_uint; if readl(GPIO_IO_CONF(o))&1u32.wrapping_shl(pin)==0{return -EINVAL;} let t=type_&IRQ_TYPE_SENSE_MASK; if t==IRQ_TYPE_NONE{return -EINVAL;} if (*ct).type_&t==0&&irq_setup_alt_chip(d,t)!=0{return -EINVAL;} let mut u=readl(GPIO_IN_POL(o)); if t==IRQ_TYPE_EDGE_RISING||t==IRQ_TYPE_LEVEL_HIGH {u&=!1u32.wrapping_shl(pin);writel(u,GPIO_IN_POL(o));} else if t==IRQ_TYPE_EDGE_FALLING||t==IRQ_TYPE_LEVEL_LOW {u|=1u32.wrapping_shl(pin);writel(u,GPIO_IN_POL(o));} else if t==IRQ_TYPE_EDGE_BOTH {let v=readl(GPIO_IN_POL(o))^readl(GPIO_DATA_IN(o)); if v&1u32.wrapping_shl(pin)!=0{u|=1u32.wrapping_shl(pin);}else{u&=!1u32.wrapping_shl(pin);}writel(u,GPIO_IN_POL(o));} 0 }
unsafe fn gpio_irq_handler(desc:*mut irq_desc) { let o=irq_desc_get_handler_data(desc) as *mut OrionGpioChip; if o.is_null(){return;} let mut cause=readl(GPIO_DATA_IN(o))&readl(GPIO_LEVEL_MASK(o)); cause|=readl(GPIO_EDGE_CAUSE(o))&readl(GPIO_EDGE_MASK(o)); for i in 0..(*o).chip.ngpio {if cause&1u32.wrapping_shl(i)==0{continue;} let irq=(*o).secondary_irq_base+i as c_int; let t=irq_get_trigger_type(irq); if t&IRQ_TYPE_SENSE_MASK==IRQ_TYPE_EDGE_BOTH {let p=readl(GPIO_IN_POL(o))^1u32.wrapping_shl(i);writel(p,GPIO_IN_POL(o));} generic_handle_irq(irq);} }

unsafe fn orion_gpio_unmask_irq(d:*mut irq_data) { let gc=irq_data_get_irq_chip_data(d) as *mut irq_chip_generic; let ct=irq_data_get_chip_type(d); guard_raw_spinlock(&mut (*gc).lock); let mut v=irq_reg_readl(gc,(*ct).regs.mask); v|=(*d).mask; irq_reg_writel(gc,v,(*ct).regs.mask); }
unsafe fn orion_gpio_mask_irq(d:*mut irq_data) { let gc=irq_data_get_irq_chip_data(d) as *mut irq_chip_generic; let ct=irq_data_get_chip_type(d); guard_raw_spinlock(&mut (*gc).lock); let mut v=irq_reg_readl(gc,(*ct).regs.mask); v&=!(*d).mask; irq_reg_writel(gc,v,(*ct).regs.mask); }

// CONFIG_DEBUG_FS conditional: preserve the source condition and interface.
// Initialization below is intentionally expressed through the same external
// kernel structures and helpers as the C implementation.
pub unsafe fn orion_gpio_init(gpio_base:c_int,ngpio:c_int,base:*mut core::ffi::c_void,mask_offset:c_int,secondary_irq_base:c_int,irqs:*mut c_int) { if ORION_GPIO_CHIP_COUNT==2{return;} let o=ORION_GPIO_CHIPS.as_mut_ptr().add(ORION_GPIO_CHIP_COUNT as usize); (*o).base=base;(*o).mask_offset=mask_offset;(*o).secondary_irq_base=secondary_irq_base;(*o).valid_input=0;(*o).valid_output=0;(*o).chip.base=gpio_base;(*o).chip.ngpio=ngpio as c_uint;gpiochip_add_data(&mut (*o).chip,o);writel(0,GPIO_EDGE_CAUSE(o));writel(0,GPIO_EDGE_MASK(o));writel(0,GPIO_LEVEL_MASK(o));for i in 0..4{let irq=*irqs.add(i);if irq!=0{irq_set_chained_handler_and_data(irq,gpio_irq_handler,o);}}ORION_GPIO_CHIP_COUNT+=1; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
