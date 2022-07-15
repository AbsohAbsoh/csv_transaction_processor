## CSV Transaction Processor

<br>

Run a with a test input like
```shell
cargo run -- __tests__/sample.csv > output.csv
```

More info on test cases in `./__tests__/readme.md`

### Caveats

- In a distributed environment, you could put the AccountActor behind a network interface or two and pipe commands in that way.
- I didn't use Serde because it gucks up the structs when you need a high level of validation / mapping. But it would have been a perfectly valid approach.
- In the real world the account actor probably wouldn't be responsible for something as specific as "write clients to stdout".
- A lot of the error branching is panicking or logging; those would be ideally be handled. 
- Tokio w/ `features = ["full"]` is probably overkill; I find it convenient when using an actor.
- We could also parallelize serializing the clients before writing to stdout.
