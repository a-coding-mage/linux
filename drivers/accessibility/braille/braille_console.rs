// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Minimalistic braille device kernel support.
 *
 * By default, shows console messages on the braille device.
 * Pressing Insert switches to VC browsing.
 *
 *  Copyright (C) Samuel Thibault <samuel.thibault@ens-lyon.org>
 */

// Kernel headers and symbols are supplied by the surrounding kernel bindings.

// Braille device support part.

/* Emit various sounds */
static mut sound: bool = false;

unsafe fn beep(freq: u32) {
    if sound {
        kd_mksound(freq, HZ / 10);
    }
}

/* mini console */
const WIDTH: usize = 40;
const BRAILLE_KEY: u32 = KEY_INSERT;
static mut console_buf: [u16; WIDTH] = [0; WIDTH];
static mut console_cursor: i32 = 0;

/* mini view of VC */
static mut vc_x: i32 = 0;
static mut vc_y: i32 = 0;
static mut lastvc_x: i32 = 0;
static mut lastvc_y: i32 = 0;

/* show console ? (or show VC) */
static mut console_show: i32 = 1;
/* pending newline ? */
static mut console_newline: i32 = 1;
static mut lastVC: i32 = -1;

static mut braille_co: *mut console = core::ptr::null_mut();

/* Very VisioBraille-specific */
unsafe fn braille_write(buf: *mut u16) {
    static mut lastwrite: [u16; WIDTH] = [0; WIDTH];
    let mut data = [0u8; 1 + 1 + 2 * WIDTH + 2 + 1];
    let mut csum: u8 = 0;

    if braille_co.is_null() {
        return;
    }
    if core::slice::from_raw_parts(lastwrite.as_ptr(), WIDTH)
        == core::slice::from_raw_parts(buf, WIDTH)
    {
        return;
    }
    core::ptr::copy_nonoverlapping(buf, lastwrite.as_mut_ptr(), WIDTH);

    const SOH: u8 = 1;
    const STX: u8 = 2;
    const ETX: u8 = 2;
    data[0] = STX;
    data[1] = b'>';
    csum ^= b'>';
    let mut pos = 2usize;
    for i in 0..WIDTH {
        let mut out = *buf.add(i);
        if out >= 0x100 {
            out = b'?' as u16;
        } else if out == 0x00 {
            out = b' ' as u16;
        }
        csum ^= out as u8;
        if out <= 0x05 {
            data[pos] = SOH;
            pos += 1;
            out |= 0x40;
        }
        data[pos] = out as u8;
        pos += 1;
    }
    if csum <= 0x05 {
        data[pos] = SOH;
        pos += 1;
        csum |= 0x40;
    }
    data[pos] = csum;
    pos += 1;
    data[pos] = ETX;
    pos += 1;
    ((*braille_co).write.unwrap())(braille_co, data.as_mut_ptr(), pos);
}

/* Follow the VC cursor*/
unsafe fn vc_follow_cursor(vc: *mut vc_data) {
    vc_x = (*vc).state.x - ((*vc).state.x % WIDTH as i32);
    vc_y = (*vc).state.y;
    lastvc_x = (*vc).state.x;
    lastvc_y = (*vc).state.y;
}

/* Maybe the VC cursor moved, if so follow it */
unsafe fn vc_maybe_cursor_moved(vc: *mut vc_data) {
    if (*vc).state.x != lastvc_x || (*vc).state.y != lastvc_y {
        vc_follow_cursor(vc);
    }
}

/* Show portion of VC at vc_x, vc_y */
unsafe fn vc_refresh(vc: *mut vc_data) {
    let mut buf = [0u16; WIDTH];
    for i in 0..WIDTH {
        let glyph = screen_glyph(vc, 2 * (vc_x + i as i32) + vc_y * (*vc).vc_size_row);
        buf[i] = inverse_translate(vc, glyph, true);
    }
    braille_write(buf.as_mut_ptr());
}

/*
 * Link to keyboard
 */
