use core::fmt;
use std::fs::File;

use log::info;

/*
 * Copyright (c) QieTv, Inc. 2018
 * @Author: idzeir
 * @Date: 2023-10-12 15:09:49
 * @Last Modified by: idzeir
 * @Last Modified time: 2023-10-12 17:52:24
 */
pub fn up() {
    let s1 = Some("some1");
    let s2 = Some("some2");
    let n: Option<&str> = None;
    info!("or: {:?}", None.or(None).or(s2).or(s1).or(None));

    let o1: Result<i32, i32> = Ok(1);
    let o2: Result<i32, i32> = Ok(2);
    let e: Result<i32, i32> = Err(9);
    info!("or: {:?}", Err(2).or(o2).or(Err(3)).or(o1));

    info!("some and some: {:?}", s1.and(s2));
    info!("some and some and none: {:?}", s1.and(s2).and(n));
    info!("ok and ok: {:?}", o1.and(o2));
    info!("ok and ok and err: {:?}", o1.and(o2).and(e));
    or_else_and_then();
}

fn or_else_and_then() {
    let s1 = Some("some1");
    let s2 = Some("some2");
    let fn_some = || Some("some2");

    let n: Option<&str> = None;
    let fn_none = || None;

    assert_eq!(s1.or_else(fn_some), s1);
    assert_eq!(s1.or_else(fn_none), s1);
    assert_eq!(n.or_else(fn_some), s2);
    assert_eq!(n.or_else(fn_none), None);

    // result
    let o1 = Ok(1);
    let o2 = Ok(2);
    let fn_ok = |_| Ok(2);

    let e1 = Err(9);
    let e2 = Err(8);
    let fn_err = |_| Err(8);

    assert_eq!(o1.or_else(fn_ok), o1);
    assert_eq!(o1.or_else(fn_err), o1);
    assert_eq!(e1.or_else(fn_ok), o2);
    assert_eq!(e1.or_else(fn_err), e2);

    assert_eq!(Some(8).filter(|x: &i32| x % 2 == 0), Some(8));
    info!("{:?}", vec![1, 2, 3, 4, 5, 19].max(vec![9]));

    info!("map: {:?}", Some(8).map(|n| format!("num is: {}", n)));
    info!("map: {:?}", None::<i32>.map(|_| 98));
    info!(
        "map: {:?}",
        Ok::<i32, i32>(9).map(|n| format!("ok num is {}", n))
    );
    info!("map: {:?}", Err::<i32, i32>(0).map(|_| Some(10)));

    let o1: Result<&str, &str> = Ok("abcde");
    let o2: Result<&str, isize> = Ok("abcde");

    let e1: Result<&str, &str> = Err("404");
    let e2: Result<&str, isize> = Err(404);

    let fn_character_count = |s: &str| -> isize { s.parse().unwrap() }; // 该函数返回一个 isize

    assert_eq!(o1.map_err(fn_character_count), o2); // Ok1 map = Ok2
    assert_eq!(e1.map_err(fn_character_count), e2); // Err1 map = Err2

    const V_DEFAULT: u32 = 1;

    let s: Result<u32, ()> = Ok(10);
    let n: Option<u32> = None;
    let fn_closure = |v: u32| v + 2;

    // assert_eq!(Some(10), 10);
    assert_eq!(s.map_or(V_DEFAULT, fn_closure), 12);
    assert_eq!(n.map_or(V_DEFAULT, fn_closure), V_DEFAULT);
    custom_error();
}

fn custom_error() {
    match produce_error() {
        Ok(_) => (),
        Err(er) => info!("error show display: {}", er),
    }

    info!("show debug {:?}", produce_error());
    // info!("show debug {:#?}", produce_error());

    this_error();
}

fn produce_error() -> Result<(), MediaError> {
    // Err(MediaError {
    //     code: 404,
    //     message: "Page not found".to_string(),
    // })

    File::open("somenotexist_file.mp4")?;
    "a5".parse::<u8>()?;
    Ok(())
}

// #[derive(Debug)]
struct MediaError {
    code: usize,
    message: String,
}

impl fmt::Display for MediaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let err_msg = match self.code {
            404 => "sorry, can not find the page!",
            _ => "sorry, something is wrong! please try again",
        };
        write!(f, "{}", err_msg)
    }
}

impl fmt::Debug for MediaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            r#"MediaError {{ code: {}, message: "{}" }}"#,
            self.code, self.message
        )
    }
}

impl From<std::io::Error> for MediaError {
    fn from(err: std::io::Error) -> Self {
        MediaError {
            code: 987,
            message: err.to_string(),
        }
    }
}

impl From<std::num::ParseIntError> for MediaError {
    fn from(value: std::num::ParseIntError) -> Self {
        MediaError {
            code: 1000,
            message: value.to_string(),
        }
    }
}

fn this_error() {
    info!("{:?}", create_error());

    if let Err(err) = create_error() {
        info!("{}", err);
    }
}

fn create_error() -> Result<(), MError> {
    std::env::var("MARKDOWN")?;
    std::fs::read_to_string("not exist.file")?;
    Ok(())
}

#[derive(thiserror::Error, Debug)]
enum MError {
    #[error("Enviroment variable not fount")]
    EnvironmentVariableNotFount(#[from] std::env::VarError),
    #[error("transparent")]
    IOError(#[from] std::io::Error),
}

// impl Display for MError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self.0 {
//             MError::EnvironmentVariableNotFount(info) => write!(f, "VAR_ERROR: {}", info.to_string()),
//             MError::IOError(info) => write!(f, "IOERROR: {}", info.to_string()),
//         }
//     }
// }
