// SPDX-License-Identifier: GPL-2.0-or-later
//
// USX2Y "rawusb" aka hwdep_pcm implementation
//
// Its usb's unableness to atomically handle power of 2 period sized data chuncs
// at standard samplerates,
// what led to this part of the usx2y module:
// It provides the alsa kernel half of the usx2y-alsa-jack driver pair.
// The pair uses a hardware dependent alsa-device for mmaped pcm transport.
// Advantage achieved:
//         The usb_hc moves pcm data from/into memory via DMA.
//         That memory is mmaped by jack's usx2y driver.
//         Jack's usx2y driver is the first/last to read/write pcm data.
//         Read/write is a combination of power of 2 period shaping and
//         float/int conversation.
//         Compared to mainline alsa/jack we leave out power of 2 period shaping inside
//         snd-usb-usx2y which needs memcpy() and additional buffers.
//         As a side effect possible unwanted pcm-data coruption resulting of
//         standard alsa's snd-usb-usx2y period shaping scheme falls away.
//         Result is sane jack operation at buffering schemes down to 128frames,
//         2 periods.
//         plain usx2y alsa mode is able to achieve 64frames, 4periods, but only at the
//         cost of easier triggered i.e. aeolus xruns (128 or 256frames,
//         2periods works but is useless cause of crackling).
//
// This is a first "proof of concept" implementation.
// Later, functionalities should migrate to more appropriate places:
// Userland:
// - The jackd could mmap its float-pcm buffers directly from alsa-lib.
// - alsa-lib could provide power of 2 period sized shaping combined with int/float
//   conversation.
//   Currently the usx2y jack driver provides above 2 services.
// Kernel:
// - rawusb dma pcm buffer transport should go to snd-usb-lib, so also snd-usb-audio
//   devices can use it.
//   Currently rawusb dma pcm buffer transport (this file) is only available to snd-usb-usx2y.

// Dependencies: linux/delay.h, linux/gfp.h, usbusx2yaudio.c
// Build condition: USX2Y_NRPACKS_VARIABLE or USX2Y_NRPACKS == 1

#[cfg(any(feature = "USX2Y_NRPACKS_VARIABLE", feature = "USX2Y_NRPACKS_1"))]
pub mod usx2y_hwdep_pcm {
    use core::mem;
    use core::ptr;

    // External types and functions (defined elsewhere)
    // These are declarations for types defined in other modules
    #[repr(C)]
    pub struct snd_usx2y_substream {
        pub completed_urb: *mut urb,
        pub pcm_substream: *mut snd_pcm_substream,
        pub hwptr_done: i32,
        pub transfer_done: i32,
        pub usx2y: *mut usx2ydev,
        pub endpoint: u32,
        pub state: atomic_t,
        pub urb: [*mut urb; 8],
        pub maxpacksize: u32,
    }

    #[repr(C)]
    pub struct snd_pcm_runtime {
        pub buffer_size: u32,
        pub period_size: u32,
        pub format: u32,
        pub rate: u32,
        pub private_data: *mut core::ffi::c_void,
    }

    #[repr(C)]
    pub struct snd_pcm_substream {
        pub runtime: *mut snd_pcm_runtime,
        pub stream: i32,
    }

    #[repr(C)]
    pub struct usx2ydev {
        pub dev: *mut usb_device,
        pub hwdep_pcm_shm: *mut snd_usx2y_hwdep_pcm_shm,
        pub stride: u32,
        pub rate: u32,
        pub format: u32,
        pub subs: [*mut snd_usx2y_substream; 4],
        pub wait_iso_frame: i32,
        pub prepare_subs: *mut snd_usx2y_substream,
        pub chip_status: u32,
    }

    #[repr(C)]
    pub struct snd_usx2y_hwdep_pcm_shm {
        pub capture_iso_head: i32,
        pub captured_iso_head: i32,
        pub captured_iso_frames: i32,
        pub captured_iso: [captured_iso_info; 8],
        pub capture_iso_start: i32,
        pub playback_iso_start: i32,
        pub playback_iso_head: i32,
        pub playback: [u8; 0x4000],
        pub capture0x8: [u8; 0x8000],
        pub capture0xA: [u8; 0x8000],
    }

    #[repr(C)]
    pub struct captured_iso_info {
        pub frame: i32,
        pub offset: u32,
        pub length: u32,
    }

    #[repr(C)]
    pub struct usb_device {
        pub bus: *mut usb_bus,
        pub devnum: i32,
    }

    #[repr(C)]
    pub struct usb_bus {
        pub busnum: i32,
    }

    #[repr(C)]
    pub struct urb {
        pub iso_frame_desc: [usb_iso_packet_descriptor; 0],
        pub status: i32,
        pub start_frame: i32,
        pub pipe: u32,
        pub transfer_buffer: *mut core::ffi::c_void,
        pub transfer_buffer_length: u32,
        pub context: *mut core::ffi::c_void,
        pub interval: u32,
        pub complete: Option<fn(*mut urb)>,
        pub transfer_flags: u32,
        pub dev: *mut usb_device,
        pub number_of_packets: i32,
    }

    #[repr(C)]
    pub struct usb_iso_packet_descriptor {
        pub offset: u32,
        pub length: u32,
        pub actual_length: u32,
        pub status: i32,
    }

    #[repr(C)]
    pub struct snd_card {
        _private: [u8; 0],
    }

    #[repr(C)]
    pub struct snd_hwdep {
        pub card: *mut snd_card,
        pub private_data: *mut core::ffi::c_void,
        pub private_free: Option<fn(*mut snd_hwdep)>,
        pub ops: snd_hwdep_ops,
        pub iface: i32,
        pub exclusive: i32,
        pub name: [u8; 32],
    }

    #[repr(C)]
    pub struct snd_hwdep_ops {
        pub open: Option<fn(*mut snd_hwdep, *mut core::ffi::c_void) -> i32>,
        pub release: Option<fn(*mut snd_hwdep, *mut core::ffi::c_void) -> i32>,
        pub mmap: Option<fn(*mut snd_hwdep, *mut core::ffi::c_void, *mut vm_area_struct) -> i32>,
    }

