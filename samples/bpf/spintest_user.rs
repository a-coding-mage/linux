// SPDX-License-Identifier: GPL-2.0
//
// C headers and libbpf symbols are supplied by the surrounding build.

use std::ffi::c_char;
use std::os::raw::{c_int, c_long, c_void};

#[repr(C)]
pub struct bpf_object {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ksym {
    pub name: *const c_char,
}

extern "C" {
    fn load_kallsyms() -> c_int;
    fn bpf_object__open_file(path: *const c_char, opts: *const c_void) -> *mut bpf_object;
    fn libbpf_get_error(ptr: *const c_void) -> c_long;
    fn bpf_object__load(obj: *mut bpf_object) -> c_int;
    fn bpf_object__find_map_fd_by_name(obj: *mut bpf_object, name: *const c_char) -> c_int;
    fn bpf_object__next_program(
        prog: *mut bpf_program,
        obj: *mut bpf_object,
    ) -> *mut bpf_program;
    fn bpf_program__attach(prog: *mut bpf_program) -> *mut bpf_link;
    fn bpf_map_get_next_key(fd: c_int, key: *const c_long, next_key: *mut c_long) -> c_int;
    fn bpf_map_lookup_elem(fd: c_int, key: *const c_long, value: *mut c_long) -> c_int;
    fn ksym_search(addr: c_long) -> *mut ksym;
    fn bpf_map_delete_elem(fd: c_int, key: *const c_long) -> c_int;
    fn bpf_link__destroy(link: *mut bpf_link);
    fn bpf_object__close(obj: *mut bpf_object);
}

pub unsafe fn main(ac: c_int, argv: *mut *mut c_char) -> c_int {
    let _ = ac;
    let mut obj: *mut bpf_object = std::ptr::null_mut();
    let mut links: [*mut bpf_link; 20] = [std::ptr::null_mut(); 20];
    let mut key: c_long;
    let mut next_key: c_long = 0;
    let mut value: c_long = 0;
    let mut prog: *mut bpf_program = std::ptr::null_mut();
    let mut map_fd: c_int;
    let mut i: c_int;
    let mut j: c_int = 0;
    let mut filename = [0 as c_char; 256];
    let mut sym: *mut ksym;

    if load_kallsyms() != 0 {
        println!("failed to process /proc/kallsyms");
        return 2;
    }

    let argv0 = std::ffi::CStr::from_ptr(*argv);
    let suffix = b".bpf.o\0";
    let mut n = 0usize;
    for &ch in argv0.to_bytes_with_nul().iter().chain(suffix[..suffix.len() - 1].iter()) {
        if n + 1 >= filename.len() {
            break;
        }
        filename[n] = ch as c_char;
        n += 1;
    }
    filename[n] = 0;
    obj = bpf_object__open_file(filename.as_ptr(), std::ptr::null());
    if libbpf_get_error(obj as *const c_void) != 0 {
        eprintln!("ERROR: opening BPF object file failed");
        obj = std::ptr::null_mut();
        goto_cleanup(&mut links, &mut j, obj);
        return 0;
    }

    if bpf_object__load(obj) != 0 {
        eprintln!("ERROR: loading BPF object file failed");
        goto_cleanup(&mut links, &mut j, obj);
        return 0;
    }

    let map_name = b"my_map\0";
    map_fd = bpf_object__find_map_fd_by_name(obj, map_name.as_ptr() as *const c_char);
    if map_fd < 0 {
        eprintln!("ERROR: finding a map in obj file failed");
        goto_cleanup(&mut links, &mut j, obj);
        return 0;
    }

    while !(prog = bpf_object__next_program(prog, obj)).is_null() {
        links[j as usize] = bpf_program__attach(prog);
        if libbpf_get_error(links[j as usize] as *const c_void) != 0 {
            eprintln!("bpf_program__attach failed");
            links[j as usize] = std::ptr::null_mut();
            goto_cleanup(&mut links, &mut j, obj);
            return 0;
        }
        j += 1;
    }

    for _ in 0..5 {
        key = 0;
        print!("kprobing funcs:");
        while bpf_map_get_next_key(map_fd, &key, &mut next_key) == 0 {
            bpf_map_lookup_elem(map_fd, &next_key, &mut value);
            assert!(next_key == value);
            sym = ksym_search(value);
            key = next_key;
            if sym.is_null() {
                println!("ksym not found. Is kallsyms loaded?");
                continue;
            }
            print!(" {}", std::ffi::CStr::from_ptr((*sym).name).to_string_lossy());
        }
        if key != 0 {
            println!();
        }
        key = 0;
        while bpf_map_get_next_key(map_fd, &key, &mut next_key) == 0 {
            bpf_map_delete_elem(map_fd, &next_key);
        }
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    goto_cleanup(&mut links, &mut j, obj);
    0
}

unsafe fn goto_cleanup(links: &mut [*mut bpf_link; 20], j: &mut c_int, obj: *mut bpf_object) {
    *j -= 1;
    while *j >= 0 {
        bpf_link__destroy(links[*j as usize]);
        *j -= 1;
    }
    bpf_object__close(obj);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
