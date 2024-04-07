pub struct Battleship {
    pub name: BattleshipName,
    pub size: u32,
}

#[derive(strum::EnumIter, Debug, strum::VariantArray, Clone, Copy)]
pub enum BattleshipName {
    GALACTIC,
    FORESTER,
    HANGOVER,
    CAMPGROUND,
    WASHBASIN,
}
