// SPDX-License-Identifier: GPL-2.0+
/*
 * virtio-snd: Virtio sound device
 * Copyright (C) 2021 OpenSynergy GmbH
 */
// Linux kernel headers would be included here:
// #include <linux/moduleparam.h>
// #include <linux/virtio_config.h>
// #include "virtio_card.h"

static mut PCM_BUFFER_MS: u32 = 160;
// module_param(pcm_buffer_ms, uint, 0644);
// MODULE_PARM_DESC(pcm_buffer_ms, "PCM substream buffer time in milliseconds");

static mut PCM_PERIODS_MIN: u32 = 2;
// module_param(pcm_periods_min, uint, 0644);
// MODULE_PARM_DESC(pcm_periods_min, "Minimum number of PCM periods");

static mut PCM_PERIODS_MAX: u32 = 16;
// module_param(pcm_periods_max, uint, 0644);
// MODULE_PARM_DESC(pcm_periods_max, "Maximum number of PCM periods");

static mut PCM_PERIOD_MS_MIN: u32 = 10;
// module_param(pcm_period_ms_min, uint, 0644);
// MODULE_PARM_DESC(pcm_period_ms_min, "Minimum PCM period time in milliseconds");

static mut PCM_PERIOD_MS_MAX: u32 = 80;
// module_param(pcm_period_ms_max, uint, 0644);
// MODULE_PARM_DESC(pcm_period_ms_max, "Maximum PCM period time in milliseconds");

// Map for converting VirtIO format to ALSA format.
static G_V2A_FORMAT_MAP: &[snd_pcm_format_t] = &[
	// [VIRTIO_SND_PCM_FMT_IMA_ADPCM] = SNDRV_PCM_FORMAT_IMA_ADPCM,
	// [VIRTIO_SND_PCM_FMT_MU_LAW] = SNDRV_PCM_FORMAT_MU_LAW,
	// [VIRTIO_SND_PCM_FMT_A_LAW] = SNDRV_PCM_FORMAT_A_LAW,
	// [VIRTIO_SND_PCM_FMT_S8] = SNDRV_PCM_FORMAT_S8,
	// [VIRTIO_SND_PCM_FMT_U8] = SNDRV_PCM_FORMAT_U8,
	// [VIRTIO_SND_PCM_FMT_S16] = SNDRV_PCM_FORMAT_S16_LE,
	// [VIRTIO_SND_PCM_FMT_U16] = SNDRV_PCM_FORMAT_U16_LE,
	// [VIRTIO_SND_PCM_FMT_S18_3] = SNDRV_PCM_FORMAT_S18_3LE,
	// [VIRTIO_SND_PCM_FMT_U18_3] = SNDRV_PCM_FORMAT_U18_3LE,
	// [VIRTIO_SND_PCM_FMT_S20_3] = SNDRV_PCM_FORMAT_S20_3LE,
	// [VIRTIO_SND_PCM_FMT_U20_3] = SNDRV_PCM_FORMAT_U20_3LE,
	// [VIRTIO_SND_PCM_FMT_S24_3] = SNDRV_PCM_FORMAT_S24_3LE,
	// [VIRTIO_SND_PCM_FMT_U24_3] = SNDRV_PCM_FORMAT_U24_3LE,
	// [VIRTIO_SND_PCM_FMT_S20] = SNDRV_PCM_FORMAT_S20_LE,
	// [VIRTIO_SND_PCM_FMT_U20] = SNDRV_PCM_FORMAT_U20_LE,
	// [VIRTIO_SND_PCM_FMT_S24] = SNDRV_PCM_FORMAT_S24_LE,
	// [VIRTIO_SND_PCM_FMT_U24] = SNDRV_PCM_FORMAT_U24_LE,
	// [VIRTIO_SND_PCM_FMT_S32] = SNDRV_PCM_FORMAT_S32_LE,
	// [VIRTIO_SND_PCM_FMT_U32] = SNDRV_PCM_FORMAT_U32_LE,
	// [VIRTIO_SND_PCM_FMT_FLOAT] = SNDRV_PCM_FORMAT_FLOAT_LE,
	// [VIRTIO_SND_PCM_FMT_FLOAT64] = SNDRV_PCM_FORMAT_FLOAT64_LE,
	// [VIRTIO_SND_PCM_FMT_DSD_U8] = SNDRV_PCM_FORMAT_DSD_U8,
	// [VIRTIO_SND_PCM_FMT_DSD_U16] = SNDRV_PCM_FORMAT_DSD_U16_LE,
	// [VIRTIO_SND_PCM_FMT_DSD_U32] = SNDRV_PCM_FORMAT_DSD_U32_LE,
	// [VIRTIO_SND_PCM_FMT_IEC958_SUBFRAME] = SNDRV_PCM_FORMAT_IEC958_SUBFRAME_LE
];

