// SPDX-License-Identifier: GPL-2.0-or-later
// External Linux/kernel and charlcd dependencies are supplied by other files.

/* LCD commands */
const LCD_CMD_DISPLAY_CLEAR: i32 = 0x01;
const LCD_CMD_ENTRY_MODE: i32 = 0x04;
const LCD_CMD_CURSOR_INC: i32 = 0x02;
const LCD_CMD_DISPLAY_CTRL: i32 = 0x08;
const LCD_CMD_DISPLAY_ON: i32 = 0x04;
const LCD_CMD_CURSOR_ON: i32 = 0x02;
const LCD_CMD_BLINK_ON: i32 = 0x01;
const LCD_CMD_SHIFT: i32 = 0x10;
const LCD_CMD_DISPLAY_SHIFT: i32 = 0x08;
const LCD_CMD_SHIFT_RIGHT: i32 = 0x04;
const LCD_CMD_FUNCTION_SET: i32 = 0x20;
const LCD_CMD_DATA_LEN_8BITS: i32 = 0x10;
const LCD_CMD_TWO_LINES: i32 = 0x08;
const LCD_CMD_FONT_5X10_DOTS: i32 = 0x04;
const LCD_CMD_SET_CGRAM_ADDR: i32 = 0x40;
const LCD_CMD_SET_DDRAM_ADDR: i32 = 0x80;

unsafe extern "C" {
    fn schedule_timeout_interruptible(timeout: i64) -> i64;
    fn msecs_to_jiffies(ms: i32) -> i64;
    fn charlcd_backlight(lcd: *mut charlcd, on: i32);
    fn charlcd_alloc(size: usize) -> *mut charlcd;
    fn charlcd_free(lcd: *mut charlcd);
    fn strchr(s: *const i8, c: i32) -> *mut i8;
    fn hex_to_bin(c: i8) -> i32;
}

unsafe fn long_sleep(ms: i32) {
    schedule_timeout_interruptible(msecs_to_jiffies(ms));
}

pub unsafe fn hd44780_common_print(lcd: *mut charlcd, c: i32) -> i32 {
    let hdc = (*lcd).drvdata;
    if (*lcd).addr.x < (*hdc).bwidth {
        ((*hdc).write_data)(hdc, c);
        return 0;
    }
    1
}

pub unsafe fn hd44780_common_gotoxy(lcd: *mut charlcd, x: u32, y: u32) -> i32 {
    let hdc = (*lcd).drvdata;
    let mut addr = if x < (*hdc).bwidth { x & ((*hdc).hwidth - 1) } else { (*hdc).bwidth - 1 };
    if y & 1 != 0 { addr += (*hdc).hwidth; }
    if y & 2 != 0 { addr += (*hdc).bwidth; }
    ((*hdc).write_cmd)(hdc, LCD_CMD_SET_DDRAM_ADDR | addr as i32);
    0
}

pub unsafe fn hd44780_common_home(lcd: *mut charlcd) -> i32 {
    hd44780_common_gotoxy(lcd, 0, 0)
}

/* clears the display and resets X/Y */
pub unsafe fn hd44780_common_clear_display(lcd: *mut charlcd) -> i32 {
    let hdc = (*lcd).drvdata;
    ((*hdc).write_cmd)(hdc, LCD_CMD_DISPLAY_CLEAR);
    /* datasheet says to wait 1,64 milliseconds */
    long_sleep(2);
    /* Other controllers do not reset DDRAM on DISPLAY_CLEAR, so home unconditionally. */
    hd44780_common_home(lcd)
}

