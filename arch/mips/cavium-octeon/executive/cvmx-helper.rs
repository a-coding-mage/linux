/* Faithful low-level Rust translation of cvmx-helper.c. Included C headers are
 * external dependencies and are intentionally not reproduced here. */

static mut interface_port_count: [i32; 9] = [0; 9];

pub unsafe fn cvmx_helper_get_number_of_interfaces() -> i32 {
    if OCTEON_IS_MODEL(OCTEON_CN68XX) { return 9; }
    if OCTEON_IS_MODEL(OCTEON_CN66XX) {
        if OCTEON_IS_MODEL(OCTEON_CN66XX_PASS1_0) { return 7; } else { return 8; }
    }
    if OCTEON_IS_MODEL(OCTEON_CN56XX) || OCTEON_IS_MODEL(OCTEON_CN52XX) { return 4; }
    if OCTEON_IS_MODEL(OCTEON_CN7XXX) { 5 } else { 3 }
}

pub unsafe fn cvmx_helper_ports_on_interface(interface: i32) -> i32 {
    interface_port_count[interface as usize]
}

unsafe fn __cvmx_get_mode_cn68xx(interface: i32) -> cvmx_helper_interface_mode {
    let mut qlm_cfg: cvmx_mio_qlmx_cfg = core::mem::zeroed();
    match interface {
        0 | 2 | 3 | 4 => {
            qlm_cfg.u64 = cvmx_read_csr(CVMX_MIO_QLMX_CFG(if interface));
            if qlm_cfg.s.qlm_spd == 15 { return CVMX_HELPER_INTERFACE_MODE_DISABLED; }
            if qlm_cfg.s.qlm_cfg == 2 { CVMX_HELPER_INTERFACE_MODE_SGMII }
            else if qlm_cfg.s.qlm_cfg == 3 { CVMX_HELPER_INTERFACE_MODE_XAUI }
            else { CVMX_HELPER_INTERFACE_MODE_DISABLED }
        }
        7 => {
            qlm_cfg.u64 = cvmx_read_csr(CVMX_MIO_QLMX_CFG(3));
            if qlm_cfg.s.qlm_spd == 15 { CVMX_HELPER_INTERFACE_MODE_DISABLED }
            else { if qlm_cfg.s.qlm_cfg != 0 { qlm_cfg.u64 = cvmx_read_csr(CVMX_MIO_QLMX_CFG(1)); if qlm_cfg.s.qlm_cfg != 0 { return CVMX_HELPER_INTERFACE_MODE_DISABLED; } } CVMX_HELPER_INTERFACE_MODE_NPI }
        }
        8 => CVMX_HELPER_INTERFACE_MODE_LOOP,
        _ => CVMX_HELPER_INTERFACE_MODE_DISABLED,
    }
}

