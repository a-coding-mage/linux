// SPDX-License-Identifier: GPL-2.0
/*
 * Procedures for drawing on the screen early on in the boot process.
 *
 * Benjamin Herrenschmidt <benh@kernel.crashing.org>
 */

// C headers provide these external kernel types, constants, and functions.

const NO_SCROLL: bool = true;

extern "C" {
    static mut prom_stdout: phandle;
    static font_sun_8x16: FontSun8x16;
    fn prom_getproperty(node: phandle, name: *const c_char, value: *mut c_char, len: i32) -> i32;
    fn prom_inst2pkg(stdout: phandle) -> phandle;
    fn strcmp(a: *const c_char, b: *const c_char) -> i32;
    fn register_console(con: *mut console);
}

type c_char = i8;
type phandle = u32;
type u32 = std::primitive::u32;

#[repr(C)]
struct FontSun8x16 { data: *const u8 }

#[repr(C)]
struct console {
    name: *const c_char,
    write: Option<unsafe extern "C" fn(*mut console, *const c_char, u32)>,
    flags: u32,
    index: i32,
}

const EINVAL: i32 = 22;
const ENODEV: i32 = 19;
const CON_PRINTBUFFER: u32 = 1 << 0;
const CON_ENABLED: u32 = 1 << 1;
const CON_BOOT: u32 = 1 << 2;
const CON_ANYTIME: u32 = 1 << 3;

static mut g_loc_X: i32 = 0;
static mut g_loc_Y: i32 = 0;
static mut g_max_loc_X: i32 = 0;
static mut g_max_loc_Y: i32 = 0;
static mut dispDeviceRowBytes: i32 = 0;
static mut dispDeviceDepth: i32 = 0;
static mut dispDeviceRect: [i32; 4] = [0; 4];
static mut dispDeviceBase: *mut u8 = std::ptr::null_mut();

unsafe fn btext_initialize(node: phandle) -> i32 {
    let mut width: u32 = 0;
    let mut height: u32 = 0;
    let mut depth: u32 = 0;
    let mut pitch: u32;
    let mut address: usize = 0;
    let mut prop: u32 = 0;

    if prom_getproperty(node, b"width\0".as_ptr() as *const c_char, &mut width as *mut _ as *mut c_char, 4) < 0 { return -EINVAL; }
    if prom_getproperty(node, b"height\0".as_ptr() as *const c_char, &mut height as *mut _ as *mut c_char, 4) < 0 { return -EINVAL; }
    if prom_getproperty(node, b"depth\0".as_ptr() as *const c_char, &mut depth as *mut _ as *mut c_char, 4) < 0 { return -EINVAL; }
    pitch = width * ((depth + 7) / 8);
    if prom_getproperty(node, b"linebytes\0".as_ptr() as *const c_char, &mut prop as *mut _ as *mut c_char, 4) >= 0 && prop != 0xffff_ffff { pitch = prop; }
    if pitch == 1 { pitch = 0x1000; }
    if prom_getproperty(node, b"address\0".as_ptr() as *const c_char, &mut prop as *mut _ as *mut c_char, 4) >= 0 { address = prop as usize; }
    /* FIXME: Add support for PCI reg properties. Right now, only
     * reliable on macs
     */
    if address == 0 { return -EINVAL; }
    g_loc_X = 0; g_loc_Y = 0;
    g_max_loc_X = (width / 8) as i32; g_max_loc_Y = (height / 16) as i32;
    dispDeviceBase = address as *mut u8;
    dispDeviceRowBytes = pitch as i32;
    dispDeviceDepth = if depth == 15 { 16 } else { depth as i32 };
    dispDeviceRect[0] = 0; dispDeviceRect[1] = 0; dispDeviceRect[2] = width as i32; dispDeviceRect[3] = height as i32;
    0
}

unsafe fn calc_base(x: i32, y: i32) -> *mut u8 {
    dispDeviceBase.add(((x + dispDeviceRect[0]) * (dispDeviceDepth >> 3)) as usize)
        .add(((y + dispDeviceRect[1]) * dispDeviceRowBytes) as usize)
}

unsafe fn btext_clearscreen() {
    let mut base = calc_base(0, 0) as *mut u32;
    let width = (((dispDeviceRect[2] - dispDeviceRect[0]) * (dispDeviceDepth >> 3)) >> 2) as usize;
    for _ in 0..(dispDeviceRect[3] - dispDeviceRect[1]) {
        for j in 0..width { base.add(j).write(0); }
        base = (base as *mut u8).add((dispDeviceRowBytes >> 2) as usize) as *mut u32;
    }
}

