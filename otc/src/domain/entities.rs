#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct CoinID(u16);

impl TryFrom<u16> for CoinID {
    type Error = ();
    fn try_from(id: u16) -> Result<Self, Self::Error> {
        if id > 0 && id < 999 {
            Ok(Self(id))
        } else {
            Err(())
        }
    }
}

impl From<CoinID> for u16 {
    fn from(id: CoinID) -> Self {
        id.0
    }
}

#[cfg(test)]
impl CoinID {
    pub fn usdt() -> Self {
        Self(25)
    }
    pub fn btc() -> Self {
        Self(4)
    }
    pub fn bad() -> Self {
        Self(0)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CoinName(String);

impl TryFrom<String> for CoinName {
    type Error = ();
    fn try_from(name: String) -> Result<Self, Self::Error> {
        if name.is_empty() {
            Err(())
        } else {
            Ok(Self(name))
        }
    }
}

impl From<CoinName> for String {
    fn from(name: CoinName) -> Self {
        name.0
    }
}

#[cfg(test)]
impl CoinName {
    pub fn usdt() -> Self {
        Self(String::from("USDT"))
    }
    pub fn btc() -> Self {
        Self(String::from("BTC"))
    }
    pub fn bad() -> Self {
        Self(String::from(""))
    }
}

#[derive(Clone)]
pub struct Coin {
    id: CoinID,
    name: CoinName,
}

impl Coin {
    pub fn new(id: CoinID, name: CoinName) -> Self {
        Self { id, name }
    }
    pub fn id(&self) -> CoinID {
        self.id
    }
    pub fn name(&self) -> CoinName {
      self.name.clone()
  }
}