// Map for converting VirtIO frame rate to ALSA frame rate.
#[repr(C)]
struct VirtsndV2aRate {
	alsa_bit: u32,
	rate: u32,
}

static G_V2A_RATE_MAP: &[VirtsndV2aRate] = &[
	// [VIRTIO_SND_PCM_RATE_5512] = { SNDRV_PCM_RATE_5512, 5512 },
	// [VIRTIO_SND_PCM_RATE_8000] = { SNDRV_PCM_RATE_8000, 8000 },
	// [VIRTIO_SND_PCM_RATE_11025] = { SNDRV_PCM_RATE_11025, 11025 },
	// [VIRTIO_SND_PCM_RATE_16000] = { SNDRV_PCM_RATE_16000, 16000 },
	// [VIRTIO_SND_PCM_RATE_22050] = { SNDRV_PCM_RATE_22050, 22050 },
	// [VIRTIO_SND_PCM_RATE_32000] = { SNDRV_PCM_RATE_32000, 32000 },
	// [VIRTIO_SND_PCM_RATE_44100] = { SNDRV_PCM_RATE_44100, 44100 },
	// [VIRTIO_SND_PCM_RATE_48000] = { SNDRV_PCM_RATE_48000, 48000 },
	// [VIRTIO_SND_PCM_RATE_64000] = { SNDRV_PCM_RATE_64000, 64000 },
	// [VIRTIO_SND_PCM_RATE_88200] = { SNDRV_PCM_RATE_88200, 88200 },
	// [VIRTIO_SND_PCM_RATE_96000] = { SNDRV_PCM_RATE_96000, 96000 },
	// [VIRTIO_SND_PCM_RATE_176400] = { SNDRV_PCM_RATE_176400, 176400 },
	// [VIRTIO_SND_PCM_RATE_192000] = { SNDRV_PCM_RATE_192000, 192000 },
	// [VIRTIO_SND_PCM_RATE_384000] = { SNDRV_PCM_RATE_384000, 384000 }
];

