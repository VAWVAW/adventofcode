const std = @import("std");

const re = @cImport(@cInclude("regex.h"));

extern fn regcomp(noalias __preg: *anyopaque, noalias __pattern: [*c]const u8, __cflags: c_int) c_int;
extern fn regexec(noalias __preg: *const anyopaque, noalias __String: [*c]const u8, __nmatch: usize, noalias __pmatch: [*c]re.regmatch_t, __eflags: c_int) c_int;
extern fn regfree(noalias __preg: *anyopaque) void;

// hardcode values for x86_64 linux
const REGEX_T_SIZEOF = 64;
const REGEX_T_ALIGNOF = 8;

const RegexError = error{ InvalidExpression, MatchError };

const result_type = u64;

fn task_1(allocator: std.mem.Allocator, file_name: []const u8) !?result_type {
    // open input
    var file = try std.fs.cwd().openFile(file_name, .{});
    defer file.close();
    var input = std.io.bufferedReader(file.reader());

    // read data
    var result: result_type = 0;

    const regex_slice = try allocator.alignedAlloc(u8, REGEX_T_ALIGNOF, REGEX_T_SIZEOF);
    defer allocator.free(regex_slice);
    const regex: *anyopaque = @ptrCast(regex_slice.ptr);

    var matches: [4]re.regmatch_t = undefined;

    // build expression
    var buf: [4096]u8 = undefined;
    var regex_str = std.ArrayList(u8).init(allocator);
    defer regex_str.deinit();
    {
        try regex_str.appendSlice("^(");

        const line = (try input.reader().readUntilDelimiterOrEof(&buf, '\n')).?;
        var iter = std.mem.tokenizeAny(u8, line, ", ");

        while (iter.next()) |match| {
            try regex_str.appendSlice(match);
            try regex_str.append('|');
        }

        try regex_str.appendSlice(")*$");
        try regex_str.append(0);
    }

    _ = try input.reader().readUntilDelimiterOrEof(&buf, '\n');

    while (try input.reader().readUntilDelimiterOrEof(&buf, '\n')) |line| {
        buf[line.len] = 0;
        const c_line: [:0]u8 = buf[0..line.len :0];

        // match expression
        if (regcomp(regex, regex_str.items[0 .. regex_str.items.len - 1 :0], re.REG_EXTENDED) != 0) {
            return RegexError.InvalidExpression;
        }
        defer regfree(regex);

        if (regexec(regex, c_line, matches.len, &matches, 0) != 0) {
            continue;
        }

        result += 1;
    }

    return result;
}

const Color = enum(u3) {
    w = 0,
    u = 1,
    b = 2,
    r = 3,
    g = 4,

    fn fromChar(char: u8) Color {
        return switch (char) {
            'w' => .w,
            'u' => .u,
            'b' => .b,
            'r' => .r,
            'g' => .g,
            else => unreachable,
        };
    }
};

const Item = struct {
    next: ?[]Item = null,
    is_towl: bool = false,

    fn deinit(self: Item, allocator: std.mem.Allocator) void {
        if (self.next) |next| {
            for (next) |item| {
                item.deinit(allocator);
            }
            allocator.free(next);
        }
    }
};

fn task_2(allocator: std.mem.Allocator, file_name: []const u8) !?result_type {
    // open input
    var file = try std.fs.cwd().openFile(file_name, .{});
    defer file.close();
    var input = std.io.bufferedReader(file.reader());

    // read data
    var towls = Item{};
    defer towls.deinit(allocator);

    var buf: [4096]u8 = undefined;
    {
        const line = (try input.reader().readUntilDelimiterOrEof(&buf, '\n')).?;
        var iter = std.mem.tokenizeAny(u8, line, ", ");

        while (iter.next()) |match| {
            var curr = &towls;
            for (match) |c| {
                if (curr.next == null) {
                    curr.next = try allocator.alloc(Item, 5);
                    @memset(curr.next.?, Item{});
                }
                curr = &curr.next.?[@intCast(@intFromEnum(Color.fromChar(c)))];
            }
            curr.is_towl = true;
        }
    }

    // calculate result
    _ = try input.reader().readUntilDelimiterOrEof(&buf, '\n');

    // offset => ways to get here
    var possibilities = std.ArrayList(result_type).init(allocator);
    defer possibilities.deinit();

    var result: result_type = 0;
    while (try input.reader().readUntilDelimiterOrEof(&buf, '\n')) |line| : (possibilities.clearRetainingCapacity()) {
        try possibilities.resize(line.len + 1);
        @memset(possibilities.items, 0);
        possibilities.items[0] = 1;

        for (0..line.len) |offset| {
            var curr = &towls;

            for (line[offset..line.len], 1..) |c, offset_2| {
                const color = Color.fromChar(c);

                if (curr.next) |next| {
                    curr = &next[@intFromEnum(color)];

                    if (curr.is_towl) {
                        possibilities.items[offset + offset_2] += possibilities.items[offset];
                    }
                } else {
                    break;
                }
            }
        }

        result += possibilities.items[line.len];
    }

    return result;
}

pub fn main() !void {
    const stdout = std.io.getStdOut().writer();

    // setup allocator
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    // read arguments
    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);

    // execute
    var result: result_type = 0;
    if (args.len < 2 or args[1][0] != '2') {
        result = (try task_1(allocator, "data.txt")).?;
    } else {
        result = (try task_2(allocator, "data.txt")).?;
    }

    try stdout.print("{d}\n", .{result});
}

test task_1 {
    try std.testing.expectEqual(6, try task_1(std.testing.allocator, "data_test1.txt"));
}

test task_2 {
    try std.testing.expectEqual(16, try task_2(std.testing.allocator, "data_test1.txt"));
}
