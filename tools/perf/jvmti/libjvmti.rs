// SPDX-License-Identifier: GPL-2.0
//
// Rust translation of perf/jvmti/libjvmti.c.
// External JVMTI/JNI and jvmti_agent symbols are supplied by other files.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uchar, c_void};

type jint = c_int;
type jlong = i64;
type jlocation = jlong;
type jmethodID = *mut c_void;
type jclass = *mut c_void;
type jvmtiError = jint;
type jvmtiEvent = jint;
type jvmtiEventMode = jint;
type jvmtiJlocationFormat = jint;
type size_t = usize;

const JVMTI_ERROR_NONE: jvmtiError = 0;
const JVMTI_ERROR_NULL_POINTER: jvmtiError = 100;
const JVMTI_ERROR_OUT_OF_MEMORY: jvmtiError = 110;
const JVMTI_ERROR_ABSENT_INFORMATION: jvmtiError = 101;
const JVMTI_ERROR_NATIVE_METHOD: jvmtiError = 104;
const JVMTI_ERROR_NOT_FOUND: jvmtiError = 98;
const JVMTI_VERSION_1: jint = 0x3001_0000;
const JVMTI_ENABLE: jvmtiEventMode = 1;
const JVMTI_EVENT_COMPILED_METHOD_LOAD: jvmtiEvent = 68;
const JVMTI_EVENT_DYNAMIC_CODE_GENERATED: jvmtiEvent = 69;
const JVMTI_JLOCATION_JVMBCI: jvmtiJlocationFormat = 1;
const JNI_OK: jint = 0;
const PATH_MAX: usize = 4096;

#[repr(C)]
pub struct jvmtiLineNumberEntry {
    pub start_location: jlocation,
    pub line_number: jint,
}

#[repr(C)]
pub struct jvmtiAddrLocationMap {
    pub start_address: *const c_void,
    pub location: jlocation,
}

#[repr(C)]
pub struct jvmti_line_info_t {
    pub pc: u64,
    pub line_number: jint,
    pub discrim: jint,
    pub methodID: jmethodID,
}

#[repr(C)]
pub struct jvmtiCapabilities {
    pub can_tag_objects_and_more: [u8; 7],
    pub can_generate_compiled_method_load_events: u8,
    pub can_generate_monitor_events_and_more: [u8; 5],
    pub can_get_source_file_name: u8,
    pub can_get_line_numbers: u8,
    pub rest: [u8; 64],
}

#[repr(C)]
pub struct jvmtiEventCallbacks {
    pub VMInit: *mut c_void,
    pub VMDeath: *mut c_void,
    pub ThreadStart: *mut c_void,
    pub ThreadEnd: *mut c_void,
    pub ClassFileLoadHook: *mut c_void,
    pub ClassLoad: *mut c_void,
    pub ClassPrepare: *mut c_void,
    pub VMStart: *mut c_void,
    pub Exception: *mut c_void,
    pub ExceptionCatch: *mut c_void,
    pub SingleStep: *mut c_void,
    pub FramePop: *mut c_void,
    pub Breakpoint: *mut c_void,
    pub FieldAccess: *mut c_void,
    pub FieldModification: *mut c_void,
    pub MethodEntry: *mut c_void,
    pub MethodExit: *mut c_void,
    pub NativeMethodBind: *mut c_void,
    pub CompiledMethodLoad: Option<
        unsafe extern "C" fn(
            *mut jvmtiEnv,
            jmethodID,
            jint,
            *const c_void,
            jint,
            *const jvmtiAddrLocationMap,
            *const c_void,
        ),
    >,
    pub CompiledMethodUnload: *mut c_void,
    pub DynamicCodeGenerated:
        Option<unsafe extern "C" fn(*mut jvmtiEnv, *const c_char, *const c_void, jint)>,
    pub DataDumpRequest: *mut c_void,
    pub reserved: [*mut c_void; 32],
}

