const std = @import("std");

const result_type = usize;
const LineList = std.ArrayList([:'\n']u8);
const Region = struct { area: result_type, perimeter: result_type };

fn count_pos(lines: *LineList, y: usize, x: usize) Region {
    const c = lines.items[y][x];

    lines.items[y][x] |= 0x20;

    var ret = Region{
        .area = 1,
        .perimeter = 0,
    };

    if (y == 0) {
        ret.perimeter += 1;
    } else if (lines.items[y - 1][x] == c) {
        const new = count_pos(lines, y - 1, x);
        ret.area += new.area;
        ret.perimeter += new.perimeter;
    } else if (lines.items[y - 1][x] == c | 0x20) {
        // field of same region already processed
    } else {
        ret.perimeter += 1;
    }

    if (x == 0) {
        ret.perimeter += 1;
    } else if (lines.items[y][x - 1] == c) {
        const new = count_pos(lines, y, x - 1);
        ret.area += new.area;
        ret.perimeter += new.perimeter;
    } else if (lines.items[y][x - 1] == c | 0x20) {
        // field of same region already processed
    } else {
        ret.perimeter += 1;
    }

    if (y == lines.items.len - 1) {
        ret.perimeter += 1;
    } else if (lines.items[y + 1][x] == c) {
        const new = count_pos(lines, y + 1, x);
        ret.area += new.area;
        ret.perimeter += new.perimeter;
    } else if (lines.items[y + 1][x] == c | 0x20) {
        // field of same region already processed
    } else {
        ret.perimeter += 1;
    }

    if (x == lines.items[0].len - 1) {
        ret.perimeter += 1;
    } else if (lines.items[y][x + 1] == c) {
        const new = count_pos(lines, y, x + 1);
        ret.area += new.area;
        ret.perimeter += new.perimeter;
    } else if (lines.items[y][x + 1] == c | 0x20) {
        // field of same region already processed
    } else {
        ret.perimeter += 1;
    }

    return ret;
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

    var lines = LineList.init(allocator);
    defer lines.deinit();

    var i: usize = 0;
    while (i < data.len) {
        try lines.append(data[i .. i + line_length :'\n']);
        i += line_length + 1;
    }

    // loop over data
    var result: result_type = 0;

    for (lines.items, 0..) |line, y| {
        for (line, 0..) |region_c, x| {
            if (region_c & 0x20 != 0) {
                continue;
            }
            const ret = count_pos(&lines, y, x);
            result += ret.area * ret.perimeter;
        }
    }

    return result;
}

const Direction = enum { top, right, bottom, left };
const Point = struct { y: usize, x: usize, d: Direction };
const PointSet = std.AutoArrayHashMap(Point, void);

fn count_pos_2(lines: *LineList, perimeter: *PointSet, y: usize, x: usize) !result_type {
    const c = lines.items[y][x];

    lines.items[y][x] |= 0x20;

    var area: result_type = 1;

    if (y == 0) {
        try perimeter.put(.{ .y = y, .x = x, .d = .top }, {});
    } else if (lines.items[y - 1][x] == c) {
        area += try count_pos_2(lines, perimeter, y - 1, x);
    } else if (lines.items[y - 1][x] == c | 0x20) {
        // field of same region already processed
    } else {
        try perimeter.put(.{ .y = y, .x = x, .d = .top }, {});
    }

    if (x == 0) {
        try perimeter.put(.{ .y = y, .x = x, .d = .left }, {});
    } else if (lines.items[y][x - 1] == c) {
        area += try count_pos_2(lines, perimeter, y, x - 1);
    } else if (lines.items[y][x - 1] == c | 0x20) {
        // field of same region already processed
    } else {
        try perimeter.put(.{ .y = y, .x = x, .d = .left }, {});
    }

    if (y == lines.items.len - 1) {
        try perimeter.put(.{ .y = y, .x = x, .d = .bottom }, {});
    } else if (lines.items[y + 1][x] == c) {
        area += try count_pos_2(lines, perimeter, y + 1, x);
    } else if (lines.items[y + 1][x] == c | 0x20) {
        // field of same region already processed
    } else {
        try perimeter.put(.{ .y = y, .x = x, .d = .bottom }, {});
    }

    if (x == lines.items[0].len - 1) {
        try perimeter.put(.{ .y = y, .x = x, .d = .right }, {});
    } else if (lines.items[y][x + 1] == c) {
        area += try count_pos_2(lines, perimeter, y, x + 1);
    } else if (lines.items[y][x + 1] == c | 0x20) {
        // field of same region already processed
    } else {
        try perimeter.put(.{ .y = y, .x = x, .d = .right }, {});
    }

    return area;
}

fn lessThatFn(ctx: void, lhs: Point, rhs: Point) bool {
    _ = ctx;

    if (lhs.d != rhs.d) {
        return @intFromEnum(lhs.d) < @intFromEnum(rhs.d);
    }

    if (lhs.d == .top or lhs.d == .bottom) {
        if (lhs.y != rhs.y) {
            return lhs.y < rhs.y;
        }
        return lhs.x < rhs.x;
    }

    if (lhs.x != rhs.x) {
        return lhs.x < rhs.x;
    }
    return lhs.y < rhs.y;
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

    var lines = LineList.init(allocator);
    defer lines.deinit();

    var i: usize = 0;
    while (i < data.len) {
        try lines.append(data[i .. i + line_length :'\n']);
        i += line_length + 1;
    }

    // loop over data
    var result: result_type = 0;

    var perimeter = PointSet.init(allocator);
    defer perimeter.deinit();

    for (lines.items, 0..) |line, y| {
        for (line, 0..) |region_c, x| {
            if (region_c & 0x20 != 0) {
                continue;
            }
            perimeter.clearRetainingCapacity();
            const area = try count_pos_2(&lines, &perimeter, y, x);

            var sides: result_type = 0;
            var last: ?Point = null;
            std.mem.sortUnstable(Point, perimeter.keys(), {}, lessThatFn);
            for (perimeter.keys()) |point| {
                if (last == null or last.?.d != point.d) {
                    sides += 1;
                } else if (point.d == .top or point.d == .bottom) {
                    if (last.?.y != point.y) {
                        sides += 1;
                    } else if (last.?.x + 1 != point.x) {
                        sides += 1;
                    }
                } else {
                    if (last.?.x != point.x) {
                        sides += 1;
                    } else if (last.?.y + 1 != point.y) {
                        sides += 1;
                    }
                }
                last = point;
            }
            result += area * sides;
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
    try std.testing.expectEqual(140, try task_1(std.testing.allocator, "data_test1.txt"));
    try std.testing.expectEqual(772, try task_1(std.testing.allocator, "data_test2.txt"));
    try std.testing.expectEqual(1930, try task_1(std.testing.allocator, "data_test3.txt"));
}

test task_2 {
    try std.testing.expectEqual(80, try task_2(std.testing.allocator, "data_test1.txt"));
    try std.testing.expectEqual(436, try task_2(std.testing.allocator, "data_test2.txt"));
    try std.testing.expectEqual(1206, try task_2(std.testing.allocator, "data_test3.txt"));
    try std.testing.expectEqual(236, try task_2(std.testing.allocator, "data_test4.txt"));
    try std.testing.expectEqual(368, try task_2(std.testing.allocator, "data_test5.txt"));
}