unsafe fn __cvmx_get_mode_octeon2(interface: i32) -> cvmx_helper_interface_mode {
    if OCTEON_IS_MODEL(OCTEON_CN68XX) { return __cvmx_get_mode_cn68xx(interface); }
    if interface == 2 { return CVMX_HELPER_INTERFACE_MODE_NPI; }
    if interface == 3 { return CVMX_HELPER_INTERFACE_MODE_LOOP; }
    if (OCTEON_IS_MODEL(OCTEON_CN63XX) && (interface == 4 || interface == 5)) || (OCTEON_IS_MODEL(OCTEON_CN66XX) && interface >= 4 && interface <= 7) { return CVMX_HELPER_INTERFACE_MODE_DISABLED; }
    if OCTEON_IS_MODEL(OCTEON_CN66XX) {
        let mut c: cvmx_mio_qlmx_cfg = core::mem::zeroed();
        if interface == 0 { c.u64 = cvmx_read_csr(CVMX_MIO_QLMX_CFG(2)); } else if interface == 1 { c.u64 = cvmx_read_csr(CVMX_MIO_QLMX_CFG(1)); } else { return CVMX_HELPER_INTERFACE_MODE_DISABLED; }
        if c.s.qlm_spd == 15 { return CVMX_HELPER_INTERFACE_MODE_DISABLED; }
        if c.s.qlm_cfg == 9 { return CVMX_HELPER_INTERFACE_MODE_SGMII; } else if c.s.qlm_cfg == 11 { return CVMX_HELPER_INTERFACE_MODE_XAUI; } else { return CVMX_HELPER_INTERFACE_MODE_DISABLED; }
    } else if OCTEON_IS_MODEL(OCTEON_CN61XX) {
        let mut c: cvmx_mio_qlmx_cfg = core::mem::zeroed();
        if interface == 0 { c.u64 = cvmx_read_csr(CVMX_MIO_QLMX_CFG(2)); } else if interface == 1 { c.u64 = cvmx_read_csr(CVMX_MIO_QLMX_CFG(0)); } else { return CVMX_HELPER_INTERFACE_MODE_DISABLED; }
        if c.s.qlm_cfg == 2 { return CVMX_HELPER_INTERFACE_MODE_SGMII; } else if c.s.qlm_cfg == 3 { return CVMX_HELPER_INTERFACE_MODE_XAUI; } else { return CVMX_HELPER_INTERFACE_MODE_DISABLED; }
    } else if OCTEON_IS_MODEL(OCTEON_CNF71XX) { if interface == 0 { let mut c: cvmx_mio_qlmx_cfg = core::mem::zeroed(); c.u64 = cvmx_read_csr(CVMX_MIO_QLMX_CFG(0)); if c.s.qlm_cfg == 2 { return CVMX_HELPER_INTERFACE_MODE_SGMII; } } return CVMX_HELPER_INTERFACE_MODE_DISABLED; }
    if interface == 1 && OCTEON_IS_MODEL(OCTEON_CN63XX) { return CVMX_HELPER_INTERFACE_MODE_DISABLED; }
    let mut mode: cvmx_gmxx_inf_mode = core::mem::zeroed(); mode.u64 = cvmx_read_csr(CVMX_GMXX_INF_MODE(interface));
    if OCTEON_IS_MODEL(OCTEON_CN63XX) { match mode.cn61xx.mode { 0 => CVMX_HELPER_INTERFACE_MODE_SGMII, 1 => CVMX_HELPER_INTERFACE_MODE_XAUI, _ => CVMX_HELPER_INTERFACE_MODE_DISABLED } }
    else if !mode.s.en { CVMX_HELPER_INTERFACE_MODE_DISABLED } else if mode.s.r#type { CVMX_HELPER_INTERFACE_MODE_GMII } else { CVMX_HELPER_INTERFACE_MODE_RGMII }
}

unsafe fn __cvmx_get_mode_cn7xxx(interface: i32) -> cvmx_helper_interface_mode {
    let mut mode: cvmx_gmxx_inf_mode = core::mem::zeroed(); mode.u64 = cvmx_read_csr(CVMX_GMXX_INF_MODE(interface));
    match interface { 0 | 1 => match mode.cn68xx.mode { 0 => CVMX_HELPER_INTERFACE_MODE_DISABLED, 1 | 2 => CVMX_HELPER_INTERFACE_MODE_SGMII, 3 => CVMX_HELPER_INTERFACE_MODE_XAUI, _ => CVMX_HELPER_INTERFACE_MODE_SGMII }, 2 => CVMX_HELPER_INTERFACE_MODE_NPI, 3 => CVMX_HELPER_INTERFACE_MODE_LOOP, 4 => CVMX_HELPER_INTERFACE_MODE_DISABLED, _ => CVMX_HELPER_INTERFACE_MODE_DISABLED }
}

