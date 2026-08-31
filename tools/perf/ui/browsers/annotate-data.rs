// SPDX-License-Identifier: GPL-2.0
//
// Rust translation of perf/ui/browsers/annotate-data.c.
// C include dependencies are preserved here as external declarations.

use core::ffi::{c_char, c_double, c_int, c_long, c_uint, c_ulonglong, c_void};
use core::mem;
use core::ptr;

const FOLDED_SIGN: c_int = '+' as c_int;
const UNFOLD_SIGN: c_int = '-' as c_int;
const NOCHLD_SIGN: c_int = ' ' as c_int;

const SEEK_SET: c_int = 0;
const SEEK_CUR: c_int = 1;
const SEEK_END: c_int = 2;

const K_TIMER: c_int = -1;
const K_F1: c_int = 0x101;
const K_LEFT: c_int = 0x104;
const K_ESC: c_int = 27;
const HE_COLORSET_ROOT: c_int = 0;

const fn CTRL(c: u8) -> c_int {
    (c & 0x1f) as c_int
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct browser_entry {
    pub node: list_head,
    pub data: *mut annotated_member,
    pub hists: *mut type_hist_entry,
    pub parent: *mut browser_entry,
    pub children: list_head,
    pub indent: c_int,     /*indentation level, starts from 0 */
    pub nr_entries: c_int, /* # of visible entries: self + descendents */
    pub folded: bool,      /* only can be false when it has children */
}

#[repr(C)]
pub struct annotated_data_browser {
    pub b: ui_browser,
    pub entries: list_head,
    pub curr: *mut browser_entry,
    pub nr_events: c_int,
}

#[repr(C)]
pub struct ui_browser {
    pub refresh: Option<unsafe extern "C" fn(*mut ui_browser) -> c_uint>,
    pub seek: Option<unsafe extern "C" fn(*mut ui_browser, off_t, c_int)>,
    pub write: Option<unsafe extern "C" fn(*mut ui_browser, *mut c_void, c_int)>,
    pub priv_: *mut c_void,
    pub extra_title_lines: c_int,
    pub entries: *mut list_head,
    pub nr_entries: u32,
    pub top: *mut list_head,
    pub top_idx: u32,
    pub index: u32,
    pub rows: c_int,
    pub width: c_int,
    pub filter: Option<unsafe extern "C" fn(*mut ui_browser, *mut list_head) -> bool>,
}

#[repr(C)]
pub struct annotated_member {
    pub node: list_head,
    pub children: list_head,
    pub type_name: *const c_char,
    pub var_name: *const c_char,
    pub offset: c_int,
    pub size: c_int,
}

#[repr(C)]
pub struct annotated_data_type {
    pub self_: annotated_member,
    pub histograms: *mut *mut type_hist,
}

#[repr(C)]
pub struct type_hist {
    pub addr: *mut type_hist_entry,
    pub period: u64,
}

#[repr(C)]
pub struct type_hist_entry {
    pub nr_samples: c_int,
    pub period: u64,
}

#[repr(C)]
pub struct evsel {
    pub core: evsel_core,
}

#[repr(C)]
pub struct evsel_core {
    pub idx: c_int,
}

#[repr(C)]
pub struct hists {
    pub stats: hists_stats,
}

#[repr(C)]
pub struct hists_stats {
    pub nr_samples: c_int,
}

#[repr(C)]
pub struct hist_entry {
    pub mem_type: *mut annotated_data_type,
    pub hists: *mut hists,
    pub stat: hist_entry_stat,
}

#[repr(C)]
pub struct hist_entry_stat {
    pub nr_events: c_int,
}

#[repr(C)]
pub struct hist_browser_timer {
    pub refresh: c_int,
    pub timer: Option<unsafe extern "C" fn(*mut c_void)>,
    pub arg: *mut c_void,
}

#[repr(C)]
pub struct symbol_conf_t {
    pub skip_empty: bool,
    pub show_total_period: bool,
    pub show_nr_samples: bool,
}

pub type off_t = c_long;

unsafe extern "C" {
    static mut symbol_conf: symbol_conf_t;

    fn zalloc(size: usize) -> *mut c_void;
    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn zfree(ptr: *mut *mut c_void);

    fn snprintf(s: *mut c_char, maxlen: usize, format: *const c_char, ...) -> c_int;
    fn strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char;

    fn ui_browser__show(uib: *mut ui_browser, title: *const c_char, help: *const c_char) -> c_int;
    fn ui_browser__hide(uib: *mut ui_browser);
    fn ui_browser__run(uib: *mut ui_browser, delay_secs: c_int) -> c_int;
    fn ui_browser__gotorc(uib: *mut ui_browser, y: c_int, x: c_int);
    fn ui_browser__gotorc_title(uib: *mut ui_browser, y: c_int, x: c_int);
    fn ui_browser__set_color(uib: *mut ui_browser, color: c_int);
    fn ui_browser__set_percent_color(uib: *mut ui_browser, percent: c_double, current: bool);
    fn ui_browser__printf(uib: *mut ui_browser, format: *const c_char, ...) -> c_int;
    fn ui_browser__write_nstring(uib: *mut ui_browser, msg: *const c_char, width: c_int);
    fn ui_browser__is_current_entry(uib: *mut ui_browser, row: c_int) -> bool;
    fn ui_browser__help_window(uib: *mut ui_browser, text: *const c_char);
    fn ui_browser__warn_unhandled_hotkey(
        uib: *mut ui_browser,
        key: c_int,
        delay_secs: c_int,
        msg: *const c_char,
    );

    fn ui_helpline__push(msg: *const c_char);

    fn hists_to_evsel(hists: *mut hists) -> *mut evsel;
    fn evsel__hists(evsel: *mut evsel) -> *mut hists;
    fn evsel__is_group_event(evsel: *mut evsel) -> bool;

    fn perf_list_init(head: *mut list_head);
    fn perf_list_add_tail(new_: *mut list_head, head: *mut list_head);
    fn perf_list_del_init(entry: *mut list_head);
    fn perf_list_empty(head: *const list_head) -> bool;
    fn perf_for_each_group_evsel_next(pos: *mut evsel, leader: *mut evsel) -> *mut evsel;
}

macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

unsafe fn container_of_browser(uib: *mut ui_browser) -> *mut annotated_data_browser {
    uib as *mut annotated_data_browser
}

unsafe fn container_of_entry(node: *mut list_head) -> *mut browser_entry {
    node as *mut browser_entry
}

unsafe fn container_of_member(node: *mut list_head) -> *mut annotated_member {
    node as *mut annotated_member
}

unsafe fn list_first_entry_browser(head: *mut list_head) -> *mut browser_entry {
    container_of_entry((*head).next)
}

unsafe fn list_last_entry_browser(head: *mut list_head) -> *mut browser_entry {
    container_of_entry((*head).prev)
}

unsafe fn list_next_entry_browser(entry: *mut browser_entry) -> *mut browser_entry {
    container_of_entry((*entry).node.next)
}

unsafe fn list_prev_entry_browser(entry: *mut browser_entry) -> *mut browser_entry {
    container_of_entry((*entry).node.prev)
}

unsafe fn list_first_entry_member(head: *mut list_head) -> *mut annotated_member {
    container_of_member((*head).next)
}

unsafe fn list_next_entry_member(entry: *mut annotated_member) -> *mut annotated_member {
    container_of_member((*entry).node.next)
}

unsafe fn get_browser(uib: *mut ui_browser) -> *mut annotated_data_browser {
    container_of_browser(uib)
}

unsafe fn update_hist_entry(dst: *mut type_hist_entry, src: *mut type_hist_entry) {
    (*dst).nr_samples += (*src).nr_samples;
    (*dst).period += (*src).period;
}

unsafe fn get_member_overhead(
    adt: *mut annotated_data_type,
    entry: *mut browser_entry,
    leader: *mut evsel,
) -> c_int {
    let member = (*entry).data;

    for i in 0..(*member).size {
        let offset = (*member).offset + i;
        let mut k = 0;
        let mut evsel = perf_for_each_group_evsel_next(ptr::null_mut(), leader);

        while !evsel.is_null() {
            if symbol_conf.skip_empty && (*evsel__hists(evsel)).stats.nr_samples == 0 {
                evsel = perf_for_each_group_evsel_next(evsel, leader);
                continue;
            }

            let h = *(*adt).histograms.add((*evsel).core.idx as usize);
            update_hist_entry((*entry).hists.add(k as usize), (*h).addr.add(offset as usize));
            k += 1;
            evsel = perf_for_each_group_evsel_next(evsel, leader);
        }
    }
    0
}

unsafe fn add_child_entries(
    browser: *mut annotated_data_browser,
    parent: *mut browser_entry,
    adt: *mut annotated_data_type,
    member: *mut annotated_member,
    evsel: *mut evsel,
    indent: c_int,
) -> c_int {
    let entry = zalloc(mem::size_of::<browser_entry>()) as *mut browser_entry;
    if entry.is_null() {
        return -1;
    }

    (*entry).hists = calloc(
        (*browser).nr_events as usize,
        mem::size_of::<type_hist_entry>(),
    ) as *mut type_hist_entry;
    if (*entry).hists.is_null() {
        free(entry as *mut c_void);
        return -1;
    }

    (*entry).data = member;
    (*entry).parent = parent;
    (*entry).indent = indent;
    if get_member_overhead(adt, entry, evsel) < 0 {
        free(entry as *mut c_void);
        return -1;
    }

    perf_list_init(&mut (*entry).children);
    let parent_list = if !parent.is_null() {
        &mut (*parent).children
    } else {
        &mut (*browser).entries
    };

    perf_list_add_tail(&mut (*entry).node, parent_list);

    let mut node = (*member).children.next;
    while node != &mut (*member).children {
        let pos = container_of_member(node);
        let nr = add_child_entries(browser, entry, adt, pos, evsel, indent + 1);
        if nr < 0 {
            return nr;
        }
        node = (*node).next;
    }

    /* add an entry for the closing bracket ("}") */
    if !perf_list_empty(&(*member).children) {
        let bracket = zalloc(mem::size_of::<browser_entry>()) as *mut browser_entry;
        if bracket.is_null() {
            return -1;
        }

        (*bracket).indent = indent;
        (*bracket).parent = entry;
        (*bracket).folded = true;
        (*bracket).nr_entries = 1;

        perf_list_init(&mut (*bracket).children);
        perf_list_add_tail(&mut (*bracket).node, &mut (*entry).children);
    }

    /* fold child entries by default */
    (*entry).folded = true;
    (*entry).nr_entries = 1;
    0
}

unsafe fn count_visible_entries(browser: *mut annotated_data_browser) -> u32 {
    let mut nr = 0;
    let mut node = (*browser).entries.next;

    while node != &mut (*browser).entries {
        let entry = container_of_entry(node);
        nr += (*entry).nr_entries;
        node = (*node).next;
    }

    nr as u32
}

unsafe fn annotated_data_browser__collect_entries(browser: *mut annotated_data_browser) -> c_int {
    let he = (*browser).b.priv_ as *mut hist_entry;
    let adt = (*he).mem_type;
    let evsel = hists_to_evsel((*he).hists);

    perf_list_init(&mut (*browser).entries);

    add_child_entries(
        browser,
        ptr::null_mut(),
        adt,
        &mut (*adt).self_,
        evsel,
        0,
    );

    (*browser).b.entries = &mut (*browser).entries;
    (*browser).b.nr_entries = count_visible_entries(browser);
    0
}

unsafe fn annotated_data_browser__delete_entries(browser: *mut annotated_data_browser) {
    let mut pos = (*browser).entries.next;

    while pos != &mut (*browser).entries {
        let tmp = (*pos).next;
        let entry = container_of_entry(pos);
        perf_list_del_init(&mut (*entry).node);
        zfree(&mut (*entry).hists as *mut *mut type_hist_entry as *mut *mut c_void);
        free(entry as *mut c_void);
        pos = tmp;
    }
}

unsafe fn get_first_child(entry: *mut browser_entry) -> *mut browser_entry {
    if perf_list_empty(&(*entry).children) {
        return ptr::null_mut();
    }

    list_first_entry_browser(&mut (*entry).children)
}

unsafe fn get_last_child(entry: *mut browser_entry) -> *mut browser_entry {
    if perf_list_empty(&(*entry).children) {
        return ptr::null_mut();
    }

    list_last_entry_browser(&mut (*entry).children)
}

unsafe fn is_first_child(entry: *mut browser_entry) -> bool {
    /* This will be checked in a different way */
    if (*entry).parent.is_null() {
        return false;
    }

    get_first_child((*entry).parent) == entry
}

unsafe fn is_last_child(entry: *mut browser_entry) -> bool {
    /* This will be checked in a different way */
    if (*entry).parent.is_null() {
        return false;
    }

    get_last_child((*entry).parent) == entry
}

unsafe fn browser__prev_entry(
    uib: *mut ui_browser,
    mut entry: *mut browser_entry,
) -> *mut browser_entry {
    let browser = get_browser(uib);
    let first = list_first_entry_browser(&mut (*browser).entries);

    while entry != first {
        if is_first_child(entry) {
            entry = (*entry).parent;
        } else {
            entry = list_prev_entry_browser(entry);
            while !(*entry).folded {
                entry = get_last_child(entry);
            }
        }

        if (*uib).filter.is_none() || !((*uib).filter.unwrap())(uib, &mut (*entry).node) {
            return entry;
        }
    }
    first
}

unsafe fn browser__next_entry(
    uib: *mut ui_browser,
    mut entry: *mut browser_entry,
) -> *mut browser_entry {
    let browser = get_browser(uib);
    let mut last = list_last_entry_browser(&mut (*browser).entries);
    while !(*last).folded {
        last = get_last_child(last);
    }

    while entry != last {
        if !(*entry).folded {
            entry = get_first_child(entry);
        } else {
            while is_last_child(entry) {
                entry = (*entry).parent;
            }

            entry = list_next_entry_browser(entry);
        }

        if (*uib).filter.is_none() || !((*uib).filter.unwrap())(uib, &mut (*entry).node) {
            return entry;
        }
    }
    last
}

unsafe extern "C" fn browser__seek(uib: *mut ui_browser, mut offset: off_t, whence: c_int) {
    let browser = get_browser(uib);
    let mut entry: *mut browser_entry;

    if (*uib).nr_entries == 0 {
        return;
    }

    match whence {
        SEEK_SET => {
            entry = list_first_entry_browser(&mut (*browser).entries);
            if (*uib).filter.is_some() && ((*uib).filter.unwrap())(uib, &mut (*entry).node) {
                entry = browser__next_entry(uib, entry);
            }
        }
        SEEK_CUR => {
            entry = container_of_entry((*uib).top);
        }
        SEEK_END => {
            entry = list_last_entry_browser(&mut (*browser).entries);
            while !(*entry).folded {
                entry = get_last_child(entry);
            }
            if (*uib).filter.is_some() && ((*uib).filter.unwrap())(uib, &mut (*entry).node) {
                entry = browser__prev_entry(uib, entry);
            }
        }
        _ => return,
    }

    assert!(!entry.is_null());

    if offset > 0 {
        while offset != 0 {
            offset -= 1;
            entry = browser__next_entry(uib, entry);
        }
    } else {
        while offset != 0 {
            offset += 1;
            entry = browser__prev_entry(uib, entry);
        }
    }

    (*uib).top = &mut (*entry).node;
}

unsafe extern "C" fn browser__refresh(uib: *mut ui_browser) -> c_uint {
    let browser = get_browser(uib);
    let mut row = 0;

    if (*uib).top.is_null() || (*uib).top == (*uib).entries {
        browser__seek(uib, SEEK_SET as off_t, 0);
    }

    let mut entry = container_of_entry((*uib).top);

    loop {
        if (*uib).filter.is_none() || !((*uib).filter.unwrap())(uib, &mut (*entry).node) {
            ui_browser__gotorc(uib, row, 0);
            ((*uib).write.unwrap())(uib, entry as *mut c_void, row);
            if (*uib).top_idx + row as u32 == (*uib).index {
                (*browser).curr = entry;
            }
            row += 1;
            if row == (*uib).rows {
                break;
            }
        }
        let next = browser__next_entry(uib, entry);
        if next == entry {
            break;
        }

        entry = next;
    }

    row as c_uint
}

unsafe fn browser__show(uib: *mut ui_browser) -> c_int {
    let he = (*uib).priv_ as *mut hist_entry;
    let adt = (*he).mem_type;
    let browser = get_browser(uib);
    let help = cstr!("Press 'h' for help on key bindings");
    let mut title = [0 as c_char; 256];

    snprintf(
        title.as_mut_ptr(),
        title.len(),
        cstr!("Annotate type: '%s' (%d samples)"),
        (*adt).self_.type_name,
        (*he).stat.nr_events,
    );

    if ui_browser__show(uib, title.as_ptr(), help) < 0 {
        return -1;
    }

    /* second line header */
    ui_browser__gotorc_title(uib, 0, 0);
    ui_browser__set_color(uib, HE_COLORSET_ROOT);

    if symbol_conf.show_total_period {
        strcpy(title.as_mut_ptr(), cstr!("Period"));
    } else if symbol_conf.show_nr_samples {
        strcpy(title.as_mut_ptr(), cstr!("Samples"));
    } else {
        strcpy(title.as_mut_ptr(), cstr!("Percent"));
    }

    ui_browser__printf(
        uib,
        cstr!("%*s %10s %10s %10s  %s"),
        2 + 11 * ((*browser).nr_events - 1),
        cstr!(""),
        title.as_ptr(),
        cstr!("Offset"),
        cstr!("Size"),
        cstr!("Field"),
    );
    ui_browser__write_nstring(uib, cstr!(""), (*uib).width);
    0
}

unsafe fn browser__write_overhead(
    uib: *mut ui_browser,
    total: *mut type_hist,
    hist: *mut type_hist_entry,
    row: c_int,
) {
    let period = (*hist).period;
    let percent = if (*total).period != 0 {
        100.0 * period as c_double / (*total).period as c_double
    } else {
        0.0
    };
    let current = ui_browser__is_current_entry(uib, row);
    let nr_samples = 0;

    ui_browser__set_percent_color(uib, percent, current);

    if symbol_conf.show_total_period {
        ui_browser__printf(uib, cstr!(" %10llu"), period as c_ulonglong);
    } else if symbol_conf.show_nr_samples {
        ui_browser__printf(uib, cstr!(" %10d"), nr_samples);
    } else {
        ui_browser__printf(uib, cstr!(" %10.2f"), percent);
    }

    ui_browser__set_percent_color(uib, 0.0, current);
}

unsafe extern "C" fn browser__write(uib: *mut ui_browser, entry: *mut c_void, row: c_int) {
    let browser = get_browser(uib);
    let be = entry as *mut browser_entry;
    let member = (*be).data;
    let he = (*uib).priv_ as *mut hist_entry;
    let adt = (*he).mem_type;
    let leader = hists_to_evsel((*he).hists);
    let mut idx = 0;
    let current = ui_browser__is_current_entry(uib, row);

    if member.is_null() {
        /* print the closing bracket */
        ui_browser__set_percent_color(uib, 0.0, current);
        ui_browser__printf(uib, cstr!("%c "), NOCHLD_SIGN);
        ui_browser__write_nstring(uib, cstr!(""), 11 * (*browser).nr_events);
        ui_browser__printf(
            uib,
            cstr!(" %10s %10s  %*s};"),
            cstr!(""),
            cstr!(""),
            (*be).indent * 4,
            cstr!(""),
        );
        ui_browser__write_nstring(uib, cstr!(""), (*uib).width);
        return;
    }

    ui_browser__set_percent_color(uib, 0.0, current);

    if !perf_list_empty(&(*be).children) {
        ui_browser__printf(
            uib,
            cstr!("%c "),
            if (*be).folded { FOLDED_SIGN } else { UNFOLD_SIGN },
        );
    } else {
        ui_browser__printf(uib, cstr!("%c "), NOCHLD_SIGN);
    }

    /* print the number */
    let mut evsel = perf_for_each_group_evsel_next(ptr::null_mut(), leader);
    while !evsel.is_null() {
        let h = *(*adt).histograms.add((*evsel).core.idx as usize);

        if symbol_conf.skip_empty && (*evsel__hists(evsel)).stats.nr_samples == 0 {
            evsel = perf_for_each_group_evsel_next(evsel, leader);
            continue;
        }

        browser__write_overhead(uib, h, (*be).hists.add(idx as usize), row);
        idx += 1;
        evsel = perf_for_each_group_evsel_next(evsel, leader);
    }

    /* print type info */
    if (*be).indent == 0 && (*member).var_name.is_null() {
        ui_browser__printf(
            uib,
            cstr!(" %#10x %#10x  %s%s"),
            (*member).offset,
            (*member).size,
            (*member).type_name,
            if perf_list_empty(&(*member).children) || (*be).folded {
                cstr!(";")
            } else {
                cstr!(" {")
            },
        );
    } else {
        ui_browser__printf(
            uib,
            cstr!(" %#10x %#10x  %*s%s\t%s%s"),
            (*member).offset,
            (*member).size,
            (*be).indent * 4,
            cstr!(""),
            (*member).type_name,
            if (*member).var_name.is_null() {
                cstr!("")
            } else {
                (*member).var_name
            },
            if perf_list_empty(&(*member).children) || (*be).folded {
                cstr!(";")
            } else {
                cstr!(" {")
            },
        );
    }
    /* fill the rest */
    ui_browser__write_nstring(uib, cstr!(""), (*uib).width);
}

unsafe fn annotated_data_browser__fold(
    browser: *mut annotated_data_browser,
    entry: *mut browser_entry,
    recursive: bool,
) {
    let _ = browser;
    if perf_list_empty(&(*entry).children) {
        return;
    }
    if (*entry).folded && !recursive {
        return;
    }

    if recursive {
        let mut node = (*entry).children.next;
        while node != &mut (*entry).children {
            let child = container_of_entry(node);
            annotated_data_browser__fold(browser, child, true);
            node = (*node).next;
        }
    }

    (*entry).nr_entries = 1;
    (*entry).folded = true;
}

unsafe fn annotated_data_browser__unfold(
    browser: *mut annotated_data_browser,
    entry: *mut browser_entry,
    recursive: bool,
) {
    let _ = browser;
    let mut nr_entries;

    if perf_list_empty(&(*entry).children) {
        return;
    }
    if !(*entry).folded && !recursive {
        return;
    }

    nr_entries = 1; /* for self */
    let mut node = (*entry).children.next;
    while node != &mut (*entry).children {
        let child = container_of_entry(node);
        if recursive {
            annotated_data_browser__unfold(browser, child, true);
        }

        nr_entries += (*child).nr_entries;
        node = (*node).next;
    }

    (*entry).nr_entries = nr_entries;
    (*entry).folded = false;
}

unsafe fn annotated_data_browser__toggle_fold(
    browser: *mut annotated_data_browser,
    recursive: bool,
) {
    let curr = (*browser).curr;
    let mut parent;

    parent = (*curr).parent;
    while !parent.is_null() {
        (*parent).nr_entries -= (*curr).nr_entries;
        parent = (*parent).parent;
    }
    (*browser).b.nr_entries -= (*curr).nr_entries as u32;

    if (*curr).folded {
        annotated_data_browser__unfold(browser, curr, recursive);
    } else {
        annotated_data_browser__fold(browser, curr, recursive);
    }

    parent = (*curr).parent;
    while !parent.is_null() {
        (*parent).nr_entries += (*curr).nr_entries;
        parent = (*parent).parent;
    }
    (*browser).b.nr_entries += (*curr).nr_entries as u32;

    assert!((*browser).b.nr_entries == count_visible_entries(browser));
}

unsafe fn annotated_data_browser__run(
    browser: *mut annotated_data_browser,
    evsel: *mut evsel,
    hbt: *mut hist_browser_timer,
) -> c_int {
    let _ = evsel;
    let delay_secs = if !hbt.is_null() { (*hbt).refresh } else { 0 };
    let mut key;

    if browser__show(&mut (*browser).b) < 0 {
        return -1;
    }

    loop {
        key = ui_browser__run(&mut (*browser).b, delay_secs);

        match key {
            K_TIMER => {
                if !hbt.is_null() {
                    if let Some(timer) = (*hbt).timer {
                        timer((*hbt).arg);
                    }
                }
                continue;
            }
            K_F1 | 104 => {
                ui_browser__help_window(
                    &mut (*browser).b,
                    cstr!(
                        "UP/DOWN/PGUP\n\
                         PGDN/SPACE    Navigate\n\
                         </>           Move to prev/next symbol\n\
                         e             Expand/Collapse current entry\n\
                         E             Expand/Collapse all children of the current\n\
                         q/ESC/CTRL+C  Exit\n\n"
                    ),
                );
                continue;
            }
            101 => {
                annotated_data_browser__toggle_fold(browser, false);
            }
            69 => {
                annotated_data_browser__toggle_fold(browser, true);
            }
            K_LEFT | 60 | 62 | K_ESC | 113 => {
                break;
            }
            x if x == CTRL(b'c') => {
                break;
            }
            _ => {
                ui_browser__warn_unhandled_hotkey(
                    &mut (*browser).b,
                    key,
                    delay_secs,
                    cstr!(", use 'h'/F1 to see actions"),
                );
                continue;
            }
        }
    }

    ui_browser__hide(&mut (*browser).b);
    key
}

#[no_mangle]
pub unsafe extern "C" fn hist_entry__annotate_data_tui(
    he: *mut hist_entry,
    evsel: *mut evsel,
    hbt: *mut hist_browser_timer,
) -> c_int {
    let mut browser = annotated_data_browser {
        b: ui_browser {
            refresh: Some(browser__refresh),
            seek: Some(browser__seek),
            write: Some(browser__write),
            priv_: he as *mut c_void,
            extra_title_lines: 1,
            entries: ptr::null_mut(),
            nr_entries: 0,
            top: ptr::null_mut(),
            top_idx: 0,
            index: 0,
            rows: 0,
            width: 0,
            filter: None,
        },
        entries: list_head {
            next: ptr::null_mut(),
            prev: ptr::null_mut(),
        },
        curr: ptr::null_mut(),
        nr_events: 1,
    };
    let mut ret;

    ui_helpline__push(cstr!("Press ESC to exit"));

    if evsel__is_group_event(evsel) {
        let mut nr = 0;
        let mut pos = perf_for_each_group_evsel_next(ptr::null_mut(), evsel);

        while !pos.is_null() {
            if !symbol_conf.skip_empty || (*evsel__hists(pos)).stats.nr_samples != 0 {
                nr += 1;
            }
            pos = perf_for_each_group_evsel_next(pos, evsel);
        }
        browser.nr_events = nr;
    }

    ret = annotated_data_browser__collect_entries(&mut browser);
    if ret < 0 {
        annotated_data_browser__delete_entries(&mut browser);
        return ret;
    }

    /* To get the top and current entry */
    browser__refresh(&mut browser.b);
    /* Show the first-level child entries by default */
    annotated_data_browser__toggle_fold(&mut browser, false);

    ret = annotated_data_browser__run(&mut browser, evsel, hbt);

    annotated_data_browser__delete_entries(&mut browser);

    ret
}
