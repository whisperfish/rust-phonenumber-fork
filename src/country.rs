// Copyright (C) 2017 1aim GmbH
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Country related types.

use crate::error;
use std::str;

#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Hash, Debug)]
pub struct Code {
    /// The country code value.
    pub(crate) value: u16,

    /// The source from which the country code is derived.
    pub(crate) source: Source,
}

/// The source from which the country code is derived. This is not set in the
/// general parsing method, but in the method that parses and keeps raw_input.
#[derive(Eq, PartialEq, Copy, Clone, Serialize, Deserialize, Hash, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Source {
    /// The country code is derived based on a phone number with a leading "+",
    /// e.g. the French number "+33 1 42 68 53 00".
    Plus,

    /// The country code is derived based on a phone number with a leading IDD,
    /// e.g. the French number "011 33 1 42 68 53 00", as it is dialled from US.
    Idd,

    /// The country code is derived based on a phone number without a leading
    /// "+", e.g. the French number "33 1 42 68 53 00" when the default country
    /// is supplied as France.
    Number,

    /// The country code is derived NOT based on the phone number itself, but
    /// from the default country parameter provided in the parsing function by
    /// the clients. This happens mostly for numbers written in the national
    /// format (without country code). For example, this would be set when
    /// parsing the French number "01 42 68 53 00", when the default country is
    /// supplied as France.
    Default,
}

impl Default for Source {
    fn default() -> Self {
        Source::Default
    }
}

impl Code {
    /// The country code number.
    pub fn value(&self) -> u16 {
        self.value
    }

    /// How the country was inferred.
    pub fn source(&self) -> Source {
        self.source
    }
}

impl From<Code> for u16 {
    fn from(code: Code) -> u16 {
        code.value
    }
}

/// CLDR country IDs.
#[derive(Eq, PartialEq, Copy, Clone, Serialize, Deserialize, Hash, Debug)]
pub enum Id {
    AC,
    AD,
    AE,
    AF,
    AG,
    AI,
    AL,
    AM,
    AO,
    AR,
    AS,
    AT,
    AU,
    AW,
    AX,
    AZ,
    BA,
    BB,
    BD,
    BE,
    BF,
    BG,
    BH,
    BI,
    BJ,
    BL,
    BM,
    BN,
    BO,
    BQ,
    BR,
    BS,
    BT,
    BW,
    BY,
    BZ,
    CA,
    CC,
    CD,
    CF,
    CG,
    CH,
    CI,
    CK,
    CL,
    CM,
    CN,
    CO,
    CR,
    CU,
    CV,
    CW,
    CX,
    CY,
    CZ,
    DE,
    DJ,
    DK,
    DM,
    DO,
    DZ,
    EC,
    EE,
    EG,
    EH,
    ER,
    ES,
    ET,
    FI,
    FJ,
    FK,
    FM,
    FO,
    FR,
    GA,
    GB,
    GD,
    GE,
    GF,
    GG,
    GH,
    GI,
    GL,
    GM,
    GN,
    GP,
    GQ,
    GR,
    GT,
    GU,
    GW,
    GY,
    HK,
    HN,
    HR,
    HT,
    HU,
    ID,
    IE,
    IL,
    IM,
    IN,
    IO,
    IQ,
    IR,
    IS,
    IT,
    JE,
    JM,
    JO,
    JP,
    KE,
    KG,
    KH,
    KI,
    KM,
    KN,
    KP,
    KR,
    KW,
    KY,
    KZ,
    LA,
    LB,
    LC,
    LI,
    LK,
    LR,
    LS,
    LT,
    LU,
    LV,
    LY,
    MA,
    MC,
    MD,
    ME,
    MF,
    MG,
    MH,
    MK,
    ML,
    MM,
    MN,
    MO,
    MP,
    MQ,
    MR,
    MS,
    MT,
    MU,
    MV,
    MW,
    MX,
    MY,
    MZ,
    NA,
    NC,
    NE,
    NF,
    NG,
    NI,
    NL,
    NO,
    NP,
    NR,
    NU,
    NZ,
    OM,
    PA,
    PE,
    PF,
    PG,
    PH,
    PK,
    PL,
    PM,
    PR,
    PS,
    PT,
    PW,
    PY,
    QA,
    RE,
    RO,
    RS,
    RU,
    RW,
    SA,
    SB,
    SC,
    SD,
    SE,
    SG,
    SH,
    SI,
    SJ,
    SK,
    SL,
    SM,
    SN,
    SO,
    SR,
    SS,
    ST,
    SV,
    SX,
    SY,
    SZ,
    TA,
    TC,
    TD,
    TG,
    TH,
    TJ,
    TK,
    TL,
    TM,
    TN,
    TO,
    TR,
    TT,
    TV,
    TW,
    TZ,
    UA,
    UG,
    US,
    UY,
    UZ,
    VA,
    VC,
    VE,
    VG,
    VI,
    VN,
    VU,
    WF,
    WS,
    XK,
    YE,
    YT,
    ZA,
    ZM,
    ZW,
}

