use diesel::PgConnection;

pub fn verify_user_owns_character(user: String, cid: i32, conn: &mut PgConnection) -> bool {
    //TODO
    true
}
