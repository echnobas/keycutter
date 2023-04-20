const CRC_TABLE: &[u32] = &[
    0x0, 0x4c11db7, 0x9823b6e, 0xd4326d9, 0x130476dc, 0x17c56b6b, 0x1a864db2, 0x1e475005,
    0x2608edb8, 0x22c9f00f, 0x2f8ad6d6, 0x2b4bcb61, 0x350c9b64, 0x31cd86d3, 0x3c8ea00a, 0x384fbdbd,
    0x4c11db70, 0x48d0c6c7, 0x4593e01e, 0x4152fda9, 0x5f15adac, 0x5bd4b01b, 0x569796c2, 0x52568b75,
    0x6a1936c8, 0x6ed82b7f, 0x639b0da6, 0x675a1011, 0x791d4014, 0x7ddc5da3, 0x709f7b7a, 0x745e66cd,
    0x9823b6e0, 0x9ce2ab57, 0x91a18d8e, 0x95609039, 0x8b27c03c, 0x8fe6dd8b, 0x82a5fb52, 0x8664e6e5,
    0xbe2b5b58, 0xbaea46ef, 0xb7a96036, 0xb3687d81, 0xad2f2d84, 0xa9ee3033, 0xa4ad16ea, 0xa06c0b5d,
    0xd4326d90, 0xd0f37027, 0xddb056fe, 0xd9714b49, 0xc7361b4c, 0xc3f706fb, 0xceb42022, 0xca753d95,
    0xf23a8028, 0xf6fb9d9f, 0xfbb8bb46, 0xff79a6f1, 0xe13ef6f4, 0xe5ffeb43, 0xe8bccd9a, 0xec7dd02d,
    0x34867077, 0x30476dc0, 0x3d044b19, 0x39c556ae, 0x278206ab, 0x23431b1c, 0x2e003dc5, 0x2ac12072,
    0x128e9dcf, 0x164f8078, 0x1b0ca6a1, 0x1fcdbb16, 0x18aeb13, 0x54bf6a4, 0x808d07d, 0xcc9cdca,
    0x7897ab07, 0x7c56b6b0, 0x71159069, 0x75d48dde, 0x6b93dddb, 0x6f52c06c, 0x6211e6b5, 0x66d0fb02,
    0x5e9f46bf, 0x5a5e5b08, 0x571d7dd1, 0x53dc6066, 0x4d9b3063, 0x495a2dd4, 0x44190b0d, 0x40d816ba,
    0xaca5c697, 0xa864db20, 0xa527fdf9, 0xa1e6e04e, 0xbfa1b04b, 0xbb60adfc, 0xb6238b25, 0xb2e29692,
    0x8aad2b2f, 0x8e6c3698, 0x832f1041, 0x87ee0df6, 0x99a95df3, 0x9d684044, 0x902b669d, 0x94ea7b2a,
    0xe0b41de7, 0xe4750050, 0xe9362689, 0xedf73b3e, 0xf3b06b3b, 0xf771768c, 0xfa325055, 0xfef34de2,
    0xc6bcf05f, 0xc27dede8, 0xcf3ecb31, 0xcbffd686, 0xd5b88683, 0xd1799b34, 0xdc3abded, 0xd8fba05a,
    0x690ce0ee, 0x6dcdfd59, 0x608edb80, 0x644fc637, 0x7a089632, 0x7ec98b85, 0x738aad5c, 0x774bb0eb,
    0x4f040d56, 0x4bc510e1, 0x46863638, 0x42472b8f, 0x5c007b8a, 0x58c1663d, 0x558240e4, 0x51435d53,
    0x251d3b9e, 0x21dc2629, 0x2c9f00f0, 0x285e1d47, 0x36194d42, 0x32d850f5, 0x3f9b762c, 0x3b5a6b9b,
    0x315d626, 0x7d4cb91, 0xa97ed48, 0xe56f0ff, 0x1011a0fa, 0x14d0bd4d, 0x19939b94, 0x1d528623,
    0xf12f560e, 0xf5ee4bb9, 0xf8ad6d60, 0xfc6c70d7, 0xe22b20d2, 0xe6ea3d65, 0xeba91bbc, 0xef68060b,
    0xd727bbb6, 0xd3e6a601, 0xdea580d8, 0xda649d6f, 0xc423cd6a, 0xc0e2d0dd, 0xcda1f604, 0xc960ebb3,
    0xbd3e8d7e, 0xb9ff90c9, 0xb4bcb610, 0xb07daba7, 0xae3afba2, 0xaafbe615, 0xa7b8c0cc, 0xa379dd7b,
    0x9b3660c6, 0x9ff77d71, 0x92b45ba8, 0x9675461f, 0x8832161a, 0x8cf30bad, 0x81b02d74, 0x857130c3,
    0x5d8a9099, 0x594b8d2e, 0x5408abf7, 0x50c9b640, 0x4e8ee645, 0x4a4ffbf2, 0x470cdd2b, 0x43cdc09c,
    0x7b827d21, 0x7f436096, 0x7200464f, 0x76c15bf8, 0x68860bfd, 0x6c47164a, 0x61043093, 0x65c52d24,
    0x119b4be9, 0x155a565e, 0x18197087, 0x1cd86d30, 0x29f3d35, 0x65e2082, 0xb1d065b, 0xfdc1bec,
    0x3793a651, 0x3352bbe6, 0x3e119d3f, 0x3ad08088, 0x2497d08d, 0x2056cd3a, 0x2d15ebe3, 0x29d4f654,
    0xc5a92679, 0xc1683bce, 0xcc2b1d17, 0xc8ea00a0, 0xd6ad50a5, 0xd26c4d12, 0xdf2f6bcb, 0xdbee767c,
    0xe3a1cbc1, 0xe760d676, 0xea23f0af, 0xeee2ed18, 0xf0a5bd1d, 0xf464a0aa, 0xf9278673, 0xfde69bc4,
    0x89b8fd09, 0x8d79e0be, 0x803ac667, 0x84fbdbd0, 0x9abc8bd5, 0x9e7d9662, 0x933eb0bb, 0x97ffad0c,
    0xafb010b1, 0xab710d06, 0xa6322bdf, 0xa2f33668, 0xbcb4666d, 0xb8757bda, 0xb5365d03, 0xb1f740b4,
];

