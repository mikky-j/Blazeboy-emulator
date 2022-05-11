#[derive(Debug)]
enum RomError {
    Logo,
    Title,
    LicenseCode,
    CatridgeType,
    HeaderChecksum,
    RomSize,
    RamSize,
}

struct CatridgeInfo {
    nintendo_logo: Vec<u8>,
    title: String,
    license_code: String,
    sgb: bool,
    catridge_type: Vec<CatridgeType>,
    rom_size: usize,
    ram_size: u8,
    japanese: bool,
    version_number: u8,
    header_checksum: bool,
    global_checksum: u16,
}

enum CatridgeType {
    Rom,
    Mbc1,
    Mbc2,
    Mbc3,
    Ram,
    Battery,
    Timer,
    MMM01,
    Mbc5,
    Mbc6,
    Rumble,
    Sensor,
    Camera,
    Tama5,
    HuC3,
    HuC1,
    Mbc7,
}
fn load_rom() -> Result<CatridgeInfo, RomError> {
    let v: Vec<u8> = vec![23];
    let logo = load_logo(&v)?;
    let title = load_title(&v)?;
    let license_code = load_license_code(&v)?;
    let catridge_type = load_catridge_type(&v)?;
    let sgb = v[0x146] == 3;
    let rom_size: usize = 32 << v[0x148];
    let ram_size = get_ram(&v)?;
    let destination_code = v[0x14A] == 1;
    let version_number = v[0x14C];
    let header_checksum = check_header_checksum(&v)?;
    let global_checksum = ((v[0x14e] as u16) << 8) | v[0x14f] as u16;

    let result = CatridgeInfo {
        nintendo_logo: logo,
        title,
        license_code,
        catridge_type,
        sgb,
        rom_size,
        ram_size,
        japanese: destination_code,
        version_number,
        header_checksum,
        global_checksum,
    };

    Ok(result)
}

fn load_title(data: &Vec<u8>) -> Result<String, RomError> {
    let title = &data[0x134..=0x143];
    let title = title.to_ascii_uppercase();
    if title.eq(&[0; 16]) {
        return Err(RomError::Title);
    }
    match String::from_utf8(title) {
        Ok(v) => Ok(v),
        _ => Err(RomError::Title),
    }
}

fn get_ram(data: &Vec<u8>) -> Result<u8, RomError> {
    let result = match data[0x149] {
        0x2 => 8,
        0x3 => 32,
        0x4 => 128,
        0x5 => 64,
        _ => 0,
    };

    if result == 0 {
        Err(RomError::RamSize)
    } else {
        Ok(result)
    }
}
fn check_header_checksum(data: &Vec<u8>) -> Result<bool, RomError> {
    let mut x: i32 = 0;
    let mut i = 0x134;
    while i <= 0x14c {
        x = x - data[i] as i32 - 1;
        i += 1;
    }
    if x & 0xFF > 0 {
        Ok(true)
    } else {
        Err(RomError::HeaderChecksum)
    }
}
fn load_catridge_type(data: &Vec<u8>) -> Result<Vec<CatridgeType>, RomError> {
    let catridge = data[0x147];
    let result = match catridge {
        0x00 => vec![CatridgeType::Rom],
        0x01 => vec![CatridgeType::Mbc1],
        0x02 => vec![CatridgeType::Mbc1, CatridgeType::Ram],
        0x03 => vec![CatridgeType::Mbc1, CatridgeType::Ram, CatridgeType::Battery],
        0x05 => vec![CatridgeType::Mbc2],
        0x06 => vec![CatridgeType::Mbc2, CatridgeType::Battery],
        0x08 => vec![CatridgeType::Rom, CatridgeType::Ram],
        0x09 => vec![CatridgeType::Rom, CatridgeType::Ram, CatridgeType::Battery],
        0x0B => vec![CatridgeType::MMM01],
        0x0C => vec![CatridgeType::MMM01, CatridgeType::Ram],
        0x0D => vec![
            CatridgeType::MMM01,
            CatridgeType::Ram,
            CatridgeType::Battery,
        ],
        0x0F => vec![
            CatridgeType::Mbc3,
            CatridgeType::Timer,
            CatridgeType::Battery,
        ],
        0x10 => vec![
            CatridgeType::Mbc3,
            CatridgeType::Timer,
            CatridgeType::Ram,
            CatridgeType::Battery,
        ],
        0x11 => vec![CatridgeType::Mbc3],
        0x12 => vec![CatridgeType::Mbc3, CatridgeType::Ram],
        0x13 => vec![CatridgeType::Mbc3, CatridgeType::Ram, CatridgeType::Battery],
        0x19 => vec![CatridgeType::Mbc5],
        0x1a => vec![CatridgeType::Mbc5, CatridgeType::Ram],
        0x1b => vec![CatridgeType::Mbc5, CatridgeType::Ram, CatridgeType::Battery],
        0x1c => vec![CatridgeType::Mbc5, CatridgeType::Rumble],
        0x1d => vec![CatridgeType::Mbc5, CatridgeType::Rumble, CatridgeType::Ram],
        0x1e => vec![
            CatridgeType::Mbc5,
            CatridgeType::Rumble,
            CatridgeType::Ram,
            CatridgeType::Battery,
        ],
        0x20 => vec![CatridgeType::Mbc6],
        0x22 => vec![
            CatridgeType::Mbc7,
            CatridgeType::Rumble,
            CatridgeType::Ram,
            CatridgeType::Battery,
            CatridgeType::Sensor,
        ],
        0xfc => vec![CatridgeType::Camera],
        0xfd => vec![CatridgeType::Tama5],
        0xfe => vec![CatridgeType::HuC3],
        0xff => vec![CatridgeType::HuC1, CatridgeType::Ram, CatridgeType::Battery],
        _ => vec![],
    };
    if result.is_empty() {
        return Err(RomError::CatridgeType);
    }
    Ok(result)
}

