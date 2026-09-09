/* Rust translation of smc37c669.c.  Kernel-provided I/O and locking symbols
 * are intentionally left as external dependencies. */

use core::ptr;

pub const KB: usize = 1024;
pub const MB: usize = 1024 * KB;
pub const GB: usize = 1024 * MB;
pub const SMC_DEBUG: u32 = 0;
pub const SMC37C669_DEVICE_IRQ_MASK: u32 = 0x8000_0000;
pub const SMC37C669_DEVICE_DRQ_MASK: u32 = 0x8000_0000;
pub const SMC37C669_DEVICE_ID: u8 = 3;
pub const SERIAL_0: usize = 0;
pub const SERIAL_1: usize = 1;
pub const PARALLEL_0: usize = 2;
pub const FLOPPY_0: usize = 3;
pub const IDE_0: usize = 4;
pub const NUM_FUNCS: usize = 5;
pub const COM1_BASE: i32 = 0x3f8;
pub const COM1_IRQ: i32 = 4;
pub const COM2_BASE: i32 = 0x2f8;
pub const COM2_IRQ: i32 = 3;
pub const PARP_BASE: i32 = 0x3bc;
pub const PARP_IRQ: i32 = 7;
pub const PARP_DRQ: i32 = 3;
pub const FDC_BASE: i32 = 0x3f0;
pub const FDC_IRQ: i32 = 6;
pub const FDC_DRQ: i32 = 2;
pub const SMC37C669_CONFIG_ON_KEY: u8 = 0x55;
pub const SMC37C669_CONFIG_OFF_KEY: u8 = 0xaa;

pub const fn device_irq(i: u32) -> i32 { (SMC37C669_DEVICE_IRQ_MASK | i) as i32 }
pub const fn is_device_irq(i: i32) -> bool { (i as u32 & SMC37C669_DEVICE_IRQ_MASK) == SMC37C669_DEVICE_IRQ_MASK }
pub const fn raw_device_irq(i: i32) -> i32 { (i as u32 & !SMC37C669_DEVICE_IRQ_MASK) as i32 }
pub const fn device_drq(i: u32) -> i32 { (SMC37C669_DEVICE_DRQ_MASK | i) as i32 }
pub const fn is_device_drq(i: i32) -> bool { (i as u32 & SMC37C669_DEVICE_DRQ_MASK) == SMC37C669_DEVICE_DRQ_MASK }
pub const fn raw_device_drq(i: i32) -> i32 { (i as u32 & !SMC37C669_DEVICE_DRQ_MASK) as i32 }

pub const SMC37C669_DEVICE_IRQ_A: i32 = device_irq(1);
pub const SMC37C669_DEVICE_IRQ_B: i32 = device_irq(2);
pub const SMC37C669_DEVICE_IRQ_C: i32 = device_irq(3);
pub const SMC37C669_DEVICE_IRQ_D: i32 = device_irq(4);
pub const SMC37C669_DEVICE_IRQ_E: i32 = device_irq(5);
pub const SMC37C669_DEVICE_IRQ_F: i32 = device_irq(6);
pub const SMC37C669_DEVICE_IRQ_H: i32 = device_irq(8);
pub const SMC37C669_DEVICE_DRQ_A: i32 = device_drq(1);
pub const SMC37C669_DEVICE_DRQ_B: i32 = device_drq(2);
pub const SMC37C669_DEVICE_DRQ_C: i32 = device_drq(3);

#[repr(C)] pub struct SMC37c669_CONFIG_REGS { pub index_port: u8, pub data_port: u8 }
#[repr(C)] #[derive(Copy, Clone)] pub struct DEVICE_CONFIG { pub port1: u32, pub port2: u32, pub irq: i32, pub drq: i32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct SMC37c669_IRQ_TRANSLATION_ENTRY { pub device_irq: i32, pub isa_irq: i32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct SMC37c669_DRQ_TRANSLATION_ENTRY { pub device_drq: i32, pub isa_drq: i32 }

extern "C" { fn inb(port: u32) -> u8; fn outb(value: u8, port: u32); fn printk(fmt: *const u8, ...); }
static mut LOCAL_CONFIG: [DEVICE_CONFIG; NUM_FUNCS] = [DEVICE_CONFIG { port1: 0, port2: 0, irq: 0, drq: 0 }; NUM_FUNCS];
static mut SMC37C669: *mut SMC37c669_CONFIG_REGS = ptr::null_mut();
static mut IRQ_TABLE: *mut SMC37c669_IRQ_TRANSLATION_ENTRY = ptr::null_mut();
static mut DRQ_TABLE: *mut SMC37c669_DRQ_TRANSLATION_ENTRY = ptr::null_mut();
static ADDRESSES: [u32; 3] = [0x3f0, 0x370, 0];
static mut DEFAULT_IRQ_TABLE: [SMC37c669_IRQ_TRANSLATION_ENTRY; 8] = [
    SMC37c669_IRQ_TRANSLATION_ENTRY {device_irq: SMC37C669_DEVICE_IRQ_A, isa_irq:-1}, SMC37c669_IRQ_TRANSLATION_ENTRY {device_irq:SMC37C669_DEVICE_IRQ_B,isa_irq:-1},
    SMC37c669_IRQ_TRANSLATION_ENTRY {device_irq:SMC37C669_DEVICE_IRQ_C,isa_irq:7}, SMC37c669_IRQ_TRANSLATION_ENTRY {device_irq:SMC37C669_DEVICE_IRQ_D,isa_irq:6},
    SMC37c669_IRQ_TRANSLATION_ENTRY {device_irq:SMC37C669_DEVICE_IRQ_E,isa_irq:4}, SMC37c669_IRQ_TRANSLATION_ENTRY {device_irq:SMC37C669_DEVICE_IRQ_F,isa_irq:3},
    SMC37c669_IRQ_TRANSLATION_ENTRY {device_irq:SMC37C669_DEVICE_IRQ_H,isa_irq:-1}, SMC37c669_IRQ_TRANSLATION_ENTRY {device_irq:-1,isa_irq:-1}];
