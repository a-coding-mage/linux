// SPDX-License-Identifier: GPL-2.0
/*
 * Handling of TPM command and other buffers.
 */

unsafe fn __tpm_buf_size_invariant(buf: *mut tpm_buf, buf_size: u16) {
    let buf_size_2 = (*buf).capacity as u32 + core::mem::size_of::<tpm_buf>() as u32;

    if (*buf).capacity == 0 {
        if buf_size > TPM_BUFSIZE {
            WARN!(1, "{}: size overflow: {}\n", "__tpm_buf_size_invariant", buf_size);
            (*buf).flags |= TPM_BUF_INVALID;
        }
    } else if buf_size as u32 != buf_size_2 {
        WARN!(1, "{}: size mismatch: {} != {}\n", "__tpm_buf_size_invariant", buf_size, buf_size_2);
        (*buf).flags |= TPM_BUF_INVALID;
    }
}

unsafe fn __tpm_buf_reset(buf: *mut tpm_buf, buf_size: u16, tag: u16, ordinal: u32) {
    let head = (*buf).data.as_mut_ptr() as *mut tpm_header;

    __tpm_buf_size_invariant(buf, buf_size);
    if (*buf).flags & TPM_BUF_INVALID != 0 {
        return;
    }

    WARN_ON!(tag != TPM_TAG_RQU_COMMAND && tag != TPM2_ST_NO_SESSIONS &&
        tag != TPM2_ST_SESSIONS && tag != 0);

    (*buf).flags = 0;
    (*buf).length = core::mem::size_of::<tpm_header>() as u16;
    (*buf).capacity = buf_size - core::mem::size_of::<tpm_buf>() as u16;
    (*buf).handles = 0;
    (*head).tag = cpu_to_be16(tag);
    (*head).length = cpu_to_be32(core::mem::size_of::<tpm_header>() as u32);
    (*head).ordinal = cpu_to_be32(ordinal);
}

unsafe fn __tpm_buf_reset_sized(buf: *mut tpm_buf, buf_size: u16) {
    __tpm_buf_size_invariant(buf, buf_size);
    if (*buf).flags & TPM_BUF_INVALID != 0 {
        return;
    }

    (*buf).flags = TPM_BUF_TPM2B;
    (*buf).length = 2;
    (*buf).capacity = buf_size - core::mem::size_of::<tpm_buf>() as u16;
    (*buf).handles = 0;
    (*buf).data[0] = 0;
    (*buf).data[1] = 0;
}

/// Initialize a TPM command.
pub unsafe fn tpm_buf_init(buf: *mut tpm_buf, buf_size: u16) {
    core::ptr::write_bytes(buf as *mut u8, 0, buf_size as usize);
    __tpm_buf_reset(buf, buf_size, TPM_TAG_RQU_COMMAND, 0);
}

/// Initialize a sized buffer.
pub unsafe fn tpm_buf_init_sized(buf: *mut tpm_buf, buf_size: u16) {
    core::ptr::write_bytes(buf as *mut u8, 0, buf_size as usize);
    __tpm_buf_reset_sized(buf, buf_size);
}

/// Re-initialize a TPM command.
pub unsafe fn tpm_buf_reset(buf: *mut tpm_buf, tag: u16, ordinal: u32) {
    let buf_size = (*buf).capacity + core::mem::size_of::<tpm_buf>() as u16;
    __tpm_buf_reset(buf, buf_size, tag, ordinal);
}

/// Re-initialize a sized buffer.
pub unsafe fn tpm_buf_reset_sized(buf: *mut tpm_buf) {
    let buf_size = (*buf).capacity + core::mem::size_of::<tpm_buf>() as u16;
    __tpm_buf_reset_sized(buf, buf_size);
}

/// Return the number of bytes consumed by the data.
pub unsafe fn tpm_buf_length(buf: *mut tpm_buf) -> u16 {
    (*buf).length
}

/// Append data to an initialized buffer.
pub unsafe fn tpm_buf_append(buf: *mut tpm_buf, new_data: *const u8, new_length: u16) {
    let total_length = (*buf).length as u32 + new_length as u32;
    if (*buf).flags & TPM_BUF_INVALID != 0 { return; }
    if total_length > (*buf).capacity as u32 {
        WARN!(1, "tpm_buf: write overflow\n");
        (*buf).flags |= TPM_BUF_INVALID;
        return;
    }
    core::ptr::copy_nonoverlapping(new_data, (*buf).data.as_mut_ptr().add((*buf).length as usize), new_length as usize);
    (*buf).length += new_length;
    if (*buf).flags & TPM_BUF_TPM2B != 0 {
        *( (*buf).data.as_mut_ptr() as *mut u16) = cpu_to_be16((*buf).length - 2);
    } else {
        (*( (*buf).data.as_mut_ptr() as *mut tpm_header)).length = cpu_to_be32((*buf).length as u32);
    }
}

pub unsafe fn tpm_buf_append_u8(buf: *mut tpm_buf, value: u8) { tpm_buf_append(buf, &value, 1); }

pub unsafe fn tpm_buf_append_u16(buf: *mut tpm_buf, value: u16) {
    let value2 = cpu_to_be16(value);
    tpm_buf_append(buf, &value2 as *const u16 as *const u8, 2);
}

pub unsafe fn tpm_buf_append_u32(buf: *mut tpm_buf, value: u32) {
    let value2 = cpu_to_be32(value);
    tpm_buf_append(buf, &value2 as *const u32 as *const u8, 4);
}

pub unsafe fn tpm_buf_append_handle(buf: *mut tpm_buf, handle: u32) {
    if (*buf).flags & TPM_BUF_INVALID != 0 { return; }
    if (*buf).flags & TPM_BUF_TPM2B != 0 {
        WARN!(1, "tpm-buf: invalid type: TPM2B\n");
        (*buf).flags |= TPM_BUF_INVALID;
        return;
    }
    tpm_buf_append_u32(buf, handle);
    (*buf).handles += 1;
}

unsafe fn tpm_buf_read(buf: *mut tpm_buf, offset: *mut isize, count: usize, output: *mut core::ffi::c_void) {
    if (*buf).flags & TPM_BUF_INVALID != 0 { return; }
    let next_offset = *offset + count as isize;
    if next_offset > (*buf).length as isize {
        WARN!(1, "tpm_buf: read out of boundary\n");
        (*buf).flags |= TPM_BUF_INVALID;
        return;
    }
    core::ptr::copy_nonoverlapping((*buf).data.as_ptr().add(*offset as usize), output as *mut u8, count);
    *offset = next_offset;
}

pub unsafe fn tpm_buf_read_u8(buf: *mut tpm_buf, offset: *mut isize) -> u8 {
    let mut value = 0u8;
    tpm_buf_read(buf, offset, core::mem::size_of_val(&value), &mut value as *mut u8 as *mut core::ffi::c_void);
    value
}

pub unsafe fn tpm_buf_read_u16(buf: *mut tpm_buf, offset: *mut isize) -> u16 {
    let mut value = 0u16;
    tpm_buf_read(buf, offset, core::mem::size_of_val(&value), &mut value as *mut u16 as *mut core::ffi::c_void);
    be16_to_cpu(value)
}

pub unsafe fn tpm_buf_read_u32(buf: *mut tpm_buf, offset: *mut isize) -> u32 {
    let mut value = 0u32;
    tpm_buf_read(buf, offset, core::mem::size_of_val(&value), &mut value as *mut u32 as *mut core::ffi::c_void);
    be32_to_cpu(value)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
