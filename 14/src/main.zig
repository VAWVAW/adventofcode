const std = @import("std");

const result_type = i32;
fn task_1(allocator: std.mem.Allocator, file_name: []const u8, steps: result_type, width: result_type, heigth: result_type) !?result_type {
    _ = allocator;
    // open input
    var file = try std.fs.cwd().openFile(file_name, .{});
    defer file.close();
    var input = std.io.bufferedReader(file.reader());

    // read data
    var ends = [_]result_type{0} ** 4;

    var buf: [32]u8 = undefined;
    while (try input.reader().readUntilDelimiterOrEof(&buf, '\n')) |line| {
        var token_iter = std.mem.tokenizeAny(u8, line, "p=, v\n");

        const x_start = try std.fmt.parseInt(result_type, token_iter.next().?, 10);
        const y_start = try std.fmt.parseInt(result_type, token_iter.next().?, 10);
        const x_diff = try std.fmt.parseInt(result_type, token_iter.next().?, 10);
        const y_diff = try std.fmt.parseInt(result_type, token_iter.next().?, 10);

        const x_end = @mod(x_start + steps * x_diff, width);
        const y_end = @mod(y_start + steps * y_diff, heigth);

        if (x_end == @divTrunc(width, 2) or y_end == @divTrunc(heigth, 2)) {
            continue;
        }

        var idx: usize = 0;
        if (x_end > @divTrunc(width, 2)) {
            idx += 1;
        }
        if (y_end > @divTrunc(heigth, 2)) {
            idx += 2;
        }
        ends[idx] += 1;
    }

    return ends[0] * ends[1] * ends[2] * ends[3];
}

const Robot = struct {
    x: result_type,
    y: result_type,
    dx: result_type,
    dy: result_type,
};

fn lessThanFn(_: void, lhs: Robot, rhs: Robot) bool {
    if (lhs.y != rhs.y) {
        return lhs.y < rhs.y;
    }
    return lhs.x < rhs.x;
}

fn task_2(allocator: std.mem.Allocator, file_name: []const u8, width: result_type, heigth: result_type) !?result_type {
    // open input
    var file = try std.fs.cwd().openFile(file_name, .{});
    defer file.close();
    var input = std.io.bufferedReader(file.reader());

    // setup stuff
    const u_width: usize = @intCast(width);
    const u_height: usize = @intCast(heigth);

    var robots = std.ArrayList(Robot).init(allocator);
    defer robots.deinit();

    var output = try allocator.alloc(u8, (u_width + 1) * u_height);
    defer allocator.free(output);
    var lines: [][:'\n']u8 = try allocator.alloc([:'\n']u8, u_height);
    defer allocator.free(lines);

    for (0..u_height) |i| {
        output[(u_width + 1) * (i + 1) - 1] = '\n';
        lines[i] = output[(u_width + 1) * i .. (u_width + 1) * (i + 1) - 1 :'\n'];
    }

    // read data
    var buf: [32]u8 = undefined;
    while (try input.reader().readUntilDelimiterOrEof(&buf, '\n')) |line| {
        var token_iter = std.mem.tokenizeAny(u8, line, "p=, v\n");

        const x_start = try std.fmt.parseInt(result_type, token_iter.next().?, 10);
        const y_start = try std.fmt.parseInt(result_type, token_iter.next().?, 10);
        const x_diff = try std.fmt.parseInt(result_type, token_iter.next().?, 10);
        const y_diff = try std.fmt.parseInt(result_type, token_iter.next().?, 10);

        try robots.append(.{
            .x = x_start,
            .y = y_start,
            .dx = x_diff,
            .dy = y_diff,
        });
    }

    // simulate steps
    var robots_per_line = try allocator.alloc(u8, u_height);
    defer allocator.free(robots_per_line);

    for (0..1_000_000_000) |step| {
        @memset(robots_per_line, 0);

        // step robots
        for (robots.items) |*robot| {
            robot.x = @mod(robot.x + robot.dx, width);
            robot.y = @mod(robot.y + robot.dy, heigth);

            robots_per_line[@intCast(robot.y)] += 1;
        }

        // detect tree
        var start_line: ?usize = null;
        for (robots_per_line[0 .. u_height - 2], robots_per_line[1 .. u_height - 1], 0..) |a, b, i| {
            if (a > b + 8) {
                start_line = i;
            }
        }
        if (start_line == null)
            continue;

        // print output
        for (lines) |*line| {
            @memset(line.*, '.');
        }
        for (robots.items) |robot| {
            const pos = &lines[@intCast(robot.y)][@intCast(robot.x)];
            pos.* = '#';
        }
        if (std.mem.indexOf(u8, lines[start_line.?], &([_]u8{'#'} ** 10)) != null) {
            return @intCast(step + 1);
        }
    }
    return error.NotFound;
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
    var result: result_type = undefined;
    if (args.len < 2 or args[1][0] != '2') {
        result = (try task_1(allocator, "data.txt", 100, 101, 103)).?;
    } else {
        result = (try task_2(allocator, "data.txt", 101, 103)).?;
    }

    try stdout.print("{d}\n", .{result});
}

test task_1 {
    try std.testing.expectEqual(12, try task_1(std.testing.allocator, "data_test1.txt", 100, 11, 7));
}
