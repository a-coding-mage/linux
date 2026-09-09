/* SPDX-License-Identifier: GPL-2.0
 *
 * compress_driver.h - compress offload driver definitions
 *
 * Translated from the C header. Types and functions supplied by included
 * kernel headers remain external dependencies.
 */

#[repr(C)]
pub struct snd_compr_task_runtime {
    pub list: list_head,
    pub input: *mut dma_buf,
    pub output: *mut dma_buf,
    pub seqno: u64,
    pub input_size: u64,
    pub output_size: u64,
    pub flags: u32,
    pub state: u8,
    pub private_value: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct snd_compr_runtime {
    pub state: snd_pcm_state_t,
    pub ops: *mut snd_compr_ops,
    pub buffer: *mut core::ffi::c_void,
    pub buffer_size: u64,
    pub fragment_size: u32,
    pub fragments: u32,
    pub total_bytes_available: u64,
    pub total_bytes_transferred: u64,
    pub sleep: wait_queue_head_t,
    pub private_data: *mut core::ffi::c_void,
    pub dma_area: *mut u8,
    pub dma_addr: dma_addr_t,
    pub dma_bytes: usize,
    pub dma_buffer_p: *mut snd_dma_buffer,
    #[cfg(feature = "CONFIG_SND_COMPRESS_ACCEL")]
    pub active_tasks: u32,
    #[cfg(feature = "CONFIG_SND_COMPRESS_ACCEL")]
    pub total_tasks: u32,
    #[cfg(feature = "CONFIG_SND_COMPRESS_ACCEL")]
    pub task_seqno: u64,
    #[cfg(feature = "CONFIG_SND_COMPRESS_ACCEL")]
    pub tasks: list_head,
}

#[repr(C)]
pub struct snd_compr_stream {
    pub name: *const core::ffi::c_char,
    pub ops: *mut snd_compr_ops,
    pub runtime: *mut snd_compr_runtime,
    pub device: *mut snd_compr,
    pub error_work: delayed_work,
    pub direction: snd_compr_direction,
    pub metadata_set: bool,
    pub next_track: bool,
    pub partial_drain: bool,
    pub pause_in_draining: bool,
    pub private_data: *mut core::ffi::c_void,
    pub dma_buffer: snd_dma_buffer,
}

#[repr(C)]
pub struct snd_compr_ops {
    pub open: Option<unsafe extern "C" fn(*mut snd_compr_stream) -> core::ffi::c_int>,
    pub free: Option<unsafe extern "C" fn(*mut snd_compr_stream) -> core::ffi::c_int>,
    pub set_params: Option<unsafe extern "C" fn(*mut snd_compr_stream, *mut snd_compr_params) -> core::ffi::c_int>,
    pub get_params: Option<unsafe extern "C" fn(*mut snd_compr_stream, *mut snd_codec) -> core::ffi::c_int>,
    pub set_metadata: Option<unsafe extern "C" fn(*mut snd_compr_stream, *mut snd_compr_metadata) -> core::ffi::c_int>,
    pub get_metadata: Option<unsafe extern "C" fn(*mut snd_compr_stream, *mut snd_compr_metadata) -> core::ffi::c_int>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_compr_stream, core::ffi::c_int) -> core::ffi::c_int>,
    pub pointer: Option<unsafe extern "C" fn(*mut snd_compr_stream, *mut snd_compr_tstamp64) -> core::ffi::c_int>,
    pub copy: Option<unsafe extern "C" fn(*mut snd_compr_stream, *mut core::ffi::c_char, usize) -> core::ffi::c_int>,
    pub mmap: Option<unsafe extern "C" fn(*mut snd_compr_stream, *mut vm_area_struct) -> core::ffi::c_int>,
    pub ack: Option<unsafe extern "C" fn(*mut snd_compr_stream, usize) -> core::ffi::c_int>,
    pub get_caps: Option<unsafe extern "C" fn(*mut snd_compr_stream, *mut snd_compr_caps) -> core::ffi::c_int>,
    pub get_codec_caps: Option<unsafe extern "C" fn(*mut snd_compr_stream, *mut snd_compr_codec_caps) -> core::ffi::c_int>,
    #[cfg(feature = "CONFIG_SND_COMPRESS_ACCEL")]
    pub task_create: Option<unsafe extern "C" fn(*mut snd_compr_stream, *mut snd_compr_task_runtime) -> core::ffi::c_int>,
    #[cfg(feature = "CONFIG_SND_COMPRESS_ACCEL")]
    pub task_start: Option<unsafe extern "C" fn(*mut snd_compr_stream, *mut snd_compr_task_runtime) -> core::ffi::c_int>,
    #[cfg(feature = "CONFIG_SND_COMPRESS_ACCEL")]
    pub task_stop: Option<unsafe extern "C" fn(*mut snd_compr_stream, *mut snd_compr_task_runtime) -> core::ffi::c_int>,
    #[cfg(feature = "CONFIG_SND_COMPRESS_ACCEL")]
    pub task_free: Option<unsafe extern "C" fn(*mut snd_compr_stream, *mut snd_compr_task_runtime) -> core::ffi::c_int>,
}

