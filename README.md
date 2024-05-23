# How to build frontend from Leptos

## Prerequisite

```sh
cargo install trunk
rustup target add wasm32-unknown-unknown
```

## Integrate with Tailwind CSS

- [First install node in WSL2](https://learn.microsoft.com/en-us/windows/dev-environment/javascript/nodejs-on-wsl)
- [Install Tailwind CSS](https://tailwindcss.com/docs/installation)
- [Setup files according Leptos example](https://book.leptos.dev/interlude_styling.html#tailwindcss-utility-first-css)

### Troubleshooting

- [sh-1-tailwindcss-permission-denied-error-on-build-process-tailwindcss-cli](https://stackoverflow.com/questions/72341549/sh-1-tailwindcss-permission-denied-error-on-build-process-tailwindcss-cli)
- [CMD.EXE was started with the above path as the current directory](https://stackoverflow.com/questions/51336147/how-to-remove-the-win10s-path-from-wsl)

## References

- [Hello World! Getting Set up for Leptos CSR Development](https://book.leptos.dev/getting_started/index.html)