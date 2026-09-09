// SPDX-License-Identifier: GPL-2.0-only
// SPDX-FileCopyrightText: Copyright (c) 2023 NVIDIA CORPORATION & AFFILIATES. All rights reserved.
/*
 * Crypto driver file to manage keys of NVIDIA Security Engine.
 */

// Dependencies supplied by the surrounding kernel/driver translation.

const SE_KEY_FULL_MASK: u16 = genmask(SE_MAX_KEYSLOT, 0);

/* Reserve keyslot 0, 14, 15 */
const SE_KEY_RSVD_MASK: u16 = (bit(0) | bit(14) | bit(15));
const SE_KEY_VALID_MASK: u16 = SE_KEY_FULL_MASK & !SE_KEY_RSVD_MASK;

/* Mutex lock to guard keyslots */
extern "C" {
    static mut kslt_lock: Mutex;
}

/* Keyslot bitmask (0 = available, 1 = in use/not available) */
static mut tegra_se_keyslots: u16 = SE_KEY_RSVD_MASK;

unsafe fn tegra_keyslot_alloc() -> u16 {
    let keyid: u16;

    mutex_lock(&raw mut kslt_lock);
    /* Check if all key slots are full */
    if tegra_se_keyslots == genmask(SE_MAX_KEYSLOT, 0) {
        mutex_unlock(&raw mut kslt_lock);
        return 0;
    }

    keyid = (!tegra_se_keyslots).trailing_zeros() as u16;
    tegra_se_keyslots |= bit(keyid);

    mutex_unlock(&raw mut kslt_lock);

    keyid
}

unsafe fn tegra_keyslot_free(slot: u16) {
    mutex_lock(&raw mut kslt_lock);
    tegra_se_keyslots &= !bit(slot);
    mutex_unlock(&raw mut kslt_lock);
}

unsafe fn tegra_key_prep_ins_cmd(
    se: *mut tegra_se,
    cpuvaddr: *mut u32,
    key: *const u32,
    keylen: u32,
    slot: u16,
    alg: u32,
) -> u32 {
    let mut i: u32 = 0;
    let mut j: u32;

    *cpuvaddr.add(i as usize) = host1x_opcode_setpayload(1); i += 1;
    *cpuvaddr.add(i as usize) = se_host1x_opcode_incr_w((*(*se).hw).regs.op); i += 1;
    *cpuvaddr.add(i as usize) = SE_AES_OP_WRSTALL | SE_AES_OP_DUMMY; i += 1;

    *cpuvaddr.add(i as usize) = host1x_opcode_setpayload(1); i += 1;
    *cpuvaddr.add(i as usize) = se_host1x_opcode_incr_w((*(*se).hw).regs.manifest); i += 1;
    *cpuvaddr.add(i as usize) = ((*se).manifest)((*se).owner, alg, keylen); i += 1;
    *cpuvaddr.add(i as usize) = host1x_opcode_setpayload(1); i += 1;
    *cpuvaddr.add(i as usize) = se_host1x_opcode_incr_w((*(*se).hw).regs.key_dst); i += 1;
    *cpuvaddr.add(i as usize) = SE_AES_KEY_DST_INDEX(slot); i += 1;

    j = 0;
    while j < keylen / 4 {
        /* Set key address */
        *cpuvaddr.add(i as usize) = host1x_opcode_setpayload(1); i += 1;
        *cpuvaddr.add(i as usize) = se_host1x_opcode_incr_w((*(*se).hw).regs.key_addr); i += 1;
        *cpuvaddr.add(i as usize) = j; i += 1;

        /* Set key data */
        *cpuvaddr.add(i as usize) = host1x_opcode_setpayload(1); i += 1;
        *cpuvaddr.add(i as usize) = se_host1x_opcode_incr_w((*(*se).hw).regs.key_data); i += 1;
        *cpuvaddr.add(i as usize) = *key.add(j as usize); i += 1;
        j += 1;
    }

    *cpuvaddr.add(i as usize) = host1x_opcode_setpayload(1); i += 1;
    *cpuvaddr.add(i as usize) = se_host1x_opcode_incr_w((*(*se).hw).regs.config); i += 1;
    *cpuvaddr.add(i as usize) = SE_CFG_INS; i += 1;
    *cpuvaddr.add(i as usize) = host1x_opcode_setpayload(1); i += 1;
    *cpuvaddr.add(i as usize) = se_host1x_opcode_incr_w((*(*se).hw).regs.op); i += 1;
    *cpuvaddr.add(i as usize) = SE_AES_OP_WRSTALL | SE_AES_OP_START | SE_AES_OP_LASTBUF; i += 1;
    *cpuvaddr.add(i as usize) = se_host1x_opcode_nonincr(host1x_uclass_incr_syncpt_r(), 1); i += 1;
    *cpuvaddr.add(i as usize) = host1x_uclass_incr_syncpt_cond_f(1) | host1x_uclass_incr_syncpt_indx_f((*se).syncpt_id); i += 1;

    dev_dbg((*se).dev, "key-slot %u key-manifest %#x\n", slot, ((*se).manifest)((*se).owner, alg, keylen));
    i
}

