// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Context.c.  Python interfaces for perf script.
 *
 * Copyright (C) 2010 Tom Zanussi <tzanussi@gmail.com>
 */

/*
 * Original C dependencies:
 * Python.h
 * ../../../util/config.h
 * ../../../util/trace-event.h
 * ../../../util/event.h
 * ../../../util/symbol.h
 * ../../../util/thread.h
 * ../../../util/map.h
 * ../../../util/maps.h
 * ../../../util/auxtrace.h
 * ../../../util/session.h
 * ../../../util/srcline.h
 * ../../../util/srccode.h
 *
 * The original C file defines PY_SSIZE_T_CLEAN before including Python.h so
 * that '#' formats use Py_ssize_t.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_void};

type Py_ssize_t = isize;
type u64 = u64;

#[repr(C)]
pub struct PyObject {
    _private: [u8; 0],
}

#[repr(C)]
pub struct PyMethodDef {
    ml_name: *const c_char,
    ml_meth: Option<unsafe extern "C" fn(*mut PyObject, *mut PyObject) -> *mut PyObject>,
    ml_flags: c_int,
    ml_doc: *const c_char,
}

#[repr(C)]
pub struct PyModuleDef_Base {
    _private: [usize; 0],
}

#[repr(C)]
pub struct PyModuleDef {
    m_base: PyModuleDef_Base,
    m_name: *const c_char,
    m_doc: *const c_char,
    m_size: Py_ssize_t,
    m_methods: *mut PyMethodDef,
    m_slots: *mut c_void,
    m_traverse: *mut c_void,
    m_clear: *mut c_void,
    m_free: *mut c_void,
}

#[repr(C)]
pub struct scripting_context {
    sample: *mut perf_sample,
    al: *mut addr_location,
    session: *mut perf_session,
}

#[repr(C)]
pub struct perf_sample {
    ip: u64,
    insn_len: u64,
    insn: *const c_char,
}

#[repr(C)]
pub struct addr_location {
    thread: *mut thread,
    map: *mut map,
    addr: u64,
}

#[repr(C)]
pub struct perf_session {
    itrace_synth_opts: *mut itrace_synth_opts,
}

#[repr(C)]
pub struct itrace_synth_opts {
    set: bool,
}

#[repr(C)]
pub struct thread {
    _private: [u8; 0],
}

#[repr(C)]
pub struct maps {
    _private: [u8; 0],
}

#[repr(C)]
pub struct machine {
    _private: [u8; 0],
}

#[repr(C)]
pub struct map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dso {
    _private: [u8; 0],
}

const METH_VARARGS: c_int = 0x0001;

unsafe extern "C" {
    static mut Py_None: *mut PyObject;
    static PyModuleDef_HEAD_INIT: PyModuleDef_Base;

    fn PyCapsule_GetPointer(capsule: *mut PyObject, name: *const c_char) -> *mut c_void;
    fn PyBytes_FromStringAndSize(v: *const c_char, len: Py_ssize_t) -> *mut PyObject;
    fn PyUnicode_AsUTF8(unicode: *mut PyObject) -> *const c_char;
    fn PyArg_UnpackTuple(
        args: *mut PyObject,
        name: *const c_char,
        min: Py_ssize_t,
        max: Py_ssize_t,
        ...
    ) -> c_int;
    fn PyArg_ParseTuple(args: *mut PyObject, format: *const c_char, ...) -> c_int;
    fn Py_BuildValue(format: *const c_char, ...) -> *mut PyObject;
    fn PyModule_Create(module: *mut PyModuleDef) -> *mut PyObject;
    fn PyObject_SetAttrString(o: *mut PyObject, attr_name: *const c_char, v: *mut PyObject)
        -> c_int;
    fn Py_IncRef(o: *mut PyObject);

    #[cfg(HAVE_LIBTRACEEVENT)]
    fn common_pc(c: *mut scripting_context) -> c_int;
    #[cfg(HAVE_LIBTRACEEVENT)]
    fn common_flags(c: *mut scripting_context) -> c_int;
    #[cfg(HAVE_LIBTRACEEVENT)]
    fn common_lock_depth(c: *mut scripting_context) -> c_int;

    fn thread__maps(thread: *mut thread) -> *mut maps;
    fn maps__machine(maps: *mut maps) -> *mut machine;
    fn perf_sample__fetch_insn(
        sample: *mut perf_sample,
        thread: *mut thread,
        machine: *mut machine,
    );
    fn itrace_do_parse_synth_opts(
        synth_opts: *mut itrace_synth_opts,
        itrace_options: *const c_char,
        flag: c_int,
    ) -> c_int;
    fn map__dso(map: *mut map) -> *mut dso;
    fn map__rip_2objdump(map: *mut map, addr: u64) -> u64;
    fn get_srcline_split(dso: *mut dso, addr: u64, line: *mut c_uint) -> *mut c_char;
    fn find_sourceline(srcfile: *mut c_char, line: c_uint, len: *mut c_int) -> *mut c_char;
    fn free(ptr: *mut c_void);
    fn perf_config_get(config_name: *const c_char) -> *const c_char;
}

unsafe fn _PyCapsule_GetPointer(arg1: *mut PyObject, arg2: *const c_char) -> *mut c_void {
    unsafe { PyCapsule_GetPointer(arg1, arg2) }
}

unsafe fn _PyBytes_FromStringAndSize(arg1: *const c_char, arg2: Py_ssize_t) -> *mut PyObject {
    unsafe { PyBytes_FromStringAndSize(arg1, arg2) }
}

unsafe fn _PyUnicode_AsUTF8(arg: *mut PyObject) -> *const c_char {
    unsafe { PyUnicode_AsUTF8(arg) }
}

unsafe fn get_args(
    args: *mut PyObject,
    name: *const c_char,
    arg2: *mut *mut PyObject,
) -> *mut scripting_context {
    let cnt: c_int = 1 + (!arg2.is_null()) as c_int;
    let mut context: *mut PyObject = core::ptr::null_mut();

    if unsafe {
        PyArg_UnpackTuple(
            args,
            name,
            1,
            cnt as Py_ssize_t,
            &mut context as *mut *mut PyObject,
            arg2,
        )
    } == 0
    {
        return core::ptr::null_mut();
    }

    unsafe { _PyCapsule_GetPointer(context, core::ptr::null()) as *mut scripting_context }
}

unsafe fn get_scripting_context(args: *mut PyObject) -> *mut scripting_context {
    unsafe { get_args(args, c"context".as_ptr(), core::ptr::null_mut()) }
}

#[cfg(HAVE_LIBTRACEEVENT)]
unsafe extern "C" fn perf_trace_context_common_pc(
    _obj: *mut PyObject,
    args: *mut PyObject,
) -> *mut PyObject {
    let c = unsafe { get_scripting_context(args) };

    if c.is_null() {
        return core::ptr::null_mut();
    }

    unsafe { Py_BuildValue(c"i".as_ptr(), common_pc(c)) }
}

#[cfg(HAVE_LIBTRACEEVENT)]
unsafe extern "C" fn perf_trace_context_common_flags(
    _obj: *mut PyObject,
    args: *mut PyObject,
) -> *mut PyObject {
    let c = unsafe { get_scripting_context(args) };

    if c.is_null() {
        return core::ptr::null_mut();
    }

    unsafe { Py_BuildValue(c"i".as_ptr(), common_flags(c)) }
}

#[cfg(HAVE_LIBTRACEEVENT)]
unsafe extern "C" fn perf_trace_context_common_lock_depth(
    _obj: *mut PyObject,
    args: *mut PyObject,
) -> *mut PyObject {
    let c = unsafe { get_scripting_context(args) };

    if c.is_null() {
        return core::ptr::null_mut();
    }

    unsafe { Py_BuildValue(c"i".as_ptr(), common_lock_depth(c)) }
}

unsafe extern "C" fn perf_sample_insn(_obj: *mut PyObject, args: *mut PyObject) -> *mut PyObject {
    let c = unsafe { get_scripting_context(args) };

    if c.is_null() {
        return core::ptr::null_mut();
    }

    unsafe {
        if (*(*c).sample).ip != 0
            && (*(*c).sample).insn_len == 0
            && !thread__maps((*(*c).al).thread).is_null()
        {
            let machine = maps__machine(thread__maps((*(*c).al).thread));

            perf_sample__fetch_insn((*c).sample, (*(*c).al).thread, machine);
        }
        if (*(*c).sample).insn_len == 0 {
            Py_IncRef(Py_None);
            return Py_None; /* N.B. This is a return statement */
        }

        _PyBytes_FromStringAndSize((*(*c).sample).insn, (*(*c).sample).insn_len as Py_ssize_t)
    }
}

