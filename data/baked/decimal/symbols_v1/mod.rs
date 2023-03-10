// @generated
#![cfg(feature = "icu_decimal")]
type DataStruct =
    <::icu_decimal::provider::DecimalSymbolsV1Marker as ::icu_provider::DataMarker>::Yokeable;
pub fn lookup(locale: &icu_provider::DataLocale) -> Option<&'static DataStruct> {
    static KEYS: [&str; 465usize] = [
        "af",
        "af-NA",
        "am",
        "ar",
        "ar-AE",
        "ar-AE-u-nu-arab",
        "ar-BH",
        "ar-BH-u-nu-latn",
        "ar-DJ",
        "ar-DJ-u-nu-latn",
        "ar-DZ",
        "ar-DZ-u-nu-arab",
        "ar-EG",
        "ar-EG-u-nu-latn",
        "ar-EH",
        "ar-EH-u-nu-arab",
        "ar-ER",
        "ar-ER-u-nu-latn",
        "ar-IL",
        "ar-IL-u-nu-latn",
        "ar-IQ",
        "ar-IQ-u-nu-latn",
        "ar-JO",
        "ar-JO-u-nu-latn",
        "ar-KM",
        "ar-KM-u-nu-latn",
        "ar-KW",
        "ar-KW-u-nu-latn",
        "ar-LB",
        "ar-LB-u-nu-latn",
        "ar-LY",
        "ar-LY-u-nu-arab",
        "ar-MA",
        "ar-MA-u-nu-arab",
        "ar-MR",
        "ar-MR-u-nu-latn",
        "ar-OM",
        "ar-OM-u-nu-latn",
        "ar-PS",
        "ar-PS-u-nu-latn",
        "ar-QA",
        "ar-QA-u-nu-latn",
        "ar-SA",
        "ar-SA-u-nu-latn",
        "ar-SD",
        "ar-SD-u-nu-latn",
        "ar-SO",
        "ar-SO-u-nu-latn",
        "ar-SS",
        "ar-SS-u-nu-latn",
        "ar-SY",
        "ar-SY-u-nu-latn",
        "ar-TD",
        "ar-TD-u-nu-latn",
        "ar-TN",
        "ar-TN-u-nu-arab",
        "ar-YE",
        "ar-YE-u-nu-latn",
        "ar-u-nu-latn",
        "as",
        "as-u-nu-latn",
        "az",
        "az-Latn",
        "be",
        "be-tarask",
        "bg",
        "bn",
        "bn-IN",
        "bn-IN-u-nu-latn",
        "bn-u-nu-latn",
        "bs",
        "bs-Latn",
        "ca",
        "ca-AD",
        "ca-ES-valencia",
        "ca-FR",
        "ca-IT",
        "chr",
        "cs",
        "cy",
        "da",
        "da-GL",
        "de",
        "de-AT",
        "de-BE",
        "de-CH",
        "de-IT",
        "de-LI",
        "de-LU",
        "dsb",
        "el",
        "el-CY",
        "en",
        "en-001",
        "en-150",
        "en-AE",
        "en-AG",
        "en-AI",
        "en-AS",
        "en-AT",
        "en-AU",
        "en-BB",
        "en-BE",
        "en-BI",
        "en-BM",
        "en-BS",
        "en-BW",
        "en-BZ",
        "en-CA",
        "en-CC",
        "en-CH",
        "en-CK",
        "en-CM",
        "en-CX",
        "en-CY",
        "en-DE",
        "en-DG",
        "en-DK",
        "en-DM",
        "en-ER",
        "en-FI",
        "en-FJ",
        "en-FK",
        "en-FM",
        "en-GB",
        "en-GD",
        "en-GG",
        "en-GH",
        "en-GI",
        "en-GM",
        "en-GU",
        "en-GY",
        "en-HK",
        "en-IE",
        "en-IL",
        "en-IM",
        "en-IN",
        "en-IO",
        "en-JE",
        "en-JM",
        "en-KE",
        "en-KI",
        "en-KN",
        "en-KY",
        "en-LC",
        "en-LR",
        "en-LS",
        "en-MG",
        "en-MH",
        "en-MO",
        "en-MP",
        "en-MS",
        "en-MT",
        "en-MU",
        "en-MV",
        "en-MW",
        "en-MY",
        "en-NA",
        "en-NF",
        "en-NG",
        "en-NL",
        "en-NR",
        "en-NU",
        "en-NZ",
        "en-PG",
        "en-PH",
        "en-PK",
        "en-PN",
        "en-PR",
        "en-PW",
        "en-RW",
        "en-SB",
        "en-SC",
        "en-SD",
        "en-SE",
        "en-SG",
        "en-SH",
        "en-SI",
        "en-SL",
        "en-SS",
        "en-SX",
        "en-SZ",
        "en-TC",
        "en-TK",
        "en-TO",
        "en-TT",
        "en-TV",
        "en-TZ",
        "en-UG",
        "en-UM",
        "en-VC",
        "en-VG",
        "en-VI",
        "en-VU",
        "en-WS",
        "en-ZA",
        "en-ZM",
        "en-ZW",
        "es",
        "es-419",
        "es-AR",
        "es-BO",
        "es-BR",
        "es-BZ",
        "es-CL",
        "es-CO",
        "es-CR",
        "es-CU",
        "es-DO",
        "es-EA",
        "es-EC",
        "es-GQ",
        "es-GT",
        "es-HN",
        "es-IC",
        "es-MX",
        "es-NI",
        "es-PA",
        "es-PE",
        "es-PH",
        "es-PR",
        "es-PY",
        "es-SV",
        "es-US",
        "es-UY",
        "es-VE",
        "et",
        "eu",
        "fa",
        "fa-AF",
        "fa-AF-u-nu-latn",
        "fa-u-nu-latn",
        "fi",
        "fil",
        "fr",
        "fr-BE",
        "fr-BF",
        "fr-BI",
        "fr-BJ",
        "fr-BL",
        "fr-CA",
        "fr-CD",
        "fr-CF",
        "fr-CG",
        "fr-CH",
        "fr-CI",
        "fr-CM",
        "fr-DJ",
        "fr-DZ",
        "fr-GA",
        "fr-GF",
        "fr-GN",
        "fr-GP",
        "fr-GQ",
        "fr-HT",
        "fr-KM",
        "fr-LU",
        "fr-MA",
        "fr-MC",
        "fr-MF",
        "fr-MG",
        "fr-ML",
        "fr-MQ",
        "fr-MR",
        "fr-MU",
        "fr-NC",
        "fr-NE",
        "fr-PF",
        "fr-PM",
        "fr-RE",
        "fr-RW",
        "fr-SC",
        "fr-SN",
        "fr-SY",
        "fr-TD",
        "fr-TG",
        "fr-TN",
        "fr-VU",
        "fr-WF",
        "fr-YT",
        "ga",
        "ga-GB",
        "gd",
        "gl",
        "gu",
        "gu-u-nu-gujr",
        "ha",
        "ha-GH",
        "ha-NE",
        "he",
        "hi",
        "hi-Latn",
        "hi-u-nu-deva",
        "hr",
        "hr-BA",
        "hsb",
        "hu",
        "hy",
        "id",
        "ig",
        "is",
        "it",
        "it-CH",
        "it-SM",
        "it-VA",
        "ja",
        "jv",
        "jv-u-nu-java",
        "ka",
        "kk",
        "km",
        "km-u-nu-khmr",
        "kn",
        "kn-u-nu-knda",
        "ko",
        "ko-KP",
        "kok",
        "kok-u-nu-deva",
        "ky",
        "lo",
        "lo-u-nu-laoo",
        "lt",
        "lv",
        "mk",
        "ml",
        "ml-u-nu-mlym",
        "mn",
        "mr",
        "mr-u-nu-latn",
        "ms",
        "ms-BN",
        "ms-ID",
        "ms-SG",
        "my",
        "my-u-nu-latn",
        "nb",
        "nb-SJ",
        "ne",
        "ne-IN",
        "ne-IN-u-nu-latn",
        "ne-u-nu-latn",
        "nl",
        "nl-AW",
        "nl-BE",
        "nl-BQ",
        "nl-CW",
        "nl-SR",
        "nl-SX",
        "nn",
        "no",
        "or",
        "or-u-nu-orya",
        "pa",
        "pa-Guru",
        "pa-Guru-u-nu-guru",
        "pa-u-nu-guru",
        "pcm",
        "pl",
        "ps",
        "ps-PK",
        "ps-PK-u-nu-latn",
        "ps-u-nu-latn",
        "pt",
        "pt-AO",
        "pt-CH",
        "pt-CV",
        "pt-GQ",
        "pt-GW",
        "pt-LU",
        "pt-MO",
        "pt-MZ",
        "pt-PT",
        "pt-ST",
        "pt-TL",
        "qu",
        "qu-BO",
        "qu-EC",
        "ro",
        "ro-MD",
        "ru",
        "ru-BY",
        "ru-KG",
        "ru-KZ",
        "ru-MD",
        "ru-UA",
        "sd",
        "sd-Arab",
        "sd-Arab-u-nu-latn",
        "sd-u-nu-latn",
        "si",
        "sk",
        "sl",
        "so",
        "so-DJ",
        "so-ET",
        "so-KE",
        "sq",
        "sq-MK",
        "sq-XK",
        "sr",
        "sr-Cyrl",
        "sr-Cyrl-BA",
        "sr-Cyrl-ME",
        "sr-Cyrl-XK",
        "sr-Latn",
        "sr-Latn-BA",
        "sr-Latn-ME",
        "sr-Latn-XK",
        "sv",
        "sv-AX",
        "sv-FI",
        "sw",
        "sw-CD",
        "sw-KE",
        "sw-UG",
        "ta",
        "ta-LK",
        "ta-LK-u-nu-tamldec",
        "ta-MY",
        "ta-MY-u-nu-tamldec",
        "ta-SG",
        "ta-SG-u-nu-tamldec",
        "ta-u-nu-tamldec",
        "te",
        "te-u-nu-telu",
        "th",
        "th-u-nu-thai",
        "tk",
        "to",
        "tr",
        "tr-CY",
        "uk",
        "und",
        "ur",
        "ur-IN",
        "ur-IN-u-nu-latn",
        "ur-u-nu-arabext",
        "uz",
        "uz-Latn",
        "vi",
        "yo",
        "yo-BJ",
        "yue",
        "yue-Hans",
        "yue-Hans-u-nu-hanidec",
        "yue-Hant",
        "yue-Hant-u-nu-hanidec",
        "yue-u-nu-hanidec",
        "zh",
        "zh-Hans",
        "zh-Hans-HK",
        "zh-Hans-HK-u-nu-hanidec",
        "zh-Hans-MO",
        "zh-Hans-MO-u-nu-hanidec",
        "zh-Hans-SG",
        "zh-Hans-SG-u-nu-hanidec",
        "zh-Hans-u-nu-hanidec",
        "zh-Hant",
        "zh-Hant-HK",
        "zh-Hant-HK-u-nu-hanidec",
        "zh-Hant-MO",
        "zh-Hant-MO-u-nu-hanidec",
        "zh-Hant-u-nu-hanidec",
        "zh-u-nu-hanidec",
        "zu",
    ];
    static DATA: [&DataStruct; 465usize] = [
        &AF,
        &AF,
        &AM,
        &AR,
        &AR_AE,
        &AR,
        &AR,
        &AR_AE,
        &AR,
        &AR_AE,
        &AR_DZ,
        &AR,
        &AR,
        &AR_AE,
        &AR_AE,
        &AR,
        &AR,
        &AR_AE,
        &AR,
        &AR_AE,
        &AR,
        &AR_AE,
        &AR,
        &AR_AE,
        &AR,
        &AR_AE,
        &AR,
        &AR_AE,
        &AR,
        &AR_DZ,
        &AR_DZ,
        &AR,
        &AR_DZ,
        &AR,
        &AR,
        &AR_DZ,
        &AR,
        &AR_AE,
        &AR,
        &AR_AE,
        &AR,
        &AR_AE,
        &AR,
        &AR_AE,
        &AR,
        &AR_AE,
        &AR,
        &AR_AE,
        &AR,
        &AR_AE,
        &AR,
        &AR_AE,
        &AR,
        &AR_AE,
        &AR_DZ,
        &AR,
        &AR,
        &AR_AE,
        &AR_AE,
        &AS,
        &AS_U_NU_LATN,
        &AZ,
        &AZ,
        &BE,
        &BE,
        &BE,
        &AS,
        &AS,
        &AS_U_NU_LATN,
        &AS_U_NU_LATN,
        &AZ,
        &AZ,
        &AZ,
        &AZ,
        &AZ,
        &AZ,
        &AZ,
        &AM,
        &AF,
        &AM,
        &AZ,
        &AZ,
        &AZ,
        &AF,
        &AZ,
        &DE_CH,
        &AZ,
        &DE_CH,
        &AZ,
        &AZ,
        &AZ,
        &AZ,
        &AM,
        &AM,
        &AM,
        &AM,
        &AM,
        &AM,
        &AM,
        &AZ,
        &AM,
        &AM,
        &AZ,
        &AM,
        &AM,
        &AM,
        &AM,
        &AM,
        &AM,
        &AM,
        &DE_CH,
        &AM,
        &AM,
        &AM,
        &AM,
        &AZ,
        &AM,
        &AZ,
        &AM,
        &AM,
        &AF,
        &AM,
        &AM,
        &AM,
        &AM,
        &AM,
        &AM,
        &AM,
        &AM,
        &AM,
        &AM,
        &AM,
        &AM,
        &AM,
        &AM,
        &AM,
        &AS_U_NU_LATN,
        &AM,
        &AM,
        &AM,
        &AM,
        &AM,
        &AM,
        &AM,
        &AM,
        &AM,
        &AM,
        &AM,
        &AM,
        &AM,
        &AM,
        &AM,
        &AM,
        &AM,
        &AM,
        &AM,
        &AM,
        &AM,
        &AM,
        &AM,
        &AZ,
        &AM,
        &AM,
        &AM,
        &AM,
        &AM,
        &AM,
        &AM,
        &AM,
        &AM,
        &AM,
        &AM,
        &AM,
        &AM,
        &AF,
        &AM,
        &AM,
        &AZ,
        &AM,
        &AM,
        &AM,
        &AM,
        &AM,
        &AM,
        &AM,
        &AM,
        &AM,
        &AM,
        &AM,
        &AM,
        &AM,
        &AM,
        &AM,
        &AM,
        &AM,
        &AF,
        &AM,
        &AM,
        &ES,
        &AM,
        &AZ,
        &AZ,
        &AM,
        &AM,
        &AZ,
        &AZ,
        &AF,
        &AM,
        &AM,
        &ES,
        &AZ,
        &ES,
        &AM,
        &AM,
        &ES,
        &AM,
        &AM,
        &AM,
        &AM,
        &ES,
        &AM,
        &AZ,
        &AM,
        &AM,
        &AZ,
        &AZ,
        &ET,
        &EU,
        &FA,
        &FA,
        &FA_AF_U_NU_LATN,
        &FA_AF_U_NU_LATN,
        &FI,
        &AM,
        &FR,
        &FR,
        &FR,
        &FR,
        &FR,
        &FR,
        &AF,
        &FR,
        &FR,
        &FR,
        &FR,
        &FR,
        &FR,
        &FR,
        &FR,
        &FR,
        &FR,
        &FR,
        &FR,
        &FR,
        &FR,
        &FR,
        &AZ,
        &AZ,
        &FR,
        &FR,
        &FR,
        &FR,
        &FR,
        &FR,
        &FR,
        &FR,
        &FR,
        &FR,
        &FR,
        &FR,
        &FR,
        &FR,
        &FR,
        &FR,
        &FR,
        &FR,
        &FR,
        &FR,
        &FR,
        &FR,
        &AM,
        &AM,
        &AM,
        &AZ,
        &AS_U_NU_LATN,
        &GU_U_NU_GUJR,
        &AM,
        &AM,
        &AM,
        &AR_AE,
        &AS_U_NU_LATN,
        &AS_U_NU_LATN,
        &HI_U_NU_DEVA,
        &EU,
        &EU,
        &AZ,
        &AF,
        &AF,
        &AZ,
        &AM,
        &AZ,
        &AZ,
        &DE_CH,
        &AZ,
        &AZ,
        &AM,
        &AZ,
        &JV_U_NU_JAVA,
        &BE,
        &AF,
        &AZ,
        &KM_U_NU_KHMR,
        &AM,
        &KN_U_NU_KNDA,
        &AM,
        &AM,
        &AM,
        &KOK_U_NU_DEVA,
        &AF,
        &AZ,
        &LO_U_NU_LAOO,
        &FI,
        &BE,
        &AZ,
        &AS_U_NU_LATN,
        &ML_U_NU_MLYM,
        &AM,
        &HI_U_NU_DEVA,
        &AS_U_NU_LATN,
        &AM,
        &AZ,
        &AZ,
        &AM,
        &MY,
        &AM,
        &FI,
        &FI,
        &HI_U_NU_DEVA,
        &HI_U_NU_DEVA,
        &AS_U_NU_LATN,
        &AS_U_NU_LATN,
        &AZ,
        &AZ,
        &AZ,
        &AZ,
        &AZ,
        &AZ,
        &AZ,
        &FI,
        &FI,
        &AS_U_NU_LATN,
        &OR_U_NU_ORYA,
        &AS_U_NU_LATN,
        &AS_U_NU_LATN,
        &PA_GURU_U_NU_GURU,
        &PA_GURU_U_NU_GURU,
        &AM,
        &BE,
        &PS,
        &PS,
        &PS_PK_U_NU_LATN,
        &PS_PK_U_NU_LATN,
        &AZ,
        &AF,
        &BE,
        &BE,
        &BE,
        &BE,
        &BE,
        &BE,
        &BE,
        &BE,
        &BE,
        &BE,
        &AM,
        &AZ,
        &AM,
        &AZ,
        &AZ,
        &AF,
        &AF,
        &AF,
        &AF,
        &AF,
        &BE,
        &AR,
        &AR,
        &AM,
        &AM,
        &AM,
        &AF,
        &EU,
        &AM,
        &AM,
        &AM,
        &AM,
        &BE,
        &BE,
        &BE,
        &AZ,
        &AZ,
        &AZ,
        &AZ,
        &AZ,
        &AZ,
        &AZ,
        &AZ,
        &AZ,
        &FI,
        &FI,
        &FI,
        &AM,
        &AZ,
        &AM,
        &AM,
        &AS_U_NU_LATN,
        &AS_U_NU_LATN,
        &TA_LK_U_NU_TAMLDEC,
        &AM,
        &TA_LK_U_NU_TAMLDEC,
        &AM,
        &TA_LK_U_NU_TAMLDEC,
        &TA_LK_U_NU_TAMLDEC,
        &AS_U_NU_LATN,
        &TE_U_NU_TELU,
        &AM,
        &TH_U_NU_THAI,
        &AF,
        &AM,
        &AZ,
        &AZ,
        &AF,
        &AM,
        &AR_AE,
        &PS,
        &AR_AE,
        &PS,
        &AF,
        &AF,
        &AZ,
        &AM,
        &AM,
        &AM,
        &AM,
        &YUE_HANS_U_NU_HANIDEC,
        &AM,
        &YUE_HANS_U_NU_HANIDEC,
        &YUE_HANS_U_NU_HANIDEC,
        &AM,
        &AM,
        &AM,
        &YUE_HANS_U_NU_HANIDEC,
        &AM,
        &YUE_HANS_U_NU_HANIDEC,
        &AM,
        &YUE_HANS_U_NU_HANIDEC,
        &YUE_HANS_U_NU_HANIDEC,
        &AM,
        &AM,
        &YUE_HANS_U_NU_HANIDEC,
        &AM,
        &YUE_HANS_U_NU_HANIDEC,
        &YUE_HANS_U_NU_HANIDEC,
        &YUE_HANS_U_NU_HANIDEC,
        &AM,
    ];
    KEYS.binary_search_by(|k| locale.strict_cmp(k.as_bytes()).reverse())
        .ok()
        .map(|i| unsafe { *DATA.get_unchecked(i) })
}
static AF: DataStruct = include!("af.rs.data");
static AM: DataStruct = include!("am.rs.data");
static AR_AE: DataStruct = include!("ar-AE.rs.data");
static AR_DZ: DataStruct = include!("ar-DZ.rs.data");
static AR: DataStruct = include!("ar.rs.data");
static AS_U_NU_LATN: DataStruct = include!("as-u-nu-latn.rs.data");
static AS: DataStruct = include!("as.rs.data");
static AZ: DataStruct = include!("az.rs.data");
static BE: DataStruct = include!("be.rs.data");
static DE_CH: DataStruct = include!("de-CH.rs.data");
static ES: DataStruct = include!("es.rs.data");
static ET: DataStruct = include!("et.rs.data");
static EU: DataStruct = include!("eu.rs.data");
static FA_AF_U_NU_LATN: DataStruct = include!("fa-AF-u-nu-latn.rs.data");
static FA: DataStruct = include!("fa.rs.data");
static FI: DataStruct = include!("fi.rs.data");
static FR: DataStruct = include!("fr.rs.data");
static GU_U_NU_GUJR: DataStruct = include!("gu-u-nu-gujr.rs.data");
static HI_U_NU_DEVA: DataStruct = include!("hi-u-nu-deva.rs.data");
static JV_U_NU_JAVA: DataStruct = include!("jv-u-nu-java.rs.data");
static KM_U_NU_KHMR: DataStruct = include!("km-u-nu-khmr.rs.data");
static KN_U_NU_KNDA: DataStruct = include!("kn-u-nu-knda.rs.data");
static KOK_U_NU_DEVA: DataStruct = include!("kok-u-nu-deva.rs.data");
static LO_U_NU_LAOO: DataStruct = include!("lo-u-nu-laoo.rs.data");
static ML_U_NU_MLYM: DataStruct = include!("ml-u-nu-mlym.rs.data");
static MY: DataStruct = include!("my.rs.data");
static OR_U_NU_ORYA: DataStruct = include!("or-u-nu-orya.rs.data");
static PA_GURU_U_NU_GURU: DataStruct = include!("pa-Guru-u-nu-guru.rs.data");
static PS_PK_U_NU_LATN: DataStruct = include!("ps-PK-u-nu-latn.rs.data");
static PS: DataStruct = include!("ps.rs.data");
static TA_LK_U_NU_TAMLDEC: DataStruct = include!("ta-LK-u-nu-tamldec.rs.data");
static TE_U_NU_TELU: DataStruct = include!("te-u-nu-telu.rs.data");
static TH_U_NU_THAI: DataStruct = include!("th-u-nu-thai.rs.data");
static YUE_HANS_U_NU_HANIDEC: DataStruct = include!("yue-Hans-u-nu-hanidec.rs.data");
