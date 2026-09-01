// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * tui.c ncurses text user interface for TMON program
 *
 * Copyright (C) 2013 Intel Corporation. All rights reserved.
 *
 * Author: Jacob Pan <jacob.jun.pan@linux.intel.com>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::ptr;

#[repr(C)]
pub struct WINDOW {
	_private: [u8; 0],
}

#[repr(C)]
pub struct PANEL {
	_private: [u8; 0],
}

#[repr(C)]
pub struct cooling_dev_info {
	pub instance: c_int,
	pub type_: [c_char; 32],
	pub cur_state: c_ulong,
	pub max_state: c_ulong,
}

#[repr(C)]
pub struct trip_point {
	pub temp: c_ulong,
	pub type_: c_int,
}

#[repr(C)]
pub struct thermal_zone_info {
	pub instance: c_int,
	pub type_: [c_char; 32],
	pub nr_trip_pts: c_int,
	pub tp: *mut trip_point,
	pub cdev_binding: c_ulong,
	pub trip_binding: *mut c_ulong,
}

#[repr(C)]
pub struct thermal_data {
	pub nr_cooling_dev: c_int,
	pub nr_tz_sensor: c_int,
	pub cdi: *mut cooling_dev_info,
	pub tzi: *mut thermal_zone_info,
}

#[repr(C)]
pub struct pid_param {
	pub kp: f64,
	pub ki: f64,
	pub kd: f64,
	pub y_k: f64,
	pub t_target: f64,
}

#[repr(C)]
pub struct thermal_record {
	pub temp: *mut c_int,
}

unsafe extern "C" {
	static mut stdscr: *mut WINDOW;
	static mut ptdata: thermal_data;
	static mut p_param: pid_param;
	static mut target_thermal_zone: c_int;
	static mut ctrl_cdev: *mut c_char;
	static mut dialogue_on: c_int;
	static mut tmon_exit: c_int;
	static mut input_lock: pthread_mutex_t;
	static mut trec: *mut thermal_record;
	static mut cur_thermal_record: c_int;

	static VERSION: [c_char; 0];

	fn del_panel(panel: *mut PANEL) -> c_int;
	fn delwin(win: *mut WINDOW) -> c_int;
	fn mvwprintw(win: *mut WINDOW, y: c_int, x: c_int, fmt: *const c_char, ...) -> c_int;
	fn wrefresh(win: *mut WINDOW) -> c_int;
	fn getmaxy(win: *mut WINDOW) -> c_int;
	fn getmaxx(win: *mut WINDOW) -> c_int;
	fn resizeterm(lines: c_int, columns: c_int) -> c_int;
	fn subwin(win: *mut WINDOW, nlines: c_int, ncols: c_int, begin_y: c_int, begin_x: c_int) -> *mut WINDOW;
	fn scrollok(win: *mut WINDOW, bf: bool) -> c_int;
	fn strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char;
	fn wmove(win: *mut WINDOW, y: c_int, x: c_int) -> c_int;
	fn new_panel(win: *mut WINDOW) -> *mut PANEL;
	fn syslog(priority: c_int, fmt: *const c_char, ...);
	fn set_panel_userptr(panel: *mut PANEL, ptr: *const c_void) -> c_int;
	fn doupdate() -> c_int;
	fn werase(win: *mut WINDOW) -> c_int;
	fn refresh() -> c_int;
	fn endwin() -> c_int;
	fn clear() -> c_int;
	fn sleep(seconds: c_uint) -> c_uint;
	fn signal(signum: c_int, handler: extern "C" fn(c_int)) -> sighandler_t;
	fn wattron(win: *mut WINDOW, attrs: c_int) -> c_int;
	fn wattroff(win: *mut WINDOW, attrs: c_int) -> c_int;
	fn wborder(
		win: *mut WINDOW,
		ls: c_int,
		rs: c_int,
		ts: c_int,
		bs: c_int,
		tl: c_int,
		tr: c_int,
		bl: c_int,
		br: c_int,
	) -> c_int;
	fn box_(win: *mut WINDOW, verch: c_int, horch: c_int) -> c_int;
	fn initscr() -> *mut WINDOW;
	fn start_color() -> c_int;
	fn keypad(win: *mut WINDOW, bf: bool) -> c_int;
	fn nonl() -> c_int;
	fn cbreak() -> c_int;
	fn noecho() -> c_int;
	fn curs_set(visibility: c_int) -> c_int;
	fn use_default_colors() -> c_int;
	fn init_pair(pair: c_short, f: c_short, b: c_short) -> c_int;
	fn wattrset(win: *mut WINDOW, attrs: c_int) -> c_int;
	fn wbkgd(win: *mut WINDOW, ch: c_int) -> c_int;
	fn strlen(s: *const c_char) -> usize;
	fn echo() -> c_int;
	fn wgetnstr(win: *mut WINDOW, str_: *mut c_char, n: c_int) -> c_int;
	fn atoi(nptr: *const c_char) -> c_int;
	fn snprintf(str_: *mut c_char, size: usize, format: *const c_char, ...) -> c_int;
	fn sysfs_set_ulong(path: *const c_char, name: *const c_char, val: c_int) -> c_int;
	fn panel_userptr(panel: *mut PANEL) -> *const c_void;
	fn top_panel(panel: *mut PANEL) -> c_int;
	fn wgetch(win: *mut WINDOW) -> c_int;
	fn pthread_mutex_lock(mutex: *mut pthread_mutex_t) -> c_int;
	fn pthread_mutex_unlock(mutex: *mut pthread_mutex_t) -> c_int;
	fn update_panels() -> c_int;
	fn mvwaddch(win: *mut WINDOW, y: c_int, x: c_int, ch: c_ulong) -> c_int;
	fn whline(win: *mut WINDOW, ch: c_ulong, n: c_int) -> c_int;
	fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
	fn get_ctrl_state(state: *mut c_ulong) -> c_int;
}

