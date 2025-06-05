//@ check-pass
//@ edition: 2015

#![deny(unused_parens)]

pub type DynIsAContextualKeywordIn2015 = Box<dyn (::std::ops::Fn())>;

fn main() {}
