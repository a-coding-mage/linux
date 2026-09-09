// SPDX-License-Identifier: (BSD-3-Clause OR GPL-2.0-only)
/* Copyright(c) 2014 - 2020 Intel Corporation */

// Dependencies supplied by the surrounding kernel/QAT translation.

const ADF_MAX_RING_THRESHOLD: u32 = 80;

#[inline]
unsafe fn adf_modulo(data: u32, shift: u32) -> u32 {
    let div = data >> shift;
    let mult = div << shift;
    data - mult
}

#[inline]
unsafe fn adf_check_ring_alignment(addr: u64, size: u64) -> i32 {
    if ((size - 1) & addr) != 0 { -EFAULT } else { 0 }
}

unsafe fn adf_verify_ring_size(msg_size: u32, msg_num: u32) -> i32 {
    let mut i = ADF_MIN_RING_SIZE;
    while i <= ADF_MAX_RING_SIZE {
        if msg_size * msg_num == ADF_SIZE_TO_RING_SIZE_IN_BYTES(i) { return i as i32; }
        i += 1;
    }
    ADF_DEFAULT_RING_SIZE as i32
}

unsafe fn adf_reserve_ring(bank: *mut adf_etr_bank_data, ring: u32) -> i32 {
    spin_lock(&mut (*bank).lock);
    if (*bank).ring_mask & (1 << ring) != 0 {
        spin_unlock(&mut (*bank).lock);
        return -EFAULT;
    }
    (*bank).ring_mask |= 1 << ring;
    spin_unlock(&mut (*bank).lock);
    0
}

unsafe fn adf_unreserve_ring(bank: *mut adf_etr_bank_data, ring: u32) {
    spin_lock(&mut (*bank).lock);
    (*bank).ring_mask &= !(1 << ring);
    spin_unlock(&mut (*bank).lock);
}

unsafe fn adf_enable_ring_irq(bank: *mut adf_etr_bank_data, ring: u32) {
    let csr_ops = GET_CSR_OPS((*bank).accel_dev);
    spin_lock_bh(&mut (*bank).lock);
    (*bank).irq_mask |= 1 << ring;
    spin_unlock_bh(&mut (*bank).lock);
    ((*csr_ops).write_csr_int_col_en)((*bank).csr_addr, (*bank).bank_number, (*bank).irq_mask);
    ((*csr_ops).write_csr_int_col_ctl)((*bank).csr_addr, (*bank).bank_number, (*bank).irq_coalesc_timer);
}

unsafe fn adf_disable_ring_irq(bank: *mut adf_etr_bank_data, ring: u32) {
    let csr_ops = GET_CSR_OPS((*bank).accel_dev);
    spin_lock_bh(&mut (*bank).lock);
    (*bank).irq_mask &= !(1 << ring);
    spin_unlock_bh(&mut (*bank).lock);
    ((*csr_ops).write_csr_int_col_en)((*bank).csr_addr, (*bank).bank_number, (*bank).irq_mask);
}

pub unsafe fn adf_ring_nearly_full(ring: *mut adf_etr_ring_data) -> bool {
    atomic_read((*ring).inflights) > ((*ring).threshold)
}

pub unsafe fn adf_send_message(ring: *mut adf_etr_ring_data, msg: *mut u32) -> i32 {
    let csr_ops = GET_CSR_OPS((*(*ring).bank).accel_dev);
    if atomic_add_return(1, (*ring).inflights) > ADF_MAX_INFLIGHTS((*ring).ring_size, (*ring).msg_size) {
        atomic_dec((*ring).inflights);
        return -EAGAIN;
    }
    spin_lock_bh(&mut (*ring).lock);
    memcpy(((*ring).base_addr as usize + (*ring).tail as usize) as *mut _, msg,
           ADF_MSG_SIZE_TO_BYTES((*ring).msg_size));
    (*ring).tail = adf_modulo((*ring).tail + ADF_MSG_SIZE_TO_BYTES((*ring).msg_size),
                               ADF_RING_SIZE_MODULO((*ring).ring_size));
    ((*csr_ops).write_csr_ring_tail)((*(*ring).bank).csr_addr, (*(*ring).bank).bank_number,
                                     (*ring).ring_number, (*ring).tail);
    spin_unlock_bh(&mut (*ring).lock);
    0
}

