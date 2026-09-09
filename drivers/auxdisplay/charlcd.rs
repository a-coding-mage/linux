// SPDX-License-Identifier: GPL-2.0+
/*
 * Character LCD driver for Linux
 *
 * Copyright (C) 2000-2008, Willy Tarreau <w@1wt.eu>
 * Copyright (C) 2016-2017 Glider bvba
 */

// Linux kernel dependencies and "charlcd.h" are supplied by the surrounding build.

const LCD_BL_TEMPO_PERIOD: u64 = 4;
const LCD_ESCAPE_LEN: usize = 24;
const LCD_ESCAPE_CHAR: u8 = 27;

#[repr(C)]
struct CharlcdPriv {
    lcd: Charlcd,
    bl_work: DelayedWork,
    bl_tempo_lock: Mutex,
    bl_tempo: bool,
    must_clear: bool,
    flags: c_ulong,
    esc_seq: EscSeq,
    drvdata: [u64; 0],
}

#[repr(C)]
struct EscSeq {
    buf: [c_char; LCD_ESCAPE_LEN + 1],
    len: c_int,
}

static mut CHARLCD_AVAILABLE: Atomic = Atomic::new(1);
static mut THE_CHARLCD: *mut Charlcd = core::ptr::null_mut();

unsafe fn charlcd_to_priv(p: *mut Charlcd) -> *mut CharlcdPriv {
    (p as *mut u8).sub(core::mem::offset_of!(CharlcdPriv, lcd)) as *mut CharlcdPriv
}

pub unsafe extern "C" fn charlcd_backlight(lcd: *mut Charlcd, on: CharLCDOnOff) {
    let priv_ = charlcd_to_priv(lcd);
    if (*(*lcd).ops).backlight.is_none() { return; }
    mutex_lock(&mut (*priv_).bl_tempo_lock);
    if !(*priv_).bl_tempo { ((*(*lcd).ops).backlight.unwrap())(lcd, on); }
    mutex_unlock(&mut (*priv_).bl_tempo_lock);
}

unsafe extern "C" fn charlcd_bl_off(work: *mut WorkStruct) {
    let dwork = to_delayed_work(work);
    let priv_ = container_of_delayed(dwork, core::mem::offset_of!(CharlcdPriv, bl_work));
    mutex_lock(&mut (*priv_).bl_tempo_lock);
    if (*priv_).bl_tempo {
        (*priv_).bl_tempo = false;
        if (*priv_).flags & LCD_FLAG_L == 0 {
            ((*(*(*priv_).lcd.ops).backlight.unwrap())(&mut (*priv_).lcd, CHARLCD_OFF));
        }
    }
    mutex_unlock(&mut (*priv_).bl_tempo_lock);
}

pub unsafe extern "C" fn charlcd_poke(lcd: *mut Charlcd) {
    let priv_ = charlcd_to_priv(lcd);
    if (*(*lcd).ops).backlight.is_none() { return; }
    cancel_delayed_work_sync(&mut (*priv_).bl_work);
    mutex_lock(&mut (*priv_).bl_tempo_lock);
    if !(*priv_).bl_tempo && (*priv_).flags & LCD_FLAG_L == 0 {
        ((*(*lcd).ops).backlight.unwrap())(lcd, CHARLCD_ON);
    }
    (*priv_).bl_tempo = true;
    schedule_delayed_work(&mut (*priv_).bl_work, LCD_BL_TEMPO_PERIOD * HZ);
    mutex_unlock(&mut (*priv_).bl_tempo_lock);
}

unsafe fn charlcd_home(lcd: *mut Charlcd) {
    (*lcd).addr.x = 0; (*lcd).addr.y = 0; ((*(*lcd).ops).home)(lcd);
}

unsafe fn charlcd_print(lcd: *mut Charlcd, mut c: c_char) {
    if (*lcd).addr.x >= (*lcd).width { return; }
    if !(*lcd).char_conv.is_null() { c = *(*lcd).char_conv.add(c as u8 as usize); }
    if !((*(*lcd).ops).print)(lcd, c) { (*lcd).addr.x += 1; }
    if (*lcd).addr.x == (*lcd).width { ((*(*lcd).ops).gotoxy)(lcd, (*lcd).addr.x - 1, (*lcd).addr.y); }
}

unsafe fn charlcd_clear_display(lcd: *mut Charlcd) {
    ((*(*lcd).ops).clear_display)(lcd); (*lcd).addr.x = 0; (*lcd).addr.y = 0;
}

