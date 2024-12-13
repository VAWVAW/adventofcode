const std = @import("std");

const result_type = usize;

fn task_1(allocator: std.mem.Allocator, file_name: []const u8) !?result_type {
    _ = allocator;

    // open input
    var file = try std.fs.cwd().openFile(file_name, .{});
    defer file.close();
    var input = std.io.bufferedReader(file.reader());

    // read data
    var result: result_type = 0;
    var buf: [32]u8 = undefined;
    while (try input.reader().readUntilDelimiterOrEof(&buf, '\n')) |a_line| {
        var a_iter = std.mem.tokenizeAny(u8, a_line["Button A: X+".len..a_line.len], ", Y+");
        const a_diff_x = try std.fmt.parseInt(result_type, a_iter.next().?, 10);
        const a_diff_y = try std.fmt.parseInt(result_type, a_iter.next().?, 10);

        const b_line = (try input.reader().readUntilDelimiterOrEof(&buf, '\n')).?;
        var b_iter = std.mem.tokenizeAny(u8, b_line["Button B: X+".len..b_line.len], ", Y+");
        const b_diff_x = try std.fmt.parseInt(result_type, b_iter.next().?, 10);
        const b_diff_y = try std.fmt.parseInt(result_type, b_iter.next().?, 10);

        const prize_line = (try input.reader().readUntilDelimiterOrEof(&buf, '\n')).?;
        var prize_iter = std.mem.tokenizeAny(u8, prize_line["Prize: X=".len..prize_line.len], ", Y=");
        const prize_x = try std.fmt.parseInt(result_type, prize_iter.next().?, 10);
        const prize_y = try std.fmt.parseInt(result_type, prize_iter.next().?, 10);

        // ignore next line
        _ = input.reader().readUntilDelimiterOrEof(&buf, '\n') catch {};

        var min_cost: ?result_type = null;
        for (0..101) |a_num| {
            const a_x = a_num * a_diff_x;

            if (a_x > prize_x)
                break;

            if ((prize_x - a_x) % b_diff_x != 0)
                continue;
            const b_num = (prize_x - a_x) / b_diff_x;

            if (a_diff_y * a_num + b_diff_y * b_num != prize_y) {
                continue;
            }

            const cost = 3 * a_num + b_num;
            if (min_cost) |old_cost| {
                if (old_cost > cost)
                    min_cost = cost;
            } else {
                min_cost = cost;
            }
        }

        if (min_cost) |cost| {
            result += cost;
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
    try std.testing.expectEqual(480, try task_1(std.testing.allocator, "data_test1.txt"));
}

test task_2 {}
