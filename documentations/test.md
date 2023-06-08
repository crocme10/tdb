
## Scenarios

A test will typically perform the following:
1. Initialization
2. Data insertion
3. Checks
4. Cleanup

For example:
1. Start a transaction.
2. Using that transaction, insert a record in a table.
3. Using that transaction Check the number of records in the table is one.
4. Rollback the transaction.

The cucumber scenario uses a 'World', a global state, which is available at every step of
the test.

The cucumber scenario can have before and after hooks to perform tasks and prepare the context.

Using a before hook:

Create a new transaction, and store it in the world.

Not necessary to have a 'Given' step.

When the user subscribes:

Take the transaction, insert the data, and return the transaction.

Then the user can be found in the database:

Take the transaction, run the query, check the data, and return the transaction.

After hook:

rollback the transaction.
