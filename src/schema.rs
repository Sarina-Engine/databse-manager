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
        done -> Bool,
    }
}

table! {
    features (vid) {
        vid -> Int4,
        name -> Text,
        value -> Text,
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

table! {
    scores (vid) {
        vid -> Int4,
        product_id -> Int4,
        emotion -> Float8,
        satisfaction -> Float8,
        recommended -> Float8,
        feeling -> Float8,
    }
}

table! {
    sentiments (vid) {
        vid -> Int4,
        comment_id -> Int4,
        recommended -> Float8,
        not_recommended -> Float8,
        no_idea -> Float8,
        sad -> Float8,
        happy -> Float8,
        positive -> Float8,
        negative -> Float8,
        furious -> Float8,
        angry -> Float8,
        neutral -> Float8,
        happy2 -> Float8,
        delighted -> Float8,
        done -> Bool,
    }
}

allow_tables_to_appear_in_same_query!(
    categories,
    comments,
    features,
    products,
    scores,
    sentiments,
);
