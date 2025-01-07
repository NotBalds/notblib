#[test]
fn status_test() {
    use crate::status;

    status::process("Test process status");
    status::success("Test success status");
    status::warning("Test warning status");
    status::error("Test error status");
    status::fatal("Test fatal status");
}
