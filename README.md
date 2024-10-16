# Investigating an issue with Postgres

When flooding Postgres with serializable transactions, some assertions performed by Postgres
are not successful ([**Postgres assertions need to be enabled during build-time**](#how-to-build-postgres-with-assertions-enabled)):

```
TRAP: failed Assert("TransactionIdIsValid(tailXid)"), File: "predicate.c", Line: 885, PID: 1350
0   postgres                            0x0000000103926c6c ExceptionalCondition + 236
1   postgres                            0x00000001036aab3c SerialAdd + 200
2   postgres                            0x00000001036aa764 SummarizeOldestCommittedSxact + 252
3   postgres                            0x00000001036a3654 GetSerializableTransactionSnapshotInt + 412
4   postgres                            0x00000001036a3210 GetSerializableTransactionSnapshot + 292
5   postgres                            0x000000010399dd88 GetTransactionSnapshot + 404
6   postgres                            0x00000001036c69bc exec_bind_message + 1952
7   postgres                            0x00000001036c4954 PostgresMain + 2900
8   postgres                            0x00000001036bbf18 BackendInitialize + 0
9   postgres                            0x0000000103599e64 postmaster_child_launch + 324
10  postgres                            0x00000001035a1c48 BackendStartup + 468
11  postgres                            0x000000010359e97c ServerLoop + 432
12  postgres                            0x000000010359d848 PostmasterMain + 6720
13  postgres                            0x0000000103422624 startup_hacks + 0
14  dyld                                0x00000001833d3154 start + 2476
```

How to run:

```shell
PG_URL=postgres://localhost/dbame cargo run --release
```

Sometimes, more than one may be required.

# Request

Can you please verify if this is true and open an issue with your results?

So far, I was only able to reproduce this on macOS, but quite consistently.

# How to build Postgres with assertions enabled?

This is how it has to be configured:

```shell
./configure --enable-cassert
```