unsafe fn parse_xy(mut s: *const c_char, x: *mut c_ulong, y: *mut c_ulong) -> bool {
    let mut new_x = *x; let mut new_y = *y;
    loop {
        if *s == 0 { return false; }
        if *s == b';' as c_char { break; }
        if *s == b'x' as c_char { let mut p = core::ptr::null_mut(); new_x = simple_strtoul(s.add(1), &mut p, 10); if p == s.add(1) { return false; } s = p; }
        else if *s == b'y' as c_char { let mut p = core::ptr::null_mut(); new_y = simple_strtoul(s.add(1), &mut p, 10); if p == s.add(1) { return false; } s = p; }
        else { return false; }
    }
    *x = new_x; *y = new_y; true
}

unsafe fn handle_lcd_special_code(lcd: *mut Charlcd) -> c_int {
    let priv_ = charlcd_to_priv(lcd); let esc = (*priv_).esc_seq.buf.as_mut_ptr().add(2); let oldflags = (*priv_).flags; let mut processed = 0;
    match *esc as u8 {
        b'D' => { (*priv_).flags |= LCD_FLAG_D; if (*priv_).flags != oldflags { ((*(*lcd).ops).display)(lcd, CHARLCD_ON); } processed = 1; }
        b'd' => { (*priv_).flags &= !LCD_FLAG_D; if (*priv_).flags != oldflags { ((*(*lcd).ops).display)(lcd, CHARLCD_OFF); } processed = 1; }
        b'C' => { (*priv_).flags |= LCD_FLAG_C; if (*priv_).flags != oldflags { ((*(*lcd).ops).cursor)(lcd, CHARLCD_ON); } processed = 1; }
        b'c' => { (*priv_).flags &= !LCD_FLAG_C; if (*priv_).flags != oldflags { ((*(*lcd).ops).cursor)(lcd, CHARLCD_OFF); } processed = 1; }
        b'B' => { (*priv_).flags |= LCD_FLAG_B; if (*priv_).flags != oldflags { ((*(*lcd).ops).blink)(lcd, CHARLCD_ON); } processed = 1; }
        b'b' => { (*priv_).flags &= !LCD_FLAG_B; if (*priv_).flags != oldflags { ((*(*lcd).ops).blink)(lcd, CHARLCD_OFF); } processed = 1; }
        b'+' => { (*priv_).flags |= LCD_FLAG_L; if (*priv_).flags != oldflags { charlcd_backlight(lcd, CHARLCD_ON); } processed = 1; }
        b'-' => { (*priv_).flags &= !LCD_FLAG_L; if (*priv_).flags != oldflags { charlcd_backlight(lcd, CHARLCD_OFF); } processed = 1; }
        b'*' => { charlcd_poke(lcd); processed = 1; }
        b'f' => { (*priv_).flags &= !LCD_FLAG_F; if (*priv_).flags != oldflags { ((*(*lcd).ops).fontsize)(lcd, CHARLCD_FONTSIZE_SMALL); } processed = 1; }
        b'F' => { (*priv_).flags |= LCD_FLAG_F; if (*priv_).flags != oldflags { ((*(*lcd).ops).fontsize)(lcd, CHARLCD_FONTSIZE_LARGE); } processed = 1; }
        b'n' => { (*priv_).flags &= !LCD_FLAG_N; if (*priv_).flags != oldflags { ((*(*lcd).ops).lines)(lcd, CHARLCD_LINES_1); } processed = 1; }
        b'N' => { (*priv_).flags |= LCD_FLAG_N; if (*priv_).flags != oldflags { ((*(*lcd).ops).lines)(lcd, CHARLCD_LINES_2); } processed = 1; }
        b'l' => { if (*lcd).addr.x > 0 { if !((*(*lcd).ops).shift_cursor)(lcd, CHARLCD_SHIFT_LEFT) { (*lcd).addr.x -= 1; } } processed = 1; }
        b'r' => { if (*lcd).addr.x < (*lcd).width { if !((*(*lcd).ops).shift_cursor)(lcd, CHARLCD_SHIFT_RIGHT) { (*lcd).addr.x += 1; } } processed = 1; }
        b'L' => { ((*(*lcd).ops).shift_display)(lcd, CHARLCD_SHIFT_LEFT); processed = 1; }
        b'R' => { ((*(*lcd).ops).shift_display)(lcd, CHARLCD_SHIFT_RIGHT); processed = 1; }
        b'k' => { let xs = (*lcd).addr.x; let ys = (*lcd).addr.y; let mut x = (*lcd).addr.x; while x < (*lcd).width { ((*(*lcd).ops).print)(lcd, b' ' as c_char); x += 1; } (*lcd).addr.x = xs; (*lcd).addr.y = ys; ((*(*lcd).ops).gotoxy)(lcd, (*lcd).addr.x, (*lcd).addr.y); processed = 1; }
        b'I' => { ((*(*lcd).ops).init_display)(lcd); (*priv_).flags = if (*lcd).height > 1 { LCD_FLAG_N } else { 0 } | LCD_FLAG_D | LCD_FLAG_C | LCD_FLAG_B; processed = 1; }
        b'G' => { processed = if let Some(f) = (*(*lcd).ops).redefine_char { f(lcd, esc) } else { 1 }; }
        b'x' | b'y' => { if (*priv_).esc_seq.buf[(*priv_).esc_seq.len as usize - 1] as u8 == b';' && parse_xy(esc, &mut (*lcd).addr.x, &mut (*lcd).addr.y) { ((*(*lcd).ops).gotoxy)(lcd, (*lcd).addr.x, (*lcd).addr.y); } processed = 1; }
        _ => {}
    } processed
}