unsafe fn tegra_key_in_kslt(keyid: u32) -> bool {
    if keyid > SE_MAX_KEYSLOT { return false; }
    mutex_lock(&raw mut kslt_lock);
    let ret = (bit(keyid) & SE_KEY_VALID_MASK != 0) && (bit(keyid) & tegra_se_keyslots != 0);
    mutex_unlock(&raw mut kslt_lock);
    ret
}

unsafe fn tegra_key_insert(se: *mut tegra_se, key: *const u8, keylen: u32, slot: u16, alg: u32) -> i32 {
    let keyval = key as *const u32;
    let addr = (*(*se).keybuf).addr;
    mutex_lock(&raw mut kslt_lock);
    let size = tegra_key_prep_ins_cmd(se, addr, keyval, keylen, slot, alg);
    let ret = tegra_se_host1x_submit(se, (*se).keybuf, size);
    mutex_unlock(&raw mut kslt_lock);
    ret
}

pub unsafe fn tegra_key_invalidate(se: *mut tegra_se, keyid: u32, alg: u32) {
    let zkey = [0u8; AES_MAX_KEY_SIZE as usize];
    if keyid == 0 { return; }
    /* Overwrite the key with 0s */
    tegra_key_insert(se, zkey.as_ptr(), AES_MAX_KEY_SIZE, keyid as u16, alg);
    tegra_keyslot_free(keyid as u16);
}

pub unsafe fn tegra_key_invalidate_reserved(se: *mut tegra_se, keyid: u32, alg: u32) {
    let zkey = [0u8; AES_MAX_KEY_SIZE as usize];
    if keyid == 0 { return; }
    /* Overwrite the key with 0s */
    tegra_key_insert(se, zkey.as_ptr(), AES_MAX_KEY_SIZE, keyid as u16, alg);
}

pub unsafe fn tegra_key_submit_reserved(se: *mut tegra_se, key: *const u8, keylen: u32, alg: u32, keyid: *mut u32) -> i32 {
    tegra_key_insert(se, key, keylen, *keyid as u16, alg)
}

pub unsafe fn tegra_key_submit(se: *mut tegra_se, key: *const u8, keylen: u32, alg: u32, keyid: *mut u32) -> i32 {
    if !tegra_key_in_kslt(*keyid) {
        *keyid = tegra_keyslot_alloc() as u32;
        if *keyid == 0 {
            dev_dbg((*se).dev, "failed to allocate key slot\n");
            return -ENOMEM;
        }
    }
    let ret = tegra_key_insert(se, key, keylen, *keyid as u16, alg);
    if ret != 0 { return ret; }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
