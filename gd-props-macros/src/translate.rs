use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use venial::Declaration;

pub fn translate<F>(input: TokenStream, fun: F) -> TokenStream
where
    F: FnOnce(Declaration) -> Result<TokenStream2, venial::Error>,
{
    let tokens2 = TokenStream2::from(input);

    let res = venial::parse_declaration(tokens2)
        .and_then(fun)
        .unwrap_or_else(|e| e.to_compile_error());

    TokenStream::from(res)
}