pub use self::Id::*;

impl str::FromStr for Id {
    type Err = error::Parse;

    fn from_str(value: &str) -> Result<Id, error::Parse> {
        match value {
            "AC" => Ok(Id::AC),
            "AD" => Ok(Id::AD),
            "AE" => Ok(Id::AE),
            "AF" => Ok(Id::AF),
            "AG" => Ok(Id::AG),
            "AI" => Ok(Id::AI),
            "AL" => Ok(Id::AL),
            "AM" => Ok(Id::AM),
            "AO" => Ok(Id::AO),
            "AR" => Ok(Id::AR),
            "AS" => Ok(Id::AS),
            "AT" => Ok(Id::AT),
            "AU" => Ok(Id::AU),
            "AW" => Ok(Id::AW),
            "AX" => Ok(Id::AX),
            "AZ" => Ok(Id::AZ),
            "BA" => Ok(Id::BA),
            "BB" => Ok(Id::BB),
            "BD" => Ok(Id::BD),
            "BE" => Ok(Id::BE),
            "BF" => Ok(Id::BF),
            "BG" => Ok(Id::BG),
            "BH" => Ok(Id::BH),
            "BI" => Ok(Id::BI),
            "BJ" => Ok(Id::BJ),
            "BL" => Ok(Id::BL),
            "BM" => Ok(Id::BM),
            "BN" => Ok(Id::BN),
            "BO" => Ok(Id::BO),
            "BQ" => Ok(Id::BQ),
            "BR" => Ok(Id::BR),
            "BS" => Ok(Id::BS),
            "BT" => Ok(Id::BT),
            "BW" => Ok(Id::BW),
            "BY" => Ok(Id::BY),
            "BZ" => Ok(Id::BZ),
            "CA" => Ok(Id::CA),
            "CC" => Ok(Id::CC),
            "CD" => Ok(Id::CD),
            "CF" => Ok(Id::CF),
            "CG" => Ok(Id::CG),
            "CH" => Ok(Id::CH),
            "CI" => Ok(Id::CI),
            "CK" => Ok(Id::CK),
            "CL" => Ok(Id::CL),
            "CM" => Ok(Id::CM),
            "CN" => Ok(Id::CN),
            "CO" => Ok(Id::CO),
            "CR" => Ok(Id::CR),
            "CU" => Ok(Id::CU),
            "CV" => Ok(Id::CV),
            "CW" => Ok(Id::CW),
            "CX" => Ok(Id::CX),
            "CY" => Ok(Id::CY),
            "CZ" => Ok(Id::CZ),
            "DE" => Ok(Id::DE),
            "DJ" => Ok(Id::DJ),
            "DK" => Ok(Id::DK),
            "DM" => Ok(Id::DM),
            "DO" => Ok(Id::DO),
            "DZ" => Ok(Id::DZ),
            "EC" => Ok(Id::EC),
            "EE" => Ok(Id::EE),
            "EG" => Ok(Id::EG),
            "EH" => Ok(Id::EH),
            "ER" => Ok(Id::ER),
            "ES" => Ok(Id::ES),
            "ET" => Ok(Id::ET),
            "FI" => Ok(Id::FI),
            "FJ" => Ok(Id::FJ),
            "FK" => Ok(Id::FK),
            "FM" => Ok(Id::FM),
            "FO" => Ok(Id::FO),
            "FR" => Ok(Id::FR),
            "GA" => Ok(Id::GA),
            "GB" => Ok(Id::GB),
            "GD" => Ok(Id::GD),
            "GE" => Ok(Id::GE),
            "GF" => Ok(Id::GF),
            "GG" => Ok(Id::GG),
            "GH" => Ok(Id::GH),
            "GI" => Ok(Id::GI),
            "GL" => Ok(Id::GL),
            "GM" => Ok(Id::GM),
            "GN" => Ok(Id::GN),
            "GP" => Ok(Id::GP),
            "GQ" => Ok(Id::GQ),
            "GR" => Ok(Id::GR),
            "GT" => Ok(Id::GT),
            "GU" => Ok(Id::GU),
            "GW" => Ok(Id::GW),
            "GY" => Ok(Id::GY),
            "HK" => Ok(Id::HK),
            "HN" => Ok(Id::HN),
            "HR" => Ok(Id::HR),
            "HT" => Ok(Id::HT),
            "HU" => Ok(Id::HU),
            "ID" => Ok(Id::ID),
            "IE" => Ok(Id::IE),
            "IL" => Ok(Id::IL),
            "IM" => Ok(Id::IM),
            "IN" => Ok(Id::IN),
            "IO" => Ok(Id::IO),
            "IQ" => Ok(Id::IQ),
            "IR" => Ok(Id::IR),
            "IS" => Ok(Id::IS),
            "IT" => Ok(Id::IT),
            "JE" => Ok(Id::JE),
            "JM" => Ok(Id::JM),
            "JO" => Ok(Id::JO),
            "JP" => Ok(Id::JP),
            "KE" => Ok(Id::KE),
            "KG" => Ok(Id::KG),
            "KH" => Ok(Id::KH),
            "KI" => Ok(Id::KI),
            "KM" => Ok(Id::KM),
            "KN" => Ok(Id::KN),
            "KP" => Ok(Id::KP),
            "KR" => Ok(Id::KR),
            "KW" => Ok(Id::KW),
            "KY" => Ok(Id::KY),
            "KZ" => Ok(Id::KZ),
            "LA" => Ok(Id::LA),
            "LB" => Ok(Id::LB),
            "LC" => Ok(Id::LC),
            "LI" => Ok(Id::LI),
            "LK" => Ok(Id::LK),
            "LR" => Ok(Id::LR),
            "LS" => Ok(Id::LS),
            "LT" => Ok(Id::LT),
            "LU" => Ok(Id::LU),
            "LV" => Ok(Id::LV),
            "LY" => Ok(Id::LY),
            "MA" => Ok(Id::MA),
            "MC" => Ok(Id::MC),
            "MD" => Ok(Id::MD),
            "ME" => Ok(Id::ME),
            "MF" => Ok(Id::MF),
            "MG" => Ok(Id::MG),
            "MH" => Ok(Id::MH),
            "MK" => Ok(Id::MK),
            "ML" => Ok(Id::ML),
            "MM" => Ok(Id::MM),
            "MN" => Ok(Id::MN),
            "MO" => Ok(Id::MO),
            "MP" => Ok(Id::MP),
            "MQ" => Ok(Id::MQ),
            "MR" => Ok(Id::MR),
            "MS" => Ok(Id::MS),
            "MT" => Ok(Id::MT),
            "MU" => Ok(Id::MU),
            "MV" => Ok(Id::MV),
            "MW" => Ok(Id::MW),
            "MX" => Ok(Id::MX),
            "MY" => Ok(Id::MY),
            "MZ" => Ok(Id::MZ),
            "NA" => Ok(Id::NA),
            "NC" => Ok(Id::NC),
            "NE" => Ok(Id::NE),
            "NF" => Ok(Id::NF),
            "NG" => Ok(Id::NG),
            "NI" => Ok(Id::NI),
            "NL" => Ok(Id::NL),
            "NO" => Ok(Id::NO),
            "NP" => Ok(Id::NP),
            "NR" => Ok(Id::NR),
            "NU" => Ok(Id::NU),
            "NZ" => Ok(Id::NZ),
            "OM" => Ok(Id::OM),
            "PA" => Ok(Id::PA),
            "PE" => Ok(Id::PE),
            "PF" => Ok(Id::PF),
            "PG" => Ok(Id::PG),
            "PH" => Ok(Id::PH),
            "PK" => Ok(Id::PK),
            "PL" => Ok(Id::PL),
            "PM" => Ok(Id::PM),
            "PR" => Ok(Id::PR),
            "PS" => Ok(Id::PS),
            "PT" => Ok(Id::PT),
            "PW" => Ok(Id::PW),
            "PY" => Ok(Id::PY),
            "QA" => Ok(Id::QA),
            "RE" => Ok(Id::RE),
            "RO" => Ok(Id::RO),
            "RS" => Ok(Id::RS),
            "RU" => Ok(Id::RU),
            "RW" => Ok(Id::RW),
            "SA" => Ok(Id::SA),
            "SB" => Ok(Id::SB),
            "SC" => Ok(Id::SC),
            "SD" => Ok(Id::SD),
            "SE" => Ok(Id::SE),
            "SG" => Ok(Id::SG),
            "SH" => Ok(Id::SH),
            "SI" => Ok(Id::SI),
            "SJ" => Ok(Id::SJ),
            "SK" => Ok(Id::SK),
            "SL" => Ok(Id::SL),
            "SM" => Ok(Id::SM),
            "SN" => Ok(Id::SN),
            "SO" => Ok(Id::SO),
            "SR" => Ok(Id::SR),
            "SS" => Ok(Id::SS),
            "ST" => Ok(Id::ST),
            "SV" => Ok(Id::SV),
            "SX" => Ok(Id::SX),
            "SY" => Ok(Id::SY),
            "SZ" => Ok(Id::SZ),
            "TA" => Ok(Id::TA),
            "TC" => Ok(Id::TC),
            "TD" => Ok(Id::TD),
            "TG" => Ok(Id::TG),
            "TH" => Ok(Id::TH),
            "TJ" => Ok(Id::TJ),
            "TK" => Ok(Id::TK),
            "TL" => Ok(Id::TL),
            "TM" => Ok(Id::TM),
            "TN" => Ok(Id::TN),
            "TO" => Ok(Id::TO),
            "TR" => Ok(Id::TR),
            "TT" => Ok(Id::TT),
            "TV" => Ok(Id::TV),
            "TW" => Ok(Id::TW),
            "TZ" => Ok(Id::TZ),
            "UA" => Ok(Id::UA),
            "UG" => Ok(Id::UG),
            "US" => Ok(Id::US),
            "UY" => Ok(Id::UY),
            "UZ" => Ok(Id::UZ),
            "VA" => Ok(Id::VA),
            "VC" => Ok(Id::VC),
            "VE" => Ok(Id::VE),
            "VG" => Ok(Id::VG),
            "VI" => Ok(Id::VI),
            "VN" => Ok(Id::VN),
            "VU" => Ok(Id::VU),
            "WF" => Ok(Id::WF),
            "WS" => Ok(Id::WS),
            "XK" => Ok(Id::XK),
            "YE" => Ok(Id::YE),
            "YT" => Ok(Id::YT),
            "ZA" => Ok(Id::ZA),
            "ZM" => Ok(Id::ZM),
            "ZW" => Ok(Id::ZW),
            _ => Err(error::Parse::InvalidCountryCode),
        }
    }
}

