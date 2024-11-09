use crate::header::Header;
use crate::serializable;
#[derive(Debug,PartialEq)]
pub enum RequestType
{
    Download(String),
    Ping,
    Pwd,
    Cd(String),
    Wrong
}
impl RequestType
{
    pub fn to_u8(&self) -> u8
    {
        return match self
        {
            RequestType::Download(str) => 1,
            RequestType::Ping => 2,
            RequestType::Pwd => 3,
            RequestType::Wrong => 5,
            RequestType::Cd(str) => 4,
        };
    }
    pub fn from_u8(value: u8, data: &[u8]) -> Option<(Self, usize)>
    {
        return match value
        {
            1 =>
                {
                let (string, bytes_read) = deserialize_string(data).unwrap();
                Some((Self::Download(string), bytes_read))
                }
            2 => Some((Self::Ping, 0)),
            3 => Some((Self::Pwd, 0)),
            4 =>
                {
                let (string, bytes_read) = deserialize_string(data).unwrap();

                Some((Self::Cd(string), bytes_read))
            }
            5 => Some((Self::Wrong, 0)),
            _ => None,
        };
    }
}
#[derive(Debug)]
pub struct Request
{
    pub header: Header,
}
impl Request
{
    pub fn new(header: Header)->Self
    {
        Self
        {
            header
        }
    }
}
impl crate::serializable::Serializable for Request
{
    fn serialize(&self) -> Vec<u8>
    {
        let mut bytes = Vec::new();

        bytes.extend(self.header.serialize());

        bytes
    }
    fn deserialize(data: &[u8]) -> Option<Self>
    where
        Self: Sized,
    {
        if data.len() < 16 {
            return None;
        }

        match Header::deserialize(&data[0..])
        {
            Some(header) => Some(Self { header }),
            None => None,
        }
    }
}
fn deserialize_string(data: &[u8]) -> Option<(String, usize)>
{
    if data.len() < 4
    {
        return None;
    }

    let len = u32::from_be_bytes(data[0..4].try_into().ok()?) as usize;

    if data.len() < 4 + len
    {
        return None;
    }

    let string = String::from_utf8(data[4..4 + len].to_vec()).ok()?;

    Some((string, 4 + len))
}
