@ECHO OFF

REM If number of threads is greater than one, errors occur because of shared fs state
cargo test -- --test-threads=1
