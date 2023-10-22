// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        first_name -> Text,
        last_name -> Text,
        email -> Text,
        created_at -> Timestamp,
        github -> Nullable<Text>,
        website -> Nullable<Text>,
        age -> Nullable<Int4>,
        age_weight -> Nullable<Int4>,
        location -> Nullable<Text>,
        location_weight -> Nullable<Int4>,
        employer -> Nullable<Text>,
        employer_weight -> Nullable<Int4>,
        reason -> Nullable<Text>,
        project_interests -> Nullable<Text>,
        project_interests_weight -> Nullable<Int4>,
        personality_interests -> Nullable<Text>,
        personality_interests_weight -> Nullable<Int4>,
        skills -> Nullable<Text>,
        skills_weight -> Nullable<Int4>,
        right_swipes -> Nullable<Array<Int4>>,
        left_swipes -> Nullable<Array<Int4>>,
        incoming_right_swipes -> Nullable<Array<Int4>>,
        incoming_left_swipes -> Nullable<Array<Int4>>,
        matches -> Nullable<Array<Int4>>,
    }
}
