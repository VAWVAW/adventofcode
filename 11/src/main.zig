const std = @import("std");

const result_type = usize;

fn task_1(allocator: std.mem.Allocator, file_name: []const u8) !?result_type {
    // open input
    var file = try std.fs.cwd().openFile(file_name, .{});
    defer file.close();
    var input = std.io.bufferedReader(file.reader());

    // read data
    const data = try input.reader().readAllAlloc(allocator, 1 << 32);
    defer allocator.free(data);

    const stone_type = u64;
    const List = std.ArrayList(stone_type);

    var old_stones = List.init(allocator);
    defer old_stones.deinit();
    var new_stones = List.init(allocator);
    defer new_stones.deinit();

    var stone_iter = std.mem.tokenizeAny(u8, data, &[_]u8{ ' ', '\n' });
    while (stone_iter.next()) |token| {
        const value = try std.fmt.parseInt(stone_type, token, 10);
        try old_stones.append(value);
    }

    // loop over data
    for (0..25) |_| {
        new_stones.clearRetainingCapacity();
        for (old_stones.items) |stone| {
            if (stone == 0) {
                try new_stones.append(1);
                continue;
            }
            const log = std.math.log10(stone);
            if (log % 2 == 1) {
                const offset = std.math.pow(stone_type, 10, (log + 1) / 2);
                try new_stones.append(stone / offset);
                try new_stones.append(stone % offset);
            } else {
                try new_stones.append(stone * 2024);
            }
        }
        std.mem.swap(List, &old_stones, &new_stones);
    }

    return old_stones.items.len;
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
    try std.testing.expectEqual(55312, try task_1(std.testing.allocator, "data_test1.txt"));
}

test task_2 {
    // try std.testing.expectEqual(81, try task_2(std.testing.allocator, "data_test1.txt"));
}
