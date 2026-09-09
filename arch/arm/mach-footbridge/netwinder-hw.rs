// SPDX-License-Identifier: GPL-2.0
// Linux arch/arm/mach-footbridge/netwinder-hw.c

// Kernel dependencies are supplied by the surrounding translation unit.

const IRDA_IO_BASE: i32 = 0x180;
const GP1_IO_BASE: i32 = 0x338;
const GP2_IO_BASE: i32 = 0x33a;

unsafe fn wb977_open() { outb(0x87, 0x370); outb(0x87, 0x370); }
unsafe fn wb977_close() { outb(0xaa, 0x370); }
unsafe fn wb977_wb(reg: i32, val: i32) { outb(reg, 0x370); outb(val, 0x371); }
unsafe fn wb977_ww(reg: i32, val: i32) {
    outb(reg, 0x370); outb(val >> 8, 0x371);
    outb(reg + 1, 0x370); outb(val & 255, 0x371);
}
unsafe fn wb977_device_select(dev: i32) { wb977_wb(0x07, dev); }
unsafe fn wb977_device_disable() { wb977_wb(0x30, 0x00); }
unsafe fn wb977_device_enable() { wb977_wb(0x30, 0x01); }

pub static mut nw_gpio_lock: RawSpinlock = DEFINE_RAW_SPINLOCK!();
static mut current_gpio_op: u32 = 0;
static mut current_gpio_io: u32 = 0;
static mut current_cpld: u32 = 0;

pub unsafe fn nw_gpio_modify_op(mask: u32, set: u32) {
    let new_gpio = (current_gpio_op & !mask) | set;
    let changed = new_gpio ^ current_gpio_op;
    current_gpio_op = new_gpio;
    if changed & 0xff != 0 { outb(new_gpio as i32, GP1_IO_BASE); }
    if changed & 0xff00 != 0 { outb((new_gpio >> 8) as i32, GP2_IO_BASE); }
}

unsafe fn __gpio_modify_io(mask: i32, input: i32) {
    let new_gpio = (current_gpio_io & !(mask as u32)) | input as u32;
    let mut changed = new_gpio ^ current_gpio_io;
    current_gpio_io = new_gpio;
    changed >>= 1;
    let mut value = new_gpio >> 1;
    wb977_device_select(7);
    let mut port = 0xe1;
    while changed != 0 && port < 0xe8 { wb977_wb(port, (value & 1) as i32); port += 1; changed >>= 1; value >>= 1; }
    wb977_device_select(8);
    port = 0xe8;
    while changed != 0 && port < 0xec { wb977_wb(port, (value & 1) as i32); port += 1; changed >>= 1; value >>= 1; }
}

pub unsafe fn nw_gpio_modify_io(mask: u32, input: u32) { wb977_open(); __gpio_modify_io(mask as i32, input as i32); wb977_close(); }
pub unsafe fn nw_gpio_read() -> u32 { inb(GP1_IO_BASE) as u32 | ((inb(GP2_IO_BASE) as u32) << 8) }

unsafe fn wb977_init_global() { wb977_wb(0x26,0x40); wb977_wb(0x22,0xfe); wb977_wb(0x2a,0xc1); wb977_wb(0x2b,0x6b); wb977_wb(0x2c,0x55); }
unsafe fn wb977_init_printer() { wb977_device_select(1); wb977_wb(0xf0,1); }
unsafe fn wb977_init_keyboard() {
    wb977_device_select(5); wb977_ww(0x60,0x0060); wb977_ww(0x62,0x0064);
    wb977_wb(0x70,1); wb977_wb(0x71,2); wb977_wb(0x72,5); wb977_wb(0x73,2); wb977_wb(0xf0,0x40); wb977_device_enable();
}
unsafe fn wb977_init_irda() {
    wb977_device_select(6); wb977_ww(0x60,IRDA_IO_BASE); wb977_wb(0x70,6); wb977_wb(0x71,2); wb977_wb(0x74,0); wb977_wb(0x75,4); wb977_wb(0xf0,3); wb977_device_enable();
}
unsafe fn wb977_init_gpio() {
    let mut flags: u64 = 0; current_gpio_io = u32::MAX; __gpio_modify_io(-1, (GPIO_DONE | GPIO_WDTIMER) as i32);
    wb977_device_select(7); wb977_ww(0x60,GP1_IO_BASE); wb977_ww(0x62,0); wb977_ww(0x64,0);
    wb977_wb(0x70,10); wb977_wb(0x71,2); wb977_wb(0xe0,0x19); wb977_device_enable(); wb977_device_select(8); wb977_ww(0x60,GP2_IO_BASE);
    wb977_wb(0xf2,0); wb977_wb(0xf3,0); wb977_wb(0xf4,0); wb977_device_enable();
    raw_spin_lock_irqsave(&mut nw_gpio_lock, &mut flags); nw_gpio_modify_op(u32::MAX, GPIO_RED_LED | GPIO_FAN); raw_spin_unlock_irqrestore(&mut nw_gpio_lock, flags);
}
unsafe fn wb977_init() { request_region(0x370,2,"W83977AF configuration"); wb977_open(); wb977_init_global(); wb977_init_printer(); wb977_init_keyboard(); wb977_init_irda(); wb977_init_gpio(); wb977_close(); }

