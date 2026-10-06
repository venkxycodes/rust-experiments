use std::io::{self, Cursor};

fn session(commands: &str) -> String {
    let mut output = Vec::new();
    mvc_counter::run(Cursor::new(commands), &mut output).unwrap();
    String::from_utf8(output).unwrap()
}

#[test]
fn commands_update_and_reset_the_counter() {
    let output = session("inc\ninc\ndec\nshow\nreset\nquit\ninc\n");
    assert_eq!(
        output,
        concat!(
            "Commands: inc | dec | reset | show | quit\n",
            "Count: 0\nCount: 1\nCount: 2\nCount: 1\nCount: 1\nCount: 0\n",
        )
    );
}

#[test]
fn invalid_commands_and_model_errors_allow_the_session_to_continue() {
    let output = session("dec\nunknown\ninc\n");
    assert!(output.contains("Error: counter cannot go below zero\n"));
    assert!(output.contains("Error: unknown command;"));
    assert!(output.ends_with("Count: 1\n"));
}

#[test]
fn empty_input_exits_cleanly_at_eof() {
    assert!(session("").ends_with("Count: 0\n"));
}

#[test]
fn output_errors_are_returned_to_the_caller() {
    struct BrokenOutput;
    impl io::Write for BrokenOutput {
        fn write(&mut self, _: &[u8]) -> io::Result<usize> {
            Err(io::Error::new(io::ErrorKind::BrokenPipe, "output closed"))
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
    let error = mvc_counter::run(Cursor::new("inc\n"), BrokenOutput).unwrap_err();
    assert_eq!(error.kind(), io::ErrorKind::BrokenPipe);
}
