# Toy transaction engine

Run the program:
```
cargo run -- transactions.csv > accounts.csv
```

Run the tests:
```
cargo test
```

## Notes:
- If the input contains an format error I decided to abort the program, instead of ignoring the faulty line.
- I assumed only deposits can be disputed.
- Disputes, resolves and chargebacks with client different from the orginal transaction's client are ignored.
- There's a corner case for which clients can go into negative balance.
- All tests are in the `tests/` directory.
- Locked accounts can't do any further deposit or withdrawal after becoming frozen.
- Test cases aren't exhaustive due to time constraits.
- Again, due to time constraits, I'm not checking some invariants (for example transaction Id uniqueness).
- Code has been tested on ARM macbook and on intel windows.