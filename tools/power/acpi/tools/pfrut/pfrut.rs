// SPDX-License-Identifier: GPL-2.0
/*
 * Platform Firmware Runtime Update tool to do Management
 * Mode code injection/driver update and telemetry retrieval.
 *
 * This tool uses the interfaces provided by pfr_update and
 * pfr_telemetry drivers. These interfaces are exposed via
 * /dev/pfr_update and /dev/pfr_telemetry. Write operation
 * on the /dev/pfr_update is to load the EFI capsule into
 * kernel space. Mmap/read operations on /dev/pfr_telemetry
 * could be used to read the telemetry data to user space.
 */
// C dependencies removed from executable Rust:
// stdio.h, stdlib.h, string.h, sys/types.h, sys/stat.h, fcntl.h,
// unistd.h, getopt.h, sys/ioctl.h, sys/mman.h, uuid/uuid.h.
// PFRUT_HEADER supplies the PFRT/PFRU ioctl constants and data layouts.

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

type size_t = usize;
type ssize_t = isize;
type off_t = i64;
type mode_t = c_uint;
type uid_t = c_uint;
type uuid_t = [u8; 16];

const LOG_ERR: c_int = 0;
const LOG_WARN: c_int = 1;
const LOG_INFO: c_int = 2;
const LOG_VERB: c_int = 4;
const LOG_EXEC_IDX: c_int = 0;
const LOG_HISTORY_IDX: c_int = 1;
const REVID_1: c_int = 1;
const REVID_2: c_int = 2;

const O_RDONLY: c_int = 0;
const O_RDWR: c_int = 0o2;
const PROT_READ: c_int = 0x1;
const MAP_SHARED: c_int = 0x01;
const MAP_FAILED: *mut c_void = !0usize as *mut c_void;

const NO_ARGUMENT: c_int = 0;
const REQUIRED_ARGUMENT: c_int = 1;

#[repr(C)]
pub struct stat {
    pub st_dev: u64,
    pub st_ino: u64,
    pub st_nlink: u64,
    pub st_mode: mode_t,
    pub st_uid: uid_t,
    pub st_gid: uid_t,
    pub __pad0: c_int,
    pub st_rdev: u64,
    pub st_size: off_t,
    pub st_blksize: c_long,
    pub st_blocks: c_long,
    pub st_atime: c_long,
    pub st_atime_nsec: c_long,
    pub st_mtime: c_long,
    pub st_mtime_nsec: c_long,
    pub st_ctime: c_long,
    pub st_ctime_nsec: c_long,
    pub __glibc_reserved: [c_long; 3],
}

#[repr(C)]
pub struct option {
    pub name: *const c_char,
    pub has_arg: c_int,
    pub flag: *mut c_int,
    pub val: c_int,
}

#[repr(C)]
pub struct pfru_update_cap_info {
    pub update_cap: c_int,
    pub code_type: uuid_t,
    pub fw_version: c_int,
    pub code_rt_version: c_int,
    pub drv_type: uuid_t,
    pub drv_rt_version: c_int,
    pub drv_svn: c_int,
    pub platform_id: uuid_t,
    pub oem_id: uuid_t,
    pub oem_info_len: c_int,
}

#[repr(C)]
pub struct pfrt_log_data_info {
    pub max_data_size: c_int,
    pub chunk1_size: c_int,
    pub chunk2_size: c_int,
    pub rollover_cnt: c_int,
    pub reset_cnt: c_int,
}

#[repr(C)]
pub struct pfrt_log_info {
    pub log_level: c_int,
    pub log_type: c_int,
    pub log_revid: c_int,
}

