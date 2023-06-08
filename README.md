# Test Database
> An exploration in sqlx::Executor to enable proper testing with transactions.

sqlx is a good choice for a rust project with a database backend. In order to connect with the
backend, you can use either a connection, or a pool of connections. When dealing with tests in
your project, you have to make sure that your tests are idempotent, in other words that if you
run your test once and it passes, then if you run it again it should pass. But if you don't pay
attention, you can run into issues, like in the following scenario: Imagine a test that inserts
data in the database, and then checks that the data is there. Your test inserts the data once,
the test checks the data is there, all is well. But when you run the test again, the test fails
because when you try to insert the data again, this is a case of duplicate key.

There are three main solutions for this problem:

1. Delete all the data you inserted.
2. Drop the database and recreate it after each test.
3. Wrap each test database side effects in a transaction, and rollback the transaction at the
   end of the test.

This project is about the 3rd option.

We need to make some of the code generic on the database `Executor`, so that in production it
can be a connection to the database, and during tests it is a transaction, which can be rolled
back.

This project, and some of its code, is motivated, and inspired by the book
[Zero to Production in Rust](https://zero2prod.com)

[Details about test setup](./documentations/test.md)

## Development setup

This is a rust project, you need to install rust. The database (Postgres) is in a docker container,
so you also need docker to be installed.

It relies on a cargo extension `sqlx-cli` to perform database migrations. This is just to make
life easy, but is not a requirement for the project to work.

The single configuration point is in the `.env` file, and is the `DATABASE_URL` connection string.

When compiling the project, sqlx will try to connect to the database to run some compile time checks.
It relies on the `DATABASE_URL` variable beeing set. For a bash shell, you just use

```sh
source .env
```

For fish users (count me in), its easy to source .env files. See eg
[this solution](https://github.com/fish-shell/fish-shell/issues/6901)

Now that DATABASE_URL is properly set for all shells, you need to start the database. There is a
script, which retrieves your configuration from the DATABASE_URL (eg Database name, user, password,
port) and starts a docker container with the database:

```sh
./scripts/init_db.sh
```

Now you can run your tests:

```sh
cargo test
```

## Release History

* 0.0.1
    * Work in progress

## Meta

Matthieu Paindavoine – YourEmail@example.com

Distributed under the MIT license. See ``LICENSE`` for more information.

[https://github.com/crocme10](https://github.com/crocme10/)

## Contributing

1. Fork it (<https://github.com/crocme10/tdb/fork>)
2. Create your feature branch (`git checkout -b feature/fooBar`)
3. Commit your changes (`git commit -am 'Add some fooBar'`)
4. Push to the branch (`git push origin feature/fooBar`)
5. Create a new Pull Request
