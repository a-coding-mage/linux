// SPDX-License-Identifier: GPL-2.0
// Translated from C implementation source:
// ../../util/util.h // perf_exe()
// ../util.h
// ../../util/evlist.h
// ../../util/hist.h
// ../../util/debug.h
// ../../util/session.h
// ../../util/symbol.h
// ../browser.h
// ../libslang.h
// config.h
// linux/err.h
// linux/string.h
// linux/zalloc.h
// subcmd/exec-cmd.h
// stdlib.h

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_void};

const SCRIPT_NAMELEN: usize = 128;
const SCRIPT_MAX_NO: usize = 64;
/*
 * Usually the full path for a script is:
 *	/home/username/libexec/perf-core/scripts/python/xxx.py
 *	/home/username/libexec/perf-core/scripts/perl/xxx.pl
 * So 256 should be long enough to contain the full path.
 */
const SCRIPT_FULLPATH_LEN: usize = 256;

const BUFSIZ: usize = 8192;
const NAME_MAX: usize = 255;
const PATH_MAX: usize = 4096;
const O_RDONLY: c_int = 0;
const O_DIRECTORY: c_int = 0o200000;
const DT_UNKNOWN: u8 = 0;
const DT_DIR: u8 = 4;
const K_ENTER: c_int = 10;
const PERF_FORMAT_GROUP: u64 = 1 << 3;
const PERF_SAMPLE_BRANCH_STACK: u64 = 1 << 11;
const PERF_SAMPLE_REGS_USER: u64 = 1 << 12;
const PERF_SAMPLE_PHYS_ADDR: u64 = 1 << 19;
const PERF_SAMPLE_REGS_INTR: u64 = 1 << 18;
const PERF_DATA_MODE_READ: c_int = 0;

#[repr(C)]
struct script_config {
    names: *mut *const c_char,
    paths: *mut *mut c_char,
    index: c_int,
    perf: *const c_char,
    extra_format: [c_char; 256],
}

#[repr(C)]
pub struct perf_event_attr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_session {
    evlist: *mut evlist,
}

#[repr(C)]
pub struct evlist {
    _private: [u8; 0],
}

#[repr(C)]
pub struct evsel {
    core: evsel_core,
}

#[repr(C)]
pub struct evsel_core {
    attr: perf_event_attr,
}

