//use embedded_hal::blocking::i2c::{Write, WriteRead};


pub enum Action {
    None,
    GoForward { speed: u16 },
    TurnTo { deg: u16 },
}

pub struct TurnTask {
    current_action: Option<Action>,
}

// impbl<TI2C: Write + WriteRead> TurnTask {


//     // pub fn poll() -> nb::Result<(),  {

//     // }
// }