/// virtsnd_pcm_build_hw() - Parse substream config and build HW descriptor.
/// @vss: VirtIO substream.
/// @info: VirtIO substream information entry.
///
/// Context: Any context.
/// Return: 0 on success, -EINVAL if configuration is invalid.
unsafe fn virtsnd_pcm_build_hw(vss: *mut VirtiopcmSubstream, info: *mut VirtioSndPcmInfo) -> i32 {
	let vdev = (*(*vss).snd).vdev;
	let mut i: u32;
	let mut values: u64;
	let mut sample_max: usize = 0;
	let mut sample_min: usize = 0;

	(*vss).features = le32_to_cpu((*info).features);

	// TODO: set SNDRV_PCM_INFO_{BATCH,BLOCK_TRANSFER} if device supports
	// only message-based transport.
	(*vss).hw.info =
		SNDRV_PCM_INFO_MMAP |
		SNDRV_PCM_INFO_MMAP_VALID |
		SNDRV_PCM_INFO_BATCH |
		SNDRV_PCM_INFO_BLOCK_TRANSFER |
		SNDRV_PCM_INFO_INTERLEAVED |
		SNDRV_PCM_INFO_PAUSE |
		SNDRV_PCM_INFO_NO_REWINDS |
		SNDRV_PCM_INFO_SYNC_APPLPTR;

	if (*info).channels_min == 0 || (*info).channels_min > (*info).channels_max {
		dev_err(&(*vdev).dev,
			"SID %u: invalid channel range [%u %u]\n",
			(*vss).sid, (*info).channels_min, (*info).channels_max);
		return -EINVAL;
	}

	(*vss).hw.channels_min = (*info).channels_min;
	(*vss).hw.channels_max = (*info).channels_max;

	values = le64_to_cpu((*info).formats);

	(*vss).hw.formats = 0;

	i = 0;
	while i < G_V2A_FORMAT_MAP.len() as u32 {
		if (values & (1u64 << i)) != 0 {
			let alsa_fmt = G_V2A_FORMAT_MAP[i as usize];
			let bytes = snd_pcm_format_physical_width(alsa_fmt) / 8;

			if sample_min == 0 || sample_min > bytes as usize {
				sample_min = bytes as usize;
			}

			if sample_max < bytes as usize {
				sample_max = bytes as usize;
			}

			(*vss).hw.formats |= pcm_format_to_bits(alsa_fmt);
		}
		i += 1;
	}

	if (*vss).hw.formats == 0 {
		dev_err(&(*vdev).dev,
			"SID %u: no supported PCM sample formats found\n",
			(*vss).sid);
		return -EINVAL;
	}

	values = le64_to_cpu((*info).rates);

	(*vss).hw.rates = 0;

	i = 0;
	while i < G_V2A_RATE_MAP.len() as u32 {
		if (values & (1u64 << i)) != 0 {
			if (*vss).hw.rate_min == 0 ||
			    (*vss).hw.rate_min > G_V2A_RATE_MAP[i as usize].rate {
				(*vss).hw.rate_min = G_V2A_RATE_MAP[i as usize].rate;
			}

			if (*vss).hw.rate_max < G_V2A_RATE_MAP[i as usize].rate {
				(*vss).hw.rate_max = G_V2A_RATE_MAP[i as usize].rate;
			}

			(*vss).hw.rates |= G_V2A_RATE_MAP[i as usize].alsa_bit;
		}
		i += 1;
	}

	if (*vss).hw.rates == 0 {
		dev_err(&(*vdev).dev,
			"SID %u: no supported PCM frame rates found\n",
			(*vss).sid);
		return -EINVAL;
	}

	(*vss).hw.periods_min = unsafe { PCM_PERIODS_MIN };
	(*vss).hw.periods_max = unsafe { PCM_PERIODS_MAX };

	// We must ensure that there is enough space in the buffer to store
	// pcm_buffer_ms ms for the combination (Cmax, Smax, Rmax), where:
	//   Cmax = maximum supported number of channels,
	//   Smax = maximum supported sample size in bytes,
	//   Rmax = maximum supported frame rate.
	let pcm_buffer_ms = unsafe { PCM_BUFFER_MS };
	(*vss).hw.buffer_bytes_max =
		PAGE_ALIGN((sample_max as u32 * (*vss).hw.channels_max * pcm_buffer_ms *
			   ((*vss).hw.rate_max / MSEC_PER_SEC)) as usize);

	// We must ensure that the minimum period size is enough to store
	// pcm_period_ms_min ms for the combination (Cmin, Smin, Rmin), where:
	//   Cmin = minimum supported number of channels,
	//   Smin = minimum supported sample size in bytes,
	//   Rmin = minimum supported frame rate.
	let pcm_period_ms_min = unsafe { PCM_PERIOD_MS_MIN };
	(*vss).hw.period_bytes_min =
		(sample_min as u32 * (*vss).hw.channels_min * pcm_period_ms_min *
		((*vss).hw.rate_min / MSEC_PER_SEC)) as usize;

	// We must ensure that the maximum period size is enough to store
	// pcm_period_ms_max ms for the combination (Cmax, Smax, Rmax).
	let pcm_period_ms_max = unsafe { PCM_PERIOD_MS_MAX };
	(*vss).hw.period_bytes_max =
		(sample_max as u32 * (*vss).hw.channels_max * pcm_period_ms_max *
		((*vss).hw.rate_max / MSEC_PER_SEC)) as usize;

	0
}

