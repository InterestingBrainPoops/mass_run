#[derive(Clone, Debug)]
pub struct Board {
    pub snakes: Vec<BattleSnake>,
    pub food: Vec<Coordinate>,
    pub hazards: Vec<Coordinate>,
}
#[derive(Clone, Debug)]
pub struct Game {
    pub turn: u32,
    pub game: GameInfo,
    pub board: Board,
}
#[derive(Clone, Debug)]
pub struct GameInfo {
    id: String,
    ruleset: RuleSet,
    timeout: u32,
}
#[derive(Clone, Debug)]
pub struct RuleSet {
    name: String,
    version: String,
}
#[derive(Clone, Debug)]

pub struct Coordinate {
    x: u32,
    y: u32,
}
#[derive(Clone, Debug)]

pub struct BattleSnake {
    id: String,
    name: String,
    health: u8,
    body: Vec<Coordinate>,
    latency: String,
    head: Coordinate,
    length: u16,
    shout: String,
    squad: String,
}
