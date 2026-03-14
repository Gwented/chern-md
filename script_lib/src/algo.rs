use std::cmp;

use common::keywords;
// C++ dev got lost

pub(crate) enum FuzzyMatch {
    KW,
    Arg,
}

pub fn fuzzy_match(given: &[u8], target: FuzzyMatch) -> Option<&str> {
    // Could make largest handled at the match itself but. Well it doesn't matter.
    match target {
        FuzzyMatch::KW => fuzzy_match_inner(given, &keywords::KEYWORDS_ARRAY, keywords::LARGEST_KW),
        FuzzyMatch::Arg => fuzzy_match_inner(
            given,
            // TODO: Please rename this
            // I don't know maybe
            &common::symbols::ARGS_ARRAY,
            common::symbols::LARGEST_ARG,
        ),
    }
}

/// `given` represents the given bytes that are to be compared to the elements of `arr`.
/// `largest` represents the largest element of `arr` so that an early return is possible.
// Returns option string instead of index because args aren't loaded at startup
fn fuzzy_match_inner<'a, 'b>(given: &'a [u8], arr: &'b [&str], largest: usize) -> Option<&'b str> {
    if given.len() > largest {
        return None;
    }

    for (i, var) in arr.iter().enumerate() {
        let mut chances = 2;
        let mut matched = 0;

        let var_bytes = var.as_bytes();

        // PLEASE DONT MAKE ME IMPORT
        let size_diff =
            cmp::max(given.len(), var_bytes.len()) - cmp::min(given.len(), var_bytes.len());

        if size_diff > 2 {
            continue;
        }

        let cap = cmp::min(given.len(), var_bytes.len());

        for j in 0..cap {
            if given[j] == var_bytes[j] {
                matched += 1;
                chances = 1;
            } else if chances == 0 {
                break;
            } else {
                chances -= 1;
            }
        }

        if matched >= 2 {
            // Similar
            return Some(arr[i]);
        }
    }

    None
}
