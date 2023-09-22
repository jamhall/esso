use crate::parser::{Function, TestCase};
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

pub fn it_impl(input: TokenStream) -> TokenStream {
  let ast: TestCase = parse_macro_input!(input);
  let description = ast.0;
  TokenStream::from(match ast.1 {
    Function::Sync(statements) => {
      quote! {
          #[test]
          fn #description() -> Result<(), Box<(dyn std::error::Error + 'static)>> {
              #( #statements )*
              Ok(())
          }
      }
    }
    Function::Async(statements) => {
      quote! {
          #[tokio::test]
          async fn #description() -> Result<(), Box<(dyn std::error::Error + 'static)>> {
              #( #statements )*
              Ok(())
          }
      }
    }
  })
}
