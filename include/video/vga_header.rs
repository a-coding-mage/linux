/* Translation of linux/include/video/vga.h. */

pub const VGA_FB_PHYS_BASE: usize = 0xA0000;
pub const VGA_FB_PHYS_SIZE: usize = 65536;

pub const VGA_CRT_DC: u16 = 0x3D5; pub const VGA_CRT_DM: u16 = 0x3B5;
pub const VGA_ATT_R: u16 = 0x3C1; pub const VGA_ATT_W: u16 = 0x3C0;
pub const VGA_GFX_D: u16 = 0x3CF; pub const VGA_SEQ_D: u16 = 0x3C5;
pub const VGA_MIS_R: u16 = 0x3CC; pub const VGA_MIS_W: u16 = 0x3C2;
pub const VGA_FTC_R: u16 = 0x3CA; pub const VGA_IS0_R: u16 = 0x3C2;
pub const VGA_IS1_RC: u16 = 0x3DA; pub const VGA_IS1_RM: u16 = 0x3BA;
pub const VGA_PEL_D: u16 = 0x3C9; pub const VGA_PEL_MSK: u16 = 0x3C6;
pub const EGA_GFX_E0: u16 = 0x3CC; pub const EGA_GFX_E1: u16 = 0x3CA;
pub const VGA_CRT_IC: u16 = 0x3D4; pub const VGA_CRT_IM: u16 = 0x3B4;
pub const VGA_ATT_IW: u16 = 0x3C0; pub const VGA_GFX_I: u16 = 0x3CE;
pub const VGA_SEQ_I: u16 = 0x3C4; pub const VGA_PEL_IW: u16 = 0x3C8;
pub const VGA_PEL_IR: u16 = 0x3C7;

pub const VGA_CRT_C: u8 = 0x19; pub const VGA_ATT_C: u8 = 0x15;
pub const VGA_GFX_C: u8 = 0x09; pub const VGA_SEQ_C: u8 = 0x05; pub const VGA_MIS_C: u8 = 1;
pub const VGA_MIS_COLOR: u8 = 1; pub const VGA_MIS_ENB_MEM_ACCESS: u8 = 2;
pub const VGA_MIS_DCLK_28322_720: u8 = 4; pub const VGA_MIS_ENB_PLL_LOAD: u8 = 4 | 8;
pub const VGA_MIS_SEL_HIGH_PAGE: u8 = 0x20;

pub const VGA_CRTC_H_TOTAL: u8=0; pub const VGA_CRTC_H_DISP:u8=1; pub const VGA_CRTC_H_BLANK_START:u8=2; pub const VGA_CRTC_H_BLANK_END:u8=3;
pub const VGA_CRTC_H_SYNC_START:u8=4; pub const VGA_CRTC_H_SYNC_END:u8=5; pub const VGA_CRTC_V_TOTAL:u8=6; pub const VGA_CRTC_OVERFLOW:u8=7;
pub const VGA_CRTC_PRESET_ROW:u8=8; pub const VGA_CRTC_MAX_SCAN:u8=9; pub const VGA_CRTC_CURSOR_START:u8=0x0A; pub const VGA_CRTC_CURSOR_END:u8=0x0B;
pub const VGA_CRTC_START_HI:u8=0x0C; pub const VGA_CRTC_START_LO:u8=0x0D; pub const VGA_CRTC_CURSOR_HI:u8=0x0E; pub const VGA_CRTC_CURSOR_LO:u8=0x0F;
pub const VGA_CRTC_V_SYNC_START:u8=0x10; pub const VGA_CRTC_V_SYNC_END:u8=0x11; pub const VGA_CRTC_V_DISP_END:u8=0x12; pub const VGA_CRTC_OFFSET:u8=0x13;
pub const VGA_CRTC_UNDERLINE:u8=0x14; pub const VGA_CRTC_V_BLANK_START:u8=0x15; pub const VGA_CRTC_V_BLANK_END:u8=0x16; pub const VGA_CRTC_MODE:u8=0x17;
pub const VGA_CRTC_LINE_COMPARE:u8=0x18; pub const VGA_CRTC_REGS:u8=VGA_CRT_C;
pub const VGA_CR11_LOCK_CR0_CR7:u8=0x80; pub const VGA_CR17_H_V_SIGNALS_ENABLED:u8=0x80;

