_almost all of this was taken from [this vid](https://www.youtube.com/watch?v=70_9IIsQfjs)_

this is just me playing around with rust workspaces

- to add a command, do

```bash
cargo new <my-command>
```

- to add a lib do

```bash
cargo new shared/<my-lib> --lib
```

inspect every package's Cargos.toml to see how to config dependencies

- to release do

```bash
cargo build -r
```