    #[repr(C)]
    pub struct snd_pcm {
        pub streams: [snd_pcm_str; 2],
        pub private_data: *mut core::ffi::c_void,
        pub info_flags: u32,
        pub name: [u8; 80],
    }

    #[repr(C)]
    pub struct snd_pcm_str {
        pub substream: *mut snd_pcm_substream,
    }

    #[repr(C)]
    pub struct snd_pcm_ops {
        pub open: Option<fn(*mut snd_pcm_substream) -> i32>,
        pub close: Option<fn(*mut snd_pcm_substream) -> i32>,
        pub hw_params: Option<fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> i32>,
        pub hw_free: Option<fn(*mut snd_pcm_substream) -> i32>,
        pub prepare: Option<fn(*mut snd_pcm_substream) -> i32>,
        pub trigger: Option<fn(*mut snd_pcm_substream, i32) -> i32>,
        pub pointer: Option<fn(*mut snd_pcm_substream) -> u32>,
    }

    #[repr(C)]
    pub struct snd_pcm_hardware {
        pub info: u32,
        pub formats: u64,
        pub rates: u32,
        pub rate_min: u32,
        pub rate_max: u32,
        pub channels_min: u32,
        pub channels_max: u32,
        pub buffer_bytes_max: u32,
        pub period_bytes_min: u32,
        pub period_bytes_max: u32,
        pub periods_min: u32,
        pub periods_max: u32,
        pub fifo_size: u32,
    }

    #[repr(C)]
    pub struct snd_pcm_hw_params {
        _private: [u8; 0],
    }

    #[repr(C)]
    pub struct vm_area_struct {
        pub vm_start: u64,
        pub vm_end: u64,
        pub vm_ops: *const vm_operations_struct,
        pub vm_private_data: *mut core::ffi::c_void,
        pub vma: *mut vm_area_struct,
        pub pgoff: u64,
        pub page: *mut page,
    }

    #[repr(C)]
    pub struct vm_fault {
        pub vma: *mut vm_area_struct,
        pub pgoff: u64,
        pub page: *mut page,
    }

    #[repr(C)]
    pub struct vm_operations_struct {
        pub open: Option<fn(*mut vm_area_struct)>,
        pub close: Option<fn(*mut vm_area_struct)>,
        pub fault: Option<fn(*mut vm_fault) -> i32>,
    }

    #[repr(C)]
    pub struct page {
        _private: [u8; 0],
    }

    #[repr(C)]
    pub struct atomic_t {
        pub counter: i32,
    }

    // Constants
    const SNDRV_PCM_STREAM_CAPTURE: i32 = 0;
    const SNDRV_PCM_STREAM_PLAYBACK: i32 = 1;
    const SNDRV_PCM_STATE_PREPARED: i32 = 2;
    const STATE_STOPPED: i32 = 0;
    const STATE_PREPARED: i32 = 1;
    const STATE_PRERUNNING: i32 = 2;
    const STATE_STARTING1: i32 = 3;
    const STATE_STARTING2: i32 = 4;
    const STATE_STARTING3: i32 = 5;
    const STATE_RUNNING: i32 = 6;
    const USX2Y_STAT_CHIP_MMAP_PCM_URBS: u32 = 0x2;
    const USX2Y_STAT_CHIP_INIT: u32 = 0x1;
    const SNDRV_PCM_INFO_MMAP: u32 = 0x1;
    const SNDRV_PCM_INFO_INTERLEAVED: u32 = 0x2;
    const SNDRV_PCM_INFO_BLOCK_TRANSFER: u32 = 0x4;
    const SNDRV_PCM_INFO_MMAP_VALID: u32 = 0x8;
    const SNDRV_PCM_FMTBIT_S16_LE: u64 = 1u64 << 0;
    const SNDRV_PCM_FMTBIT_S24_3LE: u64 = 1u64 << 3;
    const SNDRV_PCM_RATE_44100: u32 = 1u32 << 0;
    const SNDRV_PCM_RATE_48000: u32 = 1u32 << 1;
    const SNDRV_HWDEP_IFACE_USX2Y_PCM: i32 = 0x10;
    const SNDRV_DMA_TYPE_CONTINUOUS: i32 = 0;
    const GFP_KERNEL: i32 = 0xd0;
    const VM_DONTEXPAND: u32 = 0x1000;
    const VM_DONTDUMP: u32 = 0x2000;
    const VM_FAULT_SIGBUS: i32 = 1;
    const EPIPE: i32 = 32;
    const EINVAL: i32 = 22;
    const ENOMEM: i32 = 12;
    const ERESTARTSYS: i32 = 512;
    const EBUSY: i32 = 16;
    const ENODEV: i32 = 19;
    const PAGE_SHIFT: u64 = 12;
    const NRURBS: usize = 2;
    const NAME_ALLCAPS: &[u8] = b"USX2Y";
    const SND_USX2Y_USBPCM_ID: &[u8] = b"USX2Y HWDEP PCM";

