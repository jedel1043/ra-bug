// @generated
#![cfg(feature = "icu_properties")]
type DataStruct = < :: icu_properties :: provider :: FullCompositionExclusionV1Marker as :: icu_provider :: DataMarker > :: Yokeable ;
pub static DATA: litemap::LiteMap<&str, &DataStruct, &[(&str, &DataStruct)]> =
    litemap::LiteMap::from_sorted_store_unchecked(&[("und", UND)]);
static UND: &DataStruct =
    &::icu_properties::provider::PropertyCodePointSetV1::InversionList(unsafe {
        #[allow(unused_unsafe)]
        ::icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(
            unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    64u8, 3u8, 0u8, 0u8, 66u8, 3u8, 0u8, 0u8, 67u8, 3u8, 0u8, 0u8, 69u8, 3u8, 0u8,
                    0u8, 116u8, 3u8, 0u8, 0u8, 117u8, 3u8, 0u8, 0u8, 126u8, 3u8, 0u8, 0u8, 127u8,
                    3u8, 0u8, 0u8, 135u8, 3u8, 0u8, 0u8, 136u8, 3u8, 0u8, 0u8, 88u8, 9u8, 0u8, 0u8,
                    96u8, 9u8, 0u8, 0u8, 220u8, 9u8, 0u8, 0u8, 222u8, 9u8, 0u8, 0u8, 223u8, 9u8,
                    0u8, 0u8, 224u8, 9u8, 0u8, 0u8, 51u8, 10u8, 0u8, 0u8, 52u8, 10u8, 0u8, 0u8,
                    54u8, 10u8, 0u8, 0u8, 55u8, 10u8, 0u8, 0u8, 89u8, 10u8, 0u8, 0u8, 92u8, 10u8,
                    0u8, 0u8, 94u8, 10u8, 0u8, 0u8, 95u8, 10u8, 0u8, 0u8, 92u8, 11u8, 0u8, 0u8,
                    94u8, 11u8, 0u8, 0u8, 67u8, 15u8, 0u8, 0u8, 68u8, 15u8, 0u8, 0u8, 77u8, 15u8,
                    0u8, 0u8, 78u8, 15u8, 0u8, 0u8, 82u8, 15u8, 0u8, 0u8, 83u8, 15u8, 0u8, 0u8,
                    87u8, 15u8, 0u8, 0u8, 88u8, 15u8, 0u8, 0u8, 92u8, 15u8, 0u8, 0u8, 93u8, 15u8,
                    0u8, 0u8, 105u8, 15u8, 0u8, 0u8, 106u8, 15u8, 0u8, 0u8, 115u8, 15u8, 0u8, 0u8,
                    116u8, 15u8, 0u8, 0u8, 117u8, 15u8, 0u8, 0u8, 119u8, 15u8, 0u8, 0u8, 120u8,
                    15u8, 0u8, 0u8, 121u8, 15u8, 0u8, 0u8, 129u8, 15u8, 0u8, 0u8, 130u8, 15u8, 0u8,
                    0u8, 147u8, 15u8, 0u8, 0u8, 148u8, 15u8, 0u8, 0u8, 157u8, 15u8, 0u8, 0u8,
                    158u8, 15u8, 0u8, 0u8, 162u8, 15u8, 0u8, 0u8, 163u8, 15u8, 0u8, 0u8, 167u8,
                    15u8, 0u8, 0u8, 168u8, 15u8, 0u8, 0u8, 172u8, 15u8, 0u8, 0u8, 173u8, 15u8, 0u8,
                    0u8, 185u8, 15u8, 0u8, 0u8, 186u8, 15u8, 0u8, 0u8, 113u8, 31u8, 0u8, 0u8,
                    114u8, 31u8, 0u8, 0u8, 115u8, 31u8, 0u8, 0u8, 116u8, 31u8, 0u8, 0u8, 117u8,
                    31u8, 0u8, 0u8, 118u8, 31u8, 0u8, 0u8, 119u8, 31u8, 0u8, 0u8, 120u8, 31u8, 0u8,
                    0u8, 121u8, 31u8, 0u8, 0u8, 122u8, 31u8, 0u8, 0u8, 123u8, 31u8, 0u8, 0u8,
                    124u8, 31u8, 0u8, 0u8, 125u8, 31u8, 0u8, 0u8, 126u8, 31u8, 0u8, 0u8, 187u8,
                    31u8, 0u8, 0u8, 188u8, 31u8, 0u8, 0u8, 190u8, 31u8, 0u8, 0u8, 191u8, 31u8, 0u8,
                    0u8, 201u8, 31u8, 0u8, 0u8, 202u8, 31u8, 0u8, 0u8, 203u8, 31u8, 0u8, 0u8,
                    204u8, 31u8, 0u8, 0u8, 211u8, 31u8, 0u8, 0u8, 212u8, 31u8, 0u8, 0u8, 219u8,
                    31u8, 0u8, 0u8, 220u8, 31u8, 0u8, 0u8, 227u8, 31u8, 0u8, 0u8, 228u8, 31u8, 0u8,
                    0u8, 235u8, 31u8, 0u8, 0u8, 236u8, 31u8, 0u8, 0u8, 238u8, 31u8, 0u8, 0u8,
                    240u8, 31u8, 0u8, 0u8, 249u8, 31u8, 0u8, 0u8, 250u8, 31u8, 0u8, 0u8, 251u8,
                    31u8, 0u8, 0u8, 252u8, 31u8, 0u8, 0u8, 253u8, 31u8, 0u8, 0u8, 254u8, 31u8, 0u8,
                    0u8, 0u8, 32u8, 0u8, 0u8, 2u8, 32u8, 0u8, 0u8, 38u8, 33u8, 0u8, 0u8, 39u8,
                    33u8, 0u8, 0u8, 42u8, 33u8, 0u8, 0u8, 44u8, 33u8, 0u8, 0u8, 41u8, 35u8, 0u8,
                    0u8, 43u8, 35u8, 0u8, 0u8, 220u8, 42u8, 0u8, 0u8, 221u8, 42u8, 0u8, 0u8, 0u8,
                    249u8, 0u8, 0u8, 14u8, 250u8, 0u8, 0u8, 16u8, 250u8, 0u8, 0u8, 17u8, 250u8,
                    0u8, 0u8, 18u8, 250u8, 0u8, 0u8, 19u8, 250u8, 0u8, 0u8, 21u8, 250u8, 0u8, 0u8,
                    31u8, 250u8, 0u8, 0u8, 32u8, 250u8, 0u8, 0u8, 33u8, 250u8, 0u8, 0u8, 34u8,
                    250u8, 0u8, 0u8, 35u8, 250u8, 0u8, 0u8, 37u8, 250u8, 0u8, 0u8, 39u8, 250u8,
                    0u8, 0u8, 42u8, 250u8, 0u8, 0u8, 110u8, 250u8, 0u8, 0u8, 112u8, 250u8, 0u8,
                    0u8, 218u8, 250u8, 0u8, 0u8, 29u8, 251u8, 0u8, 0u8, 30u8, 251u8, 0u8, 0u8,
                    31u8, 251u8, 0u8, 0u8, 32u8, 251u8, 0u8, 0u8, 42u8, 251u8, 0u8, 0u8, 55u8,
                    251u8, 0u8, 0u8, 56u8, 251u8, 0u8, 0u8, 61u8, 251u8, 0u8, 0u8, 62u8, 251u8,
                    0u8, 0u8, 63u8, 251u8, 0u8, 0u8, 64u8, 251u8, 0u8, 0u8, 66u8, 251u8, 0u8, 0u8,
                    67u8, 251u8, 0u8, 0u8, 69u8, 251u8, 0u8, 0u8, 70u8, 251u8, 0u8, 0u8, 79u8,
                    251u8, 0u8, 0u8, 94u8, 209u8, 1u8, 0u8, 101u8, 209u8, 1u8, 0u8, 187u8, 209u8,
                    1u8, 0u8, 193u8, 209u8, 1u8, 0u8, 0u8, 248u8, 2u8, 0u8, 30u8, 250u8, 2u8, 0u8,
                ])
            },
            1120usize,
        )
    });
