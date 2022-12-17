// @generated
#![cfg(feature = "icu_properties")]
type DataStruct =
    <::icu_properties::provider::HyphenV1Marker as ::icu_provider::DataMarker>::Yokeable;
pub static DATA: litemap::LiteMap<&str, &DataStruct, &[(&str, &DataStruct)]> =
    litemap::LiteMap::from_sorted_store_unchecked(&[("und", UND)]);
static UND: &DataStruct =
    &::icu_properties::provider::PropertyCodePointSetV1::InversionList(unsafe {
        #[allow(unused_unsafe)]
        ::icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(
            unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    45u8, 0u8, 0u8, 0u8, 46u8, 0u8, 0u8, 0u8, 173u8, 0u8, 0u8, 0u8, 174u8, 0u8,
                    0u8, 0u8, 138u8, 5u8, 0u8, 0u8, 139u8, 5u8, 0u8, 0u8, 6u8, 24u8, 0u8, 0u8, 7u8,
                    24u8, 0u8, 0u8, 16u8, 32u8, 0u8, 0u8, 18u8, 32u8, 0u8, 0u8, 23u8, 46u8, 0u8,
                    0u8, 24u8, 46u8, 0u8, 0u8, 251u8, 48u8, 0u8, 0u8, 252u8, 48u8, 0u8, 0u8, 99u8,
                    254u8, 0u8, 0u8, 100u8, 254u8, 0u8, 0u8, 13u8, 255u8, 0u8, 0u8, 14u8, 255u8,
                    0u8, 0u8, 101u8, 255u8, 0u8, 0u8, 102u8, 255u8, 0u8, 0u8,
                ])
            },
            11usize,
        )
    });
