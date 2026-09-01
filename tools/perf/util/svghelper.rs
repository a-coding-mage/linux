// SPDX-License-Identifier: GPL-2.0-only
/*
 * svghelper.c - helper functions for outputting svg
 *
 * (C) Copyright 2009 Intel Corporation
 *
 * Authors:
 *     Arjan van de Ven <arjan@linux.intel.com>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use core::ffi::{c_char, c_double, c_int, c_uint, c_ulong, c_ulonglong, c_void};
use core::ptr;

type u64 = u64;

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_cpu {
    pub cpu: c_int,
}

#[repr(C)]
pub struct perf_cpu_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_env {
    pub nr_cpus_online: c_int,
    pub nr_sibling_cores: c_int,
    pub nr_sibling_threads: c_int,
    pub sibling_cores: *mut c_char,
    pub sibling_threads: *mut c_char,
}

unsafe extern "C" {
    static mut stderr: *mut FILE;

    fn fopen(filename: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fclose(stream: *mut FILE) -> c_int;
    fn fgets(s: *mut c_char, size: c_int, stream: *mut FILE) -> *mut c_char;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn sprintf(s: *mut c_char, format: *const c_char, ...) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn malloc(size: usize) -> *mut c_void;
    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn strlen(s: *const c_char) -> usize;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn strcasestr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn strlcpy(dst: *mut c_char, src: *const c_char, siz: usize) -> usize;
    fn strtoull(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulonglong;

    fn perf_cpu_map__new(cpu_list: *const c_char) -> *mut perf_cpu_map;
    fn perf_cpu_map__put(map: *mut perf_cpu_map);
    fn perf_cpu_map__nr(map: *const perf_cpu_map) -> c_int;
    fn perf_cpu_map__cpu(map: *const perf_cpu_map, idx: c_int) -> perf_cpu;
}

static mut first_time: u64 = 0;
static mut last_time: u64 = 0;
static mut turbo_frequency: u64 = 0;
static mut max_freq: u64 = 0;

const SLOT_MULT: c_double = 30.0;
const SLOT_HEIGHT: c_double = 25.0;
const SLOT_HALF: c_double = SLOT_HEIGHT / 2.0;

#[unsafe(no_mangle)]
pub static mut svg_page_width: c_int = 1000;
#[unsafe(no_mangle)]
pub static mut svg_highlight: u64 = 0;
#[unsafe(no_mangle)]
pub static mut svg_highlight_name: *const c_char = ptr::null();

const MIN_TEXT_SIZE: c_double = 0.01;
const NSEC_PER_USEC: u64 = 1000;
const NSEC_PER_MSEC: u64 = 1000000;
const MAX_NR_CPUS: usize = 8192;
const BITS_PER_LONG: usize = core::mem::size_of::<c_ulong>() * 8;
const CPUMASK_WORDS: usize = (MAX_NR_CPUS + BITS_PER_LONG - 1) / BITS_PER_LONG;

static mut total_height: u64 = 0;
static mut svgfile: *mut FILE = ptr::null_mut();

unsafe fn cpu2slot(cpu: c_int) -> c_double {
    (2 * cpu + 1) as c_double
}

static mut topology_map: *mut c_int = ptr::null_mut();
static mut topology_map_size: c_int = 0;

unsafe fn cpu2y(cpu: c_int) -> c_double {
    unsafe {
        if !topology_map.is_null() && cpu >= 0 && cpu < topology_map_size {
            return cpu2slot(*topology_map.add(cpu as usize)) * SLOT_MULT;
        }
        cpu2slot(cpu) * SLOT_MULT
    }
}

unsafe fn time2pixels(__time: u64) -> c_double {
    unsafe {
        1.0 * svg_page_width as c_double * (__time - first_time) as c_double
            / (last_time - first_time) as c_double
    }
}

/*
 * Round text sizes so that the svg viewer only needs a discrete
 * number of renderings of the font
 */
