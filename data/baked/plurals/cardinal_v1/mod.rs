// @generated
#![cfg(feature = "icu_plurals")]
type DataStruct =
    <::icu_plurals::provider::CardinalV1Marker as ::icu_provider::DataMarker>::Yokeable;
pub fn lookup(locale: &icu_provider::DataLocale) -> Option<&'static DataStruct> {
    static KEYS: [&str; 216usize] = [
        "af", "ak", "am", "an", "ar", "ars", "as", "asa", "ast", "az", "bal", "be", "bem", "bez",
        "bg", "bho", "bm", "bn", "bo", "br", "brx", "bs", "ca", "ce", "ceb", "cgg", "chr", "ckb",
        "cs", "cy", "da", "de", "doi", "dsb", "dv", "dz", "ee", "el", "en", "eo", "es", "et", "eu",
        "fa", "ff", "fi", "fil", "fo", "fr", "fur", "fy", "ga", "gd", "gl", "gsw", "gu", "guw",
        "gv", "ha", "haw", "he", "hi", "hnj", "hr", "hsb", "hu", "hy", "ia", "id", "ig", "ii",
        "io", "is", "it", "iu", "ja", "jbo", "jgo", "jmc", "jv", "jw", "ka", "kab", "kaj", "kcg",
        "kde", "kea", "kk", "kkj", "kl", "km", "kn", "ko", "ks", "ksb", "ksh", "ku", "kw", "ky",
        "lag", "lb", "lg", "lij", "lkt", "ln", "lo", "lt", "lv", "mas", "mg", "mgo", "mk", "ml",
        "mn", "mo", "mr", "ms", "mt", "my", "nah", "naq", "nb", "nd", "ne", "nl", "nn", "nnh",
        "no", "nqo", "nr", "nso", "ny", "nyn", "om", "or", "os", "osa", "pa", "pap", "pcm", "pl",
        "prg", "ps", "pt", "pt-PT", "rm", "ro", "rof", "ru", "rwk", "sah", "saq", "sat", "sc",
        "scn", "sd", "sdh", "se", "seh", "ses", "sg", "sh", "shi", "si", "sk", "sl", "sma", "smi",
        "smj", "smn", "sms", "sn", "so", "sq", "sr", "ss", "ssy", "st", "su", "sv", "sw", "syr",
        "ta", "te", "teo", "th", "ti", "tig", "tk", "tl", "tn", "to", "tpi", "tr", "ts", "tzm",
        "ug", "uk", "und", "ur", "uz", "ve", "vec", "vi", "vo", "vun", "wa", "wae", "wo", "xh",
        "xog", "yi", "yo", "yue", "zh", "zu",
    ];
    static DATA: [&DataStruct; 216usize] = [
        &AF, &AK, &AM, &AF, &AR, &AR, &AM, &AF, &AST, &AF, &AF, &BE, &AF, &AF, &AF, &AK, &BM, &AM,
        &BM, &BR, &AF, &BS, &CA, &AF, &CEB, &AF, &AF, &AF, &CS, &CY, &DA, &AST, &AM, &DSB, &AF,
        &BM, &AF, &AF, &AST, &AF, &ES, &AST, &AF, &AM, &FF, &AST, &CEB, &AF, &FR, &AF, &AST, &GA,
        &GD, &AST, &AF, &AM, &AK, &GV, &AF, &AF, &HE, &AM, &BM, &BS, &DSB, &AF, &FF, &AST, &BM,
        &BM, &BM, &AST, &IS, &CA, &IU, &BM, &BM, &AF, &AF, &BM, &BM, &AF, &FF, &AF, &AF, &BM, &BM,
        &AF, &AF, &AF, &BM, &AM, &BM, &AF, &AF, &KSH, &AF, &KW, &AF, &LAG, &AF, &AF, &AST, &BM,
        &AK, &BM, &LT, &LV, &AF, &AK, &AF, &MK, &AF, &AF, &MO, &AF, &BM, &MT, &BM, &AF, &IU, &AF,
        &AF, &AF, &AST, &AF, &AF, &AF, &BM, &AF, &AK, &AF, &AF, &AF, &AF, &AF, &BM, &AK, &AF, &AM,
        &PL, &LV, &AF, &PT, &CA, &AF, &MO, &AF, &RU, &AF, &BM, &AF, &IU, &AST, &AST, &AF, &AF, &IU,
        &AF, &BM, &BM, &BS, &SHI, &SI, &CS, &SL, &IU, &IU, &IU, &IU, &IU, &AF, &AF, &AF, &BS, &AF,
        &AF, &AF, &BM, &AST, &AST, &AF, &AF, &AF, &AF, &BM, &AK, &AF, &AF, &CEB, &AF, &BM, &BM,
        &AF, &AF, &TZM, &AF, &RU, &BM, &AST, &AF, &AF, &CA, &BM, &AF, &AF, &AK, &AF, &BM, &AF, &AF,
        &AST, &BM, &BM, &BM, &AM,
    ];
    KEYS.binary_search_by(|k| locale.strict_cmp(k.as_bytes()).reverse())
        .ok()
        .map(|i| unsafe { *DATA.get_unchecked(i) })
}
static AF: DataStruct = include!("af.rs.data");
static AK: DataStruct = include!("ak.rs.data");
static AM: DataStruct = include!("am.rs.data");
static AR: DataStruct = include!("ar.rs.data");
static AST: DataStruct = include!("ast.rs.data");
static BE: DataStruct = include!("be.rs.data");
static BM: DataStruct = include!("bm.rs.data");
static BR: DataStruct = include!("br.rs.data");
static BS: DataStruct = include!("bs.rs.data");
static CA: DataStruct = include!("ca.rs.data");
static CEB: DataStruct = include!("ceb.rs.data");
static CS: DataStruct = include!("cs.rs.data");
static CY: DataStruct = include!("cy.rs.data");
static DA: DataStruct = include!("da.rs.data");
static DSB: DataStruct = include!("dsb.rs.data");
static ES: DataStruct = include!("es.rs.data");
static FF: DataStruct = include!("ff.rs.data");
static FR: DataStruct = include!("fr.rs.data");
static GA: DataStruct = include!("ga.rs.data");
static GD: DataStruct = include!("gd.rs.data");
static GV: DataStruct = include!("gv.rs.data");
static HE: DataStruct = include!("he.rs.data");
static IS: DataStruct = include!("is.rs.data");
static IU: DataStruct = include!("iu.rs.data");
static KSH: DataStruct = include!("ksh.rs.data");
static KW: DataStruct = include!("kw.rs.data");
static LAG: DataStruct = include!("lag.rs.data");
static LT: DataStruct = include!("lt.rs.data");
static LV: DataStruct = include!("lv.rs.data");
static MK: DataStruct = include!("mk.rs.data");
static MO: DataStruct = include!("mo.rs.data");
static MT: DataStruct = include!("mt.rs.data");
static PL: DataStruct = include!("pl.rs.data");
static PT: DataStruct = include!("pt.rs.data");
static RU: DataStruct = include!("ru.rs.data");
static SHI: DataStruct = include!("shi.rs.data");
static SI: DataStruct = include!("si.rs.data");
static SL: DataStruct = include!("sl.rs.data");
static TZM: DataStruct = include!("tzm.rs.data");
