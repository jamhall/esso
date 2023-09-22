use convert_case::{Case, Casing};
use quote::format_ident;
use syn::parse::{Parse, ParseStream, Result};
use syn::{Error, ExprAsync, ExprBlock, Ident, Lit, Stmt, Token};

pub enum Function {
  Sync(Vec<Stmt>),
  Async(Vec<Stmt>),
}

pub struct TestCase(pub Ident, pub Function);

impl Parse for TestCase {
  fn parse(input: ParseStream<'_>) -> Result<TestCase> {
    if input.is_empty() {
      return Err(Error::new(
        input.span(),
        "a test case and function is required",
      ));
    }

    let description = match input.parse()? {
      Lit::Str(string) => format_ident!("{}", string.value().to_case(Case::Snake)),
      _ => return Err(Error::new(input.span(), "Expected a string")),
    };

    input.parse::<Token![,]>()?;

    let expression = match input.lookahead1().peek(Token![async]) {
      true => input
        .parse::<ExprAsync>()
        .map(|expression| expression.block.stmts)
        .map(Function::Async),
      false => input
        .parse::<ExprBlock>()
        .map(|expression| expression.block.stmts)
        .map(Function::Sync),
    }?;

    Ok(Self(description, expression))
  }
}
