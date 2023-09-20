# Rusty Journal

- [Rusty Journal](#rusty-journal)
- [Project Structure](#project-structure)
- [Installation and Set-up](#installation-and-set-up)
  - [Prerequisites:](#prerequisites)
  - [Important](#important)
  - [Compiling](#compiling)
- [Troubleshooting](#troubleshooting)
- [Improvements](#improvements)

---

The project enables data exchange between Bybit and Notion API. The underlying
motives revolve around diligent data gathering and data-driven trading. The
repository is an upgrade to a previous release coded in
[Python](https://github.com/pwislowski/seamless_journal_public). The app
connects to a Bybit API and fetches derivates positions' details and
succeedinlgy uploads them to the user-specified database residing in Notion. The
implementation requires a manual input of fields defined in
`bybit/src/structs/NewPageObject.rs`.

# Project Structure
The repository is structered as follows:
- `bybit/` - Bybit web client, supporting parsers and structs; certain structs
have implementations enabling accessible API.
- `notion/` - Notion web client, supporting parsers and structs; comprises
abstraction for Notion data types that Bybit uses to build compatible JSON
queries.
- `logger/` - logger for caching sent queries over to Notion in order to avoid
duplication in the database.
- `rusty_journal/` - the main source code directory binding the above together.
- `README.md` - project's overview.
- `Cargo.toml` - project's metadata; the root-level file specifies workspaces.
Each direcotry contains its own `Cargo.toml` for managing dependencies.
- `script.sh` - shell-wrapper to easily access a compiled executable.

# Installation and Set-up
## Prerequisites:

```
Rust 1.68.0 <=
```

## Important
The repo does not allow for database feature flexibility, i.e. features have been hard-coded. Prior to compiling, ensure your database conforms to the one set out below:

```json
{
    // feature: <notion data type>
    "Trade Type": text,
    "MS": multi-select,
    "Exit Price": number,
    "Entry Price": number,
    "Win": bool,
    "Side": select,
    "Exchange": text,
    "Asset": select,
    "Entry Model": text,
    "Confusion Matrix": multi-select,
    "Used Orderflow": bool,
    "Confluences": multi-select,
    "Improvements": multi-select,
    "Market": text,
    "Entry Date": date,
    "Exit Date": date,
    "isOpen": bool,
    "Stop-loss": number
}
```

Also, this repo tracks the `.env` file so make sure you remove it from history
or start a new repo. Once completed, populate files' properties.
You will need the following:
- Bybit API & Secret
- Notion API
- Notion Target Database ID.

## Compiling

Finally, execute the `cargo run` CLI command in the root folder to compile the
project.

# Troubleshooting
The source code aims to handle errors while being as descriptive as possible;
nevertheless, occasional bugs may occur should API response structure change.
However, if the APIs stay unchanged, the repo will work as expected.

# Improvements
Revising the source code, I noticed that:
-  struct de- and serialization could be better implemented. The manual struct
building seems verbose and could be superseded with serde's `deserialize`
function. 
- the main file logic could be improved to be less verbose, e.g. match statement
of LogFile read