pub unsafe fn hd44780_common_init_display(lcd: *mut charlcd) -> i32 {
    let hdc = (*lcd).drvdata;
    if (*hdc).ifwidth != 4 && (*hdc).ifwidth != 8 { return -22; }
    (*hdc).hd44780_common_flags = (if (*lcd).height > 1 { LCD_FLAG_N } else { 0 }) |
        LCD_FLAG_D | LCD_FLAG_C | LCD_FLAG_B;
    long_sleep(20);
    let mut init = LCD_CMD_FUNCTION_SET | LCD_CMD_DATA_LEN_8BITS;
    let write_cmd_raw: unsafe extern "C" fn(*mut hd44780_common, i32);
    if (*hdc).ifwidth == 4 {
        init >>= 4;
        write_cmd_raw = (*hdc).write_cmd_raw4;
    } else { write_cmd_raw = (*hdc).write_cmd; }
    write_cmd_raw(hdc, init); long_sleep(10);
    write_cmd_raw(hdc, init); long_sleep(10);
    write_cmd_raw(hdc, init); long_sleep(10);
    if (*hdc).ifwidth == 4 {
        ((*hdc).write_cmd_raw4)(hdc, LCD_CMD_FUNCTION_SET >> 4); long_sleep(10);
    }
    ((*hdc).write_cmd)(hdc, LCD_CMD_FUNCTION_SET |
        (if (*hdc).ifwidth == 8 { LCD_CMD_DATA_LEN_8BITS } else { 0 }) |
        (if (*hdc).hd44780_common_flags & LCD_FLAG_F != 0 { LCD_CMD_FONT_5X10_DOTS } else { 0 }) |
        (if (*hdc).hd44780_common_flags & LCD_FLAG_N != 0 { LCD_CMD_TWO_LINES } else { 0 }));
    long_sleep(10);
    ((*hdc).write_cmd)(hdc, LCD_CMD_DISPLAY_CTRL); long_sleep(10);
    ((*hdc).write_cmd)(hdc, LCD_CMD_DISPLAY_CTRL |
        (if (*hdc).hd44780_common_flags & LCD_FLAG_D != 0 { LCD_CMD_DISPLAY_ON } else { 0 }) |
        (if (*hdc).hd44780_common_flags & LCD_FLAG_C != 0 { LCD_CMD_CURSOR_ON } else { 0 }) |
        (if (*hdc).hd44780_common_flags & LCD_FLAG_B != 0 { LCD_CMD_BLINK_ON } else { 0 }));
    charlcd_backlight(lcd, if (*hdc).hd44780_common_flags & LCD_FLAG_L != 0 { 1 } else { 0 });
    long_sleep(10);
    ((*hdc).write_cmd)(hdc, LCD_CMD_ENTRY_MODE | LCD_CMD_CURSOR_INC);
    hd44780_common_clear_display(lcd); 0
}

pub unsafe fn hd44780_common_shift_cursor(lcd: *mut charlcd, dir: charlcd_shift_dir) -> i32 {
    let hdc = (*lcd).drvdata;
    if dir == CHARLCD_SHIFT_LEFT {
        if (*lcd).addr.x < (*hdc).bwidth { ((*hdc).write_cmd)(hdc, LCD_CMD_SHIFT); }
    } else if dir == CHARLCD_SHIFT_RIGHT && (*lcd).addr.x < (*hdc).bwidth - 1 {
        ((*hdc).write_cmd)(hdc, LCD_CMD_SHIFT | LCD_CMD_SHIFT_RIGHT);
    }
    0
}

pub unsafe fn hd44780_common_shift_display(lcd: *mut charlcd, dir: charlcd_shift_dir) -> i32 {
    let hdc = (*lcd).drvdata;
    if dir == CHARLCD_SHIFT_LEFT { ((*hdc).write_cmd)(hdc, LCD_CMD_SHIFT | LCD_CMD_DISPLAY_SHIFT); }
    else if dir == CHARLCD_SHIFT_RIGHT { ((*hdc).write_cmd)(hdc, LCD_CMD_SHIFT | LCD_CMD_DISPLAY_SHIFT | LCD_CMD_SHIFT_RIGHT); }
    0
}

unsafe fn hd44780_common_set_mode(hdc: *mut hd44780_common) {
    ((*hdc).write_cmd)(hdc, LCD_CMD_DISPLAY_CTRL |
        (if (*hdc).hd44780_common_flags & LCD_FLAG_D != 0 { LCD_CMD_DISPLAY_ON } else { 0 }) |
        (if (*hdc).hd44780_common_flags & LCD_FLAG_C != 0 { LCD_CMD_CURSOR_ON } else { 0 }) |
        (if (*hdc).hd44780_common_flags & LCD_FLAG_B != 0 { LCD_CMD_BLINK_ON } else { 0 }));
}