unsafe fn charlcd_write_char(lcd: *mut Charlcd, c: c_char) {
    let priv_ = charlcd_to_priv(lcd);
    if c as u8 != b'\n' && (*priv_).esc_seq.len >= 0 { let n = (*priv_).esc_seq.len as usize; (*priv_).esc_seq.buf[n] = c; (*priv_).esc_seq.len += 1; (*priv_).esc_seq.buf[n + 1] = 0; }
    else { (*priv_).esc_seq.len = -1; match c as u8 { LCD_ESCAPE_CHAR => { (*priv_).esc_seq.len = 0; (*priv_).esc_seq.buf[0] = 0; }, 8 => { if (*lcd).addr.x > 0 && !((*(*lcd).ops).shift_cursor)(lcd, CHARLCD_SHIFT_LEFT) { (*lcd).addr.x -= 1; } charlcd_print(lcd, b' ' as c_char); if !((*(*lcd).ops).shift_cursor)(lcd, CHARLCD_SHIFT_LEFT) { (*lcd).addr.x -= 1; } }, 12 => charlcd_clear_display(lcd), b'\n' => { while (*lcd).addr.x < (*lcd).width { ((*(*lcd).ops).print)(lcd, b' ' as c_char); (*lcd).addr.x += 1; } (*lcd).addr.x = 0; (*lcd).addr.y = ((*lcd).addr.y + 1) % (*lcd).height; ((*(*lcd).ops).gotoxy)(lcd, 0, (*lcd).addr.y); }, b'\r' => { (*lcd).addr.x = 0; ((*(*lcd).ops).gotoxy)(lcd, 0, (*lcd).addr.y); }, b'\t' => charlcd_print(lcd, b' ' as c_char), _ => charlcd_print(lcd, c) } }
    if (*priv_).esc_seq.len >= 2 { let s = (*priv_).esc_seq.buf.as_ptr(); let mut processed = 0; if c_string_eq(s, b"[2J\0") { charlcd_clear_display(lcd); processed = 1; } else if c_string_eq(s, b"[H\0") { charlcd_home(lcd); processed = 1; } else if (*priv_).esc_seq.len >= 3 && *s as u8 == b'[' && *s.add(1) as u8 == b'L' { processed = handle_lcd_special_code(lcd); } if processed != 0 || (*priv_).esc_seq.len as usize >= LCD_ESCAPE_LEN { (*priv_).esc_seq.len = -1; } }
}

static mut CHARLCD_FOPS: FileOperations = FileOperations { write: Some(charlcd_write), open: Some(charlcd_open), release: Some(charlcd_release) };
static mut CHARLCD_DEV: MiscDevice = MiscDevice { minor: LCD_MINOR, name: b"lcd\0".as_ptr() as *const c_char, fops: &CHARLCD_FOPS };

unsafe extern "C" fn charlcd_write(_file: *mut File, buf: *const c_char, mut count: usize, ppos: *mut LoFF) -> isize { let mut tmp = buf; while count > 0 { if ((count + 1) & 0x1f) == 0 { cond_resched(); } let mut c = 0; if get_user(&mut c, tmp) != 0 { return -EFAULT as isize; } charlcd_write_char(THE_CHARLCD, c); (*ppos).0 += 1; tmp = tmp.add(1); count -= 1; } tmp.offset_from(buf) as isize }
unsafe extern "C" fn charlcd_open(inode: *mut Inode, file: *mut File) -> c_int { let priv_ = charlcd_to_priv(THE_CHARLCD); let mut ret = -EBUSY; if !atomic_dec_and_test(&mut CHARLCD_AVAILABLE) { return ret; } ret = -EPERM; if (*file).f_mode & FMODE_READ != 0 { atomic_inc(&mut CHARLCD_AVAILABLE); return ret; } if (*priv_).must_clear { ((*(*THE_CHARLCD).ops).clear_display)(THE_CHARLCD); (*priv_).must_clear = false; (*THE_CHARLCD).addr.x = 0; (*priv_).lcd.addr.y = 0; } nonseekable_open(inode, file) }
unsafe extern "C" fn charlcd_release(_inode: *mut Inode, _file: *mut File) -> c_int { atomic_inc(&mut CHARLCD_AVAILABLE); 0 }