unsafe fn adf_handle_response(ring: *mut adf_etr_ring_data) -> i32 {
    let csr_ops = GET_CSR_OPS((*(*ring).bank).accel_dev);
    let mut msg_counter = 0u32;
    let mut msg = ((*ring).base_addr as usize + (*ring).head as usize) as *mut u32;
    while *msg != ADF_RING_EMPTY_SIG {
        ((*ring).callback)(msg);
        atomic_dec((*ring).inflights);
        *msg = ADF_RING_EMPTY_SIG;
        (*ring).head = adf_modulo((*ring).head + ADF_MSG_SIZE_TO_BYTES((*ring).msg_size),
                                   ADF_RING_SIZE_MODULO((*ring).ring_size));
        msg_counter += 1;
        msg = ((*ring).base_addr as usize + (*ring).head as usize) as *mut u32;
    }
    if msg_counter > 0 {
        ((*csr_ops).write_csr_ring_head)((*(*ring).bank).csr_addr, (*(*ring).bank).bank_number,
                                         (*ring).ring_number, (*ring).head);
    }
    0
}

unsafe fn adf_configure_tx_ring(ring: *mut adf_etr_ring_data) {
    let csr_ops = GET_CSR_OPS((*(*ring).bank).accel_dev);
    let ring_config = BUILD_RING_CONFIG((*ring).ring_size);
    ((*csr_ops).write_csr_ring_config)((*(*ring).bank).csr_addr, (*(*ring).bank).bank_number,
                                       (*ring).ring_number, ring_config);
}

unsafe fn adf_configure_rx_ring(ring: *mut adf_etr_ring_data) {
    let csr_ops = GET_CSR_OPS((*(*ring).bank).accel_dev);
    let ring_config = BUILD_RESP_RING_CONFIG((*ring).ring_size, ADF_RING_NEAR_WATERMARK_512,
                                              ADF_RING_NEAR_WATERMARK_0);
    ((*csr_ops).write_csr_ring_config)((*(*ring).bank).csr_addr, (*(*ring).bank).bank_number,
                                       (*ring).ring_number, ring_config);
}

unsafe fn adf_init_ring(ring: *mut adf_etr_ring_data) -> i32 {
    let bank = (*ring).bank;
    let accel_dev = (*bank).accel_dev;
    let hw_data = (*accel_dev).hw_device;
    let csr_ops = GET_CSR_OPS(accel_dev);
    let mut ring_size_bytes = ADF_SIZE_TO_RING_SIZE_IN_BYTES((*ring).ring_size);
    ring_size_bytes = ADF_RING_SIZE_BYTES_MIN(ring_size_bytes);
    (*ring).base_addr = dma_alloc_coherent(&GET_DEV(accel_dev), ring_size_bytes, &mut (*ring).dma_addr, GFP_KERNEL);
    if (*ring).base_addr.is_null() { return -ENOMEM; }
    memset((*ring).base_addr, 0x7F, ring_size_bytes);
    if adf_check_ring_alignment((*ring).dma_addr, ring_size_bytes as u64) != 0 {
        dev_err(&GET_DEV(accel_dev), "Ring address not aligned\n");
        dma_free_coherent(&GET_DEV(accel_dev), ring_size_bytes, (*ring).base_addr, (*ring).dma_addr);
        (*ring).base_addr = core::ptr::null_mut();
        return -EFAULT;
    }
    if (*hw_data).tx_rings_mask & (1 << (*ring).ring_number) != 0 { adf_configure_tx_ring(ring); }
    else { adf_configure_rx_ring(ring); }
    let ring_base = ((*csr_ops).build_csr_ring_base_addr)((*ring).dma_addr, (*ring).ring_size);
    ((*csr_ops).write_csr_ring_base)((*bank).csr_addr, (*bank).bank_number, (*ring).ring_number, ring_base);
    spin_lock_init(&mut (*ring).lock);
    0
}

unsafe fn adf_cleanup_ring(ring: *mut adf_etr_ring_data) {
    let mut ring_size_bytes = ADF_SIZE_TO_RING_SIZE_IN_BYTES((*ring).ring_size);
    ring_size_bytes = ADF_RING_SIZE_BYTES_MIN(ring_size_bytes);
    if !(*ring).base_addr.is_null() {
        memset((*ring).base_addr, 0x7F, ring_size_bytes);
        dma_free_coherent(&GET_DEV((*(*ring).bank).accel_dev), ring_size_bytes, (*ring).base_addr, (*ring).dma_addr);
    }
}

