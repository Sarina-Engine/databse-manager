table! {
    categories (vid) {
        vid -> Int4,
        id -> Int4,
        title_fa -> Text,
        code -> Text,
        parent_cat -> Int4,
        done -> Bool,
    }
}

table! {
    comments (vid) {
        vid -> Int4,
        id -> Int4,
        product_id -> Int4,
        body -> Text,
        rate -> Float8,
    }
}

table! {
    features (vid) {
        vid -> Int4,
        name -> Text,
        value -> Float8,
        product_id -> Int4,
    }
}

table! {
    products (vid) {
        vid -> Int4,
        id -> Int4,
        cat_id -> Int4,
        title_fa -> Text,
        rate -> Float8,
        views -> Int4,
        done -> Bool,
    }
}

allow_tables_to_appear_in_same_query!(
    categories,
    comments,
    features,
    products,
);
