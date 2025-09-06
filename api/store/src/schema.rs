// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "website_status"))]
    pub struct WebsiteStatus;
}

diesel::table! {
    region (id) {
        id -> Text,
        name -> Text,
    }
}

diesel::table! {
    user (id) {
        id -> Text,
        username -> Text,
        password -> Text,
    }
}

diesel::table! {
    website (id) {
        id -> Text,
        url -> Text,
        timeAdded -> Timestamp,
        userId -> Nullable<Text>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::WebsiteStatus;

    website_tick (id) {
        id -> Text,
        response_time_ms -> Int4,
        status -> WebsiteStatus,
        website_Id -> Text,
        region_Id -> Text,
        createdAt -> Timestamp,
    }
}

diesel::joinable!(website -> user (userId));
diesel::joinable!(website_tick -> region (region_Id));
diesel::joinable!(website_tick -> website (website_Id));

diesel::allow_tables_to_appear_in_same_query!(
    region,
    user,
    website,
    website_tick,
);
