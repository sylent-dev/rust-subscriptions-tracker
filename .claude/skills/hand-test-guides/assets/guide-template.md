<!--
  The template of a guide of the tests by hand (hand-test-guides skill). Copy it to
  docs/tests/<system>/<feature>.md, write it in the language of the testers, list it in
  docs/tests/README.md, and remove this comment. It stays blank: the results come back in the
  conversation or the pull request.
-->
# <Feature>, on <system>: the tests (<step>)

<Why, in two or three sentences: what changed, what the tests check.> Run the tests in a row. A ✅ is enough when it works; detail only what goes wrong.

## Before

1. Close <application> if it runs.
2. From a terminal of the system (not the one of VS Code installed as a Flatpak), in the folder of the project, on the branch of the step: `cargo build --release`.
3. The log is in `<path of the log>`.

## The tests

1. **<Name>**: <the action>.
   ✅ <The expected result: the exact text shown, the exact log line.>

2. **<Name>**: <the action, up to the end of the list>.
   ✅ <The expected result.>

3. **Your own scenarios**: whatever else you want to try.

## Results

| Test | Result | Remark |
|---|---|---|
| 1. <Name> | | |
| 2. <Name> | | |
| 3. Your own scenarios | | |
