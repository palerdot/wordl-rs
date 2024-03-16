# CHANGELOG

### v0.3.0
- Fix: White color text. Previously terminal theme's text color is used for text color.

### v0.2.0
- Fix: Handle edge case in guess letter check when the match already happened initially. Fixes - https://github.com/palerdot/wordl-rs/issues/2

### v0.1.6
- Fix: Handle edge cases in guess letter status check

### v0.1.5
- Fix: Handle panic when terminal does not have minimum height for keyboard hints.

### v0.1.4
- Fix: keyboard hints when multiple letters are present in the guess
- Show version string in the UI
- moved wordle data files to directory

### `v0.1.3`
- Fix: Inlining wordle files with rust crate.

### `v0.1.2`
- Removed changelog action from github workflow action

### `v0.1.1`
- Added `description` to `Cargo.toml`.

### `v0.1.0`
- `wordl-rs`: First release version