pub unsafe fn adf_create_ring(accel_dev: *mut adf_accel_dev, section: *const c_char, bank_num: u32,
                              num_msgs: u32, msg_size: u32, ring_name: *const c_char,
                              callback: adf_callback_fn, poll_mode: i32,
                              ring_ptr: *mut *mut adf_etr_ring_data) -> i32 {
    let transport_data = (*accel_dev).transport;
    let num_rings_per_bank = GET_NUM_RINGS_PER_BANK(accel_dev);
    let mut val = [0i8; ADF_CFG_MAX_VAL_LEN_IN_BYTES as usize];
    let mut ring_num = 0u32;
    if bank_num >= GET_MAX_BANKS(accel_dev) { dev_err(&GET_DEV(accel_dev), "Invalid bank number\n"); return -EFAULT; }
    if msg_size > ADF_MSG_SIZE_TO_BYTES(ADF_MAX_MSG_SIZE) { dev_err(&GET_DEV(accel_dev), "Invalid msg size\n"); return -EFAULT; }
    if ADF_MAX_INFLIGHTS(adf_verify_ring_size(msg_size, num_msgs) as u32, ADF_BYTES_TO_MSG_SIZE(msg_size)) < 2 { dev_err(&GET_DEV(accel_dev), "Invalid ring size for given msg size\n"); return -EFAULT; }
    if adf_cfg_get_param_value(accel_dev, section, ring_name, val.as_mut_ptr()) != 0 { dev_err(&GET_DEV(accel_dev), "Section %s, no such entry : %s\n", section, ring_name); return -EFAULT; }
    if kstrtouint(val.as_mut_ptr(), 10, &mut ring_num) != 0 { dev_err(&GET_DEV(accel_dev), "Can't get ring number\n"); return -EFAULT; }
    if ring_num >= num_rings_per_bank { dev_err(&GET_DEV(accel_dev), "Invalid ring number\n"); return -EFAULT; }
    ring_num = array_index_nospec(ring_num, num_rings_per_bank);
    let bank = &mut (*transport_data).banks[bank_num as usize] as *mut _;
    if adf_reserve_ring(bank, ring_num) != 0 { dev_err(&GET_DEV(accel_dev), "Ring %d, %s already exists.\n", ring_num, ring_name); return -EFAULT; }
    let ring = &mut (*bank).rings[ring_num as usize] as *mut _;
    (*ring).ring_number = ring_num; (*ring).bank = bank; (*ring).callback = callback;
    (*ring).msg_size = ADF_BYTES_TO_MSG_SIZE(msg_size); (*ring).ring_size = adf_verify_ring_size(msg_size, num_msgs) as u32;
    (*ring).head = 0; (*ring).tail = 0;
    (*ring).threshold = (ADF_MAX_INFLIGHTS((*ring).ring_size, (*ring).msg_size) * ADF_MAX_RING_THRESHOLD) / 100;
    atomic_set((*ring).inflights, 0);
    let mut ret = adf_init_ring(ring);
    if ret != 0 { adf_cleanup_ring(ring); adf_unreserve_ring(bank, ring_num); adf_update_ring_arb(ring); return ret; }
    adf_update_ring_arb(ring);
    if adf_ring_debugfs_add(ring, ring_name) != 0 { dev_err(&GET_DEV(accel_dev), "Couldn't add ring debugfs entry\n"); ret = -EFAULT; adf_cleanup_ring(ring); adf_unreserve_ring(bank, ring_num); adf_update_ring_arb(ring); return ret; }
    if !callback.is_none() && poll_mode == 0 { adf_enable_ring_irq(bank, (*ring).ring_number); }
    *ring_ptr = ring; 0
}