pub type c_uint = u32;
pub type c_short = i16;
pub type pthread_mutex_t = c_void;
pub type sighandler_t = usize;

const TRUE: bool = true;
const EOF: c_int = -1;
const LOG_DEBUG: c_int = 7;
const LOG_INFO: c_int = 6;
const SIGWINCH: c_int = 28;
const A_BOLD: c_int = 1 << 13;
const A_REVERSE: c_int = 1 << 18;
const COLOR_BLACK: c_short = 0;
const COLOR_RED: c_short = 1;
const COLOR_GREEN: c_short = 2;
const COLOR_YELLOW: c_short = 3;
const COLOR_BLUE: c_short = 4;
const COLOR_WHITE: c_short = 7;
const ACS_VLINE: c_ulong = 0;
const ACS_RARROW: c_ulong = 0;
const KEY_LEFT: c_int = 260;

const PT_COLOR_DEFAULT: c_short = 1;
const PT_COLOR_HEADER_BAR: c_short = 2;
const PT_COLOR_ERROR: c_short = 3;
const PT_COLOR_RED: c_short = 4;
const PT_COLOR_YELLOW: c_short = 5;
const PT_COLOR_GREEN: c_short = 6;
const PT_COLOR_BLUE: c_short = 7;
const PT_COLOR_BRIGHT: c_short = 8;
const MIN_CTRL_TEMP: c_int = 0;
const MAX_CTRL_TEMP: c_int = 100;
const THERMAL_SYSFS: &[u8] = b"/sys/class/thermal\0";
const CDEV: &[u8] = b"cooling_device\0";
const DIAG_Y: c_int = 5;
const DIAG_X: c_int = 5;
const NR_LINES_TZDATA: c_int = 1;
const TZONE_RECORD_SIZE: c_int = 10;
const TZ_LEFT_ALIGN: c_int = 18;
const MAX_DISP_TEMP: c_int = 120;
const TDATA_LEFT: c_int = 12;
const THERMAL_TRIP_CRITICAL: c_int = 0;
const THERMAL_TRIP_HOT: c_int = 1;
const THERMAL_TRIP_PASSIVE: c_int = 2;
const THERMAL_TRIP_ACTIVE: c_int = 3;

static mut data_panel: *mut PANEL = ptr::null_mut();
static mut dialogue_panel: *mut PANEL = ptr::null_mut();
static mut top: *mut PANEL = ptr::null_mut();

static mut title_bar_window: *mut WINDOW = ptr::null_mut();
static mut tz_sensor_window: *mut WINDOW = ptr::null_mut();
static mut cooling_device_window: *mut WINDOW = ptr::null_mut();
static mut control_window: *mut WINDOW = ptr::null_mut();
static mut status_bar_window: *mut WINDOW = ptr::null_mut();
static mut thermal_data_window: *mut WINDOW = ptr::null_mut();
static mut dialogue_window: *mut WINDOW = ptr::null_mut();

