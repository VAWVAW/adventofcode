const std = @import("std");

const result_type = usize;

const Point = struct { y: isize, x: isize };
const List = std.ArrayList(Point);

fn task_1(allocator: std.mem.Allocator, file_name: []const u8) !?result_type {
    // open input
    var file = try std.fs.cwd().openFile(file_name, .{});
    defer file.close();
    var input = std.io.bufferedReader(file.reader());

    // read data
    const data = try input.reader().readAllAlloc(allocator, 1 << 32);
    defer allocator.free(data);
    const width = std.mem.indexOf(u8, data, "\n").?;
    const height = data.len / (width + 1);

    var antennas = std.AutoArrayHashMap(u8, List).init(allocator);
    defer antennas.deinit();
    defer while (antennas.popOrNull()) |entry| {
        entry.value.deinit();
    };

    for (data, 0..) |c, i| {
        if (c == '.' or c == '\n')
            continue;

        const res = try antennas.getOrPut(c);
        if (!res.found_existing) {
            res.value_ptr.* = List.init(allocator);
        }

        const y = i / (width + 1);
        const x = i % (width + 1);

        try res.value_ptr.append(.{ .x = @intCast(x), .y = @intCast(y) });
    }

    // calculate results

    var overlappings = std.AutoArrayHashMap(Point, void).init(allocator);
    defer overlappings.deinit();

    var antenna_types = antennas.iterator();
    while (antenna_types.next()) |antanna_type| {
        const n = antanna_type.value_ptr.items.len;

        for (0..n * n) |i| {
            const a = antanna_type.value_ptr.items[i / n];
            const b = antanna_type.value_ptr.items[i % n];

            if ((a.x + a.x < b.x) or (a.y + a.y < b.y) or (a.x + a.x - b.x >= width) or (a.y + a.y - b.y >= height) or (a.x == b.x and a.y == b.y)) {
                continue;
            }

            const overlapping = Point{
                .x = a.x + a.x - b.x,
                .y = a.y + a.y - b.y,
            };

            try overlappings.put(overlapping, {});
        }
    }

    return overlappings.keys().len;
}

fn task_2(allocator: std.mem.Allocator, file_name: []const u8) !?result_type {
    // open input
    var file = try std.fs.cwd().openFile(file_name, .{});
    defer file.close();
    var input = std.io.bufferedReader(file.reader());

    // read data
    const data = try input.reader().readAllAlloc(allocator, 1 << 32);
    defer allocator.free(data);
    const width = std.mem.indexOf(u8, data, "\n").?;
    const height = data.len / (width + 1);

    var antennas = std.AutoArrayHashMap(u8, List).init(allocator);
    defer antennas.deinit();
    defer while (antennas.popOrNull()) |entry| {
        entry.value.deinit();
    };

    for (data, 0..) |c, i| {
        if (c == '.' or c == '\n')
            continue;

        const res = try antennas.getOrPut(c);
        if (!res.found_existing) {
            res.value_ptr.* = List.init(allocator);
        }

        const y = i / (width + 1);
        const x = i % (width + 1);

        try res.value_ptr.append(.{ .x = @intCast(x), .y = @intCast(y) });
    }

    // calculate results

    var overlappings = std.AutoArrayHashMap(Point, void).init(allocator);
    defer overlappings.deinit();

    var antenna_types = antennas.iterator();
    while (antenna_types.next()) |antanna_type| {
        const n = antanna_type.value_ptr.items.len;

        for (0..n * n) |i| {
            const a = antanna_type.value_ptr.items[i / n];
            const b = antanna_type.value_ptr.items[i % n];

            const dx = a.x - b.x;
            const dy = a.y - b.y;

            if (dx == 0 and dy == 0)
                continue;

            var j: usize = 0;
            var last_failed = false;
            while (true) : (j += 1) {
                const sign: isize = if (j % 2 == 0) 1 else -1;

                const j_i: isize = @intCast(j);

                const new = Point{
                    .x = a.x + sign * @divTrunc(j_i, 2) * dx,
                    .y = a.y + sign * @divTrunc(j_i, 2) * dy,
                };

                if (new.x < 0 or new.y < 0 or new.x >= width or new.y >= height) {
                    if (last_failed) {
                        break;
                    }
                    last_failed = true;
                } else {
                    try overlappings.put(new, {});
                    last_failed = false;
                }
            }
        }
    }

    return overlappings.keys().len;
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
    try std.testing.expectEqual(14, try task_1(std.testing.allocator, "data_test1.txt"));
}

test task_2 {
    try std.testing.expectEqual(34, try task_2(std.testing.allocator, "data_test2.txt"));
}
