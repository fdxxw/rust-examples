use std::{sync::Mutex, collections::HashSet};

use crate::domain::entities::{Coin, CoinID, CoinName};

pub enum Insert {
    Ok(CoinID),
    Conflict,
    Error,
}
pub enum FetchOneError {
    NotFound,
    Unknown,
}
pub enum FetchListError {
    Unknown,
}
pub enum UpdateError {
    NotFound,
    Unknown,
}
pub enum ExistsError {
    Unknown,
}

pub enum InsertBatchError {
    Unknown,
}
pub trait Repository: Send + Sync {
    fn insert(&mut self, id: CoinID, name: CoinName) -> Insert;
    fn fetch_one(&self, id: CoinID) -> Result<Coin, FetchOneError>;
    // fn fetch_by_ids(&self, ids: Vec<CoinID>) -> Result<Coin, FetchListError>;
    fn update(&mut self, coin: Coin) -> Result<(), UpdateError>;
    fn update_batch(&mut self, coins: Vec<Coin>) -> Result<(), UpdateError>;
    fn insert_batch(&mut self, coins: Vec<Coin>) -> Result<(), InsertBatchError>;
    fn exists_ids(&self, ids: Vec<CoinID>) -> Result<Vec<CoinID>, ExistsError>;
}

pub struct InMemoryRepository {
    coins: Mutex<Vec<Coin>>,
}

impl InMemoryRepository {
    pub fn new() -> Self {
        Self {
            coins: Mutex::new(vec![]),
        }
    }
}
impl Repository for InMemoryRepository {
    fn insert(&mut self, id: CoinID, name: CoinName) -> Insert {
        let mut lock = match self.coins.lock() {
            Ok(lock) => lock,
            _ => return Insert::Error,
        };
        if lock.iter().any(|coin| coin.id() == id) {
            return Insert::Conflict;
        }
        lock.push(Coin::new(id, name));
        Insert::Ok(id)
    }

    fn fetch_one(&self, id: CoinID) -> Result<Coin, FetchOneError> {
        let lock = match self.coins.lock() {
            Ok(lock) => lock,
            _ => return Err(FetchOneError::Unknown),
        };
        match lock.iter().find(|coin| coin.id() == id) {
            Some(c) => Ok(c.clone()),
            None => Err(FetchOneError::NotFound),
        }
    }

    fn update(&mut self, coin: Coin) -> Result<(), UpdateError> {
        let mut lock = match self.coins.lock() {
            Ok(lock) => lock,
            _ => return Err(UpdateError::Unknown),
        };
        match lock.iter().position(|coin| coin.id() == coin.id()) {
            Some(index) => {
                lock.remove(index);
                lock.push(coin.clone());
                Ok(())
            }
            None => Err(UpdateError::NotFound),
        }
    }

    fn update_batch(&mut self, coins: Vec<Coin>) -> Result<(), UpdateError> {
      let mut lock = match self.coins.lock() {
        Ok(lock) => lock,
        _ => return Err(UpdateError::Unknown)
      };
      let update_ids: HashSet<CoinID> = coins.iter().map(|coin|coin.id()).collect();
      // delete
      lock.retain(|coin|!update_ids.contains(&coin.id()));
      coins.iter().for_each(|coin|lock.push(coin.clone()));
      Ok(())
    }

    fn insert_batch(&mut self, coins: Vec<Coin>) -> Result<(), InsertBatchError> {
      let mut lock = match self.coins.lock() {
        Ok(lock) => lock,
        _ => return Err(InsertBatchError::Unknown)
      };
      // delete
      coins.iter().for_each(|coin|lock.push(coin.clone()));
      Ok(())
    }

    fn exists_ids(&self, ids: Vec<CoinID>) -> Result<Vec<CoinID>, ExistsError> {
        let mut r: Vec<CoinID> = vec![];
        let lock = match self.coins.lock() {
          Ok(lock) => lock,
          _ => return Err(ExistsError::Unknown)
        };
        let ids: HashSet<CoinID> = ids.iter().map(|id|id.clone()).collect();
        lock.iter().for_each(|coin| {
          if ids.contains(&coin.id()) {
            r.push(coin.id());
          }
        });
        Ok(r)
    }
}
