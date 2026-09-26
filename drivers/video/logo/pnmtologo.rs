// SPDX-License-Identifier: GPL-2.0-only
/*
 * Convert an ASCII PNM logo into C source for the Linux kernel.
 * (C) Copyright 2001-2003 Geert Uytterhoeven <geert@linux-m68k.org>
 */

//! Preserve the PNM host converter's parser, palette order and packed formats.

#![no_main]

use std::ffi::{c_char, c_int, c_uint, c_void, CStr};
use std::ptr::NonNull;

unsafe extern "C" {
    static stdout: *mut c_void;
    static stderr: *mut c_void;
    static mut optarg: *mut c_char;
    static mut optind: c_int;
    static mut opterr: c_int;
    fn getopt(argc: c_int, argv: *mut *mut c_char, options: *const c_char) -> c_int;
    fn fopen(path: *const c_char, mode: *const c_char) -> *mut c_void;
    fn fclose(stream: *mut c_void) -> c_int;
    fn fgetc(stream: *mut c_void) -> c_int;
    fn fputs(text: *const c_char, stream: *mut c_void) -> c_int;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn malloc(size: usize) -> *mut c_void;
    fn strerror(error: c_int) -> *const c_char;
    fn exit(status: c_int) -> !;
}

macro_rules! emit {
    ($stream:expr, $format:expr $(, $argument:expr)* $(,)?) => {{
        // SAFETY: all callers provide live streams, literal formats and matching
        // C-promoted integers or terminated strings valid for the whole call.
        unsafe { fprintf($stream, $format.as_ptr() $(, $argument)*) };
    }};
}

macro_rules! die {
    ($format:expr $(, $argument:expr)* $(,)?) => {{
        // SAFETY: libc initializes stderr before calling main.
        let error = unsafe { stderr };
        emit!(error, $format $(, $argument)*);
        // SAFETY: preserve libc's exit-time flushing of all open streams.
        unsafe { exit(1) }
    }};
}

const TYPES: [&CStr; 5] = [c"", c"LINUX_LOGO_MONO", c"LINUX_LOGO_VGA16",
                         c"LINUX_LOGO_CLUT224", c"LINUX_LOGO_GRAY256"];
type Color = [u8; 3];
const VGA16: [Color; 16] = [
    [0x00, 0x00, 0x00], [0x00, 0x00, 0xaa], [0x00, 0xaa, 0x00], [0x00, 0xaa, 0xaa],
    [0xaa, 0x00, 0x00], [0xaa, 0x00, 0xaa], [0xaa, 0x55, 0x00], [0xaa, 0xaa, 0xaa],
    [0x55, 0x55, 0x55], [0x55, 0x55, 0xff], [0x55, 0xff, 0x55], [0x55, 0xff, 0xff],
    [0xff, 0x55, 0x55], [0xff, 0x55, 0xff], [0xff, 0xff, 0x55], [0xff, 0xff, 0xff],
];

fn last_error() -> *const c_char {
    let error = std::io::Error::last_os_error().raw_os_error().unwrap_or(0);
    // SAFETY: strerror returns a libc-owned terminated diagnostic string.
    unsafe { strerror(error) }
}

fn next(input: *mut c_void) -> c_int {
    // SAFETY: only read_image calls this helper, with its open input stream.
    unsafe { fgetc(input) }
}

fn number(input: *mut c_void, filename: &CStr, plain_pbm: bool) -> c_uint {
    let mut byte;
    loop {
        byte = next(input);
        if byte == -1 {
            die!(c"%s: end of file\n", filename.as_ptr());
        }
        if byte == c_int::from(b'#') {
            loop {
                byte = next(input);
                if byte == -1 {
                    die!(c"%s: end of file\n", filename.as_ptr());
                }
                if byte == c_int::from(b'\n') {
                    break;
                }
            }
        }
        if !matches!(byte, 9..=13 | 32) {
            break;
        }
    }
    let mut value = 0u32;
    while (c_int::from(b'0')..=c_int::from(b'9')).contains(&byte) {
        value = value.wrapping_mul(10).wrapping_add((byte - c_int::from(b'0')) as u32);
        if plain_pbm {
            break;
        }
        byte = next(input);
        if byte == -1 {
            die!(c"%s: end of file\n", filename.as_ptr());
        }
    }
    value
}