    // External function declarations
    extern "C" {
        fn nr_of_packs() -> i32;
        fn usx2y_urb_play_retire(subs: *mut snd_usx2y_substream, urb: *mut urb) -> i32;
        fn usx2y_error_urb_status(usx2y: *mut usx2ydev, subs: *mut snd_usx2y_substream, urb: *mut urb);
        fn usx2y_urbs_set_complete(usx2y: *mut usx2ydev, complete: extern "C" fn(*mut urb));
        fn usx2y_urb_submit(subs: *mut snd_usx2y_substream, urb: *mut urb, frame: i32) -> i32;
        fn usx2y_clients_stop(usx2y: *mut usx2ydev);
        fn usx2y_subs_prepare(subs: *mut snd_usx2y_substream);
        fn usx2y_format_set(usx2y: *mut usx2ydev, format: u32) -> i32;
        fn usx2y_rate_set(usx2y: *mut usx2ydev, rate: u32) -> i32;
        fn usx2y_subs_startup_finish(usx2y: *mut usx2ydev);
        fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
        fn snd_pcm_substream_chip(substream: *mut snd_pcm_substream) -> *mut core::ffi::c_void;
        fn snd_pcm_hw_constraint_minmax(
            runtime: *mut snd_pcm_runtime,
            var: u32,
            min: u32,
            max: u32,
        ) -> i32;
        fn snd_hwdep_new(card: *mut snd_card, id: *const u8, device: i32, rhw: *mut *mut snd_hwdep) -> i32;
        fn snd_pcm_new(
            card: *mut snd_card,
            id: *const u8,
            device: i32,
            playback_count: i32,
            capture_count: i32,
            rpcm: *mut *mut snd_pcm,
        ) -> i32;
        fn snd_pcm_set_ops(pcm: *mut snd_pcm, direction: i32, ops: *const snd_pcm_ops);
        fn snd_pcm_set_managed_buffer(
            substream: *mut snd_pcm_substream,
            dma_type: i32,
            dev: *mut core::ffi::c_void,
            prealloc_size: u32,
            max_size: u32,
        );
        fn alloc_pages_exact(size: u32, flags: i32) -> *mut core::ffi::c_void;
        fn free_pages_exact(addr: *mut core::ffi::c_void, size: u32);
        fn memset(s: *mut core::ffi::c_void, c: i32, n: u32) -> *mut core::ffi::c_void;
        fn virt_to_page(addr: *mut core::ffi::c_void) -> *mut page;
        fn get_page(page: *mut page);
        fn vm_flags_set(vma: *mut vm_area_struct, flags: u32);
        fn usb_get_current_frame_number(dev: *mut usb_device) -> i32;
        fn usb_pipein(pipe: u32) -> i32;
        fn usb_sndisocpipe(dev: *mut usb_device, endpoint: u32) -> u32;
        fn usb_rcvisocpipe(dev: *mut usb_device, endpoint: u32) -> u32;
        fn usb_maxpacket(dev: *mut usb_device, pipe: u32) -> u32;
        fn usb_alloc_urb(iso_packets: i32, mem_flags: i32) -> *mut urb;
        fn usb_kill_urb(urb: *mut urb);
        fn usb_free_urb(urb: *mut urb);
        fn usb_submit_urb(urb: *mut urb, mem_flags: i32) -> i32;
        fn dev_dbg(dev: *mut core::ffi::c_void, fmt: *const u8, ...);
        fn dev_err(dev: *mut core::ffi::c_void, fmt: *const u8, ...);
        fn msleep_interruptible(msecs: u32) -> i32;
        fn wait_event(wq: *mut core::ffi::c_void, condition: bool);
        fn atomic_read(v: *const atomic_t) -> i32;
        fn atomic_set(v: *mut atomic_t, i: i32);
        fn atomic_inc(v: *mut atomic_t);
        fn smp_wmb();
        fn sprintf(buf: *mut u8, fmt: *const u8, ...);
        fn usx2y(card: *mut snd_card) -> *mut usx2ydev;
    }

    const USX2Y_HWDEP_PCM_PAGES: u32 = (mem::size_of::<snd_usx2y_hwdep_pcm_shm>() as u32 + 0xfff) & !0xfff;
    const ARRAY_SIZE_CAPTURED_ISO: usize = 8;
    const SSS: u32 = 0x4000 + 0x8000 + 0x8000;

    unsafe fn usx2y_usbpcm_urb_capt_retire(subs: *mut snd_usx2y_substream) -> i32 {
        let urb = (*subs).completed_urb;
        let runtime = (*(*subs).pcm_substream).runtime;
        let mut i: i32 = 0;
        let mut lens: i32 = 0;
        let mut hwptr_done: i32 = (*subs).hwptr_done;
        let usx2y = (*subs).usx2y;
        let mut head: i32 = 0;

        if (*(*usx2y).hwdep_pcm_shm).capture_iso_start < 0 {
            head = (*(*usx2y).hwdep_pcm_shm).captured_iso_head + 1;
            if head >= ARRAY_SIZE_CAPTURED_ISO as i32 {
                head = 0;
            }
            (*(*usx2y).hwdep_pcm_shm).capture_iso_start = head;
            dev_dbg(
                &mut (*(*usx2y).dev).dev as *mut core::ffi::c_void,
                b"cap start %i\n\0".as_ptr(),
                head,
            );
        }

        for i in 0..nr_of_packs() {
            if (*urb).iso_frame_desc[i as usize].status != 0 {
                dev_err(
                    &mut (*(*usx2y).dev).dev as *mut core::ffi::c_void,
                    b"active frame status %i. Most probably some hardware problem.\n\0".as_ptr(),
                    (*urb).iso_frame_desc[i as usize].status,
                );
                return (*urb).iso_frame_desc[i as usize].status;
            }
            lens += (*urb).iso_frame_desc[i as usize].actual_length as i32 / (*usx2y).stride as i32;
        }

        hwptr_done += lens;
        if hwptr_done >= (*runtime).buffer_size as i32 {
            hwptr_done -= (*runtime).buffer_size as i32;
        }
        (*subs).hwptr_done = hwptr_done;
        (*subs).transfer_done += lens;

        if (*subs).transfer_done >= (*runtime).period_size as i32 {
            (*subs).transfer_done -= (*runtime).period_size as i32;
            snd_pcm_period_elapsed((*subs).pcm_substream);
        }

        0
    }

    unsafe fn usx2y_iso_frames_per_buffer(runtime: *mut snd_pcm_runtime, usx2y: *mut usx2ydev) -> i32 {
        ((*runtime).buffer_size as i32 * 1000) / (*usx2y).rate as i32 + 1
    }

