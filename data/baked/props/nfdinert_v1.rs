// @generated
#![cfg(feature = "icu_properties")]
type DataStruct =
    <::icu_properties::provider::NfdInertV1Marker as ::icu_provider::DataMarker>::Yokeable;
pub static DATA: litemap::LiteMap<&str, &DataStruct, &[(&str, &DataStruct)]> =
    litemap::LiteMap::from_sorted_store_unchecked(&[("und", UND)]);
static UND: &DataStruct =
    &::icu_properties::provider::PropertyCodePointSetV1::InversionList(unsafe {
        #[allow(unused_unsafe)]
        ::icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(
            unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    0u8, 0u8, 0u8, 0u8, 192u8, 0u8, 0u8, 0u8, 198u8, 0u8, 0u8, 0u8, 199u8, 0u8,
                    0u8, 0u8, 208u8, 0u8, 0u8, 0u8, 209u8, 0u8, 0u8, 0u8, 215u8, 0u8, 0u8, 0u8,
                    217u8, 0u8, 0u8, 0u8, 222u8, 0u8, 0u8, 0u8, 224u8, 0u8, 0u8, 0u8, 230u8, 0u8,
                    0u8, 0u8, 231u8, 0u8, 0u8, 0u8, 240u8, 0u8, 0u8, 0u8, 241u8, 0u8, 0u8, 0u8,
                    247u8, 0u8, 0u8, 0u8, 249u8, 0u8, 0u8, 0u8, 254u8, 0u8, 0u8, 0u8, 255u8, 0u8,
                    0u8, 0u8, 16u8, 1u8, 0u8, 0u8, 18u8, 1u8, 0u8, 0u8, 38u8, 1u8, 0u8, 0u8, 40u8,
                    1u8, 0u8, 0u8, 49u8, 1u8, 0u8, 0u8, 52u8, 1u8, 0u8, 0u8, 56u8, 1u8, 0u8, 0u8,
                    57u8, 1u8, 0u8, 0u8, 63u8, 1u8, 0u8, 0u8, 67u8, 1u8, 0u8, 0u8, 73u8, 1u8, 0u8,
                    0u8, 76u8, 1u8, 0u8, 0u8, 82u8, 1u8, 0u8, 0u8, 84u8, 1u8, 0u8, 0u8, 102u8, 1u8,
                    0u8, 0u8, 104u8, 1u8, 0u8, 0u8, 127u8, 1u8, 0u8, 0u8, 160u8, 1u8, 0u8, 0u8,
                    162u8, 1u8, 0u8, 0u8, 175u8, 1u8, 0u8, 0u8, 177u8, 1u8, 0u8, 0u8, 205u8, 1u8,
                    0u8, 0u8, 221u8, 1u8, 0u8, 0u8, 222u8, 1u8, 0u8, 0u8, 228u8, 1u8, 0u8, 0u8,
                    230u8, 1u8, 0u8, 0u8, 241u8, 1u8, 0u8, 0u8, 244u8, 1u8, 0u8, 0u8, 246u8, 1u8,
                    0u8, 0u8, 248u8, 1u8, 0u8, 0u8, 28u8, 2u8, 0u8, 0u8, 30u8, 2u8, 0u8, 0u8, 32u8,
                    2u8, 0u8, 0u8, 38u8, 2u8, 0u8, 0u8, 52u8, 2u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8,
                    79u8, 3u8, 0u8, 0u8, 80u8, 3u8, 0u8, 0u8, 112u8, 3u8, 0u8, 0u8, 116u8, 3u8,
                    0u8, 0u8, 117u8, 3u8, 0u8, 0u8, 126u8, 3u8, 0u8, 0u8, 127u8, 3u8, 0u8, 0u8,
                    133u8, 3u8, 0u8, 0u8, 139u8, 3u8, 0u8, 0u8, 140u8, 3u8, 0u8, 0u8, 141u8, 3u8,
                    0u8, 0u8, 142u8, 3u8, 0u8, 0u8, 145u8, 3u8, 0u8, 0u8, 170u8, 3u8, 0u8, 0u8,
                    177u8, 3u8, 0u8, 0u8, 202u8, 3u8, 0u8, 0u8, 207u8, 3u8, 0u8, 0u8, 211u8, 3u8,
                    0u8, 0u8, 213u8, 3u8, 0u8, 0u8, 0u8, 4u8, 0u8, 0u8, 2u8, 4u8, 0u8, 0u8, 3u8,
                    4u8, 0u8, 0u8, 4u8, 4u8, 0u8, 0u8, 7u8, 4u8, 0u8, 0u8, 8u8, 4u8, 0u8, 0u8,
                    12u8, 4u8, 0u8, 0u8, 15u8, 4u8, 0u8, 0u8, 25u8, 4u8, 0u8, 0u8, 26u8, 4u8, 0u8,
                    0u8, 57u8, 4u8, 0u8, 0u8, 58u8, 4u8, 0u8, 0u8, 80u8, 4u8, 0u8, 0u8, 82u8, 4u8,
                    0u8, 0u8, 83u8, 4u8, 0u8, 0u8, 84u8, 4u8, 0u8, 0u8, 87u8, 4u8, 0u8, 0u8, 88u8,
                    4u8, 0u8, 0u8, 92u8, 4u8, 0u8, 0u8, 95u8, 4u8, 0u8, 0u8, 118u8, 4u8, 0u8, 0u8,
                    120u8, 4u8, 0u8, 0u8, 131u8, 4u8, 0u8, 0u8, 136u8, 4u8, 0u8, 0u8, 193u8, 4u8,
                    0u8, 0u8, 195u8, 4u8, 0u8, 0u8, 208u8, 4u8, 0u8, 0u8, 212u8, 4u8, 0u8, 0u8,
                    214u8, 4u8, 0u8, 0u8, 216u8, 4u8, 0u8, 0u8, 218u8, 4u8, 0u8, 0u8, 224u8, 4u8,
                    0u8, 0u8, 226u8, 4u8, 0u8, 0u8, 232u8, 4u8, 0u8, 0u8, 234u8, 4u8, 0u8, 0u8,
                    246u8, 4u8, 0u8, 0u8, 248u8, 4u8, 0u8, 0u8, 250u8, 4u8, 0u8, 0u8, 145u8, 5u8,
                    0u8, 0u8, 190u8, 5u8, 0u8, 0u8, 191u8, 5u8, 0u8, 0u8, 192u8, 5u8, 0u8, 0u8,
                    193u8, 5u8, 0u8, 0u8, 195u8, 5u8, 0u8, 0u8, 196u8, 5u8, 0u8, 0u8, 198u8, 5u8,
                    0u8, 0u8, 199u8, 5u8, 0u8, 0u8, 200u8, 5u8, 0u8, 0u8, 16u8, 6u8, 0u8, 0u8,
                    27u8, 6u8, 0u8, 0u8, 34u8, 6u8, 0u8, 0u8, 39u8, 6u8, 0u8, 0u8, 75u8, 6u8, 0u8,
                    0u8, 96u8, 6u8, 0u8, 0u8, 112u8, 6u8, 0u8, 0u8, 113u8, 6u8, 0u8, 0u8, 192u8,
                    6u8, 0u8, 0u8, 193u8, 6u8, 0u8, 0u8, 194u8, 6u8, 0u8, 0u8, 195u8, 6u8, 0u8,
                    0u8, 211u8, 6u8, 0u8, 0u8, 212u8, 6u8, 0u8, 0u8, 214u8, 6u8, 0u8, 0u8, 221u8,
                    6u8, 0u8, 0u8, 223u8, 6u8, 0u8, 0u8, 229u8, 6u8, 0u8, 0u8, 231u8, 6u8, 0u8,
                    0u8, 233u8, 6u8, 0u8, 0u8, 234u8, 6u8, 0u8, 0u8, 238u8, 6u8, 0u8, 0u8, 17u8,
                    7u8, 0u8, 0u8, 18u8, 7u8, 0u8, 0u8, 48u8, 7u8, 0u8, 0u8, 75u8, 7u8, 0u8, 0u8,
                    235u8, 7u8, 0u8, 0u8, 244u8, 7u8, 0u8, 0u8, 253u8, 7u8, 0u8, 0u8, 254u8, 7u8,
                    0u8, 0u8, 22u8, 8u8, 0u8, 0u8, 26u8, 8u8, 0u8, 0u8, 27u8, 8u8, 0u8, 0u8, 36u8,
                    8u8, 0u8, 0u8, 37u8, 8u8, 0u8, 0u8, 40u8, 8u8, 0u8, 0u8, 41u8, 8u8, 0u8, 0u8,
                    46u8, 8u8, 0u8, 0u8, 89u8, 8u8, 0u8, 0u8, 92u8, 8u8, 0u8, 0u8, 152u8, 8u8, 0u8,
                    0u8, 160u8, 8u8, 0u8, 0u8, 202u8, 8u8, 0u8, 0u8, 226u8, 8u8, 0u8, 0u8, 227u8,
                    8u8, 0u8, 0u8, 0u8, 9u8, 0u8, 0u8, 41u8, 9u8, 0u8, 0u8, 42u8, 9u8, 0u8, 0u8,
                    49u8, 9u8, 0u8, 0u8, 50u8, 9u8, 0u8, 0u8, 52u8, 9u8, 0u8, 0u8, 53u8, 9u8, 0u8,
                    0u8, 60u8, 9u8, 0u8, 0u8, 61u8, 9u8, 0u8, 0u8, 77u8, 9u8, 0u8, 0u8, 78u8, 9u8,
                    0u8, 0u8, 81u8, 9u8, 0u8, 0u8, 85u8, 9u8, 0u8, 0u8, 88u8, 9u8, 0u8, 0u8, 96u8,
                    9u8, 0u8, 0u8, 188u8, 9u8, 0u8, 0u8, 189u8, 9u8, 0u8, 0u8, 203u8, 9u8, 0u8,
                    0u8, 206u8, 9u8, 0u8, 0u8, 220u8, 9u8, 0u8, 0u8, 222u8, 9u8, 0u8, 0u8, 223u8,
                    9u8, 0u8, 0u8, 224u8, 9u8, 0u8, 0u8, 254u8, 9u8, 0u8, 0u8, 255u8, 9u8, 0u8,
                    0u8, 51u8, 10u8, 0u8, 0u8, 52u8, 10u8, 0u8, 0u8, 54u8, 10u8, 0u8, 0u8, 55u8,
                    10u8, 0u8, 0u8, 60u8, 10u8, 0u8, 0u8, 61u8, 10u8, 0u8, 0u8, 77u8, 10u8, 0u8,
                    0u8, 78u8, 10u8, 0u8, 0u8, 89u8, 10u8, 0u8, 0u8, 92u8, 10u8, 0u8, 0u8, 94u8,
                    10u8, 0u8, 0u8, 95u8, 10u8, 0u8, 0u8, 188u8, 10u8, 0u8, 0u8, 189u8, 10u8, 0u8,
                    0u8, 205u8, 10u8, 0u8, 0u8, 206u8, 10u8, 0u8, 0u8, 60u8, 11u8, 0u8, 0u8, 61u8,
                    11u8, 0u8, 0u8, 72u8, 11u8, 0u8, 0u8, 73u8, 11u8, 0u8, 0u8, 75u8, 11u8, 0u8,
                    0u8, 78u8, 11u8, 0u8, 0u8, 92u8, 11u8, 0u8, 0u8, 94u8, 11u8, 0u8, 0u8, 148u8,
                    11u8, 0u8, 0u8, 149u8, 11u8, 0u8, 0u8, 202u8, 11u8, 0u8, 0u8, 206u8, 11u8, 0u8,
                    0u8, 60u8, 12u8, 0u8, 0u8, 61u8, 12u8, 0u8, 0u8, 72u8, 12u8, 0u8, 0u8, 73u8,
                    12u8, 0u8, 0u8, 77u8, 12u8, 0u8, 0u8, 78u8, 12u8, 0u8, 0u8, 85u8, 12u8, 0u8,
                    0u8, 87u8, 12u8, 0u8, 0u8, 188u8, 12u8, 0u8, 0u8, 189u8, 12u8, 0u8, 0u8, 192u8,
                    12u8, 0u8, 0u8, 193u8, 12u8, 0u8, 0u8, 199u8, 12u8, 0u8, 0u8, 201u8, 12u8, 0u8,
                    0u8, 202u8, 12u8, 0u8, 0u8, 204u8, 12u8, 0u8, 0u8, 205u8, 12u8, 0u8, 0u8,
                    206u8, 12u8, 0u8, 0u8, 59u8, 13u8, 0u8, 0u8, 61u8, 13u8, 0u8, 0u8, 74u8, 13u8,
                    0u8, 0u8, 78u8, 13u8, 0u8, 0u8, 202u8, 13u8, 0u8, 0u8, 203u8, 13u8, 0u8, 0u8,
                    218u8, 13u8, 0u8, 0u8, 219u8, 13u8, 0u8, 0u8, 220u8, 13u8, 0u8, 0u8, 223u8,
                    13u8, 0u8, 0u8, 56u8, 14u8, 0u8, 0u8, 59u8, 14u8, 0u8, 0u8, 72u8, 14u8, 0u8,
                    0u8, 76u8, 14u8, 0u8, 0u8, 184u8, 14u8, 0u8, 0u8, 187u8, 14u8, 0u8, 0u8, 200u8,
                    14u8, 0u8, 0u8, 204u8, 14u8, 0u8, 0u8, 24u8, 15u8, 0u8, 0u8, 26u8, 15u8, 0u8,
                    0u8, 53u8, 15u8, 0u8, 0u8, 54u8, 15u8, 0u8, 0u8, 55u8, 15u8, 0u8, 0u8, 56u8,
                    15u8, 0u8, 0u8, 57u8, 15u8, 0u8, 0u8, 58u8, 15u8, 0u8, 0u8, 67u8, 15u8, 0u8,
                    0u8, 68u8, 15u8, 0u8, 0u8, 77u8, 15u8, 0u8, 0u8, 78u8, 15u8, 0u8, 0u8, 82u8,
                    15u8, 0u8, 0u8, 83u8, 15u8, 0u8, 0u8, 87u8, 15u8, 0u8, 0u8, 88u8, 15u8, 0u8,
                    0u8, 92u8, 15u8, 0u8, 0u8, 93u8, 15u8, 0u8, 0u8, 105u8, 15u8, 0u8, 0u8, 106u8,
                    15u8, 0u8, 0u8, 113u8, 15u8, 0u8, 0u8, 119u8, 15u8, 0u8, 0u8, 120u8, 15u8, 0u8,
                    0u8, 121u8, 15u8, 0u8, 0u8, 122u8, 15u8, 0u8, 0u8, 126u8, 15u8, 0u8, 0u8,
                    128u8, 15u8, 0u8, 0u8, 133u8, 15u8, 0u8, 0u8, 134u8, 15u8, 0u8, 0u8, 136u8,
                    15u8, 0u8, 0u8, 147u8, 15u8, 0u8, 0u8, 148u8, 15u8, 0u8, 0u8, 157u8, 15u8, 0u8,
                    0u8, 158u8, 15u8, 0u8, 0u8, 162u8, 15u8, 0u8, 0u8, 163u8, 15u8, 0u8, 0u8,
                    167u8, 15u8, 0u8, 0u8, 168u8, 15u8, 0u8, 0u8, 172u8, 15u8, 0u8, 0u8, 173u8,
                    15u8, 0u8, 0u8, 185u8, 15u8, 0u8, 0u8, 186u8, 15u8, 0u8, 0u8, 198u8, 15u8, 0u8,
                    0u8, 199u8, 15u8, 0u8, 0u8, 38u8, 16u8, 0u8, 0u8, 39u8, 16u8, 0u8, 0u8, 55u8,
                    16u8, 0u8, 0u8, 56u8, 16u8, 0u8, 0u8, 57u8, 16u8, 0u8, 0u8, 59u8, 16u8, 0u8,
                    0u8, 141u8, 16u8, 0u8, 0u8, 142u8, 16u8, 0u8, 0u8, 93u8, 19u8, 0u8, 0u8, 96u8,
                    19u8, 0u8, 0u8, 20u8, 23u8, 0u8, 0u8, 22u8, 23u8, 0u8, 0u8, 52u8, 23u8, 0u8,
                    0u8, 53u8, 23u8, 0u8, 0u8, 210u8, 23u8, 0u8, 0u8, 211u8, 23u8, 0u8, 0u8, 221u8,
                    23u8, 0u8, 0u8, 222u8, 23u8, 0u8, 0u8, 169u8, 24u8, 0u8, 0u8, 170u8, 24u8, 0u8,
                    0u8, 57u8, 25u8, 0u8, 0u8, 60u8, 25u8, 0u8, 0u8, 23u8, 26u8, 0u8, 0u8, 25u8,
                    26u8, 0u8, 0u8, 96u8, 26u8, 0u8, 0u8, 97u8, 26u8, 0u8, 0u8, 117u8, 26u8, 0u8,
                    0u8, 125u8, 26u8, 0u8, 0u8, 127u8, 26u8, 0u8, 0u8, 128u8, 26u8, 0u8, 0u8,
                    176u8, 26u8, 0u8, 0u8, 190u8, 26u8, 0u8, 0u8, 191u8, 26u8, 0u8, 0u8, 207u8,
                    26u8, 0u8, 0u8, 6u8, 27u8, 0u8, 0u8, 7u8, 27u8, 0u8, 0u8, 8u8, 27u8, 0u8, 0u8,
                    9u8, 27u8, 0u8, 0u8, 10u8, 27u8, 0u8, 0u8, 11u8, 27u8, 0u8, 0u8, 12u8, 27u8,
                    0u8, 0u8, 13u8, 27u8, 0u8, 0u8, 14u8, 27u8, 0u8, 0u8, 15u8, 27u8, 0u8, 0u8,
                    18u8, 27u8, 0u8, 0u8, 19u8, 27u8, 0u8, 0u8, 52u8, 27u8, 0u8, 0u8, 53u8, 27u8,
                    0u8, 0u8, 59u8, 27u8, 0u8, 0u8, 60u8, 27u8, 0u8, 0u8, 61u8, 27u8, 0u8, 0u8,
                    62u8, 27u8, 0u8, 0u8, 64u8, 27u8, 0u8, 0u8, 66u8, 27u8, 0u8, 0u8, 67u8, 27u8,
                    0u8, 0u8, 69u8, 27u8, 0u8, 0u8, 107u8, 27u8, 0u8, 0u8, 116u8, 27u8, 0u8, 0u8,
                    170u8, 27u8, 0u8, 0u8, 172u8, 27u8, 0u8, 0u8, 230u8, 27u8, 0u8, 0u8, 231u8,
                    27u8, 0u8, 0u8, 242u8, 27u8, 0u8, 0u8, 244u8, 27u8, 0u8, 0u8, 55u8, 28u8, 0u8,
                    0u8, 56u8, 28u8, 0u8, 0u8, 208u8, 28u8, 0u8, 0u8, 211u8, 28u8, 0u8, 0u8, 212u8,
                    28u8, 0u8, 0u8, 225u8, 28u8, 0u8, 0u8, 226u8, 28u8, 0u8, 0u8, 233u8, 28u8, 0u8,
                    0u8, 237u8, 28u8, 0u8, 0u8, 238u8, 28u8, 0u8, 0u8, 244u8, 28u8, 0u8, 0u8,
                    245u8, 28u8, 0u8, 0u8, 248u8, 28u8, 0u8, 0u8, 250u8, 28u8, 0u8, 0u8, 192u8,
                    29u8, 0u8, 0u8, 154u8, 30u8, 0u8, 0u8, 155u8, 30u8, 0u8, 0u8, 156u8, 30u8, 0u8,
                    0u8, 160u8, 30u8, 0u8, 0u8, 250u8, 30u8, 0u8, 0u8, 0u8, 31u8, 0u8, 0u8, 22u8,
                    31u8, 0u8, 0u8, 24u8, 31u8, 0u8, 0u8, 30u8, 31u8, 0u8, 0u8, 32u8, 31u8, 0u8,
                    0u8, 70u8, 31u8, 0u8, 0u8, 72u8, 31u8, 0u8, 0u8, 78u8, 31u8, 0u8, 0u8, 80u8,
                    31u8, 0u8, 0u8, 88u8, 31u8, 0u8, 0u8, 89u8, 31u8, 0u8, 0u8, 90u8, 31u8, 0u8,
                    0u8, 91u8, 31u8, 0u8, 0u8, 92u8, 31u8, 0u8, 0u8, 93u8, 31u8, 0u8, 0u8, 94u8,
                    31u8, 0u8, 0u8, 95u8, 31u8, 0u8, 0u8, 126u8, 31u8, 0u8, 0u8, 128u8, 31u8, 0u8,
                    0u8, 181u8, 31u8, 0u8, 0u8, 182u8, 31u8, 0u8, 0u8, 189u8, 31u8, 0u8, 0u8,
                    190u8, 31u8, 0u8, 0u8, 191u8, 31u8, 0u8, 0u8, 193u8, 31u8, 0u8, 0u8, 197u8,
                    31u8, 0u8, 0u8, 198u8, 31u8, 0u8, 0u8, 212u8, 31u8, 0u8, 0u8, 214u8, 31u8, 0u8,
                    0u8, 220u8, 31u8, 0u8, 0u8, 221u8, 31u8, 0u8, 0u8, 240u8, 31u8, 0u8, 0u8,
                    242u8, 31u8, 0u8, 0u8, 245u8, 31u8, 0u8, 0u8, 246u8, 31u8, 0u8, 0u8, 254u8,
                    31u8, 0u8, 0u8, 0u8, 32u8, 0u8, 0u8, 2u8, 32u8, 0u8, 0u8, 208u8, 32u8, 0u8,
                    0u8, 221u8, 32u8, 0u8, 0u8, 225u8, 32u8, 0u8, 0u8, 226u8, 32u8, 0u8, 0u8,
                    229u8, 32u8, 0u8, 0u8, 241u8, 32u8, 0u8, 0u8, 38u8, 33u8, 0u8, 0u8, 39u8, 33u8,
                    0u8, 0u8, 42u8, 33u8, 0u8, 0u8, 44u8, 33u8, 0u8, 0u8, 154u8, 33u8, 0u8, 0u8,
                    156u8, 33u8, 0u8, 0u8, 174u8, 33u8, 0u8, 0u8, 175u8, 33u8, 0u8, 0u8, 205u8,
                    33u8, 0u8, 0u8, 208u8, 33u8, 0u8, 0u8, 4u8, 34u8, 0u8, 0u8, 5u8, 34u8, 0u8,
                    0u8, 9u8, 34u8, 0u8, 0u8, 10u8, 34u8, 0u8, 0u8, 12u8, 34u8, 0u8, 0u8, 13u8,
                    34u8, 0u8, 0u8, 36u8, 34u8, 0u8, 0u8, 37u8, 34u8, 0u8, 0u8, 38u8, 34u8, 0u8,
                    0u8, 39u8, 34u8, 0u8, 0u8, 65u8, 34u8, 0u8, 0u8, 66u8, 34u8, 0u8, 0u8, 68u8,
                    34u8, 0u8, 0u8, 69u8, 34u8, 0u8, 0u8, 71u8, 34u8, 0u8, 0u8, 72u8, 34u8, 0u8,
                    0u8, 73u8, 34u8, 0u8, 0u8, 74u8, 34u8, 0u8, 0u8, 96u8, 34u8, 0u8, 0u8, 97u8,
                    34u8, 0u8, 0u8, 98u8, 34u8, 0u8, 0u8, 99u8, 34u8, 0u8, 0u8, 109u8, 34u8, 0u8,
                    0u8, 114u8, 34u8, 0u8, 0u8, 116u8, 34u8, 0u8, 0u8, 118u8, 34u8, 0u8, 0u8,
                    120u8, 34u8, 0u8, 0u8, 122u8, 34u8, 0u8, 0u8, 128u8, 34u8, 0u8, 0u8, 130u8,
                    34u8, 0u8, 0u8, 132u8, 34u8, 0u8, 0u8, 134u8, 34u8, 0u8, 0u8, 136u8, 34u8, 0u8,
                    0u8, 138u8, 34u8, 0u8, 0u8, 172u8, 34u8, 0u8, 0u8, 176u8, 34u8, 0u8, 0u8,
                    224u8, 34u8, 0u8, 0u8, 228u8, 34u8, 0u8, 0u8, 234u8, 34u8, 0u8, 0u8, 238u8,
                    34u8, 0u8, 0u8, 41u8, 35u8, 0u8, 0u8, 43u8, 35u8, 0u8, 0u8, 220u8, 42u8, 0u8,
                    0u8, 221u8, 42u8, 0u8, 0u8, 239u8, 44u8, 0u8, 0u8, 242u8, 44u8, 0u8, 0u8,
                    127u8, 45u8, 0u8, 0u8, 128u8, 45u8, 0u8, 0u8, 224u8, 45u8, 0u8, 0u8, 0u8, 46u8,
                    0u8, 0u8, 42u8, 48u8, 0u8, 0u8, 48u8, 48u8, 0u8, 0u8, 76u8, 48u8, 0u8, 0u8,
                    77u8, 48u8, 0u8, 0u8, 78u8, 48u8, 0u8, 0u8, 79u8, 48u8, 0u8, 0u8, 80u8, 48u8,
                    0u8, 0u8, 81u8, 48u8, 0u8, 0u8, 82u8, 48u8, 0u8, 0u8, 83u8, 48u8, 0u8, 0u8,
                    84u8, 48u8, 0u8, 0u8, 85u8, 48u8, 0u8, 0u8, 86u8, 48u8, 0u8, 0u8, 87u8, 48u8,
                    0u8, 0u8, 88u8, 48u8, 0u8, 0u8, 89u8, 48u8, 0u8, 0u8, 90u8, 48u8, 0u8, 0u8,
                    91u8, 48u8, 0u8, 0u8, 92u8, 48u8, 0u8, 0u8, 93u8, 48u8, 0u8, 0u8, 94u8, 48u8,
                    0u8, 0u8, 95u8, 48u8, 0u8, 0u8, 96u8, 48u8, 0u8, 0u8, 97u8, 48u8, 0u8, 0u8,
                    98u8, 48u8, 0u8, 0u8, 99u8, 48u8, 0u8, 0u8, 101u8, 48u8, 0u8, 0u8, 102u8, 48u8,
                    0u8, 0u8, 103u8, 48u8, 0u8, 0u8, 104u8, 48u8, 0u8, 0u8, 105u8, 48u8, 0u8, 0u8,
                    106u8, 48u8, 0u8, 0u8, 112u8, 48u8, 0u8, 0u8, 114u8, 48u8, 0u8, 0u8, 115u8,
                    48u8, 0u8, 0u8, 117u8, 48u8, 0u8, 0u8, 118u8, 48u8, 0u8, 0u8, 120u8, 48u8, 0u8,
                    0u8, 121u8, 48u8, 0u8, 0u8, 123u8, 48u8, 0u8, 0u8, 124u8, 48u8, 0u8, 0u8,
                    126u8, 48u8, 0u8, 0u8, 148u8, 48u8, 0u8, 0u8, 149u8, 48u8, 0u8, 0u8, 153u8,
                    48u8, 0u8, 0u8, 155u8, 48u8, 0u8, 0u8, 158u8, 48u8, 0u8, 0u8, 159u8, 48u8, 0u8,
                    0u8, 172u8, 48u8, 0u8, 0u8, 173u8, 48u8, 0u8, 0u8, 174u8, 48u8, 0u8, 0u8,
                    175u8, 48u8, 0u8, 0u8, 176u8, 48u8, 0u8, 0u8, 177u8, 48u8, 0u8, 0u8, 178u8,
                    48u8, 0u8, 0u8, 179u8, 48u8, 0u8, 0u8, 180u8, 48u8, 0u8, 0u8, 181u8, 48u8, 0u8,
                    0u8, 182u8, 48u8, 0u8, 0u8, 183u8, 48u8, 0u8, 0u8, 184u8, 48u8, 0u8, 0u8,
                    185u8, 48u8, 0u8, 0u8, 186u8, 48u8, 0u8, 0u8, 187u8, 48u8, 0u8, 0u8, 188u8,
                    48u8, 0u8, 0u8, 189u8, 48u8, 0u8, 0u8, 190u8, 48u8, 0u8, 0u8, 191u8, 48u8, 0u8,
                    0u8, 192u8, 48u8, 0u8, 0u8, 193u8, 48u8, 0u8, 0u8, 194u8, 48u8, 0u8, 0u8,
                    195u8, 48u8, 0u8, 0u8, 197u8, 48u8, 0u8, 0u8, 198u8, 48u8, 0u8, 0u8, 199u8,
                    48u8, 0u8, 0u8, 200u8, 48u8, 0u8, 0u8, 201u8, 48u8, 0u8, 0u8, 202u8, 48u8, 0u8,
                    0u8, 208u8, 48u8, 0u8, 0u8, 210u8, 48u8, 0u8, 0u8, 211u8, 48u8, 0u8, 0u8,
                    213u8, 48u8, 0u8, 0u8, 214u8, 48u8, 0u8, 0u8, 216u8, 48u8, 0u8, 0u8, 217u8,
                    48u8, 0u8, 0u8, 219u8, 48u8, 0u8, 0u8, 220u8, 48u8, 0u8, 0u8, 222u8, 48u8, 0u8,
                    0u8, 244u8, 48u8, 0u8, 0u8, 245u8, 48u8, 0u8, 0u8, 247u8, 48u8, 0u8, 0u8,
                    251u8, 48u8, 0u8, 0u8, 254u8, 48u8, 0u8, 0u8, 255u8, 48u8, 0u8, 0u8, 111u8,
                    166u8, 0u8, 0u8, 112u8, 166u8, 0u8, 0u8, 116u8, 166u8, 0u8, 0u8, 126u8, 166u8,
                    0u8, 0u8, 158u8, 166u8, 0u8, 0u8, 160u8, 166u8, 0u8, 0u8, 240u8, 166u8, 0u8,
                    0u8, 242u8, 166u8, 0u8, 0u8, 6u8, 168u8, 0u8, 0u8, 7u8, 168u8, 0u8, 0u8, 44u8,
                    168u8, 0u8, 0u8, 45u8, 168u8, 0u8, 0u8, 196u8, 168u8, 0u8, 0u8, 197u8, 168u8,
                    0u8, 0u8, 224u8, 168u8, 0u8, 0u8, 242u8, 168u8, 0u8, 0u8, 43u8, 169u8, 0u8,
                    0u8, 46u8, 169u8, 0u8, 0u8, 83u8, 169u8, 0u8, 0u8, 84u8, 169u8, 0u8, 0u8,
                    179u8, 169u8, 0u8, 0u8, 180u8, 169u8, 0u8, 0u8, 192u8, 169u8, 0u8, 0u8, 193u8,
                    169u8, 0u8, 0u8, 176u8, 170u8, 0u8, 0u8, 177u8, 170u8, 0u8, 0u8, 178u8, 170u8,
                    0u8, 0u8, 181u8, 170u8, 0u8, 0u8, 183u8, 170u8, 0u8, 0u8, 185u8, 170u8, 0u8,
                    0u8, 190u8, 170u8, 0u8, 0u8, 192u8, 170u8, 0u8, 0u8, 193u8, 170u8, 0u8, 0u8,
                    194u8, 170u8, 0u8, 0u8, 246u8, 170u8, 0u8, 0u8, 247u8, 170u8, 0u8, 0u8, 237u8,
                    171u8, 0u8, 0u8, 238u8, 171u8, 0u8, 0u8, 0u8, 172u8, 0u8, 0u8, 164u8, 215u8,
                    0u8, 0u8, 0u8, 249u8, 0u8, 0u8, 14u8, 250u8, 0u8, 0u8, 16u8, 250u8, 0u8, 0u8,
                    17u8, 250u8, 0u8, 0u8, 18u8, 250u8, 0u8, 0u8, 19u8, 250u8, 0u8, 0u8, 21u8,
                    250u8, 0u8, 0u8, 31u8, 250u8, 0u8, 0u8, 32u8, 250u8, 0u8, 0u8, 33u8, 250u8,
                    0u8, 0u8, 34u8, 250u8, 0u8, 0u8, 35u8, 250u8, 0u8, 0u8, 37u8, 250u8, 0u8, 0u8,
                    39u8, 250u8, 0u8, 0u8, 42u8, 250u8, 0u8, 0u8, 110u8, 250u8, 0u8, 0u8, 112u8,
                    250u8, 0u8, 0u8, 218u8, 250u8, 0u8, 0u8, 29u8, 251u8, 0u8, 0u8, 32u8, 251u8,
                    0u8, 0u8, 42u8, 251u8, 0u8, 0u8, 55u8, 251u8, 0u8, 0u8, 56u8, 251u8, 0u8, 0u8,
                    61u8, 251u8, 0u8, 0u8, 62u8, 251u8, 0u8, 0u8, 63u8, 251u8, 0u8, 0u8, 64u8,
                    251u8, 0u8, 0u8, 66u8, 251u8, 0u8, 0u8, 67u8, 251u8, 0u8, 0u8, 69u8, 251u8,
                    0u8, 0u8, 70u8, 251u8, 0u8, 0u8, 79u8, 251u8, 0u8, 0u8, 32u8, 254u8, 0u8, 0u8,
                    48u8, 254u8, 0u8, 0u8, 253u8, 1u8, 1u8, 0u8, 254u8, 1u8, 1u8, 0u8, 224u8, 2u8,
                    1u8, 0u8, 225u8, 2u8, 1u8, 0u8, 118u8, 3u8, 1u8, 0u8, 123u8, 3u8, 1u8, 0u8,
                    13u8, 10u8, 1u8, 0u8, 14u8, 10u8, 1u8, 0u8, 15u8, 10u8, 1u8, 0u8, 16u8, 10u8,
                    1u8, 0u8, 56u8, 10u8, 1u8, 0u8, 59u8, 10u8, 1u8, 0u8, 63u8, 10u8, 1u8, 0u8,
                    64u8, 10u8, 1u8, 0u8, 229u8, 10u8, 1u8, 0u8, 231u8, 10u8, 1u8, 0u8, 36u8, 13u8,
                    1u8, 0u8, 40u8, 13u8, 1u8, 0u8, 171u8, 14u8, 1u8, 0u8, 173u8, 14u8, 1u8, 0u8,
                    253u8, 14u8, 1u8, 0u8, 0u8, 15u8, 1u8, 0u8, 70u8, 15u8, 1u8, 0u8, 81u8, 15u8,
                    1u8, 0u8, 130u8, 15u8, 1u8, 0u8, 134u8, 15u8, 1u8, 0u8, 70u8, 16u8, 1u8, 0u8,
                    71u8, 16u8, 1u8, 0u8, 112u8, 16u8, 1u8, 0u8, 113u8, 16u8, 1u8, 0u8, 127u8,
                    16u8, 1u8, 0u8, 128u8, 16u8, 1u8, 0u8, 154u8, 16u8, 1u8, 0u8, 155u8, 16u8, 1u8,
                    0u8, 156u8, 16u8, 1u8, 0u8, 157u8, 16u8, 1u8, 0u8, 171u8, 16u8, 1u8, 0u8,
                    172u8, 16u8, 1u8, 0u8, 185u8, 16u8, 1u8, 0u8, 187u8, 16u8, 1u8, 0u8, 0u8, 17u8,
                    1u8, 0u8, 3u8, 17u8, 1u8, 0u8, 46u8, 17u8, 1u8, 0u8, 48u8, 17u8, 1u8, 0u8,
                    51u8, 17u8, 1u8, 0u8, 53u8, 17u8, 1u8, 0u8, 115u8, 17u8, 1u8, 0u8, 116u8, 17u8,
                    1u8, 0u8, 192u8, 17u8, 1u8, 0u8, 193u8, 17u8, 1u8, 0u8, 202u8, 17u8, 1u8, 0u8,
                    203u8, 17u8, 1u8, 0u8, 53u8, 18u8, 1u8, 0u8, 55u8, 18u8, 1u8, 0u8, 233u8, 18u8,
                    1u8, 0u8, 235u8, 18u8, 1u8, 0u8, 59u8, 19u8, 1u8, 0u8, 61u8, 19u8, 1u8, 0u8,
                    75u8, 19u8, 1u8, 0u8, 78u8, 19u8, 1u8, 0u8, 102u8, 19u8, 1u8, 0u8, 109u8, 19u8,
                    1u8, 0u8, 112u8, 19u8, 1u8, 0u8, 117u8, 19u8, 1u8, 0u8, 66u8, 20u8, 1u8, 0u8,
                    67u8, 20u8, 1u8, 0u8, 70u8, 20u8, 1u8, 0u8, 71u8, 20u8, 1u8, 0u8, 94u8, 20u8,
                    1u8, 0u8, 95u8, 20u8, 1u8, 0u8, 187u8, 20u8, 1u8, 0u8, 189u8, 20u8, 1u8, 0u8,
                    190u8, 20u8, 1u8, 0u8, 191u8, 20u8, 1u8, 0u8, 194u8, 20u8, 1u8, 0u8, 196u8,
                    20u8, 1u8, 0u8, 186u8, 21u8, 1u8, 0u8, 188u8, 21u8, 1u8, 0u8, 191u8, 21u8, 1u8,
                    0u8, 193u8, 21u8, 1u8, 0u8, 63u8, 22u8, 1u8, 0u8, 64u8, 22u8, 1u8, 0u8, 182u8,
                    22u8, 1u8, 0u8, 184u8, 22u8, 1u8, 0u8, 43u8, 23u8, 1u8, 0u8, 44u8, 23u8, 1u8,
                    0u8, 57u8, 24u8, 1u8, 0u8, 59u8, 24u8, 1u8, 0u8, 56u8, 25u8, 1u8, 0u8, 57u8,
                    25u8, 1u8, 0u8, 61u8, 25u8, 1u8, 0u8, 63u8, 25u8, 1u8, 0u8, 67u8, 25u8, 1u8,
                    0u8, 68u8, 25u8, 1u8, 0u8, 224u8, 25u8, 1u8, 0u8, 225u8, 25u8, 1u8, 0u8, 52u8,
                    26u8, 1u8, 0u8, 53u8, 26u8, 1u8, 0u8, 71u8, 26u8, 1u8, 0u8, 72u8, 26u8, 1u8,
                    0u8, 153u8, 26u8, 1u8, 0u8, 154u8, 26u8, 1u8, 0u8, 63u8, 28u8, 1u8, 0u8, 64u8,
                    28u8, 1u8, 0u8, 66u8, 29u8, 1u8, 0u8, 67u8, 29u8, 1u8, 0u8, 68u8, 29u8, 1u8,
                    0u8, 70u8, 29u8, 1u8, 0u8, 151u8, 29u8, 1u8, 0u8, 152u8, 29u8, 1u8, 0u8, 65u8,
                    31u8, 1u8, 0u8, 67u8, 31u8, 1u8, 0u8, 240u8, 106u8, 1u8, 0u8, 245u8, 106u8,
                    1u8, 0u8, 48u8, 107u8, 1u8, 0u8, 55u8, 107u8, 1u8, 0u8, 240u8, 111u8, 1u8, 0u8,
                    242u8, 111u8, 1u8, 0u8, 158u8, 188u8, 1u8, 0u8, 159u8, 188u8, 1u8, 0u8, 94u8,
                    209u8, 1u8, 0u8, 106u8, 209u8, 1u8, 0u8, 109u8, 209u8, 1u8, 0u8, 115u8, 209u8,
                    1u8, 0u8, 123u8, 209u8, 1u8, 0u8, 131u8, 209u8, 1u8, 0u8, 133u8, 209u8, 1u8,
                    0u8, 140u8, 209u8, 1u8, 0u8, 170u8, 209u8, 1u8, 0u8, 174u8, 209u8, 1u8, 0u8,
                    187u8, 209u8, 1u8, 0u8, 193u8, 209u8, 1u8, 0u8, 66u8, 210u8, 1u8, 0u8, 69u8,
                    210u8, 1u8, 0u8, 0u8, 224u8, 1u8, 0u8, 7u8, 224u8, 1u8, 0u8, 8u8, 224u8, 1u8,
                    0u8, 25u8, 224u8, 1u8, 0u8, 27u8, 224u8, 1u8, 0u8, 34u8, 224u8, 1u8, 0u8, 35u8,
                    224u8, 1u8, 0u8, 37u8, 224u8, 1u8, 0u8, 38u8, 224u8, 1u8, 0u8, 43u8, 224u8,
                    1u8, 0u8, 143u8, 224u8, 1u8, 0u8, 144u8, 224u8, 1u8, 0u8, 48u8, 225u8, 1u8,
                    0u8, 55u8, 225u8, 1u8, 0u8, 174u8, 226u8, 1u8, 0u8, 175u8, 226u8, 1u8, 0u8,
                    236u8, 226u8, 1u8, 0u8, 240u8, 226u8, 1u8, 0u8, 236u8, 228u8, 1u8, 0u8, 240u8,
                    228u8, 1u8, 0u8, 208u8, 232u8, 1u8, 0u8, 215u8, 232u8, 1u8, 0u8, 68u8, 233u8,
                    1u8, 0u8, 75u8, 233u8, 1u8, 0u8, 0u8, 248u8, 2u8, 0u8,
                ])
            },
            180951usize,
        )
    });
