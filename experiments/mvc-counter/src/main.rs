use std::io;

fn main() -> io::Result<()> {
    mvc_counter::run(io::stdin().lock(), io::stdout().lock())
}
