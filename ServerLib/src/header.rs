use crate::request::RequestType;
use std::time::{SystemTime, UNIX_EPOCH};
#[derive(Debug)]
pub struct Header {
    pub time_created: u128,
    pub content_length: u64,
    pub request_type: RequestType,
}

impl Header
{
    pub fn new(request_type: RequestType, size: u64) -> Self
    {
        Self
        {
            time_created: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("")
                .as_millis() ,
            content_length: size,
            request_type,
        }
    }
    pub fn serialized_size(&self) -> usize
    {

        let base_size = 1 + 8 + 8;
        match &self.request_type
        {
            RequestType::Download(s) | RequestType::Cd(s) => base_size + 4 + s.len(),
            _ => base_size,
        }
    }
}


impl crate::serializable::Serializable for Header
{
    fn serialize(&self) -> Vec<u8>
    {
        let mut bytes = Vec::new();

        bytes.push(self.request_type.to_u8());
        match &self.request_type
        {
            RequestType::Download(ref s) | RequestType::Cd(ref s) =>
                {
                bytes.extend(&(s.len() as u32).to_be_bytes());
                bytes.extend(s.as_bytes());
                }
            _ => {}
        }
     ;

        bytes.extend(&self.time_created.to_be_bytes());
        bytes.extend(&self.content_length.to_be_bytes());

        bytes
    }
    fn deserialize(data: &[u8]) -> Option<Self>
    where
        Self: Sized,
    {
        if data.len() < 16 {
            return None;
        }
        let request_type_value = data[0];
        let (request_type, bytes_read) = RequestType::from_u8(request_type_value, &data[1..])?;
        if data.len() < bytes_read + 16 {
            return None;
        }

        let time_created_start = 1 + bytes_read;
        let time_created_end = time_created_start + 16;
        let time_created =
            u128::from_be_bytes(data[time_created_start..time_created_end].try_into().ok()?);

        let content_length_start = time_created_end;
        let content_length_end = content_length_start + 8;
        let content_length_bytes: [u8; 8] = data[content_length_start..content_length_end]
            .try_into()
            .ok()?;
        let content_length = bytes_to_u64(content_length_bytes);
        Some(Self
        {
            request_type,
            time_created,
            content_length,
        })
    }
}
fn bytes_to_u64(bytes: [u8; 8]) -> u64 {
    u64::from_be_bytes(bytes)
}
