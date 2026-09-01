// SPDX-License-Identifier: GPL-2.0
// Translated from perf/ui/gtk/browser.c.
// C dependencies: "gtk.h", "../evsel.h", "../sort.h", "../hist.h",
// "../helpline.h", and <signal.h>.

use core::ffi::{c_char, c_double, c_int, c_uint, c_void};
use core::ptr;

#[repr(C)]
pub struct GtkWidget {
    _private: [u8; 0],
}

#[repr(C)]
pub struct GdkScreen {
    _private: [u8; 0],
}

#[repr(C)]
pub struct GdkWindow {
    _private: [u8; 0],
}

#[repr(C)]
pub struct GtkWindow {
    _private: [u8; 0],
}

#[repr(C)]
pub struct GtkInfoBar {
    _private: [u8; 0],
}

#[repr(C)]
pub struct GtkContainer {
    _private: [u8; 0],
}

#[repr(C)]
pub struct GtkStatusbar {
    _private: [u8; 0],
}

#[repr(C)]
pub struct GdkRectangle {
    pub x: c_int,
    pub y: c_int,
    pub width: c_int,
    pub height: c_int,
}

#[repr(C)]
pub struct perf_gtk_context {
    pub main_window: *mut GtkWidget,
    pub notebook: *mut GtkWidget,
    pub info_bar: *mut GtkWidget,
    pub message_label: *mut GtkWidget,
    pub statbar: *mut GtkWidget,
    pub statbar_ctx_id: c_uint,
}

pub const TRUE: c_int = 1;
pub const GTK_RESPONSE_OK: c_int = -5;

unsafe extern "C" {
    pub static mut pgctx: *mut perf_gtk_context;
    pub static MIN_RED: c_double;
    pub static MIN_GREEN: c_double;
    pub static GTK_STOCK_OK: *const c_char;

    pub fn perf_gtk__exit(wait_for_ok: bool);
    pub fn psignal(sig: c_int, s: *const c_char);

    pub fn gtk_widget_get_screen(widget: *mut GtkWidget) -> *mut GdkScreen;
    pub fn gdk_screen_get_monitor_at_window(
        screen: *mut GdkScreen,
        window: *mut GdkWindow,
    ) -> c_int;
    pub fn gdk_screen_get_monitor_geometry(
        screen: *mut GdkScreen,
        monitor_num: c_int,
        dest: *mut GdkRectangle,
    );
    pub fn gtk_window_resize(window: *mut GtkWindow, width: c_int, height: c_int);

    pub fn gtk_info_bar_new() -> *mut GtkWidget;
    pub fn gtk_widget_set_no_show_all(widget: *mut GtkWidget, no_show_all: c_int);
    pub fn gtk_label_new(str: *const c_char) -> *mut GtkWidget;
    pub fn gtk_widget_show(widget: *mut GtkWidget);
    pub fn gtk_info_bar_get_content_area(info_bar: *mut GtkInfoBar) -> *mut GtkWidget;
    pub fn gtk_container_add(container: *mut GtkContainer, widget: *mut GtkWidget);
    pub fn gtk_info_bar_add_button(
        info_bar: *mut GtkInfoBar,
        button_text: *const c_char,
        response_id: c_int,
    ) -> *mut GtkWidget;
    pub fn gtk_widget_hide(widget: *mut GtkWidget);
    pub fn g_signal_connect_data(
        instance: *mut c_void,
        detailed_signal: *const c_char,
        c_handler: *mut c_void,
        data: *mut c_void,
        destroy_data: *mut c_void,
        connect_flags: c_int,
    ) -> c_uint;

    pub fn gtk_statusbar_new() -> *mut GtkWidget;
    pub fn gtk_statusbar_get_context_id(
        statusbar: *mut GtkStatusbar,
        context_description: *const c_char,
    ) -> c_uint;
}

// Field layout supplied by GTK headers in C. The C source directly reads
// window->window.
#[repr(C)]
pub struct GtkWidgetWithWindow {
    pub window: *mut GdkWindow,
}

