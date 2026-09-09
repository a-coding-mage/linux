// Faithful low-level Rust translation; external kernel symbols remain unresolved by design.
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, unused_mut)]

/*
// SPDX-License-Identifier: GPL-2.0
/*
 * DAMON sysfs Interface
 * /

// #include <linux/slab.h>
// #include <linux/numa.h>

// #include "sysfs-common.h"

/*
 * probe directory
 * /

struct damos_sysfs_probe {
	struct kobject kobj;
	u8 hits;
};

static mut struct damos_sysfs_probe *damos_sysfs_probe_alloc(u8 hits)
{
	struct damos_sysfs_probe *probe;

	probe = kzalloc_obj(*probe);
	if (!probe)
		return core::ptr::null_mut();
	probe.hits = hits;
	return probe;
}

static isize hits_show(struct kobject *kobj, struct kobj_attribute *attr,
		*mut core::ffi::c_charbuf)
{
	struct damos_sysfs_probe *probe = container_of(kobj,
			struct damos_sysfs_probe, kobj);

	return sysfs_emit(buf, "%hhu\n", probe.hits);
}

static void damos_sysfs_probe_release(struct kobject *kobj)
{
	struct damos_sysfs_probe *probe = container_of(kobj,
			struct damos_sysfs_probe, kobj);

	kfree(probe);
}

static mut struct kobj_attribute damos_sysfs_probe_hits_attr =
		__ATTR_RO_MODE(hits, 0400);

static mut struct attribute *damos_sysfs_probe_attrs[] = {
	&damos_sysfs_probe_hits_attr.attr,
	core::ptr::null_mut(),
};
ATTRIBUTE_GROUPS(damos_sysfs_probe);

static const struct kobj_type damos_sysfs_probe_ktype = {
	.release = damos_sysfs_probe_release,
	.sysfs_ops = &kobj_sysfs_ops,
	.default_groups = damos_sysfs_probe_groups,
};

/*
 * probes directory
 * /

struct damos_sysfs_probes {
	struct kobject kobj;
	struct damos_sysfs_probe **probes_arr;
	int nr;
};

static mut struct damos_sysfs_probes *damos_sysfs_probes_alloc(void)
{
	return kzalloc_obj(struct damos_sysfs_probes);
}

static void damos_sysfs_probes_rm_dirs(struct damos_sysfs_probes *probes)
{
	struct damos_sysfs_probe **probes_arr = probes.probes_arr;
	int i;

	for (i = 0; i < probes.nr; i++)
		kobject_put(&probes_arr[i].kobj);
	probes.nr = 0;
	kfree(probes_arr);
	probes.probes_arr = core::ptr::null_mut();
}

static int damos_sysfs_probes_add_dirs(struct damos_sysfs_probes *probes,
		struct damon_ctx *ctx, struct damon_region *region)
{
	struct damon_probe *probe;
	struct damos_sysfs_probe **probes_arr;
	int i = 0;

	damon_for_each_probe(probe, ctx)
		i++;

	if (!i)
		return 0;

	probes_arr = kmalloc_objs(*probes_arr, i);
	if (!probes_arr)
		return -ENOMEM;
	probes.probes_arr = probes_arr;

	i = 0;
	damon_for_each_probe(probe, ctx) {
		struct damos_sysfs_probe *sys_probe;
		int err;

		sys_probe = damos_sysfs_probe_alloc(
				damon_probe_hits_mvsum(i, region, ctx));
		if (!sys_probe) {
			damos_sysfs_probes_rm_dirs(probes);
			return -ENOMEM;
		}
		err = kobject_init_and_add(&sys_probe.kobj,
				&damos_sysfs_probe_ktype, &probes.kobj, "%d",
				i);
		if (err) {
			kobject_put(&sys_probe.kobj);
			damos_sysfs_probes_rm_dirs(probes);
			return err;
		}
		probes_arr[i++] = sys_probe;
		probes.nr++;
	}
	return 0;
}

static void damos_sysfs_probes_release(struct kobject *kobj)
{
	struct damos_sysfs_probes *probes = container_of(kobj,
			struct damos_sysfs_probes, kobj);

	kfree(probes);
}

static const struct kobj_type damos_sysfs_probes_ktype = {
	.release = damos_sysfs_probes_release,
	.sysfs_ops = &kobj_sysfs_ops,
};

/*
 * scheme region directory
 * /

struct damon_sysfs_scheme_region {
	struct kobject kobj;
	struct damon_addr_range ar;
	u32 nr_accesses;
	u32 age;
	usize sz_filter_passed;
	struct damos_sysfs_probes *probes;
	struct list_head list;
};

static mut struct damon_sysfs_scheme_region *damon_sysfs_scheme_region_alloc(
		struct damon_region *region, struct damon_ctx *ctx)
{
	struct damon_sysfs_scheme_region *sysfs_region = kmalloc_obj(*sysfs_region);

	if (!sysfs_region)
		return core::ptr::null_mut();
	sysfs_region.kobj = (struct kobject){};
	sysfs_region.ar = region.ar;
	sysfs_region.nr_accesses = damon_nr_accesses_mvsum(region, ctx);
	sysfs_region.age = region.age;
	sysfs_region.probes = core::ptr::null_mut();
	INIT_LIST_HEAD(&sysfs_region.list);
	return sysfs_region;
}

static int damos_sysfs_region_add_dirs(
		struct damon_sysfs_scheme_region *region,
		struct damon_ctx *ctx,
		struct damon_region *dregion)
{
	struct damos_sysfs_probes *probes = damos_sysfs_probes_alloc();
	int err;

	if (!probes)
		return -ENOMEM;
	err = kobject_init_and_add(&probes.kobj, &damos_sysfs_probes_ktype,
			&region.kobj, "probes");
	if (err)
		goto fail;
	err = damos_sysfs_probes_add_dirs(probes, ctx, dregion);
	if (err)
		goto fail;

	region.probes = probes;
	return 0;

fail:
	kobject_put(&probes.kobj);
	return err;
}

static void damos_sysfs_region_rm_dirs(
		struct damon_sysfs_scheme_region *region)
{
	damos_sysfs_probes_rm_dirs(region.probes);
	kobject_put(&region.probes.kobj);
}

static isize start_show(struct kobject *kobj, struct kobj_attribute *attr,
		*mut core::ffi::c_charbuf)
{
	struct damon_sysfs_scheme_region *region = container_of(kobj,
			struct damon_sysfs_scheme_region, kobj);

	return sysfs_emit(buf, "%lu\n", region.ar.start);
}

static isize end_show(struct kobject *kobj, struct kobj_attribute *attr,
		*mut core::ffi::c_charbuf)
{
	struct damon_sysfs_scheme_region *region = container_of(kobj,
			struct damon_sysfs_scheme_region, kobj);

	return sysfs_emit(buf, "%lu\n", region.ar.end);
}

static isize nr_accesses_show(struct kobject *kobj,
		struct kobj_attribute *attr, *mut core::ffi::c_charbuf)
{
	struct damon_sysfs_scheme_region *region = container_of(kobj,
			struct damon_sysfs_scheme_region, kobj);

	return sysfs_emit(buf, "%u\n", region.nr_accesses);
}

static isize age_show(struct kobject *kobj, struct kobj_attribute *attr,
		*mut core::ffi::c_charbuf)
{
	struct damon_sysfs_scheme_region *region = container_of(kobj,
			struct damon_sysfs_scheme_region, kobj);

	return sysfs_emit(buf, "%u\n", region.age);
}

static isize sz_filter_passed_show(struct kobject *kobj,
		struct kobj_attribute *attr, *mut core::ffi::c_charbuf)
{
	struct damon_sysfs_scheme_region *region = container_of(kobj,
			struct damon_sysfs_scheme_region, kobj);

	return sysfs_emit(buf, "%lu\n", region.sz_filter_passed);
}

static void damon_sysfs_scheme_region_release(struct kobject *kobj)
{
	struct damon_sysfs_scheme_region *region = container_of(kobj,
			struct damon_sysfs_scheme_region, kobj);

	kfree(region);
}

static mut struct kobj_attribute damon_sysfs_scheme_region_start_attr =
		__ATTR_RO_MODE(start, 0400);

static mut struct kobj_attribute damon_sysfs_scheme_region_end_attr =
		__ATTR_RO_MODE(end, 0400);

static mut struct kobj_attribute damon_sysfs_scheme_region_nr_accesses_attr =
		__ATTR_RO_MODE(nr_accesses, 0400);

static mut struct kobj_attribute damon_sysfs_scheme_region_age_attr =
		__ATTR_RO_MODE(age, 0400);

static mut struct kobj_attribute damon_sysfs_scheme_region_sz_filter_passed_attr =
		__ATTR_RO_MODE(sz_filter_passed, 0400);

static mut struct attribute *damon_sysfs_scheme_region_attrs[] = {
	&damon_sysfs_scheme_region_start_attr.attr,
	&damon_sysfs_scheme_region_end_attr.attr,
	&damon_sysfs_scheme_region_nr_accesses_attr.attr,
	&damon_sysfs_scheme_region_age_attr.attr,
	&damon_sysfs_scheme_region_sz_filter_passed_attr.attr,
	core::ptr::null_mut(),
};
ATTRIBUTE_GROUPS(damon_sysfs_scheme_region);

static const struct kobj_type damon_sysfs_scheme_region_ktype = {
	.release = damon_sysfs_scheme_region_release,
	.sysfs_ops = &kobj_sysfs_ops,
	.default_groups = damon_sysfs_scheme_region_groups,
};

/*
 * scheme regions directory
 * /

struct damon_sysfs_scheme_regions {
	struct kobject kobj;
	struct list_head regions_list;
	int nr_regions;
	usize total_bytes;
};

static mut struct damon_sysfs_scheme_regions *
damon_sysfs_scheme_regions_alloc(void)
{
	struct damon_sysfs_scheme_regions *regions = kmalloc_obj(*regions);

	if (!regions)
		return core::ptr::null_mut();

	regions.kobj = (struct kobject){};
	INIT_LIST_HEAD(&regions.regions_list);
	regions.nr_regions = 0;
	regions.total_bytes = 0;
	return regions;
}

static isize total_bytes_show(struct kobject *kobj,
		struct kobj_attribute *attr, *mut core::ffi::c_charbuf)
{
	struct damon_sysfs_scheme_regions *regions = container_of(kobj,
			struct damon_sysfs_scheme_regions, kobj);

	return sysfs_emit(buf, "%lu\n", regions.total_bytes);
}

static void damon_sysfs_scheme_regions_rm_dirs(
		struct damon_sysfs_scheme_regions *regions)
{
	struct damon_sysfs_scheme_region *r, *next;

	list_for_each_entry_safe(r, next, &regions.regions_list, list) {
		damos_sysfs_region_rm_dirs(r);
		list_del(&r.list);
		kobject_del(&r.kobj);
		kobject_put(&r.kobj);
		regions.nr_regions--;
	}
}

static void damon_sysfs_scheme_regions_release(struct kobject *kobj)
{
	kfree(container_of(kobj, struct damon_sysfs_scheme_regions, kobj));
}

static mut struct kobj_attribute damon_sysfs_scheme_regions_total_bytes_attr =
		__ATTR_RO_MODE(total_bytes, 0400);

static mut struct attribute *damon_sysfs_scheme_regions_attrs[] = {
	&damon_sysfs_scheme_regions_total_bytes_attr.attr,
	core::ptr::null_mut(),
};
ATTRIBUTE_GROUPS(damon_sysfs_scheme_regions);

static const struct kobj_type damon_sysfs_scheme_regions_ktype = {
	.release = damon_sysfs_scheme_regions_release,
	.sysfs_ops = &kobj_sysfs_ops,
	.default_groups = damon_sysfs_scheme_regions_groups,
};

/*
 * schemes/stats directory
 * /

struct damon_sysfs_stats {
	struct kobject kobj;
	usize nr_tried;
	usize sz_tried;
	usize nr_applied;
	usize sz_applied;
	usize sz_ops_filter_passed;
	usize qt_exceeds;
	usize nr_snapshots;
	usize max_nr_snapshots;
};

static mut struct damon_sysfs_stats *damon_sysfs_stats_alloc(void)
{
	return kzalloc_obj(struct damon_sysfs_stats);
}

static isize nr_tried_show(struct kobject *kobj, struct kobj_attribute *attr,
		*mut core::ffi::c_charbuf)
{
	struct damon_sysfs_stats *stats = container_of(kobj,
			struct damon_sysfs_stats, kobj);

	return sysfs_emit(buf, "%lu\n", stats.nr_tried);
}

static isize sz_tried_show(struct kobject *kobj, struct kobj_attribute *attr,
		*mut core::ffi::c_charbuf)
{
	struct damon_sysfs_stats *stats = container_of(kobj,
			struct damon_sysfs_stats, kobj);

	return sysfs_emit(buf, "%lu\n", stats.sz_tried);
}

static isize nr_applied_show(struct kobject *kobj,
		struct kobj_attribute *attr, *mut core::ffi::c_charbuf)
{
	struct damon_sysfs_stats *stats = container_of(kobj,
			struct damon_sysfs_stats, kobj);

	return sysfs_emit(buf, "%lu\n", stats.nr_applied);
}

static isize sz_applied_show(struct kobject *kobj,
		struct kobj_attribute *attr, *mut core::ffi::c_charbuf)
{
	struct damon_sysfs_stats *stats = container_of(kobj,
			struct damon_sysfs_stats, kobj);

	return sysfs_emit(buf, "%lu\n", stats.sz_applied);
}

static isize sz_ops_filter_passed_show(struct kobject *kobj,
		struct kobj_attribute *attr, *mut core::ffi::c_charbuf)
{
	struct damon_sysfs_stats *stats = container_of(kobj,
			struct damon_sysfs_stats, kobj);

	return sysfs_emit(buf, "%lu\n", stats.sz_ops_filter_passed);
}

static isize qt_exceeds_show(struct kobject *kobj,
		struct kobj_attribute *attr, *mut core::ffi::c_charbuf)
{
	struct damon_sysfs_stats *stats = container_of(kobj,
			struct damon_sysfs_stats, kobj);

	return sysfs_emit(buf, "%lu\n", stats.qt_exceeds);
}

static isize nr_snapshots_show(struct kobject *kobj,
		struct kobj_attribute *attr, *mut core::ffi::c_charbuf)
{
	struct damon_sysfs_stats *stats = container_of(kobj,
			struct damon_sysfs_stats, kobj);

	return sysfs_emit(buf, "%lu\n", stats.nr_snapshots);
}

static isize max_nr_snapshots_show(struct kobject *kobj,
		struct kobj_attribute *attr, *mut core::ffi::c_charbuf)
{
	struct damon_sysfs_stats *stats = container_of(kobj,
			struct damon_sysfs_stats, kobj);

	return sysfs_emit(buf, "%lu\n", stats.max_nr_snapshots);
}

static isize max_nr_snapshots_store(struct kobject *kobj,
		struct kobj_attribute *attr, const *mut core::ffi::c_charbuf, usize count)
{
	struct damon_sysfs_stats *stats = container_of(kobj,
			struct damon_sysfs_stats, kobj);
	usize max_nr_snapshots, err = kstrtoul(buf, 0, &max_nr_snapshots);

	if (err)
		return err;
	stats.max_nr_snapshots = max_nr_snapshots;
	return count;
}

static void damon_sysfs_stats_release(struct kobject *kobj)
{
	kfree(container_of(kobj, struct damon_sysfs_stats, kobj));
}

static mut struct kobj_attribute damon_sysfs_stats_nr_tried_attr =
		__ATTR_RO_MODE(nr_tried, 0400);

static mut struct kobj_attribute damon_sysfs_stats_sz_tried_attr =
		__ATTR_RO_MODE(sz_tried, 0400);

static mut struct kobj_attribute damon_sysfs_stats_nr_applied_attr =
		__ATTR_RO_MODE(nr_applied, 0400);

static mut struct kobj_attribute damon_sysfs_stats_sz_applied_attr =
		__ATTR_RO_MODE(sz_applied, 0400);

static mut struct kobj_attribute damon_sysfs_stats_sz_ops_filter_passed_attr =
		__ATTR_RO_MODE(sz_ops_filter_passed, 0400);

static mut struct kobj_attribute damon_sysfs_stats_qt_exceeds_attr =
		__ATTR_RO_MODE(qt_exceeds, 0400);

static mut struct kobj_attribute damon_sysfs_stats_nr_snapshots_attr =
		__ATTR_RO_MODE(nr_snapshots, 0400);

static mut struct kobj_attribute damon_sysfs_stats_max_nr_snapshots_attr =
		__ATTR_RW_MODE(max_nr_snapshots, 0600);

static mut struct attribute *damon_sysfs_stats_attrs[] = {
	&damon_sysfs_stats_nr_tried_attr.attr,
	&damon_sysfs_stats_sz_tried_attr.attr,
	&damon_sysfs_stats_nr_applied_attr.attr,
	&damon_sysfs_stats_sz_applied_attr.attr,
	&damon_sysfs_stats_sz_ops_filter_passed_attr.attr,
	&damon_sysfs_stats_qt_exceeds_attr.attr,
	&damon_sysfs_stats_nr_snapshots_attr.attr,
	&damon_sysfs_stats_max_nr_snapshots_attr.attr,
	core::ptr::null_mut(),
};
ATTRIBUTE_GROUPS(damon_sysfs_stats);

static const struct kobj_type damon_sysfs_stats_ktype = {
	.release = damon_sysfs_stats_release,
	.sysfs_ops = &kobj_sysfs_ops,
	.default_groups = damon_sysfs_stats_groups,
};

/*
 * filter directory
 * /

/*
 * enum damos_sysfs_filter_handle_layer - Layers handling filters of a dir.
 * /
enum damos_sysfs_filter_handle_layer {
	DAMOS_SYSFS_FILTER_HANDLE_LAYER_CORE,
	DAMOS_SYSFS_FILTER_HANDLE_LAYER_OPS,
	DAMOS_SYSFS_FILTER_HANDLE_LAYER_BOTH,
};

struct damon_sysfs_scheme_filter {
	struct kobject kobj;
	enum damos_sysfs_filter_handle_layer handle_layer;
	enum damos_filter_type type;
	bool matching;
	bool allow;
	*mut core::ffi::c_charmemcg_path;
	struct damon_addr_range addr_range;
	struct damon_size_range sz_range;
	int target_idx;
};

static mut struct damon_sysfs_scheme_filter *damon_sysfs_scheme_filter_alloc(
		enum damos_sysfs_filter_handle_layer layer)
{
	struct damon_sysfs_scheme_filter *filter;

	filter = kzalloc_obj(struct damon_sysfs_scheme_filter);
	if (filter)
		filter.handle_layer = layer;
	return filter;
}

struct damos_sysfs_filter_type_name {
	enum damos_filter_type type;
	*mut core::ffi::c_charname;
};

static const struct damos_sysfs_filter_type_name
damos_sysfs_filter_type_names[] = {
	{
		.type = DAMOS_FILTER_TYPE_ANON,
		.name = "anon",
	},
	{
		.type = DAMOS_FILTER_TYPE_ACTIVE,
		.name = "active",
	},
	{
		.type = DAMOS_FILTER_TYPE_MEMCG,
		.name = "memcg",
	},
	{
		.type = DAMOS_FILTER_TYPE_YOUNG,
		.name = "young",
	},
	{
		.type = DAMOS_FILTER_TYPE_HUGEPAGE_SIZE,
		.name = "hugepage_size",
	},
	{
		.type = DAMOS_FILTER_TYPE_UNMAPPED,
		.name = "unmapped",
	},
	{
		.type = DAMOS_FILTER_TYPE_ADDR,
		.name = "addr",
	},
	{
		.type = DAMOS_FILTER_TYPE_TARGET,
		.name = "target",
	},
};

static isize type_show(struct kobject *kobj,
		struct kobj_attribute *attr, *mut core::ffi::c_charbuf)
{
	struct damon_sysfs_scheme_filter *filter = container_of(kobj,
			struct damon_sysfs_scheme_filter, kobj);
	int i;

	for (i = 0; i < ARRAY_SIZE(damos_sysfs_filter_type_names); i++) {
		const struct damos_sysfs_filter_type_name *type_name;

		type_name = &damos_sysfs_filter_type_names[i];
		if (type_name.type == filter.type)
			return sysfs_emit(buf, "%s\n", type_name.name);
	}
	return -EINVAL;
}

static bool damos_sysfs_scheme_filter_valid_type(
		enum damos_sysfs_filter_handle_layer layer,
		enum damos_filter_type type)
{
	switch (layer) {
	case DAMOS_SYSFS_FILTER_HANDLE_LAYER_BOTH:
		return true;
	case DAMOS_SYSFS_FILTER_HANDLE_LAYER_CORE:
		return !damos_filter_for_ops(type);
	case DAMOS_SYSFS_FILTER_HANDLE_LAYER_OPS:
		return damos_filter_for_ops(type);
	default:
		break;
	}
	return false;
}

static isize type_store(struct kobject *kobj,
		struct kobj_attribute *attr, const *mut core::ffi::c_charbuf, usize count)
{
	struct damon_sysfs_scheme_filter *filter = container_of(kobj,
			struct damon_sysfs_scheme_filter, kobj);
	isize ret = -EINVAL;
	int i;

	for (i = 0; i < ARRAY_SIZE(damos_sysfs_filter_type_names); i++) {
		const struct damos_sysfs_filter_type_name *type_name;

		type_name = &damos_sysfs_filter_type_names[i];
		if (sysfs_streq(buf, type_name.name)) {
			if (!damos_sysfs_scheme_filter_valid_type(
						filter.handle_layer,
						type_name.type))
				break;
			filter.type = type_name.type;
			ret = count;
			break;
		}
	}
	return ret;
}

static isize matching_show(struct kobject *kobj,
		struct kobj_attribute *attr, *mut core::ffi::c_charbuf)
{
	struct damon_sysfs_scheme_filter *filter = container_of(kobj,
			struct damon_sysfs_scheme_filter, kobj);

	return sysfs_emit(buf, "%c\n", filter.matching ? 'Y' : 'N');
}

static isize matching_store(struct kobject *kobj,
		struct kobj_attribute *attr, const *mut core::ffi::c_charbuf, usize count)
{
	struct damon_sysfs_scheme_filter *filter = container_of(kobj,
			struct damon_sysfs_scheme_filter, kobj);
	bool matching;
	int err = kstrtobool(buf, &matching);

	if (err)
		return err;

	filter.matching = matching;
	return count;
}

static isize allow_show(struct kobject *kobj,
		struct kobj_attribute *attr, *mut core::ffi::c_charbuf)
{
	struct damon_sysfs_scheme_filter *filter = container_of(kobj,
			struct damon_sysfs_scheme_filter, kobj);

	return sysfs_emit(buf, "%c\n", filter.allow ? 'Y' : 'N');
}

static isize allow_store(struct kobject *kobj,
		struct kobj_attribute *attr, const *mut core::ffi::c_charbuf, usize count)
{
	struct damon_sysfs_scheme_filter *filter = container_of(kobj,
			struct damon_sysfs_scheme_filter, kobj);
	bool allow;
	int err = kstrtobool(buf, &allow);

	if (err)
		return err;

	filter.allow = allow;
	return count;
}

static isize memcg_path_show(struct kobject *kobj,
		struct kobj_attribute *attr, *mut core::ffi::c_charbuf)
{
	struct damon_sysfs_scheme_filter *filter = container_of(kobj,
			struct damon_sysfs_scheme_filter, kobj);
	int len;

	if (!mutex_trylock(&damon_sysfs_lock))
		return -EBUSY;
	len = sysfs_emit(buf, "%s\n",
			filter.memcg_path ? filter.memcg_path : "");
	mutex_unlock(&damon_sysfs_lock);
	return len;
}

static isize memcg_path_store(struct kobject *kobj,
		struct kobj_attribute *attr, const *mut core::ffi::c_charbuf, usize count)
{
	struct damon_sysfs_scheme_filter *filter = container_of(kobj,
			struct damon_sysfs_scheme_filter, kobj);
	*mut core::ffi::c_charpath = kmalloc_array(size_add(count, 1), sizeof(*path),
				   GFP_KERNEL);

	if (!path)
		return -ENOMEM;

	strscpy(path, buf, count + 1);
	if (!mutex_trylock(&damon_sysfs_lock)) {
		kfree(path);
		return -EBUSY;
	}
	kfree(filter.memcg_path);
	filter.memcg_path = path;
	mutex_unlock(&damon_sysfs_lock);
	return count;
}

static isize addr_start_show(struct kobject *kobj,
		struct kobj_attribute *attr, *mut core::ffi::c_charbuf)
{
	struct damon_sysfs_scheme_filter *filter = container_of(kobj,
			struct damon_sysfs_scheme_filter, kobj);

	return sysfs_emit(buf, "%lu\n", filter.addr_range.start);
}

static isize addr_start_store(struct kobject *kobj,
		struct kobj_attribute *attr, const *mut core::ffi::c_charbuf, usize count)
{
	struct damon_sysfs_scheme_filter *filter = container_of(kobj,
			struct damon_sysfs_scheme_filter, kobj);
	int err = kstrtoul(buf, 0, &filter.addr_range.start);

	return err ? err : count;
}

static isize addr_end_show(struct kobject *kobj,
		struct kobj_attribute *attr, *mut core::ffi::c_charbuf)
{
	struct damon_sysfs_scheme_filter *filter = container_of(kobj,
			struct damon_sysfs_scheme_filter, kobj);

	return sysfs_emit(buf, "%lu\n", filter.addr_range.end);
}

static isize addr_end_store(struct kobject *kobj,
		struct kobj_attribute *attr, const *mut core::ffi::c_charbuf, usize count)
{
	struct damon_sysfs_scheme_filter *filter = container_of(kobj,
			struct damon_sysfs_scheme_filter, kobj);
	int err = kstrtoul(buf, 0, &filter.addr_range.end);

	return err ? err : count;
}

static isize min_show(struct kobject *kobj,
		struct kobj_attribute *attr, *mut core::ffi::c_charbuf)
{
	struct damon_sysfs_scheme_filter *filter = container_of(kobj,
			struct damon_sysfs_scheme_filter, kobj);

	return sysfs_emit(buf, "%lu\n", filter.sz_range.min);
}

static isize min_store(struct kobject *kobj,
		struct kobj_attribute *attr, const *mut core::ffi::c_charbuf, usize count)
{
	struct damon_sysfs_scheme_filter *filter = container_of(kobj,
			struct damon_sysfs_scheme_filter, kobj);
	int err = kstrtoul(buf, 0, &filter.sz_range.min);

	return err ? err : count;
}

static isize max_show(struct kobject *kobj,
		struct kobj_attribute *attr, *mut core::ffi::c_charbuf)
{
	struct damon_sysfs_scheme_filter *filter = container_of(kobj,
			struct damon_sysfs_scheme_filter, kobj);

	return sysfs_emit(buf, "%lu\n", filter.sz_range.max);
}

static isize max_store(struct kobject *kobj,
		struct kobj_attribute *attr, const *mut core::ffi::c_charbuf, usize count)
{
	struct damon_sysfs_scheme_filter *filter = container_of(kobj,
			struct damon_sysfs_scheme_filter, kobj);
	int err = kstrtoul(buf, 0, &filter.sz_range.max);

	return err ? err : count;
}

static isize damon_target_idx_show(struct kobject *kobj,
		struct kobj_attribute *attr, *mut core::ffi::c_charbuf)
{
	struct damon_sysfs_scheme_filter *filter = container_of(kobj,
			struct damon_sysfs_scheme_filter, kobj);

	return sysfs_emit(buf, "%d\n", filter.target_idx);
}

static isize damon_target_idx_store(struct kobject *kobj,
		struct kobj_attribute *attr, const *mut core::ffi::c_charbuf, usize count)
{
	struct damon_sysfs_scheme_filter *filter = container_of(kobj,
			struct damon_sysfs_scheme_filter, kobj);
	int err = kstrtoint(buf, 0, &filter.target_idx);

	return err ? err : count;
}

static void damon_sysfs_scheme_filter_release(struct kobject *kobj)
{
	struct damon_sysfs_scheme_filter *filter = container_of(kobj,
			struct damon_sysfs_scheme_filter, kobj);

	kfree(filter.memcg_path);
	kfree(filter);
}

static mut struct kobj_attribute damon_sysfs_scheme_filter_type_attr =
		__ATTR_RW_MODE(type, 0600);

static mut struct kobj_attribute damon_sysfs_scheme_filter_matching_attr =
		__ATTR_RW_MODE(matching, 0600);

static mut struct kobj_attribute damon_sysfs_scheme_filter_allow_attr =
		__ATTR_RW_MODE(allow, 0600);

static mut struct kobj_attribute damon_sysfs_scheme_filter_memcg_path_attr =
		__ATTR_RW_MODE(memcg_path, 0600);

static mut struct kobj_attribute damon_sysfs_scheme_filter_addr_start_attr =
		__ATTR_RW_MODE(addr_start, 0600);

static mut struct kobj_attribute damon_sysfs_scheme_filter_addr_end_attr =
		__ATTR_RW_MODE(addr_end, 0600);

static mut struct kobj_attribute damon_sysfs_scheme_filter_min_attr =
		__ATTR_RW_MODE(min, 0600);

static mut struct kobj_attribute damon_sysfs_scheme_filter_max_attr =
		__ATTR_RW_MODE(max, 0600);

static mut struct kobj_attribute damon_sysfs_scheme_filter_damon_target_idx_attr =
		__ATTR_RW_MODE(damon_target_idx, 0600);

static mut struct attribute *damon_sysfs_scheme_filter_attrs[] = {
	&damon_sysfs_scheme_filter_type_attr.attr,
	&damon_sysfs_scheme_filter_matching_attr.attr,
	&damon_sysfs_scheme_filter_allow_attr.attr,
	&damon_sysfs_scheme_filter_memcg_path_attr.attr,
	&damon_sysfs_scheme_filter_addr_start_attr.attr,
	&damon_sysfs_scheme_filter_addr_end_attr.attr,
	&damon_sysfs_scheme_filter_min_attr.attr,
	&damon_sysfs_scheme_filter_max_attr.attr,
	&damon_sysfs_scheme_filter_damon_target_idx_attr.attr,
	core::ptr::null_mut(),
};
ATTRIBUTE_GROUPS(damon_sysfs_scheme_filter);

static const struct kobj_type damon_sysfs_scheme_filter_ktype = {
	.release = damon_sysfs_scheme_filter_release,
	.sysfs_ops = &kobj_sysfs_ops,
	.default_groups = damon_sysfs_scheme_filter_groups,
};

/*
 * filters directory
 * /

struct damon_sysfs_scheme_filters {
	struct kobject kobj;
	enum damos_sysfs_filter_handle_layer handle_layer;
	struct damon_sysfs_scheme_filter **filters_arr;
	int nr;
};

static mut struct damon_sysfs_scheme_filters *
damon_sysfs_scheme_filters_alloc(enum damos_sysfs_filter_handle_layer layer)
{
	struct damon_sysfs_scheme_filters *filters;

	filters = kzalloc_obj(struct damon_sysfs_scheme_filters);
	if (filters)
		filters.handle_layer = layer;
	return filters;
}

static void damon_sysfs_scheme_filters_rm_dirs(
		struct damon_sysfs_scheme_filters *filters)
{
	struct damon_sysfs_scheme_filter **filters_arr = filters.filters_arr;
	int i;

	for (i = 0; i < filters.nr; i++) {
		kobject_del(&filters_arr[i].kobj);
		kobject_put(&filters_arr[i].kobj);
	}
	filters.nr = 0;
	kfree(filters_arr);
	filters.filters_arr = core::ptr::null_mut();
}

static int damon_sysfs_scheme_filters_add_dirs(
		struct damon_sysfs_scheme_filters *filters, int nr_filters)
{
	struct damon_sysfs_scheme_filter **filters_arr, *filter;
	int err, i;

	damon_sysfs_scheme_filters_rm_dirs(filters);
	if (!nr_filters)
		return 0;

	filters_arr = kmalloc_objs(*filters_arr, nr_filters,
				   GFP_KERNEL | __GFP_NOWARN);
	if (!filters_arr)
		return -ENOMEM;
	filters.filters_arr = filters_arr;

	for (i = 0; i < nr_filters; i++) {
		filter = damon_sysfs_scheme_filter_alloc(
				filters.handle_layer);
		if (!filter) {
			damon_sysfs_scheme_filters_rm_dirs(filters);
			return -ENOMEM;
		}

		err = kobject_init_and_add(&filter.kobj,
				&damon_sysfs_scheme_filter_ktype,
				&filters.kobj, "%d", i);
		if (err) {
			kobject_put(&filter.kobj);
			damon_sysfs_scheme_filters_rm_dirs(filters);
			return err;
		}

		filters_arr[i] = filter;
		filters.nr++;
	}
	return 0;
}

static isize nr_filters_show(struct kobject *kobj,
		struct kobj_attribute *attr, *mut core::ffi::c_charbuf)
{
	struct damon_sysfs_scheme_filters *filters = container_of(kobj,
			struct damon_sysfs_scheme_filters, kobj);

	return sysfs_emit(buf, "%d\n", filters.nr);
}

static isize nr_filters_store(struct kobject *kobj,
		struct kobj_attribute *attr, const *mut core::ffi::c_charbuf, usize count)
{
	struct damon_sysfs_scheme_filters *filters;
	int nr, err = kstrtoint(buf, 0, &nr);

	if (err)
		return err;
	if (nr < 0)
		return -EINVAL;

	filters = container_of(kobj, struct damon_sysfs_scheme_filters, kobj);

	if (!mutex_trylock(&damon_sysfs_lock))
		return -EBUSY;
	err = damon_sysfs_scheme_filters_add_dirs(filters, nr);
	mutex_unlock(&damon_sysfs_lock);
	if (err)
		return err;

	return count;
}

static void damon_sysfs_scheme_filters_release(struct kobject *kobj)
{
	kfree(container_of(kobj, struct damon_sysfs_scheme_filters, kobj));
}

static mut struct kobj_attribute damon_sysfs_scheme_filters_nr_attr =
		__ATTR_RW_MODE(nr_filters, 0600);

static mut struct attribute *damon_sysfs_scheme_filters_attrs[] = {
	&damon_sysfs_scheme_filters_nr_attr.attr,
	core::ptr::null_mut(),
};
ATTRIBUTE_GROUPS(damon_sysfs_scheme_filters);

static const struct kobj_type damon_sysfs_scheme_filters_ktype = {
	.release = damon_sysfs_scheme_filters_release,
	.sysfs_ops = &kobj_sysfs_ops,
	.default_groups = damon_sysfs_scheme_filters_groups,
};

/*
 * watermarks directory
 * /

struct damon_sysfs_watermarks {
	struct kobject kobj;
	enum damos_wmark_metric metric;
	usize interval_us;
	usize high;
	usize mid;
	usize low;
};

static mut struct damon_sysfs_watermarks *damon_sysfs_watermarks_alloc(
		enum damos_wmark_metric metric, usize interval_us,
		usize high, usize mid, usize low)
{
	struct damon_sysfs_watermarks *watermarks = kmalloc_obj(*watermarks);

	if (!watermarks)
		return core::ptr::null_mut();
	watermarks.kobj = (struct kobject){};
	watermarks.metric = metric;
	watermarks.interval_us = interval_us;
	watermarks.high = high;
	watermarks.mid = mid;
	watermarks.low = low;
	return watermarks;
}

struct damos_sysfs_wmark_metric_name {
	enum damos_wmark_metric metric;
	*mut core::ffi::c_charname;
};

static const struct damos_sysfs_wmark_metric_name
damos_sysfs_wmark_metric_names[] = {
	{
		.metric = DAMOS_WMARK_NONE,
		.name = "none",
	},
	{
		.metric = DAMOS_WMARK_FREE_MEM_RATE,
		.name = "free_mem_rate",
	},
};

static isize metric_show(struct kobject *kobj, struct kobj_attribute *attr,
		*mut core::ffi::c_charbuf)
{
	struct damon_sysfs_watermarks *watermarks = container_of(kobj,
			struct damon_sysfs_watermarks, kobj);
	int i;

	for (i = 0; i < ARRAY_SIZE(damos_sysfs_wmark_metric_names); i++) {
		const struct damos_sysfs_wmark_metric_name *metric_name;

		metric_name = &damos_sysfs_wmark_metric_names[i];
		if (metric_name.metric == watermarks.metric)
			return sysfs_emit(buf, "%s\n", metric_name.name);
	}
	return -EINVAL;
}

static isize metric_store(struct kobject *kobj, struct kobj_attribute *attr,
		const *mut core::ffi::c_charbuf, usize count)
{
	struct damon_sysfs_watermarks *watermarks = container_of(kobj,
			struct damon_sysfs_watermarks, kobj);
	int i;

	for (i = 0; i < ARRAY_SIZE(damos_sysfs_wmark_metric_names); i++) {
		const struct damos_sysfs_wmark_metric_name *metric_name;

		metric_name = &damos_sysfs_wmark_metric_names[i];
		if (sysfs_streq(buf, metric_name.name)) {
			watermarks.metric = metric_name.metric;
			return count;
		}
	}
	return -EINVAL;
}

static isize interval_us_show(struct kobject *kobj,
		struct kobj_attribute *attr, *mut core::ffi::c_charbuf)
{
	struct damon_sysfs_watermarks *watermarks = container_of(kobj,
			struct damon_sysfs_watermarks, kobj);

	return sysfs_emit(buf, "%lu\n", watermarks.interval_us);
}

static isize interval_us_store(struct kobject *kobj,
		struct kobj_attribute *attr, const *mut core::ffi::c_charbuf, usize count)
{
	struct damon_sysfs_watermarks *watermarks = container_of(kobj,
			struct damon_sysfs_watermarks, kobj);
	int err = kstrtoul(buf, 0, &watermarks.interval_us);

	return err ? err : count;
}

static isize high_show(struct kobject *kobj,
		struct kobj_attribute *attr, *mut core::ffi::c_charbuf)
{
	struct damon_sysfs_watermarks *watermarks = container_of(kobj,
			struct damon_sysfs_watermarks, kobj);

	return sysfs_emit(buf, "%lu\n", watermarks.high);
}

static isize high_store(struct kobject *kobj,
		struct kobj_attribute *attr, const *mut core::ffi::c_charbuf, usize count)
{
	struct damon_sysfs_watermarks *watermarks = container_of(kobj,
			struct damon_sysfs_watermarks, kobj);
	int err = kstrtoul(buf, 0, &watermarks.high);

	return err ? err : count;
}

static isize mid_show(struct kobject *kobj,
		struct kobj_attribute *attr, *mut core::ffi::c_charbuf)
{
	struct damon_sysfs_watermarks *watermarks = container_of(kobj,
			struct damon_sysfs_watermarks, kobj);

	return sysfs_emit(buf, "%lu\n", watermarks.mid);
}

static isize mid_store(struct kobject *kobj,
		struct kobj_attribute *attr, const *mut core::ffi::c_charbuf, usize count)
{
	struct damon_sysfs_watermarks *watermarks = container_of(kobj,
			struct damon_sysfs_watermarks, kobj);
	int err = kstrtoul(buf, 0, &watermarks.mid);

	return err ? err : count;
}

static isize low_show(struct kobject *kobj,
		struct kobj_attribute *attr, *mut core::ffi::c_charbuf)
{
	struct damon_sysfs_watermarks *watermarks = container_of(kobj,
			struct damon_sysfs_watermarks, kobj);

	return sysfs_emit(buf, "%lu\n", watermarks.low);
}

static isize low_store(struct kobject *kobj,
		struct kobj_attribute *attr, const *mut core::ffi::c_charbuf, usize count)
{
	struct damon_sysfs_watermarks *watermarks = container_of(kobj,
			struct damon_sysfs_watermarks, kobj);
	int err = kstrtoul(buf, 0, &watermarks.low);

	return err ? err : count;
}

static void damon_sysfs_watermarks_release(struct kobject *kobj)
{
	kfree(container_of(kobj, struct damon_sysfs_watermarks, kobj));
}

static mut struct kobj_attribute damon_sysfs_watermarks_metric_attr =
		__ATTR_RW_MODE(metric, 0600);

static mut struct kobj_attribute damon_sysfs_watermarks_interval_us_attr =
		__ATTR_RW_MODE(interval_us, 0600);

static mut struct kobj_attribute damon_sysfs_watermarks_high_attr =
		__ATTR_RW_MODE(high, 0600);

static mut struct kobj_attribute damon_sysfs_watermarks_mid_attr =
		__ATTR_RW_MODE(mid, 0600);

static mut struct kobj_attribute damon_sysfs_watermarks_low_attr =
		__ATTR_RW_MODE(low, 0600);

static mut struct attribute *damon_sysfs_watermarks_attrs[] = {
	&damon_sysfs_watermarks_metric_attr.attr,
	&damon_sysfs_watermarks_interval_us_attr.attr,
	&damon_sysfs_watermarks_high_attr.attr,
	&damon_sysfs_watermarks_mid_attr.attr,
	&damon_sysfs_watermarks_low_attr.attr,
	core::ptr::null_mut(),
};
ATTRIBUTE_GROUPS(damon_sysfs_watermarks);

static const struct kobj_type damon_sysfs_watermarks_ktype = {
	.release = damon_sysfs_watermarks_release,
	.sysfs_ops = &kobj_sysfs_ops,
	.default_groups = damon_sysfs_watermarks_groups,
};

/*
 * quota goal directory
 * /

struct damos_sysfs_quota_goal {
	struct kobject kobj;
	enum damos_quota_goal_metric metric;
	usize target_value;
	usize current_value;
	int nid;
	*mut core::ffi::c_charpath;
};

static mut struct damos_sysfs_quota_goal *damos_sysfs_quota_goal_alloc(void)
{
	return kzalloc_obj(struct damos_sysfs_quota_goal);
}

struct damos_sysfs_qgoal_metric_name {
	enum damos_quota_goal_metric metric;
	*mut core::ffi::c_charname;
};

static mut struct damos_sysfs_qgoal_metric_name damos_sysfs_qgoal_metric_names[] = {
	{
		.metric = DAMOS_QUOTA_USER_INPUT,
		.name = "user_input",
	},
	{
		.metric = DAMOS_QUOTA_SOME_MEM_PSI_US,
		.name = "some_mem_psi_us",
	},
	{
		.metric = DAMOS_QUOTA_NODE_MEM_USED_BP,
		.name = "node_mem_used_bp",
	},
	{
		.metric = DAMOS_QUOTA_NODE_MEM_FREE_BP,
		.name = "node_mem_free_bp",
	},
	{
		.metric = DAMOS_QUOTA_NODE_MEMCG_USED_BP,
		.name = "node_memcg_used_bp",
	},
	{
		.metric = DAMOS_QUOTA_NODE_MEMCG_FREE_BP,
		.name = "node_memcg_free_bp",
	},
	{
		.metric = DAMOS_QUOTA_ACTIVE_MEM_BP,
		.name = "active_mem_bp",
	},
	{
		.metric = DAMOS_QUOTA_INACTIVE_MEM_BP,
		.name = "inactive_mem_bp",
	},
	{
		.metric = DAMOS_QUOTA_NODE_ELIGIBLE_MEM_BP,
		.name = "node_eligible_mem_bp",
	},
};

static isize target_metric_show(struct kobject *kobj,
		struct kobj_attribute *attr, *mut core::ffi::c_charbuf)
{
	struct damos_sysfs_quota_goal *goal = container_of(kobj,
			struct damos_sysfs_quota_goal, kobj);
	int i;

	for (i = 0; i < ARRAY_SIZE(damos_sysfs_qgoal_metric_names); i++) {
		struct damos_sysfs_qgoal_metric_name *metric_name;

		metric_name = &damos_sysfs_qgoal_metric_names[i];
		if (metric_name.metric == goal.metric)
			return sysfs_emit(buf, "%s\n", metric_name.name);
	}
	return -EINVAL;
}

static isize target_metric_store(struct kobject *kobj,
		struct kobj_attribute *attr, const *mut core::ffi::c_charbuf, usize count)
{
	struct damos_sysfs_quota_goal *goal = container_of(kobj,
			struct damos_sysfs_quota_goal, kobj);
	int i;

	for (i = 0; i < ARRAY_SIZE(damos_sysfs_qgoal_metric_names); i++) {
		struct damos_sysfs_qgoal_metric_name *metric_name;

		metric_name = &damos_sysfs_qgoal_metric_names[i];
		if (sysfs_streq(buf, metric_name.name)) {
			goal.metric = metric_name.metric;
			return count;
		}
	}
	return -EINVAL;
}

static isize target_value_show(struct kobject *kobj,
		struct kobj_attribute *attr, *mut core::ffi::c_charbuf)
{
	struct damos_sysfs_quota_goal *goal = container_of(kobj, struct
			damos_sysfs_quota_goal, kobj);

	return sysfs_emit(buf, "%lu\n", goal.target_value);
}

static isize target_value_store(struct kobject *kobj,
		struct kobj_attribute *attr, const *mut core::ffi::c_charbuf, usize count)
{
	struct damos_sysfs_quota_goal *goal = container_of(kobj, struct
			damos_sysfs_quota_goal, kobj);
	int err = kstrtoul(buf, 0, &goal.target_value);

	return err ? err : count;
}

static isize current_value_show(struct kobject *kobj,
		struct kobj_attribute *attr, *mut core::ffi::c_charbuf)
{
	struct damos_sysfs_quota_goal *goal = container_of(kobj, struct
			damos_sysfs_quota_goal, kobj);

	return sysfs_emit(buf, "%lu\n", goal.current_value);
}

static isize current_value_store(struct kobject *kobj,
		struct kobj_attribute *attr, const *mut core::ffi::c_charbuf, usize count)
{
	struct damos_sysfs_quota_goal *goal = container_of(kobj, struct
			damos_sysfs_quota_goal, kobj);
	int err = kstrtoul(buf, 0, &goal.current_value);

	/* feed callback should check existence of this file and read value * /
	return err ? err : count;
}

static isize nid_show(struct kobject *kobj,
		struct kobj_attribute *attr, *mut core::ffi::c_charbuf)
{
	struct damos_sysfs_quota_goal *goal = container_of(kobj, struct
			damos_sysfs_quota_goal, kobj);


	return sysfs_emit(buf, "%d\n", goal.nid);
}

static isize nid_store(struct kobject *kobj,
		struct kobj_attribute *attr, const *mut core::ffi::c_charbuf, usize count)
{
	struct damos_sysfs_quota_goal *goal = container_of(kobj, struct
			damos_sysfs_quota_goal, kobj);
	int err = kstrtoint(buf, 0, &goal.nid);

	/* feed callback should check existence of this file and read value * /
	return err ? err : count;
}

static isize path_show(struct kobject *kobj,
		struct kobj_attribute *attr, *mut core::ffi::c_charbuf)
{
	struct damos_sysfs_quota_goal *goal = container_of(kobj,
			struct damos_sysfs_quota_goal, kobj);
	int len;

	if (!mutex_trylock(&damon_sysfs_lock))
		return -EBUSY;
	len = sysfs_emit(buf, "%s\n", goal.path ? goal.path : "");
	mutex_unlock(&damon_sysfs_lock);
	return len;
}

static isize path_store(struct kobject *kobj,
		struct kobj_attribute *attr, const *mut core::ffi::c_charbuf, usize count)
{
	struct damos_sysfs_quota_goal *goal = container_of(kobj,
			struct damos_sysfs_quota_goal, kobj);
	*mut core::ffi::c_charpath = kmalloc_array(size_add(count, 1), sizeof(*path),
				   GFP_KERNEL);

	if (!path)
		return -ENOMEM;

	strscpy(path, buf, count + 1);
	if (!mutex_trylock(&damon_sysfs_lock)) {
		kfree(path);
		return -EBUSY;
	}
	kfree(goal.path);
	goal.path = path;
	mutex_unlock(&damon_sysfs_lock);
	return count;
}

static void damos_sysfs_quota_goal_release(struct kobject *kobj)
{
	struct damos_sysfs_quota_goal *goal = container_of(kobj,
			struct damos_sysfs_quota_goal, kobj);

	kfree(goal.path);
	kfree(goal);
}

static mut struct kobj_attribute damos_sysfs_quota_goal_target_metric_attr =
		__ATTR_RW_MODE(target_metric, 0600);

static mut struct kobj_attribute damos_sysfs_quota_goal_target_value_attr =
		__ATTR_RW_MODE(target_value, 0600);

static mut struct kobj_attribute damos_sysfs_quota_goal_current_value_attr =
		__ATTR_RW_MODE(current_value, 0600);

static mut struct kobj_attribute damos_sysfs_quota_goal_nid_attr =
		__ATTR_RW_MODE(nid, 0600);

static mut struct kobj_attribute damos_sysfs_quota_goal_path_attr =
		__ATTR_RW_MODE(path, 0600);

static mut struct attribute *damos_sysfs_quota_goal_attrs[] = {
	&damos_sysfs_quota_goal_target_metric_attr.attr,
	&damos_sysfs_quota_goal_target_value_attr.attr,
	&damos_sysfs_quota_goal_current_value_attr.attr,
	&damos_sysfs_quota_goal_nid_attr.attr,
	&damos_sysfs_quota_goal_path_attr.attr,
	core::ptr::null_mut(),
};
ATTRIBUTE_GROUPS(damos_sysfs_quota_goal);

static const struct kobj_type damos_sysfs_quota_goal_ktype = {
	.release = damos_sysfs_quota_goal_release,
	.sysfs_ops = &kobj_sysfs_ops,
	.default_groups = damos_sysfs_quota_goal_groups,
};

/*
 * quota goals directory
 * /

struct damos_sysfs_quota_goals {
	struct kobject kobj;
	struct damos_sysfs_quota_goal **goals_arr;	/* counted by nr * /
	int nr;
};

static mut struct damos_sysfs_quota_goals *damos_sysfs_quota_goals_alloc(void)
{
	return kzalloc_obj(struct damos_sysfs_quota_goals);
}

static void damos_sysfs_quota_goals_rm_dirs(
		struct damos_sysfs_quota_goals *goals)
{
	struct damos_sysfs_quota_goal **goals_arr = goals.goals_arr;
	int i;

	for (i = 0; i < goals.nr; i++) {
		kobject_del(&goals_arr[i].kobj);
		kobject_put(&goals_arr[i].kobj);
	}
	goals.nr = 0;
	kfree(goals_arr);
	goals.goals_arr = core::ptr::null_mut();
}

static int damos_sysfs_quota_goals_add_dirs(
		struct damos_sysfs_quota_goals *goals, int nr_goals)
{
	struct damos_sysfs_quota_goal **goals_arr, *goal;
	int err, i;

	damos_sysfs_quota_goals_rm_dirs(goals);
	if (!nr_goals)
		return 0;

	goals_arr = kmalloc_objs(*goals_arr, nr_goals,
				 GFP_KERNEL | __GFP_NOWARN);
	if (!goals_arr)
		return -ENOMEM;
	goals.goals_arr = goals_arr;

	for (i = 0; i < nr_goals; i++) {
		goal = damos_sysfs_quota_goal_alloc();
		if (!goal) {
			damos_sysfs_quota_goals_rm_dirs(goals);
			return -ENOMEM;
		}

		err = kobject_init_and_add(&goal.kobj,
				&damos_sysfs_quota_goal_ktype, &goals.kobj,
				"%d", i);
		if (err) {
			kobject_put(&goal.kobj);
			damos_sysfs_quota_goals_rm_dirs(goals);
			return err;
		}

		goals_arr[i] = goal;
		goals.nr++;
	}
	return 0;
}

static isize nr_goals_show(struct kobject *kobj,
		struct kobj_attribute *attr, *mut core::ffi::c_charbuf)
{
	struct damos_sysfs_quota_goals *goals = container_of(kobj,
			struct damos_sysfs_quota_goals, kobj);

	return sysfs_emit(buf, "%d\n", goals.nr);
}

static isize nr_goals_store(struct kobject *kobj,
		struct kobj_attribute *attr, const *mut core::ffi::c_charbuf, usize count)
{
	struct damos_sysfs_quota_goals *goals;
	int nr, err = kstrtoint(buf, 0, &nr);

	if (err)
		return err;
	if (nr < 0)
		return -EINVAL;

	goals = container_of(kobj, struct damos_sysfs_quota_goals, kobj);

	if (!mutex_trylock(&damon_sysfs_lock))
		return -EBUSY;
	err = damos_sysfs_quota_goals_add_dirs(goals, nr);
	mutex_unlock(&damon_sysfs_lock);
	if (err)
		return err;

	return count;
}

static void damos_sysfs_quota_goals_release(struct kobject *kobj)
{
	kfree(container_of(kobj, struct damos_sysfs_quota_goals, kobj));
}

static mut struct kobj_attribute damos_sysfs_quota_goals_nr_attr =
		__ATTR_RW_MODE(nr_goals, 0600);

static mut struct attribute *damos_sysfs_quota_goals_attrs[] = {
	&damos_sysfs_quota_goals_nr_attr.attr,
	core::ptr::null_mut(),
};
ATTRIBUTE_GROUPS(damos_sysfs_quota_goals);

static const struct kobj_type damos_sysfs_quota_goals_ktype = {
	.release = damos_sysfs_quota_goals_release,
	.sysfs_ops = &kobj_sysfs_ops,
	.default_groups = damos_sysfs_quota_goals_groups,
};

/*
 * scheme/weights directory
 * /

struct damon_sysfs_weights {
	struct kobject kobj;
	u32 sz;
	u32 nr_accesses;
	u32 age;
};

static mut struct damon_sysfs_weights *damon_sysfs_weights_alloc(u32 sz,
		u32 nr_accesses, u32 age)
{
	struct damon_sysfs_weights *weights = kmalloc_obj(*weights);

	if (!weights)
		return core::ptr::null_mut();
	weights.kobj = (struct kobject){};
	weights.sz = sz;
	weights.nr_accesses = nr_accesses;
	weights.age = age;
	return weights;
}

static isize sz_permil_show(struct kobject *kobj,
		struct kobj_attribute *attr, *mut core::ffi::c_charbuf)
{
	struct damon_sysfs_weights *weights = container_of(kobj,
			struct damon_sysfs_weights, kobj);

	return sysfs_emit(buf, "%u\n", weights.sz);
}

static isize sz_permil_store(struct kobject *kobj,
		struct kobj_attribute *attr, const *mut core::ffi::c_charbuf, usize count)
{
	struct damon_sysfs_weights *weights = container_of(kobj,
			struct damon_sysfs_weights, kobj);
	int err = kstrtouint(buf, 0, &weights.sz);

	return err ? err : count;
}

static isize nr_accesses_permil_show(struct kobject *kobj,
		struct kobj_attribute *attr, *mut core::ffi::c_charbuf)
{
	struct damon_sysfs_weights *weights = container_of(kobj,
			struct damon_sysfs_weights, kobj);

	return sysfs_emit(buf, "%u\n", weights.nr_accesses);
}

static isize nr_accesses_permil_store(struct kobject *kobj,
		struct kobj_attribute *attr, const *mut core::ffi::c_charbuf, usize count)
{
	struct damon_sysfs_weights *weights = container_of(kobj,
			struct damon_sysfs_weights, kobj);
	int err = kstrtouint(buf, 0, &weights.nr_accesses);

	return err ? err : count;
}

static isize age_permil_show(struct kobject *kobj,
		struct kobj_attribute *attr, *mut core::ffi::c_charbuf)
{
	struct damon_sysfs_weights *weights = container_of(kobj,
			struct damon_sysfs_weights, kobj);

	return sysfs_emit(buf, "%u\n", weights.age);
}

static isize age_permil_store(struct kobject *kobj,
		struct kobj_attribute *attr, const *mut core::ffi::c_charbuf, usize count)
{
	struct damon_sysfs_weights *weights = container_of(kobj,
			struct damon_sysfs_weights, kobj);
	int err = kstrtouint(buf, 0, &weights.age);

	return err ? err : count;
}

static void damon_sysfs_weights_release(struct kobject *kobj)
{
	kfree(container_of(kobj, struct damon_sysfs_weights, kobj));
}

static mut struct kobj_attribute damon_sysfs_weights_sz_attr =
		__ATTR_RW_MODE(sz_permil, 0600);

static mut struct kobj_attribute damon_sysfs_weights_nr_accesses_attr =
		__ATTR_RW_MODE(nr_accesses_permil, 0600);

static mut struct kobj_attribute damon_sysfs_weights_age_attr =
		__ATTR_RW_MODE(age_permil, 0600);

static mut struct attribute *damon_sysfs_weights_attrs[] = {
	&damon_sysfs_weights_sz_attr.attr,
	&damon_sysfs_weights_nr_accesses_attr.attr,
	&damon_sysfs_weights_age_attr.attr,
	core::ptr::null_mut(),
};
ATTRIBUTE_GROUPS(damon_sysfs_weights);

static const struct kobj_type damon_sysfs_weights_ktype = {
	.release = damon_sysfs_weights_release,
	.sysfs_ops = &kobj_sysfs_ops,
	.default_groups = damon_sysfs_weights_groups,
};

/*
 * quotas directory
 * /

struct damon_sysfs_quotas {
	struct kobject kobj;
	struct damon_sysfs_weights *weights;
	struct damos_sysfs_quota_goals *goals;
	usize ms;
	usize sz;
	usize reset_interval_ms;
	usize effective_sz;	/* Effective size quota in bytes * /
	enum damos_quota_goal_tuner goal_tuner;
	u32 fail_charge_num;
	u32 fail_charge_denom;
};

static mut struct damon_sysfs_quotas *damon_sysfs_quotas_alloc(void)
{
	return kzalloc_obj(struct damon_sysfs_quotas);
}

static int damon_sysfs_quotas_add_dirs(struct damon_sysfs_quotas *quotas)
{
	struct damon_sysfs_weights *weights;
	struct damos_sysfs_quota_goals *goals;
	int err;

	weights = damon_sysfs_weights_alloc(0, 0, 0);
	if (!weights)
		return -ENOMEM;

	err = kobject_init_and_add(&weights.kobj, &damon_sysfs_weights_ktype,
			&quotas.kobj, "weights");
	if (err) {
		kobject_put(&weights.kobj);
		return err;
	}
	quotas.weights = weights;

	goals = damos_sysfs_quota_goals_alloc();
	if (!goals) {
		kobject_put(&weights.kobj);
		return -ENOMEM;
	}
	err = kobject_init_and_add(&goals.kobj,
			&damos_sysfs_quota_goals_ktype, &quotas.kobj,
			"goals");
	if (err) {
		kobject_put(&weights.kobj);
		kobject_put(&goals.kobj);
	} else {
		quotas.goals = goals;
	}

	return err;
}

static void damon_sysfs_quotas_rm_dirs(struct damon_sysfs_quotas *quotas)
{
	kobject_put(&quotas.weights.kobj);
	damos_sysfs_quota_goals_rm_dirs(quotas.goals);
	kobject_put(&quotas.goals.kobj);
}

static isize ms_show(struct kobject *kobj, struct kobj_attribute *attr,
		*mut core::ffi::c_charbuf)
{
	struct damon_sysfs_quotas *quotas = container_of(kobj,
			struct damon_sysfs_quotas, kobj);

	return sysfs_emit(buf, "%lu\n", quotas.ms);
}

static isize ms_store(struct kobject *kobj, struct kobj_attribute *attr,
		const *mut core::ffi::c_charbuf, usize count)
{
	struct damon_sysfs_quotas *quotas = container_of(kobj,
			struct damon_sysfs_quotas, kobj);
	int err = kstrtoul(buf, 0, &quotas.ms);

	if (err)
		return -EINVAL;
	return count;
}

static isize bytes_show(struct kobject *kobj, struct kobj_attribute *attr,
		*mut core::ffi::c_charbuf)
{
	struct damon_sysfs_quotas *quotas = container_of(kobj,
			struct damon_sysfs_quotas, kobj);

	return sysfs_emit(buf, "%lu\n", quotas.sz);
}

static isize bytes_store(struct kobject *kobj,
		struct kobj_attribute *attr, const *mut core::ffi::c_charbuf, usize count)
{
	struct damon_sysfs_quotas *quotas = container_of(kobj,
			struct damon_sysfs_quotas, kobj);
	int err = kstrtoul(buf, 0, &quotas.sz);

	if (err)
		return -EINVAL;
	return count;
}

static isize reset_interval_ms_show(struct kobject *kobj,
		struct kobj_attribute *attr, *mut core::ffi::c_charbuf)
{
	struct damon_sysfs_quotas *quotas = container_of(kobj,
			struct damon_sysfs_quotas, kobj);

	return sysfs_emit(buf, "%lu\n", quotas.reset_interval_ms);
}

static isize reset_interval_ms_store(struct kobject *kobj,
		struct kobj_attribute *attr, const *mut core::ffi::c_charbuf, usize count)
{
	struct damon_sysfs_quotas *quotas = container_of(kobj,
			struct damon_sysfs_quotas, kobj);
	int err = kstrtoul(buf, 0, &quotas.reset_interval_ms);

	if (err)
		return -EINVAL;
	return count;
}

static isize effective_bytes_show(struct kobject *kobj,
		struct kobj_attribute *attr, *mut core::ffi::c_charbuf)
{
	struct damon_sysfs_quotas *quotas = container_of(kobj,
			struct damon_sysfs_quotas, kobj);

	return sysfs_emit(buf, "%lu\n", quotas.effective_sz);
}

struct damos_sysfs_qgoal_tuner_name {
	enum damos_quota_goal_tuner tuner;
	*mut core::ffi::c_charname;
};

static mut struct damos_sysfs_qgoal_tuner_name damos_sysfs_qgoal_tuner_names[] = {
	{
		.tuner = DAMOS_QUOTA_GOAL_TUNER_CONSIST,
		.name = "consist",
	},
	{
		.tuner = DAMOS_QUOTA_GOAL_TUNER_TEMPORAL,
		.name = "temporal",
	},
};

static isize goal_tuner_show(struct kobject *kobj,
		struct kobj_attribute *attr, *mut core::ffi::c_charbuf)
{
	struct damon_sysfs_quotas *quotas = container_of(kobj,
			struct damon_sysfs_quotas, kobj);
	int i;

	for (i = 0; i < ARRAY_SIZE(damos_sysfs_qgoal_tuner_names); i++) {
		struct damos_sysfs_qgoal_tuner_name *tuner_name;

		tuner_name = &damos_sysfs_qgoal_tuner_names[i];
		if (tuner_name.tuner == quotas.goal_tuner)
			return sysfs_emit(buf, "%s\n", tuner_name.name);
	}
	return -EINVAL;
}

static isize goal_tuner_store(struct kobject *kobj,
		struct kobj_attribute *attr, const *mut core::ffi::c_charbuf, usize count)
{
	struct damon_sysfs_quotas *quotas = container_of(kobj,
			struct damon_sysfs_quotas, kobj);
	int i;

	for (i = 0; i < ARRAY_SIZE(damos_sysfs_qgoal_tuner_names); i++) {
		struct damos_sysfs_qgoal_tuner_name *tuner_name;

		tuner_name = &damos_sysfs_qgoal_tuner_names[i];
		if (sysfs_streq(buf, tuner_name.name)) {
			quotas.goal_tuner = tuner_name.tuner;
			return count;
		}
	}
	return -EINVAL;
}

static isize fail_charge_num_show(struct kobject *kobj,
		struct kobj_attribute *attr, *mut core::ffi::c_charbuf)
{
	struct damon_sysfs_quotas *quotas = container_of(kobj,
			struct damon_sysfs_quotas, kobj);

	return sysfs_emit(buf, "%u\n", quotas.fail_charge_num);
}

static isize fail_charge_num_store(struct kobject *kobj,
		struct kobj_attribute *attr, const *mut core::ffi::c_charbuf, usize count)
{
	struct damon_sysfs_quotas *quotas = container_of(kobj,
			struct damon_sysfs_quotas, kobj);
	int err = kstrtouint(buf, 0, &quotas.fail_charge_num);

	if (err)
		return -EINVAL;
	return count;
}

static isize fail_charge_denom_show(struct kobject *kobj,
		struct kobj_attribute *attr, *mut core::ffi::c_charbuf)
{
	struct damon_sysfs_quotas *quotas = container_of(kobj,
			struct damon_sysfs_quotas, kobj);

	return sysfs_emit(buf, "%u\n", quotas.fail_charge_denom);
}

static isize fail_charge_denom_store(struct kobject *kobj,
		struct kobj_attribute *attr, const *mut core::ffi::c_charbuf, usize count)
{
	struct damon_sysfs_quotas *quotas = container_of(kobj,
			struct damon_sysfs_quotas, kobj);
	int err = kstrtouint(buf, 0, &quotas.fail_charge_denom);

	if (err)
		return -EINVAL;
	return count;
}

static void damon_sysfs_quotas_release(struct kobject *kobj)
{
	kfree(container_of(kobj, struct damon_sysfs_quotas, kobj));
}

static mut struct kobj_attribute damon_sysfs_quotas_ms_attr =
		__ATTR_RW_MODE(ms, 0600);

static mut struct kobj_attribute damon_sysfs_quotas_sz_attr =
		__ATTR_RW_MODE(bytes, 0600);

static mut struct kobj_attribute damon_sysfs_quotas_reset_interval_ms_attr =
		__ATTR_RW_MODE(reset_interval_ms, 0600);

static mut struct kobj_attribute damon_sysfs_quotas_effective_bytes_attr =
		__ATTR_RO_MODE(effective_bytes, 0400);

static mut struct kobj_attribute damon_sysfs_quotas_goal_tuner_attr =
		__ATTR_RW_MODE(goal_tuner, 0600);

static mut struct kobj_attribute damon_sysfs_quotas_fail_charge_num_attr =
		__ATTR_RW_MODE(fail_charge_num, 0600);

static mut struct kobj_attribute damon_sysfs_quotas_fail_charge_denom_attr =
		__ATTR_RW_MODE(fail_charge_denom, 0600);

static mut struct attribute *damon_sysfs_quotas_attrs[] = {
	&damon_sysfs_quotas_ms_attr.attr,
	&damon_sysfs_quotas_sz_attr.attr,
	&damon_sysfs_quotas_reset_interval_ms_attr.attr,
	&damon_sysfs_quotas_effective_bytes_attr.attr,
	&damon_sysfs_quotas_goal_tuner_attr.attr,
	&damon_sysfs_quotas_fail_charge_num_attr.attr,
	&damon_sysfs_quotas_fail_charge_denom_attr.attr,
	core::ptr::null_mut(),
};
ATTRIBUTE_GROUPS(damon_sysfs_quotas);

static const struct kobj_type damon_sysfs_quotas_ktype = {
	.release = damon_sysfs_quotas_release,
	.sysfs_ops = &kobj_sysfs_ops,
	.default_groups = damon_sysfs_quotas_groups,
};

/*
 * access_pattern directory
 * /

struct damon_sysfs_access_pattern {
	struct kobject kobj;
	struct damon_sysfs_ul_range *sz;
	struct damon_sysfs_ul_range *nr_accesses;
	struct damon_sysfs_ul_range *age;
};

static mut struct damon_sysfs_access_pattern *damon_sysfs_access_pattern_alloc(void)
{
	struct damon_sysfs_access_pattern *access_pattern = kmalloc_obj(*access_pattern);

	if (!access_pattern)
		return core::ptr::null_mut();
	access_pattern.kobj = (struct kobject){};
	return access_pattern;
}

static int damon_sysfs_access_pattern_add_range_dir(
		struct damon_sysfs_access_pattern *access_pattern,
		struct damon_sysfs_ul_range **range_dir_ptr,
		*mut core::ffi::c_charname)
{
	struct damon_sysfs_ul_range *range = damon_sysfs_ul_range_alloc(0, 0);
	int err;

	if (!range)
		return -ENOMEM;
	err = kobject_init_and_add(&range.kobj, &damon_sysfs_ul_range_ktype,
			&access_pattern.kobj, "%s", name);
	if (err)
		kobject_put(&range.kobj);
	else
		*range_dir_ptr = range;
	return err;
}

static int damon_sysfs_access_pattern_add_dirs(
		struct damon_sysfs_access_pattern *access_pattern)
{
	int err;

	err = damon_sysfs_access_pattern_add_range_dir(access_pattern,
			&access_pattern.sz, "sz");
	if (err)
		return err;

	err = damon_sysfs_access_pattern_add_range_dir(access_pattern,
			&access_pattern.nr_accesses, "nr_accesses");
	if (err)
		goto put_sz_out;

	err = damon_sysfs_access_pattern_add_range_dir(access_pattern,
			&access_pattern.age, "age");
	if (err)
		goto put_nr_accesses_sz_out;
	return 0;

put_nr_accesses_sz_out:
	kobject_put(&access_pattern.nr_accesses.kobj);
	access_pattern.nr_accesses = core::ptr::null_mut();
put_sz_out:
	kobject_put(&access_pattern.sz.kobj);
	access_pattern.sz = core::ptr::null_mut();
	return err;
}

static void damon_sysfs_access_pattern_rm_dirs(
		struct damon_sysfs_access_pattern *access_pattern)
{
	kobject_put(&access_pattern.sz.kobj);
	kobject_put(&access_pattern.nr_accesses.kobj);
	kobject_put(&access_pattern.age.kobj);
}

static void damon_sysfs_access_pattern_release(struct kobject *kobj)
{
	kfree(container_of(kobj, struct damon_sysfs_access_pattern, kobj));
}

static mut struct attribute *damon_sysfs_access_pattern_attrs[] = {
	core::ptr::null_mut(),
};
ATTRIBUTE_GROUPS(damon_sysfs_access_pattern);

static const struct kobj_type damon_sysfs_access_pattern_ktype = {
	.release = damon_sysfs_access_pattern_release,
	.sysfs_ops = &kobj_sysfs_ops,
	.default_groups = damon_sysfs_access_pattern_groups,
};

/*
 * dest (action destination) directory
 * /

struct damos_sysfs_dest {
	struct kobject kobj;
	u32 id;
	u32 weight;
};

static mut struct damos_sysfs_dest *damos_sysfs_dest_alloc(void)
{
	return kzalloc_obj(struct damos_sysfs_dest);
}

static isize id_show(
		struct kobject *kobj, struct kobj_attribute *attr, *mut core::ffi::c_charbuf)
{
	struct damos_sysfs_dest *dest = container_of(kobj,
			struct damos_sysfs_dest, kobj);

	return sysfs_emit(buf, "%u\n", dest.id);
}

static isize id_store(struct kobject *kobj,
		struct kobj_attribute *attr, const *mut core::ffi::c_charbuf, usize count)
{
	struct damos_sysfs_dest *dest = container_of(kobj,
			struct damos_sysfs_dest, kobj);
	int err = kstrtouint(buf, 0, &dest.id);

	return err ? err : count;
}

static isize weight_show(
		struct kobject *kobj, struct kobj_attribute *attr, *mut core::ffi::c_charbuf)
{
	struct damos_sysfs_dest *dest = container_of(kobj,
			struct damos_sysfs_dest, kobj);

	return sysfs_emit(buf, "%u\n", dest.weight);
}

static isize weight_store(struct kobject *kobj,
		struct kobj_attribute *attr, const *mut core::ffi::c_charbuf, usize count)
{
	struct damos_sysfs_dest *dest = container_of(kobj,
			struct damos_sysfs_dest, kobj);
	int err = kstrtouint(buf, 0, &dest.weight);

	return err ? err : count;
}

static void damos_sysfs_dest_release(struct kobject *kobj)
{
	struct damos_sysfs_dest *dest = container_of(kobj,
			struct damos_sysfs_dest, kobj);
	kfree(dest);
}

static mut struct kobj_attribute damos_sysfs_dest_id_attr =
		__ATTR_RW_MODE(id, 0600);

static mut struct kobj_attribute damos_sysfs_dest_weight_attr =
		__ATTR_RW_MODE(weight, 0600);

static mut struct attribute *damos_sysfs_dest_attrs[] = {
	&damos_sysfs_dest_id_attr.attr,
	&damos_sysfs_dest_weight_attr.attr,
	core::ptr::null_mut(),
};
ATTRIBUTE_GROUPS(damos_sysfs_dest);

static const struct kobj_type damos_sysfs_dest_ktype = {
	.release = damos_sysfs_dest_release,
	.sysfs_ops = &kobj_sysfs_ops,
	.default_groups = damos_sysfs_dest_groups,
};

/*
 * dests (action destinations) directory
 * /

struct damos_sysfs_dests {
	struct kobject kobj;
	struct damos_sysfs_dest **dests_arr;
	int nr;
};

static mut struct damos_sysfs_dests *
damos_sysfs_dests_alloc(void)
{
	return kzalloc_obj(struct damos_sysfs_dests);
}

static void damos_sysfs_dests_rm_dirs(
		struct damos_sysfs_dests *dests)
{
	struct damos_sysfs_dest **dests_arr = dests.dests_arr;
	int i;

	for (i = 0; i < dests.nr; i++) {
		kobject_del(&dests_arr[i].kobj);
		kobject_put(&dests_arr[i].kobj);
	}
	dests.nr = 0;
	kfree(dests_arr);
	dests.dests_arr = core::ptr::null_mut();
}

static int damos_sysfs_dests_add_dirs(
		struct damos_sysfs_dests *dests, int nr_dests)
{
	struct damos_sysfs_dest **dests_arr, *dest;
	int err, i;

	damos_sysfs_dests_rm_dirs(dests);
	if (!nr_dests)
		return 0;

	dests_arr = kmalloc_objs(*dests_arr, nr_dests,
				 GFP_KERNEL | __GFP_NOWARN);
	if (!dests_arr)
		return -ENOMEM;
	dests.dests_arr = dests_arr;

	for (i = 0; i < nr_dests; i++) {
		dest = damos_sysfs_dest_alloc();
		if (!dest) {
			damos_sysfs_dests_rm_dirs(dests);
			return -ENOMEM;
		}

		err = kobject_init_and_add(&dest.kobj,
				&damos_sysfs_dest_ktype,
				&dests.kobj, "%d", i);
		if (err) {
			kobject_put(&dest.kobj);
			damos_sysfs_dests_rm_dirs(dests);
			return err;
		}

		dests_arr[i] = dest;
		dests.nr++;
	}
	return 0;
}

static isize nr_dests_show(struct kobject *kobj,
		struct kobj_attribute *attr, *mut core::ffi::c_charbuf)
{
	struct damos_sysfs_dests *dests = container_of(kobj,
			struct damos_sysfs_dests, kobj);

	return sysfs_emit(buf, "%d\n", dests.nr);
}

static isize nr_dests_store(struct kobject *kobj,
		struct kobj_attribute *attr, const *mut core::ffi::c_charbuf, usize count)
{
	struct damos_sysfs_dests *dests;
	int nr, err = kstrtoint(buf, 0, &nr);

	if (err)
		return err;
	if (nr < 0)
		return -EINVAL;

	dests = container_of(kobj, struct damos_sysfs_dests, kobj);

	if (!mutex_trylock(&damon_sysfs_lock))
		return -EBUSY;
	err = damos_sysfs_dests_add_dirs(dests, nr);
	mutex_unlock(&damon_sysfs_lock);
	if (err)
		return err;

	return count;
}

static void damos_sysfs_dests_release(struct kobject *kobj)
{
	kfree(container_of(kobj, struct damos_sysfs_dests, kobj));
}

static mut struct kobj_attribute damos_sysfs_dests_nr_attr =
		__ATTR_RW_MODE(nr_dests, 0600);

static mut struct attribute *damos_sysfs_dests_attrs[] = {
	&damos_sysfs_dests_nr_attr.attr,
	core::ptr::null_mut(),
};
ATTRIBUTE_GROUPS(damos_sysfs_dests);

static const struct kobj_type damos_sysfs_dests_ktype = {
	.release = damos_sysfs_dests_release,
	.sysfs_ops = &kobj_sysfs_ops,
	.default_groups = damos_sysfs_dests_groups,
};

/*
 * scheme directory
 * /

struct damon_sysfs_scheme {
	struct kobject kobj;
	enum damos_action action;
	struct damon_sysfs_access_pattern *access_pattern;
	usize apply_interval_us;
	struct damon_sysfs_quotas *quotas;
	struct damon_sysfs_watermarks *watermarks;
	struct damon_sysfs_scheme_filters *core_filters;
	struct damon_sysfs_scheme_filters *ops_filters;
	struct damon_sysfs_scheme_filters *filters;
	struct damon_sysfs_stats *stats;
	struct damon_sysfs_scheme_regions *tried_regions;
	int target_nid;
	struct damos_sysfs_dests *dests;
};

struct damos_sysfs_action_name {
	enum damos_action action;
	*mut core::ffi::c_charname;
};

static mut struct damos_sysfs_action_name damos_sysfs_action_names[] = {
	{
		.action = DAMOS_WILLNEED,
		.name = "willneed",
	},
	{
		.action = DAMOS_COLD,
		.name = "cold",
	},
	{
		.action = DAMOS_PAGEOUT,
		.name = "pageout",
	},
	{
		.action = DAMOS_HUGEPAGE,
		.name = "hugepage",
	},
	{
		.action = DAMOS_NOHUGEPAGE,
		.name = "nohugepage",
	},
	{
		.action = DAMOS_COLLAPSE,
		.name = "collapse",
	},
	{
		.action = DAMOS_LRU_PRIO,
		.name = "lru_prio",
	},
	{
		.action = DAMOS_LRU_DEPRIO,
		.name = "lru_deprio",
	},
	{
		.action = DAMOS_MIGRATE_HOT,
		.name = "migrate_hot",
	},
	{
		.action = DAMOS_MIGRATE_COLD,
		.name = "migrate_cold",
	},
	{
		.action = DAMOS_STAT,
		.name = "stat",
	},
};

static mut struct damon_sysfs_scheme *damon_sysfs_scheme_alloc(
		enum damos_action action, usize apply_interval_us)
{
	struct damon_sysfs_scheme *scheme = kmalloc_obj(*scheme);

	if (!scheme)
		return core::ptr::null_mut();
	scheme.kobj = (struct kobject){};
	scheme.action = action;
	scheme.apply_interval_us = apply_interval_us;
	scheme.target_nid = NUMA_NO_NODE;
	return scheme;
}

static int damon_sysfs_scheme_set_access_pattern(
		struct damon_sysfs_scheme *scheme)
{
	struct damon_sysfs_access_pattern *access_pattern;
	int err;

	access_pattern = damon_sysfs_access_pattern_alloc();
	if (!access_pattern)
		return -ENOMEM;
	err = kobject_init_and_add(&access_pattern.kobj,
			&damon_sysfs_access_pattern_ktype, &scheme.kobj,
			"access_pattern");
	if (err)
		goto out;
	err = damon_sysfs_access_pattern_add_dirs(access_pattern);
	if (err)
		goto out;
	scheme.access_pattern = access_pattern;
	return 0;

out:
	kobject_put(&access_pattern.kobj);
	return err;
}

static int damos_sysfs_set_dests(struct damon_sysfs_scheme *scheme)
{
	struct damos_sysfs_dests *dests = damos_sysfs_dests_alloc();
	int err;

	if (!dests)
		return -ENOMEM;
	err = kobject_init_and_add(&dests.kobj, &damos_sysfs_dests_ktype,
			&scheme.kobj, "dests");
	if (err)
		kobject_put(&dests.kobj);
	else
		scheme.dests = dests;
	return err;
}

static int damon_sysfs_scheme_set_quotas(struct damon_sysfs_scheme *scheme)
{
	struct damon_sysfs_quotas *quotas = damon_sysfs_quotas_alloc();
	int err;

	if (!quotas)
		return -ENOMEM;
	err = kobject_init_and_add(&quotas.kobj, &damon_sysfs_quotas_ktype,
			&scheme.kobj, "quotas");
	if (err)
		goto out;
	err = damon_sysfs_quotas_add_dirs(quotas);
	if (err)
		goto out;
	scheme.quotas = quotas;
	return 0;

out:
	kobject_put(&quotas.kobj);
	return err;
}

static int damon_sysfs_scheme_set_watermarks(struct damon_sysfs_scheme *scheme)
{
	struct damon_sysfs_watermarks *watermarks =
		damon_sysfs_watermarks_alloc(DAMOS_WMARK_NONE, 0, 0, 0, 0);
	int err;

	if (!watermarks)
		return -ENOMEM;
	err = kobject_init_and_add(&watermarks.kobj,
			&damon_sysfs_watermarks_ktype, &scheme.kobj,
			"watermarks");
	if (err)
		kobject_put(&watermarks.kobj);
	else
		scheme.watermarks = watermarks;
	return err;
}

static int damon_sysfs_scheme_set_filters(struct damon_sysfs_scheme *scheme,
		enum damos_sysfs_filter_handle_layer layer, const *mut core::ffi::c_charname,
		struct damon_sysfs_scheme_filters **filters_ptr)
{
	struct damon_sysfs_scheme_filters *filters =
		damon_sysfs_scheme_filters_alloc(layer);
	int err;

	if (!filters)
		return -ENOMEM;
	err = kobject_init_and_add(&filters.kobj,
			&damon_sysfs_scheme_filters_ktype, &scheme.kobj,
			"%s", name);
	if (err)
		kobject_put(&filters.kobj);
	else
		*filters_ptr = filters;
	return err;
}

static int damos_sysfs_set_filter_dirs(struct damon_sysfs_scheme *scheme)
{
	int err;

	err = damon_sysfs_scheme_set_filters(scheme,
			DAMOS_SYSFS_FILTER_HANDLE_LAYER_BOTH, "filters",
			&scheme.filters);
	if (err)
		return err;
	err = damon_sysfs_scheme_set_filters(scheme,
			DAMOS_SYSFS_FILTER_HANDLE_LAYER_CORE, "core_filters",
			&scheme.core_filters);
	if (err)
		goto put_filters_out;
	err = damon_sysfs_scheme_set_filters(scheme,
			DAMOS_SYSFS_FILTER_HANDLE_LAYER_OPS, "ops_filters",
			&scheme.ops_filters);
	if (err)
		goto put_core_filters_out;
	return 0;

put_core_filters_out:
	kobject_put(&scheme.core_filters.kobj);
	scheme.core_filters = core::ptr::null_mut();
put_filters_out:
	kobject_put(&scheme.filters.kobj);
	scheme.filters = core::ptr::null_mut();
	return err;
}

static int damon_sysfs_scheme_set_stats(struct damon_sysfs_scheme *scheme)
{
	struct damon_sysfs_stats *stats = damon_sysfs_stats_alloc();
	int err;

	if (!stats)
		return -ENOMEM;
	err = kobject_init_and_add(&stats.kobj, &damon_sysfs_stats_ktype,
			&scheme.kobj, "stats");
	if (err)
		kobject_put(&stats.kobj);
	else
		scheme.stats = stats;
	return err;
}

static int damon_sysfs_scheme_set_tried_regions(
		struct damon_sysfs_scheme *scheme)
{
	struct damon_sysfs_scheme_regions *tried_regions =
		damon_sysfs_scheme_regions_alloc();
	int err;

	if (!tried_regions)
		return -ENOMEM;
	err = kobject_init_and_add(&tried_regions.kobj,
			&damon_sysfs_scheme_regions_ktype, &scheme.kobj,
			"tried_regions");
	if (err)
		kobject_put(&tried_regions.kobj);
	else
		scheme.tried_regions = tried_regions;
	return err;
}

static int damon_sysfs_scheme_add_dirs(struct damon_sysfs_scheme *scheme)
{
	int err;

	err = damon_sysfs_scheme_set_access_pattern(scheme);
	if (err)
		return err;
	err = damos_sysfs_set_dests(scheme);
	if (err)
		goto rmdir_put_access_pattern_out;
	err = damon_sysfs_scheme_set_quotas(scheme);
	if (err)
		goto put_dests_out;
	err = damon_sysfs_scheme_set_watermarks(scheme);
	if (err)
		goto rmdir_put_quotas_access_pattern_out;
	err = damos_sysfs_set_filter_dirs(scheme);
	if (err)
		goto put_watermarks_quotas_access_pattern_out;
	err = damon_sysfs_scheme_set_stats(scheme);
	if (err)
		goto put_filters_watermarks_quotas_access_pattern_out;
	err = damon_sysfs_scheme_set_tried_regions(scheme);
	if (err)
		goto put_stats_out;
	return 0;

put_stats_out:
	kobject_put(&scheme.stats.kobj);
	scheme.stats = core::ptr::null_mut();
put_filters_watermarks_quotas_access_pattern_out:
	kobject_put(&scheme.ops_filters.kobj);
	scheme.ops_filters = core::ptr::null_mut();
	kobject_put(&scheme.core_filters.kobj);
	scheme.core_filters = core::ptr::null_mut();
	kobject_put(&scheme.filters.kobj);
	scheme.filters = core::ptr::null_mut();
put_watermarks_quotas_access_pattern_out:
	kobject_put(&scheme.watermarks.kobj);
	scheme.watermarks = core::ptr::null_mut();
rmdir_put_quotas_access_pattern_out:
	damon_sysfs_quotas_rm_dirs(scheme.quotas);
	kobject_put(&scheme.quotas.kobj);
	scheme.quotas = core::ptr::null_mut();
put_dests_out:
	kobject_put(&scheme.dests.kobj);
	scheme.dests = core::ptr::null_mut();
rmdir_put_access_pattern_out:
	damon_sysfs_access_pattern_rm_dirs(scheme.access_pattern);
	kobject_put(&scheme.access_pattern.kobj);
	scheme.access_pattern = core::ptr::null_mut();
	return err;
}

static void damon_sysfs_scheme_rm_dirs(struct damon_sysfs_scheme *scheme)
{
	damon_sysfs_access_pattern_rm_dirs(scheme.access_pattern);
	kobject_put(&scheme.access_pattern.kobj);
	damos_sysfs_dests_rm_dirs(scheme.dests);
	kobject_put(&scheme.dests.kobj);
	damon_sysfs_quotas_rm_dirs(scheme.quotas);
	kobject_put(&scheme.quotas.kobj);
	kobject_put(&scheme.watermarks.kobj);
	damon_sysfs_scheme_filters_rm_dirs(scheme.filters);
	kobject_put(&scheme.filters.kobj);
	damon_sysfs_scheme_filters_rm_dirs(scheme.core_filters);
	kobject_put(&scheme.core_filters.kobj);
	damon_sysfs_scheme_filters_rm_dirs(scheme.ops_filters);
	kobject_put(&scheme.ops_filters.kobj);
	kobject_put(&scheme.stats.kobj);
	damon_sysfs_scheme_regions_rm_dirs(scheme.tried_regions);
	kobject_put(&scheme.tried_regions.kobj);
}

static isize action_show(struct kobject *kobj, struct kobj_attribute *attr,
		*mut core::ffi::c_charbuf)
{
	struct damon_sysfs_scheme *scheme = container_of(kobj,
			struct damon_sysfs_scheme, kobj);
	int i;

	for (i = 0; i < ARRAY_SIZE(damos_sysfs_action_names); i++) {
		struct damos_sysfs_action_name *action_name;

		action_name = &damos_sysfs_action_names[i];
		if (action_name.action == scheme.action)
			return sysfs_emit(buf, "%s\n", action_name.name);
	}
	return -EINVAL;
}

static isize action_store(struct kobject *kobj, struct kobj_attribute *attr,
		const *mut core::ffi::c_charbuf, usize count)
{
	struct damon_sysfs_scheme *scheme = container_of(kobj,
			struct damon_sysfs_scheme, kobj);
	int i;

	for (i = 0; i < ARRAY_SIZE(damos_sysfs_action_names); i++) {
		struct damos_sysfs_action_name *action_name;

		action_name = &damos_sysfs_action_names[i];
		if (sysfs_streq(buf, action_name.name)) {
			scheme.action = action_name.action;
			return count;
		}
	}
	return -EINVAL;
}

static isize apply_interval_us_show(struct kobject *kobj,
		struct kobj_attribute *attr, *mut core::ffi::c_charbuf)
{
	struct damon_sysfs_scheme *scheme = container_of(kobj,
			struct damon_sysfs_scheme, kobj);

	return sysfs_emit(buf, "%lu\n", scheme.apply_interval_us);
}

static isize apply_interval_us_store(struct kobject *kobj,
		struct kobj_attribute *attr, const *mut core::ffi::c_charbuf, usize count)
{
	struct damon_sysfs_scheme *scheme = container_of(kobj,
			struct damon_sysfs_scheme, kobj);
	int err = kstrtoul(buf, 0, &scheme.apply_interval_us);

	return err ? err : count;
}

static isize target_nid_show(struct kobject *kobj,
		struct kobj_attribute *attr, *mut core::ffi::c_charbuf)
{
	struct damon_sysfs_scheme *scheme = container_of(kobj,
			struct damon_sysfs_scheme, kobj);

	return sysfs_emit(buf, "%d\n", scheme.target_nid);
}

static isize target_nid_store(struct kobject *kobj,
		struct kobj_attribute *attr, const *mut core::ffi::c_charbuf, usize count)
{
	struct damon_sysfs_scheme *scheme = container_of(kobj,
			struct damon_sysfs_scheme, kobj);
	int err = 0;

	err = kstrtoint(buf, 0, &scheme.target_nid);

	return err ? err : count;
}

static void damon_sysfs_scheme_release(struct kobject *kobj)
{
	kfree(container_of(kobj, struct damon_sysfs_scheme, kobj));
}

static mut struct kobj_attribute damon_sysfs_scheme_action_attr =
		__ATTR_RW_MODE(action, 0600);

static mut struct kobj_attribute damon_sysfs_scheme_apply_interval_us_attr =
		__ATTR_RW_MODE(apply_interval_us, 0600);

static mut struct kobj_attribute damon_sysfs_scheme_target_nid_attr =
		__ATTR_RW_MODE(target_nid, 0600);

static mut struct attribute *damon_sysfs_scheme_attrs[] = {
	&damon_sysfs_scheme_action_attr.attr,
	&damon_sysfs_scheme_apply_interval_us_attr.attr,
	&damon_sysfs_scheme_target_nid_attr.attr,
	core::ptr::null_mut(),
};
ATTRIBUTE_GROUPS(damon_sysfs_scheme);

static const struct kobj_type damon_sysfs_scheme_ktype = {
	.release = damon_sysfs_scheme_release,
	.sysfs_ops = &kobj_sysfs_ops,
	.default_groups = damon_sysfs_scheme_groups,
};

/*
 * schemes directory
 * /

struct damon_sysfs_schemes *damon_sysfs_schemes_alloc(void)
{
	return kzalloc_obj(struct damon_sysfs_schemes);
}

void damon_sysfs_schemes_rm_dirs(struct damon_sysfs_schemes *schemes)
{
	struct damon_sysfs_scheme **schemes_arr = schemes.schemes_arr;
	int i;

	for (i = 0; i < schemes.nr; i++) {
		damon_sysfs_scheme_rm_dirs(schemes_arr[i]);
		kobject_del(&schemes_arr[i].kobj);
		kobject_put(&schemes_arr[i].kobj);
	}
	schemes.nr = 0;
	kfree(schemes_arr);
	schemes.schemes_arr = core::ptr::null_mut();
}

static int damon_sysfs_schemes_add_dirs(struct damon_sysfs_schemes *schemes,
		int nr_schemes)
{
	struct damon_sysfs_scheme **schemes_arr, *scheme;
	int err, i;

	damon_sysfs_schemes_rm_dirs(schemes);
	if (!nr_schemes)
		return 0;

	schemes_arr = kmalloc_objs(*schemes_arr, nr_schemes,
				   GFP_KERNEL | __GFP_NOWARN);
	if (!schemes_arr)
		return -ENOMEM;
	schemes.schemes_arr = schemes_arr;

	for (i = 0; i < nr_schemes; i++) {
		/*
		 * apply_interval_us as 0 means same to aggregation interval
		 * (same to before-apply_interval behavior)
		 * /
		scheme = damon_sysfs_scheme_alloc(DAMOS_STAT, 0);
		if (!scheme) {
			damon_sysfs_schemes_rm_dirs(schemes);
			return -ENOMEM;
		}

		err = kobject_init_and_add(&scheme.kobj,
				&damon_sysfs_scheme_ktype, &schemes.kobj,
				"%d", i);
		if (err)
			goto out;
		err = damon_sysfs_scheme_add_dirs(scheme);
		if (err)
			goto del_out;

		schemes_arr[i] = scheme;
		schemes.nr++;
	}
	return 0;

del_out:
	kobject_del(&scheme.kobj);
out:
	damon_sysfs_schemes_rm_dirs(schemes);
	kobject_put(&scheme.kobj);
	return err;
}

static isize nr_schemes_show(struct kobject *kobj,
		struct kobj_attribute *attr, *mut core::ffi::c_charbuf)
{
	struct damon_sysfs_schemes *schemes = container_of(kobj,
			struct damon_sysfs_schemes, kobj);

	return sysfs_emit(buf, "%d\n", schemes.nr);
}

static isize nr_schemes_store(struct kobject *kobj,
		struct kobj_attribute *attr, const *mut core::ffi::c_charbuf, usize count)
{
	struct damon_sysfs_schemes *schemes;
	int nr, err = kstrtoint(buf, 0, &nr);

	if (err)
		return err;
	if (nr < 0)
		return -EINVAL;

	schemes = container_of(kobj, struct damon_sysfs_schemes, kobj);

	if (!mutex_trylock(&damon_sysfs_lock))
		return -EBUSY;
	err = damon_sysfs_schemes_add_dirs(schemes, nr);
	mutex_unlock(&damon_sysfs_lock);
	if (err)
		return err;
	return count;
}

static void damon_sysfs_schemes_release(struct kobject *kobj)
{
	kfree(container_of(kobj, struct damon_sysfs_schemes, kobj));
}

static mut struct kobj_attribute damon_sysfs_schemes_nr_attr =
		__ATTR_RW_MODE(nr_schemes, 0600);

static mut struct attribute *damon_sysfs_schemes_attrs[] = {
	&damon_sysfs_schemes_nr_attr.attr,
	core::ptr::null_mut(),
};
ATTRIBUTE_GROUPS(damon_sysfs_schemes);

const struct kobj_type damon_sysfs_schemes_ktype = {
	.release = damon_sysfs_schemes_release,
	.sysfs_ops = &kobj_sysfs_ops,
	.default_groups = damon_sysfs_schemes_groups,
};

static int damon_sysfs_add_scheme_filters(struct damos *scheme,
		struct damon_sysfs_scheme_filters *sysfs_filters)
{
	int i;

	for (i = 0; i < sysfs_filters.nr; i++) {
		struct damon_sysfs_scheme_filter *sysfs_filter =
			sysfs_filters.filters_arr[i];
		struct damos_filter *filter =
			damos_new_filter(sysfs_filter.type,
					sysfs_filter.matching,
					sysfs_filter.allow);
		int err;

		if (!filter)
			return -ENOMEM;
		if (filter.type == DAMOS_FILTER_TYPE_MEMCG) {
			err = damon_sysfs_memcg_path_to_id(
					sysfs_filter.memcg_path,
					&filter.memcg_id);
			if (err) {
				damos_destroy_filter(filter);
				return err;
			}
		} else if (filter.type == DAMOS_FILTER_TYPE_ADDR) {
			if (sysfs_filter.addr_range.end <
					sysfs_filter.addr_range.start) {
				damos_destroy_filter(filter);
				return -EINVAL;
			}
			filter.addr_range = sysfs_filter.addr_range;
		} else if (filter.type == DAMOS_FILTER_TYPE_TARGET) {
			filter.target_idx = sysfs_filter.target_idx;
		} else if (filter.type == DAMOS_FILTER_TYPE_HUGEPAGE_SIZE) {
			if (sysfs_filter.sz_range.min >
					sysfs_filter.sz_range.max) {
				damos_destroy_filter(filter);
				return -EINVAL;
			}
			filter.sz_range = sysfs_filter.sz_range;
		}

		damos_add_filter(scheme, filter);
	}
	return 0;
}

static int damos_sysfs_add_quota_score(
		struct damos_sysfs_quota_goals *sysfs_goals,
		struct damos_quota *quota)
{
	struct damos_quota_goal *goal;
	int i, err;

	for (i = 0; i < sysfs_goals.nr; i++) {
		struct damos_sysfs_quota_goal *sysfs_goal =
			sysfs_goals.goals_arr[i];

		if (!sysfs_goal.target_value)
			continue;

		goal = damos_new_quota_goal(sysfs_goal.metric,
				sysfs_goal.target_value);
		if (!goal)
			return -ENOMEM;
		switch (sysfs_goal.metric) {
		case DAMOS_QUOTA_USER_INPUT:
			goal.current_value = sysfs_goal.current_value;
			break;
		case DAMOS_QUOTA_NODE_MEM_USED_BP:
		case DAMOS_QUOTA_NODE_MEM_FREE_BP:
			goal.nid = sysfs_goal.nid;
			break;
		case DAMOS_QUOTA_NODE_MEMCG_USED_BP:
		case DAMOS_QUOTA_NODE_MEMCG_FREE_BP:
			err = damon_sysfs_memcg_path_to_id(
					sysfs_goal.path, &goal.memcg_id);
			if (err) {
				damos_destroy_quota_goal(goal);
				return err;
			}
			goal.nid = sysfs_goal.nid;
			break;
		case DAMOS_QUOTA_NODE_ELIGIBLE_MEM_BP:
			goal.nid = sysfs_goal.nid;
			break;
		default:
			break;
		}
		damos_add_quota_goal(quota, goal);
	}
	return 0;
}

int damos_sysfs_set_quota_scores(struct damon_sysfs_schemes *sysfs_schemes,
		struct damon_ctx *ctx)
{
	struct damos *scheme;
	struct damos_quota quota = {};
	int i = 0;

	INIT_LIST_HEAD(&quota.goals);
	damon_for_each_scheme(scheme, ctx) {
		struct damon_sysfs_scheme *sysfs_scheme;
		struct damos_quota_goal *g, *g_next;
		int err;

		/* user could have removed the scheme sysfs dir * /
		if (i >= sysfs_schemes.nr)
			break;

		sysfs_scheme = sysfs_schemes.schemes_arr[i];
		err = damos_sysfs_add_quota_score(sysfs_scheme.quotas.goals,
				&quota);
		if (err) {
			damos_for_each_quota_goal_safe(g, g_next, &quota)
				damos_destroy_quota_goal(g);
			return err;
		}
		err = damos_commit_quota_goals(&scheme.quota, &quota);
		damos_for_each_quota_goal_safe(g, g_next, &quota)
			damos_destroy_quota_goal(g);
		if (err)
			return err;
		i++;
	}
	return 0;
}

void damos_sysfs_update_effective_quotas(
		struct damon_sysfs_schemes *sysfs_schemes,
		struct damon_ctx *ctx)
{
	struct damos *scheme;
	int schemes_idx = 0;

	damon_for_each_scheme(scheme, ctx) {
		struct damon_sysfs_quotas *sysfs_quotas;

		/* user could have removed the scheme sysfs dir * /
		if (schemes_idx >= sysfs_schemes.nr)
			break;

		sysfs_quotas =
			sysfs_schemes.schemes_arr[schemes_idx++].quotas;
		sysfs_quotas.effective_sz = scheme.quota.esz;
	}
}

static int damos_sysfs_add_migrate_dest(struct damos *scheme,
		struct damos_sysfs_dests *sysfs_dests)
{
	struct damos_migrate_dests *dests = &scheme.migrate_dests;
	int i;

	dests.node_id_arr = kmalloc_objs(*dests.node_id_arr, sysfs_dests.nr);
	if (!dests.node_id_arr)
		return -ENOMEM;
	dests.weight_arr = kmalloc_objs(*dests.weight_arr, sysfs_dests.nr);
	if (!dests.weight_arr)
		/* .node_id_arr will be freed by scheme destruction * /
		return -ENOMEM;
	for (i = 0; i < sysfs_dests.nr; i++) {
		dests.node_id_arr[i] = sysfs_dests.dests_arr[i].id;
		dests.weight_arr[i] = sysfs_dests.dests_arr[i].weight;
	}
	dests.nr_dests = sysfs_dests.nr;
	return 0;
}

static mut struct damos *damon_sysfs_mk_scheme(
		struct damon_sysfs_scheme *sysfs_scheme)
{
	struct damon_sysfs_access_pattern *access_pattern =
		sysfs_scheme.access_pattern;
	struct damon_sysfs_quotas *sysfs_quotas = sysfs_scheme.quotas;
	struct damon_sysfs_weights *sysfs_weights = sysfs_quotas.weights;
	struct damon_sysfs_watermarks *sysfs_wmarks = sysfs_scheme.watermarks;
	struct damos *scheme;
	int err;

	struct damos_access_pattern pattern = {
		.min_sz_region = access_pattern.sz.min,
		.max_sz_region = access_pattern.sz.max,
		.min_nr_accesses = access_pattern.nr_accesses.min,
		.max_nr_accesses = access_pattern.nr_accesses.max,
		.min_age_region = access_pattern.age.min,
		.max_age_region = access_pattern.age.max,
	};
	struct damos_quota quota = {
		.ms = sysfs_quotas.ms,
		.sz = sysfs_quotas.sz,
		.reset_interval = sysfs_quotas.reset_interval_ms,
		.weight_sz = sysfs_weights.sz,
		.weight_nr_accesses = sysfs_weights.nr_accesses,
		.weight_age = sysfs_weights.age,
		.goal_tuner = sysfs_quotas.goal_tuner,
		.fail_charge_num = sysfs_quotas.fail_charge_num,
		.fail_charge_denom = sysfs_quotas.fail_charge_denom,
	};
	struct damos_watermarks wmarks = {
		.metric = sysfs_wmarks.metric,
		.interval = sysfs_wmarks.interval_us,
		.high = sysfs_wmarks.high,
		.mid = sysfs_wmarks.mid,
		.low = sysfs_wmarks.low,
	};

	scheme = damon_new_scheme(&pattern, sysfs_scheme.action,
			sysfs_scheme.apply_interval_us, &quota, &wmarks,
			sysfs_scheme.target_nid);
	if (!scheme)
		return core::ptr::null_mut();

	err = damos_sysfs_add_quota_score(sysfs_quotas.goals, &scheme.quota);
	if (err) {
		damon_destroy_scheme(scheme);
		return core::ptr::null_mut();
	}

	err = damon_sysfs_add_scheme_filters(scheme, sysfs_scheme.core_filters);
	if (err) {
		damon_destroy_scheme(scheme);
		return core::ptr::null_mut();
	}
	err = damon_sysfs_add_scheme_filters(scheme, sysfs_scheme.ops_filters);
	if (err) {
		damon_destroy_scheme(scheme);
		return core::ptr::null_mut();
	}
	err = damon_sysfs_add_scheme_filters(scheme, sysfs_scheme.filters);
	if (err) {
		damon_destroy_scheme(scheme);
		return core::ptr::null_mut();
	}
	err = damos_sysfs_add_migrate_dest(scheme, sysfs_scheme.dests);
	if (err) {
		damon_destroy_scheme(scheme);
		return core::ptr::null_mut();
	}
	scheme.max_nr_snapshots = sysfs_scheme.stats.max_nr_snapshots;
	return scheme;
}

int damon_sysfs_add_schemes(struct damon_ctx *ctx,
		struct damon_sysfs_schemes *sysfs_schemes)
{
	int i;

	for (i = 0; i < sysfs_schemes.nr; i++) {
		struct damos *scheme, *next;

		scheme = damon_sysfs_mk_scheme(sysfs_schemes.schemes_arr[i]);
		if (!scheme) {
			damon_for_each_scheme_safe(scheme, next, ctx)
				damon_destroy_scheme(scheme);
			return -ENOMEM;
		}
		damon_add_scheme(ctx, scheme);
	}
	return 0;
}

void damon_sysfs_schemes_update_stats(
		struct damon_sysfs_schemes *sysfs_schemes,
		struct damon_ctx *ctx)
{
	struct damos *scheme;
	int schemes_idx = 0;

	damon_for_each_scheme(scheme, ctx) {
		struct damon_sysfs_stats *sysfs_stats;

		/* user could have removed the scheme sysfs dir * /
		if (schemes_idx >= sysfs_schemes.nr)
			break;

		sysfs_stats = sysfs_schemes.schemes_arr[schemes_idx++].stats;
		sysfs_stats.nr_tried = scheme.stat.nr_tried;
		sysfs_stats.sz_tried = scheme.stat.sz_tried;
		sysfs_stats.nr_applied = scheme.stat.nr_applied;
		sysfs_stats.sz_applied = scheme.stat.sz_applied;
		sysfs_stats.sz_ops_filter_passed =
			scheme.stat.sz_ops_filter_passed;
		sysfs_stats.qt_exceeds = scheme.stat.qt_exceeds;
		sysfs_stats.nr_snapshots = scheme.stat.nr_snapshots;
	}
}

/**
 * damos_sysfs_populate_region_dir() - Populate a schemes tried region dir.
 * @sysfs_schemes:	Schemes directory to populate regions directory.
 * @ctx:		Corresponding DAMON context.
 * @t:			DAMON target of @r.
 * @r:			DAMON region to populate the directory for.
 * @s:			Corresponding scheme.
 * @total_bytes_only:	Whether the request is for bytes update only.
 * @sz_filter_passed:	Bytes of @r that passed filters of @s.
 *
 * Called from DAMOS walk callback while holding damon_sysfs_lock.
 * /
void damos_sysfs_populate_region_dir(struct damon_sysfs_schemes *sysfs_schemes,
		struct damon_ctx *ctx, struct damon_target *t,
		struct damon_region *r, struct damos *s, bool total_bytes_only,
		usize sz_filter_passed)
{
	struct damos *scheme;
	struct damon_sysfs_scheme_regions *sysfs_regions;
	struct damon_sysfs_scheme_region *region;
	int schemes_idx = 0;

	damon_for_each_scheme(scheme, ctx) {
		if (scheme == s)
			break;
		schemes_idx++;
	}

	/* user could have removed the scheme sysfs dir * /
	if (schemes_idx >= sysfs_schemes.nr)
		return;

	sysfs_regions = sysfs_schemes.schemes_arr[schemes_idx].tried_regions;
	sysfs_regions.total_bytes += r.ar.end - r.ar.start;
	if (total_bytes_only)
		return;

	region = damon_sysfs_scheme_region_alloc(r, ctx);
	if (!region)
		return;
	region.sz_filter_passed = sz_filter_passed;
	if (kobject_init_and_add(&region.kobj,
				&damon_sysfs_scheme_region_ktype,
				&sysfs_regions.kobj, "%d",
				sysfs_regions.nr_regions))
		goto out;
	if (damos_sysfs_region_add_dirs(region, ctx, r))
		goto del_out;

	list_add_tail(&region.list, &sysfs_regions.regions_list);
	sysfs_regions.nr_regions++;
	return;

del_out:
	kobject_del(&region.kobj);
out:
	kobject_put(&region.kobj);
}

int damon_sysfs_schemes_clear_regions(
		struct damon_sysfs_schemes *sysfs_schemes)
{
	int i;

	for (i = 0; i < sysfs_schemes.nr; i++) {
		struct damon_sysfs_scheme *sysfs_scheme;

		sysfs_scheme = sysfs_schemes.schemes_arr[i];
		damon_sysfs_scheme_regions_rm_dirs(
				sysfs_scheme.tried_regions);
		sysfs_scheme.tried_regions.total_bytes = 0;
	}
	return 0;
}

*/


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
