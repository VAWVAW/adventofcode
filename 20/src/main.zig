const std = @import("std");

const result_type = usize;
const Point = struct { y: usize, x: usize };
const Node = struct {
    pos: Point,
    cheat_pos: ?Point = null,
    cheat_start: ?Point = null,
};
const Distances = std.AutoHashMap(Node, result_type);

fn compareNode(ctx: *Distances, a: Node, b: Node) std.math.Order {
    const a_d = ctx.get(a) orelse unreachable;
    const b_d = ctx.get(b) orelse unreachable;
    return std.math.order(a_d, b_d);
}
fn comparePoint(ctx: *std.AutoHashMap(Point, result_type), a: Point, b: Point) std.math.Order {
    const a_d = ctx.get(a) orelse unreachable;
    const b_d = ctx.get(b) orelse unreachable;
    return std.math.order(a_d, b_d);
}

fn task_1(allocator: std.mem.Allocator, file_name: []const u8, min_diff: usize) !?result_type {
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

    var start_pos: ?Point = null;
    var end_pos: ?Point = null;

    for (lines.items, 0..) |line, y| {
        for (line, 0..) |c, x| {
            switch (c) {
                'S' => {
                    start_pos = Point{ .y = y, .x = x };
                },
                'E' => {
                    end_pos = Point{ .y = y, .x = x };
                },
                else => {},
            }
        }
    }

    // calculate distance without cheat
    var backwards_distances = std.AutoHashMap(Point, result_type).init(allocator);
    defer backwards_distances.deinit();

    const normal_distance = first: {
        var queue = std
            .PriorityQueue(Point, *std.AutoHashMap(Point, result_type), comparePoint)
            .init(allocator, &backwards_distances);
        defer queue.deinit();

        try backwards_distances.put(end_pos.?, 0);
        try queue.add(end_pos.?);

        while (queue.removeOrNull()) |u| {
            const distance = backwards_distances.get(u).?;

            // up
            if (u.y > 0 and lines.items[u.y - 1][u.x] != '#') {
                const new = Point{ .y = u.y - 1, .x = u.x };
                const res = try backwards_distances.getOrPut(new);
                if (!res.found_existing) {
                    res.value_ptr.* = distance + 1;
                    try queue.add(new);
                } else {
                    res.value_ptr.* = @min(res.value_ptr.*, distance + 1);
                }
            }
            // down
            if (u.y < lines.items.len - 1 and lines.items[u.y + 1][u.x] != '#') {
                const new = Point{ .y = u.y + 1, .x = u.x };
                const res = try backwards_distances.getOrPut(new);
                if (!res.found_existing) {
                    res.value_ptr.* = distance + 1;
                    try queue.add(new);
                } else {
                    res.value_ptr.* = @min(res.value_ptr.*, distance + 1);
                }
            }
            // left
            if (u.x > 0 and lines.items[u.y][u.x - 1] != '#') {
                const new = Point{ .y = u.y, .x = u.x - 1 };
                const res = try backwards_distances.getOrPut(new);
                if (!res.found_existing) {
                    res.value_ptr.* = distance + 1;
                    try queue.add(new);
                } else {
                    res.value_ptr.* = @min(res.value_ptr.*, distance + 1);
                }
            }
            // right
            if (u.x < line_length - 1 and lines.items[u.y][u.x + 1] != '#') {
                const new = Point{ .y = u.y, .x = u.x + 1 };
                const res = try backwards_distances.getOrPut(new);
                if (!res.found_existing) {
                    res.value_ptr.* = distance + 1;
                    try queue.add(new);
                } else {
                    res.value_ptr.* = @min(res.value_ptr.*, distance + 1);
                }
            }
        }

        break :first backwards_distances.get(start_pos.?).?;
    };

    // try cheats
    var distances = Distances.init(allocator);
    defer distances.deinit();

    var uncheated_distances = std.AutoHashMap(Point, usize).init(allocator);
    defer uncheated_distances.deinit();

    var queue = std.PriorityQueue(Node, *Distances, compareNode).init(allocator, &distances);
    defer queue.deinit();

    const start = Node{ .pos = start_pos.? };
    try distances.put(start, 0);
    try queue.add(start);

    var result: result_type = 0;
    while (queue.removeOrNull()) |u| {
        const distance = distances.get(u).?;
        if (distance + min_diff >= normal_distance)
            break;
        if (u.pos.x == end_pos.?.x and u.pos.y == end_pos.?.y) {
            result += 1;
            continue;
        }

        // update uncheated_distances
        if (u.cheat_pos == null) {
            const res = try uncheated_distances.getOrPut(u.pos);
            if (!res.found_existing) {
                res.value_ptr.* = distance;
            } else {
                res.value_ptr.* = @min(res.value_ptr.*, distance);
            }
        } else if (uncheated_distances.get(u.pos)) |dist| {
            if (dist <= distance)
                continue;
        }

        // short circuit after cheat
        if (u.cheat_pos != null and lines.items[u.pos.y][u.pos.x] != '#') {
            if (distance + backwards_distances.get(u.pos).? + min_diff < normal_distance) {
                result += 1;
            }
            continue;
        }

        // up
        if (u.pos.y > 0 and lines.items[u.pos.y - 1][u.pos.x] != '#') {
            const new = Node{
                .pos = .{ .y = u.pos.y - 1, .x = u.pos.x },
                .cheat_pos = u.cheat_pos,
                .cheat_start = u.cheat_start,
            };
            const res = try distances.getOrPut(new);
            if (!res.found_existing) {
                res.value_ptr.* = distance + 1;
                try queue.add(new);
            } else {
                res.value_ptr.* = @min(res.value_ptr.*, distance + 1);
            }
        }
        // down
        if (u.pos.y < lines.items.len - 1 and lines.items[u.pos.y + 1][u.pos.x] != '#') {
            const new = Node{
                .pos = .{ .y = u.pos.y + 1, .x = u.pos.x },
                .cheat_pos = u.cheat_pos,
                .cheat_start = u.cheat_start,
            };
            const res = try distances.getOrPut(new);
            if (!res.found_existing) {
                res.value_ptr.* = distance + 1;
                try queue.add(new);
            } else {
                res.value_ptr.* = @min(res.value_ptr.*, distance + 1);
            }
        }
        // left
        if (u.pos.x > 0 and lines.items[u.pos.y][u.pos.x - 1] != '#') {
            const new = Node{
                .pos = .{ .y = u.pos.y, .x = u.pos.x - 1 },
                .cheat_pos = u.cheat_pos,
                .cheat_start = u.cheat_start,
            };
            const res = try distances.getOrPut(new);
            if (!res.found_existing) {
                res.value_ptr.* = distance + 1;
                try queue.add(new);
            } else {
                res.value_ptr.* = @min(res.value_ptr.*, distance + 1);
            }
        }
        // right
        if (u.pos.x < line_length - 1 and lines.items[u.pos.y][u.pos.x + 1] != '#') {
            const new = Node{
                .pos = .{ .y = u.pos.y, .x = u.pos.x + 1 },
                .cheat_pos = u.cheat_pos,
                .cheat_start = u.cheat_start,
            };
            const res = try distances.getOrPut(new);
            if (!res.found_existing) {
                res.value_ptr.* = distance + 1;
                try queue.add(new);
            } else {
                res.value_ptr.* = @min(res.value_ptr.*, distance + 1);
            }
        }

        if (u.cheat_pos == null) {
            // try cheat

            // up
            if (u.pos.y > 0 and lines.items[u.pos.y - 1][u.pos.x] == '#') {
                const new = Node{
                    .pos = .{ .y = u.pos.y - 1, .x = u.pos.x },
                    .cheat_pos = .{ .y = u.pos.y - 1, .x = u.pos.x },
                    .cheat_start = u.pos,
                };
                const res = try distances.getOrPut(new);
                if (!res.found_existing) {
                    res.value_ptr.* = distance + 1;
                    try queue.add(new);
                } else {
                    res.value_ptr.* = @min(res.value_ptr.*, distance + 1);
                }
            }
            // down
            if (u.pos.y < lines.items.len - 1 and lines.items[u.pos.y + 1][u.pos.x] == '#') {
                const new = Node{
                    .pos = .{ .y = u.pos.y + 1, .x = u.pos.x },
                    .cheat_pos = .{ .y = u.pos.y + 1, .x = u.pos.x },
                    .cheat_start = u.pos,
                };
                const res = try distances.getOrPut(new);
                if (!res.found_existing) {
                    res.value_ptr.* = distance + 1;
                    try queue.add(new);
                } else {
                    res.value_ptr.* = @min(res.value_ptr.*, distance + 1);
                }
            }
            // left
            if (u.pos.x > 0 and lines.items[u.pos.y][u.pos.x - 1] == '#') {
                const new = Node{
                    .pos = .{ .y = u.pos.y, .x = u.pos.x - 1 },
                    .cheat_pos = .{ .y = u.pos.y, .x = u.pos.x - 1 },
                    .cheat_start = u.pos,
                };
                const res = try distances.getOrPut(new);
                if (!res.found_existing) {
                    res.value_ptr.* = distance + 1;
                    try queue.add(new);
                } else {
                    res.value_ptr.* = @min(res.value_ptr.*, distance + 1);
                }
            }
            // right
            if (u.pos.x < line_length - 1 and lines.items[u.pos.y][u.pos.x + 1] == '#') {
                const new = Node{
                    .pos = .{ .y = u.pos.y, .x = u.pos.x + 1 },
                    .cheat_pos = .{ .y = u.pos.y, .x = u.pos.x + 1 },
                    .cheat_start = u.pos,
                };
                const res = try distances.getOrPut(new);
                if (!res.found_existing) {
                    res.value_ptr.* = distance + 1;
                    try queue.add(new);
                } else {
                    res.value_ptr.* = @min(res.value_ptr.*, distance + 1);
                }
            }
        }
    }
    return result;
}

