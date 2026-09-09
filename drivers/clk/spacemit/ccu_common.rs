// SPDX-License-Identifier: GPL-2.0-only

// Dependencies supplied by the surrounding kernel/Rust translation.

static mut auxiliary_ids: ida = DEFINE_IDA!();

unsafe fn spacemit_ccu_register(
    dev: *mut device,
    regmap: *mut regmap,
    lock_regmap: *mut regmap,
    data: *const spacemit_ccu_data,
) -> i32 {
    let mut clk_data: *mut clk_hw_onecell_data;
    let mut ret: i32;

    /* Nothing to do if the CCU does not implement any clocks */
    if (*data).hws.is_null() {
        return 0;
    }

    clk_data = devm_kzalloc(dev, struct_size!(clk_data, hws, (*data).num), GFP_KERNEL);
    if clk_data.is_null() {
        return -ENOMEM;
    }

    (*clk_data).num = (*data).num;

    for i in 0..(*data).num {
        let hw: *mut clk_hw = *(*data).hws.add(i as usize);
        let common: *mut ccu_common;
        let name: *const c_char;

        if hw.is_null() {
            *(*clk_data).hws.add(i as usize) = ERR_PTR(-ENOENT);
            continue;
        }

        name = (*(*hw).init).name;

        common = hw_to_ccu_common(hw);
        (*common).regmap = regmap;
        (*common).lock_regmap = lock_regmap;

        ret = devm_clk_hw_register(dev, hw);
        if ret != 0 {
            dev_err!(dev, "Cannot register clock %d - %s\n", i, name);
            return ret;
        }

        *(*clk_data).hws.add(i as usize) = hw;
    }

    ret = devm_of_clk_add_hw_provider(dev, of_clk_hw_onecell_get, clk_data);
    if ret != 0 {
        dev_err!(dev, "failed to add clock hardware provider (%d)\n", ret);
    }

    ret
}

unsafe extern "C" fn spacemit_cadev_release(dev: *mut device) {
    let adev: *mut auxiliary_device = to_auxiliary_dev(dev);

    ida_free(&mut auxiliary_ids, (*adev).id);
    kfree(to_spacemit_ccu_adev(adev));
}

unsafe extern "C" fn spacemit_adev_unregister(data: *mut core::ffi::c_void) {
    let adev: *mut auxiliary_device = data as *mut auxiliary_device;

    auxiliary_device_delete(adev);
    auxiliary_device_uninit(adev);
}

unsafe fn spacemit_ccu_reset_register(
    dev: *mut device,
    regmap: *mut regmap,
    reset_name: *const c_char,
) -> i32 {
    let cadev: *mut spacemit_ccu_adev;
    let adev: *mut auxiliary_device;
    let mut ret: i32;

    /* Nothing to do if the CCU does not implement a reset controller */
    if reset_name.is_null() {
        return 0;
    }

    cadev = kzalloc_obj!(cadev);
    if cadev.is_null() {
        return -ENOMEM;
    }

    (*cadev).regmap = regmap;

    adev = &mut (*cadev).adev;
    (*adev).name = reset_name;
    (*adev).dev.parent = dev;
    (*adev).dev.release = Some(spacemit_cadev_release);
    (*adev).dev.of_node = (*dev).of_node;
    ret = ida_alloc(&mut auxiliary_ids, GFP_KERNEL);
    if ret < 0 {
        kfree(cadev);
        return ret;
    }
    (*adev).id = ret;

    ret = auxiliary_device_init(adev);
    if ret != 0 {
        ida_free(&mut auxiliary_ids, (*adev).id);
        kfree(cadev);
        return ret;
    }

    ret = auxiliary_device_add(adev);
    if ret != 0 {
        auxiliary_device_uninit(adev);
        return ret;
    }

    return devm_add_action_or_reset(dev, spacemit_adev_unregister, adev as *mut core::ffi::c_void);
}

pub unsafe fn spacemit_ccu_probe(pdev: *mut platform_device, compat: *const c_char) -> i32 {
    let mut base_regmap: *mut regmap;
    let mut lock_regmap: *mut regmap = core::ptr::null_mut();
    let data: *const spacemit_ccu_data;
    let dev: *mut device = &mut (*pdev).dev;
    let mut ret: i32;

    base_regmap = device_node_to_regmap((*dev).of_node);
    if IS_ERR(base_regmap) {
        return dev_err_probe(dev, PTR_ERR(base_regmap), "failed to get regmap\n");
    }

    /*
     * The lock status of PLLs locate in MPMU region, while PLLs themselves
     * are in APBS region. Reference to MPMU syscon is required to check PLL
     * status.
     */
    if !compat.is_null() && of_device_is_compatible((*dev).of_node, compat) {
        let mpmu: *mut device_node = of_parse_phandle((*dev).of_node, c"spacemit,mpmu".as_ptr(), 0);
        if mpmu.is_null() {
            return dev_err_probe(dev, -ENODEV, "Cannot parse MPMU region\n");
        }

        lock_regmap = device_node_to_regmap(mpmu);
        of_node_put(mpmu);

        if IS_ERR(lock_regmap) {
            return dev_err_probe(dev, PTR_ERR(lock_regmap), "failed to get lock regmap\n");
        }
    }

    data = of_device_get_match_data(dev);

    ret = spacemit_ccu_register(dev, base_regmap, lock_regmap, data);
    if ret != 0 {
        return dev_err_probe(dev, ret, "failed to register clocks\n");
    }

    ret = spacemit_ccu_reset_register(dev, base_regmap, (*data).reset_name);
    if ret != 0 {
        return dev_err_probe(dev, ret, "failed to register resets\n");
    }

    0
}

EXPORT_SYMBOL_NS_GPL!(spacemit_ccu_probe, "CLK_SPACEMIT");
MODULE_DESCRIPTION!("SpacemiT CCU common clock driver");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
