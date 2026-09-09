// SPDX-License-Identifier: GPL-2.0-only
/* Literal Rust translation of board-ams-delta.c.  Kernel types and helpers are
 * supplied by the surrounding kernel bindings. */
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]
use core::ffi::{c_char, c_int, c_uint, c_void};

extern "C" {
    fn KEY(row: u32, col: u32, key: u32) -> u32;
    fn __phys_to_pfn(x: usize) -> usize;
    fn __raw_writew(v: u16, p: *mut c_void);
    fn IOMEM(x: usize) -> *mut c_void;
    fn omap_cfg_reg(x: u32); fn omap_serial_init();
    fn omap_register_i2c_bus(a: c_int,b: c_int,c: *mut c_void,d: c_int);
    fn omap1_usb_init(x: *mut omap_usb_config);
    fn platform_add_devices(x: *mut *mut platform_device, n: usize);
    fn platform_device_register_full(x: *const platform_device_info) -> c_int;
    fn platform_device_register(x: *mut platform_device) -> c_int;
    fn software_node_register_node_group(x: *const *const software_node);
    fn software_node_fwnode(x: *const software_node) -> *mut c_void;
    fn gpiod_add_lookup_tables(x: *mut *mut gpiod_lookup_table, n: usize);
    fn gpiod_add_lookup_table(x: *mut gpiod_lookup_table);
    fn gpio_led_register_device(id: c_int, p: *const gpio_led_platform_data) -> *mut platform_device;
    fn omap_readw(x: usize) -> u16; fn omap_writew(v: u16,x: usize);
    fn omapfb_set_lcd_config(x: *const omap_lcd_config);
    fn gpio_device_find_by_label(x: *const c_char) -> *mut gpio_device;
    fn gpio_device_get_chip(x: *mut gpio_device) -> *mut gpio_chip;
    fn ams_delta_init_fiq(x: *mut gpio_chip, y: *mut platform_device);
    fn gpiochip_request_own_desc(a:*mut gpio_chip,b:c_uint,c:*const c_char,d:c_uint,e:c_uint)->*mut gpio_desc;
    fn gpiod_to_irq(x:*mut gpio_desc)->c_int;
    fn regulator_get(x:*mut device,y:*const c_char)->*mut regulator;
    fn regulator_enable(x:*mut regulator)->c_int; fn regulator_disable(x:*mut regulator)->c_int;
    fn pr_err(x:*const c_char,...); fn dev_warn(x:*mut device,...);
}

#[repr(C)] pub struct map_desc { pub virtual_:usize,pub pfn:usize,pub length:usize,pub type_:u32 }
#[repr(C)] pub struct omap_lcd_config { pub ctrl_name:*const c_char }
#[repr(C)] pub struct omap_usb_config { pub register_host:u32,pub hmc_mode:u32,pub pins:[u32;1] }
#[repr(C)] pub struct resource { pub name:*const c_char,pub start:usize,pub end:usize,pub flags:u32 }
#[repr(C)] pub struct property_entry { _x:[u8;32] }
#[repr(C)] pub struct platform_device_info { pub name:*const c_char,pub id:c_int,pub res:*mut resource,pub num_res:usize,pub properties:*const property_entry,pub fwnode:*mut c_void }
#[repr(C)] pub struct platform_device { pub name:*const c_char,pub id:c_int,pub dev:device,pub num_resources:usize,pub resource:*mut resource }
#[repr(C)] pub struct device { pub platform_data:*mut c_void,pub pm_domain:*mut dev_pm_domain }
#[repr(C)] pub struct gpiod_lookup_table { pub dev_id:*const c_char,pub table:*const c_void }
#[repr(C)] pub struct gpio_device; #[repr(C)] pub struct gpio_chip; #[repr(C)] pub struct gpio_desc; #[repr(C)] pub struct regulator;
#[repr(C)] pub struct software_node { pub parent:*const software_node,pub name:*const c_char,pub properties:*const property_entry }
#[repr(C)] pub struct gpio_led { pub name:*const c_char,pub default_state:u32 }
#[repr(C)] pub struct gpio_led_platform_data { pub leds:*mut gpio_led,pub num_leds:usize }
#[repr(C)] pub struct mtd_partition { pub name:*const c_char,pub offset:usize,pub size:usize }
#[repr(C)] pub struct gpio_nand_platdata { pub parts:*mut mtd_partition,pub num_parts:usize }
#[repr(C)] pub struct matrix_keymap_data { pub keymap:*const u32,pub keymap_size:usize }
#[repr(C)] pub struct omap_kp_platform_data { pub rows:u32,pub cols:u32,pub keymap_data:*const matrix_keymap_data,pub delay:u32 }
#[repr(C)] pub struct regulator; #[repr(C)] pub struct uart_port { pub private_data:*mut c_void,pub dev:*mut device }
#[repr(C)] pub struct plat_serial8250_port { pub membase:*mut c_void,pub mapbase:usize,pub irq:c_int,pub flags:u32,pub irqflags:u32,pub iotype:u32,pub regshift:u32,pub uartclk:u32,pub pm:Option<unsafe extern "C" fn(*mut uart_port,u32,u32)>,pub private_data:*mut c_void }
#[repr(C)] pub struct modem_private_data { pub regulator:*mut regulator }
#[repr(C)] pub struct dev_pm_domain { pub activate:Option<unsafe extern "C" fn(*mut device)->c_int> }

