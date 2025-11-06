vim.keymap.set(
	"n",
	"<leader>r",
	":FloatermNew --cwd=<root> --autoclose=0 cargo run -- --basemap data/haiti_admin2.geojson<CR>",
	{ desc = "Run game, no autoclose" }
)
