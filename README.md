***note: this is incomplete software, and doesn't perform as advertised yet.***

# inmap: faster human filtering.

When sorting through data, often a decision must be made that can't be automated. For example, when sorting through URLs, do we want to keep a tab open, close it, or save it in our bookmarks?

Though the *decision* can't be automated, the various actions that we might elect to perform after that decision has been made can be automated. There's no general-purpose utility for this, until now.

`inmap` is designed to assist in building workflows that combine human decisions and automated actions.

## installation

tbd.

draft: If you have `cargo`, you can `cargo install inmap`.

Alternatively, you can download one of the binaries produced by CI under the `releases` page for this repository.

## configuring:

`inmap` takes a simple TOML config file.

```toml
preview = "firefox {}"
confirm = false
execute = [
	{ key = "j", command = "echo {} > saved.txt" },
	{ key = "k", command = "firefox {}" },
	{ key = "l", command = "" },
]
```

## running:

`cat urls.txt | inmap config.toml`

Every line of stdin will substitute the `{}` in the configured `preview` function, and then the whole line will be run.

`inmap` then pauses for input, waiting for a keypress that matches one of those configured in the `execute` array.

The user presses a key, at which point `inmap` substitutes the current line of stdin for the `{}` in the relevant `execute` command. The entire line is then run.

When `confirm` is true, `inmap` will print the substituted command after a key is pressed and require a `<CR>` before executing it.

## potential pitfalls:

At the moment, `inmap` is not doing anything smart with substitution or escaping. It is behaving entirely naively. As a result, preprocessing your input to be shell-valid is recommended.

Any command in your $PATH can be configured in the preview and execution commands, so a script can be used to handle files of multiple types.
