use crate::{Diagnostic, DiagnosticCode, DiagnosticsContext};

// Diagnostic: unresolved-ident
//
// This diagnostic is triggered if an expr-position ident is invalid.
pub(crate) fn unresolved_ident(
    ctx: &DiagnosticsContext<'_>,
    d: &hir::UnresolvedIdent,
) -> Diagnostic {
    Diagnostic::new_with_syntax_node_ptr(
        ctx,
        DiagnosticCode::RustcHardError("E0425"),
        "no such value in this scope",
        d.expr.map(Into::into),
    )
    .experimental()
}

#[cfg(test)]
mod tests {
    use crate::tests::check_diagnostics;

    #[test]
    fn missing() {
        check_diagnostics(
            r#"
fn main() {
    let _ = x;
          //^ error: no such value in this scope
}
"#,
        );
    }

    #[test]
    fn present() {
        check_diagnostics(
            r#"
fn main() {
    let x = 5;
    let _ = x;
}
"#,
        );
    }
}