fn round_text_size(size: c_double) -> c_double {
    let mut loop_: c_int = 100;
    let mut target: c_double = 10.0;

    if size >= 10.0 {
        return size;
    }
    while loop_ != 0 {
        loop_ -= 1;
        if size >= target {
            return target;
        }
        target = target / 2.0;
    }
    size
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn open_svg(filename: *const c_char, cpus: c_int, rows: c_int, start: u64, end: u64) {
    unsafe {
        let mut new_width: c_int;

        svgfile = fopen(filename, c"w".as_ptr());
        if svgfile.is_null() {
            fprintf(stderr, c"Cannot open %s for output\n".as_ptr(), filename);
            return;
        }
        first_time = start;
        first_time = first_time / 100000000 * 100000000;
        last_time = end;

        /*
         * if the recording is short, we default to a width of 1000, but
         * for longer recordings we want at least 200 units of width per second
         */
        new_width = ((last_time - first_time) / 5000000) as c_int;

        if new_width > svg_page_width {
            svg_page_width = new_width;
        }

        total_height = ((1.0 + rows as c_double + cpu2slot(cpus)) * SLOT_MULT) as u64;
        fprintf(svgfile, c"<?xml version=\"1.0\" standalone=\"no\"?> \n".as_ptr());
        fprintf(svgfile, c"<!DOCTYPE svg SYSTEM \"http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd\">\n".as_ptr());
        fprintf(svgfile, c"<svg width=\"%i\" height=\"%lu\" version=\"1.1\" xmlns=\"http://www.w3.org/2000/svg\">\n".as_ptr(), svg_page_width, total_height);

        fprintf(svgfile, c"<defs>\n  <style type=\"text/css\">\n    <![CDATA[\n".as_ptr());
        fprintf(svgfile, c"      rect          { stroke-width: 1; }\n".as_ptr());
        fprintf(svgfile, c"      rect.process  { fill:rgb(180,180,180); fill-opacity:0.9; stroke-width:1;   stroke:rgb(  0,  0,  0); } \n".as_ptr());
        fprintf(svgfile, c"      rect.process2 { fill:rgb(180,180,180); fill-opacity:0.9; stroke-width:0;   stroke:rgb(  0,  0,  0); } \n".as_ptr());
        fprintf(svgfile, c"      rect.process3 { fill:rgb(180,180,180); fill-opacity:0.5; stroke-width:0;   stroke:rgb(  0,  0,  0); } \n".as_ptr());
        fprintf(svgfile, c"      rect.sample   { fill:rgb(  0,  0,255); fill-opacity:0.8; stroke-width:0;   stroke:rgb(  0,  0,  0); } \n".as_ptr());
        fprintf(svgfile, c"      rect.sample_hi{ fill:rgb(255,128,  0); fill-opacity:0.8; stroke-width:0;   stroke:rgb(  0,  0,  0); } \n".as_ptr());
        fprintf(svgfile, c"      rect.error    { fill:rgb(255,  0,  0); fill-opacity:0.5; stroke-width:0;   stroke:rgb(  0,  0,  0); } \n".as_ptr());
        fprintf(svgfile, c"      rect.net      { fill:rgb(  0,128,  0); fill-opacity:0.5; stroke-width:0;   stroke:rgb(  0,  0,  0); } \n".as_ptr());
        fprintf(svgfile, c"      rect.disk     { fill:rgb(  0,  0,255); fill-opacity:0.5; stroke-width:0;   stroke:rgb(  0,  0,  0); } \n".as_ptr());
        fprintf(svgfile, c"      rect.sync     { fill:rgb(128,128,  0); fill-opacity:0.5; stroke-width:0;   stroke:rgb(  0,  0,  0); } \n".as_ptr());
        fprintf(svgfile, c"      rect.poll     { fill:rgb(  0,128,128); fill-opacity:0.2; stroke-width:0;   stroke:rgb(  0,  0,  0); } \n".as_ptr());
        fprintf(svgfile, c"      rect.blocked  { fill:rgb(255,  0,  0); fill-opacity:0.5; stroke-width:0;   stroke:rgb(  0,  0,  0); } \n".as_ptr());
        fprintf(svgfile, c"      rect.waiting  { fill:rgb(224,214,  0); fill-opacity:0.8; stroke-width:0;   stroke:rgb(  0,  0,  0); } \n".as_ptr());
        fprintf(svgfile, c"      rect.WAITING  { fill:rgb(255,214, 48); fill-opacity:0.6; stroke-width:0;   stroke:rgb(  0,  0,  0); } \n".as_ptr());
        fprintf(svgfile, c"      rect.cpu      { fill:rgb(192,192,192); fill-opacity:0.2; stroke-width:0.5; stroke:rgb(128,128,128); } \n".as_ptr());
        fprintf(svgfile, c"      rect.pstate   { fill:rgb(128,128,128); fill-opacity:0.8; stroke-width:0; } \n".as_ptr());
        fprintf(svgfile, c"      rect.c1       { fill:rgb(255,214,214); fill-opacity:0.5; stroke-width:0; } \n".as_ptr());
        fprintf(svgfile, c"      rect.c2       { fill:rgb(255,172,172); fill-opacity:0.5; stroke-width:0; } \n".as_ptr());
        fprintf(svgfile, c"      rect.c3       { fill:rgb(255,130,130); fill-opacity:0.5; stroke-width:0; } \n".as_ptr());
        fprintf(svgfile, c"      rect.c4       { fill:rgb(255, 88, 88); fill-opacity:0.5; stroke-width:0; } \n".as_ptr());
        fprintf(svgfile, c"      rect.c5       { fill:rgb(255, 44, 44); fill-opacity:0.5; stroke-width:0; } \n".as_ptr());
        fprintf(svgfile, c"      rect.c6       { fill:rgb(255,  0,  0); fill-opacity:0.5; stroke-width:0; } \n".as_ptr());
        fprintf(svgfile, c"      line.pstate   { stroke:rgb(255,255,  0); stroke-opacity:0.8; stroke-width:2; } \n".as_ptr());
        fprintf(svgfile, c"    ]]>\n   </style>\n</defs>\n".as_ptr());
    }
}

fn normalize_height(height: c_double) -> c_double {
    if height < 0.25 {
        0.25
    } else if height < 0.50 {
        0.50
    } else if height < 0.75 {
        0.75
    } else {
        0.100
    }
}

unsafe fn cstr_or_question(s: *mut c_char) -> *mut c_char {
    if s.is_null() { c"?".as_ptr() as *mut c_char } else { s }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn svg_ubox(Yslot: c_int, start: u64, end: u64, mut height: c_double, type_: *const c_char, fd: c_int, err: c_int, merges: c_int) {
    unsafe {
        let w = time2pixels(end) - time2pixels(start);
        height = normalize_height(height);
        if svgfile.is_null() { return; }
        fprintf(svgfile, c"<g>\n".as_ptr());
        fprintf(svgfile, c"<title>fd=%d error=%d merges=%d</title>\n".as_ptr(), fd, err, merges);
        fprintf(svgfile, c"<rect x=\"%.8f\" width=\"%.8f\" y=\"%.1f\" height=\"%.1f\" class=\"%s\"/>\n".as_ptr(), time2pixels(start), w, Yslot as c_double * SLOT_MULT, SLOT_HALF * height, type_);
        fprintf(svgfile, c"</g>\n".as_ptr());
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn svg_lbox(Yslot: c_int, start: u64, end: u64, mut height: c_double, type_: *const c_char, fd: c_int, err: c_int, merges: c_int) {
    unsafe {
        let w = time2pixels(end) - time2pixels(start);
        height = normalize_height(height);
        if svgfile.is_null() { return; }
        fprintf(svgfile, c"<g>\n".as_ptr());
        fprintf(svgfile, c"<title>fd=%d error=%d merges=%d</title>\n".as_ptr(), fd, err, merges);
        fprintf(svgfile, c"<rect x=\"%.8f\" width=\"%.8f\" y=\"%.1f\" height=\"%.1f\" class=\"%s\"/>\n".as_ptr(), time2pixels(start), w, Yslot as c_double * SLOT_MULT + SLOT_HEIGHT - SLOT_HALF * height, SLOT_HALF * height, type_);
        fprintf(svgfile, c"</g>\n".as_ptr());
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn svg_fbox(Yslot: c_int, start: u64, end: u64, mut height: c_double, type_: *const c_char, fd: c_int, err: c_int, merges: c_int) {
    unsafe {
        let w = time2pixels(end) - time2pixels(start);
        height = normalize_height(height);
        if svgfile.is_null() { return; }
        fprintf(svgfile, c"<g>\n".as_ptr());
        fprintf(svgfile, c"<title>fd=%d error=%d merges=%d</title>\n".as_ptr(), fd, err, merges);
        fprintf(svgfile, c"<rect x=\"%.8f\" width=\"%.8f\" y=\"%.1f\" height=\"%.1f\" class=\"%s\"/>\n".as_ptr(), time2pixels(start), w, Yslot as c_double * SLOT_MULT + SLOT_HEIGHT - SLOT_HEIGHT * height, SLOT_HEIGHT * height, type_);
        fprintf(svgfile, c"</g>\n".as_ptr());
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn svg_box(Yslot: c_int, start: u64, end: u64, type_: *const c_char) {
    unsafe {
        if svgfile.is_null() { return; }
        fprintf(svgfile, c"<rect x=\"%.8f\" width=\"%.8f\" y=\"%.1f\" height=\"%.1f\" class=\"%s\"/>\n".as_ptr(), time2pixels(start), time2pixels(end) - time2pixels(start), Yslot as c_double * SLOT_MULT, SLOT_HEIGHT, type_);
    }
}

unsafe fn time_to_string(duration: u64) -> *mut c_char {
    static mut TEXT: [c_char; 80] = [0; 80];
    unsafe {
        TEXT[0] = 0;
        if duration < NSEC_PER_USEC {
            return &raw mut TEXT as *mut c_char;
        }
        if duration < NSEC_PER_MSEC {
            sprintf((&raw mut TEXT) as *mut c_char, c"%.1f us".as_ptr(), duration as c_double / NSEC_PER_USEC as c_double);
            return (&raw mut TEXT) as *mut c_char;
        }
        sprintf((&raw mut TEXT) as *mut c_char, c"%.1f ms".as_ptr(), duration as c_double / NSEC_PER_MSEC as c_double);
        (&raw mut TEXT) as *mut c_char
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn svg_blocked(Yslot: c_int, cpu: c_int, start: u64, end: u64, backtrace: *const c_char) {
    unsafe {
        if svgfile.is_null() { return; }
        fprintf(svgfile, c"<g>\n".as_ptr());
        fprintf(svgfile, c"<title>#%d blocked %s</title>\n".as_ptr(), cpu, time_to_string(end - start));
        if !backtrace.is_null() { fprintf(svgfile, c"<desc>Blocked on:\n%s</desc>\n".as_ptr(), backtrace); }
        svg_box(Yslot, start, end, c"blocked".as_ptr());
        fprintf(svgfile, c"</g>\n".as_ptr());
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn svg_running(Yslot: c_int, cpu: c_int, start: u64, end: u64, backtrace: *const c_char) {
    unsafe {
        if svgfile.is_null() { return; }
        let type_ = if svg_highlight != 0 && end - start > svg_highlight { c"sample_hi".as_ptr() } else { c"sample".as_ptr() };
        fprintf(svgfile, c"<g>\n".as_ptr());
        fprintf(svgfile, c"<title>#%d running %s</title>\n".as_ptr(), cpu, time_to_string(end - start));
        if !backtrace.is_null() { fprintf(svgfile, c"<desc>Switched because:\n%s</desc>\n".as_ptr(), backtrace); }
        fprintf(svgfile, c"<rect x=\"%.8f\" width=\"%.8f\" y=\"%.1f\" height=\"%.1f\" class=\"%s\"/>\n".as_ptr(), time2pixels(start), time2pixels(end) - time2pixels(start), Yslot as c_double * SLOT_MULT, SLOT_HEIGHT, type_);
        let mut text_size = time2pixels(end) - time2pixels(start);
        if cpu > 9 { text_size = text_size / 2.0; }
        if text_size > 1.25 { text_size = 1.25; }
        text_size = round_text_size(text_size);
        if text_size > MIN_TEXT_SIZE {
            fprintf(svgfile, c"<text x=\"%.8f\" y=\"%.8f\" font-size=\"%.8fpt\">%i</text>\n".as_ptr(), time2pixels(start), Yslot as c_double * SLOT_MULT + SLOT_HEIGHT - 1.0, text_size, cpu + 1);
        }
        fprintf(svgfile, c"</g>\n".as_ptr());
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn svg_waiting(Yslot: c_int, cpu: c_int, start: u64, end: u64, backtrace: *const c_char) {
    unsafe {
        if svgfile.is_null() { return; }
        let style = if end - start > 10 * NSEC_PER_MSEC { c"WAITING".as_ptr() } else { c"waiting".as_ptr() };
        let text = time_to_string(end - start);
        let mut font_size = 1.0 * (time2pixels(end) - time2pixels(start));
        if font_size > 3.0 { font_size = 3.0; }
        font_size = round_text_size(font_size);
        fprintf(svgfile, c"<g transform=\"translate(%.8f,%.8f)\">\n".as_ptr(), time2pixels(start), Yslot as c_double * SLOT_MULT);
        fprintf(svgfile, c"<title>#%d waiting %s</title>\n".as_ptr(), cpu, time_to_string(end - start));
        if !backtrace.is_null() { fprintf(svgfile, c"<desc>Waiting on:\n%s</desc>\n".as_ptr(), backtrace); }
        fprintf(svgfile, c"<rect x=\"0\" width=\"%.8f\" y=\"0\" height=\"%.1f\" class=\"%s\"/>\n".as_ptr(), time2pixels(end) - time2pixels(start), SLOT_HEIGHT, style);
        if font_size > MIN_TEXT_SIZE { fprintf(svgfile, c"<text transform=\"rotate(90)\" font-size=\"%.8fpt\"> %s</text>\n".as_ptr(), font_size, text); }
        fprintf(svgfile, c"</g>\n".as_ptr());
    }
}

unsafe fn cpu_model() -> *mut c_char {
    static mut CPU_M: [c_char; 255] = [0; 255];
    let mut buf: [c_char; 256] = [0; 256];
    unsafe {
        CPU_M[0] = 0;
        /* CPU type */
        let mut file = fopen(c"/proc/cpuinfo".as_ptr(), c"r".as_ptr());
        if !file.is_null() {
            while !fgets(buf.as_mut_ptr(), 255, file).is_null() {
                if !strcasestr(buf.as_mut_ptr(), c"model name".as_ptr()).is_null() {
                    strlcpy((&raw mut CPU_M) as *mut c_char, buf.as_mut_ptr().add(13), 255);
                    break;
                }
            }
            fclose(file);
        }

        /* CPU type */
        file = fopen(c"/sys/devices/system/cpu/cpu0/cpufreq/scaling_available_frequencies".as_ptr(), c"r".as_ptr());
        if !file.is_null() {
            while !fgets(buf.as_mut_ptr(), 255, file).is_null() {
                let freq = strtoull(buf.as_mut_ptr(), ptr::null_mut(), 10) as c_uint;
                if freq as u64 > max_freq { max_freq = freq as u64; }
            }
            fclose(file);
        }
        (&raw mut CPU_M) as *mut c_char
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn svg_cpu_box(cpu: c_int, __max_freq: u64, __turbo_freq: u64) {
    let mut cpu_string: [c_char; 80] = [0; 80];
    unsafe {
        if svgfile.is_null() { return; }
        max_freq = __max_freq;
        turbo_frequency = __turbo_freq;
        fprintf(svgfile, c"<g>\n".as_ptr());
        fprintf(svgfile, c"<rect x=\"%.8f\" width=\"%.8f\" y=\"%.1f\" height=\"%.1f\" class=\"cpu\"/>\n".as_ptr(), time2pixels(first_time), time2pixels(last_time) - time2pixels(first_time), cpu2y(cpu), SLOT_MULT + SLOT_HEIGHT);
        sprintf(cpu_string.as_mut_ptr(), c"CPU %i".as_ptr(), cpu);
        fprintf(svgfile, c"<text x=\"%.8f\" y=\"%.8f\">%s</text>\n".as_ptr(), 10.0 + time2pixels(first_time), cpu2y(cpu) + SLOT_HEIGHT / 2.0, cpu_string.as_ptr());
        fprintf(svgfile, c"<text transform=\"translate(%.8f,%.8f)\" font-size=\"1.25pt\">%s</text>\n".as_ptr(), 10.0 + time2pixels(first_time), cpu2y(cpu) + SLOT_MULT + SLOT_HEIGHT - 4.0, cpu_model());
        fprintf(svgfile, c"</g>\n".as_ptr());
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn svg_process(cpu: c_int, start: u64, end: u64, pid: c_int, name: *const c_char, backtrace: *const c_char) {
    unsafe {
        if svgfile.is_null() { return; }
        let type_ = if svg_highlight != 0 && end - start >= svg_highlight {
            c"sample_hi".as_ptr()
        } else if !svg_highlight_name.is_null() && !strstr(name, svg_highlight_name).is_null() {
            c"sample_hi".as_ptr()
        } else {
            c"sample".as_ptr()
        };
        fprintf(svgfile, c"<g transform=\"translate(%.8f,%.8f)\">\n".as_ptr(), time2pixels(start), cpu2y(cpu));
        fprintf(svgfile, c"<title>%d %s running %s</title>\n".as_ptr(), pid, name, time_to_string(end - start));
        if !backtrace.is_null() { fprintf(svgfile, c"<desc>Switched because:\n%s</desc>\n".as_ptr(), backtrace); }
        fprintf(svgfile, c"<rect x=\"0\" width=\"%.8f\" y=\"0\" height=\"%.1f\" class=\"%s\"/>\n".as_ptr(), time2pixels(end) - time2pixels(start), SLOT_MULT + SLOT_HEIGHT, type_);
        let mut width = time2pixels(end) - time2pixels(start);
        if width > 6.0 { width = 6.0; }
        width = round_text_size(width);
        if width > MIN_TEXT_SIZE { fprintf(svgfile, c"<text transform=\"rotate(90)\" font-size=\"%.8fpt\">%s</text>\n".as_ptr(), width, name); }
        fprintf(svgfile, c"</g>\n".as_ptr());
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn svg_cstate(cpu: c_int, start: u64, end: u64, mut type_: c_int) {
    let mut style: [c_char; 128] = [0; 128];
    unsafe {
        if svgfile.is_null() { return; }
        fprintf(svgfile, c"<g>\n".as_ptr());
        if type_ > 6 { type_ = 6; }
        sprintf(style.as_mut_ptr(), c"c%i".as_ptr(), type_);
        fprintf(svgfile, c"<rect class=\"%s\" x=\"%.8f\" width=\"%.8f\" y=\"%.1f\" height=\"%.1f\"/>\n".as_ptr(), style.as_ptr(), time2pixels(start), time2pixels(end) - time2pixels(start), cpu2y(cpu), SLOT_MULT + SLOT_HEIGHT);
        let mut width = (time2pixels(end) - time2pixels(start)) / 2.0;
        if width > 6.0 { width = 6.0; }
        width = round_text_size(width);
        if width > MIN_TEXT_SIZE { fprintf(svgfile, c"<text x=\"%.8f\" y=\"%.8f\" font-size=\"%.8fpt\">C%i</text>\n".as_ptr(), time2pixels(start), cpu2y(cpu) + width, width, type_); }
        fprintf(svgfile, c"</g>\n".as_ptr());
    }
}

unsafe fn HzToHuman(hz: c_ulong) -> *mut c_char {
    static mut BUFFER: [c_char; 1024] = [0; 1024];
    unsafe {
        memset((&raw mut BUFFER) as *mut c_void, 0, 1024);
        let Hz: c_ulonglong = hz as c_ulonglong;
        sprintf((&raw mut BUFFER) as *mut c_char, c"%9lli".as_ptr(), Hz);
        if Hz > 1000 { sprintf((&raw mut BUFFER) as *mut c_char, c" %6lli Mhz".as_ptr(), (Hz + 500) / 1000); }
        if Hz > 1500000 { sprintf((&raw mut BUFFER) as *mut c_char, c" %6.2f Ghz".as_ptr(), (Hz as c_double + 5000.0) / 1000000.0); }
        if Hz == turbo_frequency as c_ulonglong { sprintf((&raw mut BUFFER) as *mut c_char, c"Turbo".as_ptr()); }
        (&raw mut BUFFER) as *mut c_char
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn svg_pstate(cpu: c_int, start: u64, end: u64, freq: u64) {
    unsafe {
        let mut height: c_double = 0.0;
        if svgfile.is_null() { return; }
        fprintf(svgfile, c"<g>\n".as_ptr());
        if max_freq != 0 { height = freq as c_double * 1.0 / max_freq as c_double * (SLOT_HEIGHT + SLOT_MULT); }
        height = 1.0 + cpu2y(cpu) + SLOT_MULT + SLOT_HEIGHT - height;
        fprintf(svgfile, c"<line x1=\"%.8f\" x2=\"%.8f\" y1=\"%.1f\" y2=\"%.1f\" class=\"pstate\"/>\n".as_ptr(), time2pixels(start), time2pixels(end), height, height);
        fprintf(svgfile, c"<text x=\"%.8f\" y=\"%.8f\" font-size=\"0.25pt\">%s</text>\n".as_ptr(), time2pixels(start), height + 0.9, HzToHuman(freq as c_ulong));
        fprintf(svgfile, c"</g>\n".as_ptr());
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn svg_partial_wakeline(start: u64, row1: c_int, desc1: *mut c_char, row2: c_int, desc2: *mut c_char, backtrace: *const c_char) {
    unsafe {
        if svgfile.is_null() { return; }
        fprintf(svgfile, c"<g>\n".as_ptr());
        fprintf(svgfile, c"<title>%s wakes up %s</title>\n".as_ptr(), cstr_or_question(desc1), cstr_or_question(desc2));
        if !backtrace.is_null() { fprintf(svgfile, c"<desc>%s</desc>\n".as_ptr(), backtrace); }
        if row1 < row2 {
            if row1 != 0 {
                fprintf(svgfile, c"<line x1=\"%.8f\" y1=\"%.2f\" x2=\"%.8f\" y2=\"%.2f\" style=\"stroke:rgb(32,255,32);stroke-width:0.009\"/>\n".as_ptr(), time2pixels(start), row1 as c_double * SLOT_MULT + SLOT_HEIGHT, time2pixels(start), row1 as c_double * SLOT_MULT + SLOT_HEIGHT + SLOT_MULT / 32.0);
                if !desc2.is_null() { fprintf(svgfile, c"<g transform=\"translate(%.8f,%.8f)\"><text transform=\"rotate(90)\" font-size=\"0.02pt\">%s &gt;</text></g>\n".as_ptr(), time2pixels(start), row1 as c_double * SLOT_MULT + SLOT_HEIGHT + SLOT_HEIGHT / 48.0, desc2); }
            }
            if row2 != 0 {
                fprintf(svgfile, c"<line x1=\"%.8f\" y1=\"%.2f\" x2=\"%.8f\" y2=\"%.2f\" style=\"stroke:rgb(32,255,32);stroke-width:0.009\"/>\n".as_ptr(), time2pixels(start), row2 as c_double * SLOT_MULT - SLOT_MULT / 32.0, time2pixels(start), row2 as c_double * SLOT_MULT);
                if !desc1.is_null() { fprintf(svgfile, c"<g transform=\"translate(%.8f,%.8f)\"><text transform=\"rotate(90)\" font-size=\"0.02pt\">%s &gt;</text></g>\n".as_ptr(), time2pixels(start), row2 as c_double * SLOT_MULT - SLOT_MULT / 32.0, desc1); }
            }
        } else {
            if row2 != 0 {
                fprintf(svgfile, c"<line x1=\"%.8f\" y1=\"%.2f\" x2=\"%.8f\" y2=\"%.2f\" style=\"stroke:rgb(32,255,32);stroke-width:0.009\"/>\n".as_ptr(), time2pixels(start), row2 as c_double * SLOT_MULT + SLOT_HEIGHT, time2pixels(start), row2 as c_double * SLOT_MULT + SLOT_HEIGHT + SLOT_MULT / 32.0);
                if !desc1.is_null() { fprintf(svgfile, c"<g transform=\"translate(%.8f,%.8f)\"><text transform=\"rotate(90)\" font-size=\"0.02pt\">%s &lt;</text></g>\n".as_ptr(), time2pixels(start), row2 as c_double * SLOT_MULT + SLOT_HEIGHT + SLOT_MULT / 48.0, desc1); }
            }
            if row1 != 0 {
                fprintf(svgfile, c"<line x1=\"%.8f\" y1=\"%.2f\" x2=\"%.8f\" y2=\"%.2f\" style=\"stroke:rgb(32,255,32);stroke-width:0.009\"/>\n".as_ptr(), time2pixels(start), row1 as c_double * SLOT_MULT - SLOT_MULT / 32.0, time2pixels(start), row1 as c_double * SLOT_MULT);
                if !desc2.is_null() { fprintf(svgfile, c"<g transform=\"translate(%.8f,%.8f)\"><text transform=\"rotate(90)\" font-size=\"0.02pt\">%s &lt;</text></g>\n".as_ptr(), time2pixels(start), row1 as c_double * SLOT_MULT - SLOT_HEIGHT / 32.0, desc2); }
            }
        }
        let mut height = row1 as c_double * SLOT_MULT;
        if row2 > row1 { height += SLOT_HEIGHT; }
        if row1 != 0 { fprintf(svgfile, c"<circle  cx=\"%.8f\" cy=\"%.2f\" r = \"0.01\"  style=\"fill:rgb(32,255,32)\"/>\n".as_ptr(), time2pixels(start), height); }
        fprintf(svgfile, c"</g>\n".as_ptr());
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn svg_wakeline(start: u64, row1: c_int, row2: c_int, backtrace: *const c_char) {
    unsafe {
        if svgfile.is_null() { return; }
        fprintf(svgfile, c"<g>\n".as_ptr());
        if !backtrace.is_null() { fprintf(svgfile, c"<desc>%s</desc>\n".as_ptr(), backtrace); }
        if row1 < row2 {
            fprintf(svgfile, c"<line x1=\"%.8f\" y1=\"%.2f\" x2=\"%.8f\" y2=\"%.2f\" style=\"stroke:rgb(32,255,32);stroke-width:0.009\"/>\n".as_ptr(), time2pixels(start), row1 as c_double * SLOT_MULT + SLOT_HEIGHT, time2pixels(start), row2 as c_double * SLOT_MULT);
        } else {
            fprintf(svgfile, c"<line x1=\"%.8f\" y1=\"%.2f\" x2=\"%.8f\" y2=\"%.2f\" style=\"stroke:rgb(32,255,32);stroke-width:0.009\"/>\n".as_ptr(), time2pixels(start), row2 as c_double * SLOT_MULT + SLOT_HEIGHT, time2pixels(start), row1 as c_double * SLOT_MULT);
        }
        let mut height = row1 as c_double * SLOT_MULT;
        if row2 > row1 { height += SLOT_HEIGHT; }
        fprintf(svgfile, c"<circle  cx=\"%.8f\" cy=\"%.2f\" r = \"0.01\"  style=\"fill:rgb(32,255,32)\"/>\n".as_ptr(), time2pixels(start), height);
        fprintf(svgfile, c"</g>\n".as_ptr());
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn svg_interrupt(start: u64, row: c_int, backtrace: *const c_char) {
    unsafe {
        if svgfile.is_null() { return; }
        fprintf(svgfile, c"<g>\n".as_ptr());
        fprintf(svgfile, c"<title>Wakeup from interrupt</title>\n".as_ptr());
        if !backtrace.is_null() { fprintf(svgfile, c"<desc>%s</desc>\n".as_ptr(), backtrace); }
        fprintf(svgfile, c"<circle  cx=\"%.8f\" cy=\"%.2f\" r = \"0.01\"  style=\"fill:rgb(255,128,128)\"/>\n".as_ptr(), time2pixels(start), row as c_double * SLOT_MULT);
        fprintf(svgfile, c"<circle  cx=\"%.8f\" cy=\"%.2f\" r = \"0.01\"  style=\"fill:rgb(255,128,128)\"/>\n".as_ptr(), time2pixels(start), row as c_double * SLOT_MULT + SLOT_HEIGHT);
        fprintf(svgfile, c"</g>\n".as_ptr());
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn svg_text(Yslot: c_int, start: u64, text: *const c_char) {
    unsafe {
        if svgfile.is_null() { return; }
        fprintf(svgfile, c"<text x=\"%.8f\" y=\"%.8f\">%s</text>\n".as_ptr(), time2pixels(start), Yslot as c_double * SLOT_MULT + SLOT_HEIGHT / 2.0, text);
    }
}

unsafe fn svg_legenda_box(X: c_int, text: *const c_char, style: *const c_char) {
    unsafe {
        let boxsize = SLOT_HEIGHT / 2.0;
        fprintf(svgfile, c"<rect x=\"%i\" width=\"%.8f\" y=\"0\" height=\"%.1f\" class=\"%s\"/>\n".as_ptr(), X, boxsize, boxsize, style);
        fprintf(svgfile, c"<text transform=\"translate(%.8f, %.8f)\" font-size=\"%.8fpt\">%s</text>\n".as_ptr(), X as c_double + boxsize + 5.0, boxsize, 0.8 * boxsize, text);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn svg_io_legenda() {
    unsafe {
        if svgfile.is_null() { return; }
        fprintf(svgfile, c"<g>\n".as_ptr());
        svg_legenda_box(0, c"Disk".as_ptr(), c"disk".as_ptr());
        svg_legenda_box(100, c"Network".as_ptr(), c"net".as_ptr());
        svg_legenda_box(200, c"Sync".as_ptr(), c"sync".as_ptr());
        svg_legenda_box(300, c"Poll".as_ptr(), c"poll".as_ptr());
        svg_legenda_box(400, c"Error".as_ptr(), c"error".as_ptr());
        fprintf(svgfile, c"</g>\n".as_ptr());
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn svg_legenda() {
    unsafe {
        if svgfile.is_null() { return; }
        fprintf(svgfile, c"<g>\n".as_ptr());
        svg_legenda_box(0, c"Running".as_ptr(), c"sample".as_ptr());
        svg_legenda_box(100, c"Idle".as_ptr(), c"c1".as_ptr());
        svg_legenda_box(200, c"Deeper Idle".as_ptr(), c"c3".as_ptr());
        svg_legenda_box(350, c"Deepest Idle".as_ptr(), c"c6".as_ptr());
        svg_legenda_box(550, c"Sleeping".as_ptr(), c"process2".as_ptr());
        svg_legenda_box(650, c"Waiting for cpu".as_ptr(), c"waiting".as_ptr());
        svg_legenda_box(800, c"Blocked on IO".as_ptr(), c"blocked".as_ptr());
        fprintf(svgfile, c"</g>\n".as_ptr());
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn svg_time_grid(min_thickness: c_double) {
    unsafe {
        if svgfile.is_null() { return; }
        let mut i = first_time;
        while i < last_time {
            let mut color: c_int = 220;
            let mut thickness: c_double = 0.075;
            if i % 100000000 == 0 {
                thickness = 0.5;
                color = 192;
            }
            if i % 1000000000 == 0 {
                thickness = 2.0;
                color = 128;
            }
            if thickness >= min_thickness {
                fprintf(svgfile, c"<line x1=\"%.8f\" y1=\"%.2f\" x2=\"%.8f\" y2=\"%lu\" style=\"stroke:rgb(%i,%i,%i);stroke-width:%.3f\"/>\n".as_ptr(), time2pixels(i), SLOT_MULT / 2.0, time2pixels(i), total_height, color, color, color, thickness);
            }
            i += 10000000;
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn svg_close() {
    unsafe {
        if !svgfile.is_null() {
            fprintf(svgfile, c"</svg>\n".as_ptr());
            fclose(svgfile);
            svgfile = ptr::null_mut();
        }
    }
}

#[repr(C)]
pub struct cpumask_t {
    pub bits: [c_ulong; CPUMASK_WORDS],
}

#[repr(C)]
pub struct topology {
    pub sib_core: *mut cpumask_t,
    pub sib_core_nr: c_int,
    pub sib_thr: *mut cpumask_t,
    pub sib_thr_nr: c_int,
}

unsafe fn cpumask_bits(maskp: *mut cpumask_t) -> *mut c_ulong {
    unsafe { (*maskp).bits.as_mut_ptr() }
}

unsafe fn test_bit(nr: c_int, addr: *mut c_ulong) -> bool {
    unsafe { ((*addr.add(nr as usize / BITS_PER_LONG) >> (nr as usize % BITS_PER_LONG)) & 1) != 0 }
}

unsafe fn __set_bit(nr: c_int, addr: *mut c_ulong) {
    unsafe {
        *addr.add(nr as usize / BITS_PER_LONG) |= (1 as c_ulong) << (nr as usize % BITS_PER_LONG);
    }
}

unsafe fn scan_thread_topology(map: *mut c_int, t: *mut topology, cpu: c_int, pos: *mut c_int, nr_cpus: c_int) {
    unsafe {
        let mut i = 0;
        while i < (*t).sib_thr_nr {
            if !test_bit(cpu, cpumask_bits((*t).sib_thr.add(i as usize))) {
                i += 1;
                continue;
            }
            let mut thr = 0;
            while thr < nr_cpus {
                if test_bit(thr, cpumask_bits((*t).sib_thr.add(i as usize))) && *map.add(thr as usize) == -1 {
                    *map.add(thr as usize) = *pos;
                    *pos += 1;
                }
                thr += 1;
            }
            i += 1;
        }
    }
}

unsafe fn scan_core_topology(map: *mut c_int, t: *mut topology, nr_cpus: c_int) {
    unsafe {
        let mut pos: c_int = 0;
        let mut i = 0;
        while i < (*t).sib_core_nr {
            let mut cpu = 0;
            while cpu < nr_cpus {
                if test_bit(cpu, cpumask_bits((*t).sib_core.add(i as usize))) {
                    scan_thread_topology(map, t, cpu, &mut pos, nr_cpus);
                }
                cpu += 1;
            }
            i += 1;
        }
    }
}

unsafe fn str_to_bitmap(s: *mut c_char, b: *mut cpumask_t, nr_cpus: c_int) -> c_int {
    unsafe {
        let mut ret: c_int = 0;
        let map = perf_cpu_map__new(s);
        if map.is_null() {
            return -1;
        }
        let mut idx: c_uint = 0;
        while (idx as c_int) < perf_cpu_map__nr(map) {
            let cpu = perf_cpu_map__cpu(map, idx as c_int);
            /* perf_cpu_map__new("") returns cpu.cpu == -1 */
            if cpu.cpu < 0 || cpu.cpu >= nr_cpus {
                ret = -1;
                break;
            }
            __set_bit(cpu.cpu, cpumask_bits(b));
            idx += 1;
        }
        perf_cpu_map__put(map);
        ret
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn svg_build_topology_map(env: *mut perf_env) -> c_int {
    unsafe {
        let mut i: c_int;
        let nr_cpus = if (*env).nr_cpus_online < MAX_NR_CPUS as c_int { (*env).nr_cpus_online } else { MAX_NR_CPUS as c_int };
        let mut t = topology {
            sib_core: ptr::null_mut(),
            sib_core_nr: (*env).nr_sibling_cores,
            sib_thr: ptr::null_mut(),
            sib_thr_nr: (*env).nr_sibling_threads,
        };
        let mut sib_core = (*env).sibling_cores;
        let mut sib_thr = (*env).sibling_threads;
        let mut ret: c_int = -1;

        t.sib_core = calloc((*env).nr_sibling_cores as usize, core::mem::size_of::<cpumask_t>()) as *mut cpumask_t;
        t.sib_thr = calloc((*env).nr_sibling_threads as usize, core::mem::size_of::<cpumask_t>()) as *mut cpumask_t;

        if t.sib_core.is_null() || t.sib_thr.is_null() {
            fprintf(stderr, c"topology: no memory\n".as_ptr());
            goto_exit(&mut t);
            return ret;
        }

        i = 0;
        while i < (*env).nr_sibling_cores {
            if str_to_bitmap(sib_core, t.sib_core.add(i as usize), nr_cpus) != 0 {
                fprintf(stderr, c"topology: can't parse siblings map\n".as_ptr());
                goto_exit(&mut t);
                return ret;
            }
            sib_core = sib_core.add(strlen(sib_core) + 1);
            i += 1;
        }

        i = 0;
        while i < (*env).nr_sibling_threads {
            if str_to_bitmap(sib_thr, t.sib_thr.add(i as usize), nr_cpus) != 0 {
                fprintf(stderr, c"topology: can't parse siblings map\n".as_ptr());
                goto_exit(&mut t);
                return ret;
            }
            sib_thr = sib_thr.add(strlen(sib_thr) + 1);
            i += 1;
        }

        topology_map = malloc(core::mem::size_of::<c_int>() * nr_cpus as usize) as *mut c_int;
        if topology_map.is_null() {
            fprintf(stderr, c"topology: no memory\n".as_ptr());
            goto_exit(&mut t);
            return ret;
        }
        topology_map_size = nr_cpus;

        i = 0;
        while i < nr_cpus {
            *topology_map.add(i as usize) = -1;
            i += 1;
        }

        scan_core_topology(topology_map, &mut t, nr_cpus);
        ret = 0;

        goto_exit(&mut t);
        ret
    }
}

unsafe fn goto_exit(t: *mut topology) {
    unsafe {
        if !(*t).sib_core.is_null() {
            free((*t).sib_core as *mut c_void);
            (*t).sib_core = ptr::null_mut();
        }
        if !(*t).sib_thr.is_null() {
            free((*t).sib_thr as *mut c_void);
            (*t).sib_thr = ptr::null_mut();
        }
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
