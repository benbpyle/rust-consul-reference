local dap = require("dap")

dap.adapters.lldb = {
	type = "executable",
	command = "/usr/bin/lldb", -- adjust as needed
	name = "lldb",
}

dap.configurations.rust = {
	{
		name = "service-c",
		type = "lldb",
		request = "launch",
		program = function()
			return vim.fn.getcwd() .. "/target/debug/service-c"
		end,
		cwd = "${workspaceFolder}",
		stopOnEntry = false,
		env = {
			"BIND_ADDRESS=0.0.0.0:3000",
		},
	},
}