pub const VGA_ATC_PALETTE0:u8=0; pub const VGA_ATC_PALETTE1:u8=1; pub const VGA_ATC_PALETTE2:u8=2; pub const VGA_ATC_PALETTE3:u8=3; pub const VGA_ATC_PALETTE4:u8=4; pub const VGA_ATC_PALETTE5:u8=5; pub const VGA_ATC_PALETTE6:u8=6; pub const VGA_ATC_PALETTE7:u8=7; pub const VGA_ATC_PALETTE8:u8=8; pub const VGA_ATC_PALETTE9:u8=9; pub const VGA_ATC_PALETTEA:u8=0xA; pub const VGA_ATC_PALETTEB:u8=0xB; pub const VGA_ATC_PALETTEC:u8=0xC; pub const VGA_ATC_PALETTED:u8=0xD; pub const VGA_ATC_PALETTEE:u8=0xE; pub const VGA_ATC_PALETTEF:u8=0xF;
pub const VGA_ATC_MODE:u8=0x10; pub const VGA_ATC_OVERSCAN:u8=0x11; pub const VGA_ATC_PLANE_ENABLE:u8=0x12; pub const VGA_ATC_PEL:u8=0x13; pub const VGA_ATC_COLOR_PAGE:u8=0x14; pub const VGA_AR_ENABLE_DISPLAY:u8=0x20;
pub const VGA_SEQ_RESET:u8=0; pub const VGA_SEQ_CLOCK_MODE:u8=1; pub const VGA_SEQ_PLANE_WRITE:u8=2; pub const VGA_SEQ_CHARACTER_MAP:u8=3; pub const VGA_SEQ_MEMORY_MODE:u8=4;
pub const VGA_SR01_CHAR_CLK_8DOTS:u8=1; pub const VGA_SR01_SCREEN_OFF:u8=0x20; pub const VGA_SR02_ALL_PLANES:u8=0x0F; pub const VGA_SR04_EXT_MEM:u8=2; pub const VGA_SR04_SEQ_MODE:u8=4; pub const VGA_SR04_CHN_4M:u8=8;
pub const VGA_GFX_SR_VALUE:u8=0; pub const VGA_GFX_SR_ENABLE:u8=1; pub const VGA_GFX_COMPARE_VALUE:u8=2; pub const VGA_GFX_DATA_ROTATE:u8=3; pub const VGA_GFX_PLANE_READ:u8=4; pub const VGA_GFX_MODE:u8=5; pub const VGA_GFX_MISC:u8=6; pub const VGA_GFX_COMPARE_MASK:u8=7; pub const VGA_GFX_BIT_MASK:u8=8; pub const VGA_GR06_GRAPHICS_MODE:u8=1;

#[inline] pub const fn VGA_OUT16VAL(v: u8, r: u8) -> u16 { ((v as u16) << 8) | r as u16 }
pub const VGA_SAVE_FONT0:u32=1; pub const VGA_SAVE_FONT1:u32=2; pub const VGA_SAVE_TEXT:u32=4; pub const VGA_SAVE_FONTS:u32=7; pub const VGA_SAVE_MODE:u32=8; pub const VGA_SAVE_CMAP:u32=16;

#[repr(C)] pub struct vgastate { pub vgabase: *mut core::ffi::c_void, pub membase: usize, pub memsize:u32, pub flags:u32, pub depth:u32, pub num_attr:u32, pub num_crtc:u32, pub num_gfx:u32, pub num_seq:u32, pub vidstate:*mut core::ffi::c_void }
extern "C" { pub fn save_vga(state:*mut vgastate)->i32; pub fn restore_vga(state:*mut vgastate)->i32; pub fn readb(addr:*mut core::ffi::c_void)->u8; pub fn writeb(val:u8,addr:*mut core::ffi::c_void); pub fn writew(val:u16,addr:*mut core::ffi::c_void); pub fn inb_p(port:u16)->u8; pub fn outb_p(val:u8,port:u16); pub fn outw(val:u16,port:u16); }

#[inline] pub unsafe fn vga_mm_r(regbase:*mut core::ffi::c_void, port:u16)->u8 { readb(regbase.add(port as usize)) }
#[inline] pub unsafe fn vga_mm_w(regbase:*mut core::ffi::c_void, port:u16, val:u8) { writeb(val,regbase.add(port as usize)); }
#[inline] pub unsafe fn vga_mm_w_fast(regbase:*mut core::ffi::c_void, port:u16, reg:u8, val:u8) { writew(VGA_OUT16VAL(val,reg),regbase.add(port as usize)); }

#[cfg(feature="config_has_ioport")] #[inline] pub unsafe fn vga_io_r(port:u16)->u8 { inb_p(port) }
#[cfg(feature="config_has_ioport")] #[inline] pub unsafe fn vga_io_w(port:u16,val:u8) { outb_p(val,port) }
#[cfg(feature="config_has_ioport")] #[inline] pub unsafe fn vga_io_w_fast(port:u16,reg:u8,val:u8) { outw(VGA_OUT16VAL(val,reg),port) }
#[cfg(feature="config_has_ioport")] #[inline] pub unsafe fn vga_io_rcrt(r:u8)->u8 { vga_io_w(VGA_CRT_IC,r); vga_io_r(VGA_CRT_DC) }
#[cfg(feature="config_has_ioport")] #[inline] pub unsafe fn vga_io_wcrt(r:u8,v:u8) { vga_io_w_fast(VGA_CRT_IC,r,v) }
#[cfg(feature="config_has_ioport")] #[inline] pub unsafe fn vga_io_rseq(r:u8)->u8 { vga_io_w(VGA_SEQ_I,r); vga_io_r(VGA_SEQ_D) }
#[cfg(feature="config_has_ioport")] #[inline] pub unsafe fn vga_io_wseq(r:u8,v:u8) { vga_io_w_fast(VGA_SEQ_I,r,v) }
#[cfg(feature="config_has_ioport")] #[inline] pub unsafe fn vga_io_rgfx(r:u8)->u8 { vga_io_w(VGA_GFX_I,r); vga_io_r(VGA_GFX_D) }
#[cfg(feature="config_has_ioport")] #[inline] pub unsafe fn vga_io_wgfx(r:u8,v:u8) { vga_io_w_fast(VGA_GFX_I,r,v) }
#[cfg(feature="config_has_ioport")] #[inline] pub unsafe fn vga_io_rattr(r:u8)->u8 { vga_io_w(VGA_ATT_IW,r); vga_io_r(VGA_ATT_R) }
#[cfg(feature="config_has_ioport")] #[inline] pub unsafe fn vga_io_wattr(r:u8,v:u8) { vga_io_w(VGA_ATT_IW,r); vga_io_w(VGA_ATT_W,v) }

