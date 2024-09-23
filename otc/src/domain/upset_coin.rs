use crate::repositories::coin::{FetchOneError, Insert, Repository};

use super::entities::{Coin, CoinID, CoinName};

struct Request {
    id: u16,
    name: String,
}

enum Response {
    Ok(u16),
    BadRequest,
    Error,
}
fn execute(repo: &mut dyn Repository, req: Request) -> Response {
    match (CoinID::try_from(req.id), CoinName::try_from(req.name)) {
        (Ok(id), Ok(name)) => match repo.fetch_one(id) {
            Ok(_) => match repo.update(Coin::new(id, name)) {
                Ok(_) => Response::Ok(u16::from(id)),
                Err(_) => Response::Error,
            },
            Err(error) => match error {
                FetchOneError::NotFound => match repo.insert(id, name) {
                    Insert::Ok(id) => Response::Ok(u16::from(id)),
                    Insert::Error => Response::Error,
                    Insert::Conflict => Response::Error,
                },
                FetchOneError::Unknown => Response::Error,
            },
        },
        _ => Response::BadRequest,
    }
}

#[cfg(test)]
mod tests {
    use crate::repositories::coin::InMemoryRepository;

    use super::*;
    #[test]
    fn it_should_return_ok() {
        let mut repo = InMemoryRepository::new();
        let req = Request {
            id: u16::from(CoinID::usdt()),
            name: String::from(CoinName::usdt()),
        };
        let res = execute(&mut repo, req);
        match res {
            Response::Ok(id) => assert_eq!(id, u16::from(CoinID::usdt())),
            _ => unreachable!(),
        }
    }
}
