extern crate proc_macro;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

/// The `benchmark` macro is a procedural macro that measures the execution time of a function.
///
/// This macro is used as an attribute on a function. When the function is called, it records the current time,
/// executes the function, then prints the elapsed time to the console.
///
/// # Arguments
///
/// * `_attr: TokenStream` - The attributes applied to the macro. This is currently unused.
/// * `item: TokenStream` - The function to be benchmarked.
///
/// # Returns
///
/// * `TokenStream` - The generated code, which includes the benchmarking logic.
///
/// # Example
///
/// ```rust, ignore
/// #[benchmark]
/// fn my_function() {
///     // Some code...
/// }
/// ```
#[proc_macro_attribute]
pub fn benchmark(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);

    let ItemFn { attrs, vis, sig, block } = input_fn;

    let output = quote! {
        #(#attrs)* #vis #sig {
            let _instant = std::time::Instant::now();
            let _result = (|| #block )();
            let benchmark_msg = format!("BENCHMARK<{}> = {:?}ms", stringify!(#sig), _instant.elapsed().as_millis());
            dbg!(benchmark_msg);
            _result
        }
    };

    TokenStream::from(output)
}
