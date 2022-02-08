`cargo run` to just run in debug / development mode

`cargo build --release` will build an .exe file in `./target/release`

## CLI Args

You can pass a port:

`cargo run 3000` or `./todos-simple-backend.exe 3000`

Also you can disable the addition of initial test data:

`cargo run 3000 true` or `./todos-simple-backend.exe 3000 true`
