#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Quality {
    Highest,
    Balanced,
    Performance,
    Lowest,
}

#[derive(Debug)]
pub struct Meta {
    pub tick: u64,
    pub quality: Quality,
}

impl Meta {
    fn tick(&self) -> u64 {
        self.tick
    }
}
