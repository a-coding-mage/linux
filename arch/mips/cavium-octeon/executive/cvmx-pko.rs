/* Rust translation of cvmx-pko.c.  Declarations supplied by the SDK remain
 * external dependencies. */

unsafe fn __cvmx_pko_int(interface: i32, index: i32) -> i32 {
    match interface { 0 => index, 1 => 4, 2 => index + 0x08, 3 => index + 0x0c,
        4 => index + 0x10, 5 => 0x1c, 6 => 0x1d, 7 => 0x1e, 8 => 0x1f, _ => -1 }
}

unsafe fn __cvmx_pko_iport_config(pko_port: i32) {
    let num_queues = 1; let base_queue = pko_port;
    let static_priority_end = 1; let static_priority_base = 1;
    for queue in 0..num_queues {
        let mut config: cvmx_pko_mem_iqueue_ptrs = core::mem::zeroed();
        config.s.index = queue; config.s.qid = base_queue + queue; config.s.ipid = pko_port;
        config.s.tail = queue == num_queues - 1; config.s.s_tail = queue == static_priority_end;
        config.s.static_p = static_priority_base >= 0; config.s.static_q = queue <= static_priority_end;
        config.s.qos_mask = 0xff;
        let cmd_res = cvmx_cmd_queue_initialize(CVMX_CMD_QUEUE_PKO(base_queue + queue), CVMX_PKO_MAX_QUEUE_DEPTH,
            CVMX_FPA_OUTPUT_BUFFER_POOL, CVMX_FPA_OUTPUT_BUFFER_POOL_SIZE - CVMX_PKO_COMMAND_BUFFER_SIZE_ADJUST * 8);
        WARN(cmd_res, "{}: cmd_res={} pko_port={} base_queue={} num_queues={} queue={}\n", "__cvmx_pko_iport_config", cmd_res as i32, pko_port, base_queue, num_queues, queue);
        let buf_ptr = cvmx_cmd_queue_buffer(CVMX_CMD_QUEUE_PKO(base_queue + queue));
        config.s.buf_ptr = cvmx_ptr_to_phys(buf_ptr) >> 7; CVMX_SYNCWS;
        cvmx_write_csr(CVMX_PKO_MEM_IQUEUE_PTRS, config.u64);
    }
}

unsafe fn __cvmx_pko_queue_alloc_o68() { for port in 0..48 { __cvmx_pko_iport_config(port); } }

unsafe fn __cvmx_pko_port_map_o68() {
    let mut config: cvmx_pko_mem_iport_ptrs = core::mem::zeroed(); config.u64 = 0; config.s.eid = 31;
    for port in 0..128 { config.s.ipid = port; cvmx_write_csr(CVMX_PKO_MEM_IPORT_PTRS, config.u64); }
    for port in 0..48 {
        let interface = cvmx_helper_get_interface_num(port); let index = cvmx_helper_get_interface_index_num(port);
        let mode = cvmx_helper_interface_get_mode(interface); if mode == CVMX_HELPER_INTERFACE_MODE_DISABLED { continue; }
        config.s.ipid = port; config.s.qos_mask = 0xff; config.s.crc = 1; config.s.min_pkt = 1;
        config.s.intr = __cvmx_pko_int(interface, index); config.s.eid = config.s.intr;
        config.s.pipe = if mode == CVMX_HELPER_INTERFACE_MODE_LOOP { index } else { port };
        cvmx_write_csr(CVMX_PKO_MEM_IPORT_PTRS, config.u64);
    }
}

unsafe fn __cvmx_pko_chip_init() {
    if OCTEON_IS_MODEL(OCTEON_CN68XX) { __cvmx_pko_port_map_o68(); __cvmx_pko_queue_alloc_o68(); return; }
    for i in 0..CVMX_PKO_MAX_OUTPUT_QUEUES { let priority = 8u64; cvmx_pko_config_port(CVMX_PKO_MEM_QUEUE_PTRS_ILLEGAL_PID, i, 1, &priority); }
}