unsafe extern "C" fn perf_set_itrace_options(
    _obj: *mut PyObject,
    args: *mut PyObject,
) -> *mut PyObject {
    let c: *mut scripting_context;
    let itrace_options: *const c_char;
    let mut retval: c_int = -1;
    let mut str_: *mut PyObject = core::ptr::null_mut();

    c = unsafe { get_args(args, c"itrace_options".as_ptr(), &mut str_ as *mut *mut PyObject) };
    if c.is_null() {
        return core::ptr::null_mut();
    }

    unsafe {
        if (*c).session.is_null() || (*(*c).session).itrace_synth_opts.is_null() {
            return Py_BuildValue(c"i".as_ptr(), retval);
        }

        if (*(*(*c).session).itrace_synth_opts).set {
            retval = 1;
            return Py_BuildValue(c"i".as_ptr(), retval);
        }

        itrace_options = _PyUnicode_AsUTF8(str_);

        retval = itrace_do_parse_synth_opts((*(*c).session).itrace_synth_opts, itrace_options, 0);
        Py_BuildValue(c"i".as_ptr(), retval)
    }
}

unsafe fn perf_sample_src(
    _obj: *mut PyObject,
    args: *mut PyObject,
    get_srccode: bool,
) -> *mut PyObject {
    let c = unsafe { get_scripting_context(args) };
    let mut line: c_uint = 0;
    let mut srcfile: *mut c_char = core::ptr::null_mut();
    let mut srccode: *mut c_char = core::ptr::null_mut();
    let result: *mut PyObject;
    let map: *mut map;
    let dso: *mut dso;
    let mut len: c_int = 0;
    let addr: u64;

    if c.is_null() {
        return core::ptr::null_mut();
    }

    unsafe {
        map = (*(*c).al).map;
        addr = (*(*c).al).addr;
        dso = if !map.is_null() {
            map__dso(map)
        } else {
            core::ptr::null_mut()
        };

        if !dso.is_null() {
            srcfile = get_srcline_split(dso, map__rip_2objdump(map, addr), &mut line);
        }

        if get_srccode {
            if !srcfile.is_null() {
                srccode = find_sourceline(srcfile, line, &mut len);
            }
            result = Py_BuildValue(
                c"(sIs#)".as_ptr(),
                srcfile,
                line,
                srccode,
                len as Py_ssize_t,
            );
        } else {
            result = Py_BuildValue(c"(sI)".as_ptr(), srcfile, line);
        }

        free(srcfile as *mut c_void);
    }

    result
}

