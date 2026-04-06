```bash 

# Run Project 
cargo clean && cargo build && cargo run

# Run Project 
rm -rf Cargo.lock target && cargo clean && cargo build && cargo run


# Creating Folder Structure
mkdir controllers routes repository models state controllers/user repository/user routes/user db
touch controllers/user/user_controller.rs routes/user/user_routes.rs models/user.rs state/user.rs db/db.rs main.rs
```

# Installation & Set up
```bash

cargo init
cargo install diesel_cli --no-default-features --features postgres

cargo install cargo-watch
cargo watch --version

cargo watch -w src -x run
rm -rf Cargo.lock target 


git add . && git commit -m "Feb 10: token storing done" && git push
 
http://localhost:5019

```


# Running Migrations
```bash
# 1. Run for setup
diesel setup  
diesel print-schema > src/schema.rs 


# 2. Run these two only if there are changes in schema
diesel migration generate --diff-schema mi_
diesel migration run

```