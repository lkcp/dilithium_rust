

pub static Q:i32 = 8380417;
pub static QINV:i32 = 58728449; //q^-1 mod R
pub static MONT:i32 = -4186625; // R mod q

pub static d: u8 = 13;

pub fn get_params(level: u8) -> (u8, u8, u8) //k, l, eta
{
    match level {
        2 => (4, 4, 2),
        3 => (6, 5, 4),
        5 => (8, 7, 2),
        // others will panic
        _ => panic!("security level not supported"),
    }
}