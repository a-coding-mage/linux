// SPDX-License-Identifier: GPL-2.0
//
// soc-link.c
//
// Copyright (C) 2019 Renesas Electronics Corp.
// Kuninori Morimoto <kuninori.morimoto.gx@renesas.com>
//
// C dependencies: <sound/soc.h>, <sound/soc-link.h>

use core::ffi::{c_char, c_int, c_void};

pub const SNDRV_PCM_TRIGGER_START: c_int = 0;
pub const SNDRV_PCM_TRIGGER_STOP: c_int = 1;
pub const SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int = 3;
pub const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int = 4;
pub const SNDRV_PCM_TRIGGER_SUSPEND: c_int = 5;
pub const SNDRV_PCM_TRIGGER_RESUME: c_int = 6;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_substream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_compr_stream {
    pub private_data: *mut snd_soc_pcm_runtime,
}

#[repr(C)]
pub struct snd_soc_ops {
    pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream)>,
    pub hw_params:
        Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
    pub hw_free: Option<unsafe extern "C" fn(*mut snd_pcm_substream)>,
    pub prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int) -> c_int>,
}

#[repr(C)]
pub struct snd_compr_ops {
    pub startup: Option<unsafe extern "C" fn(*mut snd_compr_stream) -> c_int>,
    pub shutdown: Option<unsafe extern "C" fn(*mut snd_compr_stream)>,
    pub set_params: Option<unsafe extern "C" fn(*mut snd_compr_stream) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub name: *const c_char,
    pub init: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime) -> c_int>,
    pub exit: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime)>,
    pub be_hw_params_fixup:
        Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime, *mut snd_pcm_hw_params) -> c_int>,
    pub ops: *mut snd_soc_ops,
    pub compr_ops: *mut snd_compr_ops,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub dev: *mut device,
    pub dai_link: *mut snd_soc_dai_link,
    pub mark_startup: *mut snd_pcm_substream,
    pub mark_hw_params: *mut snd_pcm_substream,
    pub mark_trigger: *mut snd_pcm_substream,
    pub mark_compr_startup: *mut snd_compr_stream,
}

unsafe extern "C" {
    pub fn snd_soc_ret(dev: *mut device, ret: c_int, fmt: *const c_char, ...) -> c_int;
    pub fn snd_soc_substream_to_rtd(
        substream: *mut snd_pcm_substream,
    ) -> *mut snd_soc_pcm_runtime;
}

unsafe fn soc_link_ret(rtd: *mut snd_soc_pcm_runtime, func: *const c_char, ret: c_int) -> c_int {
    unsafe {
        snd_soc_ret(
            (*rtd).dev,
            ret,
            c"at %s() on %s\n".as_ptr(),
            func,
            (*(*rtd).dai_link).name,
        )
    }
}

