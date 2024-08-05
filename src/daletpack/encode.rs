use crate::daletl::{DlArgument, DlBody, DlTag, DlTid, IsNull, Page};

use super::{utils, DaletPackError, TypeId};

pub fn encode(page: &Page) -> Result<Vec<u8>, DaletPackError> {
    Ok(utils::compress_zstd(&encode_no_compress(page)?)
        .map_err(|_| DaletPackError::ZstdCompressError)?)
}

pub fn encode_no_compress(page: &Page) -> Result<Vec<u8>, DaletPackError> {
    if page.len() > 2usize.pow(32) {
        return Err(DaletPackError::PageMaxSizeExceeded);
    }

    let mut bv: Vec<u8> = Vec::new();

    for tag in page {
        write_tag(&mut bv, tag)?;
    }

    Ok(bv)
}

fn write_int(bv: &mut Vec<u8>, n: u8) {
    bv.push(1);
    bv.push(n);
}

fn write_str(bv: &mut Vec<u8>, string: &String) -> Result<(), DaletPackError> {
    let size = string.len();

    if size > 2usize.pow(32) {
        return Err(DaletPackError::StrMaxSizeExceeded);
    }

    if size <= 256 {
        bv.push(TypeId::Str8 as u8);
        bv.push((size - 1) as u8);
    } else if size <= 65536 {
        bv.push(TypeId::Str16 as u8);
        bv.extend(((size - 1) as u16).to_be_bytes());
    } else {
        bv.push(TypeId::Str32 as u8);
        bv.extend(((size - 1) as u32).to_be_bytes());
    }

    bv.extend_from_slice(string.as_bytes());

    Ok(())
}

fn write_array(bv: &mut Vec<u8>, arr: &Vec<DlTag>) -> Result<(), DaletPackError> {
    if arr.len() > 2usize.pow(32) {
        return Err(DaletPackError::ArrMaxSizeExceeded);
    }

    bv.push(TypeId::TagArray as u8);

    for tag in arr {
        write_tag(bv, tag)?;
    }

    bv.push(TypeId::TagArrayEnd as u8);

    Ok(())
}

fn write_tag(bv: &mut Vec<u8>, tag: &DlTag) -> Result<(), DaletPackError> {
    if tag.id == DlTid::El {
        write_tag_body(bv, &tag.body)?;
    } else if tag.body.is_null() && tag.argument.is_null() {
        bv.push(TypeId::TagId as u8);
        bv.push(tag.id as u8);
    } else if tag.argument.is_null() {
        bv.push(TypeId::TagIdBody as u8);
        bv.push(tag.id as u8);
        write_tag_body(bv, &tag.body)?;
    } else if tag.body.is_null() {
        bv.push(TypeId::TagIdArgument as u8);
        bv.push(tag.id as u8);
        write_tag_argument(bv, &tag.argument)?;
    } else {
        bv.push(TypeId::TagIdBodyArgument as u8);
        bv.push(tag.id as u8);
        write_tag_body(bv, &tag.body)?;
        write_tag_argument(bv, &tag.argument)?;
    }

    Ok(())
}

fn write_tag_body(bv: &mut Vec<u8>, body: &DlBody) -> Result<(), DaletPackError> {
    match body {
        DlBody::Text(s) => write_str(bv, s)?,
        DlBody::Tags(tags) => write_array(bv, tags)?,
        DlBody::Null => Err(DaletPackError::WriteNullBody)?,
    };

    Ok(())
}

fn write_tag_argument(bv: &mut Vec<u8>, argument: &DlArgument) -> Result<(), DaletPackError> {
    match argument {
        DlArgument::Text(s) => write_str(bv, s)?,
        DlArgument::Number(n) => write_int(bv, *n),
        DlArgument::Null => Err(DaletPackError::WriteNullArgument)?,
    };

    Ok(())
}