#[inline] pub unsafe fn vga_r(regbase:*mut core::ffi::c_void,port:u16)->u8 { vga_mm_r(regbase,port) }
#[inline] pub unsafe fn vga_w(regbase:*mut core::ffi::c_void,port:u16,val:u8) { vga_mm_w(regbase,port,val) }
#[inline] pub unsafe fn vga_w_fast(regbase:*mut core::ffi::c_void,port:u16,reg:u8,val:u8) { vga_mm_w_fast(regbase,port,reg,val) }

#[inline] pub unsafe fn vga_rcrt(b:*mut core::ffi::c_void,r:u8)->u8 { vga_w(b,VGA_CRT_IC,r); vga_r(b,VGA_CRT_DC) }
#[inline] pub unsafe fn vga_wcrt(b:*mut core::ffi::c_void,r:u8,v:u8) { vga_w_fast(b,VGA_CRT_IC,r,v) }
#[inline] pub unsafe fn vga_rseq(b:*mut core::ffi::c_void,r:u8)->u8 { vga_w(b,VGA_SEQ_I,r); vga_r(b,VGA_SEQ_D) }
#[inline] pub unsafe fn vga_wseq(b:*mut core::ffi::c_void,r:u8,v:u8) { vga_w_fast(b,VGA_SEQ_I,r,v) }
#[inline] pub unsafe fn vga_rgfx(b:*mut core::ffi::c_void,r:u8)->u8 { vga_w(b,VGA_GFX_I,r); vga_r(b,VGA_GFX_D) }
#[inline] pub unsafe fn vga_wgfx(b:*mut core::ffi::c_void,r:u8,v:u8) { vga_w_fast(b,VGA_GFX_I,r,v) }
#[inline] pub unsafe fn vga_rattr(b:*mut core::ffi::c_void,r:u8)->u8 { vga_w(b,VGA_ATT_IW,r); vga_r(b,VGA_ATT_R) }
#[inline] pub unsafe fn vga_wattr(b:*mut core::ffi::c_void,r:u8,v:u8) { vga_w(b,VGA_ATT_IW,r); vga_w(b,VGA_ATT_W,v) }
#[inline] pub unsafe fn vga_mm_rcrt(b:*mut core::ffi::c_void,r:u8)->u8 { vga_mm_w(b,VGA_CRT_IC,r); vga_mm_r(b,VGA_CRT_DC) }
#[inline] pub unsafe fn vga_mm_wcrt(b:*mut core::ffi::c_void,r:u8,v:u8) { vga_mm_w_fast(b,VGA_CRT_IC,r,v) }
#[inline] pub unsafe fn vga_mm_rseq(b:*mut core::ffi::c_void,r:u8)->u8 { vga_mm_w(b,VGA_SEQ_I,r); vga_mm_r(b,VGA_SEQ_D) }
#[inline] pub unsafe fn vga_mm_wseq(b:*mut core::ffi::c_void,r:u8,v:u8) { vga_mm_w_fast(b,VGA_SEQ_I,r,v) }
#[inline] pub unsafe fn vga_mm_rgfx(b:*mut core::ffi::c_void,r:u8)->u8 { vga_mm_w(b,VGA_GFX_I,r); vga_mm_r(b,VGA_GFX_D) }
#[inline] pub unsafe fn vga_mm_wgfx(b:*mut core::ffi::c_void,r:u8,v:u8) { vga_mm_w_fast(b,VGA_GFX_I,r,v) }
#[inline] pub unsafe fn vga_mm_rattr(b:*mut core::ffi::c_void,r:u8)->u8 { vga_mm_w(b,VGA_ATT_IW,r); vga_mm_r(b,VGA_ATT_R) }
#[inline] pub unsafe fn vga_mm_wattr(b:*mut core::ffi::c_void,r:u8,v:u8) { vga_mm_w(b,VGA_ATT_IW,r); vga_mm_w(b,VGA_ATT_W,v) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