#[repr(C)]
pub struct jvmtiInterface_1_ {
    pub reserved_0: [*mut c_void; 39],
    pub GetErrorName:
        Option<unsafe extern "C" fn(*mut jvmtiEnv, jvmtiError, *mut *mut c_char) -> jvmtiError>,
    pub GetClassSignature: Option<
        unsafe extern "C" fn(*mut jvmtiEnv, jclass, *mut *mut c_char, *mut *mut c_char) -> jvmtiError,
    >,
    pub GetClassStatus: *mut c_void,
    pub GetSourceFileName:
        Option<unsafe extern "C" fn(*mut jvmtiEnv, jclass, *mut *mut c_char) -> jvmtiError>,
    pub reserved_1: [*mut c_void; 23],
    pub GetMethodName: Option<
        unsafe extern "C" fn(
            *mut jvmtiEnv,
            jmethodID,
            *mut *mut c_char,
            *mut *mut c_char,
            *mut *mut c_char,
        ) -> jvmtiError,
    >,
    pub GetMethodDeclaringClass:
        Option<unsafe extern "C" fn(*mut jvmtiEnv, jmethodID, *mut jclass) -> jvmtiError>,
    pub reserved_2: [*mut c_void; 34],
    pub GetLineNumberTable: Option<
        unsafe extern "C" fn(
            *mut jvmtiEnv,
            jmethodID,
            *mut jint,
            *mut *mut jvmtiLineNumberEntry,
        ) -> jvmtiError,
    >,
    pub reserved_3: [*mut c_void; 4],
    pub GetJLocationFormat:
        Option<unsafe extern "C" fn(*mut jvmtiEnv, *mut jvmtiJlocationFormat) -> jvmtiError>,
    pub reserved_4: [*mut c_void; 21],
    pub Deallocate: Option<unsafe extern "C" fn(*mut jvmtiEnv, *mut c_uchar) -> jvmtiError>,
    pub reserved_5: [*mut c_void; 7],
    pub AddCapabilities:
        Option<unsafe extern "C" fn(*mut jvmtiEnv, *const jvmtiCapabilities) -> jvmtiError>,
    pub reserved_6: [*mut c_void; 5],
    pub SetEventNotificationMode: Option<
        unsafe extern "C" fn(*mut jvmtiEnv, jvmtiEventMode, jvmtiEvent, *mut c_void) -> jvmtiError,
    >,
    pub reserved_7: [*mut c_void; 2],
    pub SetEventCallbacks:
        Option<unsafe extern "C" fn(*mut jvmtiEnv, *const jvmtiEventCallbacks, jint) -> jvmtiError>,
}

type jvmtiEnv = *const jvmtiInterface_1_;

#[repr(C)]
pub struct JNIInvokeInterface_ {
    pub reserved_0: [*mut c_void; 6],
    pub GetEnv: Option<unsafe extern "C" fn(*mut JavaVM, *mut *mut c_void, jint) -> jint>,
}

type JavaVM = *const JNIInvokeInterface_;

#[cfg(feature = "HAVE_JVMTI_CMLR")]
const JVMTI_CMLR_INLINE_INFO: jint = 4;

#[cfg(feature = "HAVE_JVMTI_CMLR")]
#[repr(C)]
pub struct jvmtiCompiledMethodLoadRecordHeader {
    pub kind: jint,
    pub next: *const jvmtiCompiledMethodLoadRecordHeader,
}

#[cfg(feature = "HAVE_JVMTI_CMLR")]
#[repr(C)]
pub struct PCStackInfo {
    pub pc: *mut c_void,
    pub numstackframes: jint,
    pub methods: *mut jmethodID,
    pub bcis: *mut jint,
}

#[cfg(feature = "HAVE_JVMTI_CMLR")]
#[repr(C)]
pub struct jvmtiCompiledMethodLoadInlineRecord {
    pub header: jvmtiCompiledMethodLoadRecordHeader,
    pub numpcs: jint,
    pub pcinfo: *mut PCStackInfo,
}

