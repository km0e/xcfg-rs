use xcfg::XCfg;

#[derive(XCfg)]
union Bar {
    a: u32,
    b: f32,
}

fn main() {}
