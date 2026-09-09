// SPDX-License-Identifier: GPL-2.0
/* Procedures for drawing on the screen early on in the boot process. */

// C dependencies supplied by the surrounding kernel translation.

#[repr(C)]
pub struct DeviceNode {
    _private: [u8; 0],
}

#[repr(C)]
pub struct FontSun8x16 {
    pub data: *const u8,
}

extern "C" {
    fn rmci_on();
    fn rmci_off();
    fn ioremap_wc(base: usize, size: usize) -> *mut u8;
    fn iounmap(addr: *mut u8);
    fn of_get_property(np: *mut DeviceNode, name: *const u8, len: *mut u32) -> *const u32;
    fn of_node_is_type(np: *mut DeviceNode, name: *const u8) -> bool;
    fn of_node_put(np: *mut DeviceNode);
    fn printk(fmt: *const u8, ...);
    static mut of_stdout: *mut DeviceNode;
    static mut font_sun_8x16: FontSun8x16;
    static mut udbg_putc: Option<unsafe extern "C" fn(c: i8)>;
}

// Build-time kernel constants and PTRRELOC are provided by the surrounding code.
pub static mut disp_BAT: [usize; 2] = [0, 0];

static mut g_loc_X: i32 = 0;
static mut g_loc_Y: i32 = 0;
static mut g_max_loc_X: i32 = 0;
static mut g_max_loc_Y: i32 = 0;
static mut dispDeviceRowBytes: i32 = 0;
static mut dispDeviceDepth: i32 = 0;
static mut dispDeviceRect: [i32; 4] = [0; 4];
static mut dispDeviceBase: *mut u8 = core::ptr::null_mut();
static mut logicalDisplayBase: *mut u8 = core::ptr::null_mut();
pub static mut boot_text_mapped: i32 = 0;

#[inline]
unsafe fn rmci_maybe_on() {
    // CONFIG_PPC_EARLY_DEBUG_BOOTX && CONFIG_PPC64 conditional retained from C.
}

#[inline]
unsafe fn rmci_maybe_off() {
    // CONFIG_PPC_EARLY_DEBUG_BOOTX && CONFIG_PPC64 conditional retained from C.
}

#[cfg(target_arch = "powerpc")]
pub unsafe extern "C" fn btext_prepare_BAT() {
    // CONFIG_PPC32 implementation; PAGE_OFFSET, BL_16M, _PAGE_NO_CACHE,
    // _PAGE_GUARDED, and BPP_RW are supplied by the target kernel.
    let vaddr: usize = PAGE_OFFSET + 0x10000000;
    let mut addr = dispDeviceBase as usize;
    if addr == 0 {
        boot_text_mapped = 0;
        return;
    }
    let lowbits = addr & !0xFF000000usize;
    addr &= 0xFF000000usize;
    disp_BAT[0] = vaddr | (BL_16M << 2) | 2;
    disp_BAT[1] = addr | (_PAGE_NO_CACHE | _PAGE_GUARDED | BPP_RW);
    logicalDisplayBase = (vaddr + lowbits) as *mut u8;
}

extern "C" {
    static PAGE_OFFSET: usize;
    static BL_16M: usize;
    static _PAGE_NO_CACHE: usize;
    static _PAGE_GUARDED: usize;
    static BPP_RW: usize;
}

pub unsafe extern "C" fn btext_setup_display(width: i32, height: i32, depth: i32, pitch: i32, address: usize) {
    g_loc_X = 0; g_loc_Y = 0; g_max_loc_X = width / 8; g_max_loc_Y = height / 16;
    logicalDisplayBase = address as *mut u8; dispDeviceBase = address as *mut u8;
    dispDeviceRowBytes = pitch; dispDeviceDepth = if depth == 15 { 16 } else { depth };
    dispDeviceRect = [0, 0, width, height]; boot_text_mapped = 1;
}

pub unsafe extern "C" fn btext_unmap() { boot_text_mapped = 0; }

pub unsafe extern "C" fn btext_map() {
    boot_text_mapped = 0;
    if dispDeviceBase.is_null() { return; }
    let base = (dispDeviceBase as usize) & 0xFFFFF000;
    let offset = dispDeviceBase as usize - base;
    let size = dispDeviceRowBytes as usize * dispDeviceRect[3] as usize + offset + dispDeviceRect[0] as usize;
    let vbase = ioremap_wc(base, size);
    if vbase.is_null() { return; }
    logicalDisplayBase = vbase.add(offset); boot_text_mapped = 1;
}