unsafe extern "C" {
    static mut optarg: *mut c_char;

    // Constants supplied by PFRUT_HEADER in the original C source.
    static PFRU_IOC_QUERY_CAP: c_ulong;
    static PFRU_IOC_SET_REV: c_ulong;
    static PFRU_IOC_STAGE: c_ulong;
    static PFRU_IOC_ACTIVATE: c_ulong;
    static PFRU_IOC_STAGE_ACTIVATE: c_ulong;
    static PFRT_LOG_IOC_GET_DATA_INFO: c_ulong;
    static PFRT_LOG_IOC_GET_INFO: c_ulong;
    static PFRT_LOG_IOC_SET_INFO: c_ulong;

    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn perror(s: *const c_char);
    fn malloc(size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn exit(status: c_int) -> !;
    fn strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_long;
    fn atoi(nptr: *const c_char) -> c_int;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn basename(path: *mut c_char) -> *mut c_char;
    fn getopt_long_only(
        argc: c_int,
        argv: *mut *mut c_char,
        optstring: *const c_char,
        longopts: *const option,
        longindex: *mut c_int,
    ) -> c_int;
    fn getuid() -> uid_t;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn fstat(fd: c_int, statbuf: *mut stat) -> c_int;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn mmap(
        addr: *mut c_void,
        length: size_t,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: off_t,
    ) -> *mut c_void;
    fn munmap(addr: *mut c_void, length: size_t) -> c_int;
    fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn uuid_unparse(uu: *const u8, out: *mut c_char);

    static mut stderr: *mut c_void;
}

static mut CAPSULE_NAME: *mut c_char = core::ptr::null_mut();
static mut ACTION: c_int = 0;
static mut QUERY_CAP: c_int = 0;
static mut LOG_TYPE: c_int = 0;
static mut LOG_LEVEL: c_int = 0;
static mut LOG_READ: c_int = 0;
static mut LOG_GETINFO: c_int = 0;
static mut REVID: c_int = 0;
static mut LOG_REVID: c_int = 0;
static mut SET_LOG_LEVEL: c_int = 0;
static mut SET_LOG_TYPE: c_int = 0;
static mut SET_REVID: c_int = 0;
static mut SET_LOG_REVID: c_int = 0;

static mut PROGNAME: *mut c_char = core::ptr::null_mut();

static mut OPTION_STRING: [c_char; 16] = [
    b'l' as c_char,
    b':' as c_char,
    b's' as c_char,
    b'a' as c_char,
    b'u' as c_char,
    b'q' as c_char,
    b'd' as c_char,
    b':' as c_char,
    b'G' as c_char,
    b'T' as c_char,
    b':' as c_char,
    b'L' as c_char,
    b':' as c_char,
    b'R' as c_char,
    b'D' as c_char,
    0,
];

static mut LONG_OPTIONS: [option; 13] = [
    option {
        name: c"load".as_ptr(),
        has_arg: REQUIRED_ARGUMENT,
        flag: core::ptr::null_mut(),
        val: b'l' as c_int,
    },
    option {
        name: c"stage".as_ptr(),
        has_arg: NO_ARGUMENT,
        flag: core::ptr::null_mut(),
        val: b's' as c_int,
    },
    option {
        name: c"activate".as_ptr(),
        has_arg: NO_ARGUMENT,
        flag: core::ptr::null_mut(),
        val: b'a' as c_int,
    },
    option {
        name: c"update".as_ptr(),
        has_arg: NO_ARGUMENT,
        flag: core::ptr::null_mut(),
        val: b'u' as c_int,
    },
    option {
        name: c"query".as_ptr(),
        has_arg: NO_ARGUMENT,
        flag: core::ptr::null_mut(),
        val: b'q' as c_int,
    },
    option {
        name: c"getloginfo".as_ptr(),
        has_arg: NO_ARGUMENT,
        flag: core::ptr::null_mut(),
        val: b'G' as c_int,
    },
    option {
        name: c"type".as_ptr(),
        has_arg: REQUIRED_ARGUMENT,
        flag: core::ptr::null_mut(),
        val: b'T' as c_int,
    },
    option {
        name: c"level".as_ptr(),
        has_arg: REQUIRED_ARGUMENT,
        flag: core::ptr::null_mut(),
        val: b'L' as c_int,
    },
    option {
        name: c"read".as_ptr(),
        has_arg: NO_ARGUMENT,
        flag: core::ptr::null_mut(),
        val: b'R' as c_int,
    },
    option {
        name: c"setrev".as_ptr(),
        has_arg: REQUIRED_ARGUMENT,
        flag: core::ptr::null_mut(),
        val: b'd' as c_int,
    },
    option {
        name: c"setrevlog".as_ptr(),
        has_arg: REQUIRED_ARGUMENT,
        flag: core::ptr::null_mut(),
        val: b'D' as c_int,
    },
    option {
        name: c"help".as_ptr(),
        has_arg: NO_ARGUMENT,
        flag: core::ptr::null_mut(),
        val: b'h' as c_int,
    },
    option {
        name: core::ptr::null(),
        has_arg: 0,
        flag: core::ptr::null_mut(),
        val: 0,
    },
];

unsafe fn valid_log_level(level: c_int) -> c_int {
    (level == LOG_ERR || level == LOG_WARN || level == LOG_INFO || level == LOG_VERB) as c_int
}

unsafe fn valid_log_type(type_: c_int) -> c_int {
    (type_ == LOG_EXEC_IDX || type_ == LOG_HISTORY_IDX) as c_int
}

unsafe fn valid_log_revid(id: c_int) -> c_int {
    (id == REVID_1 || id == REVID_2) as c_int
}

unsafe fn help() {
    fprintf(
        stderr,
        c"usage: %s [OPTIONS]\n code injection:\n  -l, --load\n  -s, --stage\n  -a, --activate\n  -u, --update [stage and activate]\n  -q, --query\n  -d, --revid update\n telemetry:\n  -G, --getloginfo\n  -T, --type(0:execution, 1:history)\n  -L, --level(0, 1, 2, 4)\n  -R, --read\n  -D, --revid log\n".as_ptr(),
        PROGNAME,
    );
}

unsafe fn parse_options(argc: c_int, argv: *mut *mut c_char) {
    let mut option_index: c_int = 0;
    let pathname: *mut c_char;
    let mut endptr: *mut c_char = core::ptr::null_mut();
    let mut opt: c_int;

    pathname = strdup(*argv.offset(0));
    PROGNAME = basename(pathname);

    loop {
        opt = getopt_long_only(
            argc,
            argv,
            OPTION_STRING.as_ptr(),
            LONG_OPTIONS.as_ptr(),
            &mut option_index,
        );
        if opt == -1 {
            break;
        }

        match opt {
            x if x == b'l' as c_int => {
                CAPSULE_NAME = optarg;
            }
            x if x == b's' as c_int => {
                ACTION = 1;
            }
            x if x == b'a' as c_int => {
                ACTION = 2;
            }
            x if x == b'u' as c_int => {
                ACTION = 3;
            }
            x if x == b'q' as c_int => {
                QUERY_CAP = 1;
            }
            x if x == b'G' as c_int => {
                LOG_GETINFO = 1;
            }
            x if x == b'T' as c_int => {
                LOG_TYPE = strtol(optarg, &mut endptr, 0) as c_int;
                if *endptr != 0 || (LOG_TYPE != 0 && LOG_TYPE != 1) {
                    printf(c"Number expected: type(0:execution, 1:history) - Quit.\n".as_ptr());
                    exit(1);
                }

                SET_LOG_TYPE = 1;
            }
            x if x == b'L' as c_int => {
                LOG_LEVEL = strtol(optarg, &mut endptr, 0) as c_int;
                if *endptr != 0
                    || (LOG_LEVEL != 0 && LOG_LEVEL != 1 && LOG_LEVEL != 2 && LOG_LEVEL != 4)
                {
                    printf(c"Number expected: level(0, 1, 2, 4) - Quit.\n".as_ptr());
                    exit(1);
                }

                SET_LOG_LEVEL = 1;
            }
            x if x == b'R' as c_int => {
                LOG_READ = 1;
            }
            x if x == b'd' as c_int => {
                REVID = atoi(optarg);
                SET_REVID = 1;
            }
            x if x == b'D' as c_int => {
                LOG_REVID = atoi(optarg);
                SET_LOG_REVID = 1;
            }
            x if x == b'h' as c_int => {
                help();
                exit(0);
            }
            _ => {}
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn print_cap(cap: *mut pfru_update_cap_info) {
    let uuid: *mut c_char;

    uuid = malloc(37) as *mut c_char;
    if uuid.is_null() {
        perror(c"Can not allocate uuid buffer\n".as_ptr());
        exit(1);
    }

    printf(c"update capability:%d\n".as_ptr(), (*cap).update_cap);

    uuid_unparse((*cap).code_type.as_ptr(), uuid);
    printf(c"code injection image type:%s\n".as_ptr(), uuid);
    printf(c"fw_version:%d\n".as_ptr(), (*cap).fw_version);
    printf(c"code_rt_version:%d\n".as_ptr(), (*cap).code_rt_version);

    uuid_unparse((*cap).drv_type.as_ptr(), uuid);
    printf(c"driver update image type:%s\n".as_ptr(), uuid);
    printf(c"drv_rt_version:%d\n".as_ptr(), (*cap).drv_rt_version);
    printf(c"drv_svn:%d\n".as_ptr(), (*cap).drv_svn);

    uuid_unparse((*cap).platform_id.as_ptr(), uuid);
    printf(c"platform id:%s\n".as_ptr(), uuid);
    uuid_unparse((*cap).oem_id.as_ptr(), uuid);
    printf(c"oem id:%s\n".as_ptr(), uuid);
    printf(c"oem information length:%d\n".as_ptr(), (*cap).oem_info_len);

    free(uuid as *mut c_void);
}

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let fd_update: c_int;
    let fd_update_log: c_int;
    let mut fd_capsule: c_int;
    let mut data_info: pfrt_log_data_info = core::mem::zeroed();
    let mut info: pfrt_log_info = core::mem::zeroed();
    let mut cap: pfru_update_cap_info = core::mem::zeroed();
    let mut addr_map_capsule: *mut c_void;
    let mut st: stat = core::mem::zeroed();
    let mut log_buf: *mut c_char;
    let mut ret: c_int;

    if getuid() != 0 {
        printf(c"Please run the tool as root - Exiting.\n".as_ptr());
        return 1;
    }

    parse_options(argc, argv);

    fd_update = open(c"/dev/acpi_pfr_update0".as_ptr(), O_RDWR);
    if fd_update < 0 {
        printf(c"PFRU device not supported - Quit...\n".as_ptr());
        return 1;
    }

    fd_update_log = open(c"/dev/acpi_pfr_telemetry0".as_ptr(), O_RDWR);
    if fd_update_log < 0 {
        printf(c"PFRT device not supported - Quit...\n".as_ptr());
        close(fd_update);
        return 1;
    }

    if QUERY_CAP != 0 {
        ret = ioctl(fd_update, PFRU_IOC_QUERY_CAP, &mut cap);
        if ret != 0 {
            perror(c"Query Update Capability info failed.".as_ptr());
        } else {
            print_cap(&mut cap);
        }

        close(fd_update);
        close(fd_update_log);

        return ret;
    }

    if LOG_GETINFO != 0 {
        ret = ioctl(fd_update_log, PFRT_LOG_IOC_GET_DATA_INFO, &mut data_info);
        if ret != 0 {
            perror(c"Get telemetry data info failed.".as_ptr());
            close(fd_update);
            close(fd_update_log);

            return 1;
        }

        ret = ioctl(fd_update_log, PFRT_LOG_IOC_GET_INFO, &mut info);
        if ret != 0 {
            perror(c"Get telemetry info failed.".as_ptr());
            close(fd_update);
            close(fd_update_log);

            return 1;
        }

        printf(c"log_level:%d\n".as_ptr(), info.log_level);
        printf(c"log_type:%d\n".as_ptr(), info.log_type);
        printf(c"log_revid:%d\n".as_ptr(), info.log_revid);
        printf(c"max_data_size:%d\n".as_ptr(), data_info.max_data_size);
        printf(c"chunk1_size:%d\n".as_ptr(), data_info.chunk1_size);
        printf(c"chunk2_size:%d\n".as_ptr(), data_info.chunk2_size);
        printf(c"rollover_cnt:%d\n".as_ptr(), data_info.rollover_cnt);
        printf(c"reset_cnt:%d\n".as_ptr(), data_info.reset_cnt);
        close(fd_update);
        close(fd_update_log);
        return 0;
    }

    info.log_level = -1;
    info.log_type = -1;
    info.log_revid = -1;

    if SET_LOG_LEVEL != 0 {
        if valid_log_level(LOG_LEVEL) == 0 {
            printf(c"Invalid log level %d\n".as_ptr(), LOG_LEVEL);
        } else {
            info.log_level = LOG_LEVEL;
        }
    }

    if SET_LOG_TYPE != 0 {
        if valid_log_type(LOG_TYPE) == 0 {
            printf(c"Invalid log type %d\n".as_ptr(), LOG_TYPE);
        } else {
            info.log_type = LOG_TYPE;
        }
    }

    if SET_LOG_REVID != 0 {
        if valid_log_revid(LOG_REVID) == 0 {
            printf(c"Invalid log revid %d, unchanged.\n".as_ptr(), LOG_REVID);
        } else {
            info.log_revid = LOG_REVID;
        }
    }

    ret = ioctl(fd_update_log, PFRT_LOG_IOC_SET_INFO, &mut info);
    if ret != 0 {
        perror(c"Log information set failed.(log_level, log_type, log_revid)".as_ptr());
        close(fd_update);
        close(fd_update_log);

        return 1;
    }

    if SET_REVID != 0 {
        ret = ioctl(fd_update, PFRU_IOC_SET_REV, &mut REVID);
        if ret != 0 {
            perror(c"pfru update revid set failed".as_ptr());
            close(fd_update);
            close(fd_update_log);

            return 1;
        }

        printf(c"pfru update revid set to %d\n".as_ptr(), REVID);
    }

    if !CAPSULE_NAME.is_null() {
        fd_capsule = open(CAPSULE_NAME, O_RDONLY);
        if fd_capsule < 0 {
            perror(c"Can not open capsule file...".as_ptr());
            close(fd_update);
            close(fd_update_log);

            return 1;
        }

        if fstat(fd_capsule, &mut st) < 0 {
            perror(c"Can not fstat capsule file...".as_ptr());
            close(fd_capsule);
            close(fd_update);
            close(fd_update_log);

            return 1;
        }

        addr_map_capsule = mmap(
            core::ptr::null_mut(),
            st.st_size as size_t,
            PROT_READ,
            MAP_SHARED,
            fd_capsule,
            0,
        );
        if addr_map_capsule == MAP_FAILED {
            perror(c"Failed to mmap capsule file.".as_ptr());
            close(fd_capsule);
            close(fd_update);
            close(fd_update_log);

            return 1;
        }

        ret = write(fd_update, addr_map_capsule as *const c_char as *const c_void, st.st_size as size_t) as c_int;
        printf(c"Load %d bytes of capsule file into the system\n".as_ptr(), ret);

        if ret == -1 {
            perror(c"Failed to load capsule file".as_ptr());
            munmap(addr_map_capsule, st.st_size as size_t);
            close(fd_capsule);
            close(fd_update);
            close(fd_update_log);

            return 1;
        }

        munmap(addr_map_capsule, st.st_size as size_t);
        close(fd_capsule);
        printf(c"Load done.\n".as_ptr());
    }

    if ACTION != 0 {
        if ACTION == 1 {
            ret = ioctl(fd_update, PFRU_IOC_STAGE, core::ptr::null_mut::<c_void>());
        } else if ACTION == 2 {
            ret = ioctl(fd_update, PFRU_IOC_ACTIVATE, core::ptr::null_mut::<c_void>());
        } else if ACTION == 3 {
            ret = ioctl(
                fd_update,
                PFRU_IOC_STAGE_ACTIVATE,
                core::ptr::null_mut::<c_void>(),
            );
        } else {
            close(fd_update);
            close(fd_update_log);

            return 1;
        }
        printf(c"Update finished, return %d\n".as_ptr(), ret);
    }

    close(fd_update);

    if LOG_READ != 0 {
        let mut p_mmap: *mut c_void;
        let max_data_sz: c_int;

        ret = ioctl(fd_update_log, PFRT_LOG_IOC_GET_DATA_INFO, &mut data_info);
        if ret != 0 {
            perror(c"Get telemetry data info failed.".as_ptr());
            close(fd_update_log);

            return 1;
        }

        max_data_sz = data_info.max_data_size;
        if max_data_sz == 0 {
            printf(c"No telemetry data available.\n".as_ptr());
            close(fd_update_log);

            return 1;
        }

        log_buf = malloc((max_data_sz + 1) as size_t) as *mut c_char;
        if log_buf.is_null() {
            perror(c"log_buf allocate failed.".as_ptr());
            close(fd_update_log);

            return 1;
        }

        p_mmap = mmap(
            core::ptr::null_mut(),
            max_data_sz as size_t,
            PROT_READ,
            MAP_SHARED,
            fd_update_log,
            0,
        );
        if p_mmap == MAP_FAILED {
            perror(c"mmap error.".as_ptr());
            close(fd_update_log);
            free(log_buf as *mut c_void);
            return 1;
        }

        memcpy(log_buf as *mut c_void, p_mmap, max_data_sz as size_t);
        *log_buf.offset(max_data_sz as isize) = b'\0' as c_char;
        printf(c"%s\n".as_ptr(), log_buf);
        free(log_buf as *mut c_void);

        munmap(p_mmap, max_data_sz as size_t);
    }

    close(fd_update_log);

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
