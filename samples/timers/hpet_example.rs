// SPDX-License-Identifier: GPL-2.0

use std::ffi::CStr;
use std::os::raw::{c_char, c_int, c_long, c_ulong, c_void};

// These declarations and constants are supplied by the corresponding system headers/dependencies.
extern "C" {
    fn hpet_read(argc: c_int, argv: *const *const c_char);
}

#[repr(C)]
struct hpet_command {
    command: *mut c_char,
    func: unsafe extern "C" fn(c_int, *const *const c_char),
}

static mut HPET_COMMAND: [hpet_command; 4] = [
    hpet_command { command: b"open-close\0" as *const u8 as *mut c_char, func: hpet_open_close },
    hpet_command { command: b"info\0" as *const u8 as *mut c_char, func: hpet_info },
    hpet_command { command: b"poll\0" as *const u8 as *mut c_char, func: hpet_poll },
    hpet_command { command: b"fasync\0" as *const u8 as *mut c_char, func: hpet_fasync },
];

pub unsafe fn main(argc: c_int, argv: *const *const c_char) -> c_int {
    let mut argc = argc - 1;
    let mut argv = argv.add(1);

    if argc == 0 {
        eprintln!("-hpet: requires command");
        return -1;
    }

    let mut i = 0usize;
    while i < HPET_COMMAND.len() {
        if libc::strcmp(*argv, HPET_COMMAND[i].command) == 0 {
            argc -= 1;
            argv = argv.add(1);
            eprintln!("-hpet: executing {}", CStr::from_ptr(HPET_COMMAND[i].command).to_string_lossy());
            (HPET_COMMAND[i].func)(argc, argv);
            return 0;
        }
        i += 1;
    }

    eprintln!("do_hpet: command {} not implemented", CStr::from_ptr(*argv).to_string_lossy());
    -1
}

pub unsafe extern "C" fn hpet_open_close(argc: c_int, argv: *const *const c_char) {
    if argc != 1 {
        eprintln!("hpet_open_close: device-name");
        return;
    }
    let fd = libc::open(*argv, libc::O_RDONLY);
    if fd < 0 { eprintln!("hpet_open_close: open failed"); } else { libc::close(fd); }
}

pub unsafe extern "C" fn hpet_info(argc: c_int, argv: *const *const c_char) {
    if argc != 1 { eprintln!("hpet_info: device-name"); return; }
    let fd = libc::open(*argv, libc::O_RDONLY);
    if fd < 0 { eprintln!("hpet_info: open of {} failed", CStr::from_ptr(*argv).to_string_lossy()); return; }
    let mut info = std::mem::MaybeUninit::<hpet_info>::uninit();
    if libc::ioctl(fd, HPET_INFO, info.as_mut_ptr()) < 0 {
        eprintln!("hpet_info: failed to get info");
    } else {
        let info = info.assume_init();
        eprintln!("hpet_info: hi_irqfreq 0x{:x} hi_flags 0x{:x} hi_hpet {} hi_timer {}", info.hi_ireqfreq, info.hi_flags, info.hi_hpet, info.hi_timer);
    }
    libc::close(fd);
}

