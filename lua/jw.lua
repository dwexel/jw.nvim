local internal = require "jw_nvim"


local line
local column
local indices


vim.api.nvim_create_autocmd({"CursorMoved"}, {
    callback = function(ev)
	local cursor = vim.api.nvim_win_get_cursor(0)
	column = cursor[2]
	

	if line ~= cursor[1] then
	    line = cursor[1]
	    
	    local t = vim.api.nvim_buf_get_lines(0, line-1, line, false)

	    indices = internal.ssend(t[1])

	    -- vim.print(i)
	end
	-- vim.print(c)
    end
})

vim.keymap.set("n", "EW", function()
    if indices and column then

	local len = #indices

	if len == 0 then

	    if pcall(vim.api.nvim_win_set_cursor, 0, {line+1, 0}) then
	    end
	end

	for ii = 1, len do
	    local i = indices[ii]

	    if column < i then

		-- if index is the last one, go to next line
		if ii == len then
		    -- not the best...
		    if pcall(vim.api.nvim_win_set_cursor, 0, {line+1, 0}) then
		    end
		    
		    return
		end
		
		vim.api.nvim_win_set_cursor(0, {line, i})
		vim.print(column, indices)

		break
	    end
	end
    end
end)

vim.keymap.set("n", "EB", function()
    if indices and column then
	local len = #indices

	if len == 0 or column < indices[1] then
	    print(pcall(vim.api.nvim_win_set_cursor, 0, {line-1, vim.v.maxcol}))
	    return
	end

	-- vim.print(indices)


	for ii = len, 1, -1 do
	    local i = indices[ii]
	    if i < column then
		vim.api.nvim_win_set_cursor(0, {line, i})
		column = i
		return
		-- break
	    end
	end

	vim.api.nvim_win_set_cursor(0, {line, 0})
	column = 0
    end
end)

return { "hello" }
