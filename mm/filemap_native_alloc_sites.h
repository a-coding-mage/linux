/* SPDX-License-Identifier: GPL-2.0-only */
#ifndef RUST_FILEMAP_NATIVE_ALLOC_SITES_H
#define RUST_FILEMAP_NATIVE_ALLOC_SITES_H

/*
 * Preserve each original filemap.c allocation site's native tag and metadata.
 * DEFINE_ALLOC_TAG owns the configured layout, section, alignment and counters;
 * alloc_hooks_tag owns the native current->alloc_tag save/restore protocol.
 * Calling the noprof owner avoids an additional nested allocation-site tag.
 * The CODE_TAG_INIT override is confined to each leaf's definition.
 */
#pragma push_macro("CODE_TAG_INIT")
#undef CODE_TAG_INIT
#define CODE_TAG_INIT {                         \
	.modname = CT_MODULE_NAME,              \
	.function = "__filemap_get_folio_mpol",  \
	.filename = "mm/filemap.c",             \
	.lineno = 2018,                         \
	.flags = 0,                            \
}
struct folio *rust_filemap_alloc_folio_c2018(gfp_t gfp, unsigned int order,
		struct mempolicy *policy);
struct folio *rust_filemap_alloc_folio_c2018(gfp_t gfp, unsigned int order,
		struct mempolicy *policy)
{
	DEFINE_ALLOC_TAG(_alloc_tag);
	return alloc_hooks_tag(&_alloc_tag,
			filemap_alloc_folio_noprof(gfp, order, policy));
}
#pragma pop_macro("CODE_TAG_INIT")

#pragma push_macro("CODE_TAG_INIT")
#undef CODE_TAG_INIT
#define CODE_TAG_INIT {                         \
	.modname = CT_MODULE_NAME,              \
	.function = "filemap_create_folio",     \
	.filename = "mm/filemap.c",             \
	.lineno = 2630,                         \
	.flags = 0,                            \
}
struct folio *rust_filemap_alloc_folio_c2630(gfp_t gfp, unsigned int order,
		struct mempolicy *policy);
struct folio *rust_filemap_alloc_folio_c2630(gfp_t gfp, unsigned int order,
		struct mempolicy *policy)
{
	DEFINE_ALLOC_TAG(_alloc_tag);
	return alloc_hooks_tag(&_alloc_tag,
			filemap_alloc_folio_noprof(gfp, order, policy));
}
#pragma pop_macro("CODE_TAG_INIT")

#pragma push_macro("CODE_TAG_INIT")
#undef CODE_TAG_INIT
#define CODE_TAG_INIT {                         \
	.modname = CT_MODULE_NAME,              \
	.function = "do_read_cache_folio",      \
	.filename = "mm/filemap.c",             \
	.lineno = 4119,                         \
	.flags = 0,                            \
}
struct folio *rust_filemap_alloc_folio_c4119(gfp_t gfp, unsigned int order,
		struct mempolicy *policy);
struct folio *rust_filemap_alloc_folio_c4119(gfp_t gfp, unsigned int order,
		struct mempolicy *policy)
{
	DEFINE_ALLOC_TAG(_alloc_tag);
	return alloc_hooks_tag(&_alloc_tag,
			filemap_alloc_folio_noprof(gfp, order, policy));
}
#pragma pop_macro("CODE_TAG_INIT")

#endif