unsafe extern "C" {
    fn warnx(fmt: *const c_char, ...);
    fn errx(eval: c_int, fmt: *const c_char, ...) -> !;
    fn malloc(size: size_t) -> *mut c_void;
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn strlen(s: *const c_char) -> size_t;
    fn strcpy(dst: *mut c_char, src: *const c_char) -> *mut c_char;
    fn snprintf(s: *mut c_char, n: size_t, format: *const c_char, ...) -> c_int;
    fn strrchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strlcpy(dst: *mut c_char, src: *const c_char, size: size_t) -> size_t;

    fn jvmti_open() -> *mut c_void;
    fn jvmti_close(agent: *mut c_void) -> c_int;
    fn jvmti_write_code(
        agent: *mut c_void,
        name: *const c_char,
        addr: u64,
        code_addr: *const c_void,
        code_size: jint,
    ) -> c_int;
    fn jvmti_write_debug_info(
        agent: *mut c_void,
        addr: u64,
        nr_lines: c_int,
        line_tab: *const jvmti_line_info_t,
        file_names: *const *const c_char,
    ) -> c_int;
}

static mut has_line_numbers: c_int = 0;

#[unsafe(no_mangle)]
pub static mut jvmti_agent: *mut c_void = core::ptr::null_mut();

unsafe fn print_error(jvmti: *mut jvmtiEnv, msg: *const c_char, ret: jvmtiError) {
    let mut err_msg: *mut c_char = core::ptr::null_mut();
    let err: jvmtiError;

    err = ((*(*jvmti)).GetErrorName.unwrap())(jvmti, ret, &mut err_msg);
    if err == JVMTI_ERROR_NONE {
        warnx(c"%s failed with %s".as_ptr(), msg, err_msg);
        ((*(*jvmti)).Deallocate.unwrap())(jvmti, err_msg as *mut c_uchar);
    } else {
        warnx(c"%s failed with an unknown error %d".as_ptr(), msg, ret);
    }
}

#[cfg(feature = "HAVE_JVMTI_CMLR")]
unsafe fn do_get_line_number(
    jvmti: *mut jvmtiEnv,
    pc: *mut c_void,
    m: jmethodID,
    bci: jint,
    tab: *mut jvmti_line_info_t,
) -> jvmtiError {
    let mut i: jint;
    let mut nr_lines: jint = 0;
    let mut loc_tab: *mut jvmtiLineNumberEntry = core::ptr::null_mut();
    let mut ret: jvmtiError;
    let mut src_line: jint = -1;

    ret = ((*(*jvmti)).GetLineNumberTable.unwrap())(jvmti, m, &mut nr_lines, &mut loc_tab);
    if ret == JVMTI_ERROR_ABSENT_INFORMATION || ret == JVMTI_ERROR_NATIVE_METHOD {
        /* No debug information for this method */
        return ret;
    } else if ret != JVMTI_ERROR_NONE {
        print_error(jvmti, c"GetLineNumberTable".as_ptr(), ret);
        return ret;
    }

    i = 0;
    while i < nr_lines && (*loc_tab.add(i as usize)).start_location <= bci as jlocation {
        src_line = i;
        i += 1;
    }

    if src_line != -1 {
        (*tab).pc = pc as usize as u64;
        (*tab).line_number = (*loc_tab.add(src_line as usize)).line_number;
        (*tab).discrim = 0; /* not yet used */
        (*tab).methodID = m;

        ret = JVMTI_ERROR_NONE;
    } else {
        ret = JVMTI_ERROR_ABSENT_INFORMATION;
    }

    ((*(*jvmti)).Deallocate.unwrap())(jvmti, loc_tab as *mut c_uchar);

    ret
}