static mut DEFAULT_DRQ_TABLE: [SMC37c669_DRQ_TRANSLATION_ENTRY; 4] = [SMC37c669_DRQ_TRANSLATION_ENTRY{device_drq:SMC37C669_DEVICE_DRQ_A,isa_drq:2},SMC37c669_DRQ_TRANSLATION_ENTRY{device_drq:SMC37C669_DEVICE_DRQ_B,isa_drq:3},SMC37c669_DRQ_TRANSLATION_ENTRY{device_drq:SMC37C669_DEVICE_DRQ_C,isa_drq:-1},SMC37c669_DRQ_TRANSLATION_ENTRY{device_drq:-1,isa_drq:-1}];

unsafe fn read_config(i: u8) -> u8 { outb(i, SMC37C669 as u32); inb(SMC37C669 as u32 + 1) }
unsafe fn write_config(i: u8, v: u8) { outb(i, SMC37C669 as u32); outb(v, SMC37C669 as u32 + 1); }
unsafe fn config_mode(enable: bool) { if enable { outb(SMC37C669_CONFIG_ON_KEY, SMC37C669 as u32); outb(SMC37C669_CONFIG_ON_KEY, SMC37C669 as u32); } else { outb(SMC37C669_CONFIG_OFF_KEY, SMC37C669 as u32); } }
unsafe fn xlate_irq(v: i32) -> i32 { let mut i=0; while !IRQ_TABLE.is_null() { let e=*IRQ_TABLE.add(i); if e.device_irq == -1 && e.isa_irq == -1 { break } if (is_device_irq(v) && e.device_irq==v) || (!is_device_irq(v) && e.isa_irq==v) { return if is_device_irq(v){e.isa_irq}else{e.device_irq}; } i+=1; } -1 }
unsafe fn xlate_drq(v: i32) -> i32 { let mut i=0; while !DRQ_TABLE.is_null() { let e=*DRQ_TABLE.add(i); if e.device_drq == -1 && e.isa_drq == -1 { break } if (is_device_drq(v) && e.device_drq==v) || (!is_device_drq(v) && e.isa_drq==v) { return if is_device_drq(v){e.isa_drq}else{e.device_drq}; } i+=1; } -1 }

pub unsafe extern "C" fn SMC37c669_detect(index: i32) -> *mut SMC37c669_CONFIG_REGS { for a in ADDRESSES { if a==0 {break} SMC37C669=a as *mut _; config_mode(true); let id=read_config(0x0d); config_mode(false); if id==SMC37C669_DEVICE_ID { IRQ_TABLE=DEFAULT_IRQ_TABLE.as_mut_ptr(); DRQ_TABLE=DEFAULT_DRQ_TABLE.as_mut_ptr(); return SMC37C669; } SMC37C669=ptr::null_mut(); } let _=index; SMC37C669 }
pub unsafe extern "C" fn SMC37c669_enable_device(func: u32) -> u32 { if (func as usize)<NUM_FUNCS { config_mode(true); let c=LOCAL_CONFIG[func as usize]; write_config(0, (c.port1>>2) as u8); config_mode(false); 1 } else {0} }
pub unsafe extern "C" fn SMC37c669_disable_device(func: u32) -> u32 { if (func as usize)<NUM_FUNCS { config_mode(true); write_config(0,0); config_mode(false); 1 } else {0} }
pub unsafe extern "C" fn SMC37c669_configure_device(func:u32,port:i32,irq:i32,drq:i32)->u32 { if (func as usize)>=NUM_FUNCS{return 0} let c=&mut LOCAL_CONFIG[func as usize]; if drq & !0xff == 0 {c.drq=drq} if irq & !0xff == 0 {c.irq=irq} if port & !0xffff == 0 {c.port1=port as u32} 1 }
pub unsafe extern "C" fn SMC669_Init(index:i32) { if !SMC37c669_detect(index).is_null() { SMC37c669_disable_device(SERIAL_0 as u32); SMC37c669_configure_device(SERIAL_0 as u32,COM1_BASE,COM1_IRQ,-1); SMC37c669_enable_device(SERIAL_0 as u32); SMC37c669_disable_device(SERIAL_1 as u32); SMC37c669_configure_device(SERIAL_1 as u32,COM2_BASE,COM2_IRQ,-1); SMC37c669_enable_device(SERIAL_1 as u32); SMC37c669_disable_device(PARALLEL_0 as u32); SMC37c669_configure_device(PARALLEL_0 as u32,PARP_BASE,PARP_IRQ,PARP_DRQ); SMC37c669_enable_device(PARALLEL_0 as u32); SMC37c669_disable_device(FLOPPY_0 as u32); SMC37c669_configure_device(FLOPPY_0 as u32,FDC_BASE,FDC_IRQ,FDC_DRQ); SMC37c669_enable_device(FLOPPY_0 as u32); outb(0xc,0x3f2); SMC37c669_disable_device(IDE_0 as u32); } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