// fn crc_table() -> Vec<u32> {
//     let mut tab = Vec::with_capacity(256);
//     for i in 0..256 {
//         let mut k = i << 24;
//         for _bit in 0..8 {
//             k = if k & 0x80000000 != 0 {
//                 (k << 1) ^ 0x4c11db7
//             } else {
//                 k << 1
//             };
//         }
//         tab.push(k);
//     }
//     tab
// }

macro_rules! input {
    ($($arg:tt)*) => {{
        use ::std::io::Write;
        (|| -> std::io::Result<::std::string::String> {
            ::std::print!("{}", ::std::fmt::format(std::format_args!($($arg)*)));
            ::std::io::stdout().flush()?;
            let mut res = ::std::string::String::new();
            ::std::io::stdin().read_line(&mut res)?;
            Ok(res.trim().to_string())
        })()
    }}
}

fn main() {
    loop {
        match input!("A - Encode\nB - Decode\n> ") {
            Ok(choice) if choice == "A" => {
                fn read_field(field: &'static str, default: Option<u128>) -> Option<u128> {
                    input!(
                        "{field} ({}) :: ",
                        match default {
                            Some(d) => format!("default={d}"),
                            None => "default=auto".to_owned(),
                        }
                    )
                    .ok()
                    .and_then(|s| u128::from_str_radix(s.trim_start_matches("0x"), 16).ok())
                    .or(default)
                }
                fn read_bool(field: &'static str, default: Option<bool>) -> Option<bool> {
                    input!(
                        "{field} ({}) :: ",
                        match default {
                            Some(d) => format!("default={d}"),
                            None => "default=auto".to_owned(),
                        }
                    )
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .or(default)
                }

                let group = read_field("group", Some(0)).unwrap();
                let serial = read_field("serial", Some(0)).unwrap();
                let security = read_field("security", Some(0)).unwrap();
                let checksum = read_field("checksum", None);
                let upgrade = read_bool("upgrade", Some(false)).unwrap();
                let extra = read_bool("extra", Some(false)).unwrap();

                match ProductKeyEncoder::new(group, serial, security, upgrade, checksum, extra) {
                    Some(key) => {
                        println!("{:#x?}", key);
                    }
                    _ => eprintln!("ERROR :: Invalid key data"),
                }
            }
            Ok(choice) if choice == "B" => {
                let product_key = input!("Enter product key\n> ").unwrap();
                match ProductKeyDecoder::new(&product_key) {
                    Some(key) => {
                        println!("{:#x?}", key);
                    }
                    _ => eprintln!("ERROR :: Invalid product key"),
                }
            }
            _ => eprintln!("ERROR :: Invalid option"),
        }
        println!()
    }
}

const BASE24: &[u8] = b"BCDFGHJKMPQRTVWXY2346789";

#[derive(Debug, PartialEq, Eq)]
pub struct ProductKeyDecoder<'a> {
    product_key: &'a str,
    raw_key: u128,
    group: u128,
    serial: u128,
    security: u128,
    checksum: u128,
    upgrade: u128,
    extra: u128,
}

