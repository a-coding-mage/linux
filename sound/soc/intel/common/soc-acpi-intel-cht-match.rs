// SPDX-License-Identifier: GPL-2.0-only
/*
 * soc-acpi-intel-cht-match.c - tables and support for CHT ACPI enumeration.
 *
 * Copyright (c) 2017, Intel Corporation.
 */

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::ptr;

#[repr(C)]
pub struct dmi_strmatch {
	pub slot: c_int,
	pub substr: *const c_char,
}

#[repr(C)]
pub struct dmi_system_id {
	pub callback: Option<unsafe extern "C" fn(*const dmi_system_id) -> c_int>,
	pub ident: *const c_char,
	pub matches: [dmi_strmatch; 4],
	pub driver_data: *mut c_void,
}

#[repr(C)]
pub struct snd_soc_acpi_codecs {
	pub num_codecs: c_int,
	pub codecs: [*const c_char; 4],
}

#[repr(C)]
pub struct snd_soc_acpi_mach {
	pub id: *const c_char,
	pub uid: *const c_char,
	pub comp_ids: *const snd_soc_acpi_codecs,
	pub drv_name: *const c_char,
	pub fw_filename: *const c_char,
	pub board: *const c_char,
	pub machine_quirk: Option<unsafe extern "C" fn(*mut c_void) -> *mut snd_soc_acpi_mach>,
	pub sof_tplg_filename: *const c_char,
}

unsafe extern "C" {
	fn dmi_check_system(list: *const dmi_system_id) -> c_int;
}

const DMI_SYS_VENDOR: c_int = 1;
const DMI_PRODUCT_NAME: c_int = 2;
const DMI_PRODUCT_VERSION: c_int = 4;
const DMI_BIOS_VERSION: c_int = 5;

const fn dmi_match(slot: c_int, substr: *const c_char) -> dmi_strmatch {
	dmi_strmatch { slot, substr }
}

static mut cht_machine_id: c_ulong = 0;

const CHT_SURFACE_MACH: c_ulong = 1;

unsafe extern "C" fn cht_surface_quirk_cb(_id: *const dmi_system_id) -> c_int {
	unsafe {
		cht_machine_id = CHT_SURFACE_MACH;
	}
	1
}

static cht_table: [dmi_system_id; 2] = [
	dmi_system_id {
		callback: Some(cht_surface_quirk_cb),
		ident: ptr::null(),
		matches: [
			dmi_match(DMI_SYS_VENDOR, c"Microsoft Corporation".as_ptr()),
			dmi_match(DMI_PRODUCT_NAME, c"Surface 3".as_ptr()),
			dmi_match(0, ptr::null()),
			dmi_match(0, ptr::null()),
		],
		driver_data: ptr::null_mut(),
	},
	dmi_system_id {
		callback: None,
		ident: ptr::null(),
		matches: [
			dmi_match(0, ptr::null()),
			dmi_match(0, ptr::null()),
			dmi_match(0, ptr::null()),
			dmi_match(0, ptr::null()),
		],
		driver_data: ptr::null_mut(),
	},
];

static mut cht_surface_mach: snd_soc_acpi_mach = snd_soc_acpi_mach {
	id: c"10EC5640".as_ptr(),
	uid: ptr::null(),
	comp_ids: ptr::null(),
	drv_name: c"cht-bsw-rt5645".as_ptr(),
	fw_filename: c"intel/fw_sst_22a8.bin".as_ptr(),
	board: c"cht-bsw".as_ptr(),
	machine_quirk: None,
	sof_tplg_filename: c"sof-cht-rt5645.tplg".as_ptr(),
};

unsafe extern "C" fn cht_quirk(arg: *mut c_void) -> *mut snd_soc_acpi_mach {
	let mach = arg as *mut snd_soc_acpi_mach;

	unsafe {
		dmi_check_system(cht_table.as_ptr());

		if cht_machine_id == CHT_SURFACE_MACH {
			&raw mut cht_surface_mach
		} else {
			mach
		}
	}
}