pub unsafe fn cvmx_helper_interface_get_mode(interface: i32) -> cvmx_helper_interface_mode {
    if interface < 0 || interface >= cvmx_helper_get_number_of_interfaces() { return CVMX_HELPER_INTERFACE_MODE_DISABLED; }
    if OCTEON_IS_MODEL(OCTEON_CN7XXX) { return __cvmx_get_mode_cn7xxx(interface); }
    if OCTEON_IS_MODEL(OCTEON_CN6XXX) || OCTEON_IS_MODEL(OCTEON_CNF71XX) { return __cvmx_get_mode_octeon2(interface); }
    if interface == 2 { return CVMX_HELPER_INTERFACE_MODE_NPI; }
    if interface == 3 { return if OCTEON_IS_MODEL(OCTEON_CN56XX) || OCTEON_IS_MODEL(OCTEON_CN52XX) { CVMX_HELPER_INTERFACE_MODE_LOOP } else { CVMX_HELPER_INTERFACE_MODE_DISABLED }; }
    if interface == 1 && (OCTEON_IS_MODEL(OCTEON_CN31XX) || OCTEON_IS_MODEL(OCTEON_CN30XX) || OCTEON_IS_MODEL(OCTEON_CN50XX) || OCTEON_IS_MODEL(OCTEON_CN52XX)) { return CVMX_HELPER_INTERFACE_MODE_DISABLED; }
    let mut mode: cvmx_gmxx_inf_mode = core::mem::zeroed(); mode.u64 = cvmx_read_csr(CVMX_GMXX_INF_MODE(interface));
    if OCTEON_IS_MODEL(OCTEON_CN56XX) || OCTEON_IS_MODEL(OCTEON_CN52XX) { match mode.cn52xx.mode { 1 => CVMX_HELPER_INTERFACE_MODE_XAUI, 2 => CVMX_HELPER_INTERFACE_MODE_SGMII, 3 => CVMX_HELPER_INTERFACE_MODE_PICMG, _ => CVMX_HELPER_INTERFACE_MODE_DISABLED } }
    else if !mode.s.en { CVMX_HELPER_INTERFACE_MODE_DISABLED } else if mode.s.r#type { if OCTEON_IS_MODEL(OCTEON_CN38XX) || OCTEON_IS_MODEL(OCTEON_CN58XX) { CVMX_HELPER_INTERFACE_MODE_SPI } else { CVMX_HELPER_INTERFACE_MODE_GMII } } else { CVMX_HELPER_INTERFACE_MODE_RGMII }
}

pub unsafe fn cvmx_helper_interface_enumerate(interface: i32) -> i32 {
    let n = match cvmx_helper_interface_get_mode(interface) {
        CVMX_HELPER_INTERFACE_MODE_DISABLED | CVMX_HELPER_INTERFACE_MODE_PCIE => 0,
        CVMX_HELPER_INTERFACE_MODE_XAUI => __cvmx_helper_xaui_enumerate(interface),
        CVMX_HELPER_INTERFACE_MODE_RGMII | CVMX_HELPER_INTERFACE_MODE_GMII => __cvmx_helper_rgmii_enumerate(interface),
        CVMX_HELPER_INTERFACE_MODE_SPI => __cvmx_helper_spi_enumerate(interface),
        CVMX_HELPER_INTERFACE_MODE_SGMII | CVMX_HELPER_INTERFACE_MODE_PICMG => __cvmx_helper_sgmii_enumerate(interface),
        CVMX_HELPER_INTERFACE_MODE_NPI => __cvmx_helper_npi_enumerate(interface),
        CVMX_HELPER_INTERFACE_MODE_LOOP => __cvmx_helper_loop_enumerate(interface),
    };
    interface_port_count[interface as usize] = __cvmx_helper_board_interface_probe(interface, n);
    CVMX_SYNCWS; 0
}

pub unsafe fn cvmx_helper_interface_probe(interface: i32) -> i32 {
    cvmx_helper_interface_enumerate(interface);
    match cvmx_helper_interface_get_mode(interface) {
        CVMX_HELPER_INTERFACE_MODE_XAUI => __cvmx_helper_xaui_probe(interface),
        CVMX_HELPER_INTERFACE_MODE_RGMII | CVMX_HELPER_INTERFACE_MODE_GMII => __cvmx_helper_rgmii_probe(interface),
        CVMX_HELPER_INTERFACE_MODE_SPI => __cvmx_helper_spi_probe(interface),
        CVMX_HELPER_INTERFACE_MODE_SGMII | CVMX_HELPER_INTERFACE_MODE_PICMG => __cvmx_helper_sgmii_probe(interface),
        CVMX_HELPER_INTERFACE_MODE_NPI => __cvmx_helper_npi_probe(interface),
        CVMX_HELPER_INTERFACE_MODE_LOOP => __cvmx_helper_loop_probe(interface), _ => {}
    }; CVMX_SYNCWS; 0
}