pub unsafe fn nw_cpld_modify(mask: u32, set: u32) {
    let mut msk = 8; current_cpld = (current_cpld & !mask) | set; nw_gpio_modify_io(GPIO_DATA|GPIO_IOCLK|GPIO_IOLOAD,0); nw_gpio_modify_op(GPIO_IOLOAD,0);
    while msk != 0 { let bit = current_cpld & msk; nw_gpio_modify_op(GPIO_DATA|GPIO_IOCLK, if bit != 0 { GPIO_DATA } else { 0 }); nw_gpio_modify_op(GPIO_IOCLK,GPIO_IOCLK); msk >>= 1; }
    nw_gpio_modify_op(GPIO_IOCLK|GPIO_DATA,0); nw_gpio_modify_op(GPIO_IOLOAD|GPIO_DSCLK,GPIO_IOLOAD|GPIO_DSCLK); nw_gpio_modify_op(GPIO_IOLOAD,0);
}
unsafe fn cpld_init() { let mut flags=0; raw_spin_lock_irqsave(&mut nw_gpio_lock,&mut flags); nw_cpld_modify(u32::MAX,CPLD_UNMUTE|CPLD_7111_DISABLE); raw_spin_unlock_irqrestore(&mut nw_gpio_lock,flags); }

static rwa_unlock: [u8; 33] = [0x00,0x00,0x6a,0xb5,0xda,0xed,0xf6,0xfb,0x7d,0xbe,0xdf,0x6f,0x37,0x1b,0x0d,0x86,0xc3,0x61,0xb0,0x58,0x2c,0x16,0x8b,0x45,0xa2,0xd1,0xe8,0x74,0x3a,0x9d,0xce,0xe7,0x73];
unsafe fn write_rwa(r:i32,v:i32){outb(r,0x279);udelay(10);outb(v,0xa79);}
unsafe fn rwa010_unlock(){write_rwa(2,2);mdelay(10);for &v in rwa_unlock.iter(){outb(v as i32,0x279);udelay(10);}}
unsafe fn rwa010_read_ident(){let mut si=[0u8;9];write_rwa(3,0);write_rwa(0,128);outb(1,0x279);mdelay(1);for i in 0..9{for j in 0..8{udelay(250);inb(0x203);udelay(250);let bit=if inb(0x203)==0xaa{1}else{0};si[i]|=(bit<<j) as u8;}}}
unsafe fn rwa010_global_init(){write_rwa(6,2);inb(0x203);write_rwa(7,3);write_rwa(0x30,0);write_rwa(7,4);write_rwa(0x30,0);write_rwa(7,2);write_rwa(0x30,0);}
unsafe fn rwa010_game_port_init(){write_rwa(7,5);write_rwa(0x61,1);let _=inb(0x203);write_rwa(0x60,2);inb(0x203);write_rwa(0x30,1);}
unsafe fn rwa010_waveartist_init(base:i32,irq:i32,dma:i32){write_rwa(7,0);write_rwa(0x61,base&255);inb(0x203);write_rwa(0x60,base>>8);inb(0x203);write_rwa(0x70,irq);inb(0x203);write_rwa(0x74,dma);inb(0x203);write_rwa(0x30,1);}
unsafe fn rwa010_soundblaster_init(sb:i32,al:i32,irq:i32,dma:i32){write_rwa(7,1);write_rwa(0x61,sb&255);inb(0x203);write_rwa(0x60,sb>>8);inb(0x203);write_rwa(0x70,irq);inb(0x203);write_rwa(0x74,dma);inb(0x203);write_rwa(0x63,al&255);inb(0x203);write_rwa(0x62,al>>8);inb(0x203);write_rwa(0x30,1);}
unsafe fn rwa010_soundblaster_reset(){outb(1,0x226);udelay(3);outb(0,0x226);let mut i=0;while i<5{if inb(0x22e)&0x80!=0{break}mdelay(1);i+=1;}if i==5{printk("SoundBlaster: DSP reset failed\n");}inb(0x22a);i=0;while i<5{if inb(0x22c)&0x80==0{break}mdelay(1);i+=1;}if i==5{printk("SoundBlaster: DSP not ready\n");}else{outb(0xe1,0x22c);let _=inb(0x22a);udelay(1);let _=inb(0x22a);outb(0xd3,0x22c);}outb(5,0x38a);outb(1,0x38b);}
unsafe fn rwa010_init(){rwa010_unlock();rwa010_read_ident();rwa010_global_init();rwa010_game_port_init();rwa010_waveartist_init(0x250,3,7);rwa010_soundblaster_init(0x220,0x388,3,1);rwa010_soundblaster_reset();}