#[cfg(feature = "HAVE_JVMTI_CMLR")]
unsafe fn get_line_numbers(
    jvmti: *mut jvmtiEnv,
    compile_info: *const c_void,
    tab: *mut *mut jvmti_line_info_t,
    nr_lines: *mut c_int,
) -> jvmtiError {
    let mut hdr: *const jvmtiCompiledMethodLoadRecordHeader;
    let mut rec: *mut jvmtiCompiledMethodLoadInlineRecord;
    let mut c: *mut PCStackInfo;
    let mut ret: jint;
    let mut nr_total: c_int = 0;
    let mut i: c_int;
    let mut lines_total: c_int = 0;

    if tab.is_null() || nr_lines.is_null() {
        return JVMTI_ERROR_NULL_POINTER;
    }

    /*
     * Phase 1 -- get the number of lines necessary
     */
    hdr = compile_info as *const jvmtiCompiledMethodLoadRecordHeader;
    while !hdr.is_null() {
        if (*hdr).kind == JVMTI_CMLR_INLINE_INFO {
            rec = hdr as *mut jvmtiCompiledMethodLoadInlineRecord;
            nr_total += (*rec).numpcs;
        }
        hdr = (*hdr).next;
    }

    if nr_total == 0 {
        return JVMTI_ERROR_NOT_FOUND;
    }

    /*
     * Phase 2 -- allocate big enough line table
     */
    *tab = calloc(nr_total as size_t, core::mem::size_of::<jvmti_line_info_t>()) as *mut jvmti_line_info_t;
    if (*tab).is_null() {
        return JVMTI_ERROR_OUT_OF_MEMORY;
    }

    hdr = compile_info as *const jvmtiCompiledMethodLoadRecordHeader;
    while !hdr.is_null() {
        if (*hdr).kind == JVMTI_CMLR_INLINE_INFO {
            rec = hdr as *mut jvmtiCompiledMethodLoadInlineRecord;
            i = 0;
            while i < (*rec).numpcs {
                c = (*rec).pcinfo.add(i as usize);
                /*
                 * c->methods is the stack of inlined method calls
                 * at c->pc. [0] is the leaf method. Caller frames
                 * are ignored at the moment.
                 */
                ret = do_get_line_number(
                    jvmti,
                    (*c).pc,
                    *(*c).methods.add(0),
                    *(*c).bcis.add(0),
                    (*tab).add(lines_total as usize),
                );
                if ret == JVMTI_ERROR_NONE {
                    lines_total += 1;
                }
                i += 1;
            }
        }
        hdr = (*hdr).next;
    }
    *nr_lines = lines_total;
    JVMTI_ERROR_NONE
}

#[cfg(not(feature = "HAVE_JVMTI_CMLR"))]
unsafe fn get_line_numbers(
    _jvmti: *mut jvmtiEnv,
    _compile_info: *const c_void,
    _tab: *mut *mut jvmti_line_info_t,
    _nr_lines: *mut c_int,
) -> jvmtiError {
    JVMTI_ERROR_NONE
}

unsafe fn copy_class_filename(
    class_sign: *const c_char,
    file_name: *const c_char,
    result: *mut c_char,
    max_length: size_t,
) {
    /*
    * Assume path name is class hierarchy, this is a common practice with Java programs
    */
    if *class_sign == b'L' as c_char {
        let mut j: size_t;
        let mut i: size_t = 0;
        let p: *const c_char = strrchr(class_sign, b'/' as c_int);
        if !p.is_null() {
            /* drop the 'L' prefix and copy up to the final '/' */
            while i < p.offset_from(class_sign) as size_t {
                *result.add(i) = *class_sign.add(i + 1);
                i += 1;
            }
        }
        /*
        * append file name, we use loops and not string ops to avoid modifying
        * class_sign which is used later for the symbol name
        */
        j = 0;
        while i < max_length - 1 && !file_name.is_null() && j < strlen(file_name) {
            *result.add(i) = *file_name.add(j);
            j += 1;
            i += 1;
        }

        *result.add(i) = 0;
    } else {
        /* fallback case */
        strlcpy(result, file_name, max_length);
    }
}

