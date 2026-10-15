use backend::auth::password::hash_password;

fn main() {
    let mut args = std::env::args().skip(1);
    let email = args
        .next()
        .expect("usage: seed_user <email> <password> [role]");
    let password = args
        .next()
        .expect("usage: seed_user <email> <password> [role]");
    let role = args.next().unwrap_or_else(|| "employee".to_string());
    let hash = hash_password(&password).expect("password hashing failed");

    println!(
        "INSERT INTO users (first_name, last_name, email, password_hash, role) VALUES ('Test', 'User', '{email}', '{hash}', '{role}');"
    );
}

// a simple script to generate a SQL statement to insert a user into the database
// cargo run -q -p backend --example seed_user -- test@mybrigitte.fr MotDePasse123 | docker compose exec -T postgres sh -c 'psql -U "$POSTGRES_USER" -d "$POSTGRES_DB"'
