// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * NTFS kernel collation handling.
 *
 * Copyright (c) 2004 Anton Altaparmakov
 *
 * Part of this file is based on code from the NTFS-3G.
 * and is copyrighted by the respective authors below:
 * Copyright (c) 2004 Anton Altaparmakov
 * Copyright (c) 2005 Yura Pakhuchiy
 */

// Dependencies supplied by the surrounding NTFS implementation:
// collate.h, debug.h, ntfs.h, and Linux sort helpers.

unsafe fn ntfs_collate_binary(
	_vol: *mut ntfs_volume,
	data1: *const core::ffi::c_void,
	data1_len: u32,
	data2: *const core::ffi::c_void,
	data2_len: u32,
) -> i32 {
	let mut rc = unsafe {
		libc::memcmp(
			data1,
			data2,
			core::cmp::min(data1_len, data2_len) as usize,
		)
	};
	if rc == 0 && data1_len != data2_len {
		rc = if data1_len < data2_len { -1 } else { 1 };
	}
	rc
}

unsafe fn ntfs_collate_ntofs_ulong(
	_vol: *mut ntfs_volume,
	data1: *const core::ffi::c_void,
	data1_len: u32,
	data2: *const core::ffi::c_void,
	data2_len: u32,
) -> i32 {
	let d1 = u32::from_le(unsafe { core::ptr::read_unaligned(data1 as *const u32) });
	let d2 = u32::from_le(unsafe { core::ptr::read_unaligned(data2 as *const u32) });

	if data1_len != data2_len || data1_len != 4 {
		return -EINVAL;
	}
	if d1 < d2 {
		-1
	} else if d1 == d2 {
		0
	} else {
		1
	}
}

/*
 * ntfs_collate_ntofs_ulongs - Which of two le32 arrays should be listed first
 * @vol: ntfs volume
 * @data1: first ulong array to collate
 * @data1_len: length in bytes of @data1
 * @data2: second ulong array to collate
 * @data2_len: length in bytes of @data2
 *
 * Returns: -1, 0 or 1 depending of how the arrays compare
 */
unsafe fn ntfs_collate_ntofs_ulongs(
	vol: *mut ntfs_volume,
	data1: *const core::ffi::c_void,
	data1_len: u32,
	data2: *const core::ffi::c_void,
	data2_len: u32,
) -> i32 {
	if data1_len != data2_len || data1_len & 3 != 0 {
		ntfs_error((*vol).sb, "data1_len or data2_len not valid\n");
		return -1;
	}

	let mut p1 = data1 as *const u32;
	let mut p2 = data2 as *const u32;
	let mut len = data1_len;
	let (d1, d2);
	loop {
		let v1 = u32::from_le(core::ptr::read_unaligned(p1));
		p1 = p1.add(1);
		let v2 = u32::from_le(core::ptr::read_unaligned(p2));
		p2 = p2.add(1);
		if v1 != v2 || {
			len = len.wrapping_sub(4);
			len == 0
		} {
			d1 = v1;
			d2 = v2;
			break;
		}
	}
	cmp_int(d1, d2)
}

/*
 * ntfs_collate_file_name - Which of two filenames should be listed first
 * @vol: ntfs volume
 * @data1: first filename to collate
 * @data1_len: length in bytes of @data1(unused)
 * @data2: second filename to collate
 * @data2_len: length in bytes of @data2(unused)
 */
unsafe fn ntfs_collate_file_name(
	vol: *mut ntfs_volume,
	data1: *const core::ffi::c_void,
	_data1_len: u32,
	data2: *const core::ffi::c_void,
	_data2_len: u32,
) -> i32 {
	let mut rc = ntfs_file_compare_values(
		data1, data2, -EINVAL, IGNORE_CASE, (*vol).upcase, (*vol).upcase_len,
	);
	if rc == 0 {
		rc = ntfs_file_compare_values(
			data1, data2, -EINVAL, CASE_SENSITIVE, (*vol).upcase, (*vol).upcase_len,
		);
	}
	rc
}

/*
 * ntfs_collate - collate two data items using a specified collation rule
 * @vol: ntfs volume to which the data items belong
 * @cr: collation rule to use when comparing the items
 * @data1: first data item to collate
 * @data1_len: length in bytes of @data1
 * @data2: second data item to collate
 * @data2_len: length in bytes of @data2
 *
 * Collate the two data items @data1 and @data2 using the collation rule @cr
 * and return -1, 0, ir 1 if @data1 is found, respectively, to collate before,
 * to match, or to collate after @data2. return -EINVAL if an error occurred.
 */
pub unsafe fn ntfs_collate(
	vol: *mut ntfs_volume,
	cr: __le32,
	data1: *const core::ffi::c_void,
	data1_len: u32,
	data2: *const core::ffi::c_void,
	data2_len: u32,
) -> i32 {
	match le32_to_cpu(cr) {
		COLLATION_BINARY => ntfs_collate_binary(vol, data1, data1_len, data2, data2_len),
		COLLATION_FILE_NAME => ntfs_collate_file_name(vol, data1, data1_len, data2, data2_len),
		COLLATION_NTOFS_ULONG => ntfs_collate_ntofs_ulong(vol, data1, data1_len, data2, data2_len),
		COLLATION_NTOFS_ULONGS => ntfs_collate_ntofs_ulongs(vol, data1, data1_len, data2, data2_len),
		_ => {
			ntfs_error((*vol).sb, "Unknown collation rule 0x%x", le32_to_cpu(cr));
			-EINVAL
		}
	}
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