/*
 * Some tablets with Android factory OS have buggy DSDTs with an ESSX8316 device
 * in the ACPI tables. While they are not using an ESS8316 codec. These DSDTs
 * also have an ACPI device for the correct codec, ignore the ESSX8316.
 */
static cht_ess8316_not_present_table: [dmi_system_id; 2] = [
	dmi_system_id {
		/* Nextbook Ares 8A */
		callback: None,
		ident: ptr::null(),
		matches: [
			dmi_match(DMI_SYS_VENDOR, c"Insyde".as_ptr()),
			dmi_match(DMI_PRODUCT_NAME, c"CherryTrail".as_ptr()),
			dmi_match(DMI_BIOS_VERSION, c"M882".as_ptr()),
			dmi_match(0, ptr::null()),
		],
		driver_data: ptr::null_mut(),
	},
	dmi_system_id {
		callback: None,
		ident: ptr::null(),
		matches: [
			dmi_match(0, ptr::null()),
			dmi_match(0, ptr::null()),
			dmi_match(0, ptr::null()),
			dmi_match(0, ptr::null()),
		],
		driver_data: ptr::null_mut(),
	},
];

unsafe extern "C" fn cht_ess8316_quirk(arg: *mut c_void) -> *mut snd_soc_acpi_mach {
	unsafe {
		if dmi_check_system(cht_ess8316_not_present_table.as_ptr()) != 0 {
			return ptr::null_mut();
		}
	}

	arg as *mut snd_soc_acpi_mach
}

/*
 * The Lenovo Yoga Tab 3 Pro YT3-X90, with Android factory OS has a buggy DSDT
 * with the coded not being listed at all.
 */
static lenovo_yoga_tab3_x90: [dmi_system_id; 2] = [
	dmi_system_id {
		/* Lenovo Yoga Tab 3 Pro YT3-X90, codec missing from DSDT */
		callback: None,
		ident: ptr::null(),
		matches: [
			dmi_match(DMI_SYS_VENDOR, c"Intel Corporation".as_ptr()),
			dmi_match(DMI_PRODUCT_VERSION, c"Blade3-10A-001".as_ptr()),
			dmi_match(0, ptr::null()),
			dmi_match(0, ptr::null()),
		],
		driver_data: ptr::null_mut(),
	},
	dmi_system_id {
		callback: None,
		ident: ptr::null(),
		matches: [
			dmi_match(0, ptr::null()),
			dmi_match(0, ptr::null()),
			dmi_match(0, ptr::null()),
			dmi_match(0, ptr::null()),
		],
		driver_data: ptr::null_mut(),
	},
];

static mut cht_lenovo_yoga_tab3_x90_mach: snd_soc_acpi_mach = snd_soc_acpi_mach {
	id: c"10WM5102".as_ptr(),
	uid: ptr::null(),
	comp_ids: ptr::null(),
	drv_name: c"bytcr_wm5102".as_ptr(),
	fw_filename: c"intel/fw_sst_22a8.bin".as_ptr(),
	board: c"bytcr_wm5102".as_ptr(),
	machine_quirk: None,
	sof_tplg_filename: c"sof-cht-wm5102.tplg".as_ptr(),
};

unsafe extern "C" fn lenovo_yt3_x90_quirk(_arg: *mut c_void) -> *mut snd_soc_acpi_mach {
	unsafe {
		if dmi_check_system(lenovo_yoga_tab3_x90.as_ptr()) != 0 {
			return &raw mut cht_lenovo_yoga_tab3_x90_mach;
		}
	}

	/* Skip wildcard match snd_soc_acpi_intel_cherrytrail_machines[] entry */
	ptr::null_mut()
}

static rt5640_comp_ids: snd_soc_acpi_codecs = snd_soc_acpi_codecs {
	num_codecs: 2,
	codecs: [
		c"10EC5640".as_ptr(),
		c"10EC3276".as_ptr(),
		ptr::null(),
		ptr::null(),
	],
};

static rt5670_comp_ids: snd_soc_acpi_codecs = snd_soc_acpi_codecs {
	num_codecs: 2,
	codecs: [
		c"10EC5670".as_ptr(),
		c"10EC5672".as_ptr(),
		ptr::null(),
		ptr::null(),
	],
};

