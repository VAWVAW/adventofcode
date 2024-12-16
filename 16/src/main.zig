const std = @import("std");

const result_type = usize;
const Point = struct { y: usize, x: usize };
const Direction = enum {
    up,
    right,
    down,
    left,

    fn move(self: Direction, p: Point) Point {
        return switch (self) {
            .up => Point{ .x = p.x, .y = p.y - 1 },
            .right => Point{ .x = p.x + 1, .y = p.y },
            .down => Point{ .x = p.x, .y = p.y + 1 },
            .left => Point{ .x = p.x - 1, .y = p.y },
        };
    }
};
const Item = struct {
    pos: Point,
    dir: Direction,
};
const Context = std.AutoHashMap(Item, result_type);
fn compareFn(ctx: *Context, a: Item, b: Item) std.math.Order {
    const a_d = ctx.get(a) orelse unreachable;
    const b_d = ctx.get(b) orelse unreachable;
    return std.math.order(a_d, b_d);
}

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

    var start: ?Point = null;
    var end: ?Point = null;

    for (lines.items, 0..) |line, y| {
        for (line, 0..) |c, x| {
            switch (c) {
                'S' => {
                    start = Point{ .y = y, .x = x };
                },
                'E' => {
                    end = Point{ .y = y, .x = x };
                },
                else => {},
            }
        }
    }

    // naive dijkstra
    var vertices = Context.init(allocator);
    defer vertices.deinit();

    var q = std.PriorityQueue(Item, *Context, compareFn).init(allocator, &vertices);
    defer q.deinit();

    try vertices.put(.{
        .pos = start.?,
        .dir = .right,
    }, 0);
    try q.add(.{
        .pos = start.?,
        .dir = .right,
    });

    while (q.removeOrNull()) |u| {
        if (u.pos.x == end.?.x and u.pos.y == end.?.y)
            return vertices.get(u);
        const distance = vertices.get(u).?;

        // move forward
        const new_pos = u.dir.move(u.pos);
        if (lines.items[new_pos.y][new_pos.x] != '#') {
            const new_item = Item{ .pos = new_pos, .dir = u.dir };
            const res = try vertices.getOrPut(new_item);
            if (!res.found_existing) {
                res.value_ptr.* = distance + 1;
                try q.add(new_item);
            } else {
                res.value_ptr.* = @min(res.value_ptr.*, distance + 1);
            }
        }

        // rotate
        const new_directions = switch (u.dir) {
            .up => .{ Direction.right, Direction.left },
            .right => .{ Direction.up, Direction.down },
            .down => .{ Direction.right, Direction.left },
            .left => .{ Direction.up, Direction.down },
        };
        inline for (new_directions) |new_dir| {
            const new_item = Item{ .pos = u.pos, .dir = new_dir };
            const res = try vertices.getOrPut(new_item);
            if (!res.found_existing) {
                res.value_ptr.* = distance + 1000;
                try q.add(new_item);
            } else {
                res.value_ptr.* = @min(res.value_ptr.*, distance + 1000);
            }
        }
    }

    return error.NotImplemented;
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
    var result: result_type = undefined;
    if (args.len < 2 or args[1][0] != '2') {
        result = (try task_1(allocator, "data.txt")).?;
    } else {
        result = (try task_2(allocator, "data.txt")).?;
    }

    try stdout.print("{d}\n", .{result});
}

test task_1 {
    try std.testing.expectEqual(7036, try task_1(std.testing.allocator, "data_test1.txt"));
    try std.testing.expectEqual(11048, try task_1(std.testing.allocator, "data_test2.txt"));
}

test task_2 {
    // try std.testing.expectEqual(9021, try task_2(std.testing.allocator, "data_test1.txt"));
}
