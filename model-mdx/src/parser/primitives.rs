use super::Parser;
use nom::{bytes::complete::take, multi::count};

/// Consume 4 bytes and interpret them as LE float 32 bit
pub fn le_f32(input: &[u8]) -> Parser<f32> {
    let mut bytes: [u8; 4] = Default::default();
    let (input, bytes_vec) = take(4usize)(input)?;
    assert_eq!(bytes_vec.len(), 4);
    bytes.copy_from_slice(&bytes_vec);
    Ok((input, f32::from_le_bytes(bytes)))
}

/// Combinator that applies N times primitive parser and returns result
/// in fixed size array.
pub fn times<const N: usize, T: Copy + Default, P>(parser: P) -> impl Fn(&[u8]) -> Parser<[T; N]>
where
    P: Fn(&[u8]) -> Parser<T> + Copy,
{
    move |input| {
        let mut res: [T; N] = [Default::default(); N];
        let (input, res_vec) = count(parser, N)(input)?;
        assert_eq!(res_vec.len(), N);
        res.copy_from_slice(&res_vec);
        Ok((input, res))
    }
}