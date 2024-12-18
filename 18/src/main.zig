const std = @import("std");

const result_type = usize;
const Point = struct { y: usize, x: usize };
const Context = std.AutoHashMap(Point, result_type);
fn compareFn(ctx: *Context, a: Point, b: Point) std.math.Order {
    const a_d = ctx.get(a) orelse unreachable;
    const b_d = ctx.get(b) orelse unreachable;
    return std.math.order(a_d, b_d);
}

fn task_1(allocator: std.mem.Allocator, file_name: []const u8, comptime is_test: bool) !?result_type {
    const field_size = if (is_test) 7 else 71;
    const simulated_bytes = if (is_test) 12 else 1024;

    // open input
    var file = try std.fs.cwd().openFile(file_name, .{});
    defer file.close();
    var input = std.io.bufferedReader(file.reader());

    // read data
    var field = try allocator.alloc(u8, (field_size + 1) * field_size);
    defer allocator.free(field);
    @memset(field, '.');

    var lines: [][:'\n']u8 = try allocator.alloc([:'\n']u8, field_size);
    defer allocator.free(lines);

    for (0..field_size) |i| {
        field[(field_size + 1) * (i + 1) - 1] = '\n';
        lines[i] = field[(field_size + 1) * i .. (field_size + 1) * (i + 1) - 1 :'\n'];
    }

    var buf: [32]u8 = undefined;
    for (0..simulated_bytes) |_| {
        const line = (try input.reader().readUntilDelimiterOrEof(&buf, '\n')).?;

        var iter = std.mem.tokenizeAny(u8, line, &[_]u8{','});

        const x = try std.fmt.parseInt(u8, iter.next().?, 10);
        const y = try std.fmt.parseInt(u8, iter.next().?, 10);

        lines[y][x] = '#';
    }

    // naive dijkstra
    var vertices = Context.init(allocator);
    defer vertices.deinit();

    var q = std.PriorityQueue(Point, *Context, compareFn).init(allocator, &vertices);
    defer q.deinit();

    try vertices.put(.{ .y = 0, .x = 0 }, 0);
    try q.add(.{ .y = 0, .x = 0 });

    while (q.removeOrNull()) |u| {
        if (u.x == field_size - 1 and u.y == field_size - 1)
            return vertices.get(u);
        const distance = vertices.get(u).?;

        // up
        if (u.y > 0 and lines[u.y - 1][u.x] == '.') {
            const new = Point{ .y = u.y - 1, .x = u.x };
            const res = try vertices.getOrPut(new);
            if (!res.found_existing) {
                res.value_ptr.* = distance + 1;
                try q.add(new);
            } else {
                res.value_ptr.* = @min(res.value_ptr.*, distance + 1);
            }
        }
        // down
        if (u.y < field_size - 1 and lines[u.y + 1][u.x] == '.') {
            const new = Point{ .y = u.y + 1, .x = u.x };
            const res = try vertices.getOrPut(new);
            if (!res.found_existing) {
                res.value_ptr.* = distance + 1;
                try q.add(new);
            } else {
                res.value_ptr.* = @min(res.value_ptr.*, distance + 1);
            }
        }
        // left
        if (u.x > 0 and lines[u.y][u.x - 1] == '.') {
            const new = Point{ .y = u.y, .x = u.x - 1 };
            const res = try vertices.getOrPut(new);
            if (!res.found_existing) {
                res.value_ptr.* = distance + 1;
                try q.add(new);
            } else {
                res.value_ptr.* = @min(res.value_ptr.*, distance + 1);
            }
        }
        // right
        if (u.x < field_size - 1 and lines[u.y][u.x + 1] == '.') {
            const new = Point{ .y = u.y, .x = u.x + 1 };
            const res = try vertices.getOrPut(new);
            if (!res.found_existing) {
                res.value_ptr.* = distance + 1;
                try q.add(new);
            } else {
                res.value_ptr.* = @min(res.value_ptr.*, distance + 1);
            }
        }
    }

    return error.NoPathFound;
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
        result = (try task_1(allocator, "data.txt", false)).?;
    } else {
        result = (try task_2(allocator, "data.txt")).?;
    }

    try stdout.print("{d}\n", .{result});
}

test task_1 {
    try std.testing.expectEqual(22, try task_1(std.testing.allocator, "data_test1.txt", true));
}

test task_2 {}
