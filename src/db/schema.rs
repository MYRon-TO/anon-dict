// @generated automatically by Diesel CLI.

diesel::table! {
    stardict (id) {
        id -> Integer,
        word -> Text,
        sw -> Text,
        phonetic -> Nullable<Text>,
        definition -> Nullable<Text>,
        translation -> Nullable<Text>,
        pos -> Nullable<Text>,
        collins -> Nullable<Integer>,
        oxford -> Nullable<Integer>,
        tag -> Nullable<Text>,
        bnc -> Nullable<Integer>,
        frq -> Nullable<Integer>,
        exchange -> Nullable<Text>,
        detail -> Nullable<Text>,
        audio -> Nullable<Text>,
    }
}
