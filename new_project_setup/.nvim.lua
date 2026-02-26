vim.api.nvim_create_autocmd("BufWritePost", {
    pattern = "*.rs",
    callback = function()
        -- Runs 'make' silently and populates the quickfix list
        vim.cmd("silent make!")
    end,
})