unsafe fn charlcd_puts(lcd: *mut Charlcd, mut s: *const c_char) { let mut count = c_strlen(s); while count > 0 { if ((count + 1) & 0x1f) == 0 { cond_resched(); } charlcd_write_char(lcd, *s); s = s.add(1); count -= 1; } }

unsafe fn charlcd_init(lcd: *mut Charlcd) -> c_int { let priv_ = charlcd_to_priv(lcd); (*priv_).flags = if (*lcd).height > 1 { LCD_FLAG_N } else { 0 } | LCD_FLAG_D | LCD_FLAG_C | LCD_FLAG_B; if (*(*lcd).ops).backlight.is_some() { mutex_init(&mut (*priv_).bl_tempo_lock); init_delayed_work(&mut (*priv_).bl_work, charlcd_bl_off); } if (*(*lcd).ops).init_display.is_none() { return -EINVAL; } let ret = ((*(*lcd).ops).init_display.unwrap())(lcd); if ret != 0 { return ret; } charlcd_puts(lcd, b"\x1b[Lc\x1b[Lb\x1b[L-\0".as_ptr() as *const c_char); (*priv_).must_clear = true; charlcd_home(lcd); 0 }
unsafe fn charlcd_deinit(lcd: *mut Charlcd) { let priv_ = charlcd_to_priv(lcd); if (*(*lcd).ops).backlight.is_some() { cancel_delayed_work_sync(&mut (*priv_).bl_work); ((*(*lcd).ops).backlight.unwrap())(lcd, CHARLCD_OFF); } }
pub unsafe extern "C" fn charlcd_alloc(_drvdata_size: usize) -> *mut Charlcd { let priv_ = kzalloc(core::mem::size_of::<CharlcdPriv>(), GFP_KERNEL) as *mut CharlcdPriv; if priv_.is_null() { return core::ptr::null_mut(); } (*priv_).esc_seq.len = -1; &mut (*priv_).lcd }
pub unsafe extern "C" fn charlcd_free(lcd: *mut Charlcd) { kfree(charlcd_to_priv(lcd) as *mut c_void); }
unsafe extern "C" fn panel_notify_sys(_this: *mut NotifierBlock, code: c_ulong, _unused: *mut c_void) -> c_int { let lcd = THE_CHARLCD; match code { SYS_DOWN => charlcd_puts(lcd, b"\x0cReloading\nSystem...\x1b[Lc\x1b[Lb\x1b[L+\0".as_ptr() as *const c_char), SYS_HALT => charlcd_puts(lcd, b"\x0cSystem Halted.\x1b[Lc\x1b[Lb\x1b[L+\0".as_ptr() as *const c_char), SYS_POWER_OFF => charlcd_puts(lcd, b"\x0cPower off.\x1b[Lc\x1b[Lb\x1b[L+\0".as_ptr() as *const c_char), _ => {} } NOTIFY_DONE }
pub unsafe extern "C" fn charlcd_register(lcd: *mut Charlcd) -> c_int { let ret = charlcd_init(lcd); if ret != 0 { return ret; } let ret = misc_register(&mut CHARLCD_DEV); if ret != 0 { charlcd_deinit(lcd); return ret; } THE_CHARLCD = lcd; register_reboot_notifier(&mut PANEL_NOTIFIER); 0 }
pub unsafe extern "C" fn charlcd_unregister(lcd: *mut Charlcd) -> c_int { unregister_reboot_notifier(&mut PANEL_NOTIFIER); charlcd_puts(lcd, b"\x0cLCD driver unloaded.\x1b[Lc\x1b[Lb\x1b[L-\0".as_ptr() as *const c_char); misc_deregister(&mut CHARLCD_DEV); THE_CHARLCD = core::ptr::null_mut(); charlcd_deinit(lcd); 0 }

static mut PANEL_NOTIFIER: NotifierBlock = NotifierBlock { notifier_call: Some(panel_notify_sys) };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
