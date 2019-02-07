<!--
Copyright 2019 Fredrik Portström <https://portstrom.com>
This is free software distributed under the terms specified in
the file LICENSE at the top-level directory of this distribution.
-->

# Parse Wiki Text sandbox

Web application for interactively trying [Parse Wiki Text](https://github.com/portstrom/parse_wiki_text) and inspecting the output

![Parse Wiki Text](https://portstrom.com/parse_wiki_text.svg)

Wiki text can be entered. The application parses the wiki text every time it changes and displays the output of parsing. The default configuration of Parse Wiki Text is used. The entered text is persisted in the browser history and remains when the page reloads.

The application is a Web Assembly module with no Javascript code except the [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen) generated bindings and one line to load the module.

[Try it online](https://portstrom.com/parse_wiki_text_sandbox/)
