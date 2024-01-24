# csvq

## Up and running

```
$ cargo build --release
$ ./target/release/csvq --help
```

## Generating data

Requires `pandas` and `faker`.

```
python scripts/generate_data.py 50
```

## Examples

The examples below assume that `csvq` is in the $PATH.

### View the first rows of the file

```
csvq view <FILE_NAME>.csv
```

### Filter rows

```
csvq filter --column score --equals 30 <FILE_NAME>.csv
```
