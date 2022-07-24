# ncurses-lite

This is a light (microscopic) reimagining of [`ncurses-rs`][ncurses-rs].

After trying to use `ncurses-rs` in [`rust-warrior`][rust-warrior], and
[getting][bot-0] [alerted][bot-1] by the Github dependabot, the idea for this
new library was born.

## Security Issues

### Mishandling of format strings

> An issue was discovered in the ncurses crate for Rust. There are format string
issues in `printw` functions because C format arguments are mishandled.

###  Buffer overflow and format vulnerabilities

> An issue was discovered in the ncurses crate for Rust. There are `instr` and
`mvwinstr` buffer overflows because interaction with C functions is mishandled.

The [`instr`][instr] function has this comment:

```rs
pub fn instr(s: &mut String) -> i32
{
  /* XXX: This is probably broken. */
  unsafe
  {
```

Reassuring, right?

The [`mvwinstr`][mvwinstr] function has the same comment:

```rs
pub fn mvwinstr(w: WINDOW, y: i32, x: i32, s: &mut String) -> i32
{
  /* XXX: This is probably broken. */
  unsafe
  {
```

These vulnerabilities have been reported in this [issue][issue], which links to:

* [CVE-2019-15548][CVE-2019-15548] (elaborated in [CWE-119][CWE-119])
* [CVE-2019-15547][CVE-2019-15547] (elaborated in [CWE-134][CWE-134])

There are some curses docs online, such as this page that documents the
[`innstr`][innstr] family of functions.

## What if

Given the complex nature of the vulnerable functions, and the difficulty in
verifying whether they are currently "broken" or whether a change would be
"broken" as well...

AND given that none of these functions are used in `rust-warrior`...

Another option is to create a library that exposes the necessary parts of
ncurses to Rust without including these vulnerabilities -- by simply leaving
those functions out.

## API

The following functions are implemented:

* `initscr`
* `endwin`
* `curs_set`
* `newwin`
* `delwin`
* `waddch`
* `waddstr`
* `wclear`
* `wrefresh`

[ncurses-rs]: https://crates.io/crates/ncurses
[rust-warrior]: https://github.com/miller-time/rust-warrior
[bot-0]: https://github.com/miller-time/rust-warrior/security/dependabot/4
[bot-1]: https://github.com/miller-time/rust-warrior/security/dependabot/5
[instr]: https://github.com/jeaye/ncurses-rs/blob/1e89a6212278d8219557bafa6734c9c40ce03912/src/lib.rs#L596
[mvwinstr]: https://github.com/jeaye/ncurses-rs/blob/1e89a6212278d8219557bafa6734c9c40ce03912/src/lib.rs#L994
[innstr]: https://pubs.opengroup.org/onlinepubs/7908799/xcurses/innstr.html
[issue]: https://github.com/jeaye/ncurses-rs/issues/209
[CVE-2019-15548]: https://nvd.nist.gov/vuln/detail/CVE-2019-15548
[CWE-119]: https://cwe.mitre.org/data/definitions/119.html
[CVE-2019-15547]: https://nvd.nist.gov/vuln/detail/CVE-2019-15547
[CWE-134]: https://cwe.mitre.org/data/definitions/134.html
