vim.keymap.set(
	"n",
	"<leader>r",
	":FloatermNew --cwd=<root> --autoclose=0 cargo run -- --basemap data/uk/eng_scot_unitary_regions.geojson --outfolder uk_outputs/ --id_feature_name NAME<CR>",
	{ desc = "Run game, no autoclose" }
)