unsafe fn get_source_filename(
    jvmti: *mut jvmtiEnv,
    methodID: jmethodID,
    buffer: *mut *mut c_char,
) -> jvmtiError {
    let mut ret: jvmtiError;
    let mut decl_class: jclass = core::ptr::null_mut();
    let mut file_name: *mut c_char = core::ptr::null_mut();
    let mut class_sign: *mut c_char = core::ptr::null_mut();
    let mut fn_buf = [0 as c_char; PATH_MAX];
    let len: size_t;

    ret = ((*(*jvmti)).GetMethodDeclaringClass.unwrap())(jvmti, methodID, &mut decl_class);
    if ret != JVMTI_ERROR_NONE {
        print_error(jvmti, c"GetMethodDeclaringClass".as_ptr(), ret);
        return ret;
    }

    ret = ((*(*jvmti)).GetSourceFileName.unwrap())(jvmti, decl_class, &mut file_name);
    if ret != JVMTI_ERROR_NONE {
        print_error(jvmti, c"GetSourceFileName".as_ptr(), ret);
        return ret;
    }

    ret = ((*(*jvmti)).GetClassSignature.unwrap())(
        jvmti,
        decl_class,
        &mut class_sign,
        core::ptr::null_mut(),
    );
    if ret != JVMTI_ERROR_NONE {
        print_error(jvmti, c"GetClassSignature".as_ptr(), ret);
        ((*(*jvmti)).Deallocate.unwrap())(jvmti, file_name as *mut c_uchar);
        return ret;
    }

    copy_class_filename(class_sign, file_name, fn_buf.as_mut_ptr(), PATH_MAX);
    len = strlen(fn_buf.as_ptr());
    *buffer = malloc((len + 1) * core::mem::size_of::<c_char>()) as *mut c_char;
    if (*buffer).is_null() {
        print_error(jvmti, c"GetClassSignature".as_ptr(), ret);
        ret = JVMTI_ERROR_OUT_OF_MEMORY;
        ((*(*jvmti)).Deallocate.unwrap())(jvmti, class_sign as *mut c_uchar);
        ((*(*jvmti)).Deallocate.unwrap())(jvmti, file_name as *mut c_uchar);
        return ret;
    }
    strcpy(*buffer, fn_buf.as_ptr());
    ret = JVMTI_ERROR_NONE;

    ((*(*jvmti)).Deallocate.unwrap())(jvmti, class_sign as *mut c_uchar);
    ((*(*jvmti)).Deallocate.unwrap())(jvmti, file_name as *mut c_uchar);

    ret
}

unsafe fn fill_source_filenames(
    jvmti: *mut jvmtiEnv,
    nr_lines: c_int,
    line_tab: *const jvmti_line_info_t,
    file_names: *mut *mut c_char,
) -> jvmtiError {
    let mut index: c_int;
    let mut ret: jvmtiError;

    index = 0;
    while index < nr_lines {
        ret = get_source_filename(
            jvmti,
            (*line_tab.add(index as usize)).methodID,
            file_names.add(index as usize),
        );
        if ret != JVMTI_ERROR_NONE {
            return ret;
        }
        index += 1;
    }

    JVMTI_ERROR_NONE
}

