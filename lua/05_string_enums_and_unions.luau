--!strict
local log = require("std:log")
local draw = require("std:draw")

local level: std_log_Level = "info"
assert(log.write(level, "up") == "[info] up")

local figures: { std_draw_Figure } = { { kind = "circle", r = 1 }, { kind = "rect", w = 2, h = 3 } }
local rects = 0
local total = 0
for _, figure in figures do
	if figure.kind == "rect" then
		rects += figure.w * figure.h -- narrowed on `kind`: only a rect has `w` and `h`
	end
	total += draw.area(figure)
end
assert(rects == 6)
assert(math.abs(total - (math.pi + 6)) < 1e-9)
