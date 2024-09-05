# Contributing to Markium

## Formatting

Format the code with `cargo +nightly fmt` before comitting.

## CSS and JS

Make sure any CSS/JS is supported within atleast last 4 versions of common
browsers. The main browser to test against is Firefox. I encourage you to test
against other browsers too, however I myself only test against Firefox.

## Commit messages

When committing:

- Use infinitive verbs. (i.e. `Add` instead of `Added`)
- Keep the commit message short but descriptive.
- Describe only what was done, and what was it done to.
- Articles and determiners can be omitted.

These are good commit messages:

- `Fix link styling`
- `Add button styling`
- `Change code highlighting`

These are bad commit messages:

- `Made the link styling better`
- `added button`
- `highlighting change`

For many non-related to eachother changes, commits like these are allowed:

- `Update README.md`
- `Bunch of small changes`

## Pull requests

When creating a pull request:

- Clearly describe why a change was made, and what it does
- It's better to over-explain than to under-explain

## Issues

Because issues can be created by anyone, with or without technical knowledge,
they can be written anyhow. As long as they describe the issue good enough for
it to be understood, and are clear about what the problem is, it's fine.
