extern crate proc_macro;
extern crate syn;

use colored::*;
use proc_macro2::TokenStream;
use types::DescribeStatement;

mod parser;
mod types;

#[proc_macro]
pub fn ruspec(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = proc_macro2::TokenStream::from(input);

    let expanded = match _ruspec(input) {
        Ok(token_stream) => token_stream,
        Err(e) => {
            eprintln!("{}: {}", "error".red().bold(), e);
            std::process::exit(1);
        }
    };

    proc_macro::TokenStream::from(expanded)
}

fn _ruspec(input: proc_macro2::TokenStream) -> Result<TokenStream, failure::Error> {
    let describe_statements = parser::Parser::new(input).parse()?;
    Ok(DescribeStatement::expands(describe_statements))
}

#[cfg(test)]
mod tests {
    use crate::_ruspec;
    use quote::quote;

    #[test]
    fn should_output_expected_stream() {
        let input = quote! {
            describe "hoge" {
                it "hoge" {
                    assert!(true);
                }
            }
        };

        let expected = quote! {
            mod hoge {
                #[test]
                fn hoge() {
                    assert!(true);
                }
            }
        };

        let output = crate::_ruspec(input).unwrap();
        assert_eq!(output.to_string(), expected.to_string())
    }

    #[test]
    fn should_expand_before() {
        let input = quote! {
            describe "hoge" {
                before { let hoge = 1234; }
                it "hoge" {
                    assert!(true);
                }
            }
        };

        let expected = quote! {
            mod hoge {
                #[test]
                fn hoge() {
                    let hoge = 1234;
                    assert!(true);
                }
            }
        };

        let output = crate::_ruspec(input).unwrap();
        assert_eq!(output.to_string(), expected.to_string())
    }

    #[test]
    fn should_expand_after() {
        let input = quote! {
            describe "hoge" {
                after { let hoge = 1234; }
                it "hoge" {
                    assert!(true);
                }
            }
        };

        let expected = quote! {
            mod hoge {
                #[test]
                fn hoge() {
                    assert!(true);
                    let hoge = 1234;
                }
            }
        };

        let output = crate::_ruspec(input).unwrap();
        assert_eq!(output.to_string(), expected.to_string())
    }

    #[test]
    fn should_expand_subject() {
        let input = quote! {
            describe "hoge" {
                subject { true }
                it "hoge" {
                    assert!(subject);
                }
            }
        };

        let expected = quote! {
            mod hoge {
                #[test]
                fn hoge() {
                    // FIXME Expected code is
                    // assert!(true)
                    let subject = (true);
                    assert!(subject);
                }
            }
        };

        let output = crate::_ruspec(input).unwrap();
        assert_eq!(output.to_string(), expected.to_string())
    }

}
