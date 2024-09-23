use crate::repositories::coin::Repository;

use super::entities::{Coin, CoinID, CoinName};

pub struct Request {
    coins: Vec<(u16, String)>,
}

pub enum Response {
    Ok,
    BadRequest,
    Error,
}

pub fn execute(repo: &mut dyn Repository, req: Request) -> Response {
    match req
        .coins
        .iter()
        .map(
            |coin| match (CoinID::try_from(coin.0), CoinName::try_from(coin.1.clone())) {
                (Ok(id), Ok(name)) => Ok(Coin::new(id, name)),
                _ => Err(()),
            },
        )
        .collect()
    {
        Ok::<Vec<Coin>, _>(coins) => {
            match repo.exists_ids(coins.iter().map(|coin| coin.id()).collect()) {
                Ok::<Vec<CoinID>, _>(ids) => {
                    match (
                        repo.update_batch(
                            coins.iter().take_while(|c| ids.contains(&c.id())).map(|c|c.clone()).collect(),
                        ),
                        repo.insert_batch(
                            coins.iter().skip_while(|c| ids.contains(&c.id())).map(|c|c.clone()).collect(),
                        ),
                    ) {
                        (Ok(_), Ok(_)) => return Response::Ok,
                        _ => return Response::Error,
                    }
                }
                Err(_) => return Response::BadRequest,
            }
        }
        Err(_) => return Response::BadRequest,
    }
}
