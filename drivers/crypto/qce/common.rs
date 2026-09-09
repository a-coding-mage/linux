// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2012-2014, The Linux Foundation. All rights reserved.
 */

#[inline]
unsafe fn qce_read(qce: *mut qce_device, offset: u32) -> u32 {
    readl((*qce).base.add(offset as usize))
}

#[inline]
unsafe fn qce_write(qce: *mut qce_device, offset: u32, val: u32) {
    writel(val, (*qce).base.add(offset as usize));
}

#[inline]
unsafe fn qce_write_array(qce: *mut qce_device, offset: u32, val: *const u32, len: u32) {
    let mut i = 0;
    while i < len {
        qce_write(qce, offset + i * core::mem::size_of::<u32>() as u32, *val.add(i as usize));
        i += 1;
    }
}

#[inline]
unsafe fn qce_clear_array(qce: *mut qce_device, offset: u32, len: u32) {
    let mut i = 0;
    while i < len {
        qce_write(qce, offset + i * core::mem::size_of::<u32>() as u32, 0);
        i += 1;
    }
}

unsafe fn qce_config_reg(qce: *mut qce_device, little: i32) -> u32 {
    let beats = ((*qce).burst_size >> 3).wrapping_sub(1);
    let pipe_pair = (*qce).pipe_pair_id;
    let mut config = (beats << REQ_SIZE_SHIFT) & REQ_SIZE_MASK;
    config |= BIT(MASK_DOUT_INTR_SHIFT) | BIT(MASK_DIN_INTR_SHIFT)
        | BIT(MASK_OP_DONE_INTR_SHIFT) | BIT(MASK_ERR_INTR_SHIFT);
    config |= (pipe_pair << PIPE_SET_SELECT_SHIFT) & PIPE_SET_SELECT_MASK;
    config &= !HIGH_SPD_EN_N_SHIFT;
    if little != 0 {
        config |= BIT(LITTLE_ENDIAN_MODE_SHIFT);
    }
    config
}

pub unsafe fn qce_cpu_to_be32p_array(dst: *mut __be32, src: *const u8, len: u32) {
    let mut d = dst;
    let mut s = src;
    let mut n = len / core::mem::size_of::<u32>() as u32;
    while n > 0 {
        *d = cpu_to_be32p(s as *const __u32);
        s = s.add(core::mem::size_of::<__u32>());
        d = d.add(1);
        n -= 1;
    }
}

unsafe fn qce_setup_config(qce: *mut qce_device) {
    let config = qce_config_reg(qce, 0);
    qce_write(qce, REG_STATUS, 0);
    qce_write(qce, REG_CONFIG, config);
}

#[inline]
unsafe fn qce_crypto_go(qce: *mut qce_device, result_dump: bool) {
    if result_dump {
        qce_write(qce, REG_GOPROC, BIT(GO_SHIFT) | BIT(RESULTS_DUMP_SHIFT));
    } else {
        qce_write(qce, REG_GOPROC, BIT(GO_SHIFT));
    }
}

#[cfg(any(CONFIG_CRYPTO_DEV_QCE_SHA, CONFIG_CRYPTO_DEV_QCE_AEAD))]
unsafe fn qce_auth_cfg(flags: c_ulong, key_size: u32, auth_size: u32) -> u32 {
    let mut cfg = 0;
    if IS_CCM(flags) || IS_CMAC(flags) { cfg |= AUTH_ALG_AES << AUTH_ALG_SHIFT; }
    else { cfg |= AUTH_ALG_SHA << AUTH_ALG_SHIFT; }
    if IS_CCM(flags) || IS_CMAC(flags) {
        if key_size == AES_KEYSIZE_128 { cfg |= AUTH_KEY_SZ_AES128 << AUTH_KEY_SIZE_SHIFT; }
        else if key_size == AES_KEYSIZE_256 { cfg |= AUTH_KEY_SZ_AES256 << AUTH_KEY_SIZE_SHIFT; }
    }
    if IS_SHA256(flags) || IS_SHA256_HMAC(flags) { cfg |= AUTH_SIZE_SHA256 << AUTH_SIZE_SHIFT; }
    else if IS_CMAC(flags) { cfg |= AUTH_SIZE_ENUM_16_BYTES << AUTH_SIZE_SHIFT; }
    else if IS_CCM(flags) { cfg |= (auth_size - 1) << AUTH_SIZE_SHIFT; }
    if IS_SHA256(flags) { cfg |= AUTH_MODE_HASH << AUTH_MODE_SHIFT; }
    else if IS_SHA256_HMAC(flags) { cfg |= AUTH_MODE_HMAC << AUTH_MODE_SHIFT; }
    else if IS_CCM(flags) { cfg |= AUTH_MODE_CCM << AUTH_MODE_SHIFT; }
    else if IS_CMAC(flags) { cfg |= AUTH_MODE_CMAC << AUTH_MODE_SHIFT; }
    if IS_SHA(flags) || IS_SHA_HMAC(flags) { cfg |= AUTH_POS_BEFORE << AUTH_POS_SHIFT; }
    if IS_CCM(flags) { cfg |= QCE_MAX_NONCE_WORDS << AUTH_NONCE_NUM_WORDS_SHIFT; }
    cfg
}

