
pub struct Upgrade{
    pub damage_lvl:i32,
    pub cost:f32,
    pub hp: f32,
    pub lvl: i32,
    pub reward: f32,
}

impl Upgrade {
    pub fn update_damage_lvl(lvl: i32, cost: f32) -> (i32, f32) {
        let mut lvl = lvl;
        let mut cost = cost;
        cost = cost * lvl as f32;
        lvl += 1;
        (lvl, cost)
    }
    pub fn update_cost(cost_u: f32) -> f32 {
        let mut cost = cost_u;
        cost += 10.;
        cost
    }
    pub fn hp_update(hp:f32)->f32 {
        let mut hp = hp;
        hp +=100.;
        hp
    }
    pub fn lvl_update(lvl:i32)->i32{
        let mut lvl = lvl;
        lvl+=1;
        lvl
    }
    pub fn reward_update(reward:f32)->f32 {
        let mut reward = reward;
        reward=reward+10.0;
        reward
    }
}
