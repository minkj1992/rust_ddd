use std::convert::TryFrom;

pub struct NftId(u16);

impl TryFrom<u16> for NftId {
    type Error = ();

    fn try_from(i: u16) -> Result<Self, Self::Error> {
        if i > 0 && i < 899 {
            Ok(Self(i))
        } else {
            Err(())
        }
    }
}

impl From<NftId> for u16 {
    fn from(i: NftId) -> u16 {
        i.0
    }
}

pub struct NftName(String);

impl TryFrom<String> for NftName {
    type Error = ();

    fn try_from(n: String) -> Result<Self, Self::Error> {
        if n.is_empty() {
            Err(())
        } else {
            Ok(Self(n))
        }
    }
}

pub struct NftTypes(Vec<NftType>);

impl TryFrom<Vec<String>> for NftTypes {
    type Error = ();

    fn try_from(ts: Vec<String>) -> Result<Self, Self::Error> {
        if ts.is_empty() {
            Err(())
        } else {
            let mut pts = vec![];
            for t in ts.iter() {
                match NftType::try_from(String::from(t)) {
                    Ok(pt) => pts.push(pt),
                    _ => return Err(()),
                }
            }
            Ok(Self(pts))
        }
    }
}

enum NftType {
    Rip,
}

impl TryFrom<String> for NftType {
    type Error = ();

    fn try_from(t: String) -> Result<Self, Self::Error> {
        match t.as_str() {
            "Rip" => Ok(Self::Rip),
            _ => Err(()),
        }
    }
}
