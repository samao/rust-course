pub mod arrays;
pub mod async_code;
pub mod atomic;
pub mod channel;
pub mod complex;
pub mod dst;
pub mod grand;
pub mod hm;
pub mod iter;
pub mod ptr;
pub mod ref_loop;
pub mod semaphore;
pub mod string;
pub mod structs;
pub mod t;
pub mod thread;
pub mod un_safe_code;
pub mod vector;
pub mod worker;

mod g {
    #[allow(dead_code)]
    fn say_g() {
        super::a::c::d::say_d();
        // 模块不可见，无法调用 super::a::c::d::say_no()
    }
}

pub mod a {
    pub trait Draw {
        fn draw();
    }

    mod b {
        pub fn say_b() {
            super::c::d::say_d();
        }
    }

    pub mod c {
        pub fn say_c() {
            super::b::say_b();
        }
        pub mod d {
            pub fn say_d() {}

            pub(in super::super) fn say_no() {}
        }
    }

    pub mod m {
        pub fn say_m() {
            super::c::d::say_d();
            super::c::d::say_no();
        }
    }

    pub fn say_a() {
        c::say_c();
        c::d::say_no();
    }
}
