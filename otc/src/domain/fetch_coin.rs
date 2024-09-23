use crate::repositories::coin::{FetchOneError, Repository};

use super::entities::{Coin, CoinID};

pub struct Request {
    pub id: u16,
}

pub enum Response {
    Ok(Coin),
    BadRequest,
    NotFound,
    Unknown,
}

pub fn execute(repo: &mut dyn Repository, req: Request) -> Response {
    match CoinID::try_from(req.id) {
        Ok(id) => match repo.fetch_one(id) {
            Ok(coin) => Response::Ok(coin),
            Err(FetchOneError::NotFound) => Response::NotFound,
            Err(FetchOneError::Unknown) => Response::Unknown,
        },
        _ => Response::BadRequest,
    }
}

#[cfg(test)]
mod tests {
    use crate::{repositories::coin::{InMemoryRepository, Insert}, domain::entities::CoinName};

    use super::*;

    #[test]
    fn it_should_return_a_not_found_error_when_the_repo_does_not_contain_the_coin() {
        let mut repo = InMemoryRepository::new();
        let req = Request { id: u16::from(CoinID::usdt()) };
        let res = execute(&mut repo, req);
        match res {
            Response::NotFound => {}
            _ => unreachable!(),
        }
    }
    #[test]
    fn it_should_return_the_coin_otherwise() {
        let mut repo = InMemoryRepository::new();
        match repo.insert(CoinID::usdt(), CoinName::usdt()) {
            Insert::Ok(_) => (),
            _ => unreachable!()
        }
        let req = Request {id: u16::from(CoinID::usdt())};
        let res = execute(&mut repo, req);
        match res {
            Response::Ok(res) => {
                assert_eq!(res.id(), CoinID::usdt());
                assert_eq!(res.name(), CoinName::usdt());
            },
            _ => unreachable!()
        }
    }
}