impl AsRef<str> for Id {
    fn as_ref(&self) -> &str {
        match *self {
            Id::AC => "AC",
            Id::AD => "AD",
            Id::AE => "AE",
            Id::AF => "AF",
            Id::AG => "AG",
            Id::AI => "AI",
            Id::AL => "AL",
            Id::AM => "AM",
            Id::AO => "AO",
            Id::AR => "AR",
            Id::AS => "AS",
            Id::AT => "AT",
            Id::AU => "AU",
            Id::AW => "AW",
            Id::AX => "AX",
            Id::AZ => "AZ",
            Id::BA => "BA",
            Id::BB => "BB",
            Id::BD => "BD",
            Id::BE => "BE",
            Id::BF => "BF",
            Id::BG => "BG",
            Id::BH => "BH",
            Id::BI => "BI",
            Id::BJ => "BJ",
            Id::BL => "BL",
            Id::BM => "BM",
            Id::BN => "BN",
            Id::BO => "BO",
            Id::BQ => "BQ",
            Id::BR => "BR",
            Id::BS => "BS",
            Id::BT => "BT",
            Id::BW => "BW",
            Id::BY => "BY",
            Id::BZ => "BZ",
            Id::CA => "CA",
            Id::CC => "CC",
            Id::CD => "CD",
            Id::CF => "CF",
            Id::CG => "CG",
            Id::CH => "CH",
            Id::CI => "CI",
            Id::CK => "CK",
            Id::CL => "CL",
            Id::CM => "CM",
            Id::CN => "CN",
            Id::CO => "CO",
            Id::CR => "CR",
            Id::CU => "CU",
            Id::CV => "CV",
            Id::CW => "CW",
            Id::CX => "CX",
            Id::CY => "CY",
            Id::CZ => "CZ",
            Id::DE => "DE",
            Id::DJ => "DJ",
            Id::DK => "DK",
            Id::DM => "DM",
            Id::DO => "DO",
            Id::DZ => "DZ",
            Id::EC => "EC",
            Id::EE => "EE",
            Id::EG => "EG",
            Id::EH => "EH",
            Id::ER => "ER",
            Id::ES => "ES",
            Id::ET => "ET",
            Id::FI => "FI",
            Id::FJ => "FJ",
            Id::FK => "FK",
            Id::FM => "FM",
            Id::FO => "FO",
            Id::FR => "FR",
            Id::GA => "GA",
            Id::GB => "GB",
            Id::GD => "GD",
            Id::GE => "GE",
            Id::GF => "GF",
            Id::GG => "GG",
            Id::GH => "GH",
            Id::GI => "GI",
            Id::GL => "GL",
            Id::GM => "GM",
            Id::GN => "GN",
            Id::GP => "GP",
            Id::GQ => "GQ",
            Id::GR => "GR",
            Id::GT => "GT",
            Id::GU => "GU",
            Id::GW => "GW",
            Id::GY => "GY",
            Id::HK => "HK",
            Id::HN => "HN",
            Id::HR => "HR",
            Id::HT => "HT",
            Id::HU => "HU",
            Id::ID => "ID",
            Id::IE => "IE",
            Id::IL => "IL",
            Id::IM => "IM",
            Id::IN => "IN",
            Id::IO => "IO",
            Id::IQ => "IQ",
            Id::IR => "IR",
            Id::IS => "IS",
            Id::IT => "IT",
            Id::JE => "JE",
            Id::JM => "JM",
            Id::JO => "JO",
            Id::JP => "JP",
            Id::KE => "KE",
            Id::KG => "KG",
            Id::KH => "KH",
            Id::KI => "KI",
            Id::KM => "KM",
            Id::KN => "KN",
            Id::KP => "KP",
            Id::KR => "KR",
            Id::KW => "KW",
            Id::KY => "KY",
            Id::KZ => "KZ",
            Id::LA => "LA",
            Id::LB => "LB",
            Id::LC => "LC",
            Id::LI => "LI",
            Id::LK => "LK",
            Id::LR => "LR",
            Id::LS => "LS",
            Id::LT => "LT",
            Id::LU => "LU",
            Id::LV => "LV",
            Id::LY => "LY",
            Id::MA => "MA",
            Id::MC => "MC",
            Id::MD => "MD",
            Id::ME => "ME",
            Id::MF => "MF",
            Id::MG => "MG",
            Id::MH => "MH",
            Id::MK => "MK",
            Id::ML => "ML",
            Id::MM => "MM",
            Id::MN => "MN",
            Id::MO => "MO",
            Id::MP => "MP",
            Id::MQ => "MQ",
            Id::MR => "MR",
            Id::MS => "MS",
            Id::MT => "MT",
            Id::MU => "MU",
            Id::MV => "MV",
            Id::MW => "MW",
            Id::MX => "MX",
            Id::MY => "MY",
            Id::MZ => "MZ",
            Id::NA => "NA",
            Id::NC => "NC",
            Id::NE => "NE",
            Id::NF => "NF",
            Id::NG => "NG",
            Id::NI => "NI",
            Id::NL => "NL",
            Id::NO => "NO",
            Id::NP => "NP",
            Id::NR => "NR",
            Id::NU => "NU",
            Id::NZ => "NZ",
            Id::OM => "OM",
            Id::PA => "PA",
            Id::PE => "PE",
            Id::PF => "PF",
            Id::PG => "PG",
            Id::PH => "PH",
            Id::PK => "PK",
            Id::PL => "PL",
            Id::PM => "PM",
            Id::PR => "PR",
            Id::PS => "PS",
            Id::PT => "PT",
            Id::PW => "PW",
            Id::PY => "PY",
            Id::QA => "QA",
            Id::RE => "RE",
            Id::RO => "RO",
            Id::RS => "RS",
            Id::RU => "RU",
            Id::RW => "RW",
            Id::SA => "SA",
            Id::SB => "SB",
            Id::SC => "SC",
            Id::SD => "SD",
            Id::SE => "SE",
            Id::SG => "SG",
            Id::SH => "SH",
            Id::SI => "SI",
            Id::SJ => "SJ",
            Id::SK => "SK",
            Id::SL => "SL",
            Id::SM => "SM",
            Id::SN => "SN",
            Id::SO => "SO",
            Id::SR => "SR",
            Id::SS => "SS",
            Id::ST => "ST",
            Id::SV => "SV",
            Id::SX => "SX",
            Id::SY => "SY",
            Id::SZ => "SZ",
            Id::TA => "TA",
            Id::TC => "TC",
            Id::TD => "TD",
            Id::TG => "TG",
            Id::TH => "TH",
            Id::TJ => "TJ",
            Id::TK => "TK",
            Id::TL => "TL",
            Id::TM => "TM",
            Id::TN => "TN",
            Id::TO => "TO",
            Id::TR => "TR",
            Id::TT => "TT",
            Id::TV => "TV",
            Id::TW => "TW",
            Id::TZ => "TZ",
            Id::UA => "UA",
            Id::UG => "UG",
            Id::US => "US",
            Id::UY => "UY",
            Id::UZ => "UZ",
            Id::VA => "VA",
            Id::VC => "VC",
            Id::VE => "VE",
            Id::VG => "VG",
            Id::VI => "VI",
            Id::VN => "VN",
            Id::VU => "VU",
            Id::WF => "WF",
            Id::WS => "WS",
            Id::XK => "XK",
            Id::YE => "YE",
            Id::YT => "YT",
            Id::ZA => "ZA",
            Id::ZM => "ZM",
            Id::ZW => "ZW",
        }
    }
}