unsafe extern "C" fn perf_sample_srcline(
    obj: *mut PyObject,
    args: *mut PyObject,
) -> *mut PyObject {
    unsafe { perf_sample_src(obj, args, false) }
}

unsafe extern "C" fn perf_sample_srccode(
    obj: *mut PyObject,
    args: *mut PyObject,
) -> *mut PyObject {
    unsafe { perf_sample_src(obj, args, true) }
}

unsafe extern "C" fn __perf_config_get(_obj: *mut PyObject, args: *mut PyObject) -> *mut PyObject {
    let mut config_name: *const c_char = core::ptr::null();

    if unsafe { PyArg_ParseTuple(args, c"s".as_ptr(), &mut config_name as *mut *const c_char) } == 0
    {
        return core::ptr::null_mut();
    }
    unsafe { Py_BuildValue(c"s".as_ptr(), perf_config_get(config_name)) }
}

static mut ContextMethods: [PyMethodDef; 9] = [
    /*
     * If HAVE_LIBTRACEEVENT is enabled in the build, the original C table also
     * contains:
     * common_pc, common_flags, and common_lock_depth.
     */
    #[cfg(HAVE_LIBTRACEEVENT)]
    PyMethodDef {
        ml_name: c"common_pc".as_ptr(),
        ml_meth: Some(perf_trace_context_common_pc),
        ml_flags: METH_VARARGS,
        ml_doc: c"Get the common preempt count event field value.".as_ptr(),
    },
    #[cfg(HAVE_LIBTRACEEVENT)]
    PyMethodDef {
        ml_name: c"common_flags".as_ptr(),
        ml_meth: Some(perf_trace_context_common_flags),
        ml_flags: METH_VARARGS,
        ml_doc: c"Get the common flags event field value.".as_ptr(),
    },
    #[cfg(HAVE_LIBTRACEEVENT)]
    PyMethodDef {
        ml_name: c"common_lock_depth".as_ptr(),
        ml_meth: Some(perf_trace_context_common_lock_depth),
        ml_flags: METH_VARARGS,
        ml_doc: c"Get the common lock depth event field value.".as_ptr(),
    },
    PyMethodDef {
        ml_name: c"perf_sample_insn".as_ptr(),
        ml_meth: Some(perf_sample_insn),
        ml_flags: METH_VARARGS,
        ml_doc: c"Get the machine code instruction.".as_ptr(),
    },
    PyMethodDef {
        ml_name: c"perf_set_itrace_options".as_ptr(),
        ml_meth: Some(perf_set_itrace_options),
        ml_flags: METH_VARARGS,
        ml_doc: c"Set --itrace options.".as_ptr(),
    },
    PyMethodDef {
        ml_name: c"perf_sample_srcline".as_ptr(),
        ml_meth: Some(perf_sample_srcline),
        ml_flags: METH_VARARGS,
        ml_doc: c"Get source file name and line number.".as_ptr(),
    },
    PyMethodDef {
        ml_name: c"perf_sample_srccode".as_ptr(),
        ml_meth: Some(perf_sample_srccode),
        ml_flags: METH_VARARGS,
        ml_doc: c"Get source file name, line number and line.".as_ptr(),
    },
    PyMethodDef {
        ml_name: c"perf_config_get".as_ptr(),
        ml_meth: Some(__perf_config_get),
        ml_flags: METH_VARARGS,
        ml_doc: c"Get perf config entry".as_ptr(),
    },
    PyMethodDef {
        ml_name: core::ptr::null(),
        ml_meth: None,
        ml_flags: 0,
        ml_doc: core::ptr::null(),
    },
];

#[unsafe(no_mangle)]
pub unsafe extern "C" fn PyInit_perf_trace_context() -> *mut PyObject {
    static mut moduledef: PyModuleDef = PyModuleDef {
        m_base: unsafe { PyModuleDef_HEAD_INIT },
        m_name: c"perf_trace_context".as_ptr(), /* m_name */
        m_doc: c"".as_ptr(),                   /* m_doc */
        m_size: -1,                            /* m_size */
        m_methods: unsafe { ContextMethods.as_mut_ptr() }, /* m_methods */
        m_slots: core::ptr::null_mut(),        /* m_reload */
        m_traverse: core::ptr::null_mut(),     /* m_traverse */
        m_clear: core::ptr::null_mut(),        /* m_clear */
        m_free: core::ptr::null_mut(),         /* m_free */
    };
    let mod_: *mut PyObject;

    unsafe {
        mod_ = PyModule_Create(&mut moduledef);
        /* Add perf_script_context to the module so it can be imported */
        PyObject_SetAttrString(mod_, c"perf_script_context".as_ptr(), Py_None);

        mod_
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
