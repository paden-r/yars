use mysql::OptsBuilder;

pub fn get_opts() -> OptsBuilder {
    let db_user = std::env::var("DB_USER").unwrap();
    let db_pass = std::env::var("DB_PASS").unwrap();
    let db_port_string = std::env::var("DB_PORT").unwrap();
    let db_port: u16 = db_port_string.parse().unwrap();

    let mut builder = OptsBuilder::new();
    builder = builder.user(Some(db_user))
        .pass(Some(db_pass))
        .db_name(Some("yars"))
        .tcp_port(db_port)
        .ip_or_hostname(Some("127.0.0.1"));

    builder
}
