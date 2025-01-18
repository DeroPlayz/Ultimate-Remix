#![feature(
    concat_idents,
    proc_macro_hygiene
)]
#![allow(
    unused_macros
)]

mod ganon;
mod purin;
// mod usosf;

#[skyline::main(name = "plugin.nro")]
pub fn main() {
    ganon::install();
    purin::install();
    // usosf::install();
}