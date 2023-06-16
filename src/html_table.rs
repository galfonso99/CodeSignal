fn solution(table: String, row: i32, column: i32) -> String {
    // Given an nth row and mth column find the nth "<tr>" then from that point on find the mth <td>
    let row = table.split("<tr>").nth(row as usize + 1).unwrap_or(&"No such cell");
    let column = row.split("<td>").nth(column as usize + 1).unwrap_or(&"No such cell");  
    column.split("</td>").next().unwrap_or("No such cell").to_string()
}