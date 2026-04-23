# nt

`nt` is a small command-line tool that rewrites names in a text file using one or more translation files.

It:
- reads an input file,
- loads `old -> new` mappings from translation files,
- applies replacements using whole-word matching,
- writes the result to an output file.

## Requirements

- Rust (edition 2021)
- Cargo

## Build

```bash
cargo build
```

## Run

```bash
cargo run -- <input_file> <translation_file> [<translation_file> ...] [-o <output_file>]
```

### Arguments

- `input_file` (required): File to transform.
- `translation_file` (required, one or more): Files containing `old -> new` mappings.
- `-o, --output` (optional): Output file path.

If `--output` is not provided, output is written beside the input file as:

- `translated_<input_filename>`

## Translation file format

Each non-empty mapping line must be:

```text
old_name -> new_name
```

Comments are supported:

- Full-line comments: lines starting with `#`
- Inline comments: text after `#` on a mapping line is ignored

Example:

```text
# Rename domain terms
Customer -> Client
OrderId -> PurchaseId # inline comment
```

## Example

Input (`sample.txt`):

```text
Customer created OrderId.
Customers are grouped by Customer.
```

Translation (`map.txt`):

```text
Customer -> Client
OrderId -> PurchaseId
```

Run:

```bash
cargo run -- sample.txt map.txt
```

Output (`translated_sample.txt`):

```text
Client created PurchaseId.
Customers are grouped by Client.
```

(`Customers` is unchanged because replacements are whole-word matches.)