unsafe fn __cvmx_helper_port_setup_ipd(ipd_port:i32)->i32 { let mut p:cvmx_pip_prt_cfgx=core::mem::zeroed();let mut t:cvmx_pip_prt_tagx=core::mem::zeroed();p.u64=cvmx_read_csr(CVMX_PIP_PRT_CFGX(ipd_port));t.u64=cvmx_read_csr(CVMX_PIP_PRT_TAGX(ipd_port));p.s.qos=ipd_port&7;p.s.mode=CVMX_HELPER_INPUT_PORT_SKIP_MODE;t.s.ip6_src_flag=CVMX_HELPER_INPUT_TAG_IPV6_SRC_IP;t.s.ip6_dst_flag=CVMX_HELPER_INPUT_TAG_IPV6_DST_IP;t.s.ip6_sprt_flag=CVMX_HELPER_INPUT_TAG_IPV6_SRC_PORT;t.s.ip6_dprt_flag=CVMX_HELPER_INPUT_TAG_IPV6_DST_PORT;t.s.ip6_nxth_flag=CVMX_HELPER_INPUT_TAG_IPV6_NEXT_HEADER;t.s.ip4_src_flag=CVMX_HELPER_INPUT_TAG_IPV4_SRC_IP;t.s.ip4_dst_flag=CVMX_HELPER_INPUT_TAG_IPV4_DST_IP;t.s.ip4_sprt_flag=CVMX_HELPER_INPUT_TAG_IPV4_SRC_PORT;t.s.ip4_dprt_flag=CVMX_HELPER_INPUT_TAG_IPV4_DST_PORT;t.s.ip4_pctl_flag=CVMX_HELPER_INPUT_TAG_IPV4_PROTOCOL;t.s.inc_prt_flag=CVMX_HELPER_INPUT_TAG_INPUT_PORT;t.s.tcp6_tag_type=CVMX_HELPER_INPUT_TAG_TYPE;t.s.tcp4_tag_type=CVMX_HELPER_INPUT_TAG_TYPE;t.s.ip6_tag_type=CVMX_HELPER_INPUT_TAG_TYPE;t.s.ip4_tag_type=CVMX_HELPER_INPUT_TAG_TYPE;t.s.non_tag_type=CVMX_HELPER_INPUT_TAG_TYPE;t.s.grp=0;cvmx_pip_config_port(ipd_port,p,t);0 }

unsafe fn __cvmx_helper_interface_setup_ipd(interface: i32) -> i32 { let mut p=cvmx_helper_get_ipd_port(interface,0); let mut n=interface_port_count[interface as usize]; while n>0 { __cvmx_helper_port_setup_ipd(p); p+=1; n-=1; } 0 }
unsafe fn __cvmx_helper_global_setup_ipd() -> i32 { cvmx_ipd_config(CVMX_FPA_PACKET_POOL_SIZE/8,CVMX_HELPER_FIRST_MBUFF_SKIP/8,CVMX_HELPER_NOT_FIRST_MBUFF_SKIP/8,(CVMX_HELPER_FIRST_MBUFF_SKIP+8)/128,(CVMX_HELPER_NOT_FIRST_MBUFF_SKIP+8)/128,CVMX_FPA_WQE_POOL,CVMX_IPD_OPC_MODE_STT,CVMX_HELPER_ENABLE_BACK_PRESSURE); 0 }
unsafe fn __cvmx_helper_interface_setup_pko(interface:i32)->i32 { let pr:[u64;16]=[8,7,6,5,4,3,2,1,8,7,6,5,4,3,2,1]; let mut p=cvmx_helper_get_ipd_port(interface,0); let mut n=interface_port_count[interface as usize]; while n>0 { cvmx_pko_config_port(p,cvmx_pko_get_base_queue_per_core(p,0),cvmx_pko_get_num_queues(p),pr.as_ptr()); p+=1;n-=1;} 0 }
unsafe fn __cvmx_helper_global_setup_pko()->i32 { let mut f:cvmx_iob_fau_timeout=core::mem::zeroed();f.u64=0;f.s.tout_val=0xfff;f.s.tout_enb=0;cvmx_write_csr(CVMX_IOB_FAU_TIMEOUT,f.u64);if OCTEON_IS_MODEL(OCTEON_CN68XX){let mut p:cvmx_pko_reg_min_pkt=core::mem::zeroed();p.u64=0;p.s.size1=59;p.s.size2=59;p.s.size3=59;p.s.size4=59;p.s.size5=59;p.s.size6=59;p.s.size7=59;cvmx_write_csr(CVMX_PKO_REG_MIN_PKT,p.u64);}0 }
unsafe fn __cvmx_helper_global_setup_backpressure()->i32 { 0 }
unsafe fn __cvmx_helper_packet_hardware_enable(interface:i32)->i32 { match cvmx_helper_interface_get_mode(interface){CVMX_HELPER_INTERFACE_MODE_XAUI=>__cvmx_helper_xaui_enable(interface),CVMX_HELPER_INTERFACE_MODE_RGMII|CVMX_HELPER_INTERFACE_MODE_GMII=>__cvmx_helper_rgmii_enable(interface),CVMX_HELPER_INTERFACE_MODE_SPI=>__cvmx_helper_spi_enable(interface),CVMX_HELPER_INTERFACE_MODE_SGMII|CVMX_HELPER_INTERFACE_MODE_PICMG=>__cvmx_helper_sgmii_enable(interface),CVMX_HELPER_INTERFACE_MODE_NPI=>__cvmx_helper_npi_enable(interface),CVMX_HELPER_INTERFACE_MODE_LOOP=>__cvmx_helper_loop_enable(interface),_=>0} }

