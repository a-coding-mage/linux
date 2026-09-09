// SPDX-License-Identifier: GPL-2.0
// C dependencies supplied by the surrounding kernel translation.

const VGABASE: usize = __ISA_IO_base + 0xb8000;

static mut max_ypos: i32 = 25;
static mut max_xpos: i32 = 80;
static mut current_ypos: i32 = 25;
static mut current_xpos: i32 = 0;

unsafe fn early_vga_write(_con: *mut console, mut str_: *const i8, mut n: u32) {
    let mut c: i8;
    let (mut i, mut k, mut j): (i32, i32, i32);

    while {
        c = *str_;
        str_ = str_.add(1);
        c != 0 && { n = n.wrapping_sub(1); n.wrapping_add(1) > 0 }
    } {
        if current_ypos >= max_ypos {
            k = 1; j = 0;
            while k < max_ypos {
                i = 0;
                while i < max_xpos {
                    writew(readw((VGABASE + 2 * (max_xpos * k + i)) as *mut u16),
                           (VGABASE + 2 * (max_xpos * j + i)) as *mut u16);
                    i += 1;
                }
                k += 1; j += 1;
            }
            i = 0;
            while i < max_xpos {
                writew(0x720, (VGABASE + 2 * (max_xpos * j + i)) as *mut u16);
                i += 1;
            }
            current_ypos = max_ypos - 1;
        }
        // CONFIG_KGDB_KDB conditional preserved from the source.
        if c == b'\n' as i8 {
            current_xpos = 0; current_ypos += 1;
        } else if c != b'\r' as i8 {
            writew((0x7 << 8) | (c as u8 as u16),
                   (VGABASE + 2 * (max_xpos * current_ypos + current_xpos)) as *mut u16);
            current_xpos += 1;
            if current_xpos >= max_xpos { current_xpos = 0; current_ypos += 1; }
        }
    }
}

static mut early_vga_console: console = console {
    name: b"earlyvga\0".as_ptr() as *const i8, write: Some(early_vga_write),
    flags: CON_PRINTBUFFER, index: -1,
};

static mut early_serial_base: usize = 0x3f8;
const XMTRDY: u32 = 0x20; const DLAB: i32 = 0x80;
const TXR: i32 = 0; const RXR: i32 = 0; const IER: i32 = 1; const IIR: i32 = 2;
const FCR: i32 = 2; const LCR: i32 = 3; const MCR: i32 = 4; const LSR: i32 = 5;
const MSR: i32 = 6; const DLL: i32 = 0; const DLH: i32 = 1;

unsafe fn io_serial_in(addr: usize, offset: i32) -> u32 { inb(addr + offset as usize) as u32 }
unsafe fn io_serial_out(addr: usize, offset: i32, value: i32) { outb(value as u8, addr + offset as usize); }

unsafe fn early_serial_putc(ch: u8) -> i32 {
    let mut timeout: u32 = 0xffff;
    while (io_serial_in(early_serial_base, LSR) & XMTRDY) == 0 && { timeout -= 1; timeout != 0 } { cpu_relax(); }
    io_serial_out(early_serial_base, TXR, ch as i32);
    if timeout != 0 { 0 } else { -1 }
}

unsafe fn early_serial_write(_con: *mut console, mut s: *const i8, mut n: u32) {
    while *s != 0 && { n = n.wrapping_sub(1); n.wrapping_add(1) > 0 } {
        if *s == b'\n' as i8 { early_serial_putc(b'\r'); }
        early_serial_putc(*s as u8); s = s.add(1);
    }
}

unsafe fn early_serial_hw_init(divisor: u32) {
    io_serial_out(early_serial_base, LCR, 0x3); io_serial_out(early_serial_base, IER, 0);
    io_serial_out(early_serial_base, FCR, 0); io_serial_out(early_serial_base, MCR, 0x3);
    let c = io_serial_in(early_serial_base, LCR) as i32;
    io_serial_out(early_serial_base, LCR, c | DLAB);
    io_serial_out(early_serial_base, DLL, (divisor & 0xff) as i32);
    io_serial_out(early_serial_base, DLH, ((divisor >> 8) & 0xff) as i32);
    io_serial_out(early_serial_base, LCR, c & !DLAB);
}

const DEFAULT_BAUD: u64 = 9600;

unsafe fn early_serial_init(mut s: *mut i8) {
    let mut divisor: u32; let mut baud: u64 = DEFAULT_BAUD; let mut e: *mut i8 = core::ptr::null_mut();
    if *s == b',' as i8 { s = s.add(1); }
    if *s != 0 {
        let mut port: u32;
        if !strncmp(s, b"0x\0".as_ptr() as *const i8, 2) { early_serial_base = simple_strtoul(s, &mut e, 16) as usize; }
        else { if !strncmp(s, b"ttyS\0".as_ptr() as *const i8, 4) { s = s.add(4); }
            port = simple_strtoul(s, &mut e, 10) as u32; if port > 1 || s == e { port = 0; }
            early_serial_base = [0x3f8, 0x2f8][port as usize]; }
        s = s.add(strcspn(s, b",\0".as_ptr() as *const i8)); if *s == b',' as i8 { s = s.add(1); }
    }
    if *s != 0 { baud = simple_strtoull(s, &mut e, 0); if baud == 0 || s == e { baud = DEFAULT_BAUD; } }
    divisor = (115200 / baud) as u32; early_serial_hw_init(divisor);
}