static rt5645_comp_ids: snd_soc_acpi_codecs = snd_soc_acpi_codecs {
	num_codecs: 3,
	codecs: [
		c"10EC5645".as_ptr(),
		c"10EC5650".as_ptr(),
		c"10EC3270".as_ptr(),
		ptr::null(),
	],
};

static da7213_comp_ids: snd_soc_acpi_codecs = snd_soc_acpi_codecs {
	num_codecs: 2,
	codecs: [
		c"DGLS7212".as_ptr(),
		c"DGLS7213".as_ptr(),
		ptr::null(),
		ptr::null(),
	],
};

/* Cherryview-based platforms: CherryTrail and Braswell */
#[unsafe(no_mangle)]
pub static mut snd_soc_acpi_intel_cherrytrail_machines: [snd_soc_acpi_mach; 14] = [
	snd_soc_acpi_mach {
		id: ptr::null(),
		uid: ptr::null(),
		comp_ids: &rt5670_comp_ids,
		drv_name: c"cht-bsw-rt5672".as_ptr(),
		fw_filename: c"intel/fw_sst_22a8.bin".as_ptr(),
		board: c"cht-bsw".as_ptr(),
		machine_quirk: None,
		sof_tplg_filename: c"sof-cht-rt5670.tplg".as_ptr(),
	},
	snd_soc_acpi_mach {
		id: ptr::null(),
		uid: ptr::null(),
		comp_ids: &rt5645_comp_ids,
		drv_name: c"cht-bsw-rt5645".as_ptr(),
		fw_filename: c"intel/fw_sst_22a8.bin".as_ptr(),
		board: c"cht-bsw".as_ptr(),
		machine_quirk: None,
		sof_tplg_filename: c"sof-cht-rt5645.tplg".as_ptr(),
	},
	snd_soc_acpi_mach {
		id: c"193C9890".as_ptr(),
		uid: ptr::null(),
		comp_ids: ptr::null(),
		drv_name: c"cht-bsw-max98090".as_ptr(),
		fw_filename: c"intel/fw_sst_22a8.bin".as_ptr(),
		board: c"cht-bsw".as_ptr(),
		machine_quirk: None,
		sof_tplg_filename: c"sof-cht-max98090.tplg".as_ptr(),
	},
	snd_soc_acpi_mach {
		id: c"10508824".as_ptr(),
		uid: ptr::null(),
		comp_ids: ptr::null(),
		drv_name: c"cht-bsw-nau8824".as_ptr(),
		fw_filename: c"intel/fw_sst_22a8.bin".as_ptr(),
		board: c"cht-bsw".as_ptr(),
		machine_quirk: None,
		sof_tplg_filename: c"sof-cht-nau8824.tplg".as_ptr(),
	},
	snd_soc_acpi_mach {
		id: ptr::null(),
		uid: ptr::null(),
		comp_ids: &da7213_comp_ids,
		drv_name: c"bytcht_da7213".as_ptr(),
		fw_filename: c"intel/fw_sst_22a8.bin".as_ptr(),
		board: c"bytcht_da7213".as_ptr(),
		machine_quirk: None,
		sof_tplg_filename: c"sof-cht-da7213.tplg".as_ptr(),
	},
	snd_soc_acpi_mach {
		id: c"ESSX8316".as_ptr(),
		uid: ptr::null(),
		comp_ids: ptr::null(),
		drv_name: c"bytcht_es8316".as_ptr(),
		fw_filename: c"intel/fw_sst_22a8.bin".as_ptr(),
		board: c"bytcht_es8316".as_ptr(),
		machine_quirk: Some(cht_ess8316_quirk),
		sof_tplg_filename: c"sof-cht-es8316.tplg".as_ptr(),
	},
	/* some CHT-T platforms rely on RT5640, use Baytrail machine driver */
	snd_soc_acpi_mach {
		id: ptr::null(),
		uid: ptr::null(),
		comp_ids: &rt5640_comp_ids,
		drv_name: c"bytcr_rt5640".as_ptr(),
		fw_filename: c"intel/fw_sst_22a8.bin".as_ptr(),
		board: c"bytcr_rt5640".as_ptr(),
		machine_quirk: Some(cht_quirk),
		sof_tplg_filename: c"sof-cht-rt5640.tplg".as_ptr(),
	},
	snd_soc_acpi_mach {
		id: c"10EC5682".as_ptr(),
		uid: ptr::null(),
		comp_ids: ptr::null(),
		drv_name: c"sof_rt5682".as_ptr(),
		fw_filename: ptr::null(),
		board: ptr::null(),
		machine_quirk: None,
		sof_tplg_filename: c"sof-cht-rt5682.tplg".as_ptr(),
	},
	/* some CHT-T platforms rely on RT5651, use Baytrail machine driver */
	snd_soc_acpi_mach {
		id: c"10EC5651".as_ptr(),
		uid: ptr::null(),
		comp_ids: ptr::null(),
		drv_name: c"bytcr_rt5651".as_ptr(),
		fw_filename: c"intel/fw_sst_22a8.bin".as_ptr(),
		board: c"bytcr_rt5651".as_ptr(),
		machine_quirk: None,
		sof_tplg_filename: c"sof-cht-rt5651.tplg".as_ptr(),
	},
	snd_soc_acpi_mach {
		id: c"14F10720".as_ptr(),
		uid: ptr::null(),
		comp_ids: ptr::null(),
		drv_name: c"bytcht_cx2072x".as_ptr(),
		fw_filename: c"intel/fw_sst_22a8.bin".as_ptr(),
		board: c"bytcht_cx2072x".as_ptr(),
		machine_quirk: None,
		sof_tplg_filename: c"sof-cht-cx2072x.tplg".as_ptr(),
	},
	snd_soc_acpi_mach {
		id: c"104C5122".as_ptr(),
		uid: ptr::null(),
		comp_ids: ptr::null(),
		drv_name: c"sof_pcm512x".as_ptr(),
		fw_filename: ptr::null(),
		board: ptr::null(),
		machine_quirk: None,
		sof_tplg_filename: c"sof-cht-src-50khz-pcm512x.tplg".as_ptr(),
	},
	/*
	 * Special case for the Lenovo Yoga Tab 3 Pro YT3-X90 where the DSDT
	 * misses the codec. Match on the SST id instead, lenovo_yt3_x90_quirk()
	 * will return a YT3 specific mach or NULL when called on other hw,
	 * skipping this entry.
	 */
	snd_soc_acpi_mach {
		id: c"808622A8".as_ptr(),
		uid: ptr::null(),
		comp_ids: ptr::null(),
		drv_name: ptr::null(),
		fw_filename: ptr::null(),
		board: ptr::null(),
		machine_quirk: Some(lenovo_yt3_x90_quirk),
		sof_tplg_filename: ptr::null(),
	},
	/*
	 * C conditional preserved from source:
	 * #if IS_ENABLED(CONFIG_SND_SOC_INTEL_BYT_CHT_NOCODEC_MACH)
	 *
	 * This is always last in the table so that it is selected only when
	 * enabled explicitly and there is no codec-related information in SSDT
	 */
	snd_soc_acpi_mach {
		id: c"808622A8".as_ptr(),
		uid: ptr::null(),
		comp_ids: ptr::null(),
		drv_name: c"bytcht_nocodec".as_ptr(),
		fw_filename: c"intel/fw_sst_22a8.bin".as_ptr(),
		board: c"bytcht_nocodec".as_ptr(),
		machine_quirk: None,
		sof_tplg_filename: ptr::null(),
	},
	snd_soc_acpi_mach {
		id: ptr::null(),
		uid: ptr::null(),
		comp_ids: ptr::null(),
		drv_name: ptr::null(),
		fw_filename: ptr::null(),
		board: ptr::null(),
		machine_quirk: None,
		sof_tplg_filename: ptr::null(),
	},
];

/* EXPORT_SYMBOL_GPL(snd_soc_acpi_intel_cherrytrail_machines); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