unsafe extern "C" fn compiled_method_load_cb(
    jvmti: *mut jvmtiEnv,
    method: jmethodID,
    code_size: jint,
    code_addr: *const c_void,
    map_length: jint,
    map: *const jvmtiAddrLocationMap,
    compile_info: *const c_void,
) {
    let mut line_tab: *mut jvmti_line_info_t = core::ptr::null_mut();
    let mut line_file_names: *mut *mut c_char = core::ptr::null_mut();
    let mut decl_class: jclass = core::ptr::null_mut();
    let mut class_sign: *mut c_char = core::ptr::null_mut();
    let mut func_name: *mut c_char = core::ptr::null_mut();
    let mut func_sign: *mut c_char = core::ptr::null_mut();
    let addr: u64 = code_addr as usize as u64;
    let mut ret: jvmtiError;
    let mut nr_lines: c_int = 0; /* in line_tab[] */
    let len: size_t;
    let mut output_debug_info: c_int = 0;

    ret = ((*(*jvmti)).GetMethodDeclaringClass.unwrap())(jvmti, method, &mut decl_class);
    if ret != JVMTI_ERROR_NONE {
        print_error(jvmti, c"GetMethodDeclaringClass".as_ptr(), ret);
        return;
    }

    if has_line_numbers != 0 && !map.is_null() && map_length != 0 {
        ret = get_line_numbers(jvmti, compile_info, &mut line_tab, &mut nr_lines);
        if ret != JVMTI_ERROR_NONE {
            if ret != JVMTI_ERROR_NOT_FOUND {
                warnx(c"jvmti: cannot get line table for method".as_ptr());
            }
            nr_lines = 0;
        } else if nr_lines > 0 {
            line_file_names = calloc(nr_lines as size_t, core::mem::size_of::<*mut c_char>())
                as *mut *mut c_char;
            if line_file_names.is_null() {
                warnx(c"jvmti: cannot allocate space for line table method names".as_ptr());
            } else {
                ret = fill_source_filenames(jvmti, nr_lines, line_tab, line_file_names);
                if ret != JVMTI_ERROR_NONE {
                    warnx(c"jvmti: fill_source_filenames failed".as_ptr());
                } else {
                    output_debug_info = 1;
                }
            }
        }
    }

    ret = ((*(*jvmti)).GetClassSignature.unwrap())(
        jvmti,
        decl_class,
        &mut class_sign,
        core::ptr::null_mut(),
    );
    if ret != JVMTI_ERROR_NONE {
        print_error(jvmti, c"GetClassSignature".as_ptr(), ret);
        goto_error(jvmti, func_name, func_sign, class_sign, line_tab, line_file_names, nr_lines);
        return;
    }

    ret = ((*(*jvmti)).GetMethodName.unwrap())(
        jvmti,
        method,
        &mut func_name,
        &mut func_sign,
        core::ptr::null_mut(),
    );
    if ret != JVMTI_ERROR_NONE {
        print_error(jvmti, c"GetMethodName".as_ptr(), ret);
        goto_error(jvmti, func_name, func_sign, class_sign, line_tab, line_file_names, nr_lines);
        return;
    }

    /*
     * write source line info record if we have it
     */
    if output_debug_info != 0 {
        if jvmti_write_debug_info(
            jvmti_agent,
            addr,
            nr_lines,
            line_tab,
            line_file_names as *const *const c_char,
        ) != 0
        {
            warnx(c"jvmti: write_debug_info() failed".as_ptr());
        }
    }

    len = strlen(func_name) + strlen(class_sign) + strlen(func_sign) + 2;
    {
        let mut str_buf: Vec<c_char> = vec![0; len];
        snprintf(
            str_buf.as_mut_ptr(),
            len,
            c"%s%s%s".as_ptr(),
            class_sign,
            func_name,
            func_sign,
        );

        if jvmti_write_code(jvmti_agent, str_buf.as_ptr(), addr, code_addr, code_size) != 0 {
            warnx(c"jvmti: write_code() failed".as_ptr());
        }
    }

    goto_error(jvmti, func_name, func_sign, class_sign, line_tab, line_file_names, nr_lines);
}

unsafe fn goto_error(
    jvmti: *mut jvmtiEnv,
    func_name: *mut c_char,
    func_sign: *mut c_char,
    class_sign: *mut c_char,
    line_tab: *mut jvmti_line_info_t,
    line_file_names: *mut *mut c_char,
    mut nr_lines: c_int,
) {
    ((*(*jvmti)).Deallocate.unwrap())(jvmti, func_name as *mut c_uchar);
    ((*(*jvmti)).Deallocate.unwrap())(jvmti, func_sign as *mut c_uchar);
    ((*(*jvmti)).Deallocate.unwrap())(jvmti, class_sign as *mut c_uchar);
    free(line_tab as *mut c_void);
    while !line_file_names.is_null() && nr_lines > 0 {
        if !(*line_file_names.add((nr_lines - 1) as usize)).is_null() {
            free(*line_file_names.add((nr_lines - 1) as usize) as *mut c_void);
        }
        nr_lines -= 1;
    }
    free(line_file_names as *mut c_void);
}

