-- set `vim.o["exrc"] = true` and `:trust` this file in order to enable this configuration
vim.lsp.config("rust_analyzer", {
  settings = {
    ["rust-analyzer"] = {
      cargo = {
        target = "thumbv6m-none-eabi",
        allTargets = false,
      },
    },
  },
})
