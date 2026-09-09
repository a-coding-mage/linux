/* SPDX-License-Identifier: GPL-2.0 */

/*
 * This header is an extension of the vc_cons structure in console.c, with
 * information needed by the vt package.
 *
 * C header dependencies are supplied by the surrounding kernel translation.
 */

#[cfg(feature = "CONFIG_CONSOLE_TRANSLATIONS")]
extern "C" {
    pub fn kd_mksound(hz: u32, ticks: u32);
    pub fn kbd_rate(rep: *mut kbd_repeat) -> i32;

    pub static mut fg_console: i32;
    pub static mut last_console: i32;
    pub static mut want_console: i32;

    pub fn vc_allocate(console: u32) -> i32;
    pub fn vc_cons_allocated(console: u32) -> i32;
    pub fn __vc_resize(vc: *mut vc_data, cols: u32, lines: u32, from_user: bool) -> i32;
    pub fn vc_deallocate(console: u32) -> *mut vc_data;
    pub fn reset_palette(vc: *mut vc_data);
    pub fn do_blank_screen(entering_gfx: i32);
    pub fn do_unblank_screen(leaving_gfx: i32);
    pub fn poke_blanked_console();
    pub fn con_font_op(vc: *mut vc_data, op: *mut console_font_op) -> i32;
    pub fn con_set_cmap(cmap: *mut u8) -> i32;
    pub fn con_get_cmap(cmap: *mut u8) -> i32;
    pub fn scrollback(vc: *mut vc_data);
    pub fn scrollfront(vc: *mut vc_data, lines: i32);
    pub fn clear_buffer_attributes(vc: *mut vc_data);
    pub fn update_region(vc: *mut vc_data, start: usize, count: i32);
    pub fn redraw_screen(vc: *mut vc_data, is_switch: i32);

    pub fn tioclinux(tty: *mut tty_struct, arg: usize) -> i32;

    pub fn vt_event_post(event: u32, old: u32, new: u32);
    pub fn vt_waitactive(n: i32) -> i32;
    pub fn change_console(new_vc: *mut vc_data);
    pub fn reset_vc(vc: *mut vc_data);
    pub fn do_unbind_con_driver(csw: *const consw, first: i32, last: i32, deflt: i32) -> i32;
    pub fn vty_init(console_fops: *const file_operations) -> i32;

    pub static mut vt_dont_switch: bool;
    pub static mut default_utf8: i32;
    pub static mut global_cursor_default: i32;

    pub static mut vt_spawn_con: vt_spawn_console;

    pub fn vt_move_to_console(vt: u32, alloc: i32) -> i32;
    pub fn register_vt_notifier(nb: *mut notifier_block) -> i32;
    pub fn unregister_vt_notifier(nb: *mut notifier_block) -> i32;
    pub fn hide_boot_cursor(hide: bool);

    pub fn vt_do_diacrit(cmd: u32, up: *mut core::ffi::c_void, eperm: i32) -> i32;
    pub fn vt_do_kdskbmode(console: u32, arg: u32) -> i32;
    pub fn vt_do_kdskbmeta(console: u32, arg: u32) -> i32;
    pub fn vt_do_kbkeycode_ioctl(cmd: i32, user_kbkc: *mut kbkeycode, perm: i32) -> i32;
    pub fn vt_do_kdsk_ioctl(cmd: i32, user_kbe: *mut kbentry, perm: i32, console: u32) -> i32;
    pub fn vt_do_kdgkb_ioctl(cmd: i32, user_kdgkb: *mut kbsentry, perm: i32) -> i32;
    pub fn vt_do_kdskled(console: u32, cmd: i32, arg: usize, perm: i32) -> i32;
    pub fn vt_do_kdgkbmode(console: u32) -> i32;
    pub fn vt_do_kdgkbmeta(console: u32) -> i32;
    pub fn vt_reset_unicode(console: u32);
    pub fn vt_get_shift_state() -> i32;
    pub fn vt_reset_keyboard(console: u32);
    pub fn vt_get_leds(console: u32, flag: i32) -> i32;
    pub fn vt_get_kbd_mode_bit(console: u32, bit: i32) -> i32;
    pub fn vt_set_kbd_mode_bit(console: u32, bit: i32);
    pub fn vt_clr_kbd_mode_bit(console: u32, bit: i32);
    pub fn vt_set_led_state(console: u32, leds: i32);
    pub fn vt_kbd_con_start(console: u32);
    pub fn vt_kbd_con_stop(console: u32);
}

