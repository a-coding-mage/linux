// SPDX-License-Identifier: GPL-2.0+
/*
 * checklist.c -- implements the checklist box
 *
 * ORIGINAL AUTHOR: Savio Lam (lam836@cs.cuhk.hk)
 * Stuart Herbert - S.Herbert@sheffield.ac.uk: radiolist extension
 * Alessandro Rubini - rubini@ipvvis.unipv.it: merged the two
 * MODIFIED FOR LINUX KERNEL CONFIG BY: William Roadcap (roadcap@cfw.com)
 */

// The declarations supplied by dialog.h and the curses implementation remain
// external dependencies of this translation.

static mut list_width: i32 = 0;
static mut check_x: i32 = 0;
static mut item_x: i32 = 0;

unsafe fn print_item(win: *mut WINDOW, choice: i32, selected: i32) {
    let mut i: i32;
    let list_item = malloc((list_width + 1) as usize) as *mut c_char;

    strncpy(list_item, item_str(), (list_width - item_x) as usize);
    *list_item.add((list_width - item_x) as usize) = 0;

    wattrset(win, (*dlg).menubox.atr);
    wmove(win, choice, 0);
    i = 0;
    while i < list_width {
        waddch(win, ' ' as u32);
        i += 1;
    }

    wmove(win, choice, check_x);
    wattrset(win, if selected != 0 { (*dlg).check_selected.atr } else { (*dlg).check.atr });
    if !item_is_tag(':' as i32) {
        wprintw(win, "(%c)", if item_is_tag('X' as i32) { 'X' } else { ' ' });
    }

    wattrset(win, if selected != 0 { (*dlg).tag_selected.atr } else { (*dlg).tag.atr });
    mvwaddch(win, choice, item_x, *list_item as u32);
    wattrset(win, if selected != 0 { (*dlg).item_selected.atr } else { (*dlg).item.atr });
    waddstr(win, list_item.add(1));
    if selected != 0 {
        wmove(win, choice, check_x + 1);
        wrefresh(win);
    }
    free(list_item as *mut c_void);
}

unsafe fn print_arrows(win: *mut WINDOW, choice: i32, item_no: i32, scroll: i32,
                       mut y: i32, x: i32, height: i32) {
    wmove(win, y, x);
    if scroll > 0 {
        wattrset(win, (*dlg).uarrow.atr);
        waddch(win, ACS_UARROW);
        waddstr(win, "(-)");
    } else {
        wattrset(win, (*dlg).menubox.atr);
        for _ in 0..4 { waddch(win, ACS_HLINE); }
    }
    y += height + 1;
    wmove(win, y, x);
    if height < item_no && scroll + choice < item_no - 1 {
        wattrset(win, (*dlg).darrow.atr);
        waddch(win, ACS_DARROW);
        waddstr(win, "(+)");
    } else {
        wattrset(win, (*dlg).menubox_border.atr);
        for _ in 0..4 { waddch(win, ACS_HLINE); }
    }
}

unsafe fn print_buttons(dialog: *mut WINDOW, height: i32, width: i32, selected: i32) {
    let x = width / 2 - 11;
    let y = height - 2;
    print_button(dialog, "Select", y, x, selected == 0);
    print_button(dialog, " Help ", y, x + 14, selected == 1);
    wmove(dialog, y, x + 1 + 14 * selected);
    wrefresh(dialog);
}