// The remaining source functions are kept as direct unsafe Rust translations.
// External kernel, crypto, register, and request types/macros are supplied by other files.

#[cfg(any(CONFIG_CRYPTO_DEV_QCE_SKCIPHER, CONFIG_CRYPTO_DEV_QCE_AEAD))]
unsafe fn qce_encr_cfg(flags: c_ulong, aes_key_size: u32) -> u32 {
    let mut cfg = 0;
    if IS_AES(flags) {
        if aes_key_size == AES_KEYSIZE_128 { cfg |= ENCR_KEY_SZ_AES128 << ENCR_KEY_SZ_SHIFT; }
        else if aes_key_size == AES_KEYSIZE_256 { cfg |= ENCR_KEY_SZ_AES256 << ENCR_KEY_SZ_SHIFT; }
        cfg |= ENCR_ALG_AES << ENCR_ALG_SHIFT;
    }
    cfg |= match flags & QCE_MODE_MASK {
        QCE_MODE_CBC => ENCR_MODE_CBC << ENCR_MODE_SHIFT,
        QCE_MODE_CTR => ENCR_MODE_CTR << ENCR_MODE_SHIFT,
        QCE_MODE_XTS => ENCR_MODE_XTS << ENCR_MODE_SHIFT,
        QCE_MODE_CCM => (ENCR_MODE_CCM << ENCR_MODE_SHIFT) | LAST_CCM_XFR << LAST_CCM_SHIFT,
        _ => return !0,
    };
    cfg
}

pub unsafe fn qce_check_status(qce: *mut qce_device, status: *mut u32) -> i32 {
    *status = qce_read(qce, REG_STATUS);
    if *status & (BIT(SW_ERR_SHIFT) | BIT(AXI_ERR_SHIFT) | BIT(HSD_ERR_SHIFT)) != 0
        || *status & BIT(OPERATION_DONE_SHIFT) == 0 { -ENXIO }
    else if *status & BIT(MAC_FAILED_SHIFT) != 0 { -EBADMSG }
    else { 0 }
}

pub unsafe fn qce_start(async_req: *mut crypto_async_request, type_: u32) -> i32 {
    match type_ {
        #[cfg(CONFIG_CRYPTO_DEV_QCE_SKCIPHER)]
        CRYPTO_ALG_TYPE_SKCIPHER => qce_setup_regs_skcipher(async_req),
        #[cfg(CONFIG_CRYPTO_DEV_QCE_SHA)]
        CRYPTO_ALG_TYPE_AHASH => qce_setup_regs_ahash(async_req),
        #[cfg(CONFIG_CRYPTO_DEV_QCE_AEAD)]
        CRYPTO_ALG_TYPE_AEAD => qce_setup_regs_aead(async_req),
        _ => -EINVAL,
    }
}

pub unsafe fn qce_get_version(qce: *mut qce_device, major: *mut u32, minor: *mut u32, step: *mut u32) {
    let val = qce_read(qce, REG_VERSION);
    *major = (val & CORE_MAJOR_REV_MASK) >> CORE_MAJOR_REV_SHIFT;
    *minor = (val & CORE_MINOR_REV_MASK) >> CORE_MINOR_REV_SHIFT;
    *step = (val & CORE_STEP_REV_MASK) >> CORE_STEP_REV_SHIFT;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