pub unsafe fn cvmx_pko_initialize_global() {
    let mut config: cvmx_pko_reg_cmd_buf = core::mem::zeroed(); config.u64 = 0;
    config.s.pool = CVMX_FPA_OUTPUT_BUFFER_POOL; config.s.size = CVMX_FPA_OUTPUT_BUFFER_POOL_SIZE / 8 - 1;
    cvmx_write_csr(CVMX_PKO_REG_CMD_BUF, config.u64); __cvmx_pko_chip_init();
    if OCTEON_IS_MODEL(OCTEON_CN38XX) || OCTEON_IS_MODEL(OCTEON_CN58XX) || OCTEON_IS_MODEL(OCTEON_CN56XX) || OCTEON_IS_MODEL(OCTEON_CN52XX) {
        let n = cvmx_helper_get_number_of_interfaces(); let last = cvmx_helper_get_last_ipd_port(n - 1);
        let max = cvmx_pko_get_base_queue(last) + cvmx_pko_get_num_queues(last);
        if OCTEON_IS_MODEL(OCTEON_CN38XX) { if max <= 32 { cvmx_write_csr(CVMX_PKO_REG_QUEUE_MODE, 2); } else if max <= 64 { cvmx_write_csr(CVMX_PKO_REG_QUEUE_MODE, 1); } }
        else if max <= 64 { cvmx_write_csr(CVMX_PKO_REG_QUEUE_MODE, 2); } else if max <= 128 { cvmx_write_csr(CVMX_PKO_REG_QUEUE_MODE, 1); }
    }
}

pub unsafe fn cvmx_pko_enable() { let mut flags: cvmx_pko_reg_flags = core::mem::zeroed(); flags.u64 = cvmx_read_csr(CVMX_PKO_REG_FLAGS); if flags.s.ena_pko { cvmx_dprintf("Warning: Enabling PKO when PKO already enabled.\n"); } flags.s.ena_dwb = 1; flags.s.ena_pko = 1; flags.s.store_be = 1; cvmx_write_csr(CVMX_PKO_REG_FLAGS, flags.u64); }
pub unsafe fn cvmx_pko_disable() { let mut f: cvmx_pko_reg_flags = core::mem::zeroed(); f.u64 = cvmx_read_csr(CVMX_PKO_REG_FLAGS); f.s.ena_pko = 0; cvmx_write_csr(CVMX_PKO_REG_FLAGS, f.u64); }
unsafe fn __cvmx_pko_reset() { let mut f: cvmx_pko_reg_flags = core::mem::zeroed(); f.u64 = cvmx_read_csr(CVMX_PKO_REG_FLAGS); f.s.reset = 1; cvmx_write_csr(CVMX_PKO_REG_FLAGS, f.u64); }

pub unsafe fn cvmx_pko_shutdown() {
    cvmx_pko_disable();
    for queue in 0..CVMX_PKO_MAX_OUTPUT_QUEUES { let mut c: cvmx_pko_mem_queue_ptrs = core::mem::zeroed(); c.u64=0; c.s.tail=1; c.s.port=CVMX_PKO_MEM_QUEUE_PTRS_ILLEGAL_PID; c.s.queue=queue & 0x7f; c.s.qos_mask=0; c.s.buf_ptr=0;
        if !OCTEON_IS_MODEL(OCTEON_CN3XXX) { let mut c1: cvmx_pko_reg_queue_ptrs1=core::mem::zeroed(); c1.u64=0; c1.s.qid7=queue>>7; cvmx_write_csr(CVMX_PKO_REG_QUEUE_PTRS1,c1.u64); }
        cvmx_write_csr(CVMX_PKO_MEM_QUEUE_PTRS,c.u64); cvmx_cmd_queue_shutdown(CVMX_CMD_QUEUE_PKO(queue)); }
    __cvmx_pko_reset();
}