#[repr(C)]
pub struct perf_data {
    path: *const c_char,
    mode: c_int,
}

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct DIR {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dirent {
    d_ino: u64,
    d_off: i64,
    d_reclen: u16,
    d_type: u8,
    d_name: [c_char; 256],
}

#[repr(C)]
pub struct symbol_conf_t {
    inline_name: bool,
}

unsafe extern "C" {
    static mut input_name: *const c_char;
    static mut symbol_conf: symbol_conf_t;
    static mut stdout: *mut FILE;

    fn strcat(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    fn strlen(s: *const c_char) -> usize;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn strcspn(s: *const c_char, reject: *const c_char) -> usize;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn system(command: *const c_char) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn sprintf(s: *mut c_char, format: *const c_char, ...) -> c_int;
    fn snprintf(s: *mut c_char, maxlen: usize, format: *const c_char, ...) -> c_int;
    fn asprintf(strp: *mut *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn openat(dirfd: c_int, pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn fdopen(fd: c_int, mode: *const c_char) -> *mut FILE;
    fn fgets(s: *mut c_char, size: c_int, stream: *mut FILE) -> *mut c_char;
    fn fclose(stream: *mut FILE) -> c_int;
    fn fflush(stream: *mut FILE) -> c_int;
    fn fdopendir(fd: c_int) -> *mut DIR;
    fn readdir(dirp: *mut DIR) -> *mut dirent;
    fn closedir(dirp: *mut DIR) -> c_int;

    fn perf_exe(pbuf: *mut c_char, size: usize) -> *const c_char;
    fn strstarts(str_: *const c_char, prefix: *const c_char) -> bool;
    fn skip_spaces(str_: *mut c_char) -> *mut c_char;
    fn scnprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn perf_config(
        fn_: unsafe extern "C" fn(*const c_char, *const c_char, *mut c_void) -> c_int,
        data: *mut c_void,
    ) -> c_int;
    fn get_argv_exec_path() -> *const c_char;
    fn perf_session__new(data: *mut perf_data, repipe: *mut c_void) -> *mut perf_session;
    fn perf_session__delete(session: *mut perf_session);
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn is_directory_at(dir_fd: c_int, name: *const c_char) -> bool;
    fn evlist__first(evlist: *mut evlist) -> *mut evsel;
    fn evlist__next(evlist: *mut evlist, pos: *mut evsel) -> *mut evsel;
    fn evsel__name_is(evsel: *mut evsel, name: *const c_char) -> bool;
    fn ui__popup_menu(argc: c_int, argv: *mut *mut c_char, help: *mut c_void) -> c_int;
    fn ui_browser__input_window(
        title: *const c_char,
        text: *const c_char,
        input: *mut c_char,
        exit_msg: *const c_char,
        delay_secs: c_int,
    ) -> c_int;
    fn zfree(ptr: *mut *mut c_char);
    fn pr_err(fmt: *const c_char, ...);
    fn pr_debug(fmt: *const c_char, ...);
    fn pr_warning(fmt: *const c_char, ...);
    fn SLang_reset_tty();
    fn SLang_init_tty(a: c_int, b: c_int, c: c_int);
    fn SLtty_set_suspend_state(state: bool);
    fn SLsmg_refresh();
}

unsafe fn perf_event_attr_read_format(attr: *mut perf_event_attr) -> u64 {
    *(attr as *mut u64)
}

unsafe fn perf_event_attr_sample_type(attr: *mut perf_event_attr) -> u64 {
    *((attr as *mut u64).add(1))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn attr_to_script(extra_format: *mut c_char, attr: *mut perf_event_attr) {
    unsafe {
        *extra_format.add(0) = 0;
        if perf_event_attr_read_format(attr) & PERF_FORMAT_GROUP != 0 {
            strcat(extra_format, c" -F +metric".as_ptr());
        }
        if perf_event_attr_sample_type(attr) & PERF_SAMPLE_BRANCH_STACK != 0 {
            strcat(extra_format, c" -F +brstackinsn --xed".as_ptr());
        }
        if perf_event_attr_sample_type(attr) & PERF_SAMPLE_REGS_INTR != 0 {
            strcat(extra_format, c" -F +iregs".as_ptr());
        }
        if perf_event_attr_sample_type(attr) & PERF_SAMPLE_REGS_USER != 0 {
            strcat(extra_format, c" -F +uregs".as_ptr());
        }
        if perf_event_attr_sample_type(attr) & PERF_SAMPLE_PHYS_ADDR != 0 {
            strcat(extra_format, c" -F +phys_addr".as_ptr());
        }
    }
}

unsafe extern "C" fn add_script_option(
    name: *const c_char,
    opt: *const c_char,
    c: *mut script_config,
) -> c_int {
    unsafe {
        *(*c).names.add((*c).index as usize) = name;
        if asprintf(
            (*c).paths.add((*c).index as usize),
            c"%s script %s -F +metric %s %s".as_ptr(),
            (*c).perf,
            opt,
            if symbol_conf.inline_name {
                c" --inline".as_ptr()
            } else {
                c"".as_ptr()
            },
            (*c).extra_format.as_ptr(),
        ) < 0
        {
            return -1;
        }
        (*c).index += 1;
        0
    }
}

unsafe extern "C" fn scripts_config(
    var: *const c_char,
    value: *const c_char,
    data: *mut c_void,
) -> c_int {
    unsafe {
        let c = data as *mut script_config;

        if !strstarts(var, c"scripts.".as_ptr()) {
            return -1;
        }
        if (*c).index >= SCRIPT_MAX_NO as c_int {
            return -1;
        }
        *(*c).names.add((*c).index as usize) = strdup(var.add(7));
        if (*(*c).names.add((*c).index as usize)).is_null() {
            return -1;
        }
        if asprintf(
            (*c).paths.add((*c).index as usize),
            c"%s %s".as_ptr(),
            value,
            (*c).extra_format.as_ptr(),
        ) < 0
        {
            return -1;
        }
        (*c).index += 1;
        0
    }
}

/*
 * Some scripts specify the required events in their "xxx-record" file,
 * this function will check if the events in perf.data match those
 * mentioned in the "xxx-record".
 *
 * Fixme: All existing "xxx-record" are all in good formats "-e event ",
 * which is covered well now. And new parsing code should be added to
 * cover the future complex formats like event groups etc.
 */
unsafe extern "C" fn check_ev_match(
    dir_fd: c_int,
    scriptname: *const c_char,
    session: *mut perf_session,
) -> c_int {
    unsafe {
        let mut line = [0 as c_char; BUFSIZ];
        let fp: *mut FILE;

        {
            let mut filename = [0 as c_char; NAME_MAX + 5];
            let fd: c_int;

            scnprintf(
                filename.as_mut_ptr(),
                filename.len(),
                c"bin/%s-record".as_ptr(),
                scriptname,
            );
            fd = openat(dir_fd, filename.as_ptr(), O_RDONLY);
            if fd == -1 {
                return -1;
            }
            fp = fdopen(fd, c"r".as_ptr());
            if fp.is_null() {
                return -1;
            }
        }

        while !fgets(line.as_mut_ptr(), line.len() as c_int, fp).is_null() {
            let mut p = skip_spaces(line.as_mut_ptr());

            if *p == b'#' as c_char {
                continue;
            }

            while strlen(p) != 0 {
                let mut match_: c_int;
                let len: c_int;
                let mut pos: *mut evsel;
                let mut evname = [0 as c_char; 128];

                p = strstr(p, c"-e".as_ptr());
                if p.is_null() {
                    break;
                }

                p = p.add(2);
                p = skip_spaces(p);
                len = strcspn(p, c" \t".as_ptr()) as c_int;
                if len == 0 {
                    break;
                }

                snprintf(evname.as_mut_ptr(), (len + 1) as usize, c"%s".as_ptr(), p);

                match_ = 0;
                pos = evlist__first((*session).evlist);
                while !pos.is_null() {
                    if evsel__name_is(pos, evname.as_ptr()) {
                        match_ = 1;
                        break;
                    }
                    pos = evlist__next((*session).evlist, pos);
                }

                if match_ == 0 {
                    fclose(fp);
                    return -1;
                }
            }
        }

        fclose(fp);
        0
    }
}

/*
 * Return -1 if none is found, otherwise the actual scripts number.
 *
 * Currently the only user of this function is the script browser, which
 * will list all statically runnable scripts, select one, execute it and
 * show the output in a perf browser.
 */
unsafe extern "C" fn find_scripts(
    scripts_array: *mut *mut c_char,
    scripts_path_array: *mut *mut c_char,
    num: c_int,
    pathlen: c_int,
) -> c_int {
    unsafe {
        let mut script_dirent: *mut dirent;
        let mut lang_dirent: *mut dirent;
        let scripts_dir_fd: c_int;
        let mut lang_dir_fd: c_int;
        let scripts_dir: *mut DIR;
        let mut lang_dir: *mut DIR;
        let session: *mut perf_session;
        let mut data = perf_data {
            path: input_name,
            mode: PERF_DATA_MODE_READ,
        };
        let mut temp: *mut c_char;
        let mut i: c_int = 0;
        let exec_path = get_argv_exec_path();

        session = perf_session__new(&mut data, core::ptr::null_mut());
        if IS_ERR(session as *const c_void) {
            return PTR_ERR(session as *const c_void);
        }

        {
            let mut scripts_path = [0 as c_char; PATH_MAX];

            snprintf(
                scripts_path.as_mut_ptr(),
                scripts_path.len(),
                c"%s/scripts".as_ptr(),
                exec_path,
            );
            scripts_dir_fd = open(scripts_path.as_ptr(), O_DIRECTORY);
            pr_err(c"Failed to open directory '%s'".as_ptr(), scripts_path.as_ptr());
            if scripts_dir_fd == -1 {
                perf_session__delete(session);
                return -1;
            }
        }
        scripts_dir = fdopendir(scripts_dir_fd);
        if scripts_dir.is_null() {
            close(scripts_dir_fd);
            perf_session__delete(session);
            return -1;
        }

        loop {
            lang_dirent = readdir(scripts_dir);
            if lang_dirent.is_null() {
                break;
            }
            if (*lang_dirent).d_type != DT_DIR
                && ((*lang_dirent).d_type == DT_UNKNOWN
                    && !is_directory_at(scripts_dir_fd, (*lang_dirent).d_name.as_ptr()))
            {
                continue;
            }
            if strcmp((*lang_dirent).d_name.as_ptr(), c".".as_ptr()) == 0
                || strcmp((*lang_dirent).d_name.as_ptr(), c"..".as_ptr()) == 0
            {
                continue;
            }

            // Original C condition:
            // #ifndef HAVE_LIBPERL_SUPPORT
            if strstr((*lang_dirent).d_name.as_ptr(), c"perl".as_ptr()).is_null() == false {
                continue;
            }
            // #endif
            // Original C condition:
            // #ifndef HAVE_LIBPYTHON_SUPPORT
            if strstr((*lang_dirent).d_name.as_ptr(), c"python".as_ptr()).is_null() == false {
                continue;
            }
            // #endif

            lang_dir_fd = openat(scripts_dir_fd, (*lang_dirent).d_name.as_ptr(), O_DIRECTORY);
            if lang_dir_fd == -1 {
                continue;
            }
            lang_dir = fdopendir(lang_dir_fd);
            if lang_dir.is_null() {
                close(lang_dir_fd);
                continue;
            }
            loop {
                script_dirent = readdir(lang_dir);
                if script_dirent.is_null() {
                    break;
                }
                if (*script_dirent).d_type == DT_DIR {
                    continue;
                }
                if (*script_dirent).d_type == DT_UNKNOWN
                    && is_directory_at(lang_dir_fd, (*script_dirent).d_name.as_ptr())
                {
                    continue;
                }
                /* Skip those real time scripts: xxxtop.p[yl] */
                if !strstr((*script_dirent).d_name.as_ptr(), c"top.".as_ptr()).is_null() {
                    continue;
                }
                if i >= num {
                    break;
                }
                scnprintf(
                    *scripts_path_array.add(i as usize),
                    pathlen as usize,
                    c"%s/scripts/%s/%s".as_ptr(),
                    exec_path,
                    (*lang_dirent).d_name.as_ptr(),
                    (*script_dirent).d_name.as_ptr(),
                );
                temp = strchr((*script_dirent).d_name.as_ptr(), b'.' as c_int);
                snprintf(
                    *scripts_array.add(i as usize),
                    temp.offset_from((*script_dirent).d_name.as_ptr()) as usize + 1,
                    c"%s".as_ptr(),
                    (*script_dirent).d_name.as_ptr(),
                );

                if check_ev_match(lang_dir_fd, *scripts_array.add(i as usize), session) != 0 {
                    continue;
                }

                i += 1;
            }
            closedir(lang_dir);
        }

        closedir(scripts_dir);
        perf_session__delete(session);
        i
    }
}

/*
 * When success, will copy the full path of the selected script
 * into  the buffer pointed by script_name, and return 0.
 * Return -1 on failure.
 */
unsafe extern "C" fn list_scripts(
    script_name: *mut c_char,
    custom: *mut bool,
    evsel: *mut evsel,
) -> c_int {
    unsafe {
        let mut buf: *mut c_char;
        let mut paths = [core::ptr::null_mut::<c_char>(); SCRIPT_MAX_NO];
        let mut names = [core::ptr::null_mut::<c_char>(); SCRIPT_MAX_NO];
        let mut i: c_int;
        let mut num: c_int;
        let choice: c_int;
        let mut ret: c_int = 0;
        let max_std: c_int;
        let custom_perf: c_int;
        let mut pbuf = [0 as c_char; 256];
        let perf = perf_exe(pbuf.as_mut_ptr(), pbuf.len());
        let mut scriptc = script_config {
            names: names.as_mut_ptr() as *mut *const c_char,
            paths: paths.as_mut_ptr(),
            index: 0,
            perf,
            extra_format: [0 as c_char; 256],
        };

        *script_name.add(0) = 0;

        /* Preset the script name to SCRIPT_NAMELEN */
        buf = malloc(SCRIPT_MAX_NO * (SCRIPT_NAMELEN + SCRIPT_FULLPATH_LEN)) as *mut c_char;
        if buf.is_null() {
            return -1;
        }

        if !evsel.is_null() {
            attr_to_script(scriptc.extra_format.as_mut_ptr(), &mut (*evsel).core.attr);
        }
        add_script_option(c"Show individual samples".as_ptr(), c"".as_ptr(), &mut scriptc);
        add_script_option(
            c"Show individual samples with assembler".as_ptr(),
            c"-F +disasm".as_ptr(),
            &mut scriptc,
        );
        add_script_option(
            c"Show individual samples with source".as_ptr(),
            c"-F +srcline,+srccode".as_ptr(),
            &mut scriptc,
        );
        perf_config(scripts_config, &mut scriptc as *mut script_config as *mut c_void);
        custom_perf = scriptc.index;
        add_script_option(
            c"Show samples with custom perf script arguments".as_ptr(),
            c"".as_ptr(),
            &mut scriptc,
        );
        i = scriptc.index;
        max_std = i;

        while i < SCRIPT_MAX_NO as c_int {
            names[i as usize] =
                buf.add((i - max_std) as usize * (SCRIPT_NAMELEN + SCRIPT_FULLPATH_LEN));
            paths[i as usize] = names[i as usize].add(SCRIPT_NAMELEN);
            i += 1;
        }

        num = find_scripts(
            names.as_mut_ptr().add(max_std as usize),
            paths.as_mut_ptr().add(max_std as usize),
            SCRIPT_MAX_NO as c_int - max_std,
            SCRIPT_FULLPATH_LEN as c_int,
        );
        if num < 0 {
            num = 0;
        }
        choice = ui__popup_menu(
            num + max_std,
            names.as_mut_ptr(),
            core::ptr::null_mut(),
        );
        if choice < 0 {
            ret = -1;
            free(buf as *mut c_void);
            i = 0;
            while i < max_std {
                zfree(&mut paths[i as usize]);
                i += 1;
            }
            return ret;
        }
        if choice == custom_perf {
            let mut script_args = [0 as c_char; 50];
            let key = ui_browser__input_window(
                c"perf script command".as_ptr(),
                c"Enter perf script command line (without perf script prefix)".as_ptr(),
                script_args.as_mut_ptr(),
                c"".as_ptr(),
                0,
            );
            if key != K_ENTER {
                ret = -1;
                free(buf as *mut c_void);
                i = 0;
                while i < max_std {
                    zfree(&mut paths[i as usize]);
                    i += 1;
                }
                return ret;
            }
            sprintf(
                script_name,
                c"%s script %s".as_ptr(),
                perf,
                script_args.as_mut_ptr(),
            );
        } else if choice < num + max_std {
            strcpy(script_name, paths[choice as usize]);
        }
        *custom = choice >= max_std;

        free(buf as *mut c_void);
        i = 0;
        while i < max_std {
            zfree(&mut paths[i as usize]);
            i += 1;
        }
        ret
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn run_script(cmd: *mut c_char) {
    unsafe {
        pr_debug(c"Running %s\n".as_ptr(), cmd);
        SLang_reset_tty();
        if system(cmd) < 0 {
            pr_warning(c"Cannot run %s\n".as_ptr(), cmd);
        }
        /*
         * SLang doesn't seem to reset the whole terminal, so be more
         * forceful to get back to the original state.
         */
        printf(c"\x1b[c\x1b[H\x1b[J".as_ptr());
        fflush(stdout);
        SLang_init_tty(0, 0, 0);
        SLtty_set_suspend_state(true);
        SLsmg_refresh();
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn script_browse(script_opt: *const c_char, evsel: *mut evsel) -> c_int {
    unsafe {
        let mut cmd: *mut c_char = core::ptr::null_mut();
        let mut script_name = [0 as c_char; SCRIPT_FULLPATH_LEN];
        let mut custom = false;

        memset(
            script_name.as_mut_ptr() as *mut c_void,
            0,
            SCRIPT_FULLPATH_LEN,
        );
        if list_scripts(script_name.as_mut_ptr(), &mut custom, evsel) != 0 {
            return -1;
        }

        if asprintf(
            &mut cmd,
            c"%s%s %s %s%s 2>&1 | less".as_ptr(),
            if custom {
                c"perf script -s ".as_ptr()
            } else {
                c"".as_ptr()
            },
            script_name.as_mut_ptr(),
            if !script_opt.is_null() {
                script_opt
            } else {
                c"".as_ptr()
            },
            if !input_name.is_null() {
                c"-i ".as_ptr()
            } else {
                c"".as_ptr()
            },
            if !input_name.is_null() {
                input_name
            } else {
                c"".as_ptr()
            },
        ) < 0
        {
            return -1;
        }

        run_script(cmd);
        free(cmd as *mut c_void);

        0
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