#[cfg(not(feature = "CONFIG_CONSOLE_TRANSLATIONS"))]
pub unsafe fn con_set_trans_old(_table: *mut u8) -> i32 { 0 }
#[cfg(not(feature = "CONFIG_CONSOLE_TRANSLATIONS"))]
pub unsafe fn con_get_trans_old(_table: *mut u8) -> i32 { -EINVAL }
#[cfg(not(feature = "CONFIG_CONSOLE_TRANSLATIONS"))]
pub unsafe fn con_set_trans_new(_table: *mut u16) -> i32 { 0 }
#[cfg(not(feature = "CONFIG_CONSOLE_TRANSLATIONS"))]
pub unsafe fn con_get_trans_new(_table: *mut u16) -> i32 { -EINVAL }
#[cfg(not(feature = "CONFIG_CONSOLE_TRANSLATIONS"))]
pub unsafe fn con_clear_unimap(_vc: *mut vc_data) -> i32 { 0 }
#[cfg(not(feature = "CONFIG_CONSOLE_TRANSLATIONS"))]
pub unsafe fn con_set_unimap(_vc: *mut vc_data, _ct: u16, _list: *mut unipair) -> i32 { 0 }
#[cfg(not(feature = "CONFIG_CONSOLE_TRANSLATIONS"))]
pub unsafe fn con_get_unimap(_vc: *mut vc_data, _ct: u16, _uct: *mut u16, _list: *mut unipair) -> i32 { -EINVAL }
#[cfg(not(feature = "CONFIG_CONSOLE_TRANSLATIONS"))]
pub unsafe fn con_set_default_unimap(_vc: *mut vc_data) -> i32 { 0 }
#[cfg(not(feature = "CONFIG_CONSOLE_TRANSLATIONS"))]
pub unsafe fn con_free_unimap(_vc: *mut vc_data) {}
#[cfg(not(feature = "CONFIG_CONSOLE_TRANSLATIONS"))]
pub unsafe fn con_protect_unimap(_vc: *mut vc_data, _rdonly: i32) {}
#[cfg(not(feature = "CONFIG_CONSOLE_TRANSLATIONS"))]
pub unsafe fn con_copy_unimap(_dst_vc: *mut vc_data, _src_vc: *mut vc_data) -> i32 { 0 }

extern "C" {
    pub fn con_set_trans_old(table: *mut u8) -> i32;
    pub fn con_get_trans_old(table: *mut u8) -> i32;
    pub fn con_set_trans_new(table: *mut u16) -> i32;
    pub fn con_get_trans_new(table: *mut u16) -> i32;
    pub fn con_clear_unimap(vc: *mut vc_data) -> i32;
    pub fn con_set_unimap(vc: *mut vc_data, ct: u16, list: *mut unipair) -> i32;
    pub fn con_get_unimap(vc: *mut vc_data, ct: u16, uct: *mut u16, list: *mut unipair) -> i32;
    pub fn con_set_default_unimap(vc: *mut vc_data) -> i32;
    pub fn con_free_unimap(vc: *mut vc_data);
    pub fn con_copy_unimap(dst_vc: *mut vc_data, src_vc: *mut vc_data) -> i32;
}

#[inline]
pub unsafe fn update_screen(x: *mut vc_data) { redraw_screen(x, 0); }
#[inline]
pub unsafe fn switch_screen(x: *mut vc_data) { redraw_screen(x, 1); }
#[inline]
pub unsafe fn vc_resize(vc: *mut vc_data, cols: u32, lines: u32) -> i32 {
    __vc_resize(vc, cols, lines, false)
}

#[repr(C)]
pub struct vt_spawn_console { pub lock: spinlock_t, pub pid: *mut pid, pub sig: i32 }
#[repr(C)]
pub struct vt_notifier_param { pub vc: *mut vc_data, pub c: u32 }

/* Declarations supplied by the included kernel headers. */
pub enum kbd_repeat {}
pub enum vc_data {}
pub enum console_font_op {}
pub enum tty_struct {}
pub enum consw {}
pub enum file_operations {}
pub enum notifier_block {}
pub enum kbkeycode {}
pub enum kbentry {}
pub enum kbsentry {}
pub enum unipair {}
pub enum spinlock_t {}
pub enum pid {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