unsafe extern "C" fn keyboard_notifier_call(
    _blk: *mut notifier_block, code: c_ulong, _param: *mut c_void,
) -> i32 {
    let param = _param as *mut keyboard_notifier_param;
    let vc = (*param).vc;
    let mut ret = NOTIFY_OK;
    if !(*param).down { return ret; }
    match code {
        KBD_KEYCODE => {
            if console_show != 0 {
                if (*param).value == BRAILLE_KEY {
                    console_show = 0; beep(880); vc_maybe_cursor_moved(vc); vc_refresh(vc); ret = NOTIFY_STOP;
                }
            } else {
                ret = NOTIFY_STOP;
                match (*param).value {
                    KEY_INSERT => { beep(440); console_show = 1; lastVC = -1; braille_write(console_buf.as_mut_ptr()); }
                    KEY_LEFT => { if vc_x > 0 { vc_x -= WIDTH as i32; if vc_x < 0 { vc_x = 0; } } else if vc_y >= 1 { beep(880); vc_y -= 1; vc_x = (*vc).vc_cols - WIDTH as i32; } else { beep(220); } }
                    KEY_RIGHT => { if vc_x + WIDTH as i32 < (*vc).vc_cols { vc_x += WIDTH as i32; } else if vc_y + 1 < (*vc).vc_rows { beep(880); vc_y += 1; vc_x = 0; } else { beep(220); } }
                    KEY_DOWN => { if vc_y + 1 < (*vc).vc_rows { vc_y += 1; } else { beep(220); } }
                    KEY_UP => { if vc_y >= 1 { vc_y -= 1; } else { beep(220); } }
                    KEY_HOME => vc_follow_cursor(vc),
                    KEY_PAGEUP => { vc_x = 0; vc_y = 0; }
                    KEY_PAGEDOWN => { vc_x = 0; vc_y = (*vc).vc_rows - 1; }
                    _ => ret = NOTIFY_OK,
                }
                if ret == NOTIFY_STOP { vc_refresh(vc); }
            }
        }
        KBD_POST_KEYSYM => {
            let typ = KTYP((*param).value) - 0xf0;
            if typ == KT_SPEC {
                let val = KVAL((*param).value);
                let mut on_off = -1;
                match val { x if x == KVAL(K_CAPS) => on_off = vt_get_leds(fg_console, VC_CAPSLOCK), x if x == KVAL(K_NUM) => on_off = vt_get_leds(fg_console, VC_NUMLOCK), x if x == KVAL(K_HOLD) => on_off = vt_get_leds(fg_console, VC_SCROLLOCK), _ => {} }
                if on_off == 1 { beep(880); } else if on_off == 0 { beep(440); }
            }
        }
        KBD_UNBOUND_KEYCODE | KBD_UNICODE | KBD_KEYSYM => { /* Unused */ }
        _ => {}
    }
    ret
}

static mut keyboard_notifier_block: notifier_block = notifier_block { notifier_call: Some(keyboard_notifier_call) };

unsafe extern "C" fn vt_notifier_call(_blk: *mut notifier_block, code: c_ulong, _param: *mut c_void) -> i32 {
    let param = _param as *mut vt_notifier_param;
    let vc = (*param).vc;
    match code {
        VT_ALLOCATE | VT_DEALLOCATE => {}
        VT_WRITE => {
            let mut c = (*param).c;
            if (*vc).vc_num == fg_console {
                match c {
                    b'\x08' | 127 => { if console_cursor > 0 { console_cursor -= 1; console_buf[console_cursor as usize] = b' ' as u16; } }
                    b'\n' | b'\x0b' | b'\x0c' | b'\r' => console_newline = 1,
                    b'\t' => { c = b' '; }
                    _ => {}
                }
                if c >= 32 && c != b'\n' && c != b'\x0b' && c != b'\x0c' && c != b'\r' {
                    if console_newline != 0 { console_buf.fill(0); console_cursor = 0; console_newline = 0; }
                    if console_cursor == WIDTH as i32 { console_buf.copy_within(1..WIDTH, 0); } else { console_cursor += 1; }
                    console_buf[(console_cursor - 1) as usize] = c as u16;
                }
                if console_show != 0 { braille_write(console_buf.as_mut_ptr()); } else { vc_maybe_cursor_moved(vc); vc_refresh(vc); }
            }
        }
        VT_UPDATE => { if console_show != 0 { if (*vc).vc_num != lastVC { lastVC = (*vc).vc_num; console_buf.fill(0); console_cursor = 0; braille_write(console_buf.as_mut_ptr()); } } else { vc_maybe_cursor_moved(vc); vc_refresh(vc); } }
        _ => {}
    }
    NOTIFY_OK
}

static mut vt_notifier_block: notifier_block = notifier_block { notifier_call: Some(vt_notifier_call) };

/* Called from printk.c when console=brl is given */
pub unsafe fn braille_register_console(console: *mut console, index: i32, console_options: *mut c_char, _braille_options: *mut c_char) -> i32 {
    let options = if console_options.is_null() { b"57600o8\0".as_ptr() as *mut c_char } else { console_options };
    if !braille_co.is_null() { return -ENODEV; }
    if let Some(setup) = (*console).setup { let ret = setup(console, options); if ret != 0 { return ret; } }
    (*console).flags |= CON_ENABLED; (*console).index = index; braille_co = console;
    register_keyboard_notifier(&raw mut keyboard_notifier_block); register_vt_notifier(&raw mut vt_notifier_block); 1
}

pub unsafe fn braille_unregister_console(console: *mut console) -> i32 {
    if braille_co != console { return -EINVAL; }
    unregister_keyboard_notifier(&raw mut keyboard_notifier_block); unregister_vt_notifier(&raw mut vt_notifier_block); braille_co = core::ptr::null_mut(); 1
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