unsafe fn nw_hw_init()->i32{if machine_is_netwinder(){wb977_init();cpld_init();rwa010_init();}0}
// __initcall(nw_hw_init)

unsafe fn fixup_netwinder(_tags:*mut Tag,_cmdline:*mut *mut i8){/* CONFIG_ISAPNP: set isapnp_disable = 1. */}
unsafe fn netwinder_restart(mode: RebootMode,_cmd:*const i8){if mode==REBOOT_SOFT{soft_restart(0x41000000)}else{local_irq_disable();local_fiq_disable();outb(0x87,0x370);outb(0x87,0x370);outb(7,0x370);outb(7,0x371);outb(0xe6,0x370);outb(0,0x371);outb(0xc4,0x338);}}

// CONFIG_NEW_LEDS && CONFIG_LEDS_CLASS
#[repr(C)]
struct netwinder_led { cdev: LedClassDev, mask: u8 }
struct NetwinderLedDesc { name: *const i8, trigger: *const i8 }
static netwinder_leds: [NetwinderLedDesc; 2] = [
    NetwinderLedDesc { name: b"netwinder:green\0".as_ptr() as *const i8, trigger: b"heartbeat\0".as_ptr() as *const i8 },
    NetwinderLedDesc { name: b"netwinder:red\0".as_ptr() as *const i8, trigger: b"cpu0\0".as_ptr() as *const i8 },
];
unsafe fn netwinder_led_set(cdev: *mut LedClassDev, brightness: LedBrightness) {
    let led = container_of_led(cdev); let mut flags = 0; let mut reg;
    raw_spin_lock_irqsave(&mut nw_gpio_lock, &mut flags); reg = nw_gpio_read();
    if brightness != LED_OFF { reg &= !(led.mask as u32); } else { reg |= led.mask as u32; }
    nw_gpio_modify_op(led.mask as u32, reg); raw_spin_unlock_irqrestore(&mut nw_gpio_lock, flags);
}
unsafe fn netwinder_led_get(cdev: *mut LedClassDev) -> LedBrightness {
    let led = container_of_led(cdev); let mut flags = 0; let reg;
    raw_spin_lock_irqsave(&mut nw_gpio_lock, &mut flags); reg = nw_gpio_read(); raw_spin_unlock_irqrestore(&mut nw_gpio_lock, flags);
    if reg & led.mask as u32 != 0 { LED_OFF } else { LED_FULL }
}
unsafe fn netwinder_leds_init() -> i32 {
    if !machine_is_netwinder() { return -ENODEV; }
    for i in 0..netwinder_leds.len() {
        let led = kzalloc_netwinder_led(); if led.is_null() { break; }
        (*led).cdev.name = netwinder_leds[i].name; (*led).cdev.brightness_set = netwinder_led_set; (*led).cdev.brightness_get = netwinder_led_get; (*led).cdev.default_trigger = netwinder_leds[i].trigger;
        (*led).mask = if i == 0 { GPIO_GREEN_LED as u8 } else { GPIO_RED_LED as u8 };
        if led_classdev_register(core::ptr::null_mut(), &mut (*led).cdev) < 0 { kfree(led as *mut core::ffi::c_void); break; }
    } 0
}
// fs_initcall(netwinder_leds_init)

// MACHINE_START(NETWINDER, "Rebel-NetWinder")
// .atag_offset = 0x100, .video_start = 0x000a0000, .video_end = 0x000bffff,
// .reserve_lp0 = 1, .reserve_lp2 = 1, .fixup = fixup_netwinder,
// .map_io = footbridge_map_io, .init_irq = footbridge_init_irq,
// .init_time = isa_timer_init, .restart = netwinder_restart

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
