const std = @import("std");

const re = @cImport(@cInclude("regex.h"));

extern fn regcomp(noalias __preg: *anyopaque, noalias __pattern: [*c]const u8, __cflags: c_int) c_int;
extern fn regexec(noalias __preg: *const anyopaque, noalias __String: [*c]const u8, __nmatch: usize, noalias __pmatch: [*c]re.regmatch_t, __eflags: c_int) c_int;
extern fn regfree(noalias __preg: *anyopaque) void;

// hardcode values for x86_64 linux
const REGEX_T_SIZEOF = 64;
const REGEX_T_ALIGNOF = 8;

const RegexError = error{ InvalidExpression, MatchError };

fn task_1(allocator: std.mem.Allocator, file_name: []const u8) !?i32 {
    // open input
    var file = try std.fs.cwd().openFile(file_name, .{});
    defer file.close();
    var input = std.io.bufferedReader(file.reader());

    // read data
    var result: i32 = 0;

    const regex_slice = try allocator.alignedAlloc(u8, REGEX_T_ALIGNOF, REGEX_T_SIZEOF);
    defer allocator.free(regex_slice);
    const regex: *anyopaque = @ptrCast(regex_slice.ptr);

    var matches: [4]re.regmatch_t = undefined;

    var buf: [4096]u8 = undefined;
    while (try input.reader().readUntilDelimiterOrEof(&buf, '\n')) |line| {
        buf[line.len] = 0;
        var c_line: [:0]u8 = buf[0..line.len :0];

        if (regcomp(regex, "mul\\(([0-9]{1,3}),([0-9]{1,3})\\)", re.REG_EXTENDED) != 0) {
            return RegexError.InvalidExpression;
        }
        defer regfree(regex);

        while (true) {
            if (regexec(regex, c_line, matches.len, &matches, 0) != 0) {
                break;
            }
            if (matches[1].rm_so == -1 or matches[2].rm_so == -1 or matches[3].rm_so != -1) {
                return RegexError.MatchError;
            }

            const m_left = c_line[@intCast(matches[1].rm_so)..@intCast(matches[1].rm_eo)];
            const m_right = c_line[@intCast(matches[2].rm_so)..@intCast(matches[2].rm_eo)];

            const left = try std.fmt.parseInt(i32, m_left, 10);
            const right = try std.fmt.parseInt(i32, m_right, 10);

            result += left * right;

            c_line = c_line[@intCast(matches[0].rm_eo)..c_line.len :0];
        }
    }

    return result;
}

fn task_2(allocator: std.mem.Allocator, file_name: []const u8) !?i32 {
    // open input
    var file = try std.fs.cwd().openFile(file_name, .{});
    defer file.close();
    var input = std.io.bufferedReader(file.reader());

    // read data
    var result: i32 = 0;

    const regex_slice = try allocator.alignedAlloc(u8, REGEX_T_ALIGNOF, REGEX_T_SIZEOF);
    defer allocator.free(regex_slice);
    const regex: *anyopaque = @ptrCast(regex_slice.ptr);

    var matches: [4]re.regmatch_t = undefined;

    var mul_enabled = true;

    var buf: [4096]u8 = undefined;
    while (try input.reader().readUntilDelimiterOrEof(&buf, '\n')) |line| {
        buf[line.len] = 0;
        var c_line: [:0]u8 = buf[0..line.len :0];

        if (regcomp(regex, "mul\\(([0-9]{1,3}),([0-9]{1,3})\\)|do\\(\\)|don't\\(\\)", re.REG_EXTENDED) != 0) {
            return RegexError.InvalidExpression;
        }
        defer regfree(regex);

        while (true) {
            if (regexec(regex, c_line, matches.len, &matches, 0) != 0) {
                break;
            }
            if (matches[3].rm_so != -1) {
                return RegexError.MatchError;
            }

            defer c_line = c_line[@intCast(matches[0].rm_eo)..c_line.len :0];

            const match = c_line[@intCast(matches[0].rm_so)..@intCast(matches[0].rm_eo)];
            if (match[0] == 'd') {
                if (match.len == 4) {
                    mul_enabled = true;
                } else {
                    mul_enabled = false;
                }
                continue;
            }

            if (mul_enabled == false) {
                continue;
            }

            const m_left = c_line[@intCast(matches[1].rm_so)..@intCast(matches[1].rm_eo)];
            const m_right = c_line[@intCast(matches[2].rm_so)..@intCast(matches[2].rm_eo)];

            const left = try std.fmt.parseInt(i32, m_left, 10);
            const right = try std.fmt.parseInt(i32, m_right, 10);

            result += left * right;
        }
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
    var result: i32 = 0;
    if (args.len < 2 or args[1][0] != '2') {
        result = (try task_1(allocator, "data.txt")).?;
    } else {
        result = (try task_2(allocator, "data.txt")).?;
    }

    try stdout.print("{d}\n", .{result});
}

test task_1 {
    try std.testing.expectEqual(161, try task_1(std.testing.allocator, "data_test1.txt"));
}

test task_2 {
    try std.testing.expectEqual(48, try task_2(std.testing.allocator, "data_test2.txt"));
}