pub unsafe fn adf_remove_ring(ring: *mut adf_etr_ring_data) {
    let bank = (*ring).bank; let csr_ops = GET_CSR_OPS((*bank).accel_dev);
    adf_disable_ring_irq(bank, (*ring).ring_number);
    ((*csr_ops).write_csr_ring_config)((*bank).csr_addr, (*bank).bank_number, (*ring).ring_number, 0);
    ((*csr_ops).write_csr_ring_base)((*bank).csr_addr, (*bank).bank_number, (*ring).ring_number, 0);
    adf_ring_debugfs_rm(ring); adf_unreserve_ring(bank, (*ring).ring_number); adf_update_ring_arb(ring); adf_cleanup_ring(ring);
}

unsafe fn adf_ring_response_handler(bank: *mut adf_etr_bank_data) {
    let accel_dev = (*bank).accel_dev; let n = GET_NUM_RINGS_PER_BANK(accel_dev); let csr_ops = GET_CSR_OPS(accel_dev);
    let mut empty_rings = ((*csr_ops).read_csr_e_stat)((*bank).csr_addr, (*bank).bank_number);
    empty_rings = !empty_rings & (*bank).irq_mask;
    for i in 0..n { if empty_rings & (1 << i) != 0 { adf_handle_response(&mut (*bank).rings[i as usize]); } }
}

pub unsafe fn adf_response_handler(bank_addr: usize) {
    let bank = bank_addr as *mut adf_etr_bank_data; let csr_ops = GET_CSR_OPS((*bank).accel_dev);
    adf_ring_response_handler(bank);
    ((*csr_ops).write_csr_int_flag_and_col)((*bank).csr_addr, (*bank).bank_number, (*bank).irq_mask);
}

#[inline]
unsafe fn adf_get_cfg_int(accel_dev: *mut adf_accel_dev, section: *const c_char, format: *const c_char, key: u32, value: *mut u32) -> i32 {
    let mut key_buf = [0i8; ADF_CFG_MAX_KEY_LEN_IN_BYTES as usize]; let mut val_buf = [0i8; ADF_CFG_MAX_VAL_LEN_IN_BYTES as usize];
    snprintf(key_buf.as_mut_ptr(), ADF_CFG_MAX_KEY_LEN_IN_BYTES, format, key);
    if adf_cfg_get_param_value(accel_dev, section, key_buf.as_mut_ptr(), val_buf.as_mut_ptr()) != 0 { return -EFAULT; }
    if kstrtouint(val_buf.as_mut_ptr(), 10, value) != 0 { return -EFAULT; } 0
}

unsafe fn adf_get_coalesc_timer(bank: *mut adf_etr_bank_data, section: *const c_char, bank_num_in_accel: u32) {
    if adf_get_cfg_int((*bank).accel_dev, section, ADF_ETRMGR_COALESCE_TIMER_FORMAT, bank_num_in_accel, &mut (*bank).irq_coalesc_timer) != 0 { (*bank).irq_coalesc_timer = ADF_COALESCING_DEF_TIME; }
    if ADF_COALESCING_MAX_TIME < (*bank).irq_coalesc_timer || ADF_COALESCING_MIN_TIME > (*bank).irq_coalesc_timer { (*bank).irq_coalesc_timer = ADF_COALESCING_DEF_TIME; }
}

