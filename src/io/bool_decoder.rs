/**********************************************************\
|                                                          |
|                          hprose                          |
|                                                          |
| Official WebSite: http://www.hprose.com/                 |
|                   http://www.hprose.org/                 |
|                                                          |
\**********************************************************/
/**********************************************************\
 *                                                        *
 * io/bool_decoder.rs                                     *
 *                                                        *
 * hprose bool decoder for Rust.                          *
 *                                                        *
 * LastModified: Sep 13, 2016                             *
 * Author: Chen Fei <cf@hprose.com>                       *
 *                                                        *
\**********************************************************/

use super::*;
use super::tags::*;
use io::reader::ByteReader;

pub fn bool_decoder(r: &mut Reader, tag: u8) -> Result<bool, DecoderError> {
    match tag {
        b'0' | TAG_NULL | TAG_EMPTY | TAG_FALSE => Ok(false),
        b'1' ... b'9' | TAG_TRUE | TAG_NAN => Ok(true),
        TAG_INTEGER | TAG_LONG | TAG_DOUBLE => read_number_as_bool(r),
        _ => Err(DecoderError::CastError(tag, "bool".to_string()))
    }
}

fn read_number_as_bool(r: &mut Reader) -> Result<bool, DecoderError> {
    r
        .read_until(TAG_SEMICOLON)
        .map_err(|e| DecoderError::ParserError(e))
        .and_then(|bytes| Ok(if bytes.len() == 1 { bytes[0] != b'0' } else { true }))
}