unsafe fn dialog_checklist(title: *const c_char, prompt: *const c_char, height: i32,
                          width: i32, list_height: i32) -> i32 {
    let mut i: i32;
    let (mut x, mut y, mut box_x, mut box_y): (i32, i32, i32, i32);
    let (mut key, mut button, mut choice, mut scroll): (i32, i32, i32, i32) = (0, 0, 0, 0);
    let mut max_choice: i32;
    let (mut dialog, mut list): (*mut WINDOW, *mut WINDOW);

    item_foreach!({
        if item_is_tag('X' as i32) { choice = item_n(); }
        if item_is_selected() { choice = item_n(); break; }
    });

    'do_resize: loop {
        if getmaxy(stdscr) < height + CHECKLIST_HEIGHT_MIN { return -ERRDISPLAYTOOSMALL; }
        if getmaxx(stdscr) < width + CHECKLIST_WIDTH_MIN { return -ERRDISPLAYTOOSMALL; }
        max_choice = MIN(list_height, item_count());
        x = (getmaxx(stdscr) - width) / 2;
        y = (getmaxy(stdscr) - height) / 2;
        draw_shadow(stdscr, y, x, height, width);
        dialog = newwin(height, width, y, x);
        keypad(dialog, TRUE);
        draw_box(dialog, 0, 0, height, width, (*dlg).dialog.atr, (*dlg).border.atr);
        wattrset(dialog, (*dlg).border.atr);
        mvwaddch(dialog, height - 3, 0, ACS_LTEE);
        for _ in 0..(width - 2) { waddch(dialog, ACS_HLINE); }
        wattrset(dialog, (*dlg).dialog.atr);
        waddch(dialog, ACS_RTEE);
        print_title(dialog, title, width);
        wattrset(dialog, (*dlg).dialog.atr);
        print_autowrap(dialog, prompt, width - 2, 1, 3);
        list_width = width - 6;
        box_y = height - list_height - 5;
        box_x = (width - list_width) / 2 - 1;
        list = subwin(dialog, list_height, list_width, y + box_y + 1, x + box_x + 1);
        keypad(list, TRUE);
        draw_box(dialog, box_y, box_x, list_height + 2, list_width + 2,
                 (*dlg).menubox_border.atr, (*dlg).menubox.atr);
        check_x = 0;
        item_foreach!({ check_x = MAX(check_x, strlen(item_str()) as i32 + 4); });
        check_x = MIN(check_x, list_width);
        check_x = (list_width - check_x) / 2;
        item_x = check_x + 4;
        if choice >= list_height { scroll = choice - list_height + 1; choice -= scroll; }
        for i in 0..max_choice { item_set(scroll + i); print_item(list, i, (i == choice) as i32); }
        print_arrows(dialog, choice, item_count(), scroll, box_y, box_x + check_x + 5, list_height);
        print_buttons(dialog, height, width, 0);
        wmove(list, choice, check_x + 1); wrefresh(list);

        while key != KEY_ESC {
            key = wgetch(dialog);
            i = 0;
            while i < max_choice { item_set(i + scroll); if toupper(key) == toupper(*item_str() as i32) { break; } i += 1; }
            if i < max_choice || key == KEY_UP || key == KEY_DOWN || key == '+' as i32 || key == '-' as i32 {
                if key == KEY_UP || key == '-' as i32 { if choice == 0 { if scroll == 0 { continue; } scroll -= 1; item_set(scroll); print_item(list, 0, 1); print_arrows(dialog, choice, item_count(), scroll, box_y, box_x + check_x + 5, list_height); wnoutrefresh(dialog); wrefresh(list); continue; } else { i = choice - 1; } }
                else if key == KEY_DOWN || key == '+' as i32 { if choice == max_choice - 1 { if scroll + choice >= item_count() - 1 { continue; } scroll += 1; item_set(scroll + max_choice - 1); print_item(list, max_choice - 1, 1); print_arrows(dialog, choice, item_count(), scroll, box_y, box_x + check_x + 5, list_height); wnoutrefresh(dialog); wrefresh(list); continue; } else { i = choice + 1; } }
                if i != choice { item_set(scroll + choice); print_item(list, choice, 0); choice = i; item_set(scroll + choice); print_item(list, choice, 1); wnoutrefresh(dialog); wrefresh(list); }
                continue;
            }
            match key {
                 'H' as i32 | 'h' as i32 | '?' as i32 => { button = 1; },
                'S' as i32 | 's' as i32 | ' ' as i32 | '\n' as i32 => { item_foreach!({ item_set_selected(0); }); item_set(scroll + choice); item_set_selected(1); delwin(list); delwin(dialog); return button; },
                TAB | KEY_LEFT | KEY_RIGHT => { button = if (if key == KEY_LEFT { button - 1 } else { button + 1 }) < 0 { 1 } else if button > 1 { 0 } else { button }; print_buttons(dialog, height, width, button); wrefresh(dialog); },
                'X' as i32 | 'x' as i32 => key = KEY_ESC,
                KEY_ESC => key = on_key_esc(dialog),
                KEY_RESIZE => { delwin(list); delwin(dialog); on_key_resize(); continue 'do_resize; },
                _ => {}
            }
            doupdate();
        }
        delwin(list); delwin(dialog); return key;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
