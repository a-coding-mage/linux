/* Rust translation of linux/arch/m68k/amiga/config.c. */

use core::{ffi::c_void, ptr};

/* Kernel-provided types, constants, globals, and functions are external dependencies. */
extern "C" {
    static mut amiga_chip_size: usize;
    static mut amiga_custom: AmigaCustom;
    static mut ciab: Cia;
    static mut ciaa: Cia;
    static mut ciab_base: CiaBase;
    static mut iomem_resource: Resource;
    static mut m68k_num_memory: i32;
    static mut m68k_memory: [Memory; 16];
    fn amiga_sched_init(); fn amiga_init_IRQ(); fn amiga_get_model(_: *mut i8);
    fn amiga_get_hardware_list(_: *mut SeqFile); fn amiga_reset() -> !;
    fn amiga_mksound(_: u32, _: u32); fn amiga_init_sound(); fn amiga_chip_init();
    fn request_resource(_: *mut Resource, _: *mut Resource) -> i32;
    fn request_irq(_: i32, _: unsafe extern "C" fn(i32, *mut c_void) -> i32, _: u32, _: *const i8, _: *mut c_void) -> i32;
    fn legacy_timer_tick(_: i32); fn timer_heartbeat(); fn clocksource_register_hz(_: *mut Clocksource, _: u32) -> i32;
    fn local_irq_save(_: *mut usize); fn local_irq_restore(_: usize); fn local_irq_disable();
    fn cia_set_irq(_: *mut CiaBase, _: u8) -> u8;
    fn register_console(_: *mut Console) -> i32; fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8; fn memcpy(_: *mut c_void, _: *const c_void, _: usize) -> *mut c_void;
    fn pr_info(_: *const i8, ...); fn pr_warn(_: *const i8, ...); fn pr_err(_: *const i8, ...); fn pr_cont(_: *const i8, ...);
    fn seq_printf(_: *mut SeqFile, _: *const i8, ...); fn seq_puts(_: *mut SeqFile, _: *const i8);
    fn panic(_: *const i8) -> !;
}

#[repr(C)] pub struct Resource { pub name: *const i8, pub start: usize, pub end: usize }
#[repr(C)] pub struct Memory { pub addr: usize, pub size: usize }
#[repr(C)] pub struct AmigaCustom { pub dmacon: u16, pub serdat: u16, pub serdatr: u16, pub intreq: u16, pub intreqr: u16, pub deniseid: u16, pub vposr: u16 }
#[repr(C)] pub struct Cia { pub pra: u8, pub _pad: [u8; 0x0f], pub talo: u8, pub tahi: u8, pub _pad2: [u8; 0x1e], pub cra: u8 }
#[repr(C)] pub struct CiaBase { _x: [u8; 0] }
#[repr(C)] pub struct SeqFile { _x: [u8; 0] }
#[repr(C)] pub struct Console { pub write: Option<unsafe extern "C" fn(*mut Console, *const i8, u32)> }
#[repr(C)] pub struct Clocksource { pub name: *const i8, pub rating: i32, pub read: Option<unsafe extern "C" fn(*mut Clocksource) -> u64>, pub mask: u64, pub flags: u32 }
#[repr(C)] pub struct AmigaHwPresent { pub data: [u32; 8] }
#[repr(C)] pub struct BiRecord { pub tag: u16, pub data: *const c_void }

static mut amiga_model: usize = 0;
#[no_mangle] pub static mut amiga_eclock: u32 = 0;
#[no_mangle] pub static mut amiga_colorclock: u32 = 0;
#[no_mangle] pub static mut amiga_chipset: u32 = 0;
#[no_mangle] pub static mut amiga_vblank: u8 = 0;
static mut amiga_psfreq: u8 = 0;
#[no_mangle] pub static mut amiga_hw_present: AmigaHwPresent = AmigaHwPresent { data: [0; 8] };
static mut amiga_model_name: [u8; 13] = *b"Amiga \0\0\0\0\0\0\0";
static mut ram_resource: [Resource; 16] = [Resource { name: ptr::null(), start: 0, end: 0 }; 16];
static mut mb_resources: [Resource; 4] = [
    Resource { name: b"CIA B\0".as_ptr() as *const i8, start: 0x00bfd000, end: 0x00bfdfff },
    Resource { name: b"CIA A\0".as_ptr() as *const i8, start: 0x00bfe000, end: 0x00bfefff },
    Resource { name: b"Custom I/O\0".as_ptr() as *const i8, start: 0x00dff000, end: 0x00dfffff },
    Resource { name: b"Kickstart ROM\0".as_ptr() as *const i8, start: 0x00f80000, end: 0x00ffffff },
];

#[no_mangle] pub unsafe extern "C" fn amiga_parse_bootinfo(record: *const BiRecord) -> i32 {
    let mut unknown = 0;
    let tag = u16::from_be((*record).tag);
    match tag {
        0x0001 => amiga_model = u32::from_be(*( (*record).data as *const u32)) as usize,
        0x0002 => amiga_eclock = u32::from_be(*( (*record).data as *const u32)),
        0x0003 => amiga_chipset = u32::from_be(*( (*record).data as *const u32)),
        0x0004 => amiga_chip_size = u32::from_be(*( (*record).data as *const u32)) as usize,
        0x0005 => amiga_vblank = *((*record).data as *const u8),
        0x0006 => amiga_psfreq = *((*record).data as *const u8),
        0x0008 => {}, // BI_AMIGA_AUTOCON; CONFIG_ZORRO-dependent body
        0x0007 => {}, // serial period is ignored
        _ => unknown = 1,
    }
    unknown
}