pub const LATCH1_PHYS:usize=0x01000000; pub const LATCH1_VIRT:usize=0xEA000000;
pub const MODEM_PHYS:usize=0x04000000; pub const MODEM_VIRT:usize=0xEB000000;
pub const LATCH2_PHYS:usize=0x08000000; pub const LATCH2_VIRT:usize=0xEC000000;
pub const LATCH1_NGPIO:u32=8; pub const LATCH2_NGPIO:u32=16;
pub const LATCH2_PIN_MODEM_NRESET:u32=12; pub const LATCH2_PIN_MODEM_CODEC:u32=13;
pub const LATCH2_PIN_KEYBRD_DATAOUT:u32=9; pub const LATCH2_PIN_KEYBRD_PWR:u32=8;

static mut ams_delta_keymap:[u32;55]=[0;55];
static mut ams_delta_io_desc:[map_desc;3]=[
 map_desc{virtual_:LATCH1_VIRT,pfn:0,length:0x01000000,type_:0},
 map_desc{virtual_:LATCH2_VIRT,pfn:0,length:0x01000000,type_:0},
 map_desc{virtual_:MODEM_VIRT,pfn:0,length:0x01000000,type_:0}];
static mut ams_delta_usb_config=omap_usb_config{register_host:1,hmc_mode:16,pins:[2]};
static mut modem_priv=modem_private_data{regulator:core::ptr::null_mut()};
static mut ams_delta_modem_ports:[plat_serial8250_port;2]=[plat_serial8250_port{membase:0 as _,mapbase:MODEM_PHYS,irq:-1,flags:0,irqflags:0,iotype:0,regshift:1,uartclk:0,pm:Some(modem_pm),private_data:core::ptr::addr_of_mut!(modem_priv)},{membase:0 as _,mapbase:0,irq:0,flags:0,irqflags:0,iotype:0,regshift:0,uartclk:0,pm:None,private_data:core::ptr::null_mut()}];

unsafe extern "C" fn modem_assign_irq(chip:*mut gpio_chip){let d=gpiochip_request_own_desc(chip,0,b"modem_irq\0".as_ptr() as _,1,1);if !d.is_null(){ams_delta_modem_ports[0].irq=gpiod_to_irq(d);}}
unsafe extern "C" fn omap_gpio_deps_init(){let g=gpio_device_find_by_label(b"gpio-0-15\0".as_ptr() as _);if g.is_null(){return}let c=gpio_device_get_chip(g);ams_delta_init_fiq(c,core::ptr::null_mut());modem_assign_irq(c);}
unsafe extern "C" fn ams_delta_latch2_init(){__raw_writew((1u16<<LATCH2_PIN_MODEM_NRESET)|(1u16<<LATCH2_PIN_MODEM_CODEC),IOMEM(LATCH2_VIRT));}
unsafe extern "C" fn ams_delta_init(){omap_gpio_deps_init();ams_delta_latch2_init();omap_serial_init();omap_register_i2c_bus(1,100,core::ptr::null_mut(),0);omap1_usb_init(&mut ams_delta_usb_config);}
unsafe extern "C" fn modem_pm(port:*mut uart_port,state:u32,old:u32){let p=(*port).private_data as *mut modem_private_data;if p.is_null()||(*p).regulator.is_null()||state==old{return}if state==0{regulator_enable((*p).regulator);}else if old==0{regulator_disable((*p).regulator);}}
unsafe extern "C" fn ams_delta_modem_pm_activate(dev:*mut device)->c_int{modem_priv.regulator=regulator_get(dev,b"RESET#\0".as_ptr() as _);if modem_priv.regulator.is_null(){-517}else{0}}
unsafe extern "C" fn ams_delta_modem_init()->c_int{-19}
unsafe extern "C" fn ams_delta_map_io(){/* omap1_map_io(); iotable_init(ams_delta_io_desc, 3); */}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
