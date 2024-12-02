const std = @import("std");

fn task_1(allocator: std.mem.Allocator, file_name: []const u8) !?i32 {
    _ = allocator;

    // open input
    var file = try std.fs.cwd().openFile(file_name, .{});
    defer file.close();
    var input = std.io.bufferedReader(file.reader());

    // read data
    var result: i32 = 0;

    var buf: [256]u8 = undefined;
    while (try input.reader().readUntilDelimiterOrEof(&buf, '\n')) |line| {
        var iter = std.mem.tokenizeAny(u8, line, &[_]u8{' '});

        result += 1;
        var prev = try std.fmt.parseInt(i32, iter.next().?, 10);

        if (try std.fmt.parseInt(i32, iter.peek().?, 10) < prev) {
            while (iter.next()) |next| {
                const val = try std.fmt.parseInt(i32, next, 10);
                if (prev - val > 3 or prev - val < 1) {
                    result -= 1;
                    break;
                }
                prev = val;
            }
        } else {
            while (iter.next()) |next| {
                const val = try std.fmt.parseInt(i32, next, 10);
                if (val - prev > 3 or val - prev < 1) {
                    result -= 1;
                    break;
                }
                prev = val;
            }
        }
    }

    return result;
}

fn task_2(allocator: std.mem.Allocator, file_name: []const u8) !?i32 {
    // open input
    var file = try std.fs.cwd().openFile(file_name, .{});
    defer file.close();
    var input = std.io.bufferedReader(file.reader());

    // read data
    var result: i32 = 0;

    var buf: [256]u8 = undefined;
    while (try input.reader().readUntilDelimiterOrEof(&buf, '\n')) |line| {
        var iter = std.mem.tokenizeAny(u8, line, &[_]u8{' '});
        var list = std.ArrayList(i32).init(allocator);
        defer list.deinit();

        while (iter.next()) |next| {
            try list.append(try std.fmt.parseInt(i32, next, 10));
        }

        for (0..list.items.len) |i| {
            var new_list = try list.clone();
            defer new_list.deinit();

            _ = new_list.orderedRemove(i);

            var failed = false;
            var prev = new_list.items[0];
            const is_falling = new_list.items[1] < prev;

            for (new_list.items[1..]) |val| {
                if (@abs(prev - val) > 3 or @abs(prev - val) < 1 or (val < prev) != is_falling) {
                    failed = true;
                    break;
                }
                prev = val;
            }

            if (!failed) {
                result += 1;
                break;
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
    var result: i32 = 0;
    if (args.len < 2 or args[1][0] != '2') {
        result = (try task_1(allocator, "data.txt")).?;
    } else {
        result = (try task_2(allocator, "data.txt")).?;
    }

    try stdout.print("{d}\n", .{result});
}

test task_1 {
    try std.testing.expect(try task_1(std.testing.allocator, "data_test1.txt") == 2);
}

test task_2 {
    try std.testing.expect(try task_2(std.testing.allocator, "data_test2.txt") == 4);
}
