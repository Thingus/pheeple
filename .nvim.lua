vim.keymap.set(
	"n",
	"<leader>r",
	":FloatermNew --cwd=<root> --autoclose=0 cargo run -- --basemap data/uk/eng_scot_unitary_regions.geojson --outfolder uk_outputs/ --name_feature_id NAME<CR>",
	{ desc = "Run game, no autoclose" }
)
