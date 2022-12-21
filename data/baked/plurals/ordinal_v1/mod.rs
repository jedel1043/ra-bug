// @generated
#![cfg(feature = "icu_plurals")]
type DataStruct =
    <::icu_plurals::provider::OrdinalV1Marker as ::icu_provider::DataMarker>::Yokeable;
pub fn lookup(locale: &icu_provider::DataLocale) -> Option<&'static DataStruct> {
    static KEYS: [&str; 102usize] = [
        "af", "am", "an", "ar", "as", "ast", "az", "bal", "be", "bg", "bn", "bs", "ca", "ce", "cs",
        "cy", "da", "de", "dsb", "el", "en", "es", "et", "eu", "fa", "fi", "fil", "fr", "fy", "ga",
        "gd", "gl", "gsw", "gu", "he", "hi", "hr", "hsb", "hu", "hy", "ia", "id", "is", "it", "ja",
        "ka", "kk", "km", "kn", "ko", "kw", "ky", "lij", "lo", "lt", "lv", "mk", "ml", "mn", "mo",
        "mr", "ms", "my", "nb", "ne", "nl", "no", "or", "pa", "pl", "prg", "ps", "pt", "ro", "ru",
        "sc", "scn", "sd", "sh", "si", "sk", "sl", "sq", "sr", "sv", "sw", "ta", "te", "th", "tk",
        "tl", "tpi", "tr", "uk", "und", "ur", "uz", "vec", "vi", "yue", "zh", "zu",
    ];
    static DATA: [&DataStruct; 102usize] = [
        &AF, &AF, &AF, &AF, &AS, &AF, &AZ, &BAL, &BE, &AF, &AS, &AF, &CA, &AF, &AF, &CY, &AF, &AF,
        &AF, &AF, &EN, &AF, &AF, &AF, &AF, &AF, &BAL, &BAL, &AF, &BAL, &GD, &AF, &AF, &GU, &AF,
        &GU, &AF, &AF, &HU, &BAL, &AF, &AF, &AF, &IT, &AF, &KA, &KK, &AF, &AF, &AF, &KW, &AF, &LIJ,
        &BAL, &AF, &AF, &MK, &AF, &AF, &BAL, &MR, &BAL, &AF, &AF, &NE, &AF, &AF, &OR, &AF, &AF,
        &AF, &AF, &AF, &BAL, &AF, &IT, &IT, &AF, &AF, &AF, &AF, &AF, &SQ, &AF, &SV, &AF, &AF, &AF,
        &AF, &TK, &BAL, &AF, &AF, &UK, &AF, &AF, &AF, &IT, &BAL, &AF, &AF, &AF,
    ];
    KEYS.binary_search_by(|k| locale.strict_cmp(k.as_bytes()).reverse())
        .ok()
        .map(|i| unsafe { *DATA.get_unchecked(i) })
}
static AF: DataStruct = include!("af.rs.data");
static AS: DataStruct = include!("as.rs.data");
static AZ: DataStruct = include!("az.rs.data");
static BAL: DataStruct = include!("bal.rs.data");
static BE: DataStruct = include!("be.rs.data");
static CA: DataStruct = include!("ca.rs.data");
static CY: DataStruct = include!("cy.rs.data");
static EN: DataStruct = include!("en.rs.data");
static GD: DataStruct = include!("gd.rs.data");
static GU: DataStruct = include!("gu.rs.data");
static HU: DataStruct = include!("hu.rs.data");
static IT: DataStruct = include!("it.rs.data");
static KA: DataStruct = include!("ka.rs.data");
static KK: DataStruct = include!("kk.rs.data");
static KW: DataStruct = include!("kw.rs.data");
static LIJ: DataStruct = include!("lij.rs.data");
static MK: DataStruct = include!("mk.rs.data");
static MR: DataStruct = include!("mr.rs.data");
static NE: DataStruct = include!("ne.rs.data");
static OR: DataStruct = include!("or.rs.data");
static SQ: DataStruct = include!("sq.rs.data");
static SV: DataStruct = include!("sv.rs.data");
static TK: DataStruct = include!("tk.rs.data");
static UK: DataStruct = include!("uk.rs.data");
