mod it;
mod parser;

use crate::it::it_impl;
use proc_macro::TokenStream;

#[proc_macro]
pub fn it(input: TokenStream) -> TokenStream {
  it_impl(input)
}