unsafe fn btext_initialize(np: *mut DeviceNode) -> i32 {
    let mut prop: *const u32;
    prop = of_get_property(np, b"linux,bootx-width\0".as_ptr(), core::ptr::null_mut());
    if prop.is_null() { prop = of_get_property(np, b"width\0".as_ptr(), core::ptr::null_mut()); }
    if prop.is_null() { return -22; } let width = *prop;
    prop = of_get_property(np, b"linux,bootx-height\0".as_ptr(), core::ptr::null_mut());
    if prop.is_null() { prop = of_get_property(np, b"height\0".as_ptr(), core::ptr::null_mut()); }
    if prop.is_null() { return -22; } let height = *prop;
    prop = of_get_property(np, b"linux,bootx-depth\0".as_ptr(), core::ptr::null_mut());
    if prop.is_null() { prop = of_get_property(np, b"depth\0".as_ptr(), core::ptr::null_mut()); }
    if prop.is_null() { return -22; } let depth = *prop;
    let mut pitch = width * ((depth + 7) / 8);
    prop = of_get_property(np, b"linux,bootx-linebytes\0".as_ptr(), core::ptr::null_mut());
    if prop.is_null() { prop = of_get_property(np, b"linebytes\0".as_ptr(), core::ptr::null_mut()); }
    if !prop.is_null() && *prop != 0xffff_ffff { pitch = *prop; }
    if pitch == 1 { pitch = 0x1000; }
    prop = of_get_property(np, b"linux,bootx-addr\0".as_ptr(), core::ptr::null_mut());
    if prop.is_null() { prop = of_get_property(np, b"address\0".as_ptr(), core::ptr::null_mut()); }
    let address = if prop.is_null() { 0 } else { *prop as usize };
    if address == 0 { return -22; }
    btext_setup_display(width as i32, height as i32, depth as i32, pitch as i32, address);
    btext_map(); 0
}

pub unsafe extern "C" fn btext_find_display(allow_nonstdout: i32) -> i32 {
    let mut np = of_stdout; let mut rc = -19;
    if np.is_null() || !of_node_is_type(np, b"display\0".as_ptr()) { printk(b"boot stdout isn't a display !\n\0".as_ptr()); np = core::ptr::null_mut(); }
    if !np.is_null() { rc = btext_initialize(np); }
    if rc == 0 || allow_nonstdout == 0 { return rc; }
    // for_each_node_by_type(np, "display") and linux,opened filtering are external iteration macros.
    rc
}

unsafe fn calc_base(x: i32, y: i32) -> *mut u8 {
    let mut base = if !logicalDisplayBase.is_null() { logicalDisplayBase } else { dispDeviceBase };
    base = base.add(((x + dispDeviceRect[0]) * (dispDeviceDepth >> 3)) as usize);
    base.add(((y + dispDeviceRect[1]) * dispDeviceRowBytes) as usize)
}

pub unsafe extern "C" fn btext_update_display(phys: usize, width: i32, height: i32, depth: i32, pitch: i32) {
    if dispDeviceBase.is_null() || ((phys ^ dispDeviceBase as usize) & 0xf0000000) != 0 { return; }
    dispDeviceBase = phys as *mut u8; dispDeviceRect = [0, 0, width, height]; dispDeviceDepth = depth; dispDeviceRowBytes = pitch;
    if boot_text_mapped != 0 { iounmap(logicalDisplayBase); boot_text_mapped = 0; }
    btext_map(); g_loc_X = 0; g_loc_Y = 0; g_max_loc_X = width / 8; g_max_loc_Y = height / 16;
}

pub unsafe extern "C" fn btext_clearscreen() {
    let mut base = calc_base(0, 0) as *mut u32;
    let width = (((dispDeviceRect[2] - dispDeviceRect[0]) * (dispDeviceDepth >> 3)) >> 2) as usize;
    rmci_maybe_on();
    for _ in 0..(dispDeviceRect[3] - dispDeviceRect[1]) { for j in 0..width { base.add(j).write_volatile(0); } base = (base as *mut u8).add((dispDeviceRowBytes >> 2) as usize) as *mut u32; }
    rmci_maybe_off();
}

pub unsafe extern "C" fn btext_flushscreen() { flush_lines(0); }
pub unsafe extern "C" fn btext_flushline() { flush_lines(g_loc_Y << 4); }

