

pub static Q:i32 = 8380417;
pub static QINV:i32 = 58728449; //q^-1 mod R
pub static MONT:i32 = -4186625; // R mod q;

pub static d: u8 = 13;

pub fn get_params(level: u8) -> (i32, i32, i32) //k, l, eta
{
    match level {
        2 => (4, 4, 2),
        3 => (6, 5, 4),
        5 => (8, 7, 2),
        // others will panic
        _ => panic!("security level not supported"),
    }
}

pub fn get_params_sign(level: u8) -> (i32, i32, i32, i32, i32) //k, l, eta, gamma1, gamma2
{
    match level {
        2 => (4, 4, 2, 1<<17, 95232),
        3 => (6, 5, 4, 1<<19, 261888),
        5 => (8, 7, 2, 1<<19, 261888),
        // others will panic
        _ => panic!("security level not supported"),
    }
}