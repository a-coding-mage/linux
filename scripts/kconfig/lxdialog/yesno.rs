// SPDX-License-Identifier: GPL-2.0+
/*
 *  yesno.c -- implements the yes/no box
 *
 *  ORIGINAL AUTHOR: Savio Lam (lam836@cs.cuhk.hk)
 *  MODIFIED FOR LINUX KERNEL CONFIG BY: William Roadcap (roadcap@cfw.com)
 */

use std::os::raw::{c_char, c_int};

/* The declarations below are supplied by dialog.h and the surrounding C
 * implementation. */
#[repr(C)]
pub struct WINDOW {
    _private: [u8; 0],
}

#[repr(C)]
pub struct DialogAttributes {
    pub atr: c_int,
}

#[repr(C)]
pub struct DialogConfig {
    pub dialog: DialogAttributes,
    pub border: DialogAttributes,
}

extern "C" {
    static mut stdscr: *mut WINDOW;
    static mut dlg: DialogConfig;

    fn getmaxy(window: *mut WINDOW) -> c_int;
    fn getmaxx(window: *mut WINDOW) -> c_int;
    fn draw_shadow(window: *mut WINDOW, y: c_int, x: c_int, height: c_int, width: c_int);
    fn newwin(height: c_int, width: c_int, y: c_int, x: c_int) -> *mut WINDOW;
    fn keypad(window: *mut WINDOW, enabled: c_int);
    fn draw_box(
        window: *mut WINDOW,
        y: c_int,
        x: c_int,
        height: c_int,
        width: c_int,
        dialog_atr: c_int,
        border_atr: c_int,
    );
    fn wattrset(window: *mut WINDOW, attributes: c_int);
    fn mvwaddch(window: *mut WINDOW, y: c_int, x: c_int, character: c_int);
    fn waddch(window: *mut WINDOW, character: c_int);
    fn print_title(window: *mut WINDOW, title: *const c_char, width: c_int);
    fn print_autowrap(
        window: *mut WINDOW,
        prompt: *const c_char,
        width: c_int,
        x: c_int,
        y: c_int,
    );
    fn print_button(window: *mut WINDOW, label: *const c_char, y: c_int, x: c_int, selected: c_int);
    fn wmove(window: *mut WINDOW, y: c_int, x: c_int);
    fn wrefresh(window: *mut WINDOW);
    fn wgetch(window: *mut WINDOW) -> c_int;
    fn delwin(window: *mut WINDOW);
    fn on_key_esc(window: *mut WINDOW) -> c_int;
    fn on_key_resize();
}

/* Values supplied by the dialog implementation/build configuration. */
extern "C" {
    static YESNO_HEIGHT_MIN: c_int;
    static YESNO_WIDTH_MIN: c_int;
    static ERRDISPLAYTOOSMALL: c_int;
    static KEY_ESC: c_int;
    static TAB: c_int;
    static KEY_LEFT: c_int;
    static KEY_RIGHT: c_int;
    static KEY_RESIZE: c_int;
    static TRUE: c_int;
    static ACS_LTEE: c_int;
    static ACS_HLINE: c_int;
    static ACS_RTEE: c_int;
}

/* Display termination buttons */
unsafe fn print_buttons(dialog: *mut WINDOW, height: c_int, width: c_int, selected: c_int) {
    let x = width / 2 - 10;
    let y = height - 2;

    print_button(dialog, b" Yes \0".as_ptr() as *const c_char, y, x, (selected == 0) as c_int);
    print_button(dialog, b"  No  \0".as_ptr() as *const c_char, y, x + 13, (selected == 1) as c_int);

    wmove(dialog, y, x + 1 + 13 * selected);
    wrefresh(dialog);
}

/* Display a dialog box with two buttons - Yes and No */
pub unsafe fn dialog_yesno(
    title: *const c_char,
    prompt: *const c_char,
    height: c_int,
    width: c_int,
) -> c_int {
    let mut i: c_int;
    let mut x: c_int;
    let mut y: c_int;
    let mut key: c_int = 0;
    let mut button: c_int = 0;
    let mut dialog: *mut WINDOW;

    'do_resize: loop {
        if getmaxy(stdscr) < height + YESNO_HEIGHT_MIN {
            return -ERRDISPLAYTOOSMALL;
        }
        if getmaxx(stdscr) < width + YESNO_WIDTH_MIN {
            return -ERRDISPLAYTOOSMALL;
        }

        /* center dialog box on screen */
        x = (getmaxx(stdscr) - width) / 2;
        y = (getmaxy(stdscr) - height) / 2;

        draw_shadow(stdscr, y, x, height, width);

        dialog = newwin(height, width, y, x);
        keypad(dialog, TRUE);

        draw_box(dialog, 0, 0, height, width, dlg.dialog.atr, dlg.border.atr);
        wattrset(dialog, dlg.border.atr);
        mvwaddch(dialog, height - 3, 0, ACS_LTEE);
        i = 0;
        while i < width - 2 {
            waddch(dialog, ACS_HLINE);
            i += 1;
        }
        wattrset(dialog, dlg.dialog.atr);
        waddch(dialog, ACS_RTEE);

        print_title(dialog, title, width);

        wattrset(dialog, dlg.dialog.atr);
        print_autowrap(dialog, prompt, width - 2, 1, 3);

        print_buttons(dialog, height, width, 0);

        while key != KEY_ESC {
            key = wgetch(dialog);
            match key {
                89 | 121 => {
                    delwin(dialog);
                    return 0;
                }
                78 | 110 => {
                    delwin(dialog);
                    return 1;
                }
                k if k == TAB || k == KEY_LEFT || k == KEY_RIGHT => {
                    button = if (if key == KEY_LEFT { button -= 1; button } else { button += 1; button }) < 0 {
                        1
                    } else if button > 1 {
                        0
                    } else {
                        button
                    };
                    print_buttons(dialog, height, width, button);
                    wrefresh(dialog);
                }
                32 | 10 => {
                    delwin(dialog);
                    return button;
                }
                k if k == KEY_ESC => {
                    key = on_key_esc(dialog);
                }
                k if k == KEY_RESIZE => {
                    delwin(dialog);
                    on_key_resize();
                    continue 'do_resize;
                }
                _ => {}
            }
        }

        delwin(dialog);
        return key; /* ESC pressed */
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