/// virtsnd_pcm_find() - Find the PCM device for the specified node ID.
/// @snd: VirtIO sound device.
/// @nid: Function node ID.
///
/// Context: Any context.
/// Return: a pointer to the PCM device or ERR_PTR(-ENOENT).
unsafe fn virtsnd_pcm_find(snd: *mut VirtioSnd, nid: u32) -> *mut VirtiopcmLayout {
	// list_for_each_entry(vpcm, &snd->pcm_list, list)
	// 	if (vpcm->nid == nid)
	// 		return vpcm;
	// return ERR_PTR(-ENOENT);

	// TODO: iterate through kernel linked list in (*snd).pcm_list
	ERR_PTR(-ENOENT) as *mut VirtiopcmLayout
}

/// virtsnd_pcm_find_or_create() - Find or create the PCM device for the
///                                specified node ID.
/// @snd: VirtIO sound device.
/// @nid: Function node ID.
///
/// Context: Any context that permits to sleep.
/// Return: a pointer to the PCM device or ERR_PTR(-errno).
unsafe fn virtsnd_pcm_find_or_create(snd: *mut VirtioSnd, nid: u32) -> *mut VirtiopcmLayout {
	let vdev = (*snd).vdev;
	let mut vpcm: *mut VirtiopcmLayout;

	vpcm = virtsnd_pcm_find(snd, nid);
	if !IS_ERR(vpcm as *mut i32) {
		return vpcm;
	}

	vpcm = devm_kzalloc(&(*vdev).dev, std::mem::size_of::<VirtiopcmLayout>(), GFP_KERNEL) as *mut VirtiopcmLayout;
	if vpcm.is_null() {
		return ERR_PTR(-ENOMEM) as *mut VirtiopcmLayout;
	}

	(*vpcm).nid = nid;
	// list_add_tail(&vpcm->list, &snd->pcm_list);

	vpcm
}

/// virtsnd_pcm_validate() - Validate if the device can be started.
/// @vdev: VirtIO parent device.
///
/// Context: Any context.
/// Return: 0 on success, -EINVAL on failure.
unsafe fn virtsnd_pcm_validate(vdev: *mut VirtioDevice) -> i32 {
	let pcm_periods_min = PCM_PERIODS_MIN;
	let pcm_periods_max = PCM_PERIODS_MAX;
	let pcm_period_ms_min = PCM_PERIOD_MS_MIN;
	let pcm_period_ms_max = PCM_PERIOD_MS_MAX;
	let pcm_buffer_ms = PCM_BUFFER_MS;

	if pcm_periods_min < 2 || pcm_periods_min > pcm_periods_max {
		dev_err(&(*vdev).dev,
			"invalid range [%u %u] of the number of PCM periods\n",
			pcm_periods_min, pcm_periods_max);
		return -EINVAL;
	}

	if pcm_period_ms_min == 0 || pcm_period_ms_min > pcm_period_ms_max {
		dev_err(&(*vdev).dev,
			"invalid range [%u %u] of the size of the PCM period\n",
			pcm_period_ms_min, pcm_period_ms_max);
		return -EINVAL;
	}

	if pcm_buffer_ms < pcm_periods_min * pcm_period_ms_min {
		dev_err(&(*vdev).dev,
			"pcm_buffer_ms(=%u) value cannot be < %u ms\n",
			pcm_buffer_ms, pcm_periods_min * pcm_period_ms_min);
		return -EINVAL;
	}

	if pcm_period_ms_max > pcm_buffer_ms / 2 {
		dev_err(&(*vdev).dev,
			"pcm_period_ms_max(=%u) value cannot be > %u ms\n",
			pcm_period_ms_max, pcm_buffer_ms / 2);
		return -EINVAL;
	}

	0
}

/// virtsnd_pcm_period_elapsed() - Kernel work function to handle the elapsed
///                                period state.
/// @work: Elapsed period work.
///
/// The main purpose of this function is to call snd_pcm_period_elapsed() in
/// a process context, not in an interrupt context. This is necessary because PCM
/// devices operate in non-atomic mode.
///
/// Context: Process context.
unsafe fn virtsnd_pcm_period_elapsed(work: *mut WorkStruct) {
	// container_of(work, struct virtio_pcm_substream, elapsed_period)
	// Offset calculation: work points to elapsed_period field within VirtiopcmSubstream
	let vss = (work as *mut u8).offset(-(offset_of!(VirtiopcmSubstream, elapsed_period) as isize)) as *mut VirtiopcmSubstream;

	snd_pcm_period_elapsed((*vss).substream);
}

