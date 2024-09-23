/// 热水器，战胜寒冬的有利武器
pub struct WaterHeater {
    // 温度
    temperature: i32,
    observers: Vec<Box<dyn Observer<Self>>>,
}

impl WaterHeater {
    /// 创建一个热水器实例，默认温度 25 度
    pub fn new() -> WaterHeater {
        WaterHeater {
            temperature: 25,
            observers: vec![],
        }
    }
    /// 给热水器设置温度
    pub fn set_temperature(&mut self, temperature: i32) {
        self.temperature = temperature;
        println!("当前温度是：{} ℃", self.temperature);
        self.notifies();
    }
    /// 温度变化后进行通知
    fn notifies(&self) {
        self.observers
            .iter()
            .for_each(|observer| observer.update(self));
    }
    /// 添加一个观察者
    pub fn add_observer(&mut self, observer: Box<dyn Observer<Self>>) {
        self.observers.push(observer);
    }

    /// 获取热水器的温度
    pub fn get_temerature(&self) -> i32 {
        self.temperature
    }
}

/// 洗澡模式
pub struct WashingMode {}
impl Observer<WaterHeater> for WashingMode {
    fn update(&self, wh: &WaterHeater) {
        if wh.get_temerature() >= 50 && wh.get_temerature() < 70 {
            println!("水已烧好！温度正好，可以用来洗澡了。");
        }
    }
}
/// 饮用模式
pub struct DrinkingMode {}
impl Observer<WaterHeater> for DrinkingMode {
    fn update(&self, wh: &WaterHeater) {
        if wh.get_temerature() >= 100 {
            println!("水已烧开！可以用来饮用了。");
        }
    }
}
/// 观察者抽象
pub trait Observer<T> {
    /// 更新方法，当被监听数据变化时，会触发该方法
    fn update(&self, wh: &T);
}
/// 被观察者
pub struct Observable<T> {
    observers: Vec<Box<dyn Observer<T>>>,
}
impl<T> Observable<T> {
    pub fn new() -> Observable<T> {
        Observable { observers: vec![] }
    }
    pub fn add_observer(&mut self, observer: Box<dyn Observer<T>>) {
        self.observers.push(observer);
    }
    pub fn remove_observer(&mut self, observer: Box<dyn Observer<T>>) {}
    pub fn notify(&self, msg: &T) {
        self.observers.iter().for_each(|o| o.update(msg));
    }
}

// 模拟登录异常检测与提醒

use std::collections::HashMap;

pub struct Account {
    observable: Observable<Self>,
}
impl Account {
    fn new() -> Account {
        Account {
            observable: Observable::new(),
        }
    }
    fn login(&self, name: &str, ip: &str, time: &str) {
      if let Some(region) = self.get_region(ip) {

      } 
    }
    fn get_region(&self, ip: &str) -> Option<String> {
        let mut ip_regions = HashMap::new();
        ip_regions.insert("101.47.18.9", String::from("浙江省杭州市"));
        ip_regions.insert("67.218.147.69", String::from("美国洛杉矶"));
        if let Some(_s) = ip_regions.get(ip) {
            return Some(_s.clone());
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_water_heater() {
        let mut wh = WaterHeater::new();
        let washing_obser = WashingMode {};
        let drinking_obser = DrinkingMode {};
        wh.add_observer(Box::new(washing_obser));
        wh.add_observer(Box::new(drinking_obser));
        wh.set_temperature(40);
        wh.set_temperature(60);
        wh.set_temperature(100);
    }
}
