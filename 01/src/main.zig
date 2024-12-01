const std = @import("std");

fn task_1(allocator: std.mem.Allocator, file_name: []const u8) !?i32 {
    // open input
    var file = try std.fs.cwd().openFile(file_name, .{});
    defer file.close();
    var input = std.io.bufferedReader(file.reader());

    // data buffer
    var left = std.ArrayList(i32).init(allocator);
    defer left.deinit();
    var right = std.ArrayList(i32).init(allocator);
    defer right.deinit();

    // read data
    var buf: [16]u8 = undefined;
    while (try input.reader().readUntilDelimiterOrEof(&buf, '\n')) |line| {
        var iter = std.mem.tokenizeAny(u8, line, &[_]u8{' '});

        const l = try std.fmt.parseInt(i32, iter.next() orelse return null, 10);
        const r = try std.fmt.parseInt(i32, iter.next() orelse return null, 10);

        try left.append(l);
        try right.append(r);
    }

    // calculate result
    std.mem.sortUnstable(i32, left.items, {}, comptime std.sort.asc(i32));
    std.mem.sortUnstable(i32, right.items, {}, comptime std.sort.asc(i32));
    var diff: u32 = 0;

    for (left.items, right.items) |l, r| {
        diff += @abs(l - r);
    }

    return @intCast(diff);
}

fn task_2(allocator: std.mem.Allocator, file_name: []const u8) !?i32 {
    // open input
    var file = try std.fs.cwd().openFile(file_name, .{});
    defer file.close();
    var input = std.io.bufferedReader(file.reader());

    // data buffer
    var left = std.ArrayList(i32).init(allocator);
    defer left.deinit();
    var right = std.ArrayList(i32).init(allocator);
    defer right.deinit();

    // read data
    var buf: [16]u8 = undefined;
    while (try input.reader().readUntilDelimiterOrEof(&buf, '\n')) |line| {
        var iter = std.mem.tokenizeAny(u8, line, &[_]u8{' '});

        const l = try std.fmt.parseInt(i32, iter.next() orelse return null, 10);
        const r = try std.fmt.parseInt(i32, iter.next() orelse return null, 10);

        try left.append(l);
        try right.append(r);
    }

    // calculate result
    std.mem.sortUnstable(i32, left.items, {}, comptime std.sort.asc(i32));
    std.mem.sortUnstable(i32, right.items, {}, comptime std.sort.asc(i32));

    const len = right.items.len;
    var r_index: usize = 0;
    var score: i32 = 0;

    for (left.items) |l| {
        while (r_index < len and right.items[r_index] < l) {
            r_index += 1;
        }
        if (r_index == len) {
            break;
        }

        var offset = r_index;
        while (offset < len and right.items[offset] == l) {
            score += l;
            offset += 1;
        }
    }

    return score;
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

test "example 1" {
    try std.testing.expect(try task_1(std.testing.allocator, "data_test1.txt") == 11);
}

test "example 2" {
    try std.testing.expect(try task_2(std.testing.allocator, "data_test2.txt") == 31);
}
