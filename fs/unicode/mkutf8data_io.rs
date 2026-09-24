// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2014 SGI.
//! Narrow host stdio and scanf adapters; all Unicode algorithms use owned data.

use std::ffi::{c_char, c_int, c_void, CStr, CString};
use std::ptr::NonNull;

extern "C" {
    fn fopen(name: *const c_char, mode: *const c_char) -> *mut c_void;
    fn fclose(file: *mut c_void) -> c_int;
    fn rewind(file: *mut c_void);
    fn fgets(buffer: *mut c_char, size: c_int, file: *mut c_void) -> *mut c_char;
    fn fwrite(bytes: *const c_void, size: usize, count: usize, file: *mut c_void) -> usize;
    fn sscanf(input: *const c_char, format: *const c_char, ...) -> c_int;
    fn strtoul(input: *const c_char, end: *mut *mut c_char, radix: c_int) -> std::ffi::c_ulong;
    fn strerror(error: c_int) -> *const c_char;
    static mut stdout: *mut c_void;
}

pub(super) type Result<T> = std::result::Result<T, Vec<u8>>;

pub(super) fn output(bytes: &[u8]) {
    // SAFETY: stdout belongs to the host C runtime; the slice is readable for
    // its length. C deliberately ignores stream write errors in this utility.
    unsafe {
        fwrite(bytes.as_ptr().cast(), 1, bytes.len(), stdout);
    }
}

pub(super) fn file_error(path: &CStr) -> Vec<u8> {
    let mut error = b"Error parsing ".to_vec();
    error.extend(path.to_bytes());
    error.push(b'\n');
    error
}

pub(super) fn line_error(path: &CStr, line: &CStr) -> Vec<u8> {
    let mut error = b"Error parsing ".to_vec();
    error.extend(path.to_bytes());
    error.push(b':');
    error.extend(line.to_bytes());
    error.push(b'\n');
    error
}

pub(super) fn open_error(path: &CStr, error: std::io::Error) -> Vec<u8> {
    let code = error.raw_os_error().unwrap_or(0);
    let mut error = format!("Error {code} opening ").into_bytes();
    error.extend(path.to_bytes());
    error.extend(b": ");
    // SAFETY: strerror returns a process-owned NUL-terminated error string.
    error.extend(unsafe { CStr::from_ptr(strerror(code)) }.to_bytes());
    error.push(b'\n');
    error
}

pub(super) struct File(NonNull<c_void>);

impl File {
    pub(super) fn open(path: &CStr, write: bool) -> Result<Self> {
        // SAFETY: both arguments are valid C strings. The resulting FILE is
        // owned by this wrapper and closed exactly once by Drop.
        let pointer = unsafe { fopen(path.as_ptr(), if write { c"w" } else { c"r" }.as_ptr()) };
        if let Some(pointer) = NonNull::new(pointer) {
            return Ok(Self(pointer));
        }
        Err(open_error(path, std::io::Error::last_os_error()))
    }

    pub(super) fn line(&mut self) -> Option<CString> {
        let mut buffer = [0 as c_char; 1024];
        // SAFETY: the buffer has the stated capacity; self holds an open FILE.
        if unsafe { fgets(buffer.as_mut_ptr(), 1024, self.0.as_ptr()) }.is_null() {
            return None;
        }
        // SAFETY: successful fgets wrote a terminating NUL within the buffer.
        Some(unsafe { CStr::from_ptr(buffer.as_ptr()) }.to_owned())
    }

    pub(super) fn write(&mut self, bytes: &[u8]) {
        // SAFETY: this open FILE and readable slice remain live for the call.
        unsafe {
            fwrite(bytes.as_ptr().cast(), 1, bytes.len(), self.0.as_ptr());
        }
    }

    pub(super) fn rewind(&mut self) {
        // SAFETY: self owns a live FILE. As in C, rewind failures are not
        // separately reported; the second pass observes the resulting stream.
        unsafe {
            rewind(self.0.as_ptr());
        }
    }
}

impl Iterator for File {
    type Item = CString;
    fn next(&mut self) -> Option<Self::Item> {
        self.line()
    }
}

impl Drop for File {
    fn drop(&mut self) {
        // SAFETY: this wrapper exclusively owns the open FILE.
        unsafe {
            fclose(self.0.as_ptr());
        }
    }
}

pub(super) fn read(path: &CStr, verbose: i32) -> Result<File> {
    if verbose > 0 {
        output(b"Parsing ");
        output(path.to_bytes());
        output(b"\n");
    }
    File::open(path, false)
}

