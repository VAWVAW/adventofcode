const std = @import("std");

const Direction = enum(u2) {
    up = 0,
    right = 1,
    down = 2,
    left = 3,
};

fn task_1(allocator: std.mem.Allocator, file_name: []const u8) !?i32 {
    // open input
    var file = try std.fs.cwd().openFile(file_name, .{});
    defer file.close();
    var input = std.io.bufferedReader(file.reader());

    // read data
    const data = try input.reader().readAllAlloc(allocator, 1 << 32);
    defer allocator.free(data);

    const line_length = std.mem.indexOf(u8, data, "\n").?;

    const start_pos_data = std.mem.indexOf(u8, data, "^").?;

    var lines = std.ArrayList([:'\n']u8).init(allocator);
    defer lines.deinit();

    var i: usize = 0;
    while (i < data.len) {
        try lines.append(data[i .. i + line_length :'\n']);
        i += line_length + 1;
    }

    // loop over data
    var result: i32 = 1;

    var visited = std.AutoHashMap([2]usize, void).init(allocator);
    defer visited.deinit();

    var y = start_pos_data / (line_length + 1);
    var x = start_pos_data % (line_length + 1);
    var direction = Direction.up;

    var new_y: usize = undefined;
    var new_x: usize = undefined;

    while (true) {
        switch (direction) {
            .up => {
                if (y == 0) break;
                new_x = x;
                new_y = y - 1;
            },
            .right => {
                if (x >= line_length - 1) break;
                new_x = x + 1;
                new_y = y;
            },
            .down => {
                if (y >= lines.items.len - 1) break;
                new_x = x;
                new_y = y + 1;
            },
            .left => {
                if (x == 0) break;
                new_x = x - 1;
                new_y = y;
            },
        }

        if (lines.items[new_y][new_x] == '#') {
            const tag: u2 = @intFromEnum(direction);
            direction = @enumFromInt(tag +% 1);
            continue;
        }

        x = new_x;
        y = new_y;

        if (!visited.contains(.{ y, x }))
            result += 1;

        try visited.put(.{ y, x }, {});
    }

    return result;
}

fn task_2(allocator: std.mem.Allocator, file_name: []const u8) !?i32 {
    _ = allocator;
    _ = file_name;

    return error.Unimplemented;
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
    try std.testing.expectEqual(42, try task_1(std.testing.allocator, "data_test1.txt"));
}

test task_2 {
    // try std.testing.expectEqual(9, try task_2(std.testing.allocator, "data_test2.txt"));
}
