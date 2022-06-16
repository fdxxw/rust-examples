#[derive(Debug)]
pub enum Event {
    PlayerHitObstacle,                // 玩家撞击障碍物
    EntityMoved(EntityMoved),         // 实体移动事件
    BoxPlacedOnSpot(BoxPlacedOnSpot), // 箱子放到地点
    Won // 通过一关
}

pub type EntityId = u32;
#[derive(Debug)]
pub struct EntityMoved {
    pub id: EntityId,
}
#[derive(Debug)]
pub struct BoxPlacedOnSpot {
    pub is_correct_spot: bool, // 正确的放置
}
