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

fn task_2(allocator: std.mem.Allocator, file_name: []const u8) !?result_type {
    _ = allocator;
    _ = file_name;
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
    try std.testing.expectEqual(140, try task_1(std.testing.allocator, "data_test1.txt"));
    try std.testing.expectEqual(772, try task_1(std.testing.allocator, "data_test2.txt"));
    try std.testing.expectEqual(1930, try task_1(std.testing.allocator, "data_test3.txt"));
}

test task_2 {
    // try std.testing.expectEqual(81, try task_2(std.testing.allocator, "data_test1.txt"));
}
