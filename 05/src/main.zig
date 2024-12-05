const std = @import("std");

fn task_1(allocator: std.mem.Allocator, file_name: []const u8) !?i32 {
    // open input
    var file = try std.fs.cwd().openFile(file_name, .{});
    defer file.close();
    var input = std.io.bufferedReader(file.reader());

    // read data
    var rules = std.AutoHashMap(u8, [24]u8).init(allocator);
    defer rules.deinit();

    var readbuf: [70]u8 = undefined;
    while (input.reader().readUntilDelimiter(&readbuf, '\n')) |line| {
        if (line.len == 0) {
            break;
        }

        const left = try std.fmt.parseInt(u8, line[0..2], 10);
        const right = try std.fmt.parseInt(u8, line[3..5], 10);

        const result = try rules.getOrPut(left);

        if (!result.found_existing) {
            result.value_ptr.* = [_]u8{0} ** 24;
        }
        for (result.value_ptr) |*val| {
            if (val.* == 0) {
                val.* = right;
                break;
            }
        } else {
            return error.RuleOverflow;
        }
    } else |err| {
        return err;
    }

    // loop over data
    var result: i32 = 0;
    var seen_values = std.AutoHashMap(u8, void).init(allocator);
    defer seen_values.deinit();

    while (try input.reader().readUntilDelimiterOrEof(&readbuf, '\n')) |line| : (seen_values.clearRetainingCapacity()) {
        var iter = std.mem.tokenizeAny(u8, line, &[_]u8{','});

        check_line: while (iter.next()) |next| {
            const to_check = try std.fmt.parseInt(u8, next, 10);
            try seen_values.put(to_check, {});

            if (rules.get(to_check)) |later_values| {
                for (later_values) |later_val| {
                    if (later_val == 0) {
                        break;
                    }
                    if (seen_values.contains(later_val)) {
                        break :check_line;
                    }
                }
            }
        } else {
            // line was accepted
            const line_middle = line.len / 2;
            const middle_value = try std.fmt.parseInt(i32, line[line_middle - 1 .. line_middle + 1], 10);

            result += middle_value;
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
    var rules = std.AutoHashMap(u8, [24]u8).init(allocator);
    defer rules.deinit();

    var readbuf: [70]u8 = undefined;
    while (input.reader().readUntilDelimiter(&readbuf, '\n')) |line| {
        if (line.len == 0) {
            break;
        }

        const left = try std.fmt.parseInt(u8, line[0..2], 10);
        const right = try std.fmt.parseInt(u8, line[3..5], 10);

        const result = try rules.getOrPut(left);

        if (!result.found_existing) {
            result.value_ptr.* = [_]u8{0} ** 24;
        }
        for (result.value_ptr) |*val| {
            if (val.* == 0) {
                val.* = right;
                break;
            }
        } else {
            return error.RuleOverflow;
        }
    } else |err| {
        return err;
    }

    // loop over data
    var result: i32 = 0;
    var seen_values = std.AutoHashMap(u8, void).init(allocator);
    defer seen_values.deinit();

    while (try input.reader().readUntilDelimiterOrEof(&readbuf, '\n')) |line| : (seen_values.clearRetainingCapacity()) {
        var iter = std.mem.tokenizeAny(u8, line, &[_]u8{','});

        check_line: while (iter.next()) |next| {
            const to_check = try std.fmt.parseInt(u8, next, 10);
            try seen_values.put(to_check, {});

            if (rules.get(to_check)) |later_values| {
                for (later_values) |later_val| {
                    if (later_val == 0) {
                        break;
                    }
                    if (seen_values.contains(later_val)) {
                        break :check_line;
                    }
                }
            }
        } else {
            // line was in correct order
            continue;
        }

        // read rest of numbers
        while (iter.next()) |next| {
            const to_check = try std.fmt.parseInt(u8, next, 10);
            try seen_values.put(to_check, {});
        }

        // reorder line
        var new_order = std.ArrayList(u8).init(allocator);
        defer new_order.deinit();

        const end_index = (line.len + 1) / 6;
        while (new_order.items.len < end_index + 1) {
            var possible_val_iter = seen_values.keyIterator();
            val_outer: while (possible_val_iter.next()) |val| {
                // check if value is below any other value
                var later_val_iter = seen_values.keyIterator();
                while (later_val_iter.next()) |later_val| {
                    if (rules.get(later_val.*)) |rule| {
                        for (rule) |after_later| {
                            if (after_later == 0) {
                                break;
                            }

                            if (after_later == val.*) {
                                // later_val < val
                                continue :val_outer;
                            }
                        }
                    }
                } else {
                    // val is below later values
                    try new_order.append(val.*);
                    _ = seen_values.remove(val.*);
                    break :val_outer;
                }
            } else {
                return error.NoOrderFound;
            }
        }

        result += new_order.items[end_index];
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
    try std.testing.expectEqual(143, try task_1(std.testing.allocator, "data_test1.txt"));
}

test task_2 {
    try std.testing.expectEqual(123, try task_2(std.testing.allocator, "data_test2.txt"));
}