#[unsafe(no_mangle)]
pub static mut status_bar_slots: [[c_char; 40]; 10] = [[0; 40]; 10];

static mut maxx: c_int = 0;
static mut maxy: c_int = 0;
static mut maxwidth: c_int = 200;

const TITLE_BAR_HIGHT: c_int = 1;
const SENSOR_WIN_HIGHT: c_int = 4; /* one row for tz name, one for trip points */

/* daemon mode flag (set by startup parameter -d) */
static mut tui_disabled: c_int = 0;

unsafe fn close_panel(mut p: *mut PANEL) {
	if !p.is_null() {
		del_panel(p);
		p = ptr::null_mut();
	}
}

unsafe fn close_window(mut win: *mut WINDOW) {
	if !win.is_null() {
		delwin(win);
		win = ptr::null_mut();
	}
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn close_windows() {
	if tui_disabled != 0 {
		return;
	}
	/* must delete panels before their attached windows */
	if !dialogue_window.is_null() {
		close_panel(dialogue_panel);
	}
	if !cooling_device_window.is_null() {
		close_panel(data_panel);
	}

	close_window(title_bar_window);
	close_window(tz_sensor_window);
	close_window(status_bar_window);
	close_window(cooling_device_window);
	close_window(control_window);
	close_window(thermal_data_window);
	close_window(dialogue_window);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn write_status_bar(x: c_int, line: *mut c_char) {
	mvwprintw(status_bar_window, 0, x, b"%s\0".as_ptr() as *const c_char, line);
	wrefresh(status_bar_window);
}

/* wrap at 5 */
const DIAG_DEV_ROWS: c_int = 5;
/*
 * list cooling devices + "set temp" entry; wraps after 5 rows, if they fit
 */
unsafe fn diag_dev_rows() -> c_int {
	let entries = ptdata.nr_cooling_dev + 1;
	let rows = DIAG_DEV_ROWS.max((entries + 1) / 2);
	rows.min(entries)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn setup_windows() {
	let mut y_begin: c_int = 1;

	if tui_disabled != 0 {
		return;
	}

	maxy = getmaxy(stdscr);
	maxx = getmaxx(stdscr);
	resizeterm(maxy, maxx);

	title_bar_window = subwin(stdscr, TITLE_BAR_HIGHT, maxx, 0, 0);
	y_begin += TITLE_BAR_HIGHT;

	tz_sensor_window = subwin(stdscr, SENSOR_WIN_HIGHT, maxx, y_begin, 0);
	y_begin += SENSOR_WIN_HIGHT;

	cooling_device_window = subwin(stdscr, ptdata.nr_cooling_dev + 3, maxx, y_begin, 0);
	y_begin += ptdata.nr_cooling_dev + 3; /* 2 lines for border */
	/* two lines to show borders, one line per tz show trip point position
	 * and value.
	 * dialogue window is a pop-up, when needed it lays on top of cdev win
	 */

	dialogue_window = subwin(stdscr, diag_dev_rows() + 5, maxx - 50, DIAG_Y, DIAG_X);

	thermal_data_window = subwin(stdscr, ptdata.nr_tz_sensor * NR_LINES_TZDATA + 3, maxx, y_begin, 0);
	y_begin += ptdata.nr_tz_sensor * NR_LINES_TZDATA + 3;
	control_window = subwin(stdscr, 4, maxx, y_begin, 0);

	scrollok(cooling_device_window, TRUE);
	maxwidth = maxx - 18;
	status_bar_window = subwin(stdscr, 1, maxx, maxy - 1, 0);

	strcpy(status_bar_slots[0].as_mut_ptr(), b" Ctrl-c - Quit \0".as_ptr() as *const c_char);
	strcpy(status_bar_slots[1].as_mut_ptr(), b" TAB - Tuning \0".as_ptr() as *const c_char);
	wmove(status_bar_window, 1, 30);

	/* prepare panels for dialogue, if panel already created then we must
	 * be doing resizing, so just replace windows with new ones, old ones
	 * should have been deleted by close_window
	 */
	data_panel = new_panel(cooling_device_window);
	if data_panel.is_null() {
		syslog(LOG_DEBUG, b"No data panel\n\0".as_ptr() as *const c_char);
	} else if !dialogue_window.is_null() {
		dialogue_panel = new_panel(dialogue_window);
		if dialogue_panel.is_null() {
			syslog(LOG_DEBUG, b"No dialogue panel\n\0".as_ptr() as *const c_char);
		} else {
			/* Set up the user pointer to the next panel*/
			set_panel_userptr(data_panel, dialogue_panel as *const c_void);
			set_panel_userptr(dialogue_panel, data_panel as *const c_void);
			top = data_panel;
		}
	} else {
		syslog(LOG_INFO, b"no dialogue win, term too small\n\0".as_ptr() as *const c_char);
	}
	doupdate();
	werase(stdscr);
	refresh();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn resize_handler(sig: c_int) {
	/* start over when term gets resized, but first we clean up */
	close_windows();
	endwin();
	refresh();
	clear();
	maxy = getmaxy(stdscr);  /* get the new screen size */
	maxx = getmaxx(stdscr);
	setup_windows();
	/* rate limit */
	sleep(1);
	syslog(LOG_DEBUG, b"SIG %d, term resized to %d x %d\n\0".as_ptr() as *const c_char, sig, maxy, maxx);
	signal(SIGWINCH, resize_handler);
}

#[unsafe(no_mangle)]
pub static cdev_title: [c_char; 18] = *b" COOLING DEVICES \0";

#[unsafe(no_mangle)]
pub unsafe extern "C" fn show_cooling_device() {
	let mut i: c_int;
	let mut j: c_int;
	let mut x: c_int;
	let mut y: c_int = 0;

	if tui_disabled != 0 || cooling_device_window.is_null() {
		return;
	}

	werase(cooling_device_window);
	wattron(cooling_device_window, A_BOLD);
	mvwprintw(cooling_device_window, 1, 1, b"ID  Cooling Dev   Cur    Max   Thermal Zone Binding\0".as_ptr() as *const c_char);
	wattroff(cooling_device_window, A_BOLD);
	j = 0;
	while j < ptdata.nr_cooling_dev {
		/* draw cooling device list on the left in the order of
		 * cooling device instances. skip unused idr.
		 */
		let cdi = ptdata.cdi.add(j as usize);
		mvwprintw(cooling_device_window, j + 2, 1, b"%02d %12.12s%6lu %6lu\0".as_ptr() as *const c_char,
			(*cdi).instance, (*cdi).type_.as_ptr(), (*cdi).cur_state, (*cdi).max_state);
		j += 1;
	}

	/* show cdev binding, y is the global cooling device instance */
	i = 0;
	while i < ptdata.nr_tz_sensor {
		let tzi = ptdata.tzi.add(i as usize);
		let tz_inst = (*tzi).instance;
		j = 0;
		while j < ptdata.nr_cooling_dev {
			y = j;
			x = tz_inst * TZONE_RECORD_SIZE + TZ_LEFT_ALIGN;

			draw_hbar(cooling_device_window, y + 2, x, TZONE_RECORD_SIZE - 1, ACS_VLINE, false);

			/* draw a column of spaces to separate thermal zones */
			mvwprintw(cooling_device_window, y + 2, x - 1, b" \0".as_ptr() as *const c_char);
			if (*tzi).cdev_binding != 0 {
				let cdev_inst = (*ptdata.cdi.add(j as usize)).instance;
				let mut trip_binding = *(*tzi).trip_binding.add(cdev_inst as usize);
				let mut k: c_int = 0; /* per zone trip point id that
					    * binded to this cdev, one to
					    * many possible based on the
					    * binding bitmask.
					    */
				syslog(LOG_DEBUG, b"bind tz%d cdev%d tp%lx %d cdev%lx\n\0".as_ptr() as *const c_char,
					i, j, trip_binding, y, (*tzi).cdev_binding);
				/* draw each trip binding for the cdev */
				while {
					trip_binding >>= 1;
					trip_binding != 0
				} {
					k += 1;
					if (trip_binding & 1) == 0 {
						continue;
					}
					/* draw '*' to show binding */
					mvwprintw(cooling_device_window, y + 2, x + (*tzi).nr_trip_pts - k - 1,
						b"*\0".as_ptr() as *const c_char);
				}
			}
			j += 1;
		}
		i += 1;
	}
	/* draw border after data so that border will not be messed up
	 * even there is not enough space for all the data to be shown
	 */
	wborder(cooling_device_window, 0, 0, 0, 0, 0, 0, 0, 0);
	wattron(cooling_device_window, A_BOLD);
	mvwprintw(cooling_device_window, 0, maxx / 2 - cdev_title.len() as c_int,
		cdev_title.as_ptr());
	wattroff(cooling_device_window, A_BOLD);

	wrefresh(cooling_device_window);
}

#[unsafe(no_mangle)]
pub static DIAG_TITLE: [c_char; 13] = *b"[ TUNABLES ]\0";

#[unsafe(no_mangle)]
pub unsafe extern "C" fn show_dialogue() {
	let mut j: c_int;
	let mut x: c_int = 0;
	let mut y: c_int;
	let rows: c_int;
	let cols: c_int;
	let w = dialogue_window;

	if tui_disabled != 0 || w.is_null() {
		return;
	}

	rows = getmaxy(w);
	cols = getmaxx(w);

	/* Silence compiler 'unused' warnings */
	let _ = cols;

	werase(w);
	box_(w, 0, 0);
	mvwprintw(w, 0, maxx / 4, DIAG_TITLE.as_ptr());
	/* list all the available tunables */
	j = 0;
	while j <= ptdata.nr_cooling_dev {
		y = j % diag_dev_rows();
		if y == 0 && j != 0 {
			x += 20;
		}
		if j == ptdata.nr_cooling_dev {
			/* save last choice for target temp */
			mvwprintw(w, y + 1, x + 1, b"%C-%.12s\0".as_ptr() as *const c_char, 'A' as c_int + j, b"Set Temp\0".as_ptr() as *const c_char);
		} else {
			let cdi = ptdata.cdi.add(j as usize);
			mvwprintw(w, y + 1, x + 1, b"%C-%.10s-%2d\0".as_ptr() as *const c_char,
				'A' as c_int + j, (*cdi).type_.as_ptr(), (*cdi).instance);
		}
		j += 1;
	}
	wattron(w, A_BOLD);
	mvwprintw(w, diag_dev_rows() + 1, 1, b"Enter Choice [A-Z]?\0".as_ptr() as *const c_char);
	wattroff(w, A_BOLD);
	/* print legend at the bottom line */
	mvwprintw(w, rows - 2, 1, b"Legend: A=Active, P=Passive, C=Critical\0".as_ptr() as *const c_char);

	wrefresh(dialogue_window);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn write_dialogue_win(buf: *mut c_char, y: c_int, x: c_int) {
	let w = dialogue_window;

	mvwprintw(w, y, x, b"%s\0".as_ptr() as *const c_char, buf);
}

#[unsafe(no_mangle)]
pub static control_title: [c_char; 11] = *b" CONTROLS \0";

#[unsafe(no_mangle)]
pub unsafe extern "C" fn show_control_w() {
	let mut state: c_ulong = 0;

	get_ctrl_state(&mut state);

	if tui_disabled != 0 || control_window.is_null() {
		return;
	}

	werase(control_window);
	mvwprintw(control_window, 1, 1, b"PID gain: kp=%2.2f ki=%2.2f kd=%2.2f Output %2.2f\0".as_ptr() as *const c_char,
		p_param.kp, p_param.ki, p_param.kd, p_param.y_k);

	mvwprintw(control_window, 2, 1, b"Target Temp: %2.1fC, Zone: %d, Control Device: %.12s\0".as_ptr() as *const c_char,
		p_param.t_target, target_thermal_zone, ctrl_cdev);

	/* draw border last such that everything is within boundary */
	wborder(control_window, 0, 0, 0, 0, 0, 0, 0, 0);
	wattron(control_window, A_BOLD);
	mvwprintw(control_window, 0, maxx / 2 - control_title.len() as c_int,
		control_title.as_ptr());
	wattroff(control_window, A_BOLD);

	wrefresh(control_window);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn initialize_curses() {
	if tui_disabled != 0 {
		return;
	}

	initscr();
	start_color();
	keypad(stdscr, TRUE);	/* enable keyboard mapping */
	nonl();			/* tell curses not to do NL->CR/NL on output */
	cbreak();		/* take input chars one at a time */
	noecho();		/* dont echo input */
	curs_set(0);		/* turn off cursor */
	use_default_colors();

	init_pair(PT_COLOR_DEFAULT, COLOR_WHITE, COLOR_BLACK);
	init_pair(PT_COLOR_HEADER_BAR, COLOR_BLACK, COLOR_WHITE);
	init_pair(PT_COLOR_ERROR, COLOR_BLACK, COLOR_RED);
	init_pair(PT_COLOR_RED, COLOR_WHITE, COLOR_RED);
	init_pair(PT_COLOR_YELLOW, COLOR_WHITE, COLOR_YELLOW);
	init_pair(PT_COLOR_GREEN, COLOR_WHITE, COLOR_GREEN);
	init_pair(PT_COLOR_BLUE, COLOR_WHITE, COLOR_BLUE);
	init_pair(PT_COLOR_BRIGHT, COLOR_WHITE, COLOR_BLACK);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn show_title_bar() {
	let mut i: c_int;
	let mut x: c_int = 0;

	if tui_disabled != 0 || title_bar_window.is_null() {
		return;
	}

	wattrset(title_bar_window, COLOR_PAIR(PT_COLOR_HEADER_BAR));
	wbkgd(title_bar_window, COLOR_PAIR(PT_COLOR_HEADER_BAR));
	werase(title_bar_window);

	mvwprintw(title_bar_window, 0, 0, b"     TMON v%s\0".as_ptr() as *const c_char, VERSION.as_ptr());

	wrefresh(title_bar_window);

	werase(status_bar_window);

	i = 0;
	while i < 10 {
		if strlen(status_bar_slots[i as usize].as_ptr()) == 0 {
			i += 1;
			continue;
		}
		wattron(status_bar_window, A_REVERSE);
		mvwprintw(status_bar_window, 0, x, b"%s\0".as_ptr() as *const c_char, status_bar_slots[i as usize].as_ptr());
		wattroff(status_bar_window, A_REVERSE);
		x += strlen(status_bar_slots[i as usize].as_ptr()) as c_int + 1;
		i += 1;
	}
	wrefresh(status_bar_window);
}

unsafe fn COLOR_PAIR(n: c_short) -> c_int {
	(n as c_int) << 8
}

unsafe fn handle_input_val(ch: c_int) {
	let mut buf: [c_char; 32] = [0; 32];
	let val: c_int;
	let mut path: [c_char; 256] = [0; 256];
	let w = dialogue_window;

	echo();
	keypad(w, TRUE);
	wgetnstr(w, buf.as_mut_ptr(), 31);
	val = atoi(buf.as_ptr());

	if ch == ptdata.nr_cooling_dev {
		snprintf(buf.as_mut_ptr(), 31, b"Invalid Temp %d! %d-%d\0".as_ptr() as *const c_char, val, MIN_CTRL_TEMP, MAX_CTRL_TEMP);
		if val < MIN_CTRL_TEMP || val > MAX_CTRL_TEMP {
			write_status_bar(40, buf.as_mut_ptr());
		} else {
			p_param.t_target = val as f64;
			snprintf(buf.as_mut_ptr(), 31, b"Set New Target Temp %d\0".as_ptr() as *const c_char, val);
			write_status_bar(40, buf.as_mut_ptr());
		}
	} else {
		snprintf(path.as_mut_ptr(), 256, b"%s/%s%d\0".as_ptr() as *const c_char,
			THERMAL_SYSFS.as_ptr() as *const c_char, CDEV.as_ptr() as *const c_char, (*ptdata.cdi.add(ch as usize)).instance);
		sysfs_set_ulong(path.as_ptr(), b"cur_state\0".as_ptr() as *const c_char, val);
	}
	noecho();
	dialogue_on = 0;
	show_data_w();
	show_control_w();

	top = panel_userptr(top) as *mut PANEL;
	top_panel(top);
}

unsafe fn handle_input_choice(ch: c_int) {
	let mut buf: [c_char; 48] = [0; 48];
	let mut base: c_int = 0;
	let mut cdev_id: c_int = 0;

	if (ch >= 'A' as c_int && ch <= 'A' as c_int + ptdata.nr_cooling_dev) ||
		(ch >= 'a' as c_int && ch <= 'a' as c_int + ptdata.nr_cooling_dev) {
		base = if ch < 'a' as c_int { 'A' as c_int } else { 'a' as c_int };
		cdev_id = ch - base;
		if ptdata.nr_cooling_dev == cdev_id {
			snprintf(buf.as_mut_ptr(), buf.len(), b"New Target Temp:\0".as_ptr() as *const c_char);
		} else {
			snprintf(buf.as_mut_ptr(), buf.len(), b"New Value for %.10s-%2d: \0".as_ptr() as *const c_char,
				(*ptdata.cdi.add(cdev_id as usize)).type_.as_ptr(),
				(*ptdata.cdi.add(cdev_id as usize)).instance);
		}
		write_dialogue_win(buf.as_mut_ptr(), diag_dev_rows() + 2, 2);
		handle_input_val(cdev_id);
	} else {
		snprintf(buf.as_mut_ptr(), buf.len(), b"Invalid selection %d\0".as_ptr() as *const c_char, ch);
		write_dialogue_win(buf.as_mut_ptr(), 8, 2);
	}
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_tui_events(arg: *mut c_void) -> *mut c_void {
	let mut ch: c_int;

	keypad(cooling_device_window, TRUE);
	loop {
		ch = wgetch(cooling_device_window);
		if ch == EOF {
			break;
		}
		if tmon_exit != 0 {
			break;
		}
		/* when term size is too small, no dialogue panels are set.
		 * we need to filter out such cases.
		 */
		if data_panel.is_null() || dialogue_panel.is_null() ||
			cooling_device_window.is_null() ||
			dialogue_window.is_null() {
			continue;
		}
		pthread_mutex_lock(&mut input_lock);
		if dialogue_on != 0 {
			handle_input_choice(ch);
			/* top panel filter */
			if ch == 'q' as c_int || ch == 'Q' as c_int {
				ch = 0;
			}
		}
		match ch {
			KEY_LEFT => {
				box_(cooling_device_window, 10, 0);
			}
			9 => {
				/* TAB */
				top = panel_userptr(top) as *mut PANEL;
				top_panel(top);
				if top == dialogue_panel {
					dialogue_on = 1;
					show_dialogue();
				} else {
					dialogue_on = 0;
					/* force refresh */
					show_data_w();
					show_control_w();
				}
			}
			x if x == 'q' as c_int || x == 'Q' as c_int => {
				tmon_exit = 1;
			}
			_ => {}
		}
		update_panels();
		doupdate();
		pthread_mutex_unlock(&mut input_lock);
	}

	if !arg.is_null() {
		*(arg as *mut c_int) = 0; /* make gcc happy */
	}

	ptr::null_mut()
}

/* draw a horizontal bar in given pattern */
unsafe fn draw_hbar(win: *mut WINDOW, y: c_int, start: c_int, len: c_int, ptn: c_ulong, end: bool) {
	mvwaddch(win, y, start, ptn);
	whline(win, ptn, len);
	if end {
		mvwaddch(win, y, MAX_DISP_TEMP + TDATA_LEFT, ']' as c_ulong);
	}
}

unsafe fn trip_type_to_char(type_: c_int) -> c_char {
	if type_ == THERMAL_TRIP_CRITICAL {
		return 'C' as c_char;
	}
	if type_ == THERMAL_TRIP_HOT {
		return 'H' as c_char;
	}
	if type_ == THERMAL_TRIP_PASSIVE {
		return 'P' as c_char;
	}
	if type_ == THERMAL_TRIP_ACTIVE {
		return 'A' as c_char;
	}
	'?' as c_char
}

/* fill a string with trip point type and value in one line
 * e.g.      P(56)    C(106)
 * maintain the distance one degree per char
 */
unsafe fn draw_tp_line(tz: c_int, y: c_int) {
	let mut j: c_int;
	let x: c_int;

	j = 0;
	while j < (*ptdata.tzi.add(tz as usize)).nr_trip_pts {
		x = ((*(*ptdata.tzi.add(tz as usize)).tp.add(j as usize)).temp / 1000) as c_int;
		mvwprintw(thermal_data_window, y + 0, x + TDATA_LEFT,
			b"%c%d\0".as_ptr() as *const c_char, trip_type_to_char((*(*ptdata.tzi.add(tz as usize)).tp.add(j as usize)).type_) as c_int,
			x);
		syslog(LOG_INFO, b"%s:tz %d tp %d temp = %lu\n\0".as_ptr() as *const c_char, b"draw_tp_line\0".as_ptr() as *const c_char,
			tz, j, (*(*ptdata.tzi.add(tz as usize)).tp.add(j as usize)).temp);
		j += 1;
	}
}

#[unsafe(no_mangle)]
pub static data_win_title: [c_char; 15] = *b" THERMAL DATA \0";

#[unsafe(no_mangle)]
pub unsafe extern "C" fn show_data_w() {
	let mut i: c_int;

	if tui_disabled != 0 || thermal_data_window.is_null() {
		return;
	}

	werase(thermal_data_window);
	wattron(thermal_data_window, A_BOLD);
	mvwprintw(thermal_data_window, 0, maxx / 2 - data_win_title.len() as c_int,
		data_win_title.as_ptr());
	wattroff(thermal_data_window, A_BOLD);
	/* draw a line as ruler */
	i = 10;
	while i < MAX_DISP_TEMP {
		mvwprintw(thermal_data_window, 1, i + TDATA_LEFT, b"%2d\0".as_ptr() as *const c_char, i);
		i += 10;
	}

	i = 0;
	while i < ptdata.nr_tz_sensor {
		let temp = (*(*trec.add(cur_thermal_record as usize)).temp.add(i as usize)) / 1000;
		let mut y: c_int = 0;

		y = i * NR_LINES_TZDATA + 2;
		/* y at tz temp data line */
		mvwprintw(thermal_data_window, y, 1, b"%6.6s%2d:[%3d][\0".as_ptr() as *const c_char,
			(*ptdata.tzi.add(i as usize)).type_.as_ptr(),
			(*ptdata.tzi.add(i as usize)).instance, temp);
		draw_hbar(thermal_data_window, y, TDATA_LEFT, temp, ACS_RARROW, true);
		draw_tp_line(i, y);
		i += 1;
	}
	wborder(thermal_data_window, 0, 0, 0, 0, 0, 0, 0, 0);
	wrefresh(thermal_data_window);
}

#[unsafe(no_mangle)]
pub static tz_title: [c_char; 23] = *b"THERMAL ZONES(SENSORS)\0";

#[unsafe(no_mangle)]
pub unsafe extern "C" fn show_sensors_w() {
	let mut i: c_int;
	let mut j: c_int;
	let mut buffer: [c_char; 512] = [0; 512];

	if tui_disabled != 0 || tz_sensor_window.is_null() {
		return;
	}

	werase(tz_sensor_window);

	memset(buffer.as_mut_ptr() as *mut c_void, 0, buffer.len());
	wattron(tz_sensor_window, A_BOLD);
	mvwprintw(tz_sensor_window, 1, 1, b"Thermal Zones:\0".as_ptr() as *const c_char);
	wattroff(tz_sensor_window, A_BOLD);

	mvwprintw(tz_sensor_window, 1, TZ_LEFT_ALIGN, b"%s\0".as_ptr() as *const c_char, buffer.as_ptr());
	/* fill trip points for each tzone */
	wattron(tz_sensor_window, A_BOLD);
	mvwprintw(tz_sensor_window, 2, 1, b"Trip Points:\0".as_ptr() as *const c_char);
	wattroff(tz_sensor_window, A_BOLD);

	/* draw trip point from low to high for each tz */
	i = 0;
	while i < ptdata.nr_tz_sensor {
		let inst = (*ptdata.tzi.add(i as usize)).instance;

		mvwprintw(tz_sensor_window, 1,
			TZ_LEFT_ALIGN + TZONE_RECORD_SIZE * inst, b"%.9s%02d\0".as_ptr() as *const c_char,
			(*ptdata.tzi.add(i as usize)).type_.as_ptr(), (*ptdata.tzi.add(i as usize)).instance);
		j = (*ptdata.tzi.add(i as usize)).nr_trip_pts - 1;
		while j >= 0 {
			/* loop through all trip points */
			let type_: c_char;
			let tp_pos: c_int;
			/* reverse the order here since trips are sorted
			 * in ascending order in terms of temperature.
			 */
			tp_pos = (*ptdata.tzi.add(i as usize)).nr_trip_pts - j - 1;

			type_ = trip_type_to_char((*(*ptdata.tzi.add(i as usize)).tp.add(j as usize)).type_);
			mvwaddch(tz_sensor_window, 2,
				inst * TZONE_RECORD_SIZE + TZ_LEFT_ALIGN + tp_pos, type_ as c_ulong);
			syslog(LOG_DEBUG, b"draw tz %d tp %d ch:%c\n\0".as_ptr() as *const c_char,
				inst, j, type_ as c_int);
			if j == 0 {
				break;
			}
			j -= 1;
		}
		i += 1;
	}
	wborder(tz_sensor_window, 0, 0, 0, 0, 0, 0, 0, 0);
	wattron(tz_sensor_window, A_BOLD);
	mvwprintw(tz_sensor_window, 0, maxx / 2 - tz_title.len() as c_int, tz_title.as_ptr());
	wattroff(tz_sensor_window, A_BOLD);
	wrefresh(tz_sensor_window);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn disable_tui() {
	tui_disabled = 1;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
