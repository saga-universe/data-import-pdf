use crate::schema::countries;
use diesel::QueryDsl;
use diesel::{insert_into, Insertable, MysqlConnection, Queryable, RunQueryDsl};
use serde::Deserialize;

#[derive(Debug, Insertable, Clone)]
#[diesel(table_name = countries)]
pub struct Country<'a> {
    pub name: &'a str,
    pub code: &'a str,
}

#[derive(Debug, Queryable, Deserialize)]
#[diesel(table_name = countries)]
pub struct FullCountry {
    pub id: u32,
    pub name: String,
    pub code: String,
}

pub const LIST_OF_COUNTRY: [Country; 250] = [
    Country {
        name: "Afghanistan",
        code: "AF",
    },
    Country {
        name: "Îles Åland",
        code: "AX",
    },
    Country {
        name: "Albanie",
        code: "AL",
    },
    Country {
        name: "Algérie",
        code: "DZ",
    },
    Country {
        name: "Samoa américaines",
        code: "AS",
    },
    Country {
        name: "Andorre",
        code: "AD",
    },
    Country {
        name: "Angola",
        code: "AO",
    },
    Country {
        name: "Anguilla",
        code: "AI",
    },
    Country {
        name: "Antarctique",
        code: "AQ",
    },
    Country {
        name: "Antigua-et-Barbuda",
        code: "AG",
    },
    Country {
        name: "Argentine",
        code: "AR",
    },
    Country {
        name: "Arménie",
        code: "AM",
    },
    Country {
        name: "Aruba",
        code: "AW",
    },
    Country {
        name: "Australie",
        code: "AU",
    },
    Country {
        name: "Autriche",
        code: "AT",
    },
    Country {
        name: "Azerbaïdjan",
        code: "AZ",
    },
    Country {
        name: "Bahamas",
        code: "BS",
    },
    Country {
        name: "Bahreïn",
        code: "BH",
    },
    Country {
        name: "Bangladesh",
        code: "BD",
    },
    Country {
        name: "Barbade",
        code: "BB",
    },
    Country {
        name: "Bélarus",
        code: "BY",
    },
    Country {
        name: "Belgique",
        code: "BE",
    },
    Country {
        name: "Belize",
        code: "BZ",
    },
    Country {
        name: "Bénin",
        code: "BJ",
    },
    Country {
        name: "Bermudes",
        code: "BM",
    },
    Country {
        name: "Bhoutan",
        code: "BT",
    },
    Country {
        name: "Bolivie",
        code: "BO",
    },
    Country {
        name: "Bonaire, Saint-Eustache et Saba",
        code: "BQ",
    },
    Country {
        name: "Bosnie-Herzégovine",
        code: "BA",
    },
    Country {
        name: "Botswana",
        code: "BW",
    },
    Country {
        name: "Bouvet",
        code: "BV",
    },
    Country {
        name: "Brésil",
        code: "BR",
    },
    Country {
        name: "Indien",
        code: "IO",
    },
    Country {
        name: "Brunéi Darussalam",
        code: "BN",
    },
    Country {
        name: "Bulgarie",
        code: "BG",
    },
    Country {
        name: "Burkina Faso",
        code: "BF",
    },
    Country {
        name: "Burundi",
        code: "BI",
    },
    Country {
        name: "Cabo Verde",
        code: "CV",
    },
    Country {
        name: "Cambodge",
        code: "KH",
    },
    Country {
        name: "Cameroun",
        code: "CM",
    },
    Country {
        name: "Canada",
        code: "CA",
    },
    Country {
        name: "Caïmans",
        code: "KY",
    },
    Country {
        name: "République centrafricaine",
        code: "CF",
    },
    Country {
        name: "Tchad",
        code: "TD",
    },
    Country {
        name: "Chili",
        code: "CL",
    },
    Country {
        name: "Chine",
        code: "CN",
    },
    Country {
        name: "Christmas",
        code: "CX",
    },
    Country {
        name: "Îles Cocos/Keeling",
        code: "CC",
    },
    Country {
        name: "Colombie",
        code: "CO",
    },
    Country {
        name: "Comores",
        code: "KM",
    },
    Country {
        name: "République démocratique du Congo",
        code: "CD",
    },
    Country {
        name: "Congo",
        code: "CG",
    },
    Country {
        name: "Îles Cook",
        code: "CK",
    },
    Country {
        name: "Costa Rica",
        code: "CR",
    },
    Country {
        name: "Croatie",
        code: "HR",
    },
    Country {
        name: "Cuba",
        code: "CU",
    },
    Country {
        name: "Curaçao",
        code: "CW",
    },
    Country {
        name: "Chypre",
        code: "CY",
    },
    Country {
        name: "Tchéquie",
        code: "CZ",
    },
    Country {
        name: "Danemark",
        code: "DK",
    },
    Country {
        name: "Djibouti",
        code: "DJ",
    },
    Country {
        name: "Dominique",
        code: "DM",
    },
    Country {
        name: "République dominicaine",
        code: "DO",
    },
    Country {
        name: "Équateur",
        code: "EC",
    },
    Country {
        name: "Égypte",
        code: "EG",
    },
    Country {
        name: "El Salvador",
        code: "SV",
    },
    Country {
        name: "Guinée équatoriale",
        code: "GQ",
    },
    Country {
        name: "Érythrée",
        code: "ER",
    },
    Country {
        name: "Estonie",
        code: "EE",
    },
    Country {
        name: "Eswatini",
        code: "SZ",
    },
    Country {
        name: "Éthiopie",
        code: "ET",
    },
    Country {
        name: "Îles Falkland/Malouines",
        code: "FK",
    },
    Country {
        name: "Îles Féroé",
        code: "FO",
    },
    Country {
        name: "Fidji",
        code: "FJ",
    },
    Country {
        name: "Finlande",
        code: "FI",
    },
    Country {
        name: "France",
        code: "FR",
    },
    Country {
        name: "Guyane française",
        code: "GF",
    },
    Country {
        name: "Polynésie française",
        code: "PF",
    },
    Country {
        name: "Terres australes françaises",
        code: "TF",
    },
    Country {
        name: "Gabon",
        code: "GA",
    },
    Country {
        name: "Gambie",
        code: "GM",
    },
    Country {
        name: "Géorgie",
        code: "GE",
    },
    Country {
        name: "Allemagne",
        code: "DE",
    },
    Country {
        name: "Ghana",
        code: "GH",
    },
    Country {
        name: "Gibraltar",
        code: "GI",
    },
    Country {
        name: "Grèce",
        code: "GR",
    },
    Country {
        name: "Groenland",
        code: "GL",
    },
    Country {
        name: "Grenade",
        code: "GD",
    },
    Country {
        name: "Guadeloupe",
        code: "GP",
    },
    Country {
        name: "Guam",
        code: "GU",
    },
    Country {
        name: "Guatemala",
        code: "GT",
    },
    Country {
        name: "Guernesey",
        code: "GG",
    },
    Country {
        name: "Guinée",
        code: "GN",
    },
    Country {
        name: "Guinée-Bissau",
        code: "GW",
    },
    Country {
        name: "Guyana",
        code: "GY",
    },
    Country {
        name: "Haïti",
        code: "HT",
    },
    Country {
        name: "Îles Heard et MacDonald",
        code: "HM",
    },
    Country {
        name: "Saint-Siège",
        code: "VA",
    },
    Country {
        name: "Honduras",
        code: "HN",
    },
    Country {
        name: "Hong Kong",
        code: "HK",
    },
    Country {
        name: "Hongrie",
        code: "HU",
    },
    Country {
        name: "Islande",
        code: "IS",
    },
    Country {
        name: "Inde",
        code: "IN",
    },
    Country {
        name: "Indonésie",
        code: "ID",
    },
    Country {
        name: "Iran",
        code: "IR",
    },
    Country {
        name: "Iraq",
        code: "IQ",
    },
    Country {
        name: "Irlande",
        code: "IE",
    },
    Country {
        name: "Île de Man",
        code: "IM",
    },
    Country {
        name: "Israël",
        code: "IL",
    },
    Country {
        name: "Italie",
        code: "IT",
    },
    Country {
        name: "Côte d'Ivoire",
        code: "CI",
    },
    Country {
        name: "Jamaïque",
        code: "JM",
    },
    Country {
        name: "Japon",
        code: "JP",
    },
    Country {
        name: "Jersey",
        code: "JE",
    },
    Country {
        name: "Jordanie",
        code: "JO",
    },
    Country {
        name: "Kazakhstan",
        code: "KZ",
    },
    Country {
        name: "Kenya",
        code: "KE",
    },
    Country {
        name: "Kiribati",
        code: "KI",
    },
    Country {
        name: "Corée du Sud",
        code: "KR",
    },
    Country {
        name: "Corée du Nord",
        code: "KP",
    },
    Country {
        name: "Koweït",
        code: "KW",
    },
    Country {
        name: "Kirghizistan",
        code: "KG",
    },
    Country {
        name: "Lao",
        code: "LA",
    },
    Country {
        name: "Lettonie",
        code: "LV",
    },
    Country {
        name: "Liban",
        code: "LB",
    },
    Country {
        name: "Lesotho",
        code: "LS",
    },
    Country {
        name: "Libéria",
        code: "LR",
    },
    Country {
        name: "Libye",
        code: "LY",
    },
    Country {
        name: "Liechtenstein",
        code: "LI",
    },
    Country {
        name: "Lituanie",
        code: "LT",
    },
    Country {
        name: "Luxembourg",
        code: "LU",
    },
    Country {
        name: "Macao",
        code: "MO",
    },
    Country {
        name: "Madagascar",
        code: "MG",
    },
    Country {
        name: "Malawi",
        code: "MW",
    },
    Country {
        name: "Malaisie",
        code: "MY",
    },
    Country {
        name: "Maldives",
        code: "MV",
    },
    Country {
        name: "Mali",
        code: "ML",
    },
    Country {
        name: "Malte",
        code: "MT",
    },
    Country {
        name: "Marshall",
        code: "MH",
    },
    Country {
        name: "Martinique",
        code: "MQ",
    },
    Country {
        name: "Mauritanie",
        code: "MR",
    },
    Country {
        name: "Maurice",
        code: "MU",
    },
    Country {
        name: "Mayotte",
        code: "YT",
    },
    Country {
        name: "Mexique",
        code: "MX",
    },
    Country {
        name: "États fédérés de Micronésie",
        code: "FM",
    },
    Country {
        name: "République de Moldova",
        code: "MD",
    },
    Country {
        name: "Monaco",
        code: "MC",
    },
    Country {
        name: "Mongolie",
        code: "MN",
    },
    Country {
        name: "Monténégro",
        code: "ME",
    },
    Country {
        name: "Montserrat",
        code: "MS",
    },
    Country {
        name: "Maroc",
        code: "MA",
    },
    Country {
        name: "Mozambique",
        code: "MZ",
    },
    Country {
        name: "Myanmar",
        code: "MM",
    },
    Country {
        name: "Namibie",
        code: "NA",
    },
    Country {
        name: "Nauru",
        code: "NR",
    },
    Country {
        name: "Népal",
        code: "NP",
    },
    Country {
        name: "Pays-Bas",
        code: "NL",
    },
    Country {
        name: "Nouvelle-Calédonie",
        code: "NC",
    },
    Country {
        name: "Nouvelle-Zélande",
        code: "NZ",
    },
    Country {
        name: "Nicaragua",
        code: "NI",
    },
    Country {
        name: "Niger",
        code: "NE",
    },
    Country {
        name: "Nigéria",
        code: "NG",
    },
    Country {
        name: "Niue",
        code: "NU",
    },
    Country {
        name: "Île de Norfolk",
        code: "NF",
    },
    Country {
        name: "Macédoine du Nord",
        code: "MK",
    },
    Country {
        name: "Îles Mariannes du Nord",
        code: "MP",
    },
    Country {
        name: "Norvège",
        code: "NO",
    },
    Country {
        name: "Oman",
        code: "OM",
    },
    Country {
        name: "Pakistan",
        code: "PK",
    },
    Country {
        name: "Palaos",
        code: "PW",
    },
    Country {
        name: "Palestine",
        code: "PS",
    },
    Country {
        name: "Panama",
        code: "PA",
    },
    Country {
        name: "Papouasie-Nouvelle-Guinée",
        code: "PG",
    },
    Country {
        name: "Paraguay",
        code: "PY",
    },
    Country {
        name: "Pérou",
        code: "PE",
    },
    Country {
        name: "Philippines",
        code: "PH",
    },
    Country {
        name: "Pitcairn",
        code: "PN",
    },
    Country {
        name: "Pologne",
        code: "PL",
    },
    Country {
        name: "Portugal",
        code: "PT",
    },
    Country {
        name: "Porto Rico",
        code: "PR",
    },
    Country {
        name: "Qatar",
        code: "QA",
    },
    Country {
        name: "La Réunion",
        code: "RE",
    },
    Country {
        name: "Roumanie",
        code: "RO",
    },
    Country {
        name: "Russie",
        code: "RU",
    },
    Country {
        name: "Rwanda",
        code: "RW",
    },
    Country {
        name: "Saint-Barthélemy",
        code: "BL",
    },
    Country {
        name: "Sainte-Hélène",
        code: "SH",
    },
    Country {
        name: "Saint-Kitts-et-Nevis",
        code: "KN",
    },
    Country {
        name: "Sainte-Lucie",
        code: "LC",
    },
    Country {
        name: "Saint-Martin (partie française)",
        code: "MF",
    },
    Country {
        name: "Saint-Pierre-et-Miquelon",
        code: "PM",
    },
    Country {
        name: "Saint-Vincent-et-les Grenadines",
        code: "VC",
    },
    Country {
        name: "Samoa",
        code: "WS",
    },
    Country {
        name: "Saint-Marin",
        code: "SM",
    },
    Country {
        name: "Sao Tomé-et-Principe",
        code: "ST",
    },
    Country {
        name: "Arabie saoudite",
        code: "SA",
    },
    Country {
        name: "Sénégal",
        code: "SN",
    },
    Country {
        name: "Serbie",
        code: "RS",
    },
    Country {
        name: "Seychelles",
        code: "SC",
    },
    Country {
        name: "Sierra Leone",
        code: "SL",
    },
    Country {
        name: "Singapour",
        code: "SG",
    },
    Country {
        name: "Saint-Martin (partie néerlandaise)",
        code: "SX",
    },
    Country {
        name: "Slovaquie",
        code: "SK",
    },
    Country {
        name: "Slovénie",
        code: "SI",
    },
    Country {
        name: "Îles Salomon",
        code: "SB",
    },
    Country {
        name: "Somalie",
        code: "SO",
    },
    Country {
        name: "Afrique du Sud",
        code: "ZA",
    },
    Country {
        name: "Géorgie du Sud-et-les Îles Sandwich du Sud",
        code: "GS",
    },
    Country {
        name: "Soudan du Sud",
        code: "SS",
    },
    Country {
        name: "Espagne",
        code: "ES",
    },
    Country {
        name: "Sri Lanka",
        code: "LK",
    },
    Country {
        name: "Soudan",
        code: "SD",
    },
    Country {
        name: "Suriname",
        code: "SR",
    },
    Country {
        name: "Svalbard et l'Île Jan Mayen",
        code: "SJ",
    },
    Country {
        name: "Suède",
        code: "SE",
    },
    Country {
        name: "Suisse",
        code: "CH",
    },
    Country {
        name: "République arabe syrienne",
        code: "SY",
    },
    Country {
        name: "Taïwan",
        code: "TW",
    },
    Country {
        name: "Tadjikistan",
        code: "TJ",
    },
    Country {
        name: "Tanzanie",
        code: "TZ",
    },
    Country {
        name: "Thaïlande",
        code: "TH",
    },
    Country {
        name: "Timor-Leste",
        code: "TL",
    },
    Country {
        name: "Togo",
        code: "TG",
    },
    Country {
        name: "Tokelau",
        code: "TK",
    },
    Country {
        name: "Tonga",
        code: "TO",
    },
    Country {
        name: "Trinité-et-Tobago",
        code: "TT",
    },
    Country {
        name: "Tunisie",
        code: "TN",
    },
    Country {
        name: "Türkiye",
        code: "TR",
    },
    Country {
        name: "Turkménistan",
        code: "TM",
    },
    Country {
        name: "Îles Turks-et-Caïcos",
        code: "TC",
    },
    Country {
        name: "Tuvalu",
        code: "TV",
    },
    Country {
        name: "Ouganda",
        code: "UG",
    },
    Country {
        name: "Ukraine",
        code: "UA",
    },
    Country {
        name: "Émirats arabes unis",
        code: "AE",
    },
    Country {
        name: "Royaume-Uni",
        code: "GB",
    },
    Country {
        name: "Îles mineures éloignées des États-Unis",
        code: "UM",
    },
    Country {
        name: "États-Unis d'Amérique",
        code: "US",
    },
    Country {
        name: "Uruguay",
        code: "UY",
    },
    Country {
        name: "Ouzbékistan",
        code: "UZ",
    },
    Country {
        name: "Vanuatu",
        code: "VU",
    },
    Country {
        name: "Venezuela",
        code: "VE",
    },
    Country {
        name: "Viet Nam",
        code: "VN",
    },
    Country {
        name: "Îles Vierges britanniques",
        code: "VG",
    },
    Country {
        name: "Îles Vierges des États-Unis",
        code: "VI",
    },
    Country {
        name: "Wallis-et-Futuna",
        code: "WF",
    },
    Country {
        name: "Sahara occidental",
        code: "EH",
    },
    Country {
        name: "Yémen",
        code: "YE",
    },
    Country {
        name: "Zambie",
        code: "ZM",
    },
    Country {
        name: "Zimbabwe",
        code: "ZW",
    },
    Country {
        name: "Inconnus",
        code: "-",
    },
];

impl Country<'_> {
    pub fn create<'a>(conn: &mut MysqlConnection, records: [Country<'a>; 250]) {
        use crate::schema::countries::dsl::*;
        let record_inserted = insert_into(countries)
            .values::<Vec<Country<'a>>>(records.to_vec())
            .execute(conn);
        match record_inserted {
            Ok(_) => {}
            Err(err) => {
                panic!("Error during insertion: {:#?}", err)
            }
        }
    }

    pub fn find(conn: &mut MysqlConnection) -> Vec<FullCountry> {
        use crate::schema::countries::dsl::*;
        match countries.select((id, name, code)).load::<FullCountry>(conn) {
            Ok(status_list) => status_list,
            Err(_) => {
                panic!("No country found in database");
            }
        }
    }

    pub fn find_country_in_array(list_of_country: &Vec<FullCountry>, value: &str) -> String {
        let mut country_name = value;

        let country = list_of_country.iter().find(|x| x.name == country_name);

        match country {
            Some(country_info) => String::from(country_info.id.to_string()),
            None => {
                String::from("250".to_string())
            }
        }
    }
}