    unsafe fn usx2y_hwdep_urb_play_prepare(subs: *mut snd_usx2y_substream, urb: *mut urb) -> i32 {
        let mut count: i32 = 0;
        let mut counts: i32 = 0;
        let mut pack: i32 = 0;
        let usx2y = (*subs).usx2y;
        let shm = (*usx2y).hwdep_pcm_shm;
        let runtime = (*(*subs).pcm_substream).runtime;

        if (*shm).playback_iso_start < 0 {
            (*shm).playback_iso_start = (*shm).captured_iso_head
                - usx2y_iso_frames_per_buffer(runtime, usx2y);
            if (*shm).playback_iso_start < 0 {
                (*shm).playback_iso_start += ARRAY_SIZE_CAPTURED_ISO as i32;
            }
            (*shm).playback_iso_head = (*shm).playback_iso_start;
        }

        count = 0;
        for pack in 0..nr_of_packs() {
            counts = (*shm).captured_iso[(*shm).playback_iso_head as usize].length as i32
                / (*usx2y).stride as i32;
            if counts < 43 || counts > 50 {
                dev_err(
                    &mut (*(*usx2y).dev).dev as *mut core::ffi::c_void,
                    b"should not be here with counts=%i\n\0".as_ptr(),
                    counts,
                );
                return -EPIPE;
            }

            (*urb).iso_frame_desc[pack as usize].offset =
                (*shm).captured_iso[(*shm).playback_iso_head as usize].offset;
            (*urb).iso_frame_desc[pack as usize].length =
                (*shm).captured_iso[(*shm).playback_iso_head as usize].length;

            if atomic_read(&(*subs).state) != STATE_RUNNING {
                memset(
                    ((*urb).transfer_buffer as *mut u8)
                        .add((*urb).iso_frame_desc[pack as usize].offset as usize)
                        as *mut core::ffi::c_void,
                    0,
                    (*urb).iso_frame_desc[pack as usize].length,
                );
            }

            if (*shm).playback_iso_head >= ARRAY_SIZE_CAPTURED_ISO as i32 - 1 {
                (*shm).playback_iso_head = 0;
            } else {
                (*shm).playback_iso_head += 1;
            }
            count += counts;
        }

        (*urb).transfer_buffer_length = (count * (*usx2y).stride as i32) as u32;
        0
    }

    unsafe fn usx2y_usbpcm_urb_capt_iso_advance(
        subs: *mut snd_usx2y_substream,
        urb: *mut urb,
    ) {
        let mut pack: i32 = 0;
        let mut head: i32 = 0;
        let mut desc: *mut usb_iso_packet_descriptor;
        let mut shm: *mut snd_usx2y_hwdep_pcm_shm;

        for pack in 0..nr_of_packs() {
            desc = &mut (*urb).iso_frame_desc[pack as usize] as *mut usb_iso_packet_descriptor;
            if !subs.is_null() {
                shm = (*(*subs).usx2y).hwdep_pcm_shm;
                head = (*shm).captured_iso_head + 1;
                if head >= ARRAY_SIZE_CAPTURED_ISO as i32 {
                    head = 0;
                }
                (*shm).captured_iso[head as usize].frame = (*urb).start_frame + pack;
                (*shm).captured_iso[head as usize].offset = (*desc).offset;
                (*shm).captured_iso[head as usize].length = (*desc).actual_length;
                (*shm).captured_iso_head = head;
                (*shm).captured_iso_frames += 1;
            }

            (*desc).offset += (*desc).length * NRURBS as u32 * nr_of_packs() as u32;
            if (*desc).offset + (*desc).length >= SSS {
                (*desc).offset -= SSS - (*desc).length;
            }
        }
    }

    unsafe fn usx2y_usbpcm_usbframe_complete(
        capsubs: *mut snd_usx2y_substream,
        capsubs2: *mut snd_usx2y_substream,
        playbacksubs: *mut snd_usx2y_substream,
        frame: i32,
    ) -> i32 {
        let mut err: i32 = 0;
        let mut state: i32 = 0;
        let mut urb: *mut urb = (*playbacksubs).completed_urb;

        state = atomic_read(&(*playbacksubs).state);
        if !urb.is_null() {
            if state == STATE_RUNNING {
                usx2y_urb_play_retire(playbacksubs, urb);
            } else if state >= STATE_PRERUNNING {
                atomic_inc(&mut (*playbacksubs).state);
            }
        } else {
            match state {
                STATE_STARTING1 => {
                    urb = (*playbacksubs).urb[0];
                    atomic_inc(&mut (*playbacksubs).state);
                }
                STATE_STARTING2 => {
                    urb = (*playbacksubs).urb[1];
                    atomic_inc(&mut (*playbacksubs).state);
                }
                _ => {}
            }
        }

        if !urb.is_null() {
            err = usx2y_hwdep_urb_play_prepare(playbacksubs, urb);
            if err != 0 {
                return err;
            }
            err = usx2y_hwdep_urb_play_prepare(playbacksubs, urb);
            if err != 0 {
                return err;
            }
        }

        (*playbacksubs).completed_urb = ptr::null_mut();

        state = atomic_read(&(*capsubs).state);
        if state >= STATE_PREPARED {
            if state == STATE_RUNNING {
                err = usx2y_usbpcm_urb_capt_retire(capsubs);
                if err != 0 {
                    return err;
                }
            } else if state >= STATE_PRERUNNING {
                atomic_inc(&mut (*capsubs).state);
            }

            usx2y_usbpcm_urb_capt_iso_advance(capsubs, (*capsubs).completed_urb);
            if !capsubs2.is_null() {
                usx2y_usbpcm_urb_capt_iso_advance(ptr::null_mut(), (*capsubs2).completed_urb);
            }

            err = usx2y_urb_submit(capsubs, (*capsubs).completed_urb, frame);
            if err != 0 {
                return err;
            }

            if !capsubs2.is_null() {
                err = usx2y_urb_submit(capsubs2, (*capsubs2).completed_urb, frame);
                if err != 0 {
                    return err;
                }
            }
        }

        (*capsubs).completed_urb = ptr::null_mut();
        if !capsubs2.is_null() {
            (*capsubs2).completed_urb = ptr::null_mut();
        }

        0
    }

