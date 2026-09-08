// SPDX-License-Identifier: GPL-2.0+
/*
 *  menubox.c -- implements the menu box
 *
 *  ORIGINAL AUTHOR: Savio Lam (lam836@cs.cuhk.hk)
 *  MODIFIED FOR LINUX KERNEL CONFIG BY: William Roadcap (roadcapw@cfw.com)
 */

/* The historical change log is preserved from the C implementation. */

use core::ffi::{c_char, c_int, c_void};

static mut MENU_WIDTH: c_int = 0;
static mut ITEM_X: c_int = 0;

/* Print menu item */
unsafe fn do_print_item(win: *mut WINDOW, item: *const c_char, line_y: c_int,
                        selected: c_int, hotkey: c_int) {
    let mut j: c_int;
    let menu_item = libc::malloc((MENU_WIDTH + 1) as usize) as *mut c_char;

    libc::strncpy(menu_item, item, (MENU_WIDTH - ITEM_X) as usize);
    *menu_item.add((MENU_WIDTH - ITEM_X) as usize) = 0;
    j = first_alpha(menu_item, b"YyNnMmHh\0".as_ptr() as *const c_char);

    wattrset(win, dlg.menubox.atr);
    wmove(win, line_y, 0);
    wclrtoeol(win);
    wattrset(win, if selected != 0 { dlg.item_selected.atr } else { dlg.item.atr });
    mvwaddstr(win, line_y, ITEM_X, menu_item);
    if hotkey != 0 {
        wattrset(win, if selected != 0 { dlg.tag_key_selected.atr } else { dlg.tag_key.atr });
        mv waddch(win, line_y, ITEM_X + j, *menu_item.add(j as usize));
    }
    if selected != 0 {
        wmove(win, line_y, ITEM_X + 1);
    }
    libc::free(menu_item as *mut c_void);
    wrefresh(win);
}

macro_rules! print_item {
    ($index:expr, $choice:expr, $selected:expr) => {{
        item_set($index);
        do_print_item(menu, item_str(), $choice, $selected, !item_is_tag(b':' as c_int));
    }};
}

/* Print the scroll indicators. */
unsafe fn print_arrows(win: *mut WINDOW, item_no: c_int, scroll: c_int,
                       mut y: c_int, x: c_int, height: c_int) {
    let (mut cur_y, mut cur_x) = (0, 0);
    getyx(win, &mut cur_y, &mut cur_x);
    wmove(win, y, x);
    if scroll > 0 {
        wattrset(win, dlg.uarrow.atr); waddch(win, ACS_UARROW); waddstr(win, b"(-)\0".as_ptr() as *const c_char);
    } else {
        wattrset(win, dlg.menubox.atr);
        for _ in 0..4 { waddch(win, ACS_HLINE); }
    }
    y += height + 1; wmove(win, y, x); wrefresh(win);
    if height < item_no && scroll + height < item_no {
        wattrset(win, dlg.darrow.atr); waddch(win, ACS_DARROW); waddstr(win, b"(+)\0".as_ptr() as *const c_char);
    } else {
        wattrset(win, dlg.menubox_border.atr);
        for _ in 0..4 { waddch(win, ACS_HLINE); }
    }
    wmove(win, cur_y, cur_x); wrefresh(win);
}

/* Display the termination buttons. */
unsafe fn print_buttons(win: *mut WINDOW, height: c_int, width: c_int, selected: c_int) {
    let x = width / 2 - 28; let y = height - 2;
    print_button(win, b"Select\0".as_ptr() as *const c_char, y, x, selected == 0);
    print_button(win, b" Exit \0".as_ptr() as *const c_char, y, x + 12, selected == 1);
    print_button(win, b" Help \0".as_ptr() as *const c_char, y, x + 24, selected == 2);
    print_button(win, b" Save \0".as_ptr() as *const c_char, y, x + 36, selected == 3);
    print_button(win, b" Load \0".as_ptr() as *const c_char, y, x + 48, selected == 4);
    wmove(win, y, x + 1 + 12 * selected); wrefresh(win);
}

/* scroll up n lines (n may be negative) */
unsafe fn do_scroll(win: *mut WINDOW, scroll: *mut c_int, n: c_int) {
    scrollok(win, TRUE); wscrl(win, n); scrollok(win, FALSE);
    *scroll += n; wrefresh(win);
}

