# CoW protocol solver template

### Run Solver Server

```shell
cargo run
```

### Feed an Auction Instance to the Solver

```shell
curl -X POST "http://127.0.0.1:8000/solve" \
  -H  "accept: application/json" \
  -H  "Content-Type: application/json" \
  --data "@data/small_example.json"
```