impl<'a> ProductKeyDecoder<'a> {
    const PREDICATES: &[fn(&str) -> bool] = &[
        |key| key.len() == 29,
        |key| key.chars().filter(|&c| c == '-').count() == 4,
    ];

    pub fn new(product_key: &'a str) -> Option<Self> {
        if Self::PREDICATES.iter().all(|p| p(product_key)) {
            Self::decode_5x5(product_key).map(|decoded| Self {
                product_key,
                raw_key: decoded,
                group: decoded & 0x000000000000000000000000000fffff,
                serial: (decoded & 0x00000000000000000003fffffff00000) >> 20,
                security: (decoded & 0x0000007ffffffffffffc000000000000) >> 50,
                checksum: (decoded & 0x0001ff80000000000000000000000000) >> 103,
                upgrade: (decoded & 0x00020000000000000000000000000000) >> 113,
                extra: (decoded & 0x00040000000000000000000000000000) >> 114,
            })
        } else {
            None
        }
    }

    pub fn decode_5x5(key: &str) -> Option<u128> {
        if Self::PREDICATES.iter().all(|p| p(key)) {
            Some(
                [key.chars().filter(|&c| c != '-').position(|c| c == 'N')? as u128]
                    .into_iter()
                    .chain(key.chars().filter_map(|c| {
                        if c != 'N' && c != '-' {
                            Some(BASE24.iter().position(|&b| b as char == c)? as u128)
                        } else {
                            None
                        }
                    }))
                    .fold(0, |acc, x| (acc * 24) + x),
            )
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ProductKeyEncoder {
    product_key: String,
    raw_key: u128,
    group: u128,
    serial: u128,
    security: u128,
    checksum: u128,
    upgrade: u128,
    extra: u128,
}

impl ProductKeyEncoder {
    const BOUNDS: [u128; 6] = [0xfffff, 0x3fffffff, 0x1fffffffffffff, 0x3ff, 0x1, 0x1];

    pub fn new(
        group: u128,
        serial: u128,
        security: u128,
        upgrade: bool,
        checksum: Option<u128>,
        extra: bool,
    ) -> Option<Self> {
        let upgrade = if upgrade { 1 } else { 0 };
        let extra = if extra { 1 } else { 0 };
        if Self::BOUNDS
            .iter()
            .zip([
                group,
                serial,
                security,
                checksum.unwrap_or(0),
                upgrade,
                extra,
            ])
            .all(|(&bound, value)| value <= bound)
        {
            let mut raw_key = 0;
            raw_key |= extra << 114;
            raw_key |= upgrade << 113;
            raw_key |= security << 50;
            raw_key |= serial << 20;
            raw_key |= group;

            let checksum = checksum.unwrap_or_else(|| {
                let mut crc = 0xffffffffu32;
                for byte in raw_key.to_le_bytes() {
                    crc = (crc << 8) ^ CRC_TABLE[(((crc >> 24) ^ byte as u32) & 0xff) as usize]
                }
                (!crc & 0x3ff) as u128
            });

            raw_key |= checksum << 103;

            if extra != 0 && raw_key > (0x62A32B15518 << 72) {
                return None;
            }
            Some(Self {
                product_key: Self::to_5x5(raw_key),
                raw_key,
                group,
                serial,
                security,
                checksum,
                upgrade,
                extra,
            })
        } else {
            None
        }
    }

    pub fn to_5x5(mut raw_key: u128) -> String {
        let mut key = (0..25)
            .map(|_| {
                let v = raw_key % 24;
                raw_key /= 24;
                v
            })
            .collect::<Vec<_>>();
        key.reverse();
        let mut product_key = key.iter().skip(1).fold(String::with_capacity(29), |mut s, &v| {
            s.push(BASE24[v as usize] as char);
            s
        });
        product_key.insert(key[0] as usize, 'N');
        for i in 0..4 { product_key.insert((i + 1) * 5 + i, '-'); }
        product_key
    }
}

impl PartialEq<ProductKeyEncoder> for ProductKeyDecoder<'_> {
    fn eq(&self, other: &ProductKeyEncoder) -> bool {
        self.product_key == other.product_key
            && self.raw_key == other.raw_key
            && self.group == other.group
            && self.serial == other.serial
            && self.security == other.security
            && self.checksum == other.checksum
            && self.upgrade == other.upgrade
            && self.extra == other.extra
    }
}

#[test]
fn it_works() {
    assert!(
        ProductKeyDecoder::new("NPPR9-FWDCX-D2C8J-H872K-2YT43").unwrap()
            == ProductKeyEncoder::new(3290, 3, 4390648297412806, false, Some(51), false).unwrap()
    );
}
