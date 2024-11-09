use crate::header::Header;
use crate::serializable;
use crate::serializable::Serializable;

pub struct Resposne
{
    pub header: Header,
    pub body: Vec<u8>,
}
impl Resposne {
    pub fn new(header: Header) -> Self
    {
        Resposne
        {
            header,
            body: Vec::new(),
        }
    }
}
impl Serializable for Resposne
{
    fn serialize(&self) -> Vec<u8>
    {
        let mut bytes = Vec::new();
        bytes.extend(self.header.serialize());
        bytes.extend(self.body.iter());
        bytes
    }

    fn deserialize(data: &[u8]) -> Option<Self>
    {
        let header = Header::deserialize(data)?;
        let body = data[header.serialized_size()..].to_vec();
        Some(Resposne { header, body })
    }
}
