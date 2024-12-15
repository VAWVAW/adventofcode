const std = @import("std");

const result_type = usize;
const Point = struct { x: usize, y: usize };
const Direction = enum {
    up,
    right,
    down,
    left,

    fn fromChar(char: u8) Direction {
        return switch (char) {
            '^' => .up,
            '>' => .right,
            'v' => .down,
            '<' => .left,
            else => unreachable,
        };
    }
    fn move(self: Direction, p: Point) Point {
        return switch (self) {
            .up => Point{ .x = p.x, .y = p.y - 1 },
            .right => Point{ .x = p.x + 1, .y = p.y },
            .down => Point{ .x = p.x, .y = p.y + 1 },
            .left => Point{ .x = p.x - 1, .y = p.y },
        };
    }
};

fn doMove(d: Direction, lines: [][]u8, start: Point) bool {
    const new_pos = d.move(start);

    switch (lines[new_pos.y][new_pos.x]) {
        '#' => return false,
        'O' => {
            if (!doMove(d, lines, new_pos)) {
                return false;
            }
        },
        else => {},
    }

    std.mem.swap(u8, &lines[new_pos.y][new_pos.x], &lines[start.y][start.x]);

    return true;
}

fn task_1(allocator: std.mem.Allocator, file_name: []const u8) !?result_type {
    // open input
    var file = try std.fs.cwd().openFile(file_name, .{});
    defer file.close();
    var input = std.io.bufferedReader(file.reader());

    // read data
    var buf: [1024]u8 = undefined;

    var output = std.ArrayList(u8).init(allocator);
    defer output.deinit();

    var line_length: ?usize = null;

    while (try input.reader().readUntilDelimiterOrEof(&buf, '\n')) |line| {
        if (line.len == 0)
            break;

        try output.ensureUnusedCapacity(line.len + 1);
        output.appendSliceAssumeCapacity(line);
        output.appendAssumeCapacity('\n');

        if (line_length == null)
            line_length = line.len;
    }

    var lines = std.ArrayList([:'\n']u8).init(allocator);
    defer lines.deinit();
    {
        var i: usize = 0;
        while (i < output.items.len) : (i += line_length.? + 1) {
            try lines.append(output.items[i .. i + line_length.? :'\n']);
        }
    }

    // loop over data
    const start_pos_data = std.mem.indexOf(u8, output.items, "@").?;
    var pos = Point{
        .y = start_pos_data / (line_length.? + 1),
        .x = start_pos_data % (line_length.? + 1),
    };

    lines.items[pos.y][pos.x] = '.';

    while (true) {
        const move = input.reader().readByte() catch break;
        if (move == '\n')
            continue;
        const dir = Direction.fromChar(move);

        if (doMove(dir, lines.items, pos)) {
            pos = dir.move(pos);
        }
    }

    var result: result_type = 0;

    for (lines.items, 0..) |line, y| {
        for (line, 0..) |c, x| {
            if (c == 'O') {
                result += x + 100 * y;
            }
        }
    }

    return result;
}

fn canMove2(d: Direction, lines: []const []const u8, start: Point) bool {
    const new = d.move(start);
    const c = lines[new.y][new.x];

    const other = switch (c) {
        '#' => return false,
        '.' => return true,

        '[' => Direction.right.move(new),
        ']' => Direction.left.move(new),
        else => unreachable,
    };

    if (d == .right or d == .left)
        return canMove2(d, lines, other);

    return canMove2(d, lines, new) and
        canMove2(d, lines, other);
}
fn doMove2(d: Direction, lines: [][]u8, start: Point) void {
    const new = d.move(start);
    const c = lines[new.y][new.x];

    const other = switch (c) {
        '.' => {
            std.mem.swap(u8, &lines[new.y][new.x], &lines[start.y][start.x]);
            return;
        },

        '[' => Direction.right.move(new),
        ']' => Direction.left.move(new),
        else => unreachable,
    };

    doMove2(d, lines, other);
    if (d == .left or d == .right) {
        std.mem.swap(u8, &lines[other.y][other.x], &lines[new.y][new.x]);
    } else {
        doMove2(d, lines, new);
    }
    std.mem.swap(u8, &lines[new.y][new.x], &lines[start.y][start.x]);
}

fn task_2(allocator: std.mem.Allocator, file_name: []const u8) !?result_type {
    // open input
    var file = try std.fs.cwd().openFile(file_name, .{});
    defer file.close();
    var input = std.io.bufferedReader(file.reader());

    // read data
    var buf: [1024]u8 = undefined;

    var output = std.ArrayList(u8).init(allocator);
    defer output.deinit();

    var line_length: ?usize = null;

    while (try input.reader().readUntilDelimiterOrEof(&buf, '\n')) |line| {
        if (line.len == 0)
            break;

        try output.ensureUnusedCapacity(line.len * 2 + 1);
        for (line) |c| {
            output.appendSliceAssumeCapacity(switch (c) {
                '#' => "##",
                'O' => "[]",
                '.' => "..",
                '@' => "@.",
                else => unreachable,
            });
        }
        output.appendAssumeCapacity('\n');

        if (line_length == null)
            line_length = line.len * 2;
    }

    var lines = std.ArrayList([:'\n']u8).init(allocator);
    defer lines.deinit();
    {
        var i: usize = 0;
        while (i < output.items.len) : (i += line_length.? + 1) {
            try lines.append(output.items[i .. i + line_length.? :'\n']);
        }
    }

    // loop over data
    const start_pos_data = std.mem.indexOf(u8, output.items, "@").?;
    var pos = Point{
        .y = start_pos_data / (line_length.? + 1),
        .x = start_pos_data % (line_length.? + 1),
    };

    lines.items[pos.y][pos.x] = '.';

    while (true) {
        const move = input.reader().readByte() catch break;
        if (move == '\n')
            continue;
        const dir = Direction.fromChar(move);

        if (canMove2(dir, lines.items, pos)) {
            doMove2(dir, lines.items, pos);
            pos = dir.move(pos);
        }
    }

    var result: result_type = 0;

    for (lines.items, 0..) |line, y| {
        for (line, 0..) |c, x| {
            if (c == '[') {
                result += x + 100 * y;
            }
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
    try std.testing.expectEqual(2028, try task_1(std.testing.allocator, "data_test2.txt"));
    try std.testing.expectEqual(10092, try task_1(std.testing.allocator, "data_test1.txt"));
}

test task_2 {
    try std.testing.expectEqual(9021, try task_2(std.testing.allocator, "data_test1.txt"));
}