fn number255(input: *mut c_void, filename: &CStr, maximum: c_uint) -> u8 {
    let value = number(input, filename, false);
    if maximum == 0 {
        // Original C divides by zero for a nonempty image with maxval == 0.
        die!(c"%s: invalid maximum color value\n", filename.as_ptr());
    }
    (value.wrapping_mul(255).wrapping_add(maximum / 2) / maximum) as u8
}

// Retain the original fallible pointer-table and per-row malloc calls. The
// process owns these rows until exit, including allocation-failure paths. Raw
// element access also avoids constructing a >isize::MAX slice on ILP32 hosts.
struct Image {
    rows: NonNull<*mut Color>,
    width: c_uint,
    height: c_uint,
}

fn allocate(size: Option<usize>) -> NonNull<c_void> {
    let Some(size) = size else {
        // An overflowing allocation cannot represent a defined C image.
        die!(c"Cannot allocate image data\n");
    };
    // SAFETY: malloc accepts every size_t value, including zero.
    match NonNull::new(unsafe { malloc(size) }) {
        Some(pointer) => pointer,
        None => die!(c"%s\n", last_error()),
    }
}

impl Image {
    fn allocate(width: c_uint, height: c_uint) -> Self {
        let rows = allocate((height as usize).checked_mul(std::mem::size_of::<*mut Color>()))
            .cast::<*mut Color>();
        for row in 0..height {
            let pixels = allocate((width as usize).checked_mul(std::mem::size_of::<Color>()))
                .cast::<Color>();
            // SAFETY: rows owns height pointer slots and this slot is unique.
            unsafe { rows.as_ptr().wrapping_add(row as usize).write(pixels.as_ptr()) };
        }
        Self { rows, width, height }
    }

    fn set(&mut self, row: c_uint, column: c_uint, color: Color) {
        assert!(row < self.height && column < self.width);
        // SAFETY: each row owns width Color slots; this writes a complete pixel.
        unsafe { (*self.rows.as_ptr().wrapping_add(row as usize))
            .wrapping_add(column as usize).write(color) };
    }

    fn get(&self, row: c_uint, column: c_uint) -> Color {
        assert!(row < self.height && column < self.width);
        // SAFETY: called only after read_image initialized every pixel.
        unsafe { (*self.rows.as_ptr().wrapping_add(row as usize))
            .wrapping_add(column as usize).read() }
    }
}

fn read_image(filename: &CStr) -> Image {
    // SAFETY: filename and mode are terminated strings.
    let input = unsafe { fopen(filename.as_ptr(), c"r".as_ptr()) };
    if input.is_null() {
        die!(c"Cannot open file %s: %s\n", filename.as_ptr(), last_error());
    }
    if next(input) != c_int::from(b'P') {
        die!(c"%s is not a PNM file\n", filename.as_ptr());
    }
    let magic = next(input);
    match magic {
        49..=51 => {}
        52..=54 => die!(c"%s: Binary PNM is not supported\nUse pnmnoraw(1) to convert it to ASCII PNM\n", filename.as_ptr()),
        _ => die!(c"%s is not a PNM file\n", filename.as_ptr()),
    }
    let width = number(input, filename, false);
    let height = number(input, filename, false);
    let mut image = Image::allocate(width, height);
    let maximum = if magic == 49 { 0 } else { number(input, filename, false) };
    for row in 0..height {
        for column in 0..width {
            let color = match magic {
                49 => [255u32.wrapping_mul(1u32.wrapping_sub(number(input, filename, true))) as u8; 3],
                50 => [number255(input, filename, maximum); 3],
                _ => [number255(input, filename, maximum), number255(input, filename, maximum),
                      number255(input, filename, maximum)],
            };
            image.set(row, column, color);
        }
    }
    // SAFETY: this is the sole close of read_image's successful fopen.
    unsafe { fclose(input) };
    image
}

