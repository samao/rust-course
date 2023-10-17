use log::{debug, info, warn};

/*
 * Copyright (c) QieTv, Inc. 2018
 * @Author: idzeir
 * @Date: 2023-10-08 10:52:34
 * @Last Modified by: idzeir
 * @Last Modified time: 2023-10-08 11:51:59
 */
pub fn dst() {
    info!("GOOD dst");
    let x = Me::C as i32;
    match x.try_into() {
        Ok(Me::A) => info!("A"),
        Ok(Me::B) => info!("B"),
        Ok(Me::C) => info!("C"),
        Ok(Me::D) => info!("D"),
        _ => warn!("unknow num!"),
    }

    debug!("{:?}", Location::Known(Coord));

    debug!("{:?}", Complex::Something("soth to do".to_string()));

    debug!(
        "{:?}",
        Complex::LotOfThing {
            blah: "okok".to_string(),
            usual_struct_stuff: false
        }
    );

    transmute();
}

#[allow(dead_code)]
#[repr(i8)]
enum YEnum {
    A = 1,
    B,
    C,
}

fn transmute() {
    let x = YEnum::B;
    let y = x as u8;
    let z: YEnum = unsafe { std::mem::transmute(y) };

    match z {
        YEnum::A => info!("A"),
        YEnum::B => info!("B"),
        YEnum::C => info!("C"),
    }
}

#[macro_export]
macro_rules! back_to_enum {
    ($(#[$meta:meta])* $vis:vis enum $name:ident {
        $($(#[$vmeta:meta])* $vname:ident $(= $val:expr)?,)*
    }) => {
        $(#[$meta])*
        $vis enum $name {
            $($(#[$vmeta])* $vname $(= $val)?,)*
        }

        impl std::convert::TryFrom<i32> for $name {
            type Error = ();

            fn try_from(v: i32) -> Result<Self, Self::Error> {
                match v {
                    $(x if x == $name::$vname as i32 => Ok($name::$vname),)*
                    _ => Err(()),
                }
            }
        }
    };
}

back_to_enum! {
    enum Me {
        A,
        B,
        C,
        D,
    }
}
#[derive(Debug)]
struct Coord;
#[derive(Debug)]
#[allow(dead_code)]
enum Location {
    Unknown,
    Anonymous,
    Known(Coord),
}

#[derive(Debug)]
#[allow(dead_code)]
enum Complex {
    Nothing,
    Something(String),
    LotOfThing {
        blah: String,
        usual_struct_stuff: bool,
    },
}

// impl TryFrom<i32> for Me {
//     type Error = ();
//     fn try_from(value: i32) -> Result<Self, Self::Error> {
//         match value {
//             x if x == Me::A as i32 => Ok(Me::A),
//             x if x == Me::B as i32 => Ok(Me::B),
//             x if x == Me::C as i32 => Ok(Me::C),
//             x if x == Me::D as i32 => Ok(Me::D),
//             _ => Err(()),
//         }
//     }
// }
