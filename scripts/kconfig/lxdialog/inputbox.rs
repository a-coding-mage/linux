// SPDX-License-Identifier: GPL-2.0+
/*
 *  inputbox.c -- implements the input box
 *
 *  ORIGINAL AUTHOR: Savio Lam (lam836@cs.cuhk.hk)
 *  MODIFIED FOR LINUX KERNEL CONFIG BY: William Roadcap (roadcap@cfw.com)
 */

use core::ffi::c_char;

// Dependency supplied by dialog.h and the curses implementation.
#[repr(C)]
pub struct WINDOW {
    _private: [u8; 0],
}

extern "C" {
    static mut dialog_input_result: [c_char; MAX_LEN as usize + 1];
    static mut stdscr: *mut WINDOW;
    static mut dlg: Dlg;

    fn getmaxy(window: *mut WINDOW) -> i32;
    fn getmaxx(window: *mut WINDOW) -> i32;
    fn draw_shadow(window: *mut WINDOW, y: i32, x: i32, height: i32, width: i32);
    fn newwin(height: i32, width: i32, y: i32, x: i32) -> *mut WINDOW;
    fn keypad(window: *mut WINDOW, enabled: i32) -> i32;
    fn draw_box(window: *mut WINDOW, y: i32, x: i32, height: i32, width: i32, dialog_atr: i32, border_atr: i32);
    fn wattrset(window: *mut WINDOW, attrs: i32) -> i32;
    fn mvwaddch(window: *mut WINDOW, y: i32, x: i32, ch: u32) -> i32;
    fn waddch(window: *mut WINDOW, ch: u32) -> i32;
    fn print_title(window: *mut WINDOW, title: *const c_char, width: i32);
    fn print_autowrap(window: *mut WINDOW, prompt: *const c_char, width: i32, y: i32, x: i32);
    fn getyx(window: *mut WINDOW, y: *mut i32, x: *mut i32);
    fn wmove(window: *mut WINDOW, y: i32, x: i32) -> i32;
    fn wrefresh(window: *mut WINDOW) -> i32;
    fn waddstr(window: *mut WINDOW, string: *const c_char) -> i32;
    fn wgetch(window: *mut WINDOW) -> i32;
    fn isprint(ch: i32) -> i32;
    fn flash() -> i32;
    fn delwin(window: *mut WINDOW) -> i32;
    fn on_key_esc(window: *mut WINDOW) -> i32;
    fn on_key_resize();
    fn print_button(window: *mut WINDOW, label: *const c_char, y: i32, x: i32, selected: i32);
}

#[repr(C)]
pub struct DlgAttr {
    pub atr: i32,
}

#[repr(C)]
pub struct Dlg {
    pub dialog: DlgAttr,
    pub border: DlgAttr,
    pub inputbox: DlgAttr,
}

extern "C" {
    static MAX_LEN: i32;
    static INPUTBOX_HEIGHT_MIN: i32;
    static INPUTBOX_WIDTH_MIN: i32;
    static ERRDISPLAYTOOSMALL: i32;
    static KEY_ESC: i32;
    static TAB: i32;
    static KEY_UP: i32;
    static KEY_DOWN: i32;
    static KEY_BACKSPACE: i32;
    static KEY_LEFT: i32;
    static KEY_RIGHT: i32;
    static KEY_RESIZE: i32;
    static TRUE: i32;
    static ACS_LTEE: u32;
    static ACS_HLINE: u32;
    static ACS_RTEE: u32;
}

unsafe fn print_buttons(dialog: *mut WINDOW, height: i32, width: i32, selected: i32) {
    let x = width / 2 - 11;
    let y = height - 2;
    print_button(dialog, b"  Ok  \0".as_ptr() as *const c_char, y, x, (selected == 0) as i32);
    print_button(dialog, b" Help \0".as_ptr() as *const c_char, y, x + 14, (selected == 1) as i32);
    wmove(dialog, y, x + 1 + 14 * selected);
    wrefresh(dialog);
}