pub unsafe extern "C" fn hpet_poll(argc: c_int, argv: *const *const c_char) {
    if argc != 3 { eprintln!("hpet_poll: device-name freq iterations"); return; }
    let freq = libc::atoi((*argv.add(1))) as c_ulong;
    let iterations = libc::atoi(*argv.add(2));
    let fd = libc::open(*argv, libc::O_RDONLY);
    if fd < 0 { eprintln!("hpet_poll: open of {} failed", CStr::from_ptr(*argv).to_string_lossy()); return; }
    let mut info = std::mem::MaybeUninit::<hpet_info>::uninit();
    if libc::ioctl(fd, HPET_IRQFREQ, freq) < 0 { eprintln!("hpet_poll: HPET_IRQFREQ failed"); }
    else if libc::ioctl(fd, HPET_INFO, info.as_mut_ptr()) < 0 { eprintln!("hpet_poll: failed to get info"); }
    else {
        let info = info.assume_init();
        eprintln!("hpet_poll: info.hi_flags 0x{:x}", info.hi_flags);
        if info.hi_flags != 0 && libc::ioctl(fd, HPET_EPI, 0) < 0 { eprintln!("hpet_poll: HPET_EPI failed"); }
        else if libc::ioctl(fd, HPET_IE_ON, 0) < 0 { eprintln!("hpet_poll, HPET_IE_ON failed"); }
        else {
            let mut pfd = libc::pollfd { fd, events: libc::POLLIN, revents: 0 };
            for _ in 0..iterations {
                pfd.revents = 0;
                let mut stv = libc::timeval { tv_sec: 0, tv_usec: 0 };
                let mut etv = libc::timeval { tv_sec: 0, tv_usec: 0 };
                libc::gettimeofday(&mut stv, std::ptr::null_mut());
                if libc::poll(&mut pfd, 1, -1) < 0 { eprintln!("hpet_poll: poll failed"); }
                else {
                    libc::gettimeofday(&mut etv, std::ptr::null_mut());
                    let usec = (etv.tv_sec * 1_000_000 + etv.tv_usec) - (stv.tv_sec * 1_000_000 + stv.tv_usec);
                    eprintln!("hpet_poll: expired time = 0x{:x}", usec);
                    eprintln!("hpet_poll: revents = 0x{:x}", pfd.revents);
                    let mut data: c_long = 0;
                    if libc::read(fd, &mut data as *mut _ as *mut c_void, std::mem::size_of_val(&data)) != std::mem::size_of_val(&data) as isize { eprintln!("hpet_poll: read failed"); }
                    else { eprintln!("hpet_poll: data 0x{:x}", data); }
                }
            }
        }
    }
    libc::close(fd);
}

#[repr(C)]
struct hpet_info { hi_ireqfreq: c_ulong, hi_flags: c_ulong, hi_hpet: c_int, hi_timer: c_int }

static mut hpet_sigio_count: c_int = 0;

unsafe extern "C" fn hpet_sigio(_: c_int) { eprintln!("hpet_sigio: called"); hpet_sigio_count += 1; }

pub unsafe extern "C" fn hpet_fasync(argc: c_int, argv: *const *const c_char) {
    hpet_sigio_count = 0;
    let mut fd = -1;
    let oldsig = libc::signal(libc::SIGIO, Some(hpet_sigio));
    if oldsig == libc::SIG_ERR { eprintln!("hpet_fasync: failed to set signal handler"); return; }
    if argc != 3 { eprintln!("hpet_fasync: device-name freq iterations"); }
    else {
        fd = libc::open(*argv, libc::O_RDONLY);
        if fd < 0 { eprintln!("hpet_fasync: failed to open {}", CStr::from_ptr(*argv).to_string_lossy()); }
        else {
            let value = libc::fcntl(fd, libc::F_GETFL);
            if libc::fcntl(fd, libc::F_SETOWN, libc::getpid()) == 1 || value == 1 || libc::fcntl(fd, libc::F_SETFL, value | libc::O_ASYNC) == 1 { eprintln!("hpet_fasync: fcntl failed"); }
            else {
                let freq = libc::atoi(*argv.add(1)) as c_ulong;
                let iterations = libc::atoi(*argv.add(2));
                let mut info = std::mem::MaybeUninit::<hpet_info>::uninit();
                if libc::ioctl(fd, HPET_IRQFREQ, freq) < 0 { eprintln!("hpet_fasync: HPET_IRQFREQ failed"); }
                else if libc::ioctl(fd, HPET_INFO, info.as_mut_ptr()) < 0 { eprintln!("hpet_fasync: failed to get info"); }
                else { for _ in 0..iterations { libc::pause(); eprintln!("hpet_fasync: count = {}", hpet_sigio_count); } }
            }
        }
    }
    libc::signal(libc::SIGIO, oldsig);
    if fd >= 0 { libc::close(fd); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
