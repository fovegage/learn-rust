#![feature(type_alias_impl_trait)]

pub struct S;

#[volo::async_trait]
impl volo_gen::volo::example::ItemService for S {}
