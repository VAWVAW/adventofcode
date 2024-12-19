const std = @import("std");

const re = @cImport(@cInclude("regex.h"));

extern fn regcomp(noalias __preg: *anyopaque, noalias __pattern: [*c]const u8, __cflags: c_int) c_int;
extern fn regexec(noalias __preg: *const anyopaque, noalias __String: [*c]const u8, __nmatch: usize, noalias __pmatch: [*c]re.regmatch_t, __eflags: c_int) c_int;
extern fn regfree(noalias __preg: *anyopaque) void;

// hardcode values for x86_64 linux
const REGEX_T_SIZEOF = 64;
const REGEX_T_ALIGNOF = 8;

const RegexError = error{ InvalidExpression, MatchError };

const result_type = u16;

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

        std.debug.print("{s}\n", .{line});

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

fn task_2(allocator: std.mem.Allocator, file_name: []const u8) !?result_type {
    _ = file_name; // autofix
    _ = allocator; // autofix
    return error.NotImplemented;
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
    // try std.testing.expectEqual(48, try task_2(std.testing.allocator, "data_test2.txt"));
}