/// virtsnd_pcm_parse_cfg() - Parse the stream configuration.
/// @snd: VirtIO sound device.
///
/// This function is called during initial device initialization.
///
/// Context: Any context that permits to sleep.
/// Return: 0 on success, -errno on failure.
unsafe fn virtsnd_pcm_parse_cfg(snd: *mut VirtioSnd) -> i32 {
	let vdev = (*snd).vdev;
	let mut info: *mut VirtioSndPcmInfo;
	let mut i: u32;
	let mut rc: i32;

	virtio_cread_le(vdev, VirtioSndConfig, streams,
			&mut (*snd).nsubstreams);
	if (*snd).nsubstreams == 0 {
		return 0;
	}

	(*snd).substreams = devm_kcalloc(&(*vdev).dev, (*snd).nsubstreams,
				       std::mem::size_of::<VirtiopcmSubstream>(), GFP_KERNEL) as *mut VirtiopcmSubstream;
	if (*snd).substreams.is_null() {
		return -ENOMEM;
	}

	// Initialize critical substream fields early in case we hit an
	// error path and end up trying to clean up uninitialized structures
	// elsewhere.
	i = 0;
	while i < (*snd).nsubstreams {
		let vss = &mut *(*snd).substreams.add(i as usize);

		vss.snd = snd;
		vss.sid = i;
		// INIT_WORK(&vss->elapsed_period, virtsnd_pcm_period_elapsed);
		// Note: Work structure initialization is deferred to kernel setup
		init_waitqueue_head(&mut vss.msg_empty as *mut _ as *mut u8);
		spin_lock_init(&mut vss.lock);
		i += 1;
	}

	// kzalloc_objs(*info, (*snd).nsubstreams) - allocate array of VirtioSndPcmInfo
	info = kzalloc(std::mem::size_of::<VirtioSndPcmInfo>() * (*snd).nsubstreams as usize, GFP_KERNEL) as *mut VirtioSndPcmInfo;
	if info.is_null() {
		return -ENOMEM;
	}

	rc = virtsnd_ctl_query_info(snd, VIRTIO_SND_R_PCM_INFO, 0,
				    (*snd).nsubstreams, std::mem::size_of::<VirtioSndPcmInfo>(), info);
	if rc != 0 {
		kfree(info as *mut _);
		return rc;
	}

	i = 0;
	while i < (*snd).nsubstreams {
		let vss = &mut *(*snd).substreams.add(i as usize);
		let info_i = &*info.add(i as usize);
		let mut vpcm: *mut VirtiopcmLayout;

		rc = virtsnd_pcm_build_hw(vss, info_i as *mut _);
		if rc != 0 {
			kfree(info as *mut _);
			return rc;
		}

		vss.nid = le32_to_cpu(info_i.hdr.hda_fn_nid);

		vpcm = virtsnd_pcm_find_or_create(snd, vss.nid);
		if IS_ERR(vpcm as *mut i32) {
			rc = PTR_ERR(vpcm as *mut i32);
			kfree(info as *mut _);
			return rc;
		}

		match info_i.direction {
		VIRTIO_SND_D_OUTPUT => {
			vss.direction = SNDRV_PCM_STREAM_PLAYBACK;
		}
		VIRTIO_SND_D_INPUT => {
			vss.direction = SNDRV_PCM_STREAM_CAPTURE;
		}
		_ => {
			dev_err(&(*vdev).dev, "SID %u: unknown direction (%u)\n",
				vss.sid, info_i.direction);
			rc = -EINVAL;
			kfree(info as *mut _);
			return rc;
		}
		}

		(*vpcm).streams[vss.direction as usize].nsubstreams += 1;
		i += 1;
	}

	kfree(info as *mut _);
	0
}

