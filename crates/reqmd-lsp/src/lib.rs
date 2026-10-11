//! Language server of reqmd (FR-200 ff.), built on lsp-server (ADR-0003).

pub mod convert;
pub mod features;
mod server;

pub use server::{Error, run};

/// FR-200: serves the Language Server Protocol over stdin and stdout.
pub fn run_stdio() -> Result<(), Error> {
    let (conn, io) = lsp_server::Connection::stdio();
    run(&conn)?;
    drop(conn);
    io.join()?;
    Ok(())
}