fn task_2(allocator: std.mem.Allocator, file_name: []const u8, min_diff: usize) !?result_type {
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

    var start_pos: ?Point = null;
    var end_pos: ?Point = null;

    for (lines.items, 0..) |line, y| {
        for (line, 0..) |c, x| {
            switch (c) {
                'S' => {
                    start_pos = Point{ .y = y, .x = x };
                },
                'E' => {
                    end_pos = Point{ .y = y, .x = x };
                },
                else => {},
            }
        }
    }

    // calculate distance without cheat
    var forward_distances = std.AutoHashMap(Point, result_type).init(allocator);
    defer forward_distances.deinit();
    {
        var queue = std
            .PriorityQueue(Point, *std.AutoHashMap(Point, result_type), comparePoint)
            .init(allocator, &forward_distances);
        defer queue.deinit();

        try forward_distances.put(start_pos.?, 0);
        try queue.add(start_pos.?);

        while (queue.removeOrNull()) |u| {
            const distance = forward_distances.get(u).?;

            // up
            if (u.y > 0 and lines.items[u.y - 1][u.x] != '#') {
                const new = Point{ .y = u.y - 1, .x = u.x };
                const res = try forward_distances.getOrPut(new);
                if (!res.found_existing) {
                    res.value_ptr.* = distance + 1;
                    try queue.add(new);
                } else {
                    res.value_ptr.* = @min(res.value_ptr.*, distance + 1);
                }
            }
            // down
            if (u.y < lines.items.len - 1 and lines.items[u.y + 1][u.x] != '#') {
                const new = Point{ .y = u.y + 1, .x = u.x };
                const res = try forward_distances.getOrPut(new);
                if (!res.found_existing) {
                    res.value_ptr.* = distance + 1;
                    try queue.add(new);
                } else {
                    res.value_ptr.* = @min(res.value_ptr.*, distance + 1);
                }
            }
            // left
            if (u.x > 0 and lines.items[u.y][u.x - 1] != '#') {
                const new = Point{ .y = u.y, .x = u.x - 1 };
                const res = try forward_distances.getOrPut(new);
                if (!res.found_existing) {
                    res.value_ptr.* = distance + 1;
                    try queue.add(new);
                } else {
                    res.value_ptr.* = @min(res.value_ptr.*, distance + 1);
                }
            }
            // right
            if (u.x < line_length - 1 and lines.items[u.y][u.x + 1] != '#') {
                const new = Point{ .y = u.y, .x = u.x + 1 };
                const res = try forward_distances.getOrPut(new);
                if (!res.found_existing) {
                    res.value_ptr.* = distance + 1;
                    try queue.add(new);
                } else {
                    res.value_ptr.* = @min(res.value_ptr.*, distance + 1);
                }
            }
        }
    }
    var backward_distances = std.AutoHashMap(Point, result_type).init(allocator);
    defer backward_distances.deinit();

    const normal_distance = first: {
        var queue = std
            .PriorityQueue(Point, *std.AutoHashMap(Point, result_type), comparePoint)
            .init(allocator, &backward_distances);
        defer queue.deinit();

        try backward_distances.put(end_pos.?, 0);
        try queue.add(end_pos.?);

        while (queue.removeOrNull()) |u| {
            const distance = backward_distances.get(u).?;

            // up
            if (u.y > 0 and lines.items[u.y - 1][u.x] != '#') {
                const new = Point{ .y = u.y - 1, .x = u.x };
                const res = try backward_distances.getOrPut(new);
                if (!res.found_existing) {
                    res.value_ptr.* = distance + 1;
                    try queue.add(new);
                } else {
                    res.value_ptr.* = @min(res.value_ptr.*, distance + 1);
                }
            }
            // down
            if (u.y < lines.items.len - 1 and lines.items[u.y + 1][u.x] != '#') {
                const new = Point{ .y = u.y + 1, .x = u.x };
                const res = try backward_distances.getOrPut(new);
                if (!res.found_existing) {
                    res.value_ptr.* = distance + 1;
                    try queue.add(new);
                } else {
                    res.value_ptr.* = @min(res.value_ptr.*, distance + 1);
                }
            }
            // left
            if (u.x > 0 and lines.items[u.y][u.x - 1] != '#') {
                const new = Point{ .y = u.y, .x = u.x - 1 };
                const res = try backward_distances.getOrPut(new);
                if (!res.found_existing) {
                    res.value_ptr.* = distance + 1;
                    try queue.add(new);
                } else {
                    res.value_ptr.* = @min(res.value_ptr.*, distance + 1);
                }
            }
            // right
            if (u.x < line_length - 1 and lines.items[u.y][u.x + 1] != '#') {
                const new = Point{ .y = u.y, .x = u.x + 1 };
                const res = try backward_distances.getOrPut(new);
                if (!res.found_existing) {
                    res.value_ptr.* = distance + 1;
                    try queue.add(new);
                } else {
                    res.value_ptr.* = @min(res.value_ptr.*, distance + 1);
                }
            }
        }

        break :first backward_distances.get(start_pos.?).?;
    };

    // try cheats
    var cheats = std.AutoHashMap([2]Point, void).init(allocator);
    defer cheats.deinit();

    for (lines.items, 0..) |line, y| {
        for (line, 0..) |c, x| {
            if (c == '#')
                continue;

            var dy: isize = -20;
            while (dy <= 20) : (dy += 1) {
                var dx: isize = -20;
                while (dx <= 20) : (dx += 1) {
                    if (@abs(dy) + @abs(dx) > 20 or
                        @as(isize, @intCast(y)) + dy < 0 or
                        @as(isize, @intCast(x)) + dx < 0)
                        continue;

                    const new_y: usize = @intCast(@as(isize, @intCast(y)) + dy);
                    const new_x: usize = @intCast(@as(isize, @intCast(x)) + dx);

                    if (new_y >= lines.items.len or new_x >= line.len or lines.items[new_y][new_x] == '#')
                        continue;

                    if (forward_distances.get(.{ .y = y, .x = x }).? +
                        backward_distances.get(.{ .y = new_y, .x = new_x }).? +
                        @abs(dy) + @abs(dx) + min_diff > normal_distance)
                        continue;

                    try cheats.put([_]Point{
                        Point{ .y = y, .x = x },
                        Point{ .y = new_y, .x = new_x },
                    }, {});
                }
            }
        }
    }

    return cheats.count();
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
        result = (try task_1(allocator, "data.txt", 99)).?;
    } else {
        result = (try task_2(allocator, "data.txt", 100)).?;
    }
    try stdout.print("{d}\n", .{result});
}

test task_1 {
    try std.testing.expectEqual(44, try task_1(std.testing.allocator, "data_test1.txt", 1));
}

test task_2 {
    try std.testing.expectEqual(285, try task_2(std.testing.allocator, "data_test1.txt", 49));
}
