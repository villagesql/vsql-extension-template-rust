# Security Policy

## Reporting a vulnerability

If you believe you have found a security issue in this template or in the code it generates, please **do not** open a public issue.

Instead, report it privately through GitHub's [private vulnerability reporting](https://docs.github.com/en/code-security/security-advisories/guidance-on-reporting-and-writing-information-about-vulnerabilities/privately-reporting-a-security-vulnerability) on this repository, or email the maintainers at `security@villagesql.com`.

Please include:

- A description of the issue and its impact
- Steps to reproduce (a minimal extension or `cargo generate` invocation, if applicable)
- Versions of `cargo-generate`, `rustc`, and the `villagesql` crate involved

We will acknowledge receipt within 5 business days and aim to provide a remediation timeline within 10 business days.

## Scope

This policy covers the template itself and the boilerplate it produces. Vulnerabilities in the [`villagesql` SDK](https://github.com/villagesql/vsql-rust-sdk) or [VillageSQL server](https://github.com/villagesql/villagesql-server) should be reported to those repositories.
