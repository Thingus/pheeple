vim.keymap.set(
	"n",
	"<leader>r",
	":FloatermNew --cwd=<root> --autoclose=0 cargo run<CR>",
	{ desc = "Run game, no autoclose" }
)
