
# SPEK file

* [ ] The file is named `SPEK.md`
* [ ] There is only one in a crate
* [ ] It is properly Markdown-formatted


# CLI

A `spek` binary is produced which enforces that `SPEK.md` stays in sync with the entire suite of integration tests for this crate.

The modes are as follows:
- `-b` `--backfill`: Backfill spec. The spec will be updated to better reflect the tests.
- `-g` `--generate`: Generate tests. If the spec contains items not reflected in the tests, new test stubs will be created.
- `-i` `--ignore`: Ignore. Used in conjuction with `-g`, if new stubs are created, they will have an `#[ignore]` attr added, so that the tests won't fail.


## spek -> tests

* {v2} Ellipses in immediately nested items...
  * ...get collapsed into the same item...
    * [ ] ...even when multiple layers deep
  * [ ] ...cannot be mixed with lines that don't have ellipses
* [ ] Items can be created in-test
* In "generate" mode (`-g`)...
  * [ ] ...top-level headings generate new test files
  * [ ] ...second-level headings do ???
  * [ ] ...normal text generates comments
  * [ ] ...asterisked bullet points generate test stubs
  * [ ] ...the `-i` option adds an `#[ignore]` attr to each generated test
  * [ ] ...existing tests are undisturbed
    * both with `-i`,
    * and without
* In "backfill" mode (`-b`)...
  * [ ] ...a suite of test stubs can generate a complete SPEK
* In "default" mode (no `-b` or `-g`)...
  * [ ] ...mismatches between items are warnings
* [ ] Diffs are used to more intelligently find discrepancies
* [ ] It's OK to have w̷̤̐ë̵͔́ỉ̸̬r̵̙͝d̷͎̈́ ̶͇̾u̴̧͋n̶͎̏i̷̬̎ć̷̣o̴̧͗d̷̹̿e̸͙͗ in the name
* [ ] It's OK to have really loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong spec items.