unsafe extern "C" fn code_generated_cb(
    _jvmti: *mut jvmtiEnv,
    name: *const c_char,
    code_addr: *const c_void,
    code_size: jint,
) {
    let addr: u64 = code_addr as usize as u64;
    let ret: c_int;

    ret = jvmti_write_code(jvmti_agent, name, addr, code_addr, code_size);
    if ret != 0 {
        warnx(c"jvmti: write_code() failed for code_generated".as_ptr());
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Agent_OnLoad(
    jvm: *mut JavaVM,
    _options: *mut c_char,
    _reserved: *mut c_void,
) -> jint {
    let mut cb: jvmtiEventCallbacks = core::mem::zeroed();
    let mut caps1: jvmtiCapabilities = core::mem::zeroed();
    let mut format: jvmtiJlocationFormat = 0;
    let mut jvmti: *mut jvmtiEnv = core::ptr::null_mut();
    let mut ret: jint;

    jvmti_agent = jvmti_open();
    if jvmti_agent.is_null() {
        warnx(c"jvmti: open_agent failed".as_ptr());
        return -1;
    }

    /*
     * Request a JVMTI interface version 1 environment
     */
    ret = ((*(*jvm)).GetEnv.unwrap())(jvm, &mut jvmti as *mut *mut jvmtiEnv as *mut *mut c_void, JVMTI_VERSION_1);
    if ret != JNI_OK {
        warnx(c"jvmti: jvmti version 1 not supported".as_ptr());
        return -1;
    }

    /*
     * acquire method_load capability, we require it
     * request line numbers (optional)
     */
    caps1 = core::mem::zeroed();
    caps1.can_generate_compiled_method_load_events = 1;

    ret = ((*(*jvmti)).AddCapabilities.unwrap())(jvmti, &caps1);
    if ret != JVMTI_ERROR_NONE {
        print_error(jvmti, c"AddCapabilities".as_ptr(), ret);
        return -1;
    }
    ret = ((*(*jvmti)).GetJLocationFormat.unwrap())(jvmti, &mut format);
    if ret == JVMTI_ERROR_NONE && format == JVMTI_JLOCATION_JVMBCI {
        caps1 = core::mem::zeroed();
        caps1.can_get_line_numbers = 1;
        caps1.can_get_source_file_name = 1;
        ret = ((*(*jvmti)).AddCapabilities.unwrap())(jvmti, &caps1);
        if ret == JVMTI_ERROR_NONE {
            has_line_numbers = 1;
        }
    } else if ret != JVMTI_ERROR_NONE {
        print_error(jvmti, c"GetJLocationFormat".as_ptr(), ret);
    }

    cb = core::mem::zeroed();

    cb.CompiledMethodLoad = Some(compiled_method_load_cb);
    cb.DynamicCodeGenerated = Some(code_generated_cb);

    ret = ((*(*jvmti)).SetEventCallbacks.unwrap())(
        jvmti,
        &cb,
        core::mem::size_of::<jvmtiEventCallbacks>() as jint,
    );
    if ret != JVMTI_ERROR_NONE {
        print_error(jvmti, c"SetEventCallbacks".as_ptr(), ret);
        return -1;
    }

    ret = ((*(*jvmti)).SetEventNotificationMode.unwrap())(
        jvmti,
        JVMTI_ENABLE,
        JVMTI_EVENT_COMPILED_METHOD_LOAD,
        core::ptr::null_mut(),
    );
    if ret != JVMTI_ERROR_NONE {
        print_error(jvmti, c"SetEventNotificationMode(METHOD_LOAD)".as_ptr(), ret);
        return -1;
    }

    ret = ((*(*jvmti)).SetEventNotificationMode.unwrap())(
        jvmti,
        JVMTI_ENABLE,
        JVMTI_EVENT_DYNAMIC_CODE_GENERATED,
        core::ptr::null_mut(),
    );
    if ret != JVMTI_ERROR_NONE {
        print_error(jvmti, c"SetEventNotificationMode(CODE_GENERATED)".as_ptr(), ret);
        return -1;
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Agent_OnUnload(_jvm: *mut JavaVM) {
    let ret: c_int;

    ret = jvmti_close(jvmti_agent);
    if ret != 0 {
        errx(1, c"Error: op_close_agent()".as_ptr());
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