pub unsafe fn hd44780_common_display(lcd: *mut charlcd, on: charlcd_onoff) -> i32 { let h=(*lcd).drvdata; if on==CHARLCD_ON {(*h).hd44780_common_flags|=LCD_FLAG_D;} else {(*h).hd44780_common_flags&=!LCD_FLAG_D;} hd44780_common_set_mode(h); 0 }
pub unsafe fn hd44780_common_cursor(lcd: *mut charlcd, on: charlcd_onoff) -> i32 { let h=(*lcd).drvdata; if on==CHARLCD_ON {(*h).hd44780_common_flags|=LCD_FLAG_C;} else {(*h).hd44780_common_flags&=!LCD_FLAG_C;} hd44780_common_set_mode(h); 0 }
pub unsafe fn hd44780_common_blink(lcd: *mut charlcd, on: charlcd_onoff) -> i32 { let h=(*lcd).drvdata; if on==CHARLCD_ON {(*h).hd44780_common_flags|=LCD_FLAG_B;} else {(*h).hd44780_common_flags&=!LCD_FLAG_B;} hd44780_common_set_mode(h); 0 }

unsafe fn hd44780_common_set_function(hdc: *mut hd44780_common) {
    ((*hdc).write_cmd)(hdc, LCD_CMD_FUNCTION_SET | (if (*hdc).ifwidth==8 {LCD_CMD_DATA_LEN_8BITS} else {0}) | (if (*hdc).hd44780_common_flags&LCD_FLAG_F!=0 {LCD_CMD_FONT_5X10_DOTS}else{0}) | (if (*hdc).hd44780_common_flags&LCD_FLAG_N!=0 {LCD_CMD_TWO_LINES}else{0}));
}
pub unsafe fn hd44780_common_fontsize(lcd:*mut charlcd,size:charlcd_fontsize)->i32{let h=(*lcd).drvdata;if size==CHARLCD_FONTSIZE_LARGE{(*h).hd44780_common_flags|=LCD_FLAG_F}else{(*h).hd44780_common_flags&=!LCD_FLAG_F}hd44780_common_set_function(h);0}
pub unsafe fn hd44780_common_lines(lcd:*mut charlcd,lines:charlcd_lines)->i32{let h=(*lcd).drvdata;if lines==CHARLCD_LINES_2{(*h).hd44780_common_flags|=LCD_FLAG_N}else{(*h).hd44780_common_flags&=!LCD_FLAG_N}hd44780_common_set_function(h);0}

pub unsafe fn hd44780_common_redefine_char(lcd:*mut charlcd,esc:*mut i8)->i32{
    let h=(*lcd).drvdata; if strchr(esc, ';' as i32).is_null(){return 0;} esc=esc.add(1); let cgaddr=(*esc as u8 - b'0') as u8; esc=esc.add(1); if cgaddr>7{return 1;}
    let mut cgbytes=[0u8;8]; let mut cgoffset=0usize; let mut shift=0i32; let mut value=0i32;
    while *esc!=0 && cgoffset<8 { let mut half; shift^=4; half=hex_to_bin(*esc); esc=esc.add(1); if half<0{continue;} value|=half<<shift; if shift==0{cgbytes[cgoffset]=value as u8;cgoffset+=1;value=0;} }
    ((*h).write_cmd)(h,LCD_CMD_SET_CGRAM_ADDR|(cgaddr as i32*8)); for addr in 0..cgoffset{((*h).write_data)(h,cgbytes[addr] as i32);} ((*lcd).ops).gotoxy(lcd,(*lcd).addr.x,(*lcd).addr.y);1
}

pub unsafe fn hd44780_common_alloc()->*mut charlcd{let lcd=charlcd_alloc(core::mem::size_of::<hd44780_common>());if lcd.is_null(){return core::ptr::null_mut();}let h=(*lcd).drvdata;(*h).ifwidth=8;(*h).bwidth=DEFAULT_LCD_BWIDTH;(*h).hwidth=DEFAULT_LCD_HWIDTH;lcd}
pub unsafe fn hd44780_common_free(lcd:*mut charlcd){charlcd_free(lcd);}

// EXPORT_SYMBOL_GPL declarations and module metadata are provided by the build system.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
