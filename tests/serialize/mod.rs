mod basic;
mod enum_;
mod list;
mod map;
mod option;
mod scalar;
mod struct_;
mod vec_struct;

/// Assert that the TOML used to serialize a value can be used to deserialize the value too.
#[macro_export]
macro_rules! assert_serialize {
    ($type:ty, $val:expr $(,)?) => {{
        use eyre::WrapErr as _;

        let value = $val;
        let serialized = facet_toml::to_string(&value).unwrap();
        let deserialized: $type = facet_toml::from_str(&serialized)
            // Unfortunately we can't use the error as-is because it has a lifetime bound
            .map_err(|err| eyre::eyre!("{err}"))
            .wrap_err_with(|| format!("{value:?}"))
            .wrap_err(serialized)
            .unwrap();

        assert_eq!(deserialized, value);
    }};
}