#[repr(C)]
pub struct snd_compr {
    pub name: *const core::ffi::c_char,
    pub dev: *mut device,
    pub ops: *mut snd_compr_ops,
    pub private_data: *mut core::ffi::c_void,
    pub card: *mut snd_card,
    pub direction: core::ffi::c_uint,
    pub lock: mutex,
    pub device: core::ffi::c_int,
    pub use_pause_in_draining: bool,
    #[cfg(feature = "CONFIG_SND_VERBOSE_PROCFS")]
    pub id: [core::ffi::c_char; 64],
    #[cfg(feature = "CONFIG_SND_VERBOSE_PROCFS")]
    pub proc_root: *mut snd_info_entry,
    #[cfg(feature = "CONFIG_SND_VERBOSE_PROCFS")]
    pub proc_info_entry: *mut snd_info_entry,
}

extern "C" {
    pub fn snd_compress_new(card: *mut snd_card, device: core::ffi::c_int,
                            type_: core::ffi::c_int, id: *const core::ffi::c_char,
                            compr: *mut snd_compr) -> core::ffi::c_int;
    pub fn snd_compr_malloc_pages(stream: *mut snd_compr_stream, size: usize) -> core::ffi::c_int;
    pub fn snd_compr_free_pages(stream: *mut snd_compr_stream) -> core::ffi::c_int;
    pub fn snd_compr_stop_error(stream: *mut snd_compr_stream, state: snd_pcm_state_t) -> core::ffi::c_int;
    #[cfg(feature = "CONFIG_SND_COMPRESS_ACCEL")]
    pub fn snd_compr_task_finished(stream: *mut snd_compr_stream, task: *mut snd_compr_task_runtime);
}

#[inline]
pub unsafe fn snd_compr_use_pause_in_draining(substream: *mut snd_compr_stream) {
    (*(*substream).device).use_pause_in_draining = true;
}

#[inline]
pub unsafe fn snd_compr_fragment_elapsed(stream: *mut snd_compr_stream) {
    wake_up(&mut (*(*stream).runtime).sleep);
}

#[inline]
pub unsafe fn snd_compr_drain_notify(stream: *mut snd_compr_stream) {
    if snd_BUG_ON(stream.is_null()) {
        return;
    }
    if (*stream).partial_drain {
        (*(*stream).runtime).state = SNDRV_PCM_STATE_RUNNING;
        (*stream).partial_drain = false;
    } else {
        (*(*stream).runtime).state = SNDRV_PCM_STATE_SETUP;
    }
    wake_up(&mut (*(*stream).runtime).sleep);
}

#[inline]
pub unsafe fn snd_compr_set_runtime_buffer(stream: *mut snd_compr_stream, bufp: *mut snd_dma_buffer) {
    let runtime = (*stream).runtime;
    if !bufp.is_null() {
        (*runtime).dma_buffer_p = bufp;
        (*runtime).dma_area = (*bufp).area;
        (*runtime).dma_addr = (*bufp).addr;
        (*runtime).dma_bytes = (*bufp).bytes;
    } else {
        (*runtime).dma_buffer_p = core::ptr::null_mut();
        (*runtime).dma_area = core::ptr::null_mut();
        (*runtime).dma_addr = 0;
        (*runtime).dma_bytes = 0;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
