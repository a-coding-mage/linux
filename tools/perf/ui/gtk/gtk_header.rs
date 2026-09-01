/* SPDX-License-Identifier: GPL-2.0 */

/* Translated from perf/ui/gtk/gtk.h. */
/* Depends on C headers: <stdbool.h> and <gtk/gtk.h>. */
/* Original C suppressed -Wstrict-prototypes around the GTK include. */

#[repr(C)]
pub struct perf_gtk_context {
    pub main_window: *mut GtkWidget,
    pub notebook: *mut GtkWidget,

    /* Present when HAVE_GTK_INFO_BAR_SUPPORT is defined in the C build. */
    #[cfg(HAVE_GTK_INFO_BAR_SUPPORT)]
    pub info_bar: *mut GtkWidget,
    #[cfg(HAVE_GTK_INFO_BAR_SUPPORT)]
    pub message_label: *mut GtkWidget,

    pub statbar: *mut GtkWidget,
    pub statbar_ctx_id: guint,
}

unsafe extern "C" {
    pub fn perf_gtk__init() -> ::std::os::raw::c_int;
    pub fn perf_gtk__exit(wait_for_ok: bool);

    pub static mut pgctx: *mut perf_gtk_context;
}

#[inline]
pub unsafe fn perf_gtk__is_active_context(ctx: *mut perf_gtk_context) -> bool {
    !ctx.is_null() && unsafe { !(*ctx).main_window.is_null() }
}

unsafe extern "C" {
    pub fn perf_gtk__activate_context(window: *mut GtkWidget) -> *mut perf_gtk_context;
    pub fn perf_gtk__deactivate_context(ctx: *mut *mut perf_gtk_context) -> ::std::os::raw::c_int;

    pub fn perf_gtk__init_helpline();
    pub fn gtk_ui_progress__init();
    pub fn perf_gtk__init_hpp();

    pub fn perf_gtk__signal(sig: ::std::os::raw::c_int);
    pub fn perf_gtk__resize_window(window: *mut GtkWidget);
    pub fn perf_gtk__get_percent_color(percent: ::std::os::raw::c_double) -> *const ::std::os::raw::c_char;
    pub fn perf_gtk__setup_statusbar() -> *mut GtkWidget;
}

#[cfg(HAVE_GTK_INFO_BAR_SUPPORT)]
unsafe extern "C" {
    pub fn perf_gtk__setup_info_bar() -> *mut GtkWidget;
}

#[cfg(not(HAVE_GTK_INFO_BAR_SUPPORT))]
#[inline]
pub unsafe fn perf_gtk__setup_info_bar() -> *mut GtkWidget {
    ::std::ptr::null_mut()
}

#[repr(C)]
pub struct evsel {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct evlist {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct hist_entry {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct hist_browser_timer {
    _unused: [u8; 0],
}

unsafe extern "C" {
    pub fn evlist__gtk_browse_hists(
        evlist: *mut evlist,
        help: *const ::std::os::raw::c_char,
        hbt: *mut hist_browser_timer,
        min_pcnt: ::std::os::raw::c_float,
    ) -> ::std::os::raw::c_int;
    pub fn hist_entry__gtk_annotate(
        he: *mut hist_entry,
        evsel: *mut evsel,
        hbt: *mut hist_browser_timer,
    ) -> ::std::os::raw::c_int;
    pub fn perf_gtk__show_annotations();
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