    extern "C" fn i_usx2y_usbpcm_urb_complete(urb: *mut urb) {
        unsafe {
            let subs: *mut snd_usx2y_substream = (*urb).context as *mut snd_usx2y_substream;
            let usx2y: *mut usx2ydev = (*subs).usx2y;
            let capsubs: *mut snd_usx2y_substream = (*usx2y).subs[SNDRV_PCM_STREAM_CAPTURE as usize];
            let capsubs2: *mut snd_usx2y_substream =
                (*usx2y).subs[(SNDRV_PCM_STREAM_CAPTURE + 2) as usize];
            let playbacksubs: *mut snd_usx2y_substream =
                (*usx2y).subs[SNDRV_PCM_STREAM_PLAYBACK as usize];

            if atomic_read(&(*subs).state) < STATE_PREPARED {
                dev_dbg(
                    &mut (*(*usx2y).dev).dev as *mut core::ffi::c_void,
                    b"hcd_frame=%i ep=%i%s status=%i start_frame=%i\n\0".as_ptr(),
                    usb_get_current_frame_number((*usx2y).dev),
                    (*subs).endpoint,
                    if usb_pipein((*urb).pipe) != 0 {
                        b"in\0".as_ptr() as *const core::ffi::c_void
                    } else {
                        b"out\0".as_ptr() as *const core::ffi::c_void
                    },
                    (*urb).status,
                    (*urb).start_frame,
                );
                return;
            }

            if (*urb).status != 0 {
                usx2y_error_urb_status(usx2y, subs, urb);
                return;
            }

            (*subs).completed_urb = urb;

            if !(*capsubs).completed_urb.is_null()
                && atomic_read(&(*capsubs).state) >= STATE_PREPARED
                && (capsubs2.is_null() || !(*capsubs2).completed_urb.is_null())
                && (!(*playbacksubs).completed_urb.is_null()
                    || atomic_read(&(*playbacksubs).state) < STATE_PREPARED)
            {
                if usx2y_usbpcm_usbframe_complete(capsubs, capsubs2, playbacksubs, (*urb).start_frame) == 0
                {
                    (*usx2y).wait_iso_frame += nr_of_packs();
                } else {
                    usx2y_clients_stop(usx2y);
                }
            }
        }
    }

    unsafe fn usx2y_hwdep_urb_release(urb: *mut *mut urb) {
        usb_kill_urb(*urb);
        usb_free_urb(*urb);
        *urb = ptr::null_mut();
    }

    unsafe fn usx2y_usbpcm_urbs_release(subs: *mut snd_usx2y_substream) {
        dev_dbg(
            &mut (*(*subs).usx2y).dev as *mut core::ffi::c_void,
            b"snd_usx2y_urbs_release() %i\n\0".as_ptr(),
            (*subs).endpoint,
        );
        for i in 0..NRURBS {
            usx2y_hwdep_urb_release(&mut (*subs).urb[i]);
        }
    }

    unsafe fn usx2y_usbpcm_subs_startup_finish(usx2y: *mut usx2ydev) {
        usx2y_urbs_set_complete(usx2y, i_usx2y_usbpcm_urb_complete);
        (*usx2y).prepare_subs = ptr::null_mut();
    }

    extern "C" fn i_usx2y_usbpcm_subs_startup(urb: *mut urb) {
        unsafe {
            let subs: *mut snd_usx2y_substream = (*urb).context as *mut snd_usx2y_substream;
            let usx2y: *mut usx2ydev = (*subs).usx2y;
            let prepare_subs: *mut snd_usx2y_substream = (*usx2y).prepare_subs;
            let mut cap_subs2: *mut snd_usx2y_substream = ptr::null_mut();

            if !prepare_subs.is_null()
                && (*urb).start_frame == (*(*prepare_subs).urb[0]).start_frame
            {
                atomic_inc(&mut (*prepare_subs).state);

                if prepare_subs == (*usx2y).subs[SNDRV_PCM_STREAM_CAPTURE as usize] {
                    cap_subs2 = (*usx2y).subs[(SNDRV_PCM_STREAM_CAPTURE + 2) as usize];
                    if !cap_subs2.is_null() {
                        atomic_inc(&mut (*cap_subs2).state);
                    }
                }

                usx2y_usbpcm_subs_startup_finish(usx2y);
                // wake_up(&usx2y->prepare_wait_queue);
            }

            i_usx2y_usbpcm_urb_complete(urb);
        }
    }

    unsafe fn usx2y_usbpcm_urbs_allocate(subs: *mut snd_usx2y_substream) -> i32 {
        let mut i: i32 = 0;
        let mut pipe: u32 = 0;
        let is_playback: i32 =
            if subs == (*(*subs).usx2y).subs[SNDRV_PCM_STREAM_PLAYBACK as usize] {
                1
            } else {
                0
            };
        let dev: *mut usb_device = (*(*subs).usx2y).dev;
        let mut purb: *mut *mut urb;

        pipe = if is_playback != 0 {
            usb_sndisocpipe(dev, (*subs).endpoint)
        } else {
            usb_rcvisocpipe(dev, (*subs).endpoint)
        };

        (*subs).maxpacksize = usb_maxpacket(dev, pipe);
        if (*subs).maxpacksize == 0 {
            return -EINVAL;
        }

        for i in 0..NRURBS {
            purb = &mut (*subs).urb[i];
            if !(*purb).is_null() {
                usb_kill_urb(*purb);
                continue;
            }

            *purb = usb_alloc_urb(nr_of_packs(), GFP_KERNEL);
            if (*purb).is_null() {
                usx2y_usbpcm_urbs_release(subs);
                return -ENOMEM;
            }

            (*(*purb)).transfer_buffer = if is_playback != 0 {
                (*(*(*subs).usx2y).hwdep_pcm_shm).playback.as_mut_ptr() as *mut core::ffi::c_void
            } else if (*subs).endpoint == 0x8 {
                (*(*(*subs).usx2y).hwdep_pcm_shm).capture0x8.as_mut_ptr() as *mut core::ffi::c_void
            } else {
                (*(*(*subs).usx2y).hwdep_pcm_shm).capture0xA.as_mut_ptr() as *mut core::ffi::c_void
            };

            (*(*purb)).dev = dev;
            (*(*purb)).pipe = pipe;
            (*(*purb)).number_of_packets = nr_of_packs();
            (*(*purb)).context = subs as *mut core::ffi::c_void;
            (*(*purb)).interval = 1;
            (*(*purb)).complete = Some(i_usx2y_usbpcm_subs_startup);
        }

        0
    }