struct Output {
    stream: *mut c_void,
    name: *const c_char,
    owned: bool,
    hex_count: c_uint,
}

impl Output {
    fn open(name: &CStr, filename: Option<&CStr>) -> Self {
        let stream = if let Some(filename) = filename {
            // SAFETY: both strings are terminated and remain live.
            let stream = unsafe { fopen(filename.as_ptr(), c"w".as_ptr()) };
            if stream.is_null() {
                die!(c"Cannot create file %s: %s\n", filename.as_ptr(), last_error());
            }
            stream
        } else {
            // SAFETY: libc initializes stdout before main.
            unsafe { stdout }
        };
        let output = Self { stream, name: name.as_ptr(), owned: filename.is_some(), hex_count: 0 };
        output.text(c"/*\n");
        output.text(c" *  DO NOT EDIT THIS FILE!\n");
        output.text(c" *\n");
        emit!(stream, c" *  Linux logo %s\n", output.name);
        output.text(c" */\n\n");
        output.text(c"#include <linux/linux_logo.h>\n\n");
        emit!(stream, c"static const unsigned char %s_data[] __initconst = {\n", output.name);
        output
    }

    fn text(&self, text: &CStr) {
        // SAFETY: the stream and terminated string are valid; ignore I/O errors.
        unsafe { fputs(text.as_ptr(), self.stream) };
    }

    fn hex(&mut self, byte: u8) {
        if self.hex_count % 12 != 0 {
            emit!(self.stream, c", 0x%02x", c_int::from(byte));
        } else if self.hex_count != 0 {
            emit!(self.stream, c",\n\t0x%02x", c_int::from(byte));
        } else {
            emit!(self.stream, c"\t0x%02x", c_int::from(byte));
        }
        self.hex_count = self.hex_count.wrapping_add(1);
    }

    fn finish(self, image: &Image, kind: usize, clut_size: c_uint) {
        self.text(c"\n};\n\n");
        emit!(self.stream, c"const struct linux_logo %s __initconst = {\n", self.name);
        emit!(self.stream, c"\t.type\t\t= %s,\n", TYPES[kind].as_ptr());
        emit!(self.stream, c"\t.width\t\t= %u,\n", image.width);
        emit!(self.stream, c"\t.height\t\t= %u,\n", image.height);
        if kind == 3 {
            emit!(self.stream, c"\t.clutsize\t= %u,\n", clut_size);
            emit!(self.stream, c"\t.clut\t\t= %s_clut,\n", self.name);
        }
        emit!(self.stream, c"\t.data\t\t= %s_data\n", self.name);
        self.text(c"};\n\n");
        if self.owned {
            // SAFETY: closes the sole output-file handle; stdout stays borrowed.
            unsafe { fclose(self.stream) };
        }
    }
}

