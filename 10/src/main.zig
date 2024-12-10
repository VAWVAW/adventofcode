const std = @import("std");

const result_type = usize;
const Point = struct {
    y: usize,
    x: usize,
};

fn task_1(allocator: std.mem.Allocator, file_name: []const u8) !?result_type {
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
    const PointMap = std.AutoArrayHashMap(Point, void);
    var old_positions = PointMap.init(allocator);
    defer old_positions.deinit();
    var new_positions = PointMap.init(allocator);
    defer new_positions.deinit();

    var result: result_type = 0;

    for (lines.items, 0..) |line, y_head| {
        for (line, 0..) |c_head, x_head| {
            if (c_head != '0')
                continue;

            old_positions.clearRetainingCapacity();
            try old_positions.put(.{ .y = y_head, .x = x_head }, {});

            for (1..10) |level| {
                new_positions.clearRetainingCapacity();
                for (old_positions.keys()) |pos| {
                    if (pos.y > 0 and lines.items[pos.y - 1][pos.x] == level + '0')
                        try new_positions.put(.{ .y = pos.y - 1, .x = pos.x }, {});

                    if (pos.x > 0 and lines.items[pos.y][pos.x - 1] == level + '0')
                        try new_positions.put(.{ .y = pos.y, .x = pos.x - 1 }, {});

                    if (pos.y < lines.items.len - 1 and lines.items[pos.y + 1][pos.x] == level + '0')
                        try new_positions.put(.{ .y = pos.y + 1, .x = pos.x }, {});

                    if (pos.x < line_length - 1 and lines.items[pos.y][pos.x + 1] == level + '0')
                        try new_positions.put(.{ .y = pos.y, .x = pos.x + 1 }, {});
                }

                std.mem.swap(PointMap, &old_positions, &new_positions);
            }

            result += old_positions.keys().len;
        }
    }

    return result;
}

fn task_2(allocator: std.mem.Allocator, file_name: []const u8) !?result_type {
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
    const PointList = std.ArrayList(Point);
    var old_positions = PointList.init(allocator);
    defer old_positions.deinit();
    var new_positions = PointList.init(allocator);
    defer new_positions.deinit();

    var result: result_type = 0;

    for (lines.items, 0..) |line, y_head| {
        for (line, 0..) |c_head, x_head| {
            if (c_head != '0')
                continue;

            old_positions.clearRetainingCapacity();
            try old_positions.append(.{ .y = y_head, .x = x_head });

            for (1..10) |level| {
                new_positions.clearRetainingCapacity();
                for (old_positions.items) |pos| {
                    if (pos.y > 0 and lines.items[pos.y - 1][pos.x] == level + '0')
                        try new_positions.append(.{ .y = pos.y - 1, .x = pos.x });

                    if (pos.x > 0 and lines.items[pos.y][pos.x - 1] == level + '0')
                        try new_positions.append(.{ .y = pos.y, .x = pos.x - 1 });

                    if (pos.y < lines.items.len - 1 and lines.items[pos.y + 1][pos.x] == level + '0')
                        try new_positions.append(.{ .y = pos.y + 1, .x = pos.x });

                    if (pos.x < line_length - 1 and lines.items[pos.y][pos.x + 1] == level + '0')
                        try new_positions.append(.{ .y = pos.y, .x = pos.x + 1 });
                }

                std.mem.swap(PointList, &old_positions, &new_positions);
            }

            result += old_positions.items.len;
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
    var result: result_type = undefined;
    if (args.len < 2 or args[1][0] != '2') {
        result = (try task_1(allocator, "data.txt")).?;
    } else {
        result = (try task_2(allocator, "data.txt")).?;
    }

    try stdout.print("{d}\n", .{result});
}

test task_1 {
    try std.testing.expectEqual(36, try task_1(std.testing.allocator, "data_test1.txt"));
}

test task_2 {
    try std.testing.expectEqual(81, try task_2(std.testing.allocator, "data_test1.txt"));
}
