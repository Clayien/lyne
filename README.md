# Install
```sh
cargo install lyne --version 0.0.1-alpha
```

# bash/ zsh
```sh
ly() { eval "$(lyne)"; }
```

# pwsh
```ps1
function ly {
  $output = lyne
  Invoke-Expression $output
}
```