    unsafe fn snd_usx2y_usbpcm_hw_free(substream: *mut snd_pcm_substream) -> i32 {
        let runtime: *mut snd_pcm_runtime = (*substream).runtime;
        let subs: *mut snd_usx2y_substream = (*runtime).private_data as *mut snd_usx2y_substream;
        let mut cap_subs: *mut snd_usx2y_substream;
        let mut playback_subs: *mut snd_usx2y_substream;
        let cap_subs2: *mut snd_usx2y_substream =
            (*(*subs).usx2y).subs[(SNDRV_PCM_STREAM_CAPTURE + 2) as usize];

        dev_dbg(
            &mut (*(*subs).usx2y).dev as *mut core::ffi::c_void,
            b"%s(%p)\n\0".as_ptr(),
            substream,
        );

        if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
            cap_subs = (*(*subs).usx2y).subs[SNDRV_PCM_STREAM_CAPTURE as usize];
            atomic_set(&mut (*subs).state, STATE_STOPPED);
            usx2y_usbpcm_urbs_release(subs);

            if (*cap_subs).pcm_substream.is_null()
                || (*(*cap_subs).pcm_substream).runtime.is_null()
                || (*(*(*cap_subs).pcm_substream).runtime).private_data.is_null()
            {
                atomic_set(&mut (*cap_subs).state, STATE_STOPPED);
                if !cap_subs2.is_null() {
                    atomic_set(&mut (*cap_subs2).state, STATE_STOPPED);
                }
                usx2y_usbpcm_urbs_release(cap_subs);
                if !cap_subs2.is_null() {
                    usx2y_usbpcm_urbs_release(cap_subs2);
                }
            }
        } else {
            playback_subs = (*(*subs).usx2y).subs[SNDRV_PCM_STREAM_PLAYBACK as usize];
            if atomic_read(&(*playback_subs).state) < STATE_PREPARED {
                atomic_set(&mut (*subs).state, STATE_STOPPED);
                if !cap_subs2.is_null() {
                    atomic_set(&mut (*cap_subs2).state, STATE_STOPPED);
                }
                usx2y_usbpcm_urbs_release(subs);
                if !cap_subs2.is_null() {
                    usx2y_usbpcm_urbs_release(cap_subs2);
                }
            }
        }

