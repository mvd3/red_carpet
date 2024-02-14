# Red carpet

The specification has been implemented.

## How to run it

1. Clone the repository
2. Open 2 terminal windows (or tabs), with one navigate to `api` folder, with the other to `test-api` folder.
3. Firstly execute in `api` folder `cargo run`, and then after the server starts, run the same command in `test-api` folder.

### Some more points

I wanted to write a functionallity where every time the applications closes, the database is saved to a file on the disk, and every time the application loads, it would seek that file and load from it the data (if correct). But because the package (and time) constraints, I wasn't able to implement that. For the same reason, there might be other things that I overlooked.