fn load_license_code(data: &Vec<u8>) -> Result<String, RomError> {
    if data[0x014B] == 0x33 {
        // let code = &data[0x144..=0x145];
        let code = data[0x144] as u16;
        let manufacturer = match code {
            0x01 => "Nintendo R&D1",
            0x08 => "Capcom",
            0x13 => "Electronic Arts",
            0x18 => "Hudson Soft",
            0x19 => "b-ai",
            0x20 => "kss",
            0x22 => "pow",
            0x24 => "PCM Complete",
            0x25 => "san-x",
            0x28 => "Kemco Japan",
            0x29 => "seta",
            0x30 => "Viacom",
            0x31 => "Nintendo",
            0x32 => "Bandai",
            0x33 => "Ocean/Acclaim",
            0x34 => "Konami",
            0x35 => "Hector",
            0x37 => "Taito",
            0x38 => "Hudson",
            0x39 => "Banpresto",
            0x41 => "Ubi Soft",
            0x42 => "Atlus",
            0x44 => "Malibu",
            0x46 => "angel",
            0x47 => "Bullet-Proof",
            0x49 => "irem",
            0x50 => "Absolute",
            0x51 => "Acclaim",
            0x52 => "Activision",
            0x53 => "American sammy",
            0x54 => "Konami",
            0x55 => "Hi tech entertainment",
            0x56 => "LJN",
            0x57 => "Matchbox",
            0x58 => "Mattel",
            0x59 => "Milton Bradley",
            0x60 => "Titus",
            0x61 => "Virgin",
            0x64 => "LucasArts",
            0x67 => "Ocean",
            0x69 => "Electronic Arts",
            0x70 => "Infogrames",
            0x71 => "Interplay",
            0x72 => "Broderbund",
            0x73 => "sculptured",
            0x75 => "sci",
            0x78 => "THQ",
            0x79 => "Accolade",
            0x80 => "misawa",
            0x83 => "lozc",
            0x86 => "Tokuma Shoten Intermedia",
            0x87 => "Tsukuda Original",
            0x91 => "Chunsoft",
            0x92 => "Video system",
            0x93 => "Ocean/Acclaim",
            0x95 => "Varie",
            0x96 => "Yonezawa/s`pal",
            0x97 => "Kaneko",
            0x99 => "Pack in soft",
            0xAF => "Konami (Yu-Gi-Oh!)",
            _ => {
                return Err(RomError::LicenseCode);
            }
        };
        return Ok(manufacturer.to_string());
    } else {
        todo!("Yet to implement the old manufacturer stuff");
    }
}

fn load_logo(data: &Vec<u8>) -> Result<Vec<u8>, RomError> {
    let logo = &data[0x104..=0x134];
    let logo = logo.to_vec();
    let arr: [u8; 48] = [
        0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00,
        0x0D, 0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD,
        0xD9, 0x99, 0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC, 0x99, 0x9F, 0xBB,
        0xB9, 0x33, 0x3E,
    ];
    if !logo.eq(&arr) && logo.len() == 48 {
        return Err(RomError::Logo);
    }
    Ok(logo)
}
