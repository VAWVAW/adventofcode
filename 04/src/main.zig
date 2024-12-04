const std = @import("std");

fn task_1(allocator: std.mem.Allocator, file_name: []const u8) !?i32 {
    // open input
    var file = try std.fs.cwd().openFile(file_name, .{});
    defer file.close();
    var input = std.io.bufferedReader(file.reader());

    // read data
    const data = try input.reader().readAllAlloc(allocator, 1 << 32);
    defer allocator.free(data);

    const line_length = std.mem.indexOf(u8, data, "\n").?;

    var lines = std.ArrayList([:'\n']u8).init(allocator);
    defer lines.deinit();

    var i: usize = 0;
    while (i < data.len) {
        try lines.append(data[i .. i + line_length :'\n']);
        i += line_length + 1;
    }

    // loop over data
    const expected = "XMAS";
    var result: i32 = 0;

    for (lines.items, 0..) |line, y_start| {
        for (line, 0..) |char, x_start| {
            if (char != expected[0])
                continue;

            outer: for (0..9) |direction| {
                const d_x = @as(isize, @intCast(direction % 3)) - 1;
                const d_y = @as(isize, @intCast(direction / 3)) - 1;

                if (d_x == 0 and d_y == 0)
                    continue;

                for (expected[1..], 1..) |expected_c, offset| {
                    const x = @as(isize, @intCast(x_start)) + d_x * @as(isize, @intCast(offset));
                    const y = @as(isize, @intCast(y_start)) + d_y * @as(isize, @intCast(offset));

                    if (y < 0 or y >= lines.items.len or x < 0 or x >= line_length)
                        continue :outer;

                    if (lines.items[@intCast(y)][@intCast(x)] != expected_c)
                        continue :outer;
                }
                result += 1;
            }
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
    const data = try input.reader().readAllAlloc(allocator, 1 << 32);
    defer allocator.free(data);

    const line_length = std.mem.indexOf(u8, data, "\n").?;

    var lines = std.ArrayList([:'\n']u8).init(allocator);
    defer lines.deinit();

    var i: usize = 0;
    while (i < data.len) {
        try lines.append(data[i .. i + line_length :'\n']);
        i += line_length + 1;
    }

    // loop over data
    var result: i32 = 0;

    for (lines.items, 0..) |line, y_start_u| {
        for (line, 0..) |char, x_start_u| {
            if (char != 'A')
                continue;
            if (x_start_u == 0 or y_start_u == 0 or x_start_u == line_length or y_start_u == lines.items.len - 1) {
                continue;
            }

            const y_start: isize = @intCast(y_start_u);
            const x_start: isize = @intCast(x_start_u);

            var values: [2]u2 = .{ 0, 0 };
            for (0..9) |direction| {
                const d_x = @as(isize, @intCast(direction % 3)) - 1;
                const d_y = @as(isize, @intCast(direction / 3)) - 1;

                if (d_y == 0 or d_x == 0) {
                    continue;
                }

                if (lines.items[@intCast(y_start + d_y)][@intCast(x_start + d_x)] == 'M' and lines.items[@intCast(y_start - d_y)][@intCast(x_start - d_x)] == 'S') {
                    values[(@abs(d_y) + @abs(d_x)) % 2] += 1;
                }
            }

            result += @intCast(values[0] / 2);
            result += @intCast(values[1] / 2);
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
    try std.testing.expectEqual(18, try task_1(std.testing.allocator, "data_test1.txt"));
}

test task_2 {
    try std.testing.expectEqual(9, try task_2(std.testing.allocator, "data_test2.txt"));
}