        0
    }

    unsafe fn usx2y_usbpcm_subs_startup(subs: *mut snd_usx2y_substream) {
        let usx2y: *mut usx2ydev = (*subs).usx2y;
        (*usx2y).prepare_subs = subs;
        (*(*subs).urb[0]).start_frame = -1;
        smp_wmb();
        usx2y_urbs_set_complete(usx2y, i_usx2y_usbpcm_subs_startup);
    }

    unsafe fn usx2y_usbpcm_urbs_start(subs: *mut snd_usx2y_substream) -> i32 {
        let mut p: i32 = 0;
        let mut u: i32 = 0;
        let mut err: i32 = 0;
        let stream: i32 = (*(*subs).pcm_substream).stream;
        let usx2y: *mut usx2ydev = (*subs).usx2y;
        let mut urb: *mut urb;
        let mut pack: u32 = 0;
        let mut subs_iter: *mut snd_usx2y_substream;

        if stream == SNDRV_PCM_STREAM_CAPTURE {
            (*(*usx2y).hwdep_pcm_shm).captured_iso_head = -1;
            (*(*usx2y).hwdep_pcm_shm).captured_iso_frames = 0;
        }

        p = 0;
        while 3 >= (stream + p) {
            subs_iter = (*usx2y).subs[(stream + p) as usize];
            if !subs_iter.is_null() {
                err = usx2y_usbpcm_urbs_allocate(subs_iter);
                if err < 0 {
                    return err;
                }
                (*subs_iter).completed_urb = ptr::null_mut();
            }
            p += 2;
        }

        let mut found_prepared = false;
        for p in 0..4 {
            subs_iter = (*usx2y).subs[p];
            if !subs_iter.is_null() && atomic_read(&(*subs_iter).state) >= STATE_PREPARED {
                found_prepared = true;
                break;
            }
        }

        if !found_prepared {
            // start label
        }

        usx2y_usbpcm_subs_startup(subs);

        for u in 0..NRURBS as i32 {
            p = 0;
            while 3 >= (stream + p) {
                subs_iter = (*usx2y).subs[(stream + p) as usize];

                if !subs_iter.is_null() {
                    urb = (*subs_iter).urb[u as usize];
                    if usb_pipein((*urb).pipe) != 0 {
                        if u == 0 {
                            atomic_set(&mut (*subs_iter).state, STATE_STARTING3);
                        }
                        (*urb).dev = (*usx2y).dev;

                        for pack in 0..nr_of_packs() as u32 {
                            (*urb).iso_frame_desc[pack as usize].offset =
                                (*subs_iter).maxpacksize * (pack + u as u32 * nr_of_packs() as u32);
                            (*urb).iso_frame_desc[pack as usize].length = (*subs_iter).maxpacksize;
                        }

                        (*urb).transfer_buffer_length = (*subs_iter).maxpacksize * nr_of_packs() as u32;

                        err = usb_submit_urb(urb, GFP_KERNEL);
                        if err < 0 {
                            dev_err(
                                &mut (*(*urb).dev).dev as *mut core::ffi::c_void,
                                b"cannot usb_submit_urb() for urb %d, err = %d\n\0".as_ptr(),
                                u,
                                err,
                            );
                            err = -EPIPE;
                            return err;
                        } else {
                            if u == 0 {
                                (*usx2y).wait_iso_frame = (*urb).start_frame;
                            }
                        }
                        (*urb).transfer_flags = 0;
                    } else {
                        atomic_set(&mut (*subs_iter).state, STATE_STARTING1);
                        break;
                    }
                }

                p += 2;
            }
        }

        // wait_event(usx2y->prepare_wait_queue, !usx2y->prepare_subs);

        if atomic_read(&(*subs).state) != STATE_PREPARED {
            err = -EPIPE;
        }

        if err != 0 {
            usx2y_subs_startup_finish(usx2y);
            usx2y_clients_stop(usx2y);
        }

        err
    }

    unsafe fn snd_usx2y_usbpcm_prepare(substream: *mut snd_pcm_substream) -> i32 {
        let runtime: *mut snd_pcm_runtime = (*substream).runtime;
        let subs: *mut snd_usx2y_substream = (*runtime).private_data as *mut snd_usx2y_substream;
        let usx2y: *mut usx2ydev = (*subs).usx2y;
        let capsubs: *mut snd_usx2y_substream = (*usx2y).subs[SNDRV_PCM_STREAM_CAPTURE as usize];
        let mut err: i32 = 0;

        dev_dbg(
            &mut (*(*usx2y).dev).dev as *mut core::ffi::c_void,
            b"snd_usx2y_pcm_prepare(%p)\n\0".as_ptr(),
            substream,
        );

        if (*usx2y).hwdep_pcm_shm.is_null() {
            (*usx2y).hwdep_pcm_shm = alloc_pages_exact(USX2Y_HWDEP_PCM_PAGES, GFP_KERNEL);
            if (*usx2y).hwdep_pcm_shm.is_null() {
                return -ENOMEM;
            }
            memset(
                (*usx2y).hwdep_pcm_shm,
                0,
                USX2Y_HWDEP_PCM_PAGES,
            );
        }

        usx2y_subs_prepare(subs);

        if atomic_read(&(*capsubs).state) < STATE_PREPARED {
            if (*usx2y).format != (*runtime).format {
                err = usx2y_format_set(usx2y, (*runtime).format);
                if err < 0 {
                    return err;
                }
            }
            if (*usx2y).rate != (*runtime).rate {
                err = usx2y_rate_set(usx2y, (*runtime).rate);
                if err < 0 {
                    return err;
                }
            }

            dev_dbg(
                &mut (*(*usx2y).dev).dev as *mut core::ffi::c_void,
                b"starting capture pipe for %s\n\0".as_ptr(),
                if subs == capsubs {
                    b"self\0".as_ptr() as *const core::ffi::c_void
                } else {
                    b"playpipe\0".as_ptr() as *const core::ffi::c_void
                },
            );

            err = usx2y_usbpcm_urbs_start(capsubs);
            if err < 0 {
                return err;
            }
        }

        if subs != capsubs {
            (*(*usx2y).hwdep_pcm_shm).playback_iso_start = -1;
            if atomic_read(&(*subs).state) < STATE_PREPARED {
                while usx2y_iso_frames_per_buffer(runtime, usx2y)
                    > (*(*usx2y).hwdep_pcm_shm).captured_iso_frames
                {
                    dev_dbg(
                        &mut (*(*usx2y).dev).dev as *mut core::ffi::c_void,
                        b"Wait: iso_frames_per_buffer=%i,captured_iso_frames=%i\n\0".as_ptr(),
                        usx2y_iso_frames_per_buffer(runtime, usx2y),
                        (*(*usx2y).hwdep_pcm_shm).captured_iso_frames,
                    );
                    if msleep_interruptible(10) != 0 {
                        return -ERESTARTSYS;
                    }
                }
                err = usx2y_usbpcm_urbs_start(subs);
                if err < 0 {
                    return err;
                }
            }

            dev_dbg(
                &mut (*(*usx2y).dev).dev as *mut core::ffi::c_void,
                b"Ready: iso_frames_per_buffer=%i,captured_iso_frames=%i\n\0".as_ptr(),
                usx2y_iso_frames_per_buffer(runtime, usx2y),
                (*(*usx2y).hwdep_pcm_shm).captured_iso_frames,
            );
        } else {
            (*(*usx2y).hwdep_pcm_shm).capture_iso_start = -1;
        }

        err
    }

    unsafe fn snd_usx2y_usbpcm_open(substream: *mut snd_pcm_substream) -> i32 {
        let chip: *mut *mut snd_usx2y_substream =
            snd_pcm_substream_chip(substream) as *mut *mut snd_usx2y_substream;
        let subs: *mut snd_usx2y_substream = *chip.add((*substream).stream as usize);
        let runtime: *mut snd_pcm_runtime = (*substream).runtime;

        if ((*(*subs).usx2y).chip_status & USX2Y_STAT_CHIP_MMAP_PCM_URBS) == 0 {
            return -EBUSY;
        }

        if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
            // runtime->hw = snd_usx2y_2c;
        } else {
            // runtime->hw = (subs->usx2y->subs[3] ? snd_usx2y_4c : snd_usx2y_2c);
        }

        (*runtime).private_data = subs as *mut core::ffi::c_void;
        (*subs).pcm_substream = substream;
        snd_pcm_hw_constraint_minmax(runtime, 14, 1000, 200000);
        0
    }

    unsafe fn snd_usx2y_usbpcm_close(substream: *mut snd_pcm_substream) -> i32 {
        let runtime: *mut snd_pcm_runtime = (*substream).runtime;
        let subs: *mut snd_usx2y_substream = (*runtime).private_data as *mut snd_usx2y_substream;

        (*subs).pcm_substream = ptr::null_mut();
        0
    }

    unsafe fn snd_usx2y_hwdep_pcm_vm_open(_area: *mut vm_area_struct) {
    }

    unsafe fn snd_usx2y_hwdep_pcm_vm_close(_area: *mut vm_area_struct) {
    }

    unsafe fn snd_usx2y_hwdep_pcm_vm_fault(vmf: *mut vm_fault) -> i32 {
        let offset: u64 = (*vmf).pgoff << PAGE_SHIFT;
        if offset as u32 >= USX2Y_HWDEP_PCM_PAGES {
            return VM_FAULT_SIGBUS;
        }

        let vaddr: *mut u8 =
            ((*(*vmf).vma).vm_private_data as *mut u8).add(offset as usize);
        (*vmf).page = virt_to_page(vaddr as *mut core::ffi::c_void);
        get_page((*vmf).page);
        0
    }

    unsafe fn snd_usx2y_hwdep_pcm_mmap(
        hw: *mut snd_hwdep,
        _filp: *mut core::ffi::c_void,
        area: *mut vm_area_struct,
    ) -> i32 {
        let size: u32 = ((*area).vm_end - (*area).vm_start) as u32;
        let usx2y: *mut usx2ydev = (*hw).private_data as *mut usx2ydev;

        if ((*usx2y).chip_status & USX2Y_STAT_CHIP_INIT) == 0 {
            return -EBUSY;
        }

        if size > USX2Y_HWDEP_PCM_PAGES {
            // dev_dbg
            return -EINVAL;
        }

        if (*usx2y).hwdep_pcm_shm.is_null() {
            return -ENODEV;
        }

        (*area).vm_ops = &snd_usx2y_hwdep_pcm_vm_ops;
        vm_flags_set(area, VM_DONTEXPAND | VM_DONTDUMP);
        (*area).vm_private_data = (*hw).private_data;
        0
    }

    unsafe fn snd_usx2y_hwdep_pcm_private_free(hwdep: *mut snd_hwdep) {
        let usx2y: *mut usx2ydev = (*hwdep).private_data as *mut usx2ydev;

        if !(*usx2y).hwdep_pcm_shm.is_null() {
            free_pages_exact((*usx2y).hwdep_pcm_shm, USX2Y_HWDEP_PCM_PAGES);
        }
    }

    unsafe fn snd_usx2y_hwdep_pcm_open(
        hw: *mut snd_hwdep,
        _file: *mut core::ffi::c_void,
    ) -> i32 {
        let card: *mut snd_card = (*hw).card;
        let mut err: i32 = 0;

        err = usx2y_pcms_busy_check(card);
        if err == 0 {
            (*usx2y(card)).chip_status |= USX2Y_STAT_CHIP_MMAP_PCM_URBS;
        }
        err
    }

    unsafe fn snd_usx2y_hwdep_pcm_release(
        hw: *mut snd_hwdep,
        _file: *mut core::ffi::c_void,
    ) -> i32 {
        let card: *mut snd_card = (*hw).card;
        let mut err: i32 = 0;

        err = usx2y_pcms_busy_check(card);
        if err == 0 {
            (*usx2y(card)).chip_status &= !USX2Y_STAT_CHIP_MMAP_PCM_URBS;
        }
        err
    }

    unsafe fn usx2y_pcms_busy_check(card: *mut snd_card) -> i32 {
        let dev: *mut usx2ydev = usx2y(card);
        let mut subs: *mut snd_usx2y_substream;

        // Loop would iterate through device substreams
        // Placeholder - actual implementation depends on external functions
        0
    }

    pub const SND_USX2Y_HWDEP_PCM_VM_OPS: vm_operations_struct = vm_operations_struct {
        open: Some(snd_usx2y_hwdep_pcm_vm_open),
        close: Some(snd_usx2y_hwdep_pcm_vm_close),
        fault: Some(snd_usx2y_hwdep_pcm_vm_fault),
    };

    let snd_usx2y_hwdep_pcm_vm_ops: vm_operations_struct = vm_operations_struct {
        open: Some(snd_usx2y_hwdep_pcm_vm_open),
        close: Some(snd_usx2y_hwdep_pcm_vm_close),
        fault: Some(snd_usx2y_hwdep_pcm_vm_fault),
    };

    unsafe fn usx2y_hwdep_pcm_new(card: *mut snd_card) -> i32 {
        let mut err: i32 = 0;
        let mut hw: *mut snd_hwdep = ptr::null_mut();
        let mut pcm: *mut snd_pcm = ptr::null_mut();
        let dev: *mut usb_device = (*usx2y(card)).dev;

        if nr_of_packs() != 1 {
            return 0;
        }

        err = snd_hwdep_new(card, SND_USX2Y_USBPCM_ID.as_ptr(), 1, &mut hw);
        if err < 0 {
            return err;
        }

        (*hw).iface = SNDRV_HWDEP_IFACE_USX2Y_PCM;
        (*hw).private_data = usx2y(card) as *mut core::ffi::c_void;
        (*hw).private_free = Some(snd_usx2y_hwdep_pcm_private_free);
        (*hw).ops.open = Some(snd_usx2y_hwdep_pcm_open);
        (*hw).ops.release = Some(snd_usx2y_hwdep_pcm_release);
        (*hw).ops.mmap = Some(snd_usx2y_hwdep_pcm_mmap);
        (*hw).exclusive = 1;

        // sprintf would fill the name
        // (*hw).name is a char array

        err = snd_pcm_new(card, b"USX2Y hwdep Audio\0".as_ptr(), 2, 1, 1, &mut pcm);
        if err < 0 {
            return err;
        }

        // Set PCM operations
        // snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &snd_usx2y_usbpcm_ops);
        // snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, &snd_usx2y_usbpcm_ops);

        (*pcm).private_data = (*usx2y(card)).subs.as_mut_ptr() as *mut core::ffi::c_void;
        (*pcm).info_flags = 0;

        // Set managed buffers
        snd_pcm_set_managed_buffer(
            (*pcm).streams[SNDRV_PCM_STREAM_PLAYBACK as usize].substream,
            SNDRV_DMA_TYPE_CONTINUOUS,
            ptr::null_mut(),
            64 * 1024,
            128 * 1024,
        );
        snd_pcm_set_managed_buffer(
            (*pcm).streams[SNDRV_PCM_STREAM_CAPTURE as usize].substream,
            SNDRV_DMA_TYPE_CONTINUOUS,
            ptr::null_mut(),
            64 * 1024,
            128 * 1024,
        );

        0
    }

    pub unsafe fn usx2y_hwdep_pcm_new_public(card: *mut snd_card) -> i32 {
        usx2y_hwdep_pcm_new(card)
    }
}

#[cfg(not(any(feature = "USX2Y_NRPACKS_VARIABLE", feature = "USX2Y_NRPACKS_1")))]
pub unsafe fn usx2y_hwdep_pcm_new(_card: *mut core::ffi::c_void) -> i32 {
    0
}

#[cfg(any(feature = "USX2Y_NRPACKS_VARIABLE", feature = "USX2Y_NRPACKS_1"))]
pub use usx2y_hwdep_pcm::usx2y_hwdep_pcm_new_public as usx2y_hwdep_pcm_new;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