pub unsafe fn cvmx_helper_ipd_and_packet_input_enable()->i32 { cvmx_ipd_enable();let n=cvmx_helper_get_number_of_interfaces();for i in 0..n{if cvmx_helper_ports_on_interface(i)>0{__cvmx_helper_packet_hardware_enable(i);}}cvmx_pko_enable();0 }
pub unsafe fn cvmx_helper_initialize_packet_io_global()->i32 { let mut r=0;let n=cvmx_helper_get_number_of_interfaces();let mut l:cvmx_l2c_cfg=core::mem::zeroed();l.u64=cvmx_read_csr(CVMX_L2C_CFG);l.s.lrf_arb_mode=0;l.s.rfb_arb_mode=0;cvmx_write_csr(CVMX_L2C_CFG,l.u64);cvmx_pko_initialize_global();for i in 0..n{r|=cvmx_helper_interface_probe(i);r|=__cvmx_helper_interface_setup_ipd(i);r|=__cvmx_helper_interface_setup_pko(i);}r|=__cvmx_helper_global_setup_ipd();r|=__cvmx_helper_global_setup_pko();r|=__cvmx_helper_global_setup_backpressure();if CVMX_HELPER_ENABLE_IPD!=0{r|=cvmx_helper_ipd_and_packet_input_enable();}r}

pub unsafe fn cvmx_helper_link_get(ipd_port:i32)->cvmx_helper_link_info{let mut r:cvmx_helper_link_info=core::mem::zeroed();let intf=cvmx_helper_get_interface_num(ipd_port);let idx=cvmx_helper_get_interface_index_num(ipd_port);if idx>=cvmx_helper_ports_on_interface(intf){return r;}match cvmx_helper_interface_get_mode(intf){CVMX_HELPER_INTERFACE_MODE_XAUI=>r=__cvmx_helper_xaui_link_get(ipd_port),CVMX_HELPER_INTERFACE_MODE_GMII=>if idx==0{r=__cvmx_helper_rgmii_link_get(ipd_port)}else{r.s.full_duplex=1;r.s.link_up=1;r.s.speed=1000},CVMX_HELPER_INTERFACE_MODE_RGMII=>r=__cvmx_helper_rgmii_link_get(ipd_port),CVMX_HELPER_INTERFACE_MODE_SPI=>r=__cvmx_helper_spi_link_get(ipd_port),CVMX_HELPER_INTERFACE_MODE_SGMII|CVMX_HELPER_INTERFACE_MODE_PICMG=>r=__cvmx_helper_sgmii_link_get(ipd_port),_=>{}}r}
pub unsafe fn cvmx_helper_link_set(ipd_port:i32,link_info:cvmx_helper_link_info)->i32{let mut r=-1;let i=cvmx_helper_get_interface_num(ipd_port);if cvmx_helper_get_interface_index_num(ipd_port)>=cvmx_helper_ports_on_interface(i){return r;}match cvmx_helper_interface_get_mode(i){CVMX_HELPER_INTERFACE_MODE_XAUI=>r=__cvmx_helper_xaui_link_set(ipd_port,link_info),CVMX_HELPER_INTERFACE_MODE_RGMII|CVMX_HELPER_INTERFACE_MODE_GMII=>r=__cvmx_helper_rgmii_link_set(ipd_port,link_info),CVMX_HELPER_INTERFACE_MODE_SPI=>r=__cvmx_helper_spi_link_set(ipd_port,link_info),CVMX_HELPER_INTERFACE_MODE_SGMII|CVMX_HELPER_INTERFACE_MODE_PICMG=>r=__cvmx_helper_sgmii_link_set(ipd_port,link_info),_=>{}}r}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
