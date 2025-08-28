<table>
<tr>
<td>
<img src="https://github.com/TilePad/tilepad-desktop/raw/main/assets/tilepad-logo.svg" width="150px">
</td>
</tr>
</table>

# TilePad + Rust Plugin

This template provides a minimal template for building a **TilePad** plugin using the Rust language

## Getting started

For help with getting started check out [Getting Started](https://tilepad.pages.dev/plugins/getting-started/)

## Included in this template

- Basic counter example project
    - Tile actions for increasing and decreasing a shared counter
    - Inspector UI for choosing the "amount" of increasing / decreasing per tile
- CI for building and bundling releases
    - Building for windows, linux, and macos targets

## Parts of this project

Below are some notable parts of the project

- **src**
    - **action.rs** JSON structure for each action and its properties
    - **main.rs** Entrypoint for the plugin
    - **messages.rs** JSON structure for messages between the inspector and the plugin
    - **plugin.rs** Plugin structure and event handlers
    - **state.rs** State logic and data used by the plugin
- **.tilepadPlugin**
    - **manifest.json** Plugin manifest file
    - **ui** HTML, CSS, and JS files for inspectors and displays
    - **images** Icons and images used by the plugin
- **.github**
    - **workflows**
        - **build-and-bundle.yml** Github action for building and bundling releases

## Creating a release

Create a new release tag:

```sh
git tag "0.1.0"
```

Push the release tag to github:

```sh
git push --tags
```

Your project will be built and bundled in CI on github, after this is complete you will find a draft
release under Releases containing the .tilepadPlugin file
