@ECHO OFF

REM If number of threads is greater than one, errors occur because of sharing of fs.
cargo test -- --test-threads=1