// These fixed-format adapters preserve scanf assignment-count/prefix behavior.
// Field widths only exclude buffer-overflow inputs for which the C is undefined.
pub(super) fn numbers(line: &CStr, format: &CStr) -> (i32, [u32; 4]) {
    let [mut a, mut b, mut c, mut d] = [0u32; 4];
    // SAFETY: internal callers use at most four 32-bit integer conversions; all
    // output objects are initialized, valid, and distinct for the entire call.
    let count = unsafe {
        sscanf(
            line.as_ptr(),
            format.as_ptr(),
            &mut a,
            &mut b,
            &mut c,
            &mut d,
        )
    };
    (count, [a, b, c, d])
}

pub(super) fn decomposition(line: &CStr) -> Option<(u32, CString)> {
    let mut code = 0u32;
    let mut text = [0 as c_char; 1024];
    // SAFETY: output field widths fit the buffers, and %X writes a u32.
    let count = unsafe {
        sscanf(
            line.as_ptr(),
            c"%X;%*[^;];%*[^;];%*[^;];%*[^;];%1023[^;];".as_ptr(),
            &mut code,
            text.as_mut_ptr(),
        )
    };
    if count != 2 {
        return None;
    }
    // SAFETY: successful scanset conversion terminates this bounded buffer.
    Some((code, unsafe { CStr::from_ptr(text.as_ptr()) }.to_owned()))
}

pub(super) fn casefold(line: &CStr) -> Option<(u32, u8, CString)> {
    let mut code = 0u32;
    let mut status = 0u8;
    let mut text = [0 as c_char; 1024];
    // SAFETY: the three output types/capacities match their fixed conversions.
    let count = unsafe {
        sscanf(
            line.as_ptr(),
            c"%X; %c; %1023[^;];".as_ptr(),
            &mut code,
            &mut status,
            text.as_mut_ptr(),
        )
    };
    if count != 3 {
        return None;
    }
    // SAFETY: successful scanset conversion terminates this bounded buffer.
    Some((
        code,
        status,
        unsafe { CStr::from_ptr(text.as_ptr()) }.to_owned(),
    ))
}

pub(super) fn property(line: &CStr, range: bool) -> Option<(u32, u32, CString)> {
    let (mut first, mut last) = (0u32, 0u32);
    let mut text = [0 as c_char; 1024];
    // SAFETY: the output types/capacities match the selected fixed format.
    let count = unsafe {
        if range {
            sscanf(
                line.as_ptr(),
                c"%X..%X ; %1023s # ".as_ptr(),
                &mut first,
                &mut last,
                text.as_mut_ptr(),
            )
        } else {
            sscanf(
                line.as_ptr(),
                c"%X ; %1023s # ".as_ptr(),
                &mut first,
                text.as_mut_ptr(),
            )
        }
    };
    if count != if range { 3 } else { 2 } {
        return None;
    }
    if !range {
        last = first;
    }
    // SAFETY: successful %s conversion terminates this bounded buffer.
    Some((
        first,
        last,
        unsafe { CStr::from_ptr(text.as_ptr()) }.to_owned(),
    ))
}

pub(super) fn correction(line: &CStr) -> Option<(u32, CString, CString, [u32; 3])> {
    let (mut code, mut major, mut minor, mut revision) = (0u32, 0u32, 0u32, 0u32);
    let (mut old, mut new) = ([0 as c_char; 1024], [0 as c_char; 1024]);
    // SAFETY: the six outputs have the exact fixed conversion types/capacities.
    let count = unsafe {
        sscanf(
            line.as_ptr(),
            c"%X;%1023[^;];%1023[^;];%d.%d.%d #".as_ptr(),
            &mut code,
            old.as_mut_ptr(),
            new.as_mut_ptr(),
            &mut major,
            &mut minor,
            &mut revision,
        )
    };
    if count != 6 {
        return None;
    }
    // SAFETY: successful scanset conversions terminate both bounded buffers.
    Some((
        code,
        unsafe { CStr::from_ptr(old.as_ptr()) }.to_owned(),
        unsafe { CStr::from_ptr(new.as_ptr()) }.to_owned(),
        [major, minor, revision],
    ))
}

pub(super) fn mapping(input: &[u8]) -> Option<Vec<u32>> {
    let string = CString::new(input).ok()?;
    let mut start = string.as_ptr();
    let mut result = Vec::new();
    loop {
        // SAFETY: start points inside the live NUL-terminated CString.
        if unsafe { *start } == 0 {
            break;
        }
        let mut end = std::ptr::null_mut();
        // SAFETY: valid string and end-pointer storage. strtoul never writes input.
        let code = unsafe { strtoul(start, &mut end, 16) } as u32;
        if end.cast_const() == start || code as usize >= super::model::LIMIT {
            return None;
        }
        result.push(code);
        start = end;
    }
    // The original maps are zero-terminated; U+0000 in mapping stops traversal.
    if let Some(zero) = result.iter().position(|&c| c == 0) {
        result.truncate(zero);
    }
    Some(result)
}
