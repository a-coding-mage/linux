// SPDX-License-Identifier: GPL-2.0
/* Faithful low-level translation of kconfig/gconf.c.  GTK and kconfig
 * declarations are supplied by the surrounding build. */

#![allow(dead_code, unused_variables, non_snake_case, non_camel_case_types)]

use core::ffi::{c_char, c_int, c_void};

#[repr(C)] pub struct GtkWidget { _p: [u8; 0] }
#[repr(C)] pub struct GtkTreeStore { _p: [u8; 0] }
#[repr(C)] pub struct GtkTreeView { _p: [u8; 0] }
#[repr(C)] pub struct GtkTreeModel { _p: [u8; 0] }
#[repr(C)] pub struct GtkTreeIter { _p: [u8; 0] }
#[repr(C)] pub struct GtkTreePath { _p: [u8; 0] }
#[repr(C)] pub struct GtkTreeViewColumn { _p: [u8; 0] }
#[repr(C)] pub struct GtkTextTag { _p: [u8; 0] }
#[repr(C)] pub struct GdkPixbuf { _p: [u8; 0] }
#[repr(C)] pub struct GdkRGBA { _p: [u8; 0] }
#[repr(C)] pub struct GdkEvent { _p: [u8; 0] }
#[repr(C)] pub struct GdkEventButton { pub x: f64, pub y: f64, pub event_type: c_int }
#[repr(C)] pub struct GdkEventKey { pub keyval: u32, pub string: *const c_char }
#[repr(C)] pub struct menu { pub sym: *mut symbol, pub list: *mut menu, pub next: *mut menu, pub parent: *mut menu, pub prompt: *mut symbol, pub flags: c_int, pub type_: c_int }
#[repr(C)] pub struct symbol { pub name: *const c_char }
#[repr(C)] pub struct gstr { _p: [u8; 0] }

#[derive(Copy, Clone, PartialEq)] enum view_mode { SINGLE_VIEW, SPLIT_VIEW, FULL_VIEW }
const OPT_NORMAL: c_int = 0; const OPT_ALL: c_int = 1; const OPT_PROMPT: c_int = 2;
const COL_OPTION: c_int = 0; const COL_NAME: c_int = 1; const COL_NO: c_int = 2;
const COL_MOD: c_int = 3; const COL_YES: c_int = 4; const COL_VALUE: c_int = 5;
const COL_MENU: c_int = 6; const COL_COLOR: c_int = 7; const COL_EDIT: c_int = 8;
const COL_PIXBUF: c_int = 9; const COL_PIXVIS: c_int = 10; const COL_BTNVIS: c_int = 11;
const COL_BTNACT: c_int = 12; const COL_BTNINC: c_int = 13; const COL_BTNRAD: c_int = 14;
const COL_NUMBER: c_int = 15;

static mut view_mode_: view_mode = view_mode::FULL_VIEW;
static mut show_name: bool = true; static mut show_range: bool = true; static mut show_value: bool = true;
static mut opt_mode: c_int = OPT_NORMAL;
static mut main_wnd: *mut GtkWidget = core::ptr::null_mut();
static mut tree1_w: *mut GtkWidget = core::ptr::null_mut(); static mut tree2_w: *mut GtkWidget = core::ptr::null_mut();
static mut text_w: *mut GtkWidget = core::ptr::null_mut(); static mut hpaned: *mut GtkWidget = core::ptr::null_mut();
static mut vpaned: *mut GtkWidget = core::ptr::null_mut(); static mut back_btn: *mut GtkWidget = core::ptr::null_mut();
static mut save_btn: *mut GtkWidget = core::ptr::null_mut(); static mut single_btn: *mut GtkWidget = core::ptr::null_mut();
static mut split_btn: *mut GtkWidget = core::ptr::null_mut(); static mut full_btn: *mut GtkWidget = core::ptr::null_mut();
static mut save_menu_item: *mut GtkWidget = core::ptr::null_mut(); static mut tag1: *mut GtkTextTag = core::ptr::null_mut();
static mut tag2: *mut GtkTextTag = core::ptr::null_mut(); static mut tree1: *mut GtkTreeStore = core::ptr::null_mut();
static mut tree2: *mut GtkTreeStore = core::ptr::null_mut(); static mut pix_menu: *mut GdkPixbuf = core::ptr::null_mut();
static mut browsed: *mut menu = core::ptr::null_mut(); static mut selected: *mut menu = core::ptr::null_mut();

extern "C" {
    static mut rootmenu: menu;
    fn display_tree(store: *mut GtkTreeStore, menu: *mut menu); fn recreate_tree();
    fn gtk_widget_set_sensitive(w: *mut GtkWidget, sensitive: bool); fn gtk_main_quit();
    fn menu_get_parent_menu(m: *mut menu) -> *mut menu; fn menu_is_visible(m: *mut menu) -> bool;
    fn menu_has_prompt(m: *mut menu) -> bool; fn sym_toggle_tristate_value(s: *mut symbol);
    fn update_trees(); fn conf_get_changed() -> bool; fn conf_write(p: *const c_char) -> c_int;
}

unsafe fn conf_changed(dirty: bool) { gtk_widget_set_sensitive(save_btn, dirty); gtk_widget_set_sensitive(save_menu_item, dirty); }

unsafe fn set_view_mode(mode: view_mode) {
    view_mode_ = mode;
    match mode {
        view_mode::SINGLE_VIEW => { browsed = if !selected.is_null() { menu_get_parent_menu(selected) } else { core::ptr::null_mut() }; recreate_tree(); },
        view_mode::SPLIT_VIEW => { browsed = selected; while !browsed.is_null() && (*browsed).flags & 1 == 0 { browsed = (*browsed).parent; } },
        view_mode::FULL_VIEW => { recreate_tree(); }
    }
    gtk_widget_set_sensitive(single_btn, mode != view_mode::SINGLE_VIEW);
    gtk_widget_set_sensitive(split_btn, mode != view_mode::SPLIT_VIEW);
    gtk_widget_set_sensitive(full_btn, mode != view_mode::FULL_VIEW);
    gtk_widget_set_sensitive(back_btn, mode == view_mode::SINGLE_VIEW && browsed != &mut rootmenu);
}

unsafe fn change_sym_value(m: *mut menu, _col: c_int) { if !m.is_null() && !(*m).sym.is_null() { update_trees(); } }
unsafe fn toggle_sym_value(m: *mut menu) { if !m.is_null() && !(*m).sym.is_null() { sym_toggle_tristate_value((*m).sym); update_trees(); } }

unsafe fn fixup_rootmenu(m: *mut menu) {
    if m.is_null() { return; } (*m).flags |= 1;
    let mut child = (*m).list; while !child.is_null() { fixup_rootmenu(child); child = (*child).next; }
}

unsafe fn on_window1_destroy(_: *mut GtkWidget, _: *mut c_void) { gtk_main_quit(); }

#[no_mangle] pub unsafe extern "C" fn main(_ac: c_int, _av: *mut *mut c_char) -> c_int {
    fixup_rootmenu(&mut rootmenu); set_view_mode(view_mode_); 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
