use byteorder::{LittleEndian, ReadBytesExt};
use std::collections::HashMap;
use std::io::BufRead;
use std::io::{self, Cursor, Read};

#[derive(Debug)]
pub struct BMFont {
    pub info: Option<InfoBlock>,
    pub common: Option<CommonBlock>,
    pub pages: Vec<String>,
    pub chars: HashMap<u32, Char>,
    pub kernings: Vec<KerningPair>,
}

#[derive(Debug)]
pub struct InfoBlock {
    pub font_size: i16,
    pub bit_field: u8,
    pub char_set: u8,
    pub stretch_h: u16,
    pub aa: u8,
    pub padding: [u8; 4],
    pub spacing: [u8; 2],
    pub outline: u8,
    pub font_name: String,
}

#[derive(Debug)]
pub struct CommonBlock {
    pub line_height: u16,
    pub base: u16,
    pub scale_w: u16,
    pub scale_h: u16,
    pub pages: u16,
    pub bit_field: u8,
    pub alpha_chnl: u8,
    pub red_chnl: u8,
    pub green_chnl: u8,
    pub blue_chnl: u8,
}

#[derive(Debug)]
pub struct Char {
    pub id: u32,
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
    pub x_offset: i16,
    pub y_offset: i16,
    pub x_advance: i16,
    pub page: u8,
    pub chnl: u8,
}

#[derive(Debug)]
pub struct KerningPair {
    pub first: u32,
    pub second: u32,
    pub amount: i16,
}

impl BMFont {
    pub fn from_octets(data: &[u8]) -> io::Result<Self> {
        let mut cursor = Cursor::new(data);

        if cursor.read_u8()? != 66
            || cursor.read_u8()? != 77
            || cursor.read_u8()? != 70
            || cursor.read_u8()? != 3
        {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid BMFont header",
            ));
        }

        let mut info = None;
        let mut common = None;
        let mut pages = Vec::new();
        let mut chars = HashMap::new();
        let mut kernings = Vec::new();

        while let Ok(block_type) = cursor.read_u8() {
            let block_size = cursor.read_u32::<LittleEndian>()? as usize;
            let mut block_data = vec![0; block_size];
            cursor.read_exact(&mut block_data)?;

            match block_type {
                1 => info = Some(Self::parse_info_block(&block_data)?),
                2 => common = Some(Self::parse_common_block(&block_data)?),
                3 => pages = Self::parse_pages_block(&block_data)?,
                4 => chars = Self::parse_chars_block(&block_data)?,
                5 => kernings = Self::parse_kerning_block(&block_data)?,
                _ => (),
            }
        }

        Ok(Self {
            info,
            common,
            pages,
            chars,
            kernings,
        })
    }

    fn parse_info_block(data: &[u8]) -> io::Result<InfoBlock> {
        let mut cursor = Cursor::new(data);
        Ok(InfoBlock {
            font_size: cursor.read_i16::<LittleEndian>()?,
            bit_field: cursor.read_u8()?,
            char_set: cursor.read_u8()?,
            stretch_h: cursor.read_u16::<LittleEndian>()?,
            aa: cursor.read_u8()?,
            padding: [
                cursor.read_u8()?,
                cursor.read_u8()?,
                cursor.read_u8()?,
                cursor.read_u8()?,
            ],
            spacing: [cursor.read_u8()?, cursor.read_u8()?],
            outline: cursor.read_u8()?,
            font_name: {
                let mut font_name = Vec::new();
                cursor.read_to_end(&mut font_name)?;
                String::from_utf8(font_name)
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?
                    .trim_end_matches('\0')
                    .to_string()
            },
        })
    }

    fn parse_common_block(data: &[u8]) -> io::Result<CommonBlock> {
        let mut cursor = Cursor::new(data);
        Ok(CommonBlock {
            line_height: cursor.read_u16::<LittleEndian>()?,
            base: cursor.read_u16::<LittleEndian>()?,
            scale_w: cursor.read_u16::<LittleEndian>()?,
            scale_h: cursor.read_u16::<LittleEndian>()?,
            pages: cursor.read_u16::<LittleEndian>()?,
            bit_field: cursor.read_u8()?,
            alpha_chnl: cursor.read_u8()?,
            red_chnl: cursor.read_u8()?,
            green_chnl: cursor.read_u8()?,
            blue_chnl: cursor.read_u8()?,
        })
    }

    fn parse_pages_block(data: &[u8]) -> io::Result<Vec<String>> {
        let mut cursor = Cursor::new(data);
        let mut pages = Vec::new();
        while cursor.position() < data.len() as u64 {
            let mut page_name = Vec::new();
            cursor.read_until(0, &mut page_name)?;
            pages.push(
                String::from_utf8(page_name)
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?
                    .trim_end_matches('\0')
                    .to_string(),
            );
        }
        Ok(pages)
    }

    fn parse_chars_block(data: &[u8]) -> io::Result<HashMap<u32, Char>> {
        let mut cursor = Cursor::new(data);
        let mut chars = HashMap::new();
        while cursor.position() < data.len() as u64 {
            let ch = Char {
                id: cursor.read_u32::<LittleEndian>()?,
                x: cursor.read_u16::<LittleEndian>()?,
                y: cursor.read_u16::<LittleEndian>()?,
                width: cursor.read_u16::<LittleEndian>()?,
                height: cursor.read_u16::<LittleEndian>()?,
                x_offset: cursor.read_i16::<LittleEndian>()?,
                y_offset: cursor.read_i16::<LittleEndian>()?,
                x_advance: cursor.read_i16::<LittleEndian>()?,
                page: cursor.read_u8()?,
                chnl: cursor.read_u8()?,
            };
            chars.insert(ch.id, ch);
        }
        Ok(chars)
    }

    fn parse_kerning_block(data: &[u8]) -> io::Result<Vec<KerningPair>> {
        let mut cursor = Cursor::new(data);
        let mut kernings = Vec::new();
        while cursor.position() < data.len() as u64 {
            kernings.push(KerningPair {
                first: cursor.read_u32::<LittleEndian>()?,
                second: cursor.read_u32::<LittleEndian>()?,
                amount: cursor.read_i16::<LittleEndian>()?,
            });
        }
        Ok(kernings)
    }
}