pub unsafe fn cvmx_pko_config_port(port:u64, base_queue:u64, num_queues:u64, priority:*const u64)->cvmx_pko_status {
    if OCTEON_IS_MODEL(OCTEON_CN68XX) { return CVMX_PKO_SUCCESS; }
    if (port >= CVMX_PKO_NUM_OUTPUT_PORTS) && port != CVMX_PKO_MEM_QUEUE_PTRS_ILLEGAL_PID { return CVMX_PKO_INVALID_PORT; }
    if base_queue + num_queues > CVMX_PKO_MAX_OUTPUT_QUEUES { return CVMX_PKO_INVALID_QUEUE; }
    let mut spb:i32=-1; let mut spe:i32=-1;
    if port != CVMX_PKO_MEM_QUEUE_PTRS_ILLEGAL_PID { for q in 0..num_queues { let p=*priority.add(q as usize); if spb==-1 && p==CVMX_PKO_QUEUE_STATIC_PRIORITY {spb=q as i32;} if spb!=-1 && spe==-1 && p!=CVMX_PKO_QUEUE_STATIC_PRIORITY && q!=0 {spe=q as i32-1;} else if spb!=-1&&spe==-1&&q==num_queues-1 {spe=q as i32;} if spe!=-1&&(q as i32)>spe&&p==CVMX_PKO_QUEUE_STATIC_PRIORITY{return CVMX_PKO_INVALID_PRIORITY;} } if spb>0{return CVMX_PKO_INVALID_PRIORITY;} }
    let mut result=CVMX_PKO_SUCCESS;
    for q in 0..num_queues { let mut c:cvmx_pko_mem_queue_ptrs=core::mem::zeroed(); let mut c1:cvmx_pko_reg_queue_ptrs1=core::mem::zeroed(); c1.u64=0;c1.s.idx3=q>>3;c1.s.qid7=(base_queue+q)>>7;c.u64=0;c.s.tail=q==num_queues-1;c.s.index=q;c.s.port=port;c.s.queue=base_queue+q;
        if !cvmx_octeon_is_pass1(){c.s.static_p=spb>=0;c.s.static_q=q as i32<=spe;c.s.s_tail=q as i32==spe;}
        c.s.qos_mask=match *priority.add(q as usize){0=>0,1=>1,2=>0x11,3=>0x49,4=>0x55,5=>0x57,6=>0x77,7=>0x7f,8=>0xff,CVMX_PKO_QUEUE_STATIC_PRIORITY=>{if !cvmx_octeon_is_pass1(){0xff}else{result=CVMX_PKO_INVALID_PRIORITY;0xff}}, _=>{result=CVMX_PKO_INVALID_PRIORITY;0xff}};
        if port!=CVMX_PKO_MEM_QUEUE_PTRS_ILLEGAL_PID { let r=cvmx_cmd_queue_initialize(CVMX_CMD_QUEUE_PKO(base_queue+q),CVMX_PKO_MAX_QUEUE_DEPTH,CVMX_FPA_OUTPUT_BUFFER_POOL,CVMX_FPA_OUTPUT_BUFFER_POOL_SIZE-CVMX_PKO_COMMAND_BUFFER_SIZE_ADJUST*8); if r!=CVMX_CMD_QUEUE_SUCCESS{return CVMX_PKO_CMD_QUEUE_INIT_ERROR;} c.s.buf_ptr=cvmx_ptr_to_phys(cvmx_cmd_queue_buffer(CVMX_CMD_QUEUE_PKO(base_queue+q))); }
        CVMX_SYNCWS;if !OCTEON_IS_MODEL(OCTEON_CN3XXX){cvmx_write_csr(CVMX_PKO_REG_QUEUE_PTRS1,c1.u64);}cvmx_write_csr(CVMX_PKO_MEM_QUEUE_PTRS,c.u64);
    } result
}

pub unsafe fn cvmx_pko_rate_limit_packets(port:i32,packets_s:i32,burst:i32)->i32 { let mut r0:cvmx_pko_mem_port_rate0=core::mem::zeroed();let mut r1:cvmx_pko_mem_port_rate1=core::mem::zeroed();r0.u64=0;r0.s.pid=port;r0.s.rate_pkt=cvmx_sysinfo_get().cpu_clock_hz/packets_s/16;r0.s.rate_word=0;r1.u64=0;r1.s.pid=port;r1.s.rate_lim=((r0.s.rate_pkt as u64*burst as u64)>>8);cvmx_write_csr(CVMX_PKO_MEM_PORT_RATE0,r0.u64);cvmx_write_csr(CVMX_PKO_MEM_PORT_RATE1,r1.u64);0 }
pub unsafe fn cvmx_pko_rate_limit_bits(port:i32,bits_s:u64,burst:i32)->i32 { let mut r0:cvmx_pko_mem_port_rate0=core::mem::zeroed();let mut r1:cvmx_pko_mem_port_rate1=core::mem::zeroed();let t=cvmx_sysinfo_get().cpu_clock_hz*16/bits_s;r0.u64=0;r0.s.pid=port;r0.s.rate_pkt=24*8*t/256;r0.s.rate_word=64*t;r1.u64=0;r1.s.pid=port;r1.s.rate_lim=t*burst as u64/256;cvmx_write_csr(CVMX_PKO_MEM_PORT_RATE0,r0.u64);cvmx_write_csr(CVMX_PKO_MEM_PORT_RATE1,r1.u64);0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