fn write_image(image: &Image, kind: usize, name: &CStr, filename: Option<&CStr>) {
    let mut clut = [[0u8; 3]; 224];
    let mut clut_size = 0;
    for row in 0..image.height {
        for column in 0..image.width {
            let color = image.get(row, column);
            match kind {
                1 if color != [0; 3] && color != [255; 3] => die!(c"Image must be monochrome\n"),
                2 if !VGA16.contains(&color) => die!(c"Image must use the 16 console colors only\nUse ppmquant(1) -map clut_vga16.ppm to reduce the number of colors\n"),
                3 if !clut[..clut_size].contains(&color) => {
                    if clut_size == 224 {
                        die!(c"Image has more than %d colors\nUse ppmquant(1) to reduce the number of colors\n", 224 as c_int);
                    }
                    clut[clut_size] = color;
                    clut_size += 1;
                }
                4 if color[0] != color[1] || color[0] != color[2] => die!(c"Image must be grayscale\n"),
                _ => {}
            }
        }
    }
    let mut output = Output::open(name, filename);
    for row in 0..image.height {
        let mut column = 0;
        while column < image.width {
            let value = match kind {
                1 => {
                    let mut value = 0;
                    let mut bit = 0x80;
                    while bit != 0 && column < image.width {
                        if image.get(row, column)[0] != 0 {
                            value |= bit;
                        }
                        column += 1;
                        bit >>= 1;
                    }
                    value
                }
                2 => {
                    let color = image.get(row, column);
                    let mut value = (VGA16.iter().position(|&entry| entry == color).unwrap() as u8) << 4;
                    column += 1;
                    if column < image.width {
                        let color = image.get(row, column);
                        value |= VGA16.iter().position(|&entry| entry == color).unwrap() as u8;
                        column += 1;
                    }
                    value
                }
                3 => {
                    let color = image.get(row, column);
                    column += 1;
                    clut[..clut_size].iter().position(|&entry| entry == color).unwrap() as u8 + 32
                }
                _ => {
                    let value = image.get(row, column)[0];
                    column += 1;
                    value
                }
            };
            output.hex(value);
        }
    }
    if kind == 3 {
        output.text(c"\n};\n\n");
        emit!(output.stream, c"static const unsigned char %s_clut[] __initconst = {\n", output.name);
        output.hex_count = 0;
        for color in &clut[..clut_size] {
            for &byte in color {
                output.hex(byte);
            }
        }
    }
    output.finish(image, kind, clut_size as c_uint);
}

fn usage(program: *const c_char) -> ! {
    die!(c"\nUsage: %s [options] <filename>\n\nValid options:\n\t-h\t\t  : display this usage information\n\t-n <name>   : specify logo name (default: linux_logo)\n\t-o <output> : output to file <output> instead of stdout\n\t-t <type>   : specify logo type, one of\n\t\t\t\t\t  mono\t: monochrome black/white\n\t\t\t\t\t  vga16   : 16 colors VGA text palette\n\t\t\t\t\t  clut224 : 224 colors (default)\n\t\t\t\t\t  gray256 : 256 levels grayscale\n\n", program);
}

/// C startup preserves libc getopt semantics and inherited signal dispositions.
///
/// # Safety
/// The C runtime supplies argc live argument strings and a writable argv array.
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    // SAFETY: the runtime supplies argv[0] and initialized getopt globals.
    let program = unsafe { *argv };
    unsafe { opterr = 0 };
    let mut name = c"linux_logo";
    let mut output = None;
    let mut kind = 3;
    loop {
        // SAFETY: argv has the writable array expected by libc getopt.
        let option = unsafe { getopt(argc, argv, c"hn:o:t:".as_ptr()) };
        if option == -1 {
            break;
        }
        match option {
            110 | 111 | 116 => {
                // SAFETY: these options require an argument; getopt returns '?'
                // instead when absent, so optarg names a live terminated string.
                let argument = unsafe { CStr::from_ptr(optarg) };
                match option {
                    110 => name = argument,
                    111 => output = Some(argument),
                    _ => kind = match argument.to_bytes() {
                        b"mono" => 1,
                        b"vga16" => 2,
                        b"clut224" => 3,
                        b"gray256" => 4,
                        _ => usage(program),
                    },
                }
            }
            _ => usage(program),
        }
    }
    // SAFETY: optind was initialized and updated by this process's getopt calls.
    let index = unsafe { optind };
    if index != argc - 1 {
        usage(program);
    }
    // SAFETY: exactly one positional argument remains in the argv array.
    let filename = unsafe { CStr::from_ptr(*argv.add(index as usize)) };
    let image = read_image(filename);
    write_image(&image, kind, name, output);
    // SAFETY: libc exit preserves the original stdout and ignored-close behavior.
    unsafe { exit(0) }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
