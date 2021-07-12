# ``gtk3``
Test for gtk3 running on windows.

## Installing
Follow https://www.gtk.org/docs/installations/windows  

If that doesn't work, maybe the WSL will. Windows dev support for gtk in general is very limited unfortunately.

``Main.ui`` was made with glade. Some don't recommend it because it's super outdated and may produce bad output, but I'm using it in this example project.

## Building
You need to use the x86_64 gnu target.

```
cargo build --target=x86_64-pc-windows-gnu
```

## Debugging
Use vscode & .vscode/launch.json.