pub unsafe fn dialog_inputbox(title: *const c_char, prompt: *const c_char, height: i32, width: i32, init: *const c_char) -> i32 {
    let mut i: i32;
    let mut x: i32;
    let mut y: i32;
    let mut box_y: i32;
    let mut box_x: i32;
    let mut box_width: i32;
    let mut input_x: i32 = 0;
    let mut key: i32 = 0;
    let mut button: i32 = -1;
    let mut show_x: i32;
    let mut len: i32;
    let mut pos: i32;
    let instr = dialog_input_result.as_mut_ptr();
    let mut dialog: *mut WINDOW;

    if init.is_null() {
        *instr = 0;
    } else {
        let mut n = 0usize;
        while n < (MAX_LEN as usize) && *init.add(n) != 0 {
            *instr.add(n) = *init.add(n);
            n += 1;
        }
        *instr.add(n) = 0;
    }

    'do_resize: loop {
        if getmaxy(stdscr) <= height - INPUTBOX_HEIGHT_MIN || getmaxx(stdscr) <= width - INPUTBOX_WIDTH_MIN {
            return -ERRDISPLAYTOOSMALL;
        }
        x = (getmaxx(stdscr) - width) / 2;
        y = (getmaxy(stdscr) - height) / 2;
        draw_shadow(stdscr, y, x, height, width);
        dialog = newwin(height, width, y, x);
        keypad(dialog, TRUE);
        draw_box(dialog, 0, 0, height, width, dlg.dialog.atr, dlg.border.atr);
        wattrset(dialog, dlg.border.atr);
        mvwaddch(dialog, height - 3, 0, ACS_LTEE);
        i = 0;
        while i < width - 2 { waddch(dialog, ACS_HLINE); i += 1; }
        wattrset(dialog, dlg.dialog.atr);
        waddch(dialog, ACS_RTEE);
        print_title(dialog, title, width);
        wattrset(dialog, dlg.dialog.atr);
        print_autowrap(dialog, prompt, width - 2, 1, 3);
        box_width = width - 6;
        getyx(dialog, &mut y, &mut x);
        box_y = y + 2;
        box_x = (width - box_width) / 2;
        draw_box(dialog, y + 1, box_x - 1, 3, box_width + 2, dlg.dialog.atr, dlg.border.atr);
        print_buttons(dialog, height, width, 0);
        wmove(dialog, box_y, box_x);
        wattrset(dialog, dlg.inputbox.atr);
        len = 0;
        while *instr.add(len as usize) != 0 { len += 1; }
        pos = len;
        if len >= box_width {
            show_x = len - box_width + 1;
            input_x = box_width - 1;
            i = 0; while i < box_width - 1 { waddch(dialog, *instr.add((show_x + i) as usize) as u32); i += 1; }
        } else { show_x = 0; input_x = len; waddstr(dialog, instr); }
        wmove(dialog, box_y, box_x + input_x);
        wrefresh(dialog);

        while key != KEY_ESC {
            key = wgetch(dialog);
            if button == -1 {
                match key {
                    k if k == TAB || k == KEY_UP || k == KEY_DOWN => {}
                    k if k == KEY_BACKSPACE || k == 8 || k == 127 => {
                        if pos != 0 { if input_x == 0 { show_x -= 1; } else { input_x -= 1; } if pos < len { i = pos - 1; while i < len { *instr.add(i as usize) = *instr.add((i + 1) as usize); i += 1; } } pos -= 1; len -= 1; *instr.add(len as usize) = 0; wmove(dialog, box_y, box_x); i = 0; while i < box_width { let ch = *instr.add((show_x + i) as usize); if ch == 0 { waddch(dialog, b' ' as u32); break; } waddch(dialog, ch as u32); i += 1; } wmove(dialog, box_y, input_x + box_x); wrefresh(dialog); }
                        continue;
                    }
                    k if k == KEY_LEFT => { if pos > 0 { if input_x > 0 { input_x -= 1; wmove(dialog, box_y, input_x + box_x); } else { show_x -= 1; wmove(dialog, box_y, box_x); i = 0; while i < box_width { let ch = *instr.add((show_x + i) as usize); if ch == 0 { waddch(dialog, b' ' as u32); break; } waddch(dialog, ch as u32); i += 1; } wmove(dialog, box_y, box_x); } pos -= 1; } continue; }
                    k if k == KEY_RIGHT => { if pos < len { if input_x < box_width - 1 { input_x += 1; wmove(dialog, box_y, input_x + box_x); } else { show_x += 1; wmove(dialog, box_y, box_x); i = 0; while i < box_width { let ch = *instr.add((show_x + i) as usize); if ch == 0 { waddch(dialog, b' ' as u32); break; } waddch(dialog, ch as u32); i += 1; } wmove(dialog, box_y, input_x + box_x); } pos += 1; } continue; }
                    _ => { if key < 0x100 && isprint(key) != 0 { if len < MAX_LEN { if pos < len { i = len; while i > pos { *instr.add(i as usize) = *instr.add((i - 1) as usize); i -= 1; } *instr.add(pos as usize) = key as c_char; } else { *instr.add(len as usize) = key as c_char; } pos += 1; len += 1; *instr.add(len as usize) = 0; if input_x == box_width - 1 { show_x += 1; } else { input_x += 1; } wmove(dialog, box_y, box_x); i = 0; while i < box_width { let ch = *instr.add((show_x + i) as usize); if ch == 0 { waddch(dialog, b' ' as u32); break; } waddch(dialog, ch as u32); i += 1; } wmove(dialog, box_y, input_x + box_x); wrefresh(dialog); } else { flash(); } continue; } }
                }
            }
            match key {
                79 | 111 => { delwin(dialog); return 0; }
                72 | 104 => { delwin(dialog); return 1; }
                k if k == KEY_UP || k == KEY_LEFT => { button = match button { -1 => 1, 0 => -1, 1 => 0, _ => button }; print_buttons(dialog, height, width, if button == 1 { 1 } else { 0 }); if button == -1 { wmove(dialog, box_y, box_x + input_x); wrefresh(dialog); } }
                k if k == TAB || k == KEY_DOWN || k == KEY_RIGHT => { button = match button { -1 => 0, 0 => 1, 1 => -1, _ => button }; print_buttons(dialog, height, width, if button == 1 { 1 } else { 0 }); if button == -1 { wmove(dialog, box_y, box_x + input_x); wrefresh(dialog); } }
                32 | 10 => { delwin(dialog); return if button == -1 { 0 } else { button }; }
                88 | 120 => key = KEY_ESC,
                k if k == KEY_ESC => key = on_key_esc(dialog),
                k if k == KEY_RESIZE => { delwin(dialog); on_key_resize(); continue 'do_resize; }
                _ => {}
            }
        }
        delwin(dialog);
        return KEY_ESC;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