/* Display a menu for choosing among a number of options */
pub unsafe fn dialog_menu(title: *const c_char, prompt: *const c_char,
                          selected: *const c_void, s_scroll: *mut c_int) -> c_int {
    let (mut i, mut j, mut x, mut y, mut box_x, mut box_y);
    let (mut height, mut width, mut menu_height);
    let (mut key, mut button, mut scroll, mut choice) = (0, 0, 0, 0);
    let (mut first_item, mut max_choice);
    let (mut dialog, mut menu): (*mut WINDOW, *mut WINDOW);

    'do_resize: loop {
        height = getmaxy(stdscr); width = getmaxx(stdscr);
        if height < MENUBOX_HEIGHT_MIN || width < MENUBOX_WIDTH_MIN { return -ERRDISPLAYTOOSMALL; }
        height -= 4; width -= 5; menu_height = height - 10;
        max_choice = MIN(menu_height, item_count());
        x = (getmaxx(stdscr) - width) / 2; y = (getmaxy(stdscr) - height) / 2;
        draw_shadow(stdscr, y, x, height, width);
        dialog = newwin(height, width, y, x); keypad(dialog, TRUE);
        draw_box(dialog, 0, 0, height, width, dlg.dialog.atr, dlg.border.atr);
        wattrset(dialog, dlg.border.atr); mvwaddch(dialog, height - 3, 0, ACS_LTEE);
        for _ in 0..(width - 2) { waddch(dialog, ACS_HLINE); }
        wattrset(dialog, dlg.dialog.atr); wbkgdset(dialog, dlg.dialog.atr & A_COLOR); waddch(dialog, ACS_RTEE);
        print_title(dialog, title, width); wattrset(dialog, dlg.dialog.atr);
        print_autowrap(dialog, prompt, width - 2, 1, 3);
        MENU_WIDTH = width - 6; box_y = height - menu_height - 5; box_x = (width - MENU_WIDTH) / 2 - 1;
        menu = subwin(dialog, menu_height, MENU_WIDTH, y + box_y + 1, x + box_x + 1); keypad(menu, TRUE);
        draw_box(dialog, box_y, box_x, menu_height + 2, MENU_WIDTH + 2, dlg.menubox_border.atr, dlg.menubox.atr);
        ITEM_X = if MENU_WIDTH >= 80 { (MENU_WIDTH - 70) / 2 } else { 4 };
        item_foreach!(); if !selected.is_null() && selected == item_data() { choice = item_n(); }
        scroll = *s_scroll; first_item = 0;
        if scroll <= choice && scroll + max_choice > choice && scroll >= 0 && scroll + max_choice <= item_count() { first_item = scroll; choice -= scroll; } else { scroll = 0; }
        if choice >= max_choice { scroll = item_count() - max_choice; if choice < item_count() - max_choice / 2 { scroll = choice - max_choice / 2; } first_item = scroll; choice -= scroll; }
        for i in 0..max_choice { print_item!(first_item + i, i, i == choice); }
        wnoutrefresh(menu); print_arrows(dialog, item_count(), scroll, box_y, box_x + ITEM_X + 1, menu_height);
        print_buttons(dialog, height, width, 0); wmove(menu, choice, ITEM_X + 1); wrefresh(menu);
        while key != KEY_ESC {
            key = wgetch(menu); if key < 256 && isalpha(key) != 0 { key = tolower(key); }
            if libc::strchr(b"ynmh \0".as_ptr() as *const c_char, key).is_null() { i = choice + 1; while i < max_choice { item_set(scroll + i); j = first_alpha(item_str(), b"YyNnMmHh\0".as_ptr() as *const c_char); if key == tolower(*item_str().add(j as usize)) { break; } i += 1; } if i == max_choice { i = 0; } } else { i = max_choice; }
            if item_count() != 0 && (i < max_choice || key == KEY_UP || key == KEY_DOWN || key == b'-' as c_int || key == b'+' as c_int || key == KEY_PPAGE || key == KEY_NPAGE) {
                print_item!(scroll + choice, choice, FALSE);
                if key == KEY_UP || key == b'-' as c_int { if choice < 2 && scroll != 0 { do_scroll(menu, &mut scroll, -1); print_item!(scroll, 0, FALSE); } else { choice = MAX(choice - 1, 0); } }
                else if key == KEY_DOWN || key == b'+' as c_int { if choice > max_choice - 3 && scroll + max_choice < item_count() { do_scroll(menu, &mut scroll, 1); print_item!(scroll + max_choice - 1, max_choice - 1, FALSE); } else { choice = MIN(choice + 1, max_choice - 1); } }
                else if key == KEY_PPAGE { for _ in 0..max_choice { if scroll > 0 { do_scroll(menu, &mut scroll, -1); print_item!(scroll, 0, FALSE); } else if choice > 0 { choice -= 1; } } }
                else if key == KEY_NPAGE { for _ in 0..max_choice { if scroll + max_choice < item_count() { do_scroll(menu, &mut scroll, 1); print_item!(scroll + max_choice - 1, max_choice - 1, FALSE); } else if choice + 1 < max_choice { choice += 1; } } }
                else { choice = i; }
                print_item!(scroll + choice, choice, TRUE); print_arrows(dialog, item_count(), scroll, box_y, box_x + ITEM_X + 1, menu_height); wnoutrefresh(dialog); wrefresh(menu); continue;
            }
            match key {
                KEY_LEFT | TAB | KEY_RIGHT => { button = if (if key == KEY_LEFT { button - 1 } else { button + 1 }) < 0 { 4 } else if button > 4 { 0 } else { button }; print_buttons(dialog, height, width, button); wrefresh(menu); }
                b' ' as c_int | b's' as c_int | b'y' as c_int | b'n' as c_int | b'm' as c_int | b'/' as c_int | b'h' as c_int | b'?' as c_int | b'z' as c_int | b'\n' as c_int => { *s_scroll = scroll; delwin(menu); delwin(dialog); item_set(scroll + choice); item_set_selected(1); return match key { b'h' as c_int | b'?' as c_int => 2, b's' as c_int | b'y' as c_int => 5, b'n' as c_int => 6, b'm' as c_int => 7, b' ' as c_int => 8, b'/' as c_int => 9, b'z' as c_int => 10, _ => button }; }
                b'e' as c_int | b'x' as c_int => { key = KEY_ESC; }
                KEY_ESC => { key = on_key_esc(menu); }
                KEY_RESIZE => { on_key_resize(); delwin(menu); delwin(dialog); continue 'do_resize; }
                _ => {}
            }
        }
        delwin(menu); delwin(dialog); return key;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