#[inline]
unsafe fn GTK_WINDOW(widget: *mut GtkWidget) -> *mut GtkWindow {
    widget as *mut GtkWindow
}

#[inline]
unsafe fn GTK_INFO_BAR(widget: *mut GtkWidget) -> *mut GtkInfoBar {
    widget as *mut GtkInfoBar
}

#[inline]
unsafe fn GTK_CONTAINER(widget: *mut GtkWidget) -> *mut GtkContainer {
    widget as *mut GtkContainer
}

#[inline]
unsafe fn GTK_STATUSBAR(widget: *mut GtkWidget) -> *mut GtkStatusbar {
    widget as *mut GtkStatusbar
}

#[inline]
unsafe fn G_CALLBACK(func: unsafe extern "C" fn(*mut GtkWidget)) -> *mut c_void {
    func as *mut c_void
}

#[inline]
unsafe fn g_signal_connect(
    instance: *mut GtkWidget,
    detailed_signal: *const c_char,
    c_handler: *mut c_void,
    data: *mut c_void,
) -> c_uint {
    unsafe {
        g_signal_connect_data(
            instance as *mut c_void,
            detailed_signal,
            c_handler,
            data,
            ptr::null_mut(),
            0,
        )
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_gtk__signal(sig: c_int) {
    unsafe {
        perf_gtk__exit(false);
        psignal(sig, c"perf".as_ptr());
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_gtk__resize_window(window: *mut GtkWidget) {
    let mut rect: GdkRectangle = GdkRectangle {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    let screen: *mut GdkScreen;
    let monitor: c_int;
    let height: c_int;
    let width: c_int;

    unsafe {
        screen = gtk_widget_get_screen(window);

        monitor = gdk_screen_get_monitor_at_window(
            screen,
            (*(window as *mut GtkWidgetWithWindow)).window,
        );

        gdk_screen_get_monitor_geometry(screen, monitor, &mut rect);

        width = rect.width * 3 / 4;
        height = rect.height * 3 / 4;

        gtk_window_resize(GTK_WINDOW(window), width, height);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_gtk__get_percent_color(percent: c_double) -> *const c_char {
    unsafe {
        if percent >= MIN_RED {
            return c"<span fgcolor='red'>".as_ptr();
        }
        if percent >= MIN_GREEN {
            return c"<span fgcolor='dark green'>".as_ptr();
        }
        ptr::null()
    }
}

// Original C conditional: #ifdef HAVE_GTK_INFO_BAR_SUPPORT
#[cfg(HAVE_GTK_INFO_BAR_SUPPORT)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_gtk__setup_info_bar() -> *mut GtkWidget {
    let info_bar: *mut GtkWidget;
    let label: *mut GtkWidget;
    let content_area: *mut GtkWidget;

    unsafe {
        info_bar = gtk_info_bar_new();
        gtk_widget_set_no_show_all(info_bar, TRUE);

        label = gtk_label_new(c"".as_ptr());
        gtk_widget_show(label);

        content_area = gtk_info_bar_get_content_area(GTK_INFO_BAR(info_bar));
        gtk_container_add(GTK_CONTAINER(content_area), label);

        gtk_info_bar_add_button(GTK_INFO_BAR(info_bar), GTK_STOCK_OK, GTK_RESPONSE_OK);
        g_signal_connect(
            info_bar,
            c"response".as_ptr(),
            G_CALLBACK(gtk_widget_hide),
            ptr::null_mut(),
        );

        (*pgctx).info_bar = info_bar;
        (*pgctx).message_label = label;

        info_bar
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_gtk__setup_statusbar() -> *mut GtkWidget {
    let stbar: *mut GtkWidget;
    let ctxid: c_uint;

    unsafe {
        stbar = gtk_statusbar_new();

        ctxid = gtk_statusbar_get_context_id(GTK_STATUSBAR(stbar), c"perf report".as_ptr());
        (*pgctx).statbar = stbar;
        (*pgctx).statbar_ctx_id = ctxid;

        stbar
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
