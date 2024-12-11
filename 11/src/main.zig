const std = @import("std");

const result_type = usize;
const stone_type = u64;
const StoneContainer = std.AutoArrayHashMap(stone_type, result_type);

fn add_stone(container: *StoneContainer, stone: stone_type, n: result_type) !void {
    const res = try container.getOrPutValue(stone, 0);
    res.value_ptr.* += n;
}

fn task_1(allocator: std.mem.Allocator, file_name: []const u8, runs: usize) !?result_type {
    // open input
    var file = try std.fs.cwd().openFile(file_name, .{});
    defer file.close();
    var input = std.io.bufferedReader(file.reader());

    // read data
    const data = try input.reader().readAllAlloc(allocator, 1 << 32);
    defer allocator.free(data);

    var old_stones = StoneContainer.init(allocator);
    defer old_stones.deinit();
    var new_stones = StoneContainer.init(allocator);
    defer new_stones.deinit();

    var data_iter = std.mem.tokenizeAny(u8, data, &[_]u8{ ' ', '\n' });
    while (data_iter.next()) |token| {
        const value = try std.fmt.parseInt(stone_type, token, 10);
        try add_stone(&old_stones, value, 1);
    }

    // loop over data
    for (0..runs) |_| {
        new_stones.clearRetainingCapacity();

        var stone_iter = old_stones.iterator();
        while (stone_iter.next()) |entry| {
            const stone = entry.key_ptr.*;
            const n = entry.value_ptr.*;

            if (stone == 0) {
                try add_stone(&new_stones, 1, n);
                continue;
            }
            const log = std.math.log10(stone);
            if (log % 2 == 1) {
                const offset = std.math.pow(stone_type, 10, (log + 1) / 2);
                try add_stone(&new_stones, stone / offset, n);
                try add_stone(&new_stones, stone % offset, n);
            } else {
                try add_stone(&new_stones, stone * 2024, n);
            }
        }
        std.mem.swap(StoneContainer, &old_stones, &new_stones);
    }

    var result: result_type = 0;
    var iter = old_stones.iterator();
    while (iter.next()) |entry| {
        result += entry.value_ptr.*;
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
        result = (try task_1(allocator, "data.txt", 25)).?;
    } else {
        result = (try task_1(allocator, "data.txt", 75)).?;
    }

    try stdout.print("{d}\n", .{result});
}

test task_1 {
    try std.testing.expectEqual(22, try task_1(std.testing.allocator, "data_test1.txt", 6));
    try std.testing.expectEqual(55312, try task_1(std.testing.allocator, "data_test1.txt", 25));
}
