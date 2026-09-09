// SPDX-License-Identifier: GPL-2.0
/*
 * Hidraw Userspace Example
 *
 * Copyright (c) 2010 Alan Ott <alan@signal11.us>
 * Copyright (c) 2010 Signal 11 Software
 *
 * The code may be used by anyone for any purpose,
 * and can serve as a starting point for developing
 * applications using hidraw.
 */

/* Linux and Unix declarations are supplied by the corresponding system
 * headers/dependencies. */

// Ugly hack preserved from the C source for systems with old hidraw headers.
// #ifndef HIDIOCSFEATURE
// #warning Please have your distro update the userspace kernel headers
// #define HIDIOCSFEATURE(len) _IOC(_IOC_WRITE|_IOC_READ, 'H', 0x06, len)
// #define HIDIOCGFEATURE(len) _IOC(_IOC_WRITE|_IOC_READ, 'H', 0x07, len)
// #endif

pub unsafe fn main(argc: std::ffi::c_int, argv: *mut *mut std::ffi::c_char) -> std::ffi::c_int {
    let mut fd: std::ffi::c_int;
    let mut i: std::ffi::c_int;
    let mut res: std::ffi::c_int;
    let mut desc_size: std::ffi::c_int = 0;
    let mut buf = [0i8; 256];
    let mut rpt_desc: hidraw_report_descriptor = std::mem::zeroed();
    let mut info: hidraw_devinfo = std::mem::zeroed();
    let mut device = b"/dev/hidraw0\0".as_ptr() as *mut std::ffi::c_char;

    if argc > 1 {
        device = *argv.add(1);
    }

    /* Open the Device with non-blocking reads. In real life,
       don't use a hard coded path; use libudev instead. */
    fd = open(device, O_RDWR | O_NONBLOCK);

    if fd < 0 {
        perror(b"Unable to open device\0".as_ptr() as *const std::ffi::c_char);
        return 1;
    }

    std::ptr::write_bytes(&mut rpt_desc as *mut _ as *mut u8, 0, std::mem::size_of::<hidraw_report_descriptor>());
    std::ptr::write_bytes(&mut info as *mut _ as *mut u8, 0, std::mem::size_of::<hidraw_devinfo>());
    std::ptr::write_bytes(buf.as_mut_ptr() as *mut u8, 0, buf.len());

    /* Get Report Descriptor Size */
    res = ioctl(fd, HIDIOCGRDESCSIZE, &mut desc_size);
    if res < 0 {
        perror(b"HIDIOCGRDESCSIZE\0".as_ptr() as *const std::ffi::c_char);
    } else {
        printf(b"Report Descriptor Size: %d\n\0".as_ptr() as *const std::ffi::c_char, desc_size);
    }

    /* Get Report Descriptor */
    rpt_desc.size = desc_size as _;
    res = ioctl(fd, HIDIOCGRDESC, &mut rpt_desc);
    if res < 0 {
        perror(b"HIDIOCGRDESC\0".as_ptr() as *const std::ffi::c_char);
    } else {
        printf(b"Report Descriptor:\n\0".as_ptr() as *const std::ffi::c_char);
        for i in 0..rpt_desc.size {
            printf(b"%hhx \0".as_ptr() as *const std::ffi::c_char, rpt_desc.value[i as usize]);
        }
        puts(b"\n\0".as_ptr() as *const std::ffi::c_char);
    }

    /* Get Raw Name */
    res = ioctl(fd, HIDIOCGRAWNAME(256), buf.as_mut_ptr());
    if res < 0 { perror(b"HIDIOCGRAWNAME\0".as_ptr() as *const std::ffi::c_char); }
    else { printf(b"Raw Name: %s\n\0".as_ptr() as *const std::ffi::c_char, buf.as_ptr()); }

    /* Get Physical Location */
    res = ioctl(fd, HIDIOCGRAWPHYS(256), buf.as_mut_ptr());
    if res < 0 { perror(b"HIDIOCGRAWPHYS\0".as_ptr() as *const std::ffi::c_char); }
    else { printf(b"Raw Phys: %s\n\0".as_ptr() as *const std::ffi::c_char, buf.as_ptr()); }

    /* Get Raw Info */
    res = ioctl(fd, HIDIOCGRAWINFO, &mut info);
    if res < 0 { perror(b"HIDIOCGRAWINFO\0".as_ptr() as *const std::ffi::c_char); }
    else {
        printf(b"Raw Info:\n\0".as_ptr() as *const std::ffi::c_char);
        printf(b"\tbustype: %d (%s)\n\0".as_ptr() as *const std::ffi::c_char, info.bustype, bus_str(info.bustype));
        printf(b"\tvendor: 0x%04hx\n\0".as_ptr() as *const std::ffi::c_char, info.vendor);
        printf(b"\tproduct: 0x%04hx\n\0".as_ptr() as *const std::ffi::c_char, info.product);
    }

    /* Set Feature */
    buf[0] = 0x9; /* Report Number */ buf[1] = -1; buf[2] = -1; buf[3] = -1;
    res = ioctl(fd, HIDIOCSFEATURE(4), buf.as_mut_ptr());
    if res < 0 { perror(b"HIDIOCSFEATURE\0".as_ptr() as *const std::ffi::c_char); }
    else { printf(b"ioctl HIDIOCSFEATURE returned: %d\n\0".as_ptr() as *const std::ffi::c_char, res); }

    /* Get Feature */
    buf[0] = 0x9; /* Report Number */
    res = ioctl(fd, HIDIOCGFEATURE(256), buf.as_mut_ptr());
    if res < 0 { perror(b"HIDIOCGFEATURE\0".as_ptr() as *const std::ffi::c_char); }
    else {
        printf(b"ioctl HIDIOCGFEATURE returned: %d\nReport data:\n\t\0".as_ptr() as *const std::ffi::c_char, res);
        for i in 0..res { printf(b"%hhx \0".as_ptr() as *const std::ffi::c_char, buf[i as usize]); }
        puts(b"\n\0".as_ptr() as *const std::ffi::c_char);
    }

    /* Send a Report to the Device */
    buf[0] = 0x1; /* Report Number */ buf[1] = 0x77;
    res = write(fd, buf.as_ptr() as *const _, 2);
    if res < 0 { printf(b"Error: %d\n\0".as_ptr() as *const std::ffi::c_char, errno); perror(b"write\0".as_ptr() as *const std::ffi::c_char); }
    else { printf(b"write() wrote %d bytes\n\0".as_ptr() as *const std::ffi::c_char, res); }

    /* Get a report from the device */
    res = read(fd, buf.as_mut_ptr() as *mut _, 16);
    if res < 0 { perror(b"read\0".as_ptr() as *const std::ffi::c_char); }
    else {
        printf(b"read() read %d bytes:\n\t\0".as_ptr() as *const std::ffi::c_char, res);
        for i in 0..res { printf(b"%hhx \0".as_ptr() as *const std::ffi::c_char, buf[i as usize]); }
        puts(b"\n\0".as_ptr() as *const std::ffi::c_char);
    }
    close(fd);
    0
}

pub unsafe fn bus_str(bus: std::ffi::c_int) -> *const std::ffi::c_char {
    match bus {
        BUS_USB => b"USB\0".as_ptr() as *const _,
        BUS_HIL => b"HIL\0".as_ptr() as *const _,
        BUS_BLUETOOTH => b"Bluetooth\0".as_ptr() as *const _,
        BUS_VIRTUAL => b"Virtual\0".as_ptr() as *const _,
        _ => b"Other\0".as_ptr() as *const _,
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
