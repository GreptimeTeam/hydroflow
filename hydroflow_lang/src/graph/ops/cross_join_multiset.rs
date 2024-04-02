use quote::quote_spanned;
use syn::parse_quote;

use super::{GraphEdgeType, OperatorCategory, OperatorConstraints, WriteContextArgs, RANGE_1};

/// > 2 input streams of type S and T, 1 output stream of type (S, T)
///
/// Forms the multiset cross-join (Cartesian product) of the (possibly duplicated) items in the input streams, returning all
/// tupled pairs regardless of duplicates.
///
/// ```hydroflow
/// source_iter(vec!["happy", "happy", "sad"]) -> [0]my_join;
/// source_iter(vec!["dog", "cat", "cat"]) -> [1]my_join;
/// my_join = cross_join_multiset() -> sort() -> assert_eq([
///     ("happy", "cat"),
///     ("happy", "cat"),
///     ("happy", "cat"),
///     ("happy", "cat"),
///     ("happy", "dog"),
///     ("happy", "dog"),
///     ("sad", "cat"),
///     ("sad", "cat"),
///     ("sad", "dog"), ]);
/// ```
pub const CROSS_JOIN_MULTISET: OperatorConstraints = OperatorConstraints {
    name: "cross_join_multiset",
    categories: &[OperatorCategory::MultiIn],
    hard_range_inn: &(2..=2),
    soft_range_inn: &(2..=2),
    hard_range_out: RANGE_1,
    soft_range_out: RANGE_1,
    num_args: 0,
    persistence_args: &(0..=2),
    type_args: &(0..=1),
    is_external_input: false,
    ports_inn: Some(|| super::PortListSpec::Fixed(parse_quote! { 0, 1 })),
    ports_out: None,
    input_delaytype_fn: |_| None,
    input_edgetype_fn: |_| Some(GraphEdgeType::Value),
    output_edgetype_fn: |_| GraphEdgeType::Value,
    flow_prop_fn: None,
    write_fn: |wc @ &WriteContextArgs {
                   op_span,
                   ident,
                   inputs,
                   ..
               },
               diagnostics| {
        let mut output = (super::join_multiset::JOIN_MULTISET.write_fn)(wc, diagnostics)?;

        let lhs = &inputs[0];
        let rhs = &inputs[1];
        let write_iterator = output.write_iterator;
        output.write_iterator = quote_spanned!(op_span=>
            let #lhs = #lhs.map(|a| ((), a));
            let #rhs = #rhs.map(|b| ((), b));
            #write_iterator
            let #ident = #ident.map(|((), (a, b))| (a, b));
        );

        Ok(output)
    },
};
