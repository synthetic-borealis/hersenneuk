# Changelog

## v0.2.2
- 20/10/2022 - Added support for C/C++ style multi-line comments.

## v0.2.1

- 13/10/2022 - `run_*`, `get_char` & `put_char` take anything that implements `Read` or `Write` traits.

## v0.2.0

- 12/10/2022 - Added `run_with_dynamic_block`.
- 12/10/2022 - Renamed `run` to `run_with_fixed_block`.
- 12/10/2022 - `run` takes `block_size` as an argument instead of using a constant value.
- 12/10/2022 - `run`, `get_char` & `put_char` take read/write buffer instead of using stdout/stdin directly.
