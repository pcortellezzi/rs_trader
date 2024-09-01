use enumflags2::{bitflags, BitFlags};

#[bitflags]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Features {
    OrderRouting = 0b001,
    MarketData = 0b010,
    HistoricalData = 0b100,
}

#[typetag::serde(tag = "connector")]
pub trait Connector {
    fn features(self: &Self) -> BitFlags<Features>;
    fn name(self: &Self) -> &str;
    //fn as_any(&self) -> &dyn Any;
}

/*impl Clone for Box<dyn Connector> {
    fn clone(&self) -> Self {
        self.clone()
    }
}*/