unsafe fn amiga_identify() {
    if amiga_eclock == 0 { amiga_eclock = 709379; }
    amiga_hw_present = AmigaHwPresent { data: [0; 8] };
    pr_info(b"Amiga hardware found: \0".as_ptr() as *const i8);
    /* AMIGAHW_SET/AMIGAHW_PRESENT and model constants are supplied by asm/amigahw.h. */
    pr_cont(b"\n\0".as_ptr() as *const i8);
}

unsafe extern "C" fn amiga_random_get_entropy(_: *mut Clocksource) -> u64 { amiga_custom.vposr as u64 }

#[no_mangle] pub unsafe extern "C" fn config_amiga() {
    amiga_identify();
    iomem_resource.name = b"Memory\0".as_ptr() as *const i8;
    for i in 0..4 { request_resource(&mut iomem_resource, &mut mb_resources[i]); }
    amiga_colorclock = 5 * amiga_eclock;
    amiga_custom.dmacon = 0x7fff;
    amiga_custom.dmacon = 0x8000 | 0x0200;
    for i in 0..m68k_num_memory as usize {
        ram_resource[i].name = if m68k_memory[i].addr >= 0x01000000 { b"32-bit Fast RAM\0" } else if m68k_memory[i].addr < 0x00c00000 { b"16-bit Fast RAM\0" } else { b"16-bit Slow RAM\0" }.as_ptr() as *const i8;
        ram_resource[i].start = m68k_memory[i].addr;
        ram_resource[i].end = m68k_memory[i].addr + m68k_memory[i].size - 1;
        request_resource(&mut iomem_resource, &mut ram_resource[i]);
    }
    amiga_chip_init();
    amiga_init_sound();
}

unsafe extern "C" fn amiga_serial_putc(c: i8) { amiga_custom.serdat = c as u16 | 0x100; while amiga_custom.serdatr & 0x2000 == 0 {} }
unsafe extern "C" fn amiga_serial_console_write(_: *mut Console, s: *const i8, mut count: u32) { while count != 0 { let c = *s; if c == b'\n' as i8 { amiga_serial_putc(b'\r' as i8); } amiga_serial_putc(c); count -= 1; } }

#[cfg(feature = "heartbeat")]
unsafe extern "C" fn amiga_heartbeat(on: i32) { if on != 0 { ciaa.pra &= !2; } else { ciaa.pra |= 2; } }

#[no_mangle] pub unsafe extern "C" fn amiga_get_model(model: *mut i8) { strcpy(model, amiga_model_name.as_ptr() as *const i8); }

unsafe extern "C" fn amiga_mem_console_write(_: *mut Console, _: *const i8, _: u32) {}
static mut savekmsg: *mut c_void = ptr::null_mut();
unsafe extern "C" fn amiga_debug_setup(arg: *mut i8) -> i32 { if strcmp(arg, b"ser\0".as_ptr() as *const i8) != 0 { return 0; } 0 }
unsafe extern "C" fn amiga_savekmsg_setup(arg: *mut i8) -> i32 { if strcmp(arg, b"mem\0".as_ptr() as *const i8) != 0 { return 0; } savekmsg = ptr::null_mut(); 0 }

static mut jiffy_ticks: u16 = 0;
static mut clk_total: u32 = 0;
static mut clk_offset: u32 = 0;
unsafe extern "C" fn ciab_timer_handler(_: i32, _: *mut c_void) -> i32 { clk_total = clk_total.wrapping_add(jiffy_ticks as u32); clk_offset = 0; legacy_timer_tick(1); timer_heartbeat(); 1 }
unsafe extern "C" fn amiga_read_clk(_: *mut Clocksource) -> u64 {
    let mut flags = 0usize; local_irq_save(&mut flags); let mut hi = ciab.tahi; let mut lo = ciab.talo; let hi2 = ciab.tahi;
    if hi != hi2 { lo = ciab.talo; hi = hi2; } let mut ticks = ((hi as u32) << 8) | lo as u32;
    if ticks > jiffy_ticks as u32 / 2 && cia_set_irq(&mut ciab_base, 0) & 1 != 0 { clk_offset = jiffy_ticks as u32; }
    ticks = (jiffy_ticks as u32 - ticks).wrapping_add(clk_offset).wrapping_add(clk_total); local_irq_restore(flags); ticks as u64
}
unsafe extern "C" fn amiga_sched_init() { jiffy_ticks = amiga_eclock as u16; ciab.cra &= 0xc0; ciab.talo = (jiffy_ticks % 256) as u8; ciab.tahi = (jiffy_ticks / 256) as u8; ciab.cra |= 0x11; }
#[no_mangle] pub unsafe extern "C" fn amiga_get_hardware_list(m: *mut SeqFile) { seq_puts(m, b"Detected hardware:\n\0".as_ptr() as *const i8); }
#[no_mangle] pub unsafe extern "C" fn amiga_reset() -> ! { local_irq_disable(); loop { core::hint::spin_loop(); } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