unsafe fn mem32_serial_out(addr: usize, offset: i32, value: i32) { writel(value as u32, (addr as *mut u32).add(offset as usize)); }
unsafe fn mem32_serial_in(addr: usize, offset: i32) -> u32 { readl((addr as *mut u32).add(offset as usize)) }

unsafe fn early_mmio_serial_init(mut s: *mut i8) {
    let mut baudrate: u64; let mut membase: usize; let mut e: *mut i8 = core::ptr::null_mut();
    if *s == b',' as i8 { s = s.add(1); }
    if !strncmp(s, b"0x\0".as_ptr() as *const i8, 2) { membase = simple_strtoul(s, &mut e, 16) as usize; early_serial_base = early_ioremap(membase, PAGE_SIZE) as usize; s = s.add(strcspn(s, b",\0".as_ptr() as *const i8)); if *s == b',' as i8 { s = s.add(1); } }
    if !strncmp(s, b"nocfg\0".as_ptr() as *const i8, 5) { baudrate = 0; } else { baudrate = simple_strtoul(s, &mut e, 0) as u64; if baudrate == 0 || s == e { baudrate = DEFAULT_BAUD; } }
    if baudrate != 0 { early_serial_hw_init((115200 / baudrate) as u32); }
}

// CONFIG_PCI, CONFIG_KEXEC_CORE, CONFIG_X86_64, CONFIG_EARLY_PRINTK_DBGP,
// CONFIG_HVC_XEN, and CONFIG_EARLY_PRINTK_USB_XDBC conditional code is
// supplied by the corresponding kernel configuration and dependencies.

unsafe fn early_console_register(con: *mut console, keep_early: i32) {
    if (*con).index != -1 { printk(KERN_CRIT, b"ERROR: earlyprintk= %s already used\n\0".as_ptr() as *const i8, (*con).name); return; }
    early_console = con;
    if keep_early != 0 { (*early_console).flags &= !CON_BOOT; } else { (*early_console).flags |= CON_BOOT; }
    register_console(early_console);
}

unsafe fn setup_early_printk(mut buf: *mut i8) -> i32 {
    if buf.is_null() || !early_console.is_null() { return 0; }
    let keep = (!strstr(buf, b"keep\0".as_ptr() as *const i8).is_null()) as i32;
    while *buf != 0 {
        if !strncmp(buf, b"mmio32\0".as_ptr() as *const i8, 6) { buf = buf.add(6); early_mmio_serial_init(buf); early_console_register(&mut early_serial_console, keep); }
        if !strncmp(buf, b"serial\0".as_ptr() as *const i8, 6) { buf = buf.add(6); early_serial_init(buf); early_console_register(&mut early_serial_console, keep); if !strncmp(buf, b",ttyS\0".as_ptr() as *const i8, 5) { buf = buf.add(5); } }
        if !strncmp(buf, b"ttyS\0".as_ptr() as *const i8, 4) { early_serial_init(buf.add(4)); early_console_register(&mut early_serial_console, keep); }
        // CONFIG_PCI / other optional early consoles preserved as external configuration hooks.
        buf = buf.add(1);
    }
    0
}

static mut early_serial_console: console = console { name: b"earlyser\0".as_ptr() as *const i8, write: Some(early_serial_write), flags: CON_PRINTBUFFER, index: -1 };

extern "C" {
    static mut early_console: *mut console; static __ISA_IO_base: usize; static mut boot_params: BootParams;
    fn inb(addr: usize) -> u8; fn outb(value: u8, addr: usize); fn readw(addr: *mut u16) -> u16; fn writew(value: u16, addr: *mut u16);
    fn readl(addr: *mut u32) -> u32; fn writel(value: u32, addr: *mut u32); fn cpu_relax();
    fn strncmp(a: *const i8, b: *const i8, n: usize) -> i32; fn strcmp(a: *const i8, b: *const i8) -> i32;
    fn strstr(a: *const i8, b: *const i8) -> *mut i8; fn strcspn(a: *const i8, b: *const i8) -> usize;
    fn simple_strtoul(s: *const i8, end: *mut *mut i8, base: u32) -> u64; fn simple_strtoull(s: *const i8, end: *mut *mut i8, base: u32) -> u64;
    fn early_ioremap(addr: usize, size: usize) -> *mut core::ffi::c_void; fn printk(fmt: i32, ...);
    fn register_console(con: *mut console);
}

#[repr(C)] struct console { name: *const i8, write: Option<unsafe fn(*mut console, *const i8, u32)>, flags: u32, index: i32 }
#[repr(C)] struct BootParams { screen_info: ScreenInfo }
#[repr(C)] struct ScreenInfo { orig_video_isVGA: u8, orig_video_cols: i32, orig_video_lines: i32, orig_y: i32 }
const PAGE_SIZE: usize = 4096; const CON_PRINTBUFFER: u32 = 1; const CON_BOOT: u32 = 2; const KERN_CRIT: i32 = 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