/// virtsnd_pcm_build_devs() - Build ALSA PCM devices.
/// @snd: VirtIO sound device.
///
/// Context: Any context that permits to sleep.
/// Return: 0 on success, -errno on failure.
unsafe fn virtsnd_pcm_build_devs(snd: *mut VirtioSnd) -> i32 {
	let vdev = (*snd).vdev;
	let mut vpcm: *mut VirtiopcmLayout;
	let mut i: u32;
	let mut rc: i32;

	// list_for_each_entry(vpcm, &snd->pcm_list, list)
	// ...

	i = 0;
	while i < (*snd).nsubstreams {
		let vss = &*(*snd).substreams.add(i as usize);

		vpcm = virtsnd_pcm_find(snd, vss.nid);
		if IS_ERR(vpcm as *mut i32) {
			return PTR_ERR(vpcm as *mut i32);
		}

		let vs = &mut (*vpcm).streams[vss.direction as usize];
		vs.substreams[vs.nsubstreams as usize] = vss as *mut _;
		vs.nsubstreams += 1;
		i += 1;
	}

	// list_for_each_entry(vpcm, &snd->pcm_list, list)
	// ...

	0
}

/// virtsnd_pcm_event() - Handle the PCM device event notification.
/// @snd: VirtIO sound device.
/// @event: VirtIO sound event.
///
/// Context: Interrupt context.
unsafe fn virtsnd_pcm_event(snd: *mut VirtioSnd, event: *mut VirtioSndEvent) {
	let mut vss: *mut VirtiopcmSubstream;
	let sid = le32_to_cpu((*event).data);

	if sid >= (*snd).nsubstreams {
		return;
	}

	vss = (*snd).substreams.add(sid as usize) as *mut _;

	match le32_to_cpu((*event).hdr.code) {
	VIRTIO_SND_EVT_PCM_PERIOD_ELAPSED => {
		// TODO: deal with shmem elapsed period
	}
	VIRTIO_SND_EVT_PCM_XRUN => {
		// scoped_guard(spinlock, &vss->lock) {
		// 	if (vss->xfer_enabled)
		// 		vss->xfer_xrun = true;
		// }

		// Simplified equivalent without scoped_guard macro:
		let _guard = spinlock_guard(&(*vss).lock);
		if (*vss).xfer_enabled {
			(*vss).xfer_xrun = true;
		}
	}
	_ => {}
	}
}

// External type and function declarations from kernel headers
type snd_pcm_format_t = u32;
type WorkStruct = *mut u8;

#[repr(C)]
struct VirtioDevice {
	dev: *mut u8, // device structure
}

#[repr(C)]
struct VirtioSndConfig {
	jacks: u32,
	streams: u32,
	chmaps: u32,
}

#[repr(C)]
struct VirtioSndEvent {
	hdr: VirtioSndEventHdr,
	data: u32,
}

#[repr(C)]
struct VirtioSndEventHdr {
	code: u32,
}

#[repr(C)]
struct VirtioSndPcmInfo {
	hdr: VirtioSndPcmInfoHdr,
	features: u32,
	formats: u64,
	rates: u64,
	direction: u8,
	channels_min: u8,
	channels_max: u8,
	padding: u8,
}

#[repr(C)]
struct VirtioSndPcmInfoHdr {
	hda_fn_nid: u32,
}

#[repr(C)]
struct VirtiopcmHw {
	info: u32,
	formats: u64,
	rates: u32,
	rate_min: u32,
	rate_max: u32,
	channels_min: u32,
	channels_max: u32,
	periods_min: u32,
	periods_max: u32,
	period_bytes_min: usize,
	period_bytes_max: usize,
	buffer_bytes_max: usize,
}

#[repr(C)]
struct VirtiopcmSubstream {
	snd: *mut VirtioSnd,
	sid: u32,
	nid: u32,
	features: u32,
	direction: u32,
	hw: VirtiopcmHw,
	substream: *mut u8,
	elapsed_period: WorkStruct,
	msg_empty: *mut u8,
	lock: u32,
	xfer_enabled: bool,
	xfer_xrun: bool,
}

