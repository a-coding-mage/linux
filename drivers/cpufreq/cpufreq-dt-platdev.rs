// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2016 Linaro.
 * Viresh Kumar <viresh.kumar@linaro.org>
 */

// Kernel dependencies supplied by other translation units.

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const ::core::ffi::c_char,
    pub data: *const ::core::ffi::c_void,
}

#[repr(C)]
pub struct cpufreq_dt_platform_data {
    pub have_governor_per_policy: bool,
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

extern "C" {
    fn of_cpu_device_node_get(cpu: i32) -> *mut device_node;
    fn of_property_present(np: *const device_node, propname: *const ::core::ffi::c_char) -> bool;
    fn of_machine_device_match(matches: *const of_device_id) -> bool;
    fn of_machine_get_match_data(matches: *const of_device_id) -> *const ::core::ffi::c_void;
    fn platform_device_register_data(
        parent: *mut ::core::ffi::c_void,
        name: *const ::core::ffi::c_char,
        id: i32,
        data: *const ::core::ffi::c_void,
        size: usize,
    ) -> *mut ::core::ffi::c_void;
    fn PTR_ERR_OR_ZERO(ptr: *mut ::core::ffi::c_void) -> i32;
}

const ENODEV: i32 = 19;

static RK3399_PLATFORM_DATA: cpufreq_dt_platform_data = cpufreq_dt_platform_data {
    have_governor_per_policy: true,
};

/*
 * Machines for which the cpufreq device is *always* created, mostly used for
 * platforms using "operating-points" (V1) property.
 */
static ALLOWLIST: &[of_device_id] = &[
    of_device_id { compatible: b"allwinner,sun4i-a10\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"allwinner,sun5i-a10s\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"allwinner,sun5i-a13\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"allwinner,sun5i-r8\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"allwinner,sun6i-a31\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"allwinner,sun6i-a31s\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"allwinner,sun7i-a20\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"allwinner,sun8i-a23\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"allwinner,sun8i-a83t\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"allwinner,sun8i-h3\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"apm,xgene-shadowcat\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"arm,integrator-ap\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"arm,integrator-cp\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"hisilicon,hi3660\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"fsl,imx27\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"fsl,imx51\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"fsl,imx53\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"marvell,berlin\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"marvell,pxa250\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"marvell,pxa270\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"samsung,exynos3250\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"samsung,exynos4210\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"samsung,exynos5250\0".as_ptr() as _, data: core::ptr::null() },
    // CONFIG_BL_SWITCHER-disabled entry in the C source is omitted when enabled.
    of_device_id { compatible: b"samsung,exynos5800\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"renesas,emev2\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"renesas,r7s72100\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"renesas,r8a73a4\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"renesas,r8a7740\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"renesas,r8a7742\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"renesas,r8a7743\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"renesas,r8a7744\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"renesas,r8a7745\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"renesas,r8a7778\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"renesas,r8a7779\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"renesas,r8a7790\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"renesas,r8a7791\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"renesas,r8a7792\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"renesas,r8a7793\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"renesas,r8a7794\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"renesas,sh73a0\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"rockchip,rk2928\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"rockchip,rk3036\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"rockchip,rk3066a\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"rockchip,rk3066b\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"rockchip,rk3188\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"rockchip,rk3228\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"rockchip,rk3288\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"rockchip,rk3328\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"rockchip,rk3366\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"rockchip,rk3368\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"rockchip,rk3399\0".as_ptr() as _, data: &RK3399_PLATFORM_DATA as *const _ as _ },
    of_device_id { compatible: b"spacemit,k1\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"st-ericsson,u8500\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"st-ericsson,u8540\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"st-ericsson,u9500\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"st-ericsson,u9540\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"starfive,jh7110\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"starfive,jh7110s\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"ti,omap2\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"ti,omap4\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"ti,omap5\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"xlnx,zynq-7000\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"xlnx,zynqmp\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: core::ptr::null(), data: core::ptr::null() },
];

/*
 * Machines for which the cpufreq device is *not* created, mostly used for
 * platforms using "operating-points-v2" property.
 */
static blocklist: &[of_device_id] = &[
    of_device_id { compatible: b"airoha,an7583\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"airoha,en7581\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"allwinner,sun50i-a100\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"allwinner,sun50i-h6\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"allwinner,sun50i-h616\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"allwinner,sun50i-h618\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"allwinner,sun50i-h700\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"apple,arm-platform\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"arm,vexpress\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"calxeda,highbank\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"calxeda,ecx-2000\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"fsl,imx7ulp\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"fsl,imx7d\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"fsl,imx7s\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"fsl,imx8mq\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"fsl,imx8mm\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"fsl,imx8mn\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"fsl,imx8mp\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"marvell,armadaxp\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"mediatek,mt2701\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"mediatek,mt2712\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"mediatek,mt7622\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"mediatek,mt7623\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"mediatek,mt8167\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"mediatek,mt817x\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"mediatek,mt8173\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"mediatek,mt8176\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"mediatek,mt8183\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"mediatek,mt8186\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"mediatek,mt8365\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"mediatek,mt8516\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"nvidia,tegra20\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"nvidia,tegra30\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"nvidia,tegra114\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"nvidia,tegra124\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"nvidia,tegra186\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"nvidia,tegra194\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"nvidia,tegra210\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"nvidia,tegra234\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"qcom,apq8096\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"qcom,msm8909\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"qcom,msm8996\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"qcom,msm8998\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"qcom,qcm2290\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"qcom,qcm6490\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"qcom,qcs404\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"qcom,qcs8300\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"qcom,qdu1000\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"qcom,sa8155p\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"qcom,sa8540p\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"qcom,sa8775p\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"qcom,sc7180\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"qcom,sc7280\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"qcom,sc8180x\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"qcom,sc8280xp\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"qcom,sdm670\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"qcom,sdm845\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"qcom,sdx75\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"qcom,sm6115\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"qcom,sm6125\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"qcom,sm6150\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"qcom,sm6350\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"qcom,sm6375\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"qcom,sm7125\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"qcom,sm7225\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"qcom,sm7325\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"qcom,sm8150\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"qcom,sm8250\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"qcom,sm8350\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"qcom,sm8450\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"qcom,sm8550\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"qcom,sm8650\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"st,stih407\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"st,stih410\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"st,stih418\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"ti,am33xx\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"ti,am43\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"ti,dra7\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"ti,omap3\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"ti,am625\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"ti,am62a7\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"ti,am62d2\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"ti,am62l3\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"ti,am62p5\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"qcom,ipq5210\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"qcom,ipq5332\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"qcom,ipq5424\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"qcom,ipq6018\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"qcom,ipq8064\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"qcom,ipq8074\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"qcom,ipq9574\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"qcom,apq8064\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"qcom,msm8974\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: b"qcom,msm8960\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: core::ptr::null(), data: core::ptr::null() },
];

unsafe fn cpu0_node_has_opp_v2_prop() -> bool {
    let np = of_cpu_device_node_get(0);
    of_property_present(np, b"operating-points-v2\0".as_ptr() as _)
}

unsafe fn cpufreq_dt_platdev_init() -> i32 {
    let mut data: *const ::core::ffi::c_void = core::ptr::null();

    if of_machine_device_match(ALLOWLIST.as_ptr()) {
        data = of_machine_get_match_data(ALLOWLIST.as_ptr());
        return PTR_ERR_OR_ZERO(platform_device_register_data(
            core::ptr::null_mut(), b"cpufreq-dt\0".as_ptr() as _, -1, data,
            core::mem::size_of::<cpufreq_dt_platform_data>(),
        ));
    }

    if cpu0_node_has_opp_v2_prop() && !of_machine_device_match(blocklist.as_ptr()) {
        return PTR_ERR_OR_ZERO(platform_device_register_data(
            core::ptr::null_mut(), b"cpufreq-dt\0".as_ptr() as _, -1, data,
            core::mem::size_of::<cpufreq_dt_platform_data>(),
        ));
    }

    -ENODEV
}

// core_initcall(cpufreq_dt_platdev_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
