#[doc(hidden)]
pub extern crate cagra_parser as imp;

#[macro_export]
macro_rules! graph {
    ($scalar:ty, $proc:block) => {{
        // non-hygienic hack
        // See https://qiita.com/ubnt_intrepid/items/dcfabd5b0ae4d4e105da (Japanese)
        enum DummyGraphNew {}
        impl DummyGraphNew {
            $crate::imp::graph_impl!($scalar, $proc);
        }
        DummyGraphNew::graph_new()
    }};
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_graph_macro() {
        let g = graph!(f64, {
            let x = 1.0;
            let y = 3.0;
            let z = x + y + 2.0 * x * y;
        });
    }
}