/*
 * We might want to check substream by using list.
 * In such case, we can update these macros.
 */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_soc_link_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let mut ret: c_int = 0;

    unsafe {
        if let Some(init) = (*(*rtd).dai_link).init {
            ret = init(rtd);
        }

        soc_link_ret(rtd, c"snd_soc_link_init".as_ptr(), ret)
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_soc_link_exit(rtd: *mut snd_soc_pcm_runtime) {
    unsafe {
        if let Some(exit) = (*(*rtd).dai_link).exit {
            exit(rtd);
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_soc_link_be_hw_params_fixup(
    rtd: *mut snd_soc_pcm_runtime,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let mut ret: c_int = 0;

    unsafe {
        if let Some(be_hw_params_fixup) = (*(*rtd).dai_link).be_hw_params_fixup {
            ret = be_hw_params_fixup(rtd, params);
        }

        soc_link_ret(rtd, c"snd_soc_link_be_hw_params_fixup".as_ptr(), ret)
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_soc_link_startup(
    substream: *mut snd_pcm_substream,
) -> c_int {
    let rtd: *mut snd_soc_pcm_runtime = unsafe { snd_soc_substream_to_rtd(substream) };
    let mut ret: c_int = 0;

    unsafe {
        if !(*(*rtd).dai_link).ops.is_null() {
            if let Some(startup) = (*(*(*rtd).dai_link).ops).startup {
                ret = startup(substream);
            }
        }

        /* mark substream if succeeded */
        if ret == 0 {
            (*rtd).mark_startup = substream;
        }

        soc_link_ret(rtd, c"snd_soc_link_startup".as_ptr(), ret)
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_soc_link_shutdown(
    substream: *mut snd_pcm_substream,
    rollback: c_int,
) {
    let rtd: *mut snd_soc_pcm_runtime = unsafe { snd_soc_substream_to_rtd(substream) };

    unsafe {
        if rollback != 0 && (*rtd).mark_startup != substream {
            return;
        }

        if !(*(*rtd).dai_link).ops.is_null() {
            if let Some(shutdown) = (*(*(*rtd).dai_link).ops).shutdown {
                shutdown(substream);
            }
        }

        /* remove marked substream */
        (*rtd).mark_startup = core::ptr::null_mut();
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_soc_link_prepare(
    substream: *mut snd_pcm_substream,
) -> c_int {
    let rtd: *mut snd_soc_pcm_runtime = unsafe { snd_soc_substream_to_rtd(substream) };
    let mut ret: c_int = 0;

    unsafe {
        if !(*(*rtd).dai_link).ops.is_null() {
            if let Some(prepare) = (*(*(*rtd).dai_link).ops).prepare {
                ret = prepare(substream);
            }
        }

        soc_link_ret(rtd, c"snd_soc_link_prepare".as_ptr(), ret)
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_soc_link_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd: *mut snd_soc_pcm_runtime = unsafe { snd_soc_substream_to_rtd(substream) };
    let mut ret: c_int = 0;

    unsafe {
        if !(*(*rtd).dai_link).ops.is_null() {
            if let Some(hw_params) = (*(*(*rtd).dai_link).ops).hw_params {
                ret = hw_params(substream, params);
            }
        }

        /* mark substream if succeeded */
        if ret == 0 {
            (*rtd).mark_hw_params = substream;
        }

        soc_link_ret(rtd, c"snd_soc_link_hw_params".as_ptr(), ret)
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_soc_link_hw_free(
    substream: *mut snd_pcm_substream,
    rollback: c_int,
) {
    let rtd: *mut snd_soc_pcm_runtime = unsafe { snd_soc_substream_to_rtd(substream) };

    unsafe {
        if rollback != 0 && (*rtd).mark_hw_params != substream {
            return;
        }

        if !(*(*rtd).dai_link).ops.is_null() {
            if let Some(hw_free) = (*(*(*rtd).dai_link).ops).hw_free {
                hw_free(substream);
            }
        }

        /* remove marked substream */
        (*rtd).mark_hw_params = core::ptr::null_mut();
    }
}

unsafe fn soc_link_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    let rtd: *mut snd_soc_pcm_runtime = unsafe { snd_soc_substream_to_rtd(substream) };
    let mut ret: c_int = 0;

    unsafe {
        if !(*(*rtd).dai_link).ops.is_null() {
            if let Some(trigger) = (*(*(*rtd).dai_link).ops).trigger {
                ret = trigger(substream, cmd);
            }
        }

        soc_link_ret(rtd, c"soc_link_trigger".as_ptr(), ret)
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_soc_link_trigger(
    substream: *mut snd_pcm_substream,
    cmd: c_int,
    rollback: c_int,
) -> c_int {
    let rtd: *mut snd_soc_pcm_runtime = unsafe { snd_soc_substream_to_rtd(substream) };
    let mut ret: c_int = 0;

    match cmd {
        SNDRV_PCM_TRIGGER_START
        | SNDRV_PCM_TRIGGER_RESUME
        | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => unsafe {
            ret = soc_link_trigger(substream, cmd);
            if ret >= 0 {
                (*rtd).mark_trigger = substream;
            }
        },
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
            unsafe {
                if !(rollback != 0 && (*rtd).mark_trigger != substream) {
                    ret = soc_link_trigger(substream, cmd);
                    (*rtd).mark_startup = core::ptr::null_mut();
                }
            }
        }
        _ => {}
    }

    ret
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_soc_link_compr_startup(
    cstream: *mut snd_compr_stream,
) -> c_int {
    let rtd: *mut snd_soc_pcm_runtime = unsafe { (*cstream).private_data };
    let mut ret: c_int = 0;

    unsafe {
        if !(*(*rtd).dai_link).compr_ops.is_null() {
            if let Some(startup) = (*(*(*rtd).dai_link).compr_ops).startup {
                ret = startup(cstream);
            }
        }

        if ret == 0 {
            (*rtd).mark_compr_startup = cstream;
        }

        soc_link_ret(rtd, c"snd_soc_link_compr_startup".as_ptr(), ret)
    }
}
// EXPORT_SYMBOL_GPL(snd_soc_link_compr_startup);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_soc_link_compr_shutdown(
    cstream: *mut snd_compr_stream,
    rollback: c_int,
) {
    let rtd: *mut snd_soc_pcm_runtime = unsafe { (*cstream).private_data };

    unsafe {
        if rollback != 0 && (*rtd).mark_compr_startup != cstream {
            return;
        }

        if !(*(*rtd).dai_link).compr_ops.is_null() {
            if let Some(shutdown) = (*(*(*rtd).dai_link).compr_ops).shutdown {
                shutdown(cstream);
            }
        }

        (*rtd).mark_compr_startup = core::ptr::null_mut();
    }
}
// EXPORT_SYMBOL_GPL(snd_soc_link_compr_shutdown);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_soc_link_compr_set_params(
    cstream: *mut snd_compr_stream,
) -> c_int {
    let rtd: *mut snd_soc_pcm_runtime = unsafe { (*cstream).private_data };
    let mut ret: c_int = 0;

    unsafe {
        if !(*(*rtd).dai_link).compr_ops.is_null() {
            if let Some(set_params) = (*(*(*rtd).dai_link).compr_ops).set_params {
                ret = set_params(cstream);
            }
        }

        soc_link_ret(rtd, c"snd_soc_link_compr_set_params".as_ptr(), ret)
    }
}
// EXPORT_SYMBOL_GPL(snd_soc_link_compr_set_params);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
