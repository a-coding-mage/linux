// SPDX-License-Identifier: GPL-2.0
/* Builtin firmware support */

/* Dependencies supplied by the surrounding kernel translation. */
use core::ffi::{c_char, c_void};

#[repr(C)]
pub struct firmware {
    pub size: usize,
    pub data: *const c_void,
}

#[repr(C)]
pub struct builtin_fw {
    pub name: *mut c_char,
    pub data: *mut c_void,
    pub size: usize,
}

unsafe extern "C" {
    pub static mut __start_builtin_fw: builtin_fw;
    pub static mut __end_builtin_fw: builtin_fw;

    fn strcmp(lhs: *const c_char, rhs: *const c_char) -> i32;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
}

/* Only if FW_LOADER=y. */

unsafe fn fw_copy_to_prealloc_buf(
    fw: *const firmware,
    buf: *mut c_void,
    size: usize,
) -> bool {
    if buf.is_null() {
        return true;
    }
    if size < (*fw).size {
        return false;
    }
    memcpy(buf, (*fw).data, (*fw).size);
    true
}

/**
 * firmware_request_builtin() - load builtin firmware
 * @fw: pointer to firmware struct
 * @name: name of firmware file
 *
 * Some use cases in the kernel have a requirement so that no memory allocator
 * is involved as these calls take place early in boot process. An example is
 * the x86 CPU microcode loader. In these cases all the caller wants is to see
 * if the firmware was built-in and if so use it right away. This can be used
 * for such cases.
 *
 * This looks for the firmware in the built-in kernel. Only if the kernel was
 * built-in with the firmware you are looking for will this return successfully.
 *
 * Callers of this API do not need to use release_firmware() as the pointer to
 * the firmware is expected to be provided locally on the stack of the caller.
 **/
pub unsafe fn firmware_request_builtin(fw: *mut firmware, name: *const c_char) -> bool {
    let mut b_fw: *mut builtin_fw = &raw mut __start_builtin_fw;
    let end: *mut builtin_fw = &raw mut __end_builtin_fw;

    if fw.is_null() {
        return false;
    }

    while b_fw != end {
        if strcmp(name, (*b_fw).name) == 0 {
            if (*b_fw).size == 0 {
                return false;
            }
            (*fw).size = (*b_fw).size;
            (*fw).data = (*b_fw).data;
            return true;
        }
        b_fw = b_fw.add(1);
    }

    false
}

/* EXPORT_SYMBOL_NS_GPL(firmware_request_builtin, "TEST_FIRMWARE"); */

/**
 * firmware_request_builtin_buf() - load builtin firmware into optional buffer
 * @fw: pointer to firmware struct
 * @name: name of firmware file
 * @buf: If set this lets you use a pre-allocated buffer so that the built-in
 *\tfirmware into is copied into. This field can be NULL. It is used by
 *\tcallers such as request_firmware_into_buf() and
 *\trequest_partial_firmware_into_buf()
 * @size: if buf was provided, the max size of the allocated buffer available.
 *\tIf the built-in firmware does not fit into the pre-allocated @buf this
 *\tcall will fail.
 *
 * This looks for the firmware in the built-in kernel. Only if the kernel was
 * built-in with the firmware you are looking for will this call possibly
 * succeed. If you passed a @buf the firmware will be copied into it *iff* the
 * built-in firmware fits into the pre-allocated buffer size specified in
 * @size.
 *
 * This caller is to be used internally by the firmware_loader only.
 **/
pub unsafe fn firmware_request_builtin_buf(
    fw: *mut firmware,
    name: *const c_char,
    buf: *mut c_void,
    size: usize,
) -> bool {
    if !firmware_request_builtin(fw, name) {
        return false;
    }

    fw_copy_to_prealloc_buf(fw, buf, size)
}

pub unsafe fn firmware_is_builtin(fw: *const firmware) -> bool {
    let mut b_fw: *mut builtin_fw = &raw mut __start_builtin_fw;
    let end: *mut builtin_fw = &raw mut __end_builtin_fw;

    while b_fw != end {
        if (*fw).data == (*b_fw).data {
            return true;
        }
        b_fw = b_fw.add(1);
    }

    false
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