#[repr(C)]
struct VirtiopcmStream {
	nsubstreams: u32,
	substreams: *mut *mut VirtiopcmSubstream,
}

#[repr(C)]
struct VirtiopcmLayout {
	nid: u32,
	pcm: *mut u8,
	streams: [VirtiopcmStream; 2],
}

#[repr(C)]
struct VirtioSnd {
	vdev: *mut VirtioDevice,
	nsubstreams: u32,
	substreams: *mut VirtiopcmSubstream,
	pcm_list: *mut u8,
	card: *mut u8,
}

// Kernel macro and function stubs
const SNDRV_PCM_INFO_MMAP: u32 = 0x00000001;
const SNDRV_PCM_INFO_MMAP_VALID: u32 = 0x00000002;
const SNDRV_PCM_INFO_BATCH: u32 = 0x00000010;
const SNDRV_PCM_INFO_BLOCK_TRANSFER: u32 = 0x00000040;
const SNDRV_PCM_INFO_INTERLEAVED: u32 = 0x00000100;
const SNDRV_PCM_INFO_PAUSE: u32 = 0x00000800;
const SNDRV_PCM_INFO_NO_REWINDS: u32 = 0x00100000;
const SNDRV_PCM_INFO_SYNC_APPLPTR: u32 = 0x00200000;

const SNDRV_PCM_STREAM_PLAYBACK: u32 = 0;
const SNDRV_PCM_STREAM_CAPTURE: u32 = 1;

const VIRTIO_SND_D_OUTPUT: u8 = 0;
const VIRTIO_SND_D_INPUT: u8 = 1;

const VIRTIO_SND_EVT_PCM_PERIOD_ELAPSED: u32 = 0x0100;
const VIRTIO_SND_EVT_PCM_XRUN: u32 = 0x0101;

const VIRTIO_SND_R_PCM_INFO: u32 = 4;

const MSEC_PER_SEC: u32 = 1000;
const GFP_KERNEL: u32 = 0;

const EINVAL: i32 = -22;
const ENOMEM: i32 = -12;
const ENOENT: i32 = -2;

// Helper macro for offset calculation
macro_rules! offset_of {
	($ty:ty, $field:tt) => {{
		let dummy = core::mem::MaybeUninit::<$ty>::uninit();
		let dummy_ptr = dummy.as_ptr();
		unsafe {
			(std::ptr::addr_of!((*dummy_ptr).$field) as usize) - (dummy_ptr as usize)
		}
	}};
}

extern "C" {
	fn le32_to_cpu(x: u32) -> u32;
	fn le64_to_cpu(x: u64) -> u64;
	fn dev_err(dev: *mut u8, fmt: *const u8, ...) -> ();
	fn dev_warn(dev: *mut u8, fmt: *const u8, ...) -> ();
	fn devm_kzalloc(dev: *mut u8, size: usize, flags: u32) -> *mut u8;
	fn devm_kcalloc(dev: *mut u8, n: u32, size: usize, flags: u32) -> *mut u8;
	fn kzalloc(size: usize, flags: u32) -> *mut u8;
	fn kfree(ptr: *mut u8);
	fn virtsnd_ctl_query_info(snd: *mut VirtioSnd, code: u32, start: u32, count: u32, size: usize, info: *mut u8) -> i32;
	fn snd_pcm_format_physical_width(fmt: snd_pcm_format_t) -> i32;
	fn pcm_format_to_bits(fmt: snd_pcm_format_t) -> u64;
	fn init_waitqueue_head(queue: *mut u8);
	fn spin_lock_init(lock: *mut u32);
	fn snd_pcm_period_elapsed(substream: *mut u8);
	fn virtio_cread_le(vdev: *mut VirtioDevice, config_type: *const u8, field: *const u8, data: *mut u8) -> ();
	fn IS_ERR(ptr: *mut i32) -> bool;
	fn ERR_PTR(error: i32) -> *mut u8;
	fn PTR_ERR(ptr: *mut i32) -> i32;
	fn PAGE_ALIGN(x: usize) -> usize;
	fn spinlock_guard(lock: *mut u32) -> *mut u8;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