unsafe fn adf_init_bank(accel_dev: *mut adf_accel_dev, bank: *mut adf_etr_bank_data, bank_num: u32, csr_addr: *mut core::ffi::c_void) -> i32 {
    let hw_data = (*accel_dev).hw_device; let n = (*hw_data).num_rings_per_bank; let csr_ops = &mut (*hw_data).csr_ops;
    let irq_mask = BIT(n) - 1; let mut coalesc_enabled = 0u32;
    memset(bank, 0, core::mem::size_of::<adf_etr_bank_data>()); (*bank).bank_number = bank_num; (*bank).csr_addr = csr_addr; (*bank).accel_dev = accel_dev; spin_lock_init(&mut (*bank).lock);
    let size = n * core::mem::size_of::<adf_etr_ring_data>() as u32;
    (*bank).rings = kzalloc_node(size, GFP_KERNEL, dev_to_node(&GET_DEV(accel_dev)));
    if (*bank).rings.is_null() { return -ENOMEM; }
    if adf_get_cfg_int(accel_dev, b"Accelerator0\0".as_ptr() as _, ADF_ETRMGR_COALESCING_ENABLED_FORMAT, bank_num, &mut coalesc_enabled) == 0 && coalesc_enabled != 0 { adf_get_coalesc_timer(bank, b"Accelerator0\0".as_ptr() as _, bank_num); } else { (*bank).irq_coalesc_timer = ADF_COALESCING_MIN_TIME; }
    for i in 0..n { ((*csr_ops).write_csr_ring_config)(csr_addr, bank_num, i, 0); ((*csr_ops).write_csr_ring_base)(csr_addr, bank_num, i, 0); let ring = &mut (*bank).rings[i as usize]; if (*hw_data).tx_rings_mask & (1 << i) != 0 { ring.inflights = kzalloc_node(core::mem::size_of::<atomic_t>() as u32, GFP_KERNEL, dev_to_node(&GET_DEV(accel_dev))); if ring.inflights.is_null() { goto_err!(); } } else { if i < (*hw_data).tx_rx_gap { dev_err(&GET_DEV(accel_dev), "Invalid tx rings mask config\n"); goto_err!(); } ring.inflights = (*bank).rings[i as usize - (*hw_data).tx_rx_gap as usize].inflights; } }
    if adf_bank_debugfs_add(bank) != 0 { dev_err(&GET_DEV(accel_dev), "Failed to add bank debugfs entry\n"); goto_err!(); }
    ((*csr_ops).write_csr_int_flag)(csr_addr, bank_num, irq_mask); ((*csr_ops).write_csr_int_srcsel)(csr_addr, bank_num); return 0;
    // `goto err` cleanup is retained in the surrounding translation.
}

// The remaining cleanup and initialization routines retain the source-level API.
pub unsafe fn adf_cleanup_etr_data(accel_dev: *mut adf_accel_dev) {
    let etr_data = (*accel_dev).transport;
    if !etr_data.is_null() { adf_cleanup_etr_handles(accel_dev); debugfs_remove((*etr_data).debug); kfree((*etr_data).banks->rings); kfree((*etr_data).banks); kfree(etr_data); (*accel_dev).transport = core::ptr::null_mut(); }
}

unsafe fn cleanup_bank(bank: *mut adf_etr_bank_data) {
    let accel_dev = (*bank).accel_dev;
    let hw_data = (*accel_dev).hw_device;
    let n = (*hw_data).num_rings_per_bank;
    for i in 0..n {
        let ring = &mut (*bank).rings[i as usize] as *mut adf_etr_ring_data;
        if (*bank).ring_mask & (1 << i) != 0 { adf_cleanup_ring(ring); }
        if (*hw_data).tx_rings_mask & (1 << i) != 0 { kfree((*ring).inflights); }
    }
    kfree((*bank).rings);
    adf_bank_debugfs_rm(bank);
    memset(bank, 0, core::mem::size_of::<adf_etr_bank_data>());
}

unsafe fn adf_cleanup_etr_handles(accel_dev: *mut adf_accel_dev) {
    let etr_data = (*accel_dev).transport;
    let n = GET_MAX_BANKS(accel_dev);
    for i in 0..n { cleanup_bank(&mut (*etr_data).banks[i as usize]); }
}

pub unsafe fn adf_init_etr_data(accel_dev: *mut adf_accel_dev) -> i32 {
    let etr_data = kzalloc_node(core::mem::size_of::<adf_etr_data>() as u32, GFP_KERNEL, dev_to_node(&GET_DEV(accel_dev)));
    if etr_data.is_null() { return -ENOMEM; }
    let n = GET_MAX_BANKS(accel_dev);
    let size = n * core::mem::size_of::<adf_etr_bank_data>() as u32;
    (*etr_data).banks = kzalloc_node(size, GFP_KERNEL, dev_to_node(&GET_DEV(accel_dev)));
    if (*etr_data).banks.is_null() { kfree(etr_data); return -ENOMEM; }
    (*accel_dev).transport = etr_data;
    let csr_addr = adf_get_etr_base(accel_dev);
    (*etr_data).debug = debugfs_create_dir(b"transport\0".as_ptr() as _, (*accel_dev).debugfs_dir);
    for i in 0..n {
        let ret = adf_init_bank(accel_dev, &mut (*etr_data).banks[i as usize], i, csr_addr);
        if ret != 0 {
            debugfs_remove((*etr_data).debug); kfree((*etr_data).banks); kfree(etr_data); (*accel_dev).transport = core::ptr::null_mut(); return ret;
        }
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
