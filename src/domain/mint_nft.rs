use crate::domain::entities::{NftId, NftName, NftTypes};

use std::convert::TryFrom;

struct Request {
    id: u16,
    name: String,
    types: Vec<String>,
}

enum Response {
    Ok(u16),
    BadRequest,
}

fn execute(r: Request) -> Response {
    match (
        NftId::try_from(r.id),
        NftName::try_from(r.name),
        NftTypes::try_from(r.types),
    ) {
        (Ok(i), Ok(_), Ok(_)) => Response::Ok(u16::from(i)),
        _ => Response::BadRequest,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_tx_id_after_mint() {
        let id = 24;
        let req = Request {
            id,
            name: String::from("jx"),
            types: vec![String::from("Rip")],
        };

        let res = execute(req);

        match res {
            Response::Ok(res_number) => assert_eq!(res_number, id),
            _ => unreachable!(),
        };
    }

    #[test]
    fn should_return_400_error_when_name_is_empty() {
        let empty_name = String::from("");

        let req = Request {
            id: 24,
            name: empty_name,
            types: vec![String::from("Rip")],
        };
        let res = execute(req);

        match res {
            Response::BadRequest => {}
            _ => unreachable!(),
        };
    }
}