unsafe fn flush_lines(y: i32) {
    let mut base = calc_base(0, y) as *mut u32;
    let width = (((dispDeviceRect[2] - dispDeviceRect[0]) * (dispDeviceDepth >> 3)) >> 2) as usize;
    let lines = if y == 0 { dispDeviceRect[3] - dispDeviceRect[1] } else { 16 };
    for _ in 0..lines { let mut ptr = base; let mut j = width; while j > 0 { core::arch::asm!("dcbst 0, {p}", p = in(reg) ptr); ptr = ptr.add(8); j -= 8; } base = (base as *mut u8).add((dispDeviceRowBytes >> 2) as usize) as *mut u32; }
    core::arch::asm!("sync");
}

static EXPAND_BITS_8: [u32; 16] = [0,0xff,0xff00,0xffff,0xff0000,0xff00ff,0xffff00,0xffffff,0xff000000,0xff0000ff,0xff00ff00,0xff00ffff,0xffff0000,0xffff00ff,0xffffff00,0xffffffff];
static EXPAND_BITS_16: [u32; 4] = [0,0xffff,0xffff0000,0xffffffff];

unsafe fn draw_byte_32(mut font: *const u8, mut base: *mut u32, rb: i32) { for _ in 0..16 { let bits = *font; font = font.add(1); for n in 0..8 { base.add(n).write((0u32.wrapping_sub((bits >> (7-n)) as u32)) ^ 0); } base = (base as *mut u8).add(rb as usize) as *mut u32; } }
unsafe fn draw_byte_16(mut font: *const u8, mut base: *mut u32, rb: i32) { for _ in 0..16 { let bits=*font; font=font.add(1); base.write(EXPAND_BITS_16[(bits>>6) as usize]); base.add(1).write(EXPAND_BITS_16[((bits>>4)&3) as usize]); base.add(2).write(EXPAND_BITS_16[((bits>>2)&3) as usize]); base.add(3).write(EXPAND_BITS_16[(bits&3) as usize]); base=(base as *mut u8).add(rb as usize) as *mut u32; } }
unsafe fn draw_byte_8(mut font: *const u8, mut base: *mut u32, rb: i32) { for _ in 0..16 { let bits=*font; font=font.add(1); base.write(EXPAND_BITS_8[(bits>>4) as usize] & 0x0f0f0f0f); base.add(1).write(EXPAND_BITS_8[(bits&15) as usize] & 0x0f0f0f0f); base=(base as *mut u8).add(rb as usize) as *mut u32; } }

unsafe fn draw_byte(c: u8, loc_x: i32, loc_y: i32) { let base=calc_base(loc_x<<3, loc_y<<4); let font=font_sun_8x16.data.add((c as usize)*16); match dispDeviceDepth { 24|32=>draw_byte_32(font,base as *mut u32,dispDeviceRowBytes), 15|16=>draw_byte_16(font,base as *mut u32,dispDeviceRowBytes), 8=>draw_byte_8(font,base as *mut u32,dispDeviceRowBytes), _=>{} } }

pub unsafe extern "C" fn btext_drawchar(c: i8) { if boot_text_mapped==0{return;} let c=c as u8; let mut cline=0; match c { 8=>{if g_loc_X>0{g_loc_X-=1;}}, 9=>g_loc_X=(g_loc_X & -8)+8, 13=>g_loc_X=0, 10=>{g_loc_X=0;g_loc_Y+=1;cline=1;}, _=>{draw_byte(c,g_loc_X,g_loc_Y);g_loc_X+=1;} } if g_loc_X>=g_max_loc_X {g_loc_X=0;g_loc_Y+=1;cline=1;} if g_loc_Y>=g_max_loc_Y {g_loc_Y=0;} if cline!=0 {for x in 0..g_max_loc_X {draw_byte(b' ',x,g_loc_Y);}} }
pub unsafe extern "C" fn btext_drawstring(mut c:*const i8){if boot_text_mapped==0{return;} while *c!=0 {btext_drawchar(*c);c=c.add(1);}}
pub unsafe extern "C" fn btext_drawtext(mut c:*const i8, mut len:u32){if boot_text_mapped==0{return;} while len!=0 {btext_drawchar(*c);c=c.add(1);len-=1;}}
pub unsafe extern "C" fn btext_drawhex(v:usize){if boot_text_mapped==0{return;} for i in (0..(core::mem::size_of::<usize>()*2)).rev(){let n=((v>>(i*4))&15) as u8; btext_drawchar(if n<10 {b'0'+n}else{b'a'+n-10} as i8);} btext_drawchar(b' ' as i8);}
pub unsafe extern "C" fn udbg_init_btext(){udbg_putc=Some(btext_drawchar);}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