unsafe fn btext_drawchar(c: c_char) {
    let mut cline = 0;
    match c as u8 {
        b'\x08' => if g_loc_X > 0 { g_loc_X -= 1; },
        b'\t' => g_loc_X = (g_loc_X & -8) + 8,
        b'\r' => g_loc_X = 0,
        b'\n' => { g_loc_X = 0; g_loc_Y += 1; cline = 1; },
        _ => { draw_byte(c as u8, g_loc_X as i64, g_loc_Y as i64); g_loc_X += 1; }
    }
    if g_loc_X >= g_max_loc_X { g_loc_X = 0; g_loc_Y += 1; cline = 1; }
    if g_loc_Y >= g_max_loc_Y { g_loc_Y = 0; }
    if cline != 0 { for x in 0..g_max_loc_X { draw_byte(b' ', x as i64, g_loc_Y as i64); } }
}

unsafe fn btext_drawtext(mut c: *const c_char, mut len: u32) { while len != 0 { btext_drawchar(*c); c = c.add(1); len -= 1; } }

unsafe fn draw_byte(c: u8, loc_x: i64, loc_y: i64) {
    let base = calc_base((loc_x << 3) as i32, (loc_y << 4) as i32);
    let font = font_sun_8x16.data.add((c as usize) * 16);
    match dispDeviceDepth { 24 | 32 => draw_byte_32(font, base as *mut u32, dispDeviceRowBytes), 15 | 16 => draw_byte_16(font, base as *mut u32, dispDeviceRowBytes), 8 => draw_byte_8(font, base as *mut u32, dispDeviceRowBytes), _ => {} }
}

static expand_bits_8: [u32; 16] = [0,0xff,0xff00,0xffff,0xff0000,0xff00ff,0xffff00,0xffffff,0xff000000,0xff0000ff,0xff00ff00,0xff00ffff,0xffff0000,0xffff00ff,0xffffff00,0xffffffff];
static expand_bits_16: [u32; 4] = [0,0xffff,0xffff0000,0xffffffff];

unsafe fn draw_byte_32(mut font: *const u8, mut base: *mut u32, rb: i32) { for _ in 0..16 { let bits = *font; font = font.add(1); for i in 0..8 { base.add(i).write((-( ((bits >> (7-i)) & 1) as i32) as u32) ^ 0); } base = (base as *mut u8).add(rb as usize) as *mut u32; } }
unsafe fn draw_byte_16(mut font: *const u8, mut base: *mut u32, rb: i32) { for _ in 0..16 { let bits=*font; font=font.add(1); base.write(expand_bits_16[(bits>>6) as usize]); base.add(1).write(expand_bits_16[((bits>>4)&3) as usize]); base.add(2).write(expand_bits_16[((bits>>2)&3) as usize]); base.add(3).write(expand_bits_16[(bits&3) as usize]); base=(base as *mut u8).add(rb as usize) as *mut u32; } }
unsafe fn draw_byte_8(mut font: *const u8, mut base: *mut u32, rb: i32) { for _ in 0..16 { let bits=*font; font=font.add(1); base.write(expand_bits_8[(bits>>4) as usize] & 0x0f0f0f0f); base.add(1).write(expand_bits_8[(bits&0xf) as usize] & 0x0f0f0f0f); base=(base as *mut u8).add(rb as usize) as *mut u32; } }

unsafe extern "C" fn btext_console_write(_con: *mut console, s: *const c_char, n: u32) { btext_drawtext(s, n); }
static mut btext_console: console = console { name: b"btext\0".as_ptr() as *const c_char, write: Some(btext_console_write), flags: CON_PRINTBUFFER|CON_ENABLED|CON_BOOT|CON_ANYTIME, index: 0 };

pub unsafe extern "C" fn btext_find_display() -> i32 {
    let node = prom_inst2pkg(prom_stdout); let mut ty = [0i8; 32];
    if prom_getproperty(node, b"device_type\0".as_ptr() as *const c_char, ty.as_mut_ptr(), 32) < 0 { return -ENODEV; }
    if strcmp(ty.as_ptr(), b"display\0".as_ptr() as *const c_char) != 0 { return -ENODEV; }
    let ret = btext_initialize(node); if ret == 0 { btext_clearscreen(); register_console(&mut btext_console); } ